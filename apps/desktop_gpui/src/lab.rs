//! Full-page workspace views (Home / Notebook / Registry / Inventory /
//! Workflows / Insights / Automation) for the revamped left-navigation shell.
//! Lists/tables live here; detailed editors reuse the shared dialogs.

use gpui::{canvas, div, prelude::*, px, size, AnyElement, Context, SharedString};

use appcore::actions as act;
use appcore::state::{Dialog, View};

use crate::app::HelixBench;
use crate::draw::{self, PxVal};
use crate::theme;
use crate::views::map::{paint as paint_map, MapDraw};
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

fn section_title(text: &str) -> gpui::Div {
    div()
        .text_size(px(theme::T_MD))
        .font_weight(gpui::FontWeight::SEMIBOLD)
        .text_color(theme::TEXT)
        .child(text.to_owned())
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

    body = body.child(section_title("Calendar"));
    let y = app.cal_year;
    let m = app.cal_month;
    let mut buckets: std::collections::HashMap<u32, (usize, usize)> =
        std::collections::HashMap::new();
    for e in &app.app.records.entries {
        let (ey, em, ed) = crate::util::civil_from_unix(e.created);
        if ey == y && em == m {
            buckets.entry(ed).or_insert((0, 0)).0 += 1;
        }
    }
    for t in &app.app.workflows.tasks {
        let (ty, tm, td) = crate::util::civil_from_unix(t.created);
        if ty == y && tm == m {
            buckets.entry(td).or_insert((0, 0)).1 += 1;
        }
    }
    let mut hdr = div().flex().items_center().gap(px(8.0));
    hdr = hdr.child(
        widgets::button("cal-prev", "‹", Btn::Ghost).on_click(cx.listener(|t, _, _, cx| {
            let (ny, nm) = crate::util::add_months(t.cal_year, t.cal_month, -1);
            t.cal_year = ny;
            t.cal_month = nm;
            t.cal_day = None;
            cx.notify();
        })),
    );
    hdr = hdr.child(widgets::label(
        &format!("{y:04}-{m:02}"),
        theme::T_SM,
        theme::TEXT,
    ));
    hdr = hdr.child(
        widgets::button("cal-next", "›", Btn::Ghost).on_click(cx.listener(|t, _, _, cx| {
            let (ny, nm) = crate::util::add_months(t.cal_year, t.cal_month, 1);
            t.cal_year = ny;
            t.cal_month = nm;
            t.cal_day = None;
            cx.notify();
        })),
    );
    hdr = hdr.child(
        widgets::button("cal-today", "Today", Btn::Subtle).on_click(cx.listener(|t, _, _, cx| {
            let (ny, nm, nd) = crate::util::civil_from_unix(crate::util::now_unix());
            t.cal_year = ny;
            t.cal_month = nm;
            t.cal_day = Some(nd);
            cx.notify();
        })),
    );
    body = body.child(hdr);
    let mut grid = div().flex().flex_col().gap(px(2.0));
    let mut week = div().flex().gap(px(2.0));
    for wd in ["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"] {
        week = week.child(
            div()
                .w(px(36.0))
                .text_size(px(theme::T_XS))
                .text_color(theme::FAINT)
                .child(wd),
        );
    }
    grid = grid.child(week);
    let start_wd = crate::util::weekday(y, m, 1);
    let dim = crate::util::days_in_month(y, m);
    let mut row = div().flex().gap(px(2.0));
    for _ in 0..start_wd {
        row = row.child(div().w(px(36.0)).h(px(32.0)));
    }
    let mut cell = start_wd;
    for d in 1..=dim {
        let (ne, nt) = buckets.get(&d).copied().unwrap_or((0, 0));
        let selected = app.cal_day == Some(d);
        row = row.child(
            div()
                .id(SharedString::from(format!("cal-{y}-{m}-{d}")))
                .w(px(36.0))
                .h(px(32.0))
                .flex()
                .flex_col()
                .items_center()
                .justify_center()
                .rounded(px(4.0))
                .cursor_pointer()
                .bg(if selected {
                    theme::SURFACE_2
                } else {
                    theme::transparent()
                })
                .hover(|s| s.bg(theme::SURFACE))
                .child(
                    div()
                        .text_size(px(theme::T_XS))
                        .text_color(theme::TEXT)
                        .child(d.to_string()),
                )
                .child(if ne + nt > 0 {
                    div()
                        .text_size(px(9.0))
                        .text_color(theme::ACCENT)
                        .child(format!("{}", ne + nt))
                } else {
                    div()
                })
                .on_click(cx.listener(move |t, _, _, cx| {
                    t.cal_day = Some(d);
                    cx.notify();
                })),
        );
        cell += 1;
        if cell.is_multiple_of(7) {
            grid = grid.child(row);
            row = div().flex().gap(px(2.0));
        }
    }
    if !cell.is_multiple_of(7) {
        while !cell.is_multiple_of(7) {
            row = row.child(div().w(px(36.0)).h(px(32.0)));
            cell += 1;
        }
        grid = grid.child(row);
    }
    body = body.child(grid);
    if let Some(d) = app.cal_day {
        let day_start = crate::util::unix_from_civil(y, m, d);
        let day_end = day_start + 86400;
        let mut list = div().flex().flex_col().gap(px(4.0));
        let mut any = false;
        for e in &app.app.records.entries {
            if e.created >= day_start && e.created < day_end {
                any = true;
                let id = e.id;
                list = list.child(
                    row_card()
                        .child(
                            div()
                                .flex_grow()
                                .text_size(px(theme::T_SM))
                                .text_color(theme::TEXT)
                                .child(format!("Entry: {}", e.title)),
                        )
                        .child(
                            widgets::button(
                                SharedString::from(format!("cal-open-{id}")),
                                "Open",
                                Btn::Subtle,
                            )
                            .on_click(cx.listener(
                                move |t, _, _, cx| {
                                    act::open_entry_editor(&mut t.app, Some(id), 0);
                                    cx.notify();
                                },
                            )),
                        ),
                );
            }
        }
        for t in &app.app.workflows.tasks {
            if t.created >= day_start && t.created < day_end {
                any = true;
                list = list.child(
                    row_card()
                        .child(
                            div()
                                .flex_grow()
                                .text_size(px(theme::T_SM))
                                .text_color(theme::TEXT)
                                .child(format!("Task: {}", t.title)),
                        )
                        .child(widgets::label(t.status.label(), theme::T_XS, theme::MUTED)),
                );
            }
        }
        if !any {
            list = list.child(widgets::label(
                "Nothing on this day.",
                theme::T_XS,
                theme::FAINT,
            ));
        }
        body = body.child(list);
    }

    body = body.child(section_title("My Work"));
    body = body.child(
        row_card()
            .child(
                div()
                    .flex_grow()
                    .text_size(px(theme::T_SM))
                    .text_color(theme::TEXT)
                    .child(app.app.doc.name.clone()),
            )
            .child(widgets::label("current", theme::T_XS, theme::ACCENT)),
    );
    for (i, od) in app.app.library.iter().enumerate() {
        let name = od.doc.name.clone();
        body = body.child(
            row_card()
                .child(
                    div()
                        .flex_grow()
                        .text_size(px(theme::T_SM))
                        .text_color(theme::TEXT)
                        .child(name),
                )
                .child(
                    widgets::button(
                        SharedString::from(format!("hm-switch-{i}")),
                        "Switch",
                        Btn::Subtle,
                    )
                    .on_click(cx.listener(move |t, _, _, cx| {
                        act::switch_doc(&mut t.app, i);
                        cx.notify();
                    })),
                ),
        );
    }

    body = body.child(section_title("Recent files"));
    if app.app.config.recent_files.is_empty() {
        body = body.child(widgets::label(
            "No recent files yet.",
            theme::T_XS,
            theme::FAINT,
        ));
    } else {
        for (i, p) in app.app.config.recent_files.iter().take(6).enumerate() {
            let path = p.clone();
            let label = p
                .file_name()
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_else(|| p.display().to_string());
            body = body.child(
                row_card()
                    .child(
                        div()
                            .flex_grow()
                            .text_size(px(theme::T_SM))
                            .text_color(theme::TEXT)
                            .child(label),
                    )
                    .child(
                        widgets::button(
                            SharedString::from(format!("hm-file-{i}")),
                            "Open",
                            Btn::Subtle,
                        )
                        .on_click(cx.listener(move |t, _, _, cx| {
                            act::open_path(&mut t.app, &path);
                            cx.notify();
                        })),
                    ),
            );
        }
    }

    body = body.child(section_title("Worklist"));
    let open_tasks: Vec<domain::workflow::Task> = app
        .app
        .workflows
        .tasks
        .iter()
        .filter(|t| t.status != domain::workflow::TaskStatus::Completed)
        .take(6)
        .cloned()
        .collect();
    if open_tasks.is_empty() {
        body = body.child(widgets::label("No open tasks.", theme::T_XS, theme::FAINT));
    } else {
        for t in open_tasks {
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
                            SharedString::from(format!("hm-wl-{id}")),
                            "Open",
                            Btn::Subtle,
                        )
                        .on_click(cx.listener(|t, _, _, cx| t.set_view(View::Workflows, cx))),
                    ),
            );
        }
    }

    body = body.child(section_title("Recent entries"));
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
        let status = e.status;
        let rev = e.revision;
        let mut actions: Vec<AnyElement> = vec![widgets::button(
            SharedString::from(format!("nb-open-{id}")),
            "Open",
            Btn::Subtle,
        )
        .on_click(cx.listener(move |t, _, _, cx| {
            act::open_entry_editor(&mut t.app, Some(id), 0);
            cx.notify();
        }))
        .into_any_element()];
        match status {
            domain::notebook::EntryStatus::Draft => actions.push(
                widgets::button(
                    SharedString::from(format!("nb-sub-{id}")),
                    "Submit",
                    Btn::Subtle,
                )
                .on_click(cx.listener(move |t, _, _, cx| {
                    act::entry_action(&mut t.app, id, "submit");
                    cx.notify();
                }))
                .into_any_element(),
            ),
            domain::notebook::EntryStatus::ChangesRequested => actions.push(
                widgets::button(
                    SharedString::from(format!("nb-resub-{id}")),
                    "Resubmit",
                    Btn::Primary,
                )
                .on_click(cx.listener(move |t, _, _, cx| {
                    act::entry_action(&mut t.app, id, "resubmit");
                    cx.notify();
                }))
                .into_any_element(),
            ),
            domain::notebook::EntryStatus::InReview => {
                actions.push(
                    widgets::button(
                        SharedString::from(format!("nb-chg-{id}")),
                        "Request changes",
                        Btn::Subtle,
                    )
                    .on_click(cx.listener(move |t, _, _, cx| {
                        act::entry_action(&mut t.app, id, "request_changes");
                        cx.notify();
                    }))
                    .into_any_element(),
                );
                actions.push(
                    widgets::button(
                        SharedString::from(format!("nb-app-{id}")),
                        "Approve",
                        Btn::Subtle,
                    )
                    .on_click(cx.listener(move |t, _, _, cx| {
                        act::entry_action(&mut t.app, id, "approve");
                        cx.notify();
                    }))
                    .into_any_element(),
                );
            }
            _ => {}
        }
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
                                    "{} sections · {} tables{}",
                                    e.sections.len(),
                                    e.tables.len(),
                                    if rev > 0 {
                                        format!(" · rev {rev}")
                                    } else {
                                        String::new()
                                    }
                                )),
                        ),
                )
                .child(widgets::label(
                    status.label(),
                    theme::T_XS,
                    if status == domain::notebook::EntryStatus::ChangesRequested {
                        theme::WARN
                    } else {
                        theme::MUTED
                    },
                ))
                .children(actions),
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

