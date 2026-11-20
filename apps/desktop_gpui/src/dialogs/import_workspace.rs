//! File-import review lives in the workspace; the captured draft still owns navigation.
use super::*;
use appcore::import::Molecule;

#[derive(Default)]
pub struct ImportReviewUi {
    operation: Option<domain::OperationId>,
    tab: usize,
}

pub fn import_workspace_active(app: &AppState) -> bool {
    matches!(&app.dialog, Some(Dialog::ImportReview(draft)) if draft.is_workspace())
}

fn property(label: &str, value: &str) -> Div {
    div()
        .flex()
        .gap(px(16.0))
        .py(px(12.0))
        .border_b_1()
        .border_color(theme::BORDER)
        .child(
            widgets::label(label, theme::T_BASE, theme::MUTED)
                .w(px(140.0))
                .flex_shrink_0(),
        )
        .child(
            widgets::label(value, theme::T_BASE, theme::TEXT)
                .flex_1()
                .min_w(px(0.0)),
        )
}

pub fn import_workspace(app: &mut HelixBench, cx: &mut Context<HelixBench>) -> AnyElement {
    // Pull the latest visible text before computing row labels, validation or footer readiness.
    if app.import_ui.tab == 1 {
        if let Some(input) = app.inputs.get("dlg:import:name") {
            let name = input.read(cx).text();
            if let Some(Dialog::ImportReview(draft)) = &mut app.app.dialog {
                if app.import_ui.operation == Some(draft.operation()) {
                    if let Err(error) = draft.sync_name(draft.active, &name) {
                        draft.error = Some(error);
                    }
                }
            }
        }
    }
    let Some(Dialog::ImportReview(draft)) = &app.app.dialog else {
        return div().into_any_element();
    };
    let operation = draft.operation();
    if app.import_ui.operation != Some(operation) {
        app.import_ui = ImportReviewUi {
            operation: Some(operation),
            tab: 0,
        };
    }
    let summary = draft.summary();
    let blocking = draft.validate().err();
    let error = draft.error.clone();
    let active = draft.active.min(summary.total.saturating_sub(1));
    let source_name = draft.preview.source.name.clone();
    let format = draft.preview.format.label();
    let source_size = draft.preview.source.original_bytes().len();
    let ack_changes = draft.acknowledge_changes;
    let ack_skipped = draft.acknowledge_skipped;
    let tab = app.import_ui.tab;
    let rows: Vec<_> = draft
        .preview
        .records
        .iter()
        .enumerate()
        .map(|(i, row)| {
            let valid = row.document.is_some() && row.errors.is_empty();
            let length = row
                .document
                .as_ref()
                .map(|doc| format!("{} bp", doc.sequence.len()))
                .unwrap_or_else(|| {
                    format!(
                        "{} {}",
                        draft.source_facts(i).map_or(0, |facts| facts.residue_count),
                        if row.interpreted == Molecule::Protein {
                            "aa"
                        } else {
                            "symbols"
                        }
                    )
                });
            let status = if !valid {
                "Needs correction".to_owned()
            } else if row.requires_acknowledgment {
                "Review changes".to_owned()
            } else {
                format!(
                    "{} features · no changes",
                    row.document.as_ref().map_or(0, |doc| doc.features.len())
                )
            };
            (
                draft.names[i].clone(),
                row.interpreted.label(),
                length,
                status,
                draft.selected[i],
                valid,
            )
        })
        .collect();
    let mut progress = div().flex().gap(px(12.0)).py(px(12.0)).flex_shrink_0();
    for (index, label) in ["Choose input", "Review records", "Confirm import"]
        .iter()
        .enumerate()
    {
        progress = progress.child(
            div()
                .flex_1()
                .min_w(px(0.0))
                .h(px(48.0))
                .flex()
                .items_center()
                .gap(px(10.0))
                .child(
                    div()
                        .size(px(26.0))
                        .rounded_full()
                        .flex()
                        .items_center()
                        .justify_center()
                        .bg(if index < 2 { theme::MINT } else { theme::PANEL })
                        .text_color(if index < 2 {
                            theme::ACCENT_TEXT
                        } else {
                            theme::MUTED
                        })
                        .child((index + 1).to_string()),
                )
                .child(widgets::label(
                    label,
                    theme::T_BASE,
                    if index == 1 {
                        theme::TEXT
                    } else {
                        theme::MUTED
                    },
                )),
        );
    }
    let notice = div().flex().gap(px(12.0)).p(px(16.0))
        .child(crate::icons::v6_asset("activity", 18.0, 18.0, theme::MUTED))
        .child(div().flex_1().min_w(px(0.0)).flex().flex_col().gap(px(3.0))
            .child(widgets::label("Sequence file ready for review", theme::T_MD, theme::TEXT))
            .child(widgets::label(&format!("{source_name} · {format} · {} records recognized · {source_size} bytes · original file retained", summary.total), theme::T_BASE, theme::MUTED)));
    let mut headings = div()
        .h(px(44.0))
        .flex()
        .items_center()
        .px(px(12.0))
        .gap(px(12.0))
        .bg(theme::BG);
    for (label, width) in [
        ("Include", 76.0),
        ("Record", 0.0),
        ("Molecule", 86.0),
        ("Length", 90.0),
        ("Review", 0.0),
    ] {
        let cell = widgets::label(label, theme::T_SM, theme::MUTED);
        headings = headings.child(if width == 0.0 {
            cell.flex_1().min_w(px(0.0))
        } else {
            cell.w(px(width)).flex_shrink_0()
        });
    }
    let owner = cx.entity();
    let name_width =
        ((app.viewport.width - px(224.0 + 56.0 + 76.0 + 86.0 + 90.0 + 72.0)).v() / 2.0).max(90.0);
    let count = rows.len();
    let list = uniform_list("v6-import-records", count, move |range, window, _cx| {
        range
            .map(|i| {
                let (name, molecule, length, status, selected, valid) = &rows[i];
                let selected = *selected;
                let valid = *valid;
                let include_owner = owner.clone();
                let review_owner = owner.clone();
                let name = crate::views::ellipsize(window, name, name_width, theme::T_BASE, false);
                let status =
                    crate::views::ellipsize(window, status, name_width, theme::T_BASE, false);
                div()
                    .id(("v6-import-row", i))
                    .w_full()
                    .h(px(60.0))
                    .flex()
                    .items_center()
                    .gap(px(12.0))
                    .px(px(12.0))
                    .bg(if i == active {
                        theme::MINT.opacity(0.35)
                    } else {
                        theme::PANEL
                    })
                    .border_b_1()
                    .border_color(theme::BORDER)
                    .child(
                        div()
                            .id(("v6-import-include", i))
                            .w(px(76.0))
                            .h(px(40.0))
                            .flex_shrink_0()
                            .flex()
                            .items_center()
                            .gap(px(6.0))
                            .tab_index(0)
                            .cursor_pointer()
                            .rounded(px(4.0))
                            .focus(|s| s.border_1().border_color(theme::ACCENT_TEXT))
                            .child(check_box(
                                SharedString::from(format!("v6-import-check-{i}")),
                                selected,
                            ))
                            .child(widgets::label(
                                if selected { "Include" } else { "Skip" },
                                theme::T_SM,
                                theme::TEXT,
                            ))
                            .on_click(move |_, _, cx| {
                                include_owner.update(cx, |this, cx| {
                                    if let Some(Dialog::ImportReview(draft)) = &mut this.app.dialog
                                    {
                                        if let Err(error) = draft.set_selected(i, !selected) {
                                            draft.active = i;
                                            draft.error = Some(error);
                                            this.import_ui.tab = 1;
                                        }
                                    }
                                    cx.notify();
                                })
                            }),
                    )
                    .child(
                        widgets::button(
                            SharedString::from(format!("v6-import-record-{i}")),
                            &name,
                            Btn::Ghost,
                        )
                        .tab_index(0)
                        .flex_1()
                        .min_w(px(0.0))
                        .justify_start()
                        .px(px(0.0))
                        .on_click(move |_, _, cx| {
                            review_owner.update(cx, |this, cx| {
                                if let Some(Dialog::ImportReview(draft)) = &mut this.app.dialog {
                                    draft.active = i;
                                }
                                this.import_ui.tab = if valid { 0 } else { 1 };
                                cx.notify();
                            })
                        }),
                    )
                    .child(
                        widgets::label(molecule, theme::T_BASE, theme::MUTED)
                            .w(px(86.0))
                            .flex_shrink_0(),
                    )
                    .child(
                        widgets::label(length, theme::T_BASE, theme::MUTED)
                            .w(px(90.0))
                            .flex_shrink_0(),
                    )
                    .child(
                        widgets::label(
                            &status,
                            theme::T_BASE,
                            if valid { theme::MUTED } else { theme::WARN },
                        )
                        .flex_1()
                        .min_w(px(0.0)),
                    )
                    .into_any_element()
            })
            .collect()
    })
    .h(px((count.clamp(1, 4) * 60) as f32))
    .w_full()
    .track_scroll(app.detect_scroll.clone());
    let table = div()
        .flex_shrink_0()
        .rounded(px(12.0))
        .border_1()
        .border_color(theme::BORDER)
        .shadow(theme::shadow_card())
        .overflow_hidden()
        .child(headings)
        .child(list);
    let Some(Dialog::ImportReview(draft)) = &app.app.dialog else {
        return div().into_any_element();
    };
    let mut tabs = div()
        .flex()
        .gap(px(16.0))
        .border_b_1()
        .border_color(theme::BORDER);
    for (index, label) in ["Overview", "Interpretation", "Original input"]
        .iter()
        .enumerate()
    {
        tabs = tabs.child(
            div()
                .id(("v6-import-tab", index))
                .h(px(40.0))
                .tab_index(0)
                .flex()
                .items_center()
                .cursor_pointer()
                .border_b_2()
                .border_color(if tab == index {
                    theme::ACCENT_TEXT
                } else {
                    theme::transparent()
                })
                .text_color(if tab == index {
                    theme::ACCENT_TEXT
                } else {
                    theme::MUTED
                })
                .focus(|s| s.bg(theme::MINT))
                .child(label.to_string())
                .on_click(cx.listener(move |this, _, _, cx| {
                    this.import_ui.tab = index;
                    cx.notify();
                })),
        );
    }
    let mut detail = div()
        .flex_1()
        .min_w(px(0.0))
        .flex()
        .flex_col()
        .gap(px(10.0))
        .p(px(20.0))
        .bg(theme::PANEL)
        .rounded(px(12.0))
        .border_1()
        .border_color(theme::BORDER)
        .shadow(theme::shadow_card())
        .child(tabs);
    if let Some(record) = draft.preview.records.get(active) {
        if tab == 0 {
            detail = detail
                .child(widgets::label(
                    &draft.names[active],
                    theme::T_SECTION,
                    theme::TEXT,
                ))
                .child(property(
                    "Interpretation",
                    &format!(
                        "{} · {}",
                        record.interpreted.label(),
                        record
                            .document
                            .as_ref()
                            .map_or("Unavailable", |doc| doc.sequence.topology.label())
                    ),
                ))
                .child(property(
                    "Annotations",
                    &record
                        .document
                        .as_ref()
                        .map_or("Unavailable until corrected".into(), |doc| {
                            format!(
                                "{} features · {} primers",
                                doc.features.len(),
                                doc.primers.len()
                            )
                        }),
                ))
                .child(property(
                    "Transformations",
                    &if record.transformations.is_empty() {
                        "None".into()
                    } else {
                        record.transformations.join("; ")
                    },
                ))
                .child(property("Original input", "Retained for provenance"));
            for issue in &record.errors {
                detail = detail.child(error_label(issue));
            }
        } else if tab == 2 {
            let excerpt: String = record.original_record.chars().take(600).collect();
            detail = detail
                .child(widgets::label(
                    "Original input · first 600 characters",
                    theme::T_MD,
                    theme::TEXT,
                ))
                .child(
                    widgets::label(&excerpt, theme::T_SM, theme::MUTED)
                        .font_family(theme::mono_family()),
                )
                .child(
                    widgets::button(
                        "v6-import-copy-original",
                        "Copy complete original record",
                        Btn::Outline,
                    )
                    .tab_index(0)
                    .on_click(cx.listener(|this, _, _, cx| {
                        if let Some(Dialog::ImportReview(draft)) = &this.app.dialog {
                            if let Some(record) = draft.preview.records.get(draft.active) {
                                cx.write_to_clipboard(gpui::ClipboardItem::new_string(
                                    record.original_record.clone(),
                                ));
                            }
                        }
                    })),
                );
        }
    }
    if tab == 1 {
        detail = detail.child(interpretation(app, cx));
    }
    let mut scope = div().w(px(320.0)).flex_shrink_0().flex().flex_col().gap(px(12.0))
        .child(widgets::label("Import scope", theme::T_SECTION, theme::TEXT))
        .child(widgets::label(&format!("{} DNA records will open as independent, unsaved documents. {} records will be excluded. Your current work stays open.", summary.selected, summary.excluded), theme::T_BASE, theme::MUTED));
    for (id, label, checked, needed, changes_ack) in [
        (
            "v6-import-ack-changes",
            "I reviewed the conversions and metadata changes for all included records",
            ack_changes,
            summary.needs_changes_ack,
            true,
        ),
        (
            "v6-import-ack-skipped",
            "I reviewed the excluded records and want to import only the included records",
            ack_skipped,
            summary.needs_skipped_ack,
            false,
        ),
    ] {
        if needed {
            scope = scope.child(
                div()
                    .id(id)
                    .tab_index(0)
                    .min_h(px(40.0))
                    .flex()
                    .items_start()
                    .gap(px(10.0))
                    .cursor_pointer()
                    .focus(|s| s.bg(theme::MINT))
                    .child(
                        check_box(SharedString::from(format!("{id}-check")), checked)
                            .flex_shrink_0(),
                    )
                    .child(
                        widgets::label(label, theme::T_BASE, theme::TEXT)
                            .flex_1()
                            .min_w(px(0.0)),
                    )
                    .on_click(cx.listener(move |this, _, _, cx| {
                        if let Some(Dialog::ImportReview(draft)) = &mut this.app.dialog {
                            if changes_ack {
                                draft.acknowledge_changes = !draft.acknowledge_changes;
                            } else {
                                draft.acknowledge_skipped = !draft.acknowledge_skipped;
                            }
                            draft.invalidate();
                        }
                        cx.notify();
                    })),
            );
        }
    }
    let narrow = app.viewport.width < px(1200.0);
    let details = div()
        .flex()
        .gap(px(24.0))
        .when(narrow, |d| d.flex_col())
        .child(detail)
        .child(scope);
    let mut content = div()
        .id("v6-import-content")
        .flex_1()
        .min_h(px(0.0))
        .overflow_y_scroll()
        .track_scroll(&app.dlg_scroll)
        .px(px(28.0))
        .pb(px(24.0))
        .flex()
        .flex_col()
        .gap(px(20.0))
        .child(notice)
        .child(table)
        .child(details);
    if source_size > 64 * 1024 * 1024 {
        content = content.child(widgets::label("Large input: review memory and processing cost before importing. Cancel retains your original file.", theme::T_BASE, theme::WARN));
    }
    let ready = blocking.is_none();
    let mut commit = widgets::v6_button(
        "v6-import-commit",
        "upload",
        &format!("Import {} sequences", summary.selected),
        Btn::Primary,
    );
    if ready {
        commit = commit
            .tab_index(0)
            .on_click(cx.listener(|this, _, window, cx| {
                if let Err(error) = appcore::import::confirm_import(&mut this.app) {
                    if let Some(Dialog::ImportReview(draft)) = &mut this.app.dialog {
                        draft.error = Some(error);
                    }
                } else {
                    this.focus_handle.focus(window);
                }
                cx.notify();
            }));
    } else {
        commit = commit.opacity(0.45).cursor_default();
    }
    let mut footer = div()
        .flex_shrink_0()
        .px(px(28.0))
        .py(px(16.0))
        .bg(theme::PANEL)
        .border_t_1()
        .border_color(theme::BORDER)
        .flex()
        .flex_col()
        .gap(px(8.0));
    if let Some(message) = error.or(blocking) {
        footer = footer.child(widgets::label(&message, theme::T_BASE, theme::WARN));
    }
    footer = footer.child(
        div()
            .flex()
            .items_center()
            .justify_between()
            .gap(px(16.0))
            .child(
                widgets::button("v6-import-cancel", "Cancel import", Btn::Outline)
                    .tab_index(0)
                    .on_click(cx.listener(|this, _, window, cx| {
                        close_dialog(this, cx);
                        this.focus_handle.focus(window);
                    })),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(12.0))
                    .child(widgets::label(
                        if ready {
                            "Ready · not yet imported"
                        } else {
                            "Review required"
                        },
                        theme::T_SM,
                        theme::MUTED,
                    ))
                    .child(commit),
            ),
    );
    div()
        .size_full()
        .flex()
        .flex_col()
        .bg(theme::BG)
        .child(
            div()
                .px(px(28.0))
                .pt(px(20.0))
                .flex_shrink_0()
                .child(widgets::page_header(
                    "Review imported records",
                    "Original input is retained. Choose exactly what to open.",
                ))
                .child(progress),
        )
        .child(content)
        .child(footer)
        .into_any_element()
}

