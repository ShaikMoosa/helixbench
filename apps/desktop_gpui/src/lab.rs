//! Full-page workspace views (Home / Notebook / Registry / Inventory /
//! Workflows / Insights / Automation) for the revamped left-navigation shell.
//! Lists/tables live here; detailed editors reuse the shared dialogs.

use gpui::{canvas, div, prelude::*, px, size, AnyElement, Context, SharedString};

use appcore::actions as act;
use appcore::state::{Dialog, View};

use crate::app::HelixBench;
use crate::draw::{self, PxVal};
use crate::theme;
use crate::widgets::{self, Btn};

fn page(title: &str, subtitle: &str) -> gpui::Div {
    div().flex().flex_col().size_full().bg(theme::BG).child(
        div()
            .flex()
            .items_center()
            .justify_between()
            .px(px(16.0))
            .py(px(12.0))
            .bg(theme::PANEL)
            .border_b_1()
            .border_color(theme::BORDER)
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(2.0))
                    .child(
                        div()
                            .text_size(px(theme::T_LG))
                            .font_weight(gpui::FontWeight::SEMIBOLD)
                            .text_color(theme::TEXT)
                            .child(title.to_owned()),
                    )
                    .child(
                        div()
                            .text_size(px(theme::T_SM))
                            .text_color(theme::MUTED)
                            .child(subtitle.to_owned()),
                    ),
            ),
    )
}

fn scroll(id: &str) -> gpui::Stateful<gpui::Div> {
    div()
        .id(SharedString::from(id.to_string()))
        .flex_1()
        .min_h(px(0.0))
        .overflow_y_scroll()
        .px(px(16.0))
        .py(px(12.0))
        .flex()
        .flex_col()
        .gap(px(6.0))
}

fn row_card() -> gpui::Div {
    div()
        .flex()
        .items_center()
        .gap(px(8.0))
        .p(px(8.0))
        .rounded(px(6.0))
        .bg(theme::PANEL)
        .border_1()
        .border_color(theme::BORDER)
}

pub fn render_home(app: &mut HelixBench, cx: &mut Context<HelixBench>) -> AnyElement {
    let mut p = page("Home", "Your local workspace at a glance");
    let mut body = scroll("home-scroll");
    let mut cards = div().flex().gap(px(10.0));
    for (label, value) in [
        ("Documents", app.app.library.len() + 1),
        ("Registry", app.app.registry.entries.len()),
        ("Inventory", app.app.inventory.containers.len()),
        ("Tasks", app.app.workflows.tasks.len()),
        ("Entries", app.app.records.entries.len()),
    ] {
        cards = cards.child(
            div()
                .flex_1()
                .flex()
                .flex_col()
                .gap(px(4.0))
                .p(px(12.0))
                .rounded(px(8.0))
                .bg(theme::PANEL)
                .border_1()
                .border_color(theme::BORDER)
                .child(
                    div()
                        .text_size(px(theme::T_XL))
                        .font_weight(gpui::FontWeight::BOLD)
                        .text_color(theme::ACCENT)
                        .child(value.to_string()),
                )
                .child(
                    div()
                        .text_size(px(theme::T_SM))
                        .text_color(theme::MUTED)
                        .child(label.to_owned()),
                ),
        );
    }
    body = body.child(cards);
    body = body.child(
        div()
            .text_size(px(theme::T_MD))
            .font_weight(gpui::FontWeight::SEMIBOLD)
            .text_color(theme::TEXT)
            .child("Recent entries"),
    );
    for e in app.app.records.entries.iter().rev().take(6) {
        let id = e.id;
        body = body.child(
            row_card()
                .child(
                    div()
                        .flex_grow()
                        .text_size(px(theme::T_SM))
                        .text_color(theme::TEXT)
                        .child(e.title.clone()),
                )
                .child(widgets::label(e.status.label(), theme::T_XS, theme::MUTED))
                .child(
                    widgets::button(
                        SharedString::from(format!("hm-open-{id}")),
                        "Open",
                        Btn::Subtle,
                    )
                    .on_click(cx.listener(move |t, _, _, cx| {
                        act::open_entry_editor(&mut t.app, Some(id), 0);
                        cx.notify();
                    })),
                ),
        );
    }
    let mut quick = div().flex().gap(px(6.0));
    for (v, label) in [
        (View::Notebook, "Notebook"),
        (View::Registry, "Registry"),
        (View::Inventory, "Inventory"),
        (View::Workflows, "Workflows"),
        (View::Insights, "Insights"),
        (View::Automation, "Automation"),
    ] {
        quick = quick.child(
            widgets::button(
                SharedString::from(format!("hm-nav-{}", v.label())),
                label,
                Btn::Outline,
            )
            .on_click(cx.listener(move |t, _, _, cx| t.set_view(v, cx))),
        );
    }
    body = body.child(quick);
    p = p.child(body);
    p.into_any_element()
}