fn split_pane(d: MapDraw, name: String, subtitle: String) -> gpui::Div {
    div()
        .flex()
        .flex_col()
        .flex_1()
        .min_w(px(0.0))
        .h_full()
        .rounded(px(8.0))
        .bg(theme::PANEL)
        .border_1()
        .border_color(theme::BORDER)
        .overflow_hidden()
        .child(
            div()
                .px(px(10.0))
                .py(px(6.0))
                .border_b_1()
                .border_color(theme::BORDER)
                .flex()
                .items_center()
                .justify_between()
                .child(
                    div()
                        .text_size(px(theme::T_SM))
                        .font_weight(gpui::FontWeight::SEMIBOLD)
                        .text_color(theme::TEXT)
                        .child(name),
                )
                .child(widgets::label(&subtitle, theme::T_XS, theme::FAINT)),
        )
        .child(
            canvas(
                move |_, _, _| {},
                move |b, _s, w, cx| paint_map(&d, b, w, cx),
            )
            .size_full(),
        )
}

pub fn render_split(app: &mut HelixBench, cx: &mut Context<HelixBench>) -> AnyElement {
    let mut p = page("Split Workspace", "Compare two open documents side-by-side");
    let lib_len = app.app.library.len();
    if lib_len == 0 {
        let body = scroll("split-scroll").child(widgets::label(
            "Open a second document (e.g. a PCR product) to compare it with the active one.",
            theme::T_SM,
            theme::FAINT,
        ));
        p = p.child(body);
        return p.into_any_element();
    }
    let sel = app.app.split_sel.min(lib_len - 1);
    let left = MapDraw::build_compare(&app.app.doc, &app.app.prefs);
    let right_doc = app.app.library[sel].doc.clone();
    let right = MapDraw::build_compare(&right_doc, &app.app.prefs);
    let left_name = app.app.doc.name.clone();
    let right_name = right_doc.name.clone();
    let left_sub = format!(
        "{} bp · {}",
        app.app.doc.sequence.len(),
        app.app.doc.sequence.topology.label()
    );
    let right_sub = format!(
        "{} bp · {}",
        right_doc.sequence.len(),
        right_doc.sequence.topology.label()
    );
    let mut pick = div().flex().flex_row().gap(px(6.0)).items_center();
    pick = pick.child(widgets::label("Right pane:", theme::T_XS, theme::FAINT));
    for (i, od) in app.app.library.iter().enumerate() {
        let active = i == sel;
        let name = od.doc.name.clone();
        pick = pick.child(
            widgets::button(
                SharedString::from(format!("split-pick-{i}")),
                &name,
                if active { Btn::Primary } else { Btn::Outline },
            )
            .on_click(cx.listener(move |t, _, _, cx| {
                t.app.split_sel = i;
                cx.notify();
            })),
        );
    }
    let panes = div()
        .flex()
        .flex_row()
        .gap(px(10.0))
        .flex_1()
        .min_h(px(0.0))
        .child(split_pane(left, left_name, left_sub))
        .child(split_pane(right, right_name, right_sub));
    let content = div()
        .flex()
        .flex_col()
        .flex_1()
        .min_h(px(0.0))
        .px(px(16.0))
        .py(px(12.0))
        .gap(px(10.0))
        .child(pick)
        .child(panes);
    p = p.child(content);
    p.into_any_element()
}