fn interpretation(app: &mut HelixBench, cx: &mut Context<HelixBench>) -> AnyElement {
    let Some(Dialog::ImportReview(draft)) = &app.app.dialog else {
        return div().into_any_element();
    };
    let active = draft
        .active
        .min(draft.preview.records.len().saturating_sub(1));
    let Some(record) = draft.preview.records.get(active) else {
        return div().into_any_element();
    };
    let seed_name = draft.names[active].clone();
    let row_index = record.index;
    let interpreted = record.interpreted;
    let facts = draft.source_facts(active);
    let contains_u = facts.is_some_and(|facts| facts.contains_u);
    let contains_gaps = facts.is_some_and(|facts| facts.contains_gaps);
    let options = draft.options.get(&row_index).cloned().unwrap_or_default();
    let topology = record
        .document
        .as_ref()
        .map(|doc| doc.sequence.topology)
        .or(options.topology);
    let changes = record.transformations.clone();
    let errors = record.errors.clone();
    let existing_name = app.app.doc.name == seed_name
        || app.app.library.iter().any(|doc| doc.doc.name == seed_name)
        || draft
            .names
            .iter()
            .filter(|name| **name == seed_name)
            .count()
            > 1;
    let field_width = if app.viewport.width < px(1200.0) {
        460.0
    } else {
        (app.viewport.width.v() - 224.0 - 56.0 - 320.0 - 24.0 - 40.0 - 120.0).clamp(180.0, 460.0)
    };
    let name_input = app.input_entity(cx, "dlg:import:name", &seed_name, "Record name", false);
    let mut body = div().flex().flex_col().gap(px(12.0));
    body = body.child(field_row(
        "Name",
        input_field(name_input, field_width).into_any_element(),
    ));
    if existing_name {
        body = body.child(widgets::label("This name is already open. Import creates a new independent copy; it does not overwrite that document.", theme::T_BASE, theme::WARN));
    }
    let mut types = div().flex().gap(px(6.0));
    for molecule in [Molecule::Dna, Molecule::Rna, Molecule::Protein] {
        types = types.child(
            seg(
                SharedString::from(format!("import-type-{}", molecule.label())),
                molecule.label(),
                interpreted == molecule,
            )
            .tab_index(0)
            .on_click(cx.listener(move |this, _, _, cx| {
                if let Some(Dialog::ImportReview(draft)) = &mut this.app.dialog {
                    draft.options.entry(row_index).or_default().molecule = Some(molecule);
                    if let Err(error) = draft.refresh() {
                        draft.error = Some(error);
                    }
                }
                cx.notify();
            })),
        );
    }
    body = body.child(field_row("Interpret as", types.into_any_element()));
    let mut topologies = div().flex().gap(px(6.0));
    for choice in [Topology::Linear, Topology::Circular] {
        topologies = topologies.child(
            seg(
                SharedString::from(format!("import-topology-{}", choice.label())),
                choice.label(),
                topology == Some(choice),
            )
            .tab_index(0)
            .on_click(cx.listener(move |this, _, _, cx| {
                if let Some(Dialog::ImportReview(draft)) = &mut this.app.dialog {
                    draft.options.entry(row_index).or_default().topology = Some(choice);
                    if let Err(error) = draft.refresh() {
                        draft.error = Some(error);
                    }
                }
                cx.notify();
            })),
        );
    }
    body = body.child(field_row("Topology", topologies.into_any_element()));
    if contains_u {
        body = body.child(
            div()
                .flex()
                .items_center()
                .gap(px(8.0))
                .child(
                    check_box("import-convert-rna", options.convert_rna_to_dna)
                        .tab_index(0)
                        .on_click(cx.listener(move |this, _, _, cx| {
                            if let Some(Dialog::ImportReview(draft)) = &mut this.app.dialog {
                                let options = draft.options.entry(row_index).or_default();
                                options.convert_rna_to_dna = !options.convert_rna_to_dna;
                                if let Err(error) = draft.refresh() {
                                    draft.error = Some(error);
                                }
                            }
                            cx.notify();
                        })),
                )
                .child(widgets::label(
                    "Allow U→T conversion when interpreting this record as DNA",
                    theme::T_BASE,
                    theme::TEXT,
                )),
        );
    }
    if contains_gaps {
        body = body.child(
            div()
                .flex()
                .items_center()
                .gap(px(8.0))
                .child(
                    check_box("import-remove-gaps", options.remove_gaps)
                        .tab_index(0)
                        .on_click(cx.listener(move |this, _, _, cx| {
                            if let Some(Dialog::ImportReview(draft)) = &mut this.app.dialog {
                                let options = draft.options.entry(row_index).or_default();
                                options.remove_gaps = !options.remove_gaps;
                                if let Err(error) = draft.refresh() {
                                    draft.error = Some(error);
                                }
                            }
                            cx.notify();
                        })),
                )
                .child(widgets::label(
                    "Remove gap characters (annotated GenBank requires source correction)",
                    theme::T_BASE,
                    theme::TEXT,
                )),
        );
    }
    for change in changes {
        body = body.child(widgets::label(&change, theme::T_BASE, theme::MUTED));
    }
    for issue in errors {
        body = body.child(error_label(&issue));
    }
    body.child(widgets::label("Protein: use Biologics. RNA: use RNA / Oligos, or explicitly review a DNA conversion here. Unsupported residues are never removed to manufacture DNA.", theme::T_BASE, theme::MUTED)).into_any_element()
}
