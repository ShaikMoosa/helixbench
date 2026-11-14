//! Primers view: oligo table with Tm, GC and binding-site counts.

use egui_extras::{Column, TableBuilder};

use domain::primer::{melting_temperature, HybridizationParams};

use crate::state::{AppState, SortDir};
use crate::theme;

pub fn show(app: &mut AppState, ui: &mut egui::Ui) {
    app.ensure_analysis();
    let params = HybridizationParams::default();

    ui.horizontal(|ui| {
        ui.add(
            egui::TextEdit::singleline(&mut app.primer_filter)
                .hint_text("Filter primers…")
                .desired_width(180.0),
        );
        if ui.button("＋ Add Primer").clicked() {
            crate::dialogs::open_primer_editor(app, None);
        }
        if ui
            .button("＋ From Selection")
            .on_hover_text("Create a primer from the selected bases")
            .clicked()
        {
            crate::actions::primer_from_selection(app);
        }
        let sel = app.selected_primer;
        if ui
            .add_enabled(sel.is_some(), egui::Button::new("✎ Edit"))
            .clicked()
        {
            crate::dialogs::open_primer_editor(app, sel);
        }
        if ui
            .add_enabled(sel.is_some(), egui::Button::new("🗑 Delete"))
            .clicked()
        {
            if let Some(id) = sel {
                crate::actions::delete_primer(app, id);
            }
        }
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.label(
                egui::RichText::new(format!(
                    "{} primers · Tm at {} mM Na⁺, {} nM oligo",
                    app.doc.primers.len(),
                    params.salt_mm,
                    params.primer_nm
                ))
                .color(theme::MUTED)
                .size(12.0),
            );
        });
    });
    ui.add_space(4.0);

    let filter = app.primer_filter.to_ascii_lowercase();
    let mut items: Vec<u64> = app
        .doc
        .primers
        .iter()
        .filter(|p| {
            filter.is_empty()
                || p.name.to_ascii_lowercase().contains(&filter)
                || p.description.to_ascii_lowercase().contains(&filter)
        })
        .map(|p| p.id)
        .collect();

    let sort_col = app.primer_sort.column;
    let dir = app.primer_sort.dir;
    if let Some(dir) = dir {
        let doc = &app.doc;
        let analysis = &app.analysis;
        items.sort_by(|a, b| {
            let pa = doc.primer(*a).unwrap();
            let pb = doc.primer(*b).unwrap();
            let ord = match sort_col {
                1 => pa.name.to_lowercase().cmp(&pb.name.to_lowercase()),
                3 => pa.len().cmp(&pb.len()),
                4 => pa.gc_percent().total_cmp(&pb.gc_percent()),
                5 => {
                    let ta = melting_temperature(&pa.sequence, &params).unwrap_or(-1.0);
                    let tb = melting_temperature(&pb.sequence, &params).unwrap_or(-1.0);
                    ta.total_cmp(&tb)
                }
                6 => {
                    let na = analysis.primer_sites.get(a).map(|s| s.len()).unwrap_or(0);
                    let nb = analysis.primer_sites.get(b).map(|s| s.len()).unwrap_or(0);
                    na.cmp(&nb)
                }
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
    let selected = app.selected_primer;

    let table = TableBuilder::new(ui)
        .striped(true)
        .resizable(true)
        .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
        .column(Column::auto().at_least(34.0))
        .column(Column::auto().at_least(130.0))
        .column(Column::remainder().at_least(180.0))
        .column(Column::auto().at_least(50.0))
        .column(Column::auto().at_least(60.0))
        .column(Column::auto().at_least(70.0))
        .column(Column::auto().at_least(50.0))
        .column(Column::remainder().at_least(100.0))
        .header(26.0, |mut row| {
            let mut sort_header =
                |row: &mut egui_extras::TableRow<'_, '_>, i: usize, label: &str| {
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
                };
            row.col(|ui| {
                ui.label(egui::RichText::new("👁").size(12.0).color(theme::MUTED));
            });
            sort_header(&mut row, 1, "Name");
            row.col(|ui| {
                ui.label(
                    egui::RichText::new("Sequence 5′→3′")
                        .size(12.0)
                        .strong()
                        .color(theme::MUTED),
                );
            });
            sort_header(&mut row, 3, "Length");
            sort_header(&mut row, 4, "GC%");
            sort_header(&mut row, 5, "Tm (°C)");
            sort_header(&mut row, 6, "Sites");
            row.col(|ui| {
                ui.label(
                    egui::RichText::new("Description")
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
            let p = app.doc.primer(id).unwrap().clone();
            row.set_selected(selected == Some(id));
            row.col(|ui| {
                let mut v = p.visible;
                if ui.checkbox(&mut v, "").changed() {
                    toggle = Some(id);
                }
            });
            row.col(|ui| {
                ui.horizontal(|ui| {
                    let swatch = egui::Frame::new()
                        .fill(theme::rgb(p.color))
                        .corner_radius(3)
                        .inner_margin(egui::Margin::same(5));
                    swatch.show(ui, |ui| ui.set_min_size(egui::vec2(2.0, 2.0)));
                    ui.label(egui::RichText::new(&p.name).color(theme::TEXT));
                    if p.phosphorylated {
                        ui.label(egui::RichText::new("ⓟ").size(11.0).color(theme::WARN))
                            .on_hover_text("5′ phosphorylated");
                    }
                });
            });
            row.col(|ui| {
                let seq = String::from_utf8_lossy(&p.sequence);
                let shown: String = seq.chars().take(36).collect();
                let ell = if seq.len() > 36 { "…" } else { "" };
                ui.add(
                    egui::Label::new(
                        egui::RichText::new(format!("{shown}{ell}"))
                            .monospace()
                            .size(11.5)
                            .color(theme::MUTED),
                    )
                    .truncate(),
                );
            });
            row.col(|ui| {
                ui.label(
                    egui::RichText::new(format!("{}", p.len()))
                        .monospace()
                        .size(12.0),
                );
            });
            row.col(|ui| {
                ui.label(
                    egui::RichText::new(format!("{:.0}", p.gc_percent()))
                        .monospace()
                        .size(12.0),
                );
            });
            row.col(|ui| match melting_temperature(&p.sequence, &params) {
                Some(tm) => {
                    let col = if tm < params.min_tm {
                        theme::WARN
                    } else {
                        theme::OK
                    };
                    ui.label(
                        egui::RichText::new(format!("{tm:.1}"))
                            .monospace()
                            .size(12.0)
                            .color(col),
                    );
                }
                None => {
                    ui.label(egui::RichText::new("—").color(theme::FAINT));
                }
            });
            row.col(|ui| {
                let n = app
                    .analysis
                    .primer_sites
                    .get(&id)
                    .map(|s| s.len())
                    .unwrap_or(0);
                let col = if n == 0 { theme::WARN } else { theme::TEXT };
                ui.label(
                    egui::RichText::new(format!("{n}"))
                        .monospace()
                        .size(12.0)
                        .color(col),
                );
            });
            row.col(|ui| {
                ui.add(
                    egui::Label::new(
                        egui::RichText::new(&p.description)
                            .size(12.0)
                            .color(theme::MUTED),
                    )
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
        if app.primer_sort.column == i {
            app.primer_sort.dir = match app.primer_sort.dir {
                Some(SortDir::Asc) => Some(SortDir::Desc),
                Some(SortDir::Desc) => None,
                None => Some(SortDir::Asc),
            };
        } else {
            app.primer_sort.column = i;
            app.primer_sort.dir = Some(SortDir::Asc);
        }
    }
    if let Some(id) = toggle {
        let name = app
            .doc
            .primer(id)
            .map(|p| p.name.clone())
            .unwrap_or_default();
        app.mutate(&format!("Toggle primer \"{name}\""), Vec::new(), |doc| {
            if let Some(p) = doc.primer_mut(id) {
                p.visible = !p.visible;
            }
        });
    }
    if let Some(id) = clicked {
        app.selected_primer = Some(id);
        app.selected_feature = None;
        if let Some(sites) = app.analysis.primer_sites.get(&id) {
            if let Some(s) = sites.first() {
                app.select(Some(domain::Range::new(s.start, s.len)));
            }
        }
    }
    if let Some(id) = double {
        crate::dialogs::open_primer_editor(app, Some(id));
    }
}