fn draw_biologic_glyph(
    format: domain::biologics::Format,
    bounds: gpui::Bounds<gpui::Pixels>,
    window: &mut gpui::Window,
) {
    use domain::biologics::Format;
    let c = bounds.center();
    let (cx, cy) = (c.x.v(), c.y.v());
    match format {
        Format::Nanobody => {
            draw::disc(window, gpui::point(px(cx), px(cy)), 20.0, theme::ACCENT);
        }
        Format::Tcr => {
            draw::rect(
                window,
                gpui::Bounds::new(
                    gpui::point(px(cx - 26.0), px(cy - 34.0)),
                    size(px(20.0), px(68.0)),
                ),
                6.0,
                theme::ACCENT,
            );
            draw::rect(
                window,
                gpui::Bounds::new(
                    gpui::point(px(cx + 6.0), px(cy - 34.0)),
                    size(px(20.0), px(68.0)),
                ),
                6.0,
                theme::INFO,
            );
        }
        f => {
            // Y-shaped antibody (IgG / bispecific / Fab).
            draw::segment(
                window,
                gpui::point(px(cx), px(cy + 40.0)),
                gpui::point(px(cx), px(cy)),
                5.0,
                theme::MUTED,
            );
            draw::segment(
                window,
                gpui::point(px(cx), px(cy)),
                gpui::point(px(cx - 34.0), px(cy - 34.0)),
                5.0,
                theme::MUTED,
            );
            draw::disc(
                window,
                gpui::point(px(cx - 34.0), px(cy - 34.0)),
                9.0,
                theme::ACCENT,
            );
            if f != Format::Fab {
                draw::segment(
                    window,
                    gpui::point(px(cx), px(cy)),
                    gpui::point(px(cx + 34.0), px(cy - 34.0)),
                    5.0,
                    theme::MUTED,
                );
                let tip = if f == Format::Bispecific {
                    theme::INFO
                } else {
                    theme::ACCENT
                };
                draw::disc(window, gpui::point(px(cx + 34.0), px(cy - 34.0)), 9.0, tip);
            }
        }
    }
}

