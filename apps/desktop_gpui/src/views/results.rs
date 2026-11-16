//! Saved result catalog, immutable input inspection and explicit next actions.
use crate::{
    app::HelixBench,
    theme,
    widgets::{self, Btn},
};
use appcore::{results as r, state::Dialog};
use domain::results::AnalysisResult;
use gpui::{div, prelude::*, px, AnyElement, Context, SharedString};
pub fn render(app: &mut HelixBench, cx: &mut Context<HelixBench>) -> AnyElement {
    let mut page = div()
        .id("analysis-results-page")
        .size_full()
        .overflow_y_scroll()
        .flex()
        .flex_col()
        .gap(px(14.0))
        .p(px(24.0))
        .bg(theme::BG);
    page = page.child(
        widgets::button("result-back", "Back to previous task", Btn::Outline).on_click(
            cx.listener(|t, _, _, cx| {
                if let Err(e) = r::close(&mut t.app) {
                    t.app.results.error = Some(e);
                }
                cx.notify();
            }),
        ),
    );
    let result = app.app.results.pending.as_deref().cloned().or_else(|| {
        app.app.first_use.selected_result.and_then(|id| {
            app.app
                .first_use
                .results
                .results
                .iter()
                .find(|r| r.id == id)
                .cloned()
        })
    });
    if let Some(result) = result {
        let saved = app
            .app
            .first_use
            .results
            .results
            .iter()
            .any(|r| r.id == result.id);
        page = page
            .child(widgets::label(
                &format!("{} analysis result", result.tool),
                theme::T_XL,
                theme::TEXT,
            ))
            .child(widgets::label(
                &format!(
                    "{} · {} · {}",
                    result.id,
                    if saved {
                        "Saved locally"
                    } else {
                        "Unsaved completed run"
                    },
                    crate::util::timestamp_utc(result.created)
                ),
                theme::T_SM,
                theme::MUTED,
            ));
        let mut actions = div().flex().flex_wrap().gap(px(8.0));
        if !saved {
            let candidate = result.clone();
            actions = actions.child(
                widgets::button("result-save", "Save result", Btn::Primary).on_click(cx.listener(
                    move |t, _, _, cx| {
                        match r::save_analysis(&mut t.app, &candidate) {
                            Ok(id) => {
                                t.app.results.pending = None;
                                t.app.first_use.selected_result = Some(id);
                                t.app.results.error = None;
                            }
                            Err(e) => t.app.results.error = Some(e),
                        }
                        cx.notify();
                    },
                )),
            );
        }
        let original = result.clone();
        actions = actions.child(
            widgets::button("result-rerun", "Review rerun inputs", Btn::Outline).on_click(
                cx.listener(move |t, _, _, cx| {
                    match r::prepare_rerun(&t.app, &original, false) {
                        Ok(plan) => t.app.dialog = Some(Dialog::ResultRerun(Box::new(plan))),
                        Err(e) => t.app.results.error = Some(e),
                    }
                    cx.notify();
                }),
            ),
        );
        actions = actions.child(
            widgets::button("result-snapshot", "Inspect captured input", Btn::Outline).on_click(
                cx.listener(|t, _, _, cx| {
                    t.app.results.snapshot = !t.app.results.snapshot;
                    cx.notify();
                }),
            ),
        );
        let source_result = result.clone();
        actions = actions.child(
            widgets::button("result-live-source", "Open current source", Btn::Ghost).on_click(
                cx.listener(move |t, _, _, cx| {
                    if let Err(e) = r::open_current_source(&mut t.app, &source_result) {
                        t.app.results.error = Some(e);
                    }
                    cx.notify();
                }),
            ),
        );
        page = page.child(actions).child(widgets::label(
            &format!(
                "Method: {} · parameters and input version stay attached to this output",
                result.method_version
            ),
            theme::T_BASE,
            theme::TEXT,
        ));
        for input in &result.inputs {
            let status=match r::source_status(&app.app,input){r::SourceStatus::Current=>"Current open input",r::SourceStatus::OlderInput=>"Older input — the open source changed; this recorded output remains unchanged",r::SourceStatus::Unavailable=>"Source unavailable or closed — captured input retained read-only; no source is recreated"};
            page = page
                .child(widgets::operation_notice(status))
                .child(widgets::label(
                    &format!(
                        "{} · {} bp · {:?} · revision {}",
                        input.document().name,
                        input.document().sequence.len(),
                        input.document().sequence.topology,
                        input.source().revision
                    ),
                    theme::T_SM,
                    theme::TEXT,
                ));
            if app.app.results.snapshot {
                let doc = input.document();
                page = page
                    .child(widgets::label(
                        &format!(
                            "Captured document {} · SHA-256 {}",
                            input.source().document,
                            input.sha256()
                        ),
                        theme::T_XS,
                        theme::MUTED,
                    ))
                    .child(widgets::label(
                        "Captured sequence (first 120 bases; full snapshot included in export)",
                        theme::T_SM,
                        theme::MUTED,
                    ))
                    .child(widgets::label(
                        &String::from_utf8_lossy(
                            &doc.sequence.bases[..doc.sequence.len().min(120)],
                        ),
                        theme::T_SM,
                        theme::TEXT,
                    ))
                    .child(widgets::label(
                        &format!(
                            "{} features · {} primers · genetic code {}",
                            doc.features.len(),
                            doc.primers.len(),
                            doc.default_genetic_code
                        ),
                        theme::T_SM,
                        theme::MUTED,
                    ));
            }
        }
        page = page.child(output(&result));
        page = page.child(widgets::label(
            "Recorded settings",
            theme::T_LG,
            theme::TEXT,
        ));
        page = page.child(settings(&result));
        page = page.child(widgets::label(
            "Compare with another saved result",
            theme::T_LG,
            theme::TEXT,
        ));
        for other in app
            .app
            .first_use
            .results
            .results
            .iter()
            .filter(|r| r.id != result.id)
        {
            let id = other.id;
            page = page.child(
                widgets::button(
                    SharedString::from(format!("result-compare-{id}")),
                    &format!("Compare {} · {}", other.tool, id),
                    Btn::Outline,
                )
                .on_click(cx.listener(move |t, _, _, cx| {
                    t.app.results.compare = Some(id);
                    cx.notify();
                })),
            );
        }
        let mut exports = vec![result.clone()];
        if let Some(id) = app.app.results.compare {
            if let Some(other) = app
                .app
                .first_use
                .results
                .results
                .iter()
                .find(|r| r.id == id)
            {
                exports.push(other.clone());
                page = page
                    .child(widgets::label(
                        &format!("Comparison: {} / {}", result.id, other.id),
                        theme::T_LG,
                        theme::TEXT,
                    ))
                    .child(widgets::label(
                        &format!(
                            "Methods: {} / {} · same parameters: {} · same recorded output: {}",
                            result.method_version,
                            other.method_version,
                            result.parameters == other.parameters,
                            result.output == other.output
                        ),
                        theme::T_SM,
                        theme::TEXT,
                    ))
                    .child(widgets::label(
                        &format!(
                            "Other captured revisions: {}",
                            other
                                .inputs
                                .iter()
                                .map(|i| i.source().revision.to_string())
                                .collect::<Vec<_>>()
                                .join(", ")
                        ),
                        theme::T_SM,
                        theme::MUTED,
                    ))
                    .child(output(other));
            }
        }
        let mut export = div().flex().flex_wrap().gap(px(8.0));
        for (id, label, format) in [
            (
                "json",
                "Export JSON with full provenance",
                r::ExportFormat::Json,
            ),
            (
                "csv",
                "Export CSV with full provenance",
                r::ExportFormat::Csv,
            ),
        ] {
            let results = exports.clone();
            export = export.child(
                widgets::button(
                    SharedString::from(format!("result-export-{id}")),
                    label,
                    Btn::Outline,
                )
                .on_click(cx.listener(move |t, _, _, cx| {
                    if let Some(path) = appcore::fs_dialogs::pick_save(
                        "Export analysis result",
                        id,
                        &[id],
                        &format!("analysis-results.{id}"),
                        t.app.config.last_directory.as_ref(),
                    ) {
                        if let Err(e) = r::export_to(&mut t.app, &results, format, &path) {
                            t.app.results.error = Some(e);
                        }
                    }
                    cx.notify();
                })),
            );
        }
        page=page.child(export).child(widgets::label("Export includes exact input snapshots, method, settings, run identity and recorded output. JSON preserves the complete structured record; CSV stores one provenance-rich row per result. Canceling the file dialog writes nothing.",theme::T_SM,theme::MUTED));
    } else if app.app.first_use.selected_result.is_some() {
        page = page.child(widgets::label(
            "This result is unavailable. Refresh the catalog or return to the previous task.",
            theme::T_BASE,
            theme::DANGER,
        ));
    } else {
        page = page.child(widgets::label(
            "Saved analysis results",
            theme::T_XL,
            theme::TEXT,
        ));
        let input = app.input_entity(
            cx,
            "results:query",
            &app.app.results.query.clone(),
            "Find by result ID, input name, tool or method",
            false,
        );
        app.app.results.query = input.read(cx).text();
        let query = app.app.results.query.to_ascii_lowercase();
        page = page.child(input);
        page = page.child(
            widgets::button("results-refresh", "Refresh results", Btn::Outline).on_click(
                cx.listener(|t, _, _, cx| {
                    appcore::first_use::refresh_results(&mut t.app);
                    cx.notify();
                }),
            ),
        );
        let mut n = 0;
        for result in app.app.first_use.results.results.iter().rev() {
            let searchable = format!(
                "{} {} {} {}",
                result.id,
                result.tool,
                result.method_version,
                result
                    .inputs
                    .iter()
                    .map(|i| i.document().name.clone())
                    .collect::<Vec<_>>()
                    .join(" ")
            )
            .to_ascii_lowercase();
            if !searchable.contains(&query) {
                continue;
            }
            n += 1;
            let id = result.id;
            page = page.child(
                div()
                    .flex()
                    .flex_wrap()
                    .items_center()
                    .gap(px(8.0))
                    .p(px(12.0))
                    .bg(theme::PANEL)
                    .rounded(px(12.0))
                    .child(
                        div()
                            .flex_1()
                            .min_w(px(200.0))
                            .child(widgets::label(
                                &format!(
                                    "{} · {}",
                                    result.tool,
                                    result
                                        .inputs
                                        .iter()
                                        .map(|i| i.document().name.clone())
                                        .collect::<Vec<_>>()
                                        .join(", ")
                                ),
                                theme::T_BASE,
                                theme::TEXT,
                            ))
                            .child(widgets::label(
                                &format!("{} · {}", id, crate::util::timestamp_utc(result.created)),
                                theme::T_XS,
                                theme::MUTED,
                            )),
                    )
                    .child(
                        widgets::button(
                            SharedString::from(format!("results-open-{id}")),
                            "Open result",
                            Btn::Outline,
                        )
                        .on_click(cx.listener(move |t, _, _, cx| {
                            if let Err(e) = r::open(&mut t.app, id) {
                                t.app.results.error = Some(e);
                            }
                            cx.notify();
                        })),
                    ),
            );
        }
        if n == 0 {
            page=page.child(widgets::label(if query.is_empty(){"No saved results yet. Run a digest or PCR analysis, then save the completed result."}else{"No matching results. Change or clear the search."},theme::T_BASE,theme::MUTED));
        }
    }
    if let Some(e) = app
        .app
        .results
        .error
        .as_ref()
        .or(app.app.first_use.results_error.as_ref())
    {
        page = page.child(widgets::label(e, theme::T_BASE, theme::DANGER));
    }
    page.into_any_element()
}
fn output(result: &AnalysisResult) -> gpui::Div {
    let mut table = div()
        .flex()
        .flex_col()
        .gap(px(8.0))
        .p(px(14.0))
        .bg(theme::PANEL)
        .rounded(px(14.0))
        .child(widgets::label("Recorded output", theme::T_LG, theme::TEXT));
    if let Some(items) = result.output["fragments"].as_array() {
        if result.output["completed_no_sites"].as_bool() == Some(true) {
            table = table.child(widgets::label(
                "Completed — no cut sites; whole input retained",
                theme::T_BASE,
                theme::TEXT,
            ));
        }
        let mut total = 0;
        for (i, f) in items.iter().enumerate() {
            let size = f["size"].as_u64().unwrap_or(0);
            total += size;
            table = table.child(widgets::label(
                &format!(
                    "Fragment {} · {} bp · boundary {} → {}",
                    i + 1,
                    size,
                    f["start"],
                    f["end"]
                ),
                theme::T_BASE,
                theme::TEXT,
            ));
        }
        table = table.child(widgets::label(
            &format!("Total: {total} bp"),
            theme::T_BASE,
            theme::TEXT,
        ));
    } else if let Some(items) = result.output["products"].as_array() {
        if items.is_empty() {
            table = table.child(widgets::label(
                "Completed — no predicted PCR product",
                theme::T_BASE,
                theme::TEXT,
            ));
        }
        for (i, p) in items.iter().enumerate() {
            table = table.child(widgets::label(
                &format!(
                    "Product {} · {} bp · template body {} bp · forward {} / reverse {}",
                    i + 1,
                    p["len"],
                    p["template_len"],
                    p["fwd_site"]["start"].as_u64().unwrap_or(0) + 1,
                    p["rev_site"]["start"].as_u64().unwrap_or(0) + 1
                ),
                theme::T_BASE,
                theme::TEXT,
            ));
        }
    } else {
        table = table.child(widgets::label(
            &result.output.to_string(),
            theme::T_SM,
            theme::TEXT,
        ));
    }
    table
}

