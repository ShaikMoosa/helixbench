//! Immutable result lifecycle; persisted outputs never follow later source edits.
use crate::{
    navigation::{NavigationIntent, ReturnLocation, Route},
    state::{AppState, View},
};
use domain::{
    results::{AnalysisResult, InputSnapshot},
    OperationId, ResultId,
};
use serde_json::json;
use std::path::Path;
pub struct ResultReturn {
    location: ReturnLocation,
    dialog: Option<crate::state::Dialog>,
    pending: Option<Box<AnalysisResult>>,
    list: bool,
}
/// Run title derived from recorded parameters, without invented sample values.
pub fn display_title(result: &AnalysisResult) -> String {
    match result.tool.as_str() {
        "digest" => {
            let names = result
                .parameters
                .get("enzymes")
                .and_then(|v| v.as_array())
                .map(|enzymes| {
                    enzymes
                        .iter()
                        .filter_map(|e| {
                            e.get("name")
                                .and_then(|v| v.as_str())
                                .or_else(|| e.as_str())
                        })
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();
            if names.is_empty() {
                "Restriction digest".into()
            } else {
                format!("{} digest", names.join(" + "))
            }
        }
        "pcr" => "PCR prediction".into(),
        _ => result.tool.clone(),
    }
}
#[derive(Default)]
pub struct ResultsState {
    pub stack: Vec<ResultReturn>,
    pub compare: Option<ResultId>,
    pub snapshot: bool,
    pub pending: Option<Box<AnalysisResult>>,
    pub error: Option<String>,
    pub query: String,
    pub list: bool,
}
/// Completed previews retained by navigation, distinct from durable saved outputs.
pub fn unsaved_previews(app: &AppState) -> Vec<&AnalysisResult> {
    let mut seen = std::collections::HashSet::new();
    app.results
        .pending
        .as_deref()
        .into_iter()
        .chain(
            app.results
                .stack
                .iter()
                .rev()
                .filter_map(|r| r.pending.as_deref()),
        )
        .filter(|r| {
            seen.insert(r.id)
                && !app
                    .first_use
                    .results
                    .results
                    .iter()
                    .any(|saved| saved.id == r.id)
        })
        .collect()
}
pub fn resume_preview(app: &mut AppState, id: ResultId) -> Result<(), String> {
    if app.dialog.is_some() || app.navigation.pending.is_some() {
        return Err("Finish or cancel the open form first".into());
    }
    let result = unsaved_previews(app)
        .into_iter()
        .find(|r| r.id == id)
        .cloned()
        .ok_or("This session output is no longer available")?;
    open_preview(app, result)
}
fn push_return(app: &mut AppState) {
    let location = ReturnLocation::capture(app);
    let dialog = app.dialog.take();
    let pending = app.results.pending.take();
    let list = app.results.list;
    app.results.stack.push(ResultReturn {
        location,
        dialog,
        pending,
        list,
    });
}
pub fn browse(app: &mut AppState) -> Result<(), String> {
    if app.dialog.is_some() || app.navigation.pending.is_some() {
        return Err("Finish the open form first".into());
    }
    if app.view == View::Results
        && app.results.list
        && app.results.pending.is_none()
        && app.first_use.selected_result.is_none()
    {
        crate::first_use::refresh_results(app);
        return Ok(());
    }
    push_return(app);
    crate::navigation::request_navigation(
        app,
        NavigationIntent::Navigate(Route {
            view: View::Results,
            document: app.doc.id,
            record: None,
        }),
    )?;
    crate::first_use::refresh_results(app);
    app.first_use.selected_result = None;
    app.results.list = true;
    Ok(())
}
pub fn open_preview(app: &mut AppState, result: AnalysisResult) -> Result<(), String> {
    result.validate()?;
    if app.navigation.pending.is_some() {
        return Err("Finish the pending navigation decision first".into());
    }
    push_return(app);
    crate::navigation::request_navigation(
        app,
        NavigationIntent::Navigate(Route {
            view: View::Results,
            document: app.doc.id,
            record: None,
        }),
    )?;
    app.results.pending = Some(Box::new(result));
    app.results.list = false;
    app.results.error = None;
    app.results.compare = None;
    app.results.snapshot = false;
    Ok(())
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SourceStatus {
    Current,
    OlderInput,
    Unavailable,
}
pub fn source_status(app: &AppState, input: &InputSnapshot) -> SourceStatus {
    let live = std::iter::once(&app.doc)
        .chain(app.library.iter().map(|d| &d.doc))
        .find(|d| d.id == input.source().document);
    match live {
        None => SourceStatus::Unavailable,
        Some(d)
            if d.source_ref() == input.source()
                && InputSnapshot::capture(d, input.selection())
                    .is_ok_and(|i| i.sha256() == input.sha256()) =>
        {
            SourceStatus::Current
        }
        Some(_) => SourceStatus::OlderInput,
    }
}
pub fn from_digest(run: &crate::enzyme_workflow::DigestRun) -> Result<AnalysisResult, String> {
    Ok(AnalysisResult {
        id: run.result_id,
        operation: run.operation,
        tool: "digest".into(),
        method_version: "restriction-v1".into(),
        created: run.created,
        inputs: vec![run.input.clone()],
        parameters: json!({"enzymes":run.enzymes,"scope":"whole-template","cleavage":"top-strand boundary; bases before cut","gel":"logarithmic; 100–10000 bp; illustrative ladder; no methylation model"}),
        output: json!({"sites":run.sites,"fragments":run.fragments.iter().map(|f|json!({"start":f.start,"end":f.end,"size":f.size})).collect::<Vec<_>>(),"completed_no_sites":run.completed_no_sites}),
    })
}
pub fn from_pcr(run: &crate::primer_workflow::PcrRun) -> Result<AnalysisResult, String> {
    Ok(AnalysisResult {
        id: run.result_id,
        operation: run.operation,
        tool: "pcr".into(),
        method_version: "pcr-v1".into(),
        created: run.created,
        inputs: vec![(*run.input).clone()],
        parameters: json!({"forward":run.forward,"reverse":run.reverse,"binding":run.params,"copy_features":run.copy_features,"pair":run.pair}),
        output: json!({"products":run.products,"diagnostic":run.diagnostic}),
    })
}
pub fn save_analysis(app: &mut AppState, result: &AnalysisResult) -> Result<ResultId, String> {
    result.validate()?;
    let path = app.profile_path(storage::results::path());
    let (mut next, before) = storage::results::load_captured(&path).map_err(|e| {
        format!("Saved result catalog could not be read: {e}. It was not replaced.")
    })?;
    if let Some(old) = next
        .results
        .iter()
        .find(|r| r.id == result.id || r.operation == result.operation)
    {
        if old != result {
            return Err(
                "This run was already saved with different content; run again with a new identity"
                    .into(),
            );
        }
    } else {
        next.results.push(result.clone());
        let text = storage::results::catalog_text(&next).map_err(|e| e.to_string())?;
        let journal = app.operation_journal();
        let attempt = (|| {
            let plan = journal.prepare_text_checked(
                result.inputs[0].document(),
                &path,
                text,
                before.as_deref(),
            )?;
            journal.execute(plan.id())
        })();
        app.recovery = journal.recovery();
        let receipt = attempt.map_err(|e| {
            format!("Result save failed: {e}. The completed run is retained for retry.")
        })?;
        if receipt.state != domain::operation::OperationState::Completed {
            return Err("Result output awaits a completion receipt; use operation recovery before reporting it saved".into());
        }
    }
    app.first_use.results = next;
    app.first_use.results_error = None;
    app.config.remember_work(storage::config::RecentWork {
        target: storage::config::RecentTarget::Result(result.id),
        title: format!("{} · {}", result.tool, result.inputs[0].document().name),
    });
    let config_error = storage::config::save_to(
        &app.config,
        &app.profile_path(storage::config::config_path()),
    )
    .err();
    app.operation_error=config_error.map(|e|format!("Result saved and available in Results, but its recent-work shortcut could not be saved: {e}"));
    app.toast("Analysis result saved — find it in Results");
    Ok(result.id)
}
pub fn open(app: &mut AppState, id: ResultId) -> Result<(), String> {
    if app.dialog.is_some() || app.navigation.pending.is_some() {
        return Err("Finish or cancel the open form first".into());
    }
    crate::first_use::refresh_results(app);
    if let Some(e) = &app.first_use.results_error {
        return Err(e.clone());
    }
    if !app.first_use.results.results.iter().any(|r| r.id == id) {
        return Err("This saved result is no longer available; refresh the catalog".into());
    }
    push_return(app);
    crate::navigation::request_navigation(
        app,
        NavigationIntent::Navigate(Route {
            view: View::Results,
            document: app.doc.id,
            record: None,
        }),
    )?;
    app.first_use.selected_result = Some(id);
    app.results.list = false;
    app.results.snapshot = false;
    app.results.compare = None;
    app.results.error = None;
    Ok(())
}
pub fn close(app: &mut AppState) -> Result<(), String> {
    app.first_use.selected_result = None;
    app.results.snapshot = false;
    app.results.compare = None;
    app.results.pending = None;
    app.results.list = false;
    app.results.error = None;
    if let Some(origin) = app.results.stack.pop() {
        crate::navigation::restore_return_location(app, origin.location)?;
        app.dialog = origin.dialog;
        app.results.pending = origin.pending;
        app.results.list = origin.list;
    }
    Ok(())
}
pub fn open_current_source(app: &mut AppState, result: &AnalysisResult) -> Result<(), String> {
    let source = result.inputs.first().ok_or("No captured input")?.source();
    if !std::iter::once(&app.doc)
        .chain(app.library.iter().map(|d| &d.doc))
        .any(|d| d.id == source.document)
    {
        return Err("The current source is not open or is unavailable. Inspect its captured input here, or reopen the source file explicitly.".into());
    }
    crate::navigation::request_navigation(
        app,
        NavigationIntent::Navigate(Route {
            view: View::Map,
            document: source.document,
            record: None,
        }),
    )?;
    Ok(())
}
#[derive(Clone)]
pub struct RerunPlan {
    pub original: AnalysisResult,
    pub input: InputSnapshot,
    pub current: bool,
}
pub fn prepare_rerun(
    app: &AppState,
    original: &AnalysisResult,
    current: bool,
) -> Result<RerunPlan, String> {
    original.validate()?;
    if original.inputs.len() != 1 {
        return Err("This tool needs its own multi-input rerun adapter".into());
    }
    if !matches!(
        (original.tool.as_str(), original.method_version.as_str()),
        ("digest", "restriction-v1") | ("pcr", "pcr-v1")
    ) {
        return Err("Rerun is unavailable for this recorded method version; its historical output is still readable/exportable".into());
    }
    let input = if current {
        let d = std::iter::once(&app.doc)
            .chain(app.library.iter().map(|d| &d.doc))
            .find(|d| d.id == original.inputs[0].source().document)
            .ok_or("The current source is not open; reopen it or use the captured input")?;
        InputSnapshot::capture(d, None)?
    } else {
        original.inputs[0].clone()
    };
    Ok(RerunPlan {
        original: original.clone(),
        input,
        current,
    })
}
pub fn rerun(app: &AppState, plan: &RerunPlan) -> Result<AnalysisResult, String> {
    if plan.current && source_status(app, &plan.input) != SourceStatus::Current {
        return Err("The selected current input changed after review; reopen rerun setup".into());
    }
    let p = &plan.original.parameters;
    let d = plan.input.document();
    let output = match plan.original.tool.as_str() {
        "digest" => {
            if plan.original.method_version != "restriction-v1" {
                return Err("Unsupported recorded method".into());
            }
            let enzymes: Vec<domain::Enzyme> =
                serde_json::from_value(p["enzymes"].clone()).map_err(|e| e.to_string())?;
            if enzymes.is_empty() {
                return Err("The recorded enzyme set is empty".into());
            }
            crate::enzyme_workflow::predict(d, &enzymes)?
        }
        "pcr" => {
            if plan.original.method_version != "pcr-v1" {
                return Err("Unsupported recorded method".into());
            }
            let f = serde_json::from_value(p["forward"].clone()).map_err(|e| e.to_string())?;
            let r = serde_json::from_value(p["reverse"].clone()).map_err(|e| e.to_string())?;
            let params = serde_json::from_value(p["binding"].clone()).map_err(|e| e.to_string())?;
            let products = crate::primer_workflow::predict(d, &f, &r, &params)?;
            let diagnostic = products
                .is_empty()
                .then(|| "Run completed: no predicted product".to_string());
            json!({"products":products,"diagnostic":diagnostic})
        }
        _ => return Err("Rerun adapter unavailable".into()),
    };
    Ok(AnalysisResult {
        id: ResultId::new(),
        operation: OperationId::new(),
        tool: plan.original.tool.clone(),
        method_version: plan.original.method_version.clone(),
        created: crate::actions::unix_now(),
        inputs: vec![plan.input.clone()],
        parameters: p.clone(),
        output,
    })
}
#[derive(Clone, Copy)]
pub enum ExportFormat {
    Json,
    Csv,
}
pub fn export_text(results: &[AnalysisResult], format: ExportFormat) -> Result<String, String> {
    if results.is_empty() {
        return Err("Choose a result to export".into());
    }
    for r in results {
        r.validate()?;
    }
    match format {
        ExportFormat::Json => serde_json::to_string_pretty(
            &json!({"format":"HelixBench analysis export v1","results":results}),
        )
        .map_err(|e| e.to_string()),
        ExportFormat::Csv => {
            let q = |s: String| {
                let safe = if s.trim_start().starts_with(['=', '+', '-', '@'])
                    || s.starts_with(['\t', '\r', '\n'])
                {
                    format!("'{s}")
                } else {
                    s
                };
                format!("\"{}\"", safe.replace('"', "\"\""))
            };
            let mut out="result_id,operation_id,tool,method,created_utc_epoch,input_snapshots,parameters,output\r\n".to_string();
            for r in results {
                out.push_str(
                    &[
                        r.id.to_string(),
                        r.operation.to_string(),
                        r.tool.clone(),
                        r.method_version.clone(),
                        r.created.to_string(),
                        serde_json::to_string(&r.inputs).map_err(|e| e.to_string())?,
                        r.parameters.to_string(),
                        r.output.to_string(),
                    ]
                    .into_iter()
                    .map(q)
                    .collect::<Vec<_>>()
                    .join(","),
                );
                out.push_str("\r\n");
            }
            Ok(out)
        }
    }
}
pub fn export_to(
    app: &mut AppState,
    results: &[AnalysisResult],
    format: ExportFormat,
    path: &Path,
) -> Result<(), String> {
    let text = export_text(results, format)?;
    if path
        .extension()
        .and_then(|s| s.to_str())
        .is_none_or(|s| !matches!(s.to_ascii_lowercase().as_str(), "json" | "csv"))
    {
        return Err("Choose a .json or .csv export destination".into());
    }
    let normalize = |p: &Path| {
        std::path::absolute(p).map(|p| p.to_string_lossy().replace('/', "\\").to_ascii_lowercase())
    };
    let dest = normalize(path).map_err(|e| e.to_string())?;
    for protected in [
        app.profile_path(storage::results::path()),
        app.profile_path(storage::config::config_path()),
    ]
    .iter()
    .chain(app.path.iter())
    .chain(app.library.iter().filter_map(|d| d.path.as_ref()))
    {
        if normalize(protected).is_ok_and(|p| p == dest) {
            return Err("Choose another destination; an export cannot replace a source or workspace catalog".into());
        }
    }
    let journal = app.operation_journal();
    let attempt = (|| {
        let plan = journal.prepare_text(results[0].inputs[0].document(), path, text)?;
        journal.execute(plan.id())
    })();
    app.recovery = journal.recovery();
    let receipt = attempt.map_err(|e| e.to_string())?;
    if receipt.state != domain::operation::OperationState::Completed {
        return Err("Export awaits a completion receipt".into());
    }
    app.toast("Result export completed with input snapshots, method, settings and output");
    Ok(())
}
