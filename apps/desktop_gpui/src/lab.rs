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
use crate::widgets::{self, row_card, section_title, Btn};

fn page(_title: &str, _subtitle: &str) -> gpui::Div {
    div().flex().flex_col().size_full().bg(theme::BG)
}

fn scroll(id: &str) -> gpui::Stateful<gpui::Div> {
    div()
        .id(SharedString::from(id.to_string()))
        .flex_1()
        .min_h(px(0.0))
        .overflow_y_scroll()
        .px(px(theme::PAGE_PAD))
        .py(px(16.0))
        .flex()
        .flex_col()
        .gap(px(8.0))
}

pub fn render_home(app: &mut HelixBench, cx: &mut Context<HelixBench>) -> AnyElement {
    let mut p = page("Home", "Your local workspace at a glance");
    let mut body = scroll("home-scroll").child(first_use_home(app, cx));
    body = body.child(widgets::label(
        &format!(
            "{} open documents · {} Registry entities · {} inventory containers · {} tasks",
            app.app.library.len() + usize::from(!app.app.doc.sequence.is_empty() || app.app.dirty),
            app.app.registry.entries.len(),
            app.app.inventory.containers.len(),
            app.app.workflows.tasks.len()
        ),
        theme::T_SM,
        theme::MUTED,
    ));

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
                        .text_color(theme::ACCENT_TEXT)
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
                                    resume_work(
                                        t,
                                        storage::config::RecentTarget::Record(domain::RecordRef {
                                            store: domain::StoreKind::Notebook,
                                            local_id: id,
                                        }),
                                        cx,
                                    );
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
    if !app.app.doc.sequence.is_empty() || app.app.dirty || app.app.path.is_some() {
        let active_target = storage::config::RecentTarget::Document {
            document: app.app.doc.id,
            path: app.app.path.clone(),
        };
        body = body.child(
            row_card()
                .child(
                    div()
                        .flex_grow()
                        .text_size(px(theme::T_SM))
                        .text_color(theme::TEXT)
                        .child(app.app.doc.name.clone()),
                )
                .child(widgets::label("current", theme::T_XS, theme::ACCENT_TEXT))
                .child(
                    widgets::button("home-resume-current", "Resume", Btn::Outline).on_click(
                        cx.listener(move |t, _, _, cx| resume_work(t, active_target.clone(), cx)),
                    ),
                ),
        );
    }
    for (i, od) in app.app.library.iter().enumerate() {
        let name = od.doc.name.clone();
        let document_id = od.doc.id;
        let document_path = od.path.clone();
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
                        "Resume",
                        Btn::Subtle,
                    )
                    .on_click(cx.listener(move |t, _, _, cx| {
                        resume_work(
                            t,
                            storage::config::RecentTarget::Document {
                                document: document_id,
                                path: document_path.clone(),
                            },
                            cx,
                        );
                    })),
                ),
        );
    }

    body = body.child(section_title("Recent files"));
    if app.app.config.recent_files.is_empty() {
        body = body.child(widgets::label(
            "No recent files yet.",
            theme::T_SM,
            theme::MUTED,
        ));
    }
    for (i, path) in app
        .app
        .config
        .recent_files
        .iter()
        .take(6)
        .cloned()
        .enumerate()
    {
        let open_path = path.clone();
        let forget_path = path.clone();
        body = body.child(
            row_card()
                .child(div().flex_1().child(widgets::label(
                    &path.display().to_string(),
                    theme::T_SM,
                    theme::TEXT,
                )))
                .child(
                    widgets::button(
                        SharedString::from(format!("home-file-open-{i}")),
                        "Open",
                        Btn::Outline,
                    )
                    .on_click(cx.listener(move |t, _, _, cx| {
                        resume_work(
                            t,
                            storage::config::RecentTarget::File(open_path.clone()),
                            cx,
                        )
                    })),
                )
                .child(
                    widgets::button(
                        SharedString::from(format!("home-file-locate-{i}")),
                        "Open another file",
                        Btn::Ghost,
                    )
                    .on_click(cx.listener(|t, _, _, cx| {
                        if let Some(path) =
                            appcore::fs_dialogs::pick_open(t.app.config.last_directory.as_ref())
                        {
                            resume_work(t, storage::config::RecentTarget::File(path), cx);
                        }
                        cx.notify();
                    })),
                )
                .child(
                    widgets::button(
                        SharedString::from(format!("home-file-forget-{i}")),
                        "Forget link",
                        Btn::Ghost,
                    )
                    .on_click(cx.listener(move |t, _, _, cx| {
                        if let Err(error) = appcore::first_use::forget(
                            &mut t.app,
                            &storage::config::RecentTarget::File(forget_path.clone()),
                        ) {
                            t.app.first_use.error = Some(error);
                        }
                        cx.notify();
                    })),
                ),
        );
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
                        .on_click(cx.listener(move |t, _, _, cx| {
                            resume_work(
                                t,
                                storage::config::RecentTarget::Record(domain::RecordRef {
                                    store: domain::StoreKind::Workflow,
                                    local_id: id,
                                }),
                                cx,
                            )
                        })),
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
                        resume_work(
                            t,
                            storage::config::RecentTarget::Record(domain::RecordRef {
                                store: domain::StoreKind::Notebook,
                                local_id: id,
                            }),
                            cx,
                        );
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
                    .font_family(theme::mono_family())
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
                .child(widgets::label(
                    b.format.label(),
                    theme::T_XS,
                    theme::ACCENT_TEXT,
                ))
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
                            .font_family(theme::mono_family())
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
                        .font_family(theme::mono_family())
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

pub fn render_rna(app: &mut HelixBench, cx: &mut Context<HelixBench>) -> AnyElement {
    let mut p = page(
        "RNA / Oligos",
        "Modified oligos: monomer library, patterns, HELM",
    );
    let mut body = scroll("rna-scroll");
    body = body.child(
        div()
            .flex()
            .items_center()
            .gap(px(8.0))
            .child(
                widgets::button("rna-add", "Add oligo", Btn::Primary).on_click(cx.listener(
                    |t, _, _, cx| {
                        act::open_rna_add(&mut t.app);
                        cx.notify();
                    },
                )),
            )
            .child(widgets::label(
                &format!("{} oligos", app.app.rna.oligos.len()),
                theme::T_SM,
                theme::MUTED,
            )),
    );
    if app.app.rna.oligos.is_empty() {
        body = body.child(widgets::label(
            "No oligos yet. Add an RNA sequence with a modification pattern, or import HELM.",
            theme::T_SM,
            theme::FAINT,
        ));
    }
    for o in app.app.rna.oligos.clone() {
        let id = o.id;
        let selected = app.app.rna_sel == Some(id);
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
                                .child(o.name.clone()),
                        )
                        .child(
                            div()
                                .font_family(theme::mono_family())
                                .text_size(px(theme::T_XS))
                                .text_color(theme::FAINT)
                                .child(format!(
                                    "{} nt · {} mods · GC {:.0}%",
                                    o.len(),
                                    o.mods.len(),
                                    o.gc_percent()
                                )),
                        ),
                )
                .child(widgets::label(
                    &format!("{:.1} Da", o.molecular_weight()),
                    theme::T_XS,
                    theme::MUTED,
                ))
                .child(
                    widgets::button(
                        SharedString::from(format!("rna-sel-{id}")),
                        "Select",
                        if selected { Btn::Primary } else { Btn::Subtle },
                    )
                    .on_click(cx.listener(move |t, _, _, cx| {
                        t.app.rna_sel = Some(id);
                        cx.notify();
                    })),
                )
                .child(
                    widgets::button(
                        SharedString::from(format!("rna-del-{id}")),
                        "Delete",
                        Btn::Ghost,
                    )
                    .on_click(cx.listener(move |t, _, _, cx| {
                        act::rna_delete(&mut t.app, id);
                        cx.notify();
                    })),
                ),
        );
    }
    if let Some(sel) = app.app.rna_sel {
        if let Some(o) = app.app.rna.get(sel).cloned() {
            body = body.child(section_title(&format!("{} · {} nt", o.name, o.len())));
            body = body.child(
                div()
                    .font_family(theme::mono_family())
                    .text_size(px(theme::T_SM))
                    .text_color(theme::TEXT)
                    .child(o.seq.clone()),
            );
            body = body.child(widgets::label(
                &format!(
                    "MW {:.1} Da · GC {:.0}%",
                    o.molecular_weight(),
                    o.gc_percent()
                ),
                theme::T_SM,
                theme::MUTED,
            ));
            body = body.child(widgets::label("HELM:", theme::T_SM, theme::FAINT));
            body = body.child(
                div()
                    .font_family(theme::mono_family())
                    .text_size(px(theme::T_XS))
                    .text_color(theme::ACCENT_TEXT)
                    .child(o.to_helm()),
            );
            if !o.mods.is_empty() {
                body = body.child(widgets::label("Modifications:", theme::T_SM, theme::FAINT));
                let lib = domain::rna::monomer_library();
                for m in &o.mods {
                    let name = lib
                        .iter()
                        .find(|x| x.code == m.code)
                        .map(|x| x.name.clone())
                        .unwrap_or_else(|| m.code.clone());
                    body = body.child(
                        div()
                            .font_family(theme::mono_family())
                            .text_size(px(theme::T_XS))
                            .text_color(theme::MUTED)
                            .child(format!("pos {} · {} ({})", m.pos + 1, m.code, name)),
                    );
                }
            }
            body = body.child(widgets::label(
                "Apply bulk pattern (replaces mods at those positions):",
                theme::T_SM,
                theme::FAINT,
            ));
            let mut pats = div().flex().flex_wrap().gap(px(6.0));
            for (i, pat) in domain::rna::Pattern::ALL.iter().enumerate() {
                pats = pats.child(
                    widgets::button(
                        SharedString::from(format!("rna-pat-{i}")),
                        pat.label(),
                        Btn::Outline,
                    )
                    .on_click(cx.listener(move |t, _, _, cx| {
                        act::rna_apply_pattern(&mut t.app, i, 4);
                        cx.notify();
                    })),
                );
            }
            body = body.child(pats);
        }
    }
    body = body.child(section_title("Monomer / modification library"));
    for m in domain::rna::monomer_library() {
        body = body.child(
            div()
                .flex()
                .gap(px(8.0))
                .child(
                    div()
                        .w(px(50.0))
                        .font_family(theme::mono_family())
                        .text_size(px(theme::T_XS))
                        .text_color(theme::ACCENT_TEXT)
                        .child(m.code.clone()),
                )
                .child(
                    div()
                        .flex_grow()
                        .text_size(px(theme::T_XS))
                        .text_color(theme::MUTED)
                        .child(m.name.clone()),
                )
                .child(widgets::label(
                    &format!("{} / {} · {:.1}", m.sugar, m.phosphate, m.mw),
                    theme::T_XS,
                    theme::FAINT,
                )),
        );
    }
    p = p.child(body);
    p.into_any_element()
}