pub fn render_biologics(app: &mut HelixBench, cx: &mut Context<HelixBench>) -> AnyElement {
    let mut p = page(
        "Biologics",
        "Format-aware protein entities: chains, domains, numbering",
    );
    let mut body = scroll("bio-scroll");
    body = body.child(
        div()
            .flex()
            .items_center()
            .gap(px(8.0))
            .child(
                widgets::button("bio-reg", "Register biologic", Btn::Primary).on_click(
                    cx.listener(|t, _, _, cx| {
                        act::open_biologics_register(&mut t.app);
                        cx.notify();
                    }),
                ),
            )
            .child(widgets::label(
                &format!("{} entities", app.app.biologics.items.len()),
                theme::T_SM,
                theme::MUTED,
            )),
    );
    if app.app.biologics.items.is_empty() {
        body = body.child(widgets::label(
            "No biologics yet. Register an IgG, bispecific, nanobody, TCR or Fab.",
            theme::T_SM,
            theme::FAINT,
        ));
    }
    for b in app.app.biologics.items.clone() {
        let id = b.id;
        let selected = app.app.bio_sel == Some(id);
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
                                .child(b.name.clone()),
                        )
                        .child(
                            div()
                                .text_size(px(theme::T_XS))
                                .text_color(theme::FAINT)
                                .child(format!(
                                    "{} chains · {} residues",
                                    b.chains.len(),
                                    b.total_residues()
                                )),
                        ),
                )
                .child(widgets::label(b.format.label(), theme::T_XS, theme::ACCENT))
                .child(
                    widgets::button(
                        SharedString::from(format!("bio-sel-{id}")),
                        "Select",
                        if selected { Btn::Primary } else { Btn::Subtle },
                    )
                    .on_click(cx.listener(move |t, _, _, cx| {
                        t.app.bio_sel = Some(id);
                        cx.notify();
                    })),
                )
                .child(
                    widgets::button(
                        SharedString::from(format!("bio-del-{id}")),
                        "Delete",
                        Btn::Ghost,
                    )
                    .on_click(cx.listener(move |t, _, _, cx| {
                        act::biologics_delete(&mut t.app, id);
                        cx.notify();
                    })),
                ),
        );
    }
    if let Some(sel) = app.app.bio_sel {
        if let Some(b) = app.app.biologics.get(sel).cloned() {
            body = body.child(section_title(&format!("{} · {}", b.name, b.format.label())));
            let fmt = b.format;
            body = body.child(
                div()
                    .h(px(160.0))
                    .w_full()
                    .rounded(px(8.0))
                    .bg(theme::PANEL)
                    .border_1()
                    .border_color(theme::BORDER)
                    .child(
                        canvas(
                            move |_, _, _| {},
                            move |bounds, _s, window, _cx| draw_biologic_glyph(fmt, bounds, window),
                        )
                        .size_full(),
                    ),
            );
            body = body.child(widgets::label("Chains", theme::T_SM, theme::FAINT));
            for (i, c) in b.chains.iter().enumerate() {
                body = body.child(
                    row_card()
                        .child(
                            div()
                                .flex_grow()
                                .text_size(px(theme::T_SM))
                                .text_color(theme::TEXT)
                                .child(format!("{} · {}", c.name, c.kind.label())),
                        )
                        .child(widgets::label(
                            &format!("{} aa", c.sequence.len()),
                            theme::T_XS,
                            theme::MUTED,
                        ))
                        .child(widgets::label(
                            &format!("#{}", i + 1),
                            theme::T_XS,
                            theme::FAINT,
                        )),
                );
            }
            if !b.domains.is_empty() {
                body = body.child(widgets::label("Domains", theme::T_SM, theme::FAINT));
                for d in &b.domains {
                    let cname = b
                        .chains
                        .get(d.chain)
                        .map(|c| c.name.clone())
                        .unwrap_or_default();
                    body = body.child(
                        div()
                            .font_family(theme::MONO_FONT)
                            .text_size(px(theme::T_XS))
                            .text_color(theme::MUTED)
                            .child(format!(
                                "{} {} ({}) {}..{}",
                                cname, d.name, d.kind, d.start, d.end
                            )),
                    );
                }
            }
            if let Some(c0) = b.chains.first() {
                let numbered = domain::biologics::number_chain(&c0.sequence, b.numbering);
                let preview: String = numbered
                    .iter()
                    .take(30)
                    .map(|r| format!("{}{}", r.label, r.aa))
                    .collect::<Vec<_>>()
                    .join(" ");
                body = body.child(widgets::label(
                    &format!("Numbering ({}):", b.numbering.label()),
                    theme::T_SM,
                    theme::FAINT,
                ));
                body = body.child(
                    div()
                        .font_family(theme::MONO_FONT)
                        .text_size(px(theme::T_XS))
                        .text_color(theme::TEXT)
                        .child(preview),
                );
            }
        }
    }
    p = p.child(body);
    p.into_any_element()
}