pub fn settings(result: &AnalysisResult) -> gpui::Div {
    let mut body = div().flex().flex_col().gap(px(6.0));
    for (key, value) in result.parameters.as_object().into_iter().flatten() {
        let text=match key.as_str(){
 "enzymes"=>format!("Enzymes: {}",value.as_array().map(|v|v.iter().filter_map(|e|e["name"].as_str()).collect::<Vec<_>>().join(", ")).unwrap_or_default()),
 "forward"|"reverse"=>{let sequence=value["sequence"].as_array().map(|v|v.iter().filter_map(|b|b.as_u64().and_then(|b|u8::try_from(b).ok())).map(char::from).collect::<String>()).unwrap_or_default();format!("{} primer: {} · 5′ {} 3′",if key=="forward"{"Forward"}else{"Reverse"},value["name"].as_str().unwrap_or("Entered sequence"),sequence)},
 "binding"=>format!("Binding: ≥{} matching 3′ bases · minimum Tm {}°C · salt {} mM · primer {} nM",value["min_3prime_match"],value["min_tm"],value["salt_mm"],value["primer_nm"]),
 "copy_features"=>format!("Copy contained visible features: {}",if value.as_bool()==Some(true){"Yes"}else{"No"}),
 "pair"=>continue,
 "scope"=>"Scope: entire template".into(),
 "cleavage"=>"Coordinates: top-strand cleavage boundaries count bases before the cut".into(),
 "gel"=>"Gel: logarithmic 100–10,000 bp model; illustrative ladder; methylation and uncut circular migration are not modeled".into(),
 _=>format!("{key}: {value}")};
        body = body.child(widgets::label(&text, theme::T_SM, theme::MUTED));
    }
    body
}