pub fn render_invivo(app: &mut HelixBench, cx: &mut Context<HelixBench>) -> AnyElement {
    let mut p = page(
        "In Vivo",
        "Studies, cohorts, randomization, dosing, welfare",
    );
    let mut body = scroll("iv-scroll");
    body = body.child(
        div()
            .flex()
            .items_center()
            .gap(px(8.0))
            .child(
                widgets::button("iv-study", "New study", Btn::Primary).on_click(cx.listener(
                    |t, _, _, cx| {
                        act::open_iv_study(&mut t.app);
                        cx.notify();
                    },
                )),
            )
            .child(widgets::label(
                &format!("{} studies", app.app.invivo.studies.len()),
                theme::T_SM,
                theme::MUTED,
            )),
    );
    for s in app.app.invivo.studies.clone() {
        let id = s.id;
        let selected = app.app.iv_sel == Some(id);
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
                                .child(s.name.clone()),
                        )
                        .child(
                            div()
                                .text_size(px(theme::T_XS))
                                .text_color(theme::FAINT)
                                .child(format!("{} · {}", s.species, s.strain)),
                        ),
                )
                .child(
                    widgets::button(
                        SharedString::from(format!("iv-sel-{id}")),
                        "Select",
                        if selected { Btn::Primary } else { Btn::Subtle },
                    )
                    .on_click(cx.listener(move |t, _, _, cx| {
                        t.app.iv_sel = Some(id);
                        cx.notify();
                    })),
                )
                .child(
                    widgets::button(
                        SharedString::from(format!("iv-del-{id}")),
                        "Delete",
                        Btn::Ghost,
                    )
                    .on_click(cx.listener(move |t, _, _, cx| {
                        act::iv_delete_study(&mut t.app, id);
                        cx.notify();
                    })),
                ),
        );
    }
    if let Some(sel) = app.app.iv_sel {
        let cohort_ids: Vec<u64> = app
            .app
            .invivo
            .cohorts
            .iter()
            .filter(|c| c.study == sel)
            .map(|c| c.id)
            .collect();
        let animals: Vec<domain::invivo::Animal> = app
            .app
            .invivo
            .animals
            .iter()
            .filter(|a| cohort_ids.contains(&a.cohort))
            .cloned()
            .collect();
        body = body.child(section_title("Animals"));
        body = body.child(
            div()
                .flex()
                .gap(px(6.0))
                .child(
                    widgets::button("iv-animal", "Add animal", Btn::Outline).on_click(cx.listener(
                        |t, _, _, cx| {
                            act::open_iv_animal(&mut t.app);
                            cx.notify();
                        },
                    )),
                )
                .child(
                    widgets::button("iv-measure", "Record measurement", Btn::Outline).on_click(
                        cx.listener(|t, _, _, cx| {
                            act::open_iv_measure(&mut t.app);
                            cx.notify();
                        }),
                    ),
                )
                .child(
                    widgets::button("iv-rand", "Randomize (2 groups)", Btn::Subtle).on_click(
                        cx.listener(|t, _, _, cx| {
                            act::iv_randomize(&mut t.app, 2, 42);
                            cx.notify();
                        }),
                    ),
                ),
        );
        let mut alerts = Vec::new();
        for a in &animals {
            let latest = app
                .app
                .invivo
                .measurements
                .iter()
                .filter(|m| m.animal == a.id)
                .max_by_key(|m| m.day);
            let cur = latest.map(|m| m.weight_g).unwrap_or(a.baseline_weight_g);
            let alert = domain::invivo::welfare_alert(a.baseline_weight_g, cur, 0.2);
            if alert {
                alerts.push(format!(
                    "{}: weight {:.1}→{:.1} g (≥20% loss)",
                    a.tag, a.baseline_weight_g, cur
                ));
            }
            body = body.child(
                row_card()
                    .child(
                        div()
                            .flex_grow()
                            .text_size(px(theme::T_SM))
                            .text_color(theme::TEXT)
                            .child(format!("{} · {} · group {}", a.tag, a.sex, a.group + 1)),
                    )
                    .child(widgets::label(
                        &format!("{:.1} g", cur),
                        theme::T_XS,
                        if alert { theme::DANGER } else { theme::MUTED },
                    )),
            );
        }
        if !alerts.is_empty() {
            body = body.child(section_title("Welfare alerts"));
            for al in alerts {
                body = body.child(
                    div()
                        .text_size(px(theme::T_XS))
                        .text_color(theme::DANGER)
                        .child(al),
                );
            }
        }
        // mean weight per day graph
        let mut by_day: std::collections::BTreeMap<u32, (f32, usize)> =
            std::collections::BTreeMap::new();
        for m in &app.app.invivo.measurements {
            if animals.iter().any(|a| a.id == m.animal) {
                let e = by_day.entry(m.day).or_insert((0.0, 0));
                e.0 += m.weight_g;
                e.1 += 1;
            }
        }
        let series: Vec<(u32, f32)> = by_day
            .iter()
            .map(|(d, (s, n))| (*d, s / *n as f32))
            .collect();
        if !series.is_empty() {
            body = body.child(section_title("Mean weight by day"));
            let ser = series.clone();
            body = body.child(
                div()
                    .h(px(140.0))
                    .w_full()
                    .rounded(px(8.0))
                    .bg(theme::PANEL)
                    .border_1()
                    .border_color(theme::BORDER)
                    .child(
                        canvas(
                            move |_, _, _| {},
                            move |bounds, _s, window, cx| {
                                let maxw = ser.iter().map(|(_, w)| *w).fold(1.0, f32::max);
                                let maxd = ser.iter().map(|(d, _)| *d).max().unwrap_or(1).max(1);
                                let pts: Vec<gpui::Point<gpui::Pixels>> = ser
                                    .iter()
                                    .map(|(d, w)| {
                                        let x = bounds.origin.x.v()
                                            + 20.0
                                            + (*d as f32 / maxd as f32)
                                                * (bounds.size.width.v() - 40.0);
                                        let y = bounds.origin.y.v() + bounds.size.height.v()
                                            - 20.0
                                            - (w / maxw) * (bounds.size.height.v() - 40.0);
                                        gpui::point(px(x), px(y))
                                    })
                                    .collect();
                                draw::polyline(window, &pts, 2.0, theme::ACCENT);
                                for (i, (d, w)) in ser.iter().enumerate() {
                                    draw::disc(window, pts[i], 3.0, theme::ACCENT);
                                    draw::text(
                                        window,
                                        cx,
                                        gpui::point(
                                            pts[i].x,
                                            bounds.origin.y + bounds.size.height - px(8.0),
                                        ),
                                        &format!("d{d}"),
                                        9.0,
                                        theme::MUTED,
                                        theme::ui_font(),
                                        draw::Align::Center,
                                    );
                                    let _ = w;
                                }
                            },
                        )
                        .size_full(),
                    ),
            );
        }
        // dosing calculator
        body = body.child(section_title("Dosing calculator (weight-based)"));
        let dose_seed = app.app.iv_dose.clone();
        let conc_seed = app.app.iv_conc.clone();
        let dose_inp = app.input_entity(cx, "iv:dose", &dose_seed, "mg/kg", true);
        let conc_inp = app.input_entity(cx, "iv:conc", &conc_seed, "mg/mL", true);
        app.app.iv_dose = dose_inp.read(cx).text();
        app.app.iv_conc = conc_inp.read(cx).text();
        let dose: f32 = app.app.iv_dose.trim().parse().unwrap_or(10.0);
        let conc: f32 = app.app.iv_conc.trim().parse().unwrap_or(5.0);
        body = body.child(
            div()
                .flex()
                .items_center()
                .gap(px(8.0))
                .child(div().w(px(80.0)).child(dose_inp))
                .child(widgets::label("mg/kg", theme::T_XS, theme::FAINT))
                .child(div().w(px(80.0)).child(conc_inp))
                .child(widgets::label("mg/mL", theme::T_XS, theme::FAINT)),
        );
        for a in animals.iter().take(6) {
            let latest = app
                .app
                .invivo
                .measurements
                .iter()
                .filter(|m| m.animal == a.id)
                .max_by_key(|m| m.day);
            let w = latest.map(|m| m.weight_g).unwrap_or(a.baseline_weight_g);
            let vol = domain::invivo::dose_volume_ml(dose, w, conc);
            body = body.child(
                div()
                    .font_family(theme::mono_family())
                    .text_size(px(theme::T_XS))
                    .text_color(theme::MUTED)
                    .child(format!("{} ({:.1} g): {:.3} mL", a.tag, w, vol)),
            );
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
                                .font_family(theme::mono_family())
                                .text_size(px(theme::T_XS))
                                .text_color(theme::FAINT)
                                .child(c.smiles.clone()),
                        ),
                )
                .child(widgets::label(&c.formula, theme::T_XS, theme::ACCENT_TEXT))
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
                        .font_family(theme::mono_family())
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