pub fn render_chemistry(app: &mut HelixBench, cx: &mut Context<HelixBench>) -> AnyElement {
    let mut p = page(
        "Chemistry",
        "Compounds, substructure search, reactions, lots, salts",
    );
    let mut body = scroll("chem-scroll");
    let seed = app.app.chem_search.clone();
    let search_inp = app.input_entity(cx, "filter:chem", &seed, "Substructure SMILES…", true);
    app.app.chem_search = search_inp.read(cx).text();
    let query = domain::chemistry::Molecule::parse(&app.app.chem_search).ok();
    body = body.child(
        div()
            .flex()
            .items_center()
            .gap(px(8.0))
            .child(div().w(px(220.0)).child(search_inp))
            .child(
                widgets::button("chem-add", "Add compound", Btn::Primary).on_click(cx.listener(
                    |t, _, _, cx| {
                        act::open_chem_add(&mut t.app);
                        cx.notify();
                    },
                )),
            )
            .child(
                widgets::button("chem-rxn", "Add reaction", Btn::Outline).on_click(cx.listener(
                    |t, _, _, cx| {
                        act::open_chem_reaction(&mut t.app);
                        cx.notify();
                    },
                )),
            ),
    );
    let compounds: Vec<domain::chemistry::Compound> = app
        .app
        .chemistry
        .compounds
        .iter()
        .filter(|c| match &query {
            Some(q) => domain::chemistry::Molecule::parse(&c.smiles)
                .map(|m| m.contains(q))
                .unwrap_or(false),
            None => true,
        })
        .cloned()
        .collect();
    if compounds.is_empty() {
        body = body.child(widgets::label(
            "No compounds match. Add a compound (SMILES) or clear the substructure filter.",
            theme::T_SM,
            theme::FAINT,
        ));
    }
    for c in compounds {
        let id = c.id;
        let selected = app.app.chem_sel == Some(id);
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
                                .child(c.name.clone()),
                        )
                        .child(
                            div()
                                .font_family(theme::MONO_FONT)
                                .text_size(px(theme::T_XS))
                                .text_color(theme::FAINT)
                                .child(c.smiles.clone()),
                        ),
                )
                .child(widgets::label(&c.formula, theme::T_XS, theme::ACCENT))
                .child(widgets::label(
                    &format!("{:.2} g/mol", c.mw),
                    theme::T_XS,
                    theme::MUTED,
                ))
                .child(
                    widgets::button(
                        SharedString::from(format!("chem-sel-{id}")),
                        "Select",
                        if selected { Btn::Primary } else { Btn::Subtle },
                    )
                    .on_click(cx.listener(move |t, _, _, cx| {
                        t.app.chem_sel = Some(id);
                        cx.notify();
                    })),
                )
                .child(
                    widgets::button(
                        SharedString::from(format!("chem-del-{id}")),
                        "Delete",
                        Btn::Ghost,
                    )
                    .on_click(cx.listener(move |t, _, _, cx| {
                        act::chem_delete_compound(&mut t.app, id);
                        cx.notify();
                    })),
                ),
        );
    }
    if let Some(sel) = app.app.chem_sel {
        if let Some(c) = app
            .app
            .chemistry
            .compounds
            .iter()
            .find(|c| c.id == sel)
            .cloned()
        {
            body = body.child(section_title(&format!("{} · {}", c.name, c.formula)));
            if let Ok(m) = domain::chemistry::Molecule::parse(&c.smiles) {
                body = body.child(
                    div()
                        .flex()
                        .gap(px(14.0))
                        .child(widgets::label(
                            &format!("MW {:.2}", m.molecular_weight()),
                            theme::T_SM,
                            theme::TEXT,
                        ))
                        .child(widgets::label(
                            &format!("H-donors {}", m.h_bond_donors()),
                            theme::T_SM,
                            theme::MUTED,
                        ))
                        .child(widgets::label(
                            &format!("H-acceptors {}", m.h_bond_acceptors()),
                            theme::T_SM,
                            theme::MUTED,
                        ))
                        .child(widgets::label(
                            &format!("{} atoms", m.atoms.len()),
                            theme::T_SM,
                            theme::MUTED,
                        )),
                );
            }
            body = body.child(
                widgets::button("chem-lot", "Add lot", Btn::Outline).on_click(cx.listener(
                    |t, _, _, cx| {
                        act::open_chem_lot(&mut t.app);
                        cx.notify();
                    },
                )),
            );
            let lots: Vec<storage::chemistry::Lot> = app
                .app
                .chemistry
                .lots
                .iter()
                .filter(|l| l.compound == sel)
                .cloned()
                .collect();
            let lib = domain::chemistry::salt_solvate_library();
            for l in lots {
                let fname = lib
                    .iter()
                    .find(|f| f.id == l.form)
                    .map(|f| f.name.clone())
                    .unwrap_or_else(|| "free".into());
                body = body.child(
                    div()
                        .font_family(theme::MONO_FONT)
                        .text_size(px(theme::T_XS))
                        .text_color(theme::MUTED)
                        .child(format!("Lot: {} {} · {}", l.quantity, l.unit, fname)),
                );
            }
        }
    }
    body = body.child(section_title("Reactions"));
    if app.app.chemistry.reactions.is_empty() {
        body = body.child(widgets::label(
            "No reactions yet.",
            theme::T_XS,
            theme::FAINT,
        ));
    }
    for r in app.app.chemistry.reactions.clone() {
        let id = r.id;
        let name_of = |cid: u64| {
            app.app
                .chemistry
                .compounds
                .iter()
                .find(|c| c.id == cid)
                .map(|c| c.name.clone())
                .unwrap_or_else(|| "?".into())
        };
        let react: Vec<String> = r
            .reactants
            .iter()
            .map(|x| format!("{} ({:.1} eq)", name_of(x.compound), x.equivalents))
            .collect();
        let prod: Vec<String> = r
            .products
            .iter()
            .map(|x| format!("{} ({:.1} eq)", name_of(x.compound), x.equivalents))
            .collect();
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
                                .child(r.name.clone()),
                        )
                        .child(
                            div()
                                .text_size(px(theme::T_XS))
                                .text_color(theme::FAINT)
                                .child(format!("{} → {}", react.join(" + "), prod.join(" + "))),
                        ),
                )
                .child(
                    widgets::button(
                        SharedString::from(format!("chem-rxdel-{id}")),
                        "Delete",
                        Btn::Ghost,
                    )
                    .on_click(cx.listener(move |t, _, _, cx| {
                        act::chem_delete_reaction(&mut t.app, id);
                        cx.notify();
                    })),
                ),
        );
    }
    body = body.child(section_title("Salt & solvate library"));
    for f in domain::chemistry::salt_solvate_library() {
        body = body.child(
            div()
                .flex()
                .gap(px(8.0))
                .child(
                    div()
                        .flex_grow()
                        .text_size(px(theme::T_XS))
                        .text_color(theme::MUTED)
                        .child(f.name.clone()),
                )
                .child(widgets::label(
                    &format!("{} · {:.2}", f.formula, f.mw),
                    theme::T_XS,
                    theme::FAINT,
                )),
        );
    }
    p = p.child(body);
    p.into_any_element()
}
