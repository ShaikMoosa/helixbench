//! Native Soft Gallery shell, backed by typed live selection and shared commands.
use crate::draw::PxVal;
use crate::{
    app::HelixBench,
    icons, theme,
    widgets::{self, Btn},
};
use appcore::{
    actions, commands, first_use,
    shell::{self, InspectorPlacement, PanelContext, PanelTab},
    state::{Dialog, MapMode, View},
};
use gpui::{div, img, prelude::*, px, AnyElement, Context, SharedString, Window};

fn label(text: &str, size: f32) -> gpui::Div {
    widgets::label(text, size, theme::TEXT)
}
fn property(name: &str, value: String) -> gpui::Div {
    div()
        .flex()
        .flex_col()
        .gap(px(3.0))
        .child(widgets::label(name, theme::T_SM, theme::MUTED))
        .child(label(&value, theme::T_BASE))
}
pub fn navigation(app: &mut HelixBench, cx: &mut Context<HelixBench>) -> AnyElement {
    let logo = if app.app.view == View::Home {
        "gallery/logo-home.svg"
    } else {
        "gallery/logo-workspace.svg"
    };
    let mut body = div()
        .id("gallery-navigation-scroll")
        .flex_1()
        .min_h(px(0.0))
        .overflow_y_scroll()
        .flex()
        .flex_col()
        .gap(px(7.0));
    body = body.child(
        widgets::label("WORKSPACE", theme::T_XS, theme::NAV_GROUP_TEXT)
            .font_weight(gpui::FontWeight::SEMIBOLD)
            .mt(px(16.0))
            .mb(px(6.0)),
    );
    let primary = [
        View::Home,
        View::Notebook,
        View::Registry,
        View::Inventory,
        View::Workflows,
        View::Insights,
        View::Automation,
        View::Split,
    ];
    for view in primary {
        body = body.child(nav_item(app, view, cx));
    }
    body = body.child(
        widgets::button(
            "gallery-all-modules",
            if app.app.shell.all_modules {
                "Fewer modules"
            } else {
                "All modules"
            },
            Btn::Outline,
        )
        .w_full()
        .on_click(cx.listener(|t, _, _, cx| {
            t.app.shell.all_modules = !t.app.shell.all_modules;
            cx.notify();
        })),
    );
    if app.app.shell.all_modules {
        for view in [
            View::Biologics,
            View::Chemistry,
            View::Rna,
            View::InVivo,
            View::Bioprocess,
        ] {
            body = body.child(nav_item(app, view, cx));
        }
    }
    body = body.child(widgets::label("Active sequence", theme::T_SM, theme::MUTED).mt(px(14.0)));
    let id = app.app.doc.id;
    let topology = if app.app.doc.sequence.is_circular() {
        "circular"
    } else {
        "linear"
    };
    body = body.child(
        div()
            .id("gallery-active-sequence")
            .flex()
            .flex_col()
            .gap(px(3.0))
            .p(px(10.0))
            .rounded(px(12.0))
            .bg(if View::ALL.contains(&app.app.view) {
                theme::MINT
            } else {
                theme::BG
            })
            .child(label(&app.app.doc.name, theme::T_BASE).overflow_hidden())
            .child(widgets::label(
                &format!("{} bp · {topology}", app.app.doc.sequence.len()),
                theme::T_SM,
                theme::MUTED,
            ))
            .cursor_pointer()
            .on_click(cx.listener(move |t, _, _, cx| {
                if let Err(error) = appcore::navigation::request_navigation(
                    &mut t.app,
                    appcore::navigation::NavigationIntent::Navigate(appcore::navigation::Route {
                        view: View::Map,
                        document: id,
                        record: None,
                    }),
                ) {
                    t.app.toast(error);
                }
                cx.notify();
            })),
    );
    div()
        .w(px(theme::NAV_WIDTH))
        .flex_shrink_0()
        .h_full()
        .flex()
        .flex_col()
        .gap(px(7.0))
        .p(px(16.0))
        .bg(theme::PANEL)
        .child(
            div()
                .h(px(64.0))
                .flex_shrink_0()
                .flex()
                .items_center()
                .gap(px(8.0))
                .child(img(logo).w(px(34.0)).h(px(34.0)))
                .child(label("HelixBench", 15.0).font_weight(gpui::FontWeight::MEDIUM)),
        )
        .child(body)
        .child(
            widgets::label(
                if first_use::is_sample(&app.app) {
                    "Temporary sample"
                } else {
                    "Local workspace"
                },
                theme::T_SM,
                theme::MUTED,
            )
            .mt(px(8.0)),
        )
        .child(
            widgets::button("gallery-settings", "Settings & help", Btn::Outline)
                .w_full()
                .on_click(cx.listener(|t, _, _, cx| {
                    if t.app.dialog.is_none() {
                        t.app.dialog = Some(Dialog::Options);
                    } else {
                        t.app.toast("Apply or cancel the open form first");
                    }
                    cx.notify();
                })),
        )
        .into_any_element()
}
fn nav_item(
    app: &HelixBench,
    view: View,
    cx: &mut Context<HelixBench>,
) -> gpui::Stateful<gpui::Div> {
    let active = app.app.view == view && !app.app.workspace_search.open;
    div()
        .id(SharedString::from(format!("gallery-nav-{view:?}")))
        .w_full()
        .h(px(40.0))
        .flex_shrink_0()
        .px(px(8.0))
        .rounded(px(8.0))
        .border_l(px(3.0))
        .border_color(if active {
            theme::SELECT
        } else {
            theme::transparent()
        })
        .flex()
        .items_center()
        .gap(px(10.0))
        .bg(if active {
            theme::MINT
        } else {
            theme::transparent()
        })
        .text_color(if active {
            theme::ACCENT_TEXT
        } else {
            theme::NAV_TEXT
        })
        .child(icons::gallery_view(
            view,
            if active {
                theme::ACCENT_TEXT
            } else {
                theme::MUTED
            },
        ))
        .child(widgets::label(
            view.label(),
            theme::T_BASE,
            if active {
                theme::ACCENT_TEXT
            } else {
                theme::NAV_TEXT
            },
        ))
        .cursor_pointer()
        .hover(|s| s.bg(theme::FILL_HOVER))
        .on_click(cx.listener(move |t, _, _, cx| t.set_view(view, cx)))
}
pub fn page_header(app: &mut HelixBench, cx: &mut Context<HelixBench>) -> AnyElement {
    let layout = app.app.shell.layout(app.viewport.width.v());
    let title = if app.app.workspace_search.open {
        "Search workspace"
    } else if app.app.first_use.selected_result.is_some() && app.app.view == View::Home {
        "Saved result"
    } else {
        app.app.view.label()
    };
    let subtitle = match app.app.view {
        View::Home => "Continue your work and resolve what needs attention.",
        View::Map => "Inspect a construct and act on the selected region.",
        View::Sequence => "Read, find and edit the active DNA sequence.",
        View::Features => "Inspect annotations and their exact sequence ranges.",
        View::Primers => "Review primers and design against your sequence.",
        View::Enzymes => "Inspect restriction sites and choose an analysis.",
        _ => "Your local workspace. Select an item for details and available tools.",
    };
    let mut actions_bar = div()
        .w(px(layout.header_actions_width))
        .flex_shrink_0()
        .flex()
        .flex_wrap()
        .justify_end()
        .gap(px(8.0))
        .child(
            widgets::button("gallery-search", "Search", Btn::Outline).on_click(cx.listener(
                |t, _, _, cx| {
                    if let Err(error) = appcore::workspace_search::open(&mut t.app) {
                        t.app.toast(error);
                    }
                    cx.notify();
                },
            )),
        )
        .child(
            widgets::button("gallery-inspector-toggle", "Details & tools", Btn::Outline)
                .track_focus(&app.inspector_toggle_focus)
                .on_click(cx.listener(|t, _, window, cx| toggle_inspector(t, window, cx))),
        );
    if View::ALL.contains(&app.app.view) {
        actions_bar = actions_bar.child(
            widgets::button("gallery-save", "Save", Btn::Primary).on_click(cx.listener(
                |t, _, _, cx| {
                    actions::save(&mut t.app);
                    cx.notify();
                },
            )),
        );
    } else {
        actions_bar = actions_bar.child(
            widgets::button("gallery-new", "New", Btn::Primary).on_click(cx.listener(
                |t, _, _, cx| {
                    actions::guard(&mut t.app, appcore::state::PendingAction::NewEmpty);
                    cx.notify();
                },
            )),
        );
    }
    div()
        .flex()
        .flex_wrap()
        .items_center()
        .justify_between()
        .gap(px(layout.gap))
        .px(px(layout.page_padding))
        .py(px(20.0))
        .child(
            div()
                .flex_1()
                .min_w(px(layout.title_min_width))
                .flex()
                .flex_col()
                .gap(px(6.0))
                .child(
                    label(title, theme::T_TITLE)
                        .line_height(px(40.0))
                        .font_weight(gpui::FontWeight::MEDIUM),
                )
                .child(widgets::label(subtitle, theme::T_BASE, theme::MUTED)),
        )
        .child(actions_bar)
        .into_any_element()
}
pub fn toggle_inspector(app: &mut HelixBench, window: &mut Window, cx: &mut Context<HelixBench>) {
    if app.app.dialog.is_some() || app.app.navigation.pending.is_some() {
        app.app.toast("Apply or cancel the open form first");
        cx.notify();
        return;
    }
    if app.app.shell.layout(app.viewport.width.v()).inspector == InspectorPlacement::Hidden {
        app.app.shell.open(shell::FocusReturn::InspectorToggle);
    } else {
        close_inspector(app, window);
    }
    cx.notify();
}
pub fn inspector(app: &mut HelixBench, cx: &mut Context<HelixBench>) -> AnyElement {
    let context = shell::context(&app.app);
    let mut body = div()
        .w_full()
        .max_w(px(272.0))
        .min_w(px(0.0))
        .flex()
        .flex_col()
        .gap(px(14.0));
    let mut tabs = div().flex().gap(px(4.0));
    for (tab, name) in [
        (PanelTab::Details, "Details"),
        (PanelTab::Tools, "Tools"),
        (PanelTab::Help, "Help"),
    ] {
        tabs = tabs.child(
            widgets::button(
                SharedString::from(format!("panel-tab-{tab:?}")),
                name,
                if app.app.shell.tab == tab {
                    Btn::Subtle
                } else {
                    Btn::Ghost
                },
            )
            .on_click(cx.listener(move |t, _, _, cx| {
                t.app.shell.tab = tab;
                cx.notify();
            })),
        );
    }
    body = body.child(tabs);
    if context == PanelContext::WorkspaceSearch {
        body=body.child(label("Workspace search",theme::T_LG)).child(property("Scope",app.app.workspace_search.query.scope.label().into())).child(property("Query",app.app.workspace_search.query.text.clone())).child(widgets::label("Workspace search finds loaded records and tools. Ctrl+F searches DNA in the active sequence.",theme::T_BASE,theme::MUTED));
    } else if app.app.shell.tab == PanelTab::Help {
        body=body.child(label(&format!("How to use {}",app.app.view.label()),theme::T_LG)).child(widgets::label("Select an item to inspect its identity and available actions. Tools explain their prerequisites. Use Search or Ctrl+K to find tools and records. Use the labeled sequence tabs to switch views; a view change does not change DNA topology.",theme::T_BASE,theme::MUTED));
    } else if app.app.view == View::Home
        && app.app.first_use.selected_result.is_none()
        && app.app.shell.tab == PanelTab::Details
    {
        body=body.child(label("Your first task",theme::T_SECTION)).child(widgets::label("Import your own sequence, or explore a labeled sample. Your personal workspace remains separate from sample work.",theme::T_MD,theme::MUTED))
            .child(widgets::button("inspector-import","Import sequence",Btn::Outline).w_full().on_click(cx.listener(|t,_,_,cx|{actions::guard(&mut t.app,appcore::state::PendingAction::Open);cx.notify();})))
            .child(widgets::button("inspector-sample","Try a sample",Btn::Outline).w_full().on_click(cx.listener(|t,_,_,cx|{if let Err(error)=first_use::try_sample(&mut t.app){t.app.toast(error);}cx.notify();})))
            .child(widgets::button("inspector-task-help","Task help",Btn::Outline).w_full().on_click(cx.listener(|t,_,_,cx|{t.app.shell.tab=PanelTab::Help;cx.notify();})));
    } else {
        body = body.child(context_properties(app, context, cx));
        if app.app.shell.tab == PanelTab::Details {
            if let PanelContext::Primer { primer, .. } = context {
                body = body.child(
                    widgets::button("inspector-edit-primer", "Edit primer", Btn::Outline)
                        .w_full()
                        .on_click(cx.listener(move |t, _, _, cx| {
                            if let Err(error) = shell::validate_context(&t.app, &context) {
                                t.app.toast(error);
                            } else {
                                crate::dialogs::open_primer_editor(&mut t.app, Some(primer));
                            }
                            cx.notify();
                        })),
                );
            }
            let active_feature = matches!(context, PanelContext::Feature { .. });
            if active_feature {
                body = body.child(
                    widgets::button("inspector-edit-feature", "Edit feature", Btn::Outline)
                        .w_full()
                        .on_click(cx.listener(move |t, _, _, cx| {
                            if let Err(error) = shell::validate_context(&t.app, &context) {
                                t.app.toast(error);
                            } else if let PanelContext::Feature { feature, .. } = context {
                                crate::dialogs::open_feature_editor(&mut t.app, Some(feature));
                            }
                            cx.notify();
                        })),
                );
            }
            if matches!(
                context,
                PanelContext::Range { .. } | PanelContext::Feature { .. }
            ) {
                let expected_range = app.app.selection;
                body = body
                    .child(
                        widgets::button(
                            "inspector-add-primer",
                            "Add primer from range",
                            Btn::Outline,
                        )
                        .w_full()
                        .on_click(cx.listener(move |t, _, _, cx| {
                            if let Err(error) = shell::validate_context(&t.app, &context).and_then(|_|if t.app.selection==expected_range{Ok(())}else{Err("The selected range changed; review it before creating a primer".into())}) {
                                t.app.toast(error);
                            } else {
                                let range = match shell::primer_range(&t.app, &context) {
                                    Ok(range) => range,
                                    Err(error) => { t.app.toast(error); cx.notify(); return; }
                                };
                                let sequence = String::from_utf8_lossy(&t.app.doc.sequence.region(range)).into_owned();
                                crate::dialogs::open_primer_editor(&mut t.app, None);
                                if let Some(Dialog::PrimerEditor(draft)) = &mut t.app.dialog {
                                    draft.sequence = sequence;
                                }
                            }
                            cx.notify();
                        })),
                    )
                    .child(widgets::label(
                        "New primers use the top-strand range; review the sequence before saving.",
                        theme::T_SM,
                        theme::MUTED,
                    ));
            }
        }
        if app.app.shell.tab == PanelTab::Tools
            && matches!(app.app.view, View::Map | View::Sequence)
        {
            body = body.child(label("Display", theme::T_MD));
            for toggle in appcore::selection::DisplayToggle::ALL {
                let text = format!(
                    "{} · {}",
                    toggle.label(),
                    if toggle.enabled(&app.app) {
                        "On"
                    } else {
                        "Off"
                    }
                );
                body = body.child(
                    widgets::button(
                        SharedString::from(format!("display-{}", toggle.label())),
                        &text,
                        Btn::Outline,
                    )
                    .w_full()
                    .on_click(cx.listener(move |t, _, _, cx| {
                        appcore::selection::toggle_display(&mut t.app, toggle);
                        cx.notify();
                    })),
                );
            }
            body=body.child(widgets::label("Display settings change the view only. Copy uses top-strand coordinates; choose reverse complement explicitly when needed.",theme::T_SM,theme::MUTED));
        }
        body = body.child(label("Tools", theme::T_MD));
        let mut tools = commands::owning_page(app.app.view);
        if matches!(
            app.app.view,
            View::Map | View::Sequence | View::Enzymes | View::Primers
        ) {
            for id in [
                commands::CommandId::Digest,
                commands::CommandId::Pcr,
                commands::CommandId::PrimerDesign,
            ] {
                if let Some(tool) = commands::tools().iter().find(|d| d.id == id) {
                    if !tools.iter().any(|d| d.id == id) {
                        tools.push(*tool);
                    }
                }
            }
        }
        for descriptor in tools {
            let id = descriptor.id;
            let available = commands::availability(&app.app, id);
            body = body.child(
                widgets::button(
                    SharedString::from(format!("inspector-command-{id:?}")),
                    descriptor.label,
                    Btn::Outline,
                )
                .w_full()
                .opacity(if available.enabled() { 1.0 } else { 0.55 })
                .on_click(cx.listener(move |t, _, _, cx| {
                    if let Err(error) = shell::validate_context(&t.app, &context)
                        .and_then(|_| commands::execute(&mut t.app, id))
                    {
                        t.app.toast(error);
                    }
                    cx.notify();
                })),
            );
            if !available.enabled() {
                body = body.child(widgets::label(
                    available.reason(),
                    theme::T_SM,
                    theme::MUTED,
                ));
            }
        }
        if View::ALL.contains(&app.app.view) {
            body = body.child(label("Related records", theme::T_MD));
            let related: Vec<_> = app
                .app
                .registry
                .entries
                .iter()
                .filter(|entry| entry.linked_document == Some(app.app.doc.id))
                .map(|entry| (entry.id, entry.name().to_string()))
                .collect();
            if related.is_empty() {
                body = body.child(widgets::label(
                    "No linked Registry records.",
                    theme::T_SM,
                    theme::MUTED,
                ));
            }
            for (id, name) in related {
                body = body.child(
                    widgets::button(
                        SharedString::from(format!("inspector-related-{id}")),
                        &name,
                        Btn::Subtle,
                    )
                    .w_full()
                    .child(icons::gallery_related())
                    .on_click(cx.listener(move |t, _, _, cx| {
                        if let Err(error) = first_use::resume(
                            &mut t.app,
                            storage::config::RecentTarget::Record(domain::RecordRef {
                                store: domain::StoreKind::Registry,
                                local_id: id,
                            }),
                        ) {
                            t.app.toast(error);
                        }
                        cx.notify();
                    })),
                );
            }
        }
    }
    div()
        .w(px(theme::INSPECTOR_WIDTH))
        .flex_shrink_0()
        .h_full()
        .flex()
        .flex_col()
        .gap(px(14.0))
        .p(px(theme::PANEL_PAD))
        .bg(theme::PANEL)
        .border_l_1()
        .border_color(theme::BORDER)
        .child(
            div()
                .flex()
                .items_center()
                .justify_between()
                .gap(px(8.0))
                .child(label("Details & tools", 17.0).font_weight(gpui::FontWeight::MEDIUM))
                .child(icons::gallery_sliders())
                .child(
                    widgets::button("inspector-close", "Close", Btn::Ghost).on_click(cx.listener(
                        |t, _, window, cx| {
                            close_inspector(t, window);
                            cx.notify();
                        },
                    )),
                ),
        )
        .child(
            div()
                .id("gallery-inspector-scroll")
                .w_full()
                .min_w(px(0.0))
                .flex_1()
                .min_h(px(0.0))
                .overflow_y_scroll()
                .child(body),
        )
        .into_any_element()
}
fn context_properties(
    app: &HelixBench,
    context: PanelContext,
    cx: &mut Context<HelixBench>,
) -> gpui::Div {
    let mut body = div()
        .w_full()
        .min_w(px(0.0))
        .flex()
        .flex_col()
        .gap(px(14.0));
    let title = match context {
        PanelContext::Feature { feature, .. } => app
            .app
            .doc
            .feature(feature)
            .map(|f| f.name.clone())
            .unwrap_or_default(),
        PanelContext::Primer { primer, .. } => app
            .app
            .doc
            .primer(primer)
            .map(|p| p.name.clone())
            .unwrap_or_default(),
        PanelContext::Record(record) => workspace_hit(app, record)
            .map(|h| h.title)
            .unwrap_or_else(|| "Unavailable record".into()),
        PanelContext::Result(id) => format!("Saved result {id}"),
        _ => app.app.doc.name.clone(),
    };
    body = body.child(
        div()
            .p(px(12.0))
            .rounded(px(12.0))
            .bg(theme::BG)
            .child(widgets::label(
                "Current selection",
                theme::T_SM,
                theme::MUTED,
            ))
            .child(label(&title, 15.0)),
    );
    if shell::context_document(context).is_some() {
        body = body
            .child(property("Document", app.app.doc.id.to_string()))
            .child(property("Revision", app.app.doc.revision_id.to_string()))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(4.0))
                    .child(widgets::label("Status", theme::T_SM, theme::MUTED))
                    .child(widgets::document_status(app.app.dirty)),
            );
        if let Some(provenance) = &app.app.doc.pcr_source {
            let parent = provenance.parent;
            body=body.child(property("Derived by","PCR · independent product".into()))
                .child(property("Parent version",format!("{} @ {}",parent.document,parent.revision)))
                .child(property("Captured primers",format!("F: {} · ID {:?}; R: {} · ID {:?}",provenance.forward.name,provenance.forward.id,provenance.reverse.name,provenance.reverse.id)))
                .child(property("Copied features",if provenance.copy_features{"Contained visible spans; joined order retained"}else{"None"}.into()))
                .child(widgets::button("pcr-parent-open","Open parent template",Btn::Outline).w_full().on_click(cx.listener(move|t,_,_,cx|{
                    let available=t.app.library.iter().any(|doc|doc.doc.source_ref()==parent);
                    if let Err(error)=shell::validate_context(&t.app,&context){t.app.toast(error);}else if available{if let Err(error)=appcore::navigation::request_navigation(&mut t.app,appcore::navigation::NavigationIntent::SwitchDocument(parent.document)){t.app.toast(error);}}else{t.app.toast("The recorded parent version is not currently open; its provenance remains recorded here");}cx.notify();
                })));
        }
        if let PanelContext::Primer { primer, .. } = context {
            if let Some(pair) = app
                .app
                .doc
                .primer_pairs
                .iter()
                .rev()
                .find(|pair| pair.forward.id == Some(primer) || pair.reverse.id == Some(primer))
            {
                let intact = [&pair.forward, &pair.reverse].iter().all(|input| {
                    input
                        .id
                        .and_then(|id| app.app.doc.primer(id))
                        .is_some_and(|primer| primer.sequence == input.sequence)
                });
                body = body
                    .child(property(
                        "Recorded design pair",
                        format!("{} / {}", pair.forward.name, pair.reverse.name),
                    ))
                    .child(property(
                        "Pair status",
                        if intact {
                            "Members match captured sequences"
                        } else {
                            "Review needed: a member changed or was removed"
                        }
                        .into(),
                    ))
                    .child(property(
                        "Design target (captured)",
                        format!(
                            "{}–{} · {} nt",
                            pair.target.start + 1,
                            (pair.target.start + pair.target.len - 1) % pair.template_length.max(1)
                                + 1,
                            pair.target.len
                        ),
                    ));
            }
        }
        body = body.child(property(
            "Selected range",
            appcore::selection::summary(&app.app),
        ));
        if let PanelContext::Feature { feature, .. } = context {
            if let Some(f) = app.app.doc.feature(feature) {
                body = body
                    .child(property("Type", f.kind.clone()))
                    .child(property("Direction", format!("{:?}", f.direction)))
                    .child(property("Note", f.note.clone()));
            }
        }
        body = body.child(property(
            "DNA topology",
            if app.app.doc.sequence.is_circular() {
                "Circular"
            } else {
                "Linear"
            }
            .into(),
        ));
        if app.app.view == View::Map {
            let mut choices = div().flex().flex_wrap().gap(px(4.0));
            for (mode, name) in [
                (MapMode::Auto, "Auto"),
                (MapMode::Plasmid, "Circular view"),
                (MapMode::Linear, "Linear view"),
                (MapMode::Metadata, "Metadata"),
            ] {
                choices = choices.child(
                    widgets::button(
                        SharedString::from(format!("representation-{name}")),
                        name,
                        Btn::Subtle,
                    )
                    .on_click(cx.listener(move |t, _, _, cx| {
                        shell::set_representation(&mut t.app, mode);
                        cx.notify();
                    })),
                );
            }
            body = body
                .child(label("View representation", theme::T_MD))
                .child(choices)
                .child(widgets::label(
                    "Changing the view leaves DNA topology unchanged.",
                    theme::T_SM,
                    theme::MUTED,
                ));
        }
    } else if let PanelContext::Record(record) = context {
        if let Some(hit) = workspace_hit(app, record) {
            body = body
                .child(property(
                    "Record identity",
                    format!("{:?}:{}", record.store, record.local_id),
                ))
                .child(label(&hit.detail, theme::T_BASE));
        }
    }
    body
}
fn workspace_hit(
    app: &HelixBench,
    record: domain::RecordRef,
) -> Option<appcore::workspace_search::SearchHit> {
    appcore::workspace_search::lookup(
        &app.app,
        appcore::workspace_search::SearchTarget::Record(record),
    )
}

pub fn open_page_tools(app: &mut HelixBench, cx: &mut Context<HelixBench>) {
    if app.app.dialog.is_some() || app.app.navigation.pending.is_some() {
        app.app.toast("Apply or cancel the open form first");
    } else {
        app.app.shell.tab = PanelTab::Tools;
        app.app.shell.open(shell::FocusReturn::PageTools);
    }
    cx.notify();
}
pub fn close_inspector(app: &mut HelixBench, window: &mut Window) {
    match app.app.shell.close() {
        shell::FocusReturn::InspectorToggle => app.inspector_toggle_focus.focus(window),
        shell::FocusReturn::PageTools => app.page_tools_focus.focus(window),
        shell::FocusReturn::Editor => app.focus_handle.focus(window),
    }
}