pub fn render_bioprocess(app: &mut HelixBench, cx: &mut Context<HelixBench>) -> AnyElement {
    let mut p = page("Bioprocess", "ISA-88 recipes, runs, worksheets, replicates");
    let mut body = scroll("bp-scroll");
    body = body.child(
        div()
            .flex()
            .items_center()
            .gap(px(8.0))
            .child(
                widgets::button("bp-recipe", "New recipe", Btn::Primary).on_click(cx.listener(
                    |t, _, _, cx| {
                        act::open_bp_recipe(&mut t.app);
                        cx.notify();
                    },
                )),
            )
            .child(
                widgets::button("bp-run", "Plan run", Btn::Outline).on_click(cx.listener(
                    |t, _, _, cx| {
                        let n = t.app.bioprocess.runs.len() + 1;
                        let _ = act::bp_run_create(&mut t.app, 0, &format!("Run-{n}"));
                        cx.notify();
                    },
                )),
            ),
    );
    body = body.child(section_title("Recipes"));
    if app.app.bioprocess.recipes.is_empty() {
        body = body.child(widgets::label(
            "No recipes. Create one (default upstream→downstream template).",
            theme::T_SM,
            theme::FAINT,
        ));
    }
    for r in app.app.bioprocess.recipes.clone() {
        body = body.child(
            row_card()
                .child(
                    div()
                        .flex_grow()
                        .text_size(px(theme::T_SM))
                        .text_color(theme::TEXT)
                        .child(format!("{} · {}", r.name, r.product)),
                )
                .child(widgets::label(
                    &format!("{} ops · {:.0} h", r.ops.len(), r.total_duration_h()),
                    theme::T_XS,
                    theme::MUTED,
                )),
        );
    }
    body = body.child(section_title("Runs"));
    for run in app.app.bioprocess.runs.clone() {
        let id = run.id;
        let selected = app.app.bp_sel == Some(id);
        let mut actions = vec![widgets::button(
            SharedString::from(format!("bp-sel-{id}")),
            "Select",
            if selected { Btn::Primary } else { Btn::Subtle },
        )
        .on_click(cx.listener(move |t, _, _, cx| {
            t.app.bp_sel = Some(id);
            cx.notify();
        }))
        .into_any_element()];
        match run.status {
            domain::bioprocess::RunStatus::Draft => actions.push(
                widgets::button(
                    SharedString::from(format!("bp-start-{id}")),
                    "Start",
                    Btn::Subtle,
                )
                .on_click(cx.listener(move |t, _, _, cx| {
                    act::bp_run_action(&mut t.app, id, "start");
                    cx.notify();
                }))
                .into_any_element(),
            ),
            domain::bioprocess::RunStatus::Executing => {
                actions.push(
                    widgets::button(
                        SharedString::from(format!("bp-done-{id}")),
                        "Complete",
                        Btn::Subtle,
                    )
                    .on_click(cx.listener(move |t, _, _, cx| {
                        act::bp_run_action(&mut t.app, id, "complete");
                        cx.notify();
                    }))
                    .into_any_element(),
                );
                actions.push(
                    widgets::button(
                        SharedString::from(format!("bp-abort-{id}")),
                        "Abort",
                        Btn::Ghost,
                    )
                    .on_click(cx.listener(move |t, _, _, cx| {
                        act::bp_run_action(&mut t.app, id, "abort");
                        cx.notify();
                    }))
                    .into_any_element(),
                );
            }
            _ => {}
        }
        actions.push(
            widgets::button(
                SharedString::from(format!("bp-del-{id}")),
                "Delete",
                Btn::Ghost,
            )
            .on_click(cx.listener(move |t, _, _, cx| {
                act::bp_delete_run(&mut t.app, id);
                cx.notify();
            }))
            .into_any_element(),
        );
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
                                .child(run.name.clone()),
                        )
                        .child(
                            div()
                                .text_size(px(theme::T_XS))
                                .text_color(theme::FAINT)
                                .child(format!(
                                    "{} · {:.0}% · {} reps",
                                    run.status.label(),
                                    run.progress() * 100.0,
                                    run.replicates.len()
                                )),
                        ),
                )
                .children(actions),
        );
    }
    if let Some(sel) = app.app.bp_sel {
        if let Some(run) = app
            .app
            .bioprocess
            .runs
            .iter()
            .find(|r| r.id == sel)
            .cloned()
        {
            body = body.child(section_title(&format!("Worksheet · {}", run.name)));
            for (i, op) in run.ops.iter().enumerate() {
                body = body.child(
                    row_card()
                        .child(
                            div()
                                .flex_grow()
                                .text_size(px(theme::T_SM))
                                .text_color(if op.done { theme::OK } else { theme::TEXT })
                                .child(format!(
                                    "{} · {} ({:.1} h)",
                                    op.name,
                                    op.kind.label(),
                                    op.duration_h
                                )),
                        )
                        .child(
                            widgets::button(
                                SharedString::from(format!("bp-step-{i}")),
                                if op.done { "Done ✓" } else { "Mark done" },
                                if op.done { Btn::Primary } else { Btn::Subtle },
                            )
                            .on_click(cx.listener(
                                move |t, _, _, cx| {
                                    act::bp_step_toggle(&mut t.app, sel, i);
                                    cx.notify();
                                },
                            )),
                        ),
                );
            }
            body = body.child(
                widgets::button("bp-rep", "Add replicate", Btn::Outline).on_click(cx.listener(
                    |t, _, _, cx| {
                        act::open_bp_replicate(&mut t.app);
                        cx.notify();
                    },
                )),
            );
            for rep in &run.replicates {
                let temp = rep.conditions.get("temp_C").cloned().unwrap_or_default();
                let ph = rep.conditions.get("pH").cloned().unwrap_or_default();
                body = body.child(
                    div()
                        .font_family(theme::mono_family())
                        .text_size(px(theme::T_XS))
                        .text_color(theme::MUTED)
                        .child(format!(
                            "{} · {} mg/L · {}°C pH{}",
                            rep.label, rep.titer_mg_l, temp, ph
                        )),
                );
            }
            if !run.replicates.is_empty() {
                body = body.child(widgets::label(
                    &format!("Mean titer {:.1} mg/L", run.mean_titer()),
                    theme::T_SM,
                    theme::ACCENT_TEXT,
                ));
            }
        }
    }
    // cross-run titer comparison
    let series = domain::bioprocess::titer_series(&app.app.bioprocess.runs);
    if !series.is_empty() {
        body = body.child(section_title("Cross-run titer (mg/L)"));
        let ser = series.clone();
        body = body.child(
            div()
                .h(px(140.0))
                .w_full()
                .rounded(px(8.0))
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
    }
    p = p.child(body);
    p.into_any_element()
}