pub fn render_notebook(app: &mut HelixBench, cx: &mut Context<HelixBench>) -> AnyElement {
    let mut p = page(
        "Notebook",
        "Experimental entries with structured tables and review",
    );
    let mut body = scroll("nb-scroll");
    body = body.child(
        div()
            .flex()
            .gap(px(6.0))
            .child(
                widgets::button("nb-new", "New entry", Btn::Primary).on_click(cx.listener(
                    |t, _, _, cx| {
                        act::open_entry_editor(&mut t.app, None, 0);
                        cx.notify();
                    },
                )),
            )
            .child(
                widgets::button("nb-tpl", "From template", Btn::Outline).on_click(cx.listener(
                    |t, _, _, cx| {
                        act::open_entry_editor(&mut t.app, None, 0);
                        cx.notify();
                    },
                )),
            ),
    );
    let entries: Vec<domain::notebook::Entry> = app.app.records.entries.to_vec();
    if entries.is_empty() {
        body = body.child(widgets::label(
            "No entries yet. Create one to start documenting experiments.",
            theme::T_SM,
            theme::FAINT,
        ));
    }
    for e in entries {
        let id = e.id;
        body = body.child(
            row_card()
                .child(
                    div()
                        .flex_grow()
                        .flex()
                        .flex_col()
                        .gap(px(2.0))
                        .child(
                            div()
                                .text_size(px(theme::T_SM))
                                .text_color(theme::TEXT)
                                .child(e.title.clone()),
                        )
                        .child(
                            div()
                                .text_size(px(theme::T_XS))
                                .text_color(theme::FAINT)
                                .child(format!(
                                    "{} sections · {} tables",
                                    e.sections.len(),
                                    e.tables.len()
                                )),
                        ),
                )
                .child(widgets::label(e.status.label(), theme::T_XS, theme::MUTED))
                .child(
                    widgets::button(
                        SharedString::from(format!("nb-open-{id}")),
                        "Open",
                        Btn::Subtle,
                    )
                    .on_click(cx.listener(move |t, _, _, cx| {
                        act::open_entry_editor(&mut t.app, Some(id), 0);
                        cx.notify();
                    })),
                )
                .child(
                    widgets::button(
                        SharedString::from(format!("nb-sub-{id}")),
                        "Submit",
                        Btn::Subtle,
                    )
                    .on_click(cx.listener(move |t, _, _, cx| {
                        act::entry_action(&mut t.app, id, "submit");
                        cx.notify();
                    })),
                )
                .child(
                    widgets::button(
                        SharedString::from(format!("nb-app-{id}")),
                        "Approve",
                        Btn::Subtle,
                    )
                    .on_click(cx.listener(move |t, _, _, cx| {
                        act::entry_action(&mut t.app, id, "approve");
                        cx.notify();
                    })),
                ),
        );
    }
    p = p.child(body);
    p.into_any_element()
}

pub fn render_registry(app: &mut HelixBench, cx: &mut Context<HelixBench>) -> AnyElement {
    let mut p = page("Registry", "Catalog of plasmids, strains and primers");
    let mut body = scroll("rg-scroll");
    body = body.child(
        div()
            .flex()
            .gap(px(6.0))
            .child(
                widgets::button("rg-add", "Add entry", Btn::Primary).on_click(cx.listener(
                    |t, _, _, cx| {
                        act::open_registry_editor(&mut t.app, 0, None);
                        cx.notify();
                    },
                )),
            )
            .child(
                widgets::button("rg-import", "Import CSV", Btn::Outline).on_click(cx.listener(
                    |t, _, _, cx| {
                        act::open_registry_import(&mut t.app);
                        cx.notify();
                    },
                )),
            )
            .child(
                widgets::button("rg-map", "Entity Map", Btn::Outline).on_click(cx.listener(
                    |t, _, _, cx| {
                        act::open_entity_map(&mut t.app);
                        cx.notify();
                    },
                )),
            ),
    );
    let entries: Vec<domain::registry::Entry> = app.app.registry.entries.to_vec();
    for e in entries {
        let id = e.id;
        let archived = e.archived;
        body = body.child(
            row_card()
                .child(
                    div()
                        .flex_grow()
                        .text_size(px(theme::T_SM))
                        .text_color(if archived { theme::FAINT } else { theme::TEXT })
                        .child(e.name().to_string()),
                )
                .child(widgets::label(e.kind.label(), theme::T_XS, theme::MUTED))
                .child(widgets::label(
                    e.linked_doc.as_deref().unwrap_or("—"),
                    theme::T_XS,
                    theme::FAINT,
                ))
                .child(
                    widgets::button(
                        SharedString::from(format!("rg-edit-{id}")),
                        "Edit",
                        Btn::Subtle,
                    )
                    .on_click(cx.listener(move |t, _, _, cx| {
                        let kind = e_kind(&t.app, id);
                        act::open_registry_editor(&mut t.app, kind, Some(id));
                        cx.notify();
                    })),
                )
                .child(
                    widgets::button(
                        SharedString::from(format!("rg-arch-{id}")),
                        if archived { "Unarchive" } else { "Archive" },
                        Btn::Ghost,
                    )
                    .on_click(cx.listener(move |t, _, _, cx| {
                        act::archive_registry_entry(&mut t.app, id, !archived);
                        cx.notify();
                    })),
                ),
        );
    }
    p = p.child(body);
    p.into_any_element()
}

