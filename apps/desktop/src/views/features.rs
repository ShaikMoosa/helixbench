//! Features view: sortable annotation table with visibility toggles.

use egui_extras::{Column, TableBuilder};

use crate::state::{AppState, SortDir};
use crate::theme;

pub fn show(app: &mut AppState, ui: &mut egui::Ui) {
    app.ensure_analysis();
    let l = app.doc.sequence.len();

    ui.horizontal(|ui| {
        ui.add(
            egui::TextEdit::singleline(&mut app.feature_filter)
                .hint_text("Filter features…")
                .desired_width(180.0),
        );
        if ui
            .button("＋ Add Feature")
            .on_hover_text("Annotate the current selection")
            .clicked()
        {
            crate::dialogs::open_feature_editor(app, None);
        }
        let sel = app.selected_feature;
        if ui
            .add_enabled(sel.is_some(), egui::Button::new("✎ Edit"))
            .clicked()
        {
            crate::dialogs::open_feature_editor(app, sel);
        }
        if ui
            .add_enabled(sel.is_some(), egui::Button::new("🗑 Delete"))
            .clicked()
        {
            if let Some(id) = sel {
                crate::actions::delete_feature(app, id);
            }
        }
        if ui
            .button("◎ Detect Common Features")
            .on_hover_text("Scan for promoters, tags, origins and other known elements")
            .clicked()
        {
            crate::dialogs::open_detect_common(app);
        }
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.label(
                egui::RichText::new(format!("{} features", app.doc.features.len()))
                    .color(theme::MUTED)
                    .size(12.0),
            );
        });
    });
    ui.add_space(4.0);

    let filter = app.feature_filter.to_ascii_lowercase();
    let mut items: Vec<u64> = app
        .doc
        .features
        .iter()
        .filter(|f| {
            filter.is_empty()
                || f.name.to_ascii_lowercase().contains(&filter)
                || f.kind.to_ascii_lowercase().contains(&filter)
                || f.product.to_ascii_lowercase().contains(&filter)
        })
        .map(|f| f.id)
        .collect();

    let sort_col = app.feature_sort.column;
    let dir = app.feature_sort.dir;
    if let Some(dir) = dir {
        let doc = &app.doc;
        items.sort_by(|a, b| {
            let fa = doc.feature(*a).unwrap();
            let fb = doc.feature(*b).unwrap();
            let ord = match sort_col {
                1 => fa.name.to_lowercase().cmp(&fb.name.to_lowercase()),
                2 => fa.kind.cmp(&fb.kind),
                3 => fa.direction.label().cmp(fb.direction.label()),
                4 => fa.total_len().cmp(&fb.total_len()),
                5 => fa.span(l).start.cmp(&fb.span(l).start),
                _ => std::cmp::Ordering::Equal,
            };
            if dir == SortDir::Desc {
                ord.reverse()
            } else {
                ord
            }
        });
    }

    let mut toggle: Option<u64> = None;
    let mut clicked: Option<u64> = None;
    let mut double: Option<u64> = None;
    let mut sort_request: Option<usize> = None;
    let selected = app.selected_feature;

    let table = TableBuilder::new(ui)
        .striped(true)
        .resizable(true)
        .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
        .column(Column::auto().at_least(34.0))
        .column(Column::remainder().at_least(140.0))
        .column(Column::auto().at_least(90.0))
        .column(Column::auto().at_least(40.0))
        .column(Column::auto().at_least(70.0))
        .column(Column::auto().at_least(120.0))
        .column(Column::auto().at_least(70.0))
        .column(Column::remainder().at_least(120.0))
        .header(26.0, |mut row| {
            row.col(|ui| {
                ui.label(egui::RichText::new("👁").size(12.0).color(theme::MUTED));
            });
            for (i, label) in [
                (1, "Name"),
                (2, "Kind"),
                (3, "Dir"),
                (4, "Length"),
                (5, "Location"),
                (6, "Translated"),
            ] {
                row.col(|ui| {
                    let active = sort_col == i && dir.is_some();
                    let arrow = if active {
                        if dir == Some(SortDir::Asc) {
                            " ↑"
                        } else {
                            " ↓"
                        }
                    } else {
                        ""
                    };
                    if ui
                        .add(
                            egui::Button::new(
                                egui::RichText::new(format!("{label}{arrow}"))
                                    .size(12.0)
                                    .strong()
                                    .color(if active { theme::ACCENT } else { theme::MUTED }),
                            )
                            .frame(false),
                        )
                        .clicked()
                    {
                        sort_request = Some(i);
                    }
                });
            }
            row.col(|ui| {
                ui.label(
                    egui::RichText::new("Product / Note")
                        .size(12.0)
                        .strong()
                        .color(theme::MUTED),
                );
            });
        });

    table.body(|body| {
        body.rows(27.0, items.len(), |mut row| {
            let idx = row.index();
            let id = items[idx];
            let f = app.doc.feature(id).unwrap().clone();
            row.set_selected(selected == Some(id));
            row.col(|ui| {
                let mut v = f.visible;
                if ui.checkbox(&mut v, "").changed() {
                    toggle = Some(id);
                }
            });
            row.col(|ui| {
                ui.horizontal(|ui| {
                    let swatch = egui::Frame::new()
                        .fill(theme::rgb(f.color))
                        .corner_radius(3)
                        .inner_margin(egui::Margin::same(5));
                    swatch.show(ui, |ui| ui.set_min_size(egui::vec2(2.0, 2.0)));
                    ui.label(egui::RichText::new(&f.name).color(theme::TEXT));
                });
            });
            row.col(|ui| {
                ui.label(egui::RichText::new(&f.kind).color(theme::MUTED).size(12.0));
            });
            row.col(|ui| {
                ui.label(egui::RichText::new(f.direction.label()).color(theme::ACCENT));
            });
            row.col(|ui| {
                ui.label(
                    egui::RichText::new(format!("{}", f.total_len()))
                        .monospace()
                        .size(12.0),
                );
            });
            row.col(|ui| {
                let span = f.span(l);
                let loc = if f.segments.len() > 1 {
                    format!("join({}..{})", span.start + 1, span.start + span.len)
                } else {
                    format!("{}..{}", span.start + 1, span.start + span.len)
                };
                ui.label(
                    egui::RichText::new(loc)
                        .monospace()
                        .size(12.0)
                        .color(theme::MUTED),
                );
            });
            row.col(|ui| {
                if f.translated {
                    let stop = f.has_internal_stop(
                        &app.doc.sequence,
                        domain::genetic_code::code_by_id(app.doc.default_genetic_code),
                    );
                    if stop {
                        ui.label(egui::RichText::new("yes ⚠").color(theme::WARN).size(12.0))
                            .on_hover_text("Contains an internal stop codon");
                    } else {
                        ui.label(egui::RichText::new("yes").color(theme::OK).size(12.0));
                    }
                } else {
                    ui.label(egui::RichText::new("—").color(theme::FAINT));
                }
            });
            row.col(|ui| {
                let text = if !f.product.is_empty() {
                    f.product.clone()
                } else {
                    f.note.clone()
                };
                ui.add(
                    egui::Label::new(egui::RichText::new(text).size(12.0).color(theme::MUTED))
                        .truncate(),
                );
            });
            let resp = row.response();
            if resp.clicked() {
                clicked = Some(id);
            }
            if resp.double_clicked() {
                double = Some(id);
            }
        });
    });

    if let Some(i) = sort_request {
        if app.feature_sort.column == i {
            app.feature_sort.dir = match app.feature_sort.dir {
                Some(SortDir::Asc) => Some(SortDir::Desc),
                Some(SortDir::Desc) => None,
                None => Some(SortDir::Asc),
            };
        } else {
            app.feature_sort.column = i;
            app.feature_sort.dir = Some(SortDir::Asc);
        }
    }
    if let Some(id) = toggle {
        let name = app
            .doc
            .feature(id)
            .map(|f| f.name.clone())
            .unwrap_or_default();
        app.mutate(&format!("Toggle feature \"{name}\""), Vec::new(), |doc| {
            if let Some(f) = doc.feature_mut(id) {
                f.visible = !f.visible;
            }
        });
    }
    if let Some(id) = clicked {
        app.select_feature(id);
    }
    if let Some(id) = double {
        crate::dialogs::open_feature_editor(app, Some(id));
    }
}