fn resume_work(
    app: &mut HelixBench,
    target: storage::config::RecentTarget,
    cx: &mut Context<HelixBench>,
) {
    if let Err(error) = appcore::first_use::resume(&mut app.app, target) {
        app.app.first_use.error = Some(error);
    }
    cx.notify();
}
fn first_use_home(app: &mut HelixBench, cx: &mut Context<HelixBench>) -> gpui::Div {
    use appcore::first_use as first;
    use storage::config::RecentTarget;
    let mut body = div().flex().flex_col().gap(px(16.0));
    if !app.app.config.first_use_dismissed {
        body = body
            .child(section_title("Start with your sequence"))
            .child(widgets::label(
                "Open your file, understand the input, and run your first analysis.",
                theme::T_BASE,
                theme::MUTED,
            ));
        body=body.child(div().flex().flex_col().gap(px(18.0)).p(px(28.0)).rounded(px(theme::CARD_RADIUS)).bg(theme::PANEL)
        .child(widgets::page_header("Import a sequence","Open GenBank, FASTA or a HelixBench document. Review the recognized sequence before opening it."))
        .child(div().flex().flex_wrap().gap(px(10.0))
            .child(widgets::button("home-import","Import sequence",Btn::Primary).on_click(cx.listener(|t,_,_,cx|{act::guard(&mut t.app,appcore::state::PendingAction::Open);cx.notify();})))
            .child(widgets::button("home-sample","Try a sample",Btn::Outline).on_click(cx.listener(|t,_,_,cx|{if let Err(error)=first::try_sample(&mut t.app){t.app.first_use.error=Some(error);}cx.notify();}))))
        .child(widgets::label("No sequence ready? A sample is labeled and kept separate from your own files.",theme::T_BASE,theme::MUTED)));
        body = body
            .child(section_title("A short path to your first result"))
            .child(widgets::label(
                "Review input → inspect sequence → run analysis → save and find it again.",
                theme::T_BASE,
                theme::MUTED,
            ))
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .gap(px(10.0))
                    .child(
                        widgets::button("home-create", "Create a blank sequence", Btn::Outline)
                            .on_click(cx.listener(|t, _, _, cx| {
                                act::guard(&mut t.app, appcore::state::PendingAction::NewEmpty);
                                cx.notify();
                            })),
                    )
                    .child(
                        widgets::button("home-new-entry", "New notebook entry", Btn::Outline)
                            .on_click(cx.listener(|t, _, _, cx| {
                                if let Err(error) = appcore::commands::execute(
                                    &mut t.app,
                                    appcore::commands::CommandId::NewEntry,
                                ) {
                                    t.app.first_use.error = Some(error);
                                }
                                cx.notify();
                            })),
                    ),
            )
            .child(
                widgets::button(
                    "home-hide-guidance",
                    "Hide introductory guidance",
                    Btn::Ghost,
                )
                .on_click(cx.listener(|t, _, _, cx| {
                    if let Err(error) = first::dismiss_guidance(&mut t.app) {
                        t.app.first_use.error = Some(error);
                    }
                    cx.notify();
                })),
            );
    }
    if let Some(error) = &app.app.first_use.error {
        body = body.child(widgets::label(error, theme::T_BASE, theme::DANGER));
    }
    body = body.child(section_title("Continue working"));
    if app.app.config.recent_work.is_empty() {
        body=body.child(widgets::label("No recent work yet. Open a document, notebook entry, task or saved result to find it here.",theme::T_SM,theme::MUTED));
    }
    for (i, work) in app
        .app
        .config
        .recent_work
        .iter()
        .take(12)
        .cloned()
        .enumerate()
    {
        let target = work.target.clone();
        let forget = work.target.clone();
        let mut row = row_card()
            .child(
                div()
                    .flex_1()
                    .child(widgets::label(&work.title, theme::T_BASE, theme::TEXT))
                    .child(widgets::label(
                        work.target.kind(),
                        theme::T_SM,
                        theme::MUTED,
                    )),
            )
            .child(
                widgets::button(
                    SharedString::from(format!("resume-{i}")),
                    "Resume",
                    Btn::Outline,
                )
                .on_click(cx.listener(move |t, _, _, cx| resume_work(t, target.clone(), cx))),
            );
        if let RecentTarget::Document {
            document,
            path: Some(_),
        } = work.target
        {
            row = row.child(
                widgets::button(
                    SharedString::from(format!("locate-{i}")),
                    "Locate file",
                    Btn::Ghost,
                )
                .on_click(cx.listener(move |t, _, _, cx| {
                    if let Some(path) =
                        appcore::fs_dialogs::pick_open(t.app.config.last_directory.as_ref())
                    {
                        resume_work(
                            t,
                            RecentTarget::Document {
                                document,
                                path: Some(path),
                            },
                            cx,
                        );
                    }
                    cx.notify();
                })),
            );
        }
        row = row.child(
            widgets::button(
                SharedString::from(format!("forget-{i}")),
                "Forget link",
                Btn::Ghost,
            )
            .on_click(cx.listener(move |t, _, _, cx| {
                if let Err(error) = first::forget(&mut t.app, &forget) {
                    t.app.first_use.error = Some(error);
                }
                cx.notify();
            })),
        );
        body = body.child(row);
    }
    body = body.child(
        widgets::button(
            "browse-all-results",
            "Browse and search all saved results",
            Btn::Outline,
        )
        .on_click(cx.listener(|t, _, _, cx| {
            if let Err(e) = appcore::results::browse(&mut t.app) {
                t.app.results.error = Some(e);
            }
            cx.notify();
        })),
    );
    body = body.child(section_title("Saved analysis results")).child(
        widgets::button("refresh-home-results", "Refresh saved results", Btn::Ghost).on_click(
            cx.listener(|t, _, _, cx| {
                first::refresh_results(&mut t.app);
                cx.notify();
            }),
        ),
    );
    if let Some(error) = &app.app.first_use.results_error {
        body = body.child(widgets::label(error, theme::T_BASE, theme::DANGER));
    }
    if app.app.first_use.results.results.is_empty() && app.app.first_use.results_error.is_none() {
        body = body.child(widgets::label(
            "No saved analysis results yet.",
            theme::T_SM,
            theme::MUTED,
        ));
    }
    for result in app.app.first_use.results.results.iter().rev().take(10) {
        let id = result.id;
        body =
            body.child(
                row_card()
                    .child(
                        div()
                            .flex_1()
                            .child(widgets::label(
                                &appcore::results::display_title(result),
                                theme::T_BASE,
                                theme::TEXT,
                            ))
                            .child(widgets::label(
                                &format!(
                                    "Method {} · recorded {}",
                                    result.method_version, result.created
                                ),
                                theme::T_SM,
                                theme::MUTED,
                            )),
                    )
                    .child(
                        widgets::button(
                            SharedString::from(format!("home-result-{id}")),
                            "Open result",
                            Btn::Outline,
                        )
                        .on_click(cx.listener(move |t, _, _, cx| {
                            resume_work(t, RecentTarget::Result(id), cx)
                        })),
                    ),
            );
    }
    body
}