fn e_kind(app: &appcore::state::AppState, id: u64) -> usize {
    app.registry.get(id).map(|e| e.kind as usize).unwrap_or(0)
}

pub fn render_inventory(app: &mut HelixBench, cx: &mut Context<HelixBench>) -> AnyElement {
    let mut p = page(
        "Inventory",
        "Physical storage: locations, boxes, plates, containers",
    );
    let mut body = scroll("inv-scroll");
    body = body.child(
        div()
            .flex()
            .gap(px(6.0))
            .child(
                widgets::button("inv-loc", "Add location", Btn::Outline).on_click(cx.listener(
                    |t, _, _, cx| {
                        let d = appcore::state::InventoryDraft {
                            name: "Location".into(),
                            kind: 0,
                            ..Default::default()
                        };
                        act::inv_add(&mut t.app, &d);
                        cx.notify();
                    },
                )),
            )
            .child(
                widgets::button("inv-plate", "Add plate", Btn::Outline).on_click(cx.listener(
                    |t, _, _, cx| {
                        let d = appcore::state::InventoryDraft {
                            name: "Plate".into(),
                            kind: 2,
                            ..Default::default()
                        };
                        act::inv_add(&mut t.app, &d);
                        cx.notify();
                    },
                )),
            )
            .child(
                widgets::button("inv-open", "Open manager", Btn::Primary).on_click(cx.listener(
                    |t, _, _, cx| {
                        act::open_inventory(&mut t.app);
                        cx.notify();
                    },
                )),
            ),
    );
    let containers: Vec<domain::inventory::Container> = app.app.inventory.containers.to_vec();
    for c in containers {
        let id = c.id;
        body = body.child(
            row_card()
                .child(
                    div()
                        .flex_grow()
                        .text_size(px(theme::T_SM))
                        .text_color(theme::TEXT)
                        .child(c.name.clone()),
                )
                .child(widgets::label(c.kind.label(), theme::T_XS, theme::MUTED))
                .child(widgets::label(
                    c.entity.as_deref().unwrap_or("—"),
                    theme::T_XS,
                    theme::FAINT,
                ))
                .child(
                    widgets::button(
                        SharedString::from(format!("inv-sel-{id}")),
                        "Manage",
                        Btn::Subtle,
                    )
                    .on_click(cx.listener(move |t, _, _, cx| {
                        act::open_inventory(&mut t.app);
                        if let Some(Dialog::Inventory(d)) = &mut t.app.dialog {
                            d.selected_plate = Some(id);
                        }
                        cx.notify();
                    })),
                ),
        );
    }
    p = p.child(body);
    p.into_any_element()
}

pub fn render_workflows(app: &mut HelixBench, cx: &mut Context<HelixBench>) -> AnyElement {
    let mut p = page(
        "Workflows",
        "Assigned tasks, requests and flowchart execution",
    );
    let mut body = scroll("wf-scroll");
    body = body.child(
        div()
            .flex()
            .gap(px(6.0))
            .child(
                widgets::button("wf-open", "Open manager", Btn::Primary).on_click(cx.listener(
                    |t, _, _, cx| {
                        act::open_workflows(&mut t.app);
                        cx.notify();
                    },
                )),
            )
            .child(
                widgets::button("wf-exec", "Execute next step", Btn::Outline).on_click(
                    cx.listener(|t, _, _, cx| {
                        act::wf_execute_next(&mut t.app);
                        cx.notify();
                    }),
                ),
            ),
    );
    let tasks: Vec<domain::workflow::Task> = app.app.workflows.tasks.to_vec();
    for t in tasks {
        let id = t.id;
        body = body.child(
            row_card()
                .child(
                    div()
                        .flex_grow()
                        .text_size(px(theme::T_SM))
                        .text_color(theme::TEXT)
                        .child(t.title.clone()),
                )
                .child(widgets::label(t.status.label(), theme::T_XS, theme::MUTED))
                .child(
                    widgets::button(
                        SharedString::from(format!("wf-start-{id}")),
                        "Start",
                        Btn::Subtle,
                    )
                    .on_click(cx.listener(move |t, _, _, cx| {
                        act::wf_set_status(
                            &mut t.app,
                            id,
                            domain::workflow::TaskStatus::InProgress,
                        );
                        cx.notify();
                    })),
                )
                .child(
                    widgets::button(
                        SharedString::from(format!("wf-done-{id}")),
                        "Complete",
                        Btn::Subtle,
                    )
                    .on_click(cx.listener(move |t, _, _, cx| {
                        act::wf_set_status(&mut t.app, id, domain::workflow::TaskStatus::Completed);
                        cx.notify();
                    })),
                ),
        );
    }
    p = p.child(body);
    p.into_any_element()
}

pub fn render_insights(app: &mut HelixBench, cx: &mut Context<HelixBench>) -> AnyElement {
    let mut p = page(
        "Insights",
        "Local dashboards over your registry / inventory / tasks / entries",
    );
    let mut body = scroll("ins-scroll");
    let datasets = act::datasets(&app.app);
    let mut pick = div().flex().gap(px(6.0));
    for ds in &datasets {
        let name = ds.name.clone();
        let group = if ds.columns.iter().any(|c| c == "Kind") {
            Some("Kind".to_string())
        } else if ds.columns.iter().any(|c| c == "Status") {
            Some("Status".to_string())
        } else {
            None
        };
        pick = pick.child(
            widgets::button(
                SharedString::from(format!("ins-src-{}", ds.name)),
                &ds.name,
                Btn::Outline,
            )
            .on_click(cx.listener(move |t, _, _, cx| {
                act::insights_add_block(
                    &mut t.app,
                    &name,
                    &name,
                    domain::insights::Chart::Bar,
                    group.clone(),
                );
                cx.notify();
            })),
        );
    }
    body = body.child(pick);
    // render blocks
    let blocks: Vec<domain::insights::Block> = app.app.analytics.blocks.to_vec();
    for b in &blocks {
        let ds = datasets
            .iter()
            .find(|d| d.name == b.source)
            .cloned()
            .unwrap_or_default();
        let series = domain::insights::series(b, &ds);
        body = body.child(
            div()
                .text_size(px(theme::T_MD))
                .font_weight(gpui::FontWeight::SEMIBOLD)
                .text_color(theme::TEXT)
                .child(format!("{} · {}", b.name, b.chart.label())),
        );
        // bar chart canvas
        let ser = series.clone();
        body = body.child(
            div()
                .h(px(140.0))
                .w_full()
                .rounded(px(6.0))
                .bg(theme::PANEL)
                .border_1()
                .border_color(theme::BORDER)
                .child(
                    canvas(
                        move |_, _, _| {},
                        move |bounds, _s, window, cx| {
                            let max = ser.iter().map(|(_, v)| *v).fold(1.0, f32::max);
                            let n = ser.len().max(1);
                            let bw = (bounds.size.width.v() - 20.0) / n as f32;
                            for (i, (label, v)) in ser.iter().enumerate() {
                                let h = (v / max) * (bounds.size.height.v() - 30.0);
                                let x = bounds.origin.x.v() + 10.0 + i as f32 * bw;
                                let y = bounds.origin.y.v() + bounds.size.height.v() - 20.0 - h;
                                draw::rect(
                                    window,
                                    gpui::Bounds::new(
                                        gpui::point(px(x + 4.0), px(y)),
                                        size(px(bw - 8.0), px(h)),
                                    ),
                                    3.0,
                                    theme::ACCENT,
                                );
                                draw::text(
                                    window,
                                    cx,
                                    gpui::point(
                                        px(x + bw / 2.0),
                                        px(bounds.origin.y.v() + bounds.size.height.v() - 8.0),
                                    ),
                                    label,
                                    9.0,
                                    theme::MUTED,
                                    theme::ui_font(),
                                    draw::Align::Center,
                                );
                            }
                        },
                    )
                    .size_full(),
                ),
        );
        let src = b.source.clone();
        body = body.child(
            widgets::button(
                SharedString::from(format!("ins-exp-{}", b.name)),
                "Export CSV",
                Btn::Ghost,
            )
            .on_click(cx.listener(move |t, _, _, cx| {
                act::insights_export(&mut t.app, &src);
                cx.notify();
            })),
        );
    }
    if blocks.is_empty() {
        body = body.child(widgets::label(
            "Add a dataset above to create a dashboard block.",
            theme::T_SM,
            theme::FAINT,
        ));
    }
    p = p.child(body);
    p.into_any_element()
}