pub fn sample_banner(app: &mut HelixBench, cx: &mut Context<HelixBench>) -> Option<AnyElement> {
    if !appcore::first_use::is_sample(&app.app) {
        return None;
    }
    Some(
        div()
            .flex()
            .flex_wrap()
            .items_center()
            .justify_between()
            .gap(px(10.0))
            .px(px(16.0))
            .py(px(8.0))
            .bg(theme::MINT)
            .child(widgets::label(
                "Sample workspace - separate from your personal files",
                theme::T_SM,
                theme::TEXT,
            ))
            .child(
                div()
                    .flex()
                    .gap(px(8.0))
                    .child(
                        widgets::button("sample-copy", "Make personal copy", Btn::Outline)
                            .on_click(cx.listener(|t, _, _, cx| {
                                if let Err(error) =
                                    appcore::first_use::request_personal_copy(&mut t.app)
                                {
                                    t.app.toast(error);
                                }
                                cx.notify();
                            })),
                    )
                    .child(
                        widgets::button("sample-leave", "Leave sample", Btn::Ghost).on_click(
                            cx.listener(|t, _, _, cx| {
                                if let Err(error) =
                                    appcore::first_use::leave_sample(&mut t.app, false)
                                {
                                    t.app.toast(error);
                                }
                                cx.notify();
                            }),
                        ),
                    ),
            )
            .into_any_element(),
    )
}