pub fn render_automation(app: &mut HelixBench, cx: &mut Context<HelixBench>) -> AnyElement {
    let mut p = page(
        "Automation",
        "File-ingestion → transform → writeback pipelines",
    );
    let mut body = scroll("auto-scroll");
    let csv = app.app.automation_csv.clone();
    let run_csv = csv.clone();
    let loaded = app.app.automation_csv_name.clone();
    body = body.child(
        div()
            .flex()
            .gap(px(6.0))
            .child(
                widgets::button("auto-add", "New pipeline", Btn::Primary).on_click(cx.listener(
                    |t, _, _, cx| {
                        act::automation_add_pipeline(&mut t.app, "Pipeline");
                        cx.notify();
                    },
                )),
            )
            .child(
                widgets::button("auto-load", "Load CSV…", Btn::Outline).on_click(cx.listener(
                    |t, _, _, cx| {
                        act::automation_load_csv(&mut t.app);
                        cx.notify();
                    },
                )),
            )
            .child(
                widgets::button("auto-run", "Run on CSV", Btn::Outline).on_click(cx.listener(
                    move |t, _, _, cx| {
                        let id = t.app.analytics.automation.pipelines.last().map(|p| p.id);
                        if let Some(id) = id {
                            act::automation_run(&mut t.app, id, &run_csv);
                        }
                        cx.notify();
                    },
                )),
            ),
    );
    if loaded.is_empty() {
        body = body.child(widgets::label(
            "Load a CSV file (header row + data rows) to run a pipeline.",
            theme::T_SM,
            theme::FAINT,
        ));
    } else {
        let header = csv.lines().next().unwrap_or("").to_owned();
        let nrows = csv
            .lines()
            .filter(|l| !l.trim().is_empty())
            .count()
            .saturating_sub(1);
        body = body.child(
            row_card()
                .child(
                    div()
                        .flex_grow()
                        .text_size(px(theme::T_SM))
                        .text_color(theme::TEXT)
                        .child(loaded),
                )
                .child(widgets::label(
                    &format!("{nrows} rows · {header}"),
                    theme::T_XS,
                    theme::FAINT,
                )),
        );
    }
    let pipelines: Vec<domain::automation::Pipeline> =
        app.app.analytics.automation.pipelines.to_vec();
    for pl in pipelines {
        let id = pl.id;
        body = body.child(
            row_card()
                .child(
                    div()
                        .flex_grow()
                        .text_size(px(theme::T_SM))
                        .text_color(theme::TEXT)
                        .child(pl.name.clone()),
                )
                .child(widgets::label(
                    if pl.published { "Published" } else { "Draft" },
                    theme::T_XS,
                    theme::MUTED,
                ))
                .child(
                    widgets::button(
                        SharedString::from(format!("auto-pub-{id}")),
                        "Publish",
                        Btn::Subtle,
                    )
                    .on_click(cx.listener(move |t, _, _, cx| {
                        act::automation_publish(&mut t.app, id);
                        cx.notify();
                    })),
                ),
        );
    }
    body = body.child(
        div()
            .text_size(px(theme::T_MD))
            .font_weight(gpui::FontWeight::SEMIBOLD)
            .text_color(theme::TEXT)
            .child("Run history"),
    );
    for r in app.app.analytics.automation.runs.iter().rev().take(6) {
        body = body.child(
            row_card()
                .child(
                    div()
                        .flex_grow()
                        .text_size(px(theme::T_SM))
                        .text_color(theme::TEXT)
                        .child(r.pipeline.clone()),
                )
                .child(widgets::label(
                    match r.status {
                        domain::automation::RunStatus::Completed => "Completed",
                        _ => "Failed",
                    },
                    theme::T_XS,
                    theme::MUTED,
                ))
                .child(widgets::label(
                    &format!("{} rows", r.output.len()),
                    theme::T_XS,
                    theme::FAINT,
                )),
        );
        for l in r.log.iter().take(4) {
            body = body.child(
                div()
                    .text_size(px(theme::T_XS))
                    .font_family(theme::MONO_FONT)
                    .text_color(theme::FAINT)
                    .child(l.clone()),
            );
        }
    }
    p = p.child(body);
    p.into_any_element()
}
