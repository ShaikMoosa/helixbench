//! Enzymes view: choose the working enzyme set with filters and cut counts.

use egui_extras::{Column, TableBuilder};

use domain::enzyme::Overhang;

use crate::state::{AppState, SortDir, CUT_COUNT_OPTIONS, OVERHANG_OPTIONS, SITE_LEN_OPTIONS};
use crate::theme;

pub fn show(app: &mut AppState, ui: &mut egui::Ui) {
    app.ensure_analysis();
    let l = app.doc.sequence.len();

    ui.horizontal(|ui| {
        ui.label(egui::RichText::new("Enzyme set:").strong());
        let sets = app.analysis.sets.clone();
        let current = current_set_name(app, &sets);
        let mut chosen_set: Option<Vec<String>> = None;
        egui::ComboBox::from_id_salt("enzyme_set_combo")
            .selected_text(&current)
            .width(200.0)
            .show_ui(ui, |ui| {
                for s in &sets {
                    let n_cutting = s
                        .enzymes
                        .iter()
                        .filter(|n| app.analysis.counts.get(*n).copied().unwrap_or(0) > 0)
                        .count();
                    if ui
                        .selectable_label(current == s.name, format!("{}  ({})", s.name, n_cutting))
                        .clicked()
                    {
                        chosen_set = Some(s.enzymes.clone());
                    }
                }
            });
        if let Some(enzymes) = chosen_set {
            let name = sets
                .iter()
                .find(|s| s.enzymes == enzymes)
                .map(|s| s.name.clone())
                .unwrap_or_default();
            app.mutate(&format!("Use enzyme set \"{name}\""), Vec::new(), |doc| {
                doc.chosen_enzymes = enzymes;
            });
            app.toast(format!("Enzyme set: {name}"));
        }
        ui.separator();
        ui.label(
            egui::RichText::new(format!(
                "{} enzymes shown on map",
                app.analysis.chosen.len()
            ))
            .color(theme::MUTED)
            .size(12.0),
        );
    });
    ui.add_space(2.0);
    ui.horizontal(|ui| {
        ui.add(
            egui::TextEdit::singleline(&mut app.enzyme_filter.name)
                .hint_text("Name contains…")
                .desired_width(140.0),
        );
        egui::ComboBox::from_id_salt("enz_cut_count")
            .selected_text(CUT_COUNT_OPTIONS[app.enzyme_filter.cut_count])
            .show_ui(ui, |ui| {
                for (i, opt) in CUT_COUNT_OPTIONS.iter().enumerate() {
                    ui.selectable_value(&mut app.enzyme_filter.cut_count, i, *opt);
                }
            });
        egui::ComboBox::from_id_salt("enz_overhang")
            .selected_text(OVERHANG_OPTIONS[app.enzyme_filter.overhang])
            .show_ui(ui, |ui| {
                for (i, opt) in OVERHANG_OPTIONS.iter().enumerate() {
                    ui.selectable_value(&mut app.enzyme_filter.overhang, i, *opt);
                }
            });
        egui::ComboBox::from_id_salt("enz_site_len")
            .selected_text(SITE_LEN_OPTIONS[app.enzyme_filter.site_len])
            .show_ui(ui, |ui| {
                for (i, opt) in SITE_LEN_OPTIONS.iter().enumerate() {
                    ui.selectable_value(&mut app.enzyme_filter.site_len, i, *opt);
                }
            });
        if ui.button("✓ Check shown").clicked() {
            let names = filtered_names(app);
            app.mutate("Add filtered enzymes to set", Vec::new(), |doc| {
                for n in names {
                    if !doc.chosen_enzymes.contains(&n) {
                        doc.chosen_enzymes.push(n);
                    }
                }
            });
        }
        if ui.button("✗ Uncheck shown").clicked() {
            let names = filtered_names(app);
            app.mutate("Remove filtered enzymes from set", Vec::new(), |doc| {
                doc.chosen_enzymes.retain(|n| !names.contains(n));
            });
        }
        if ui.button("Clear set").clicked() {
            app.mutate("Clear enzyme set", Vec::new(), |doc| {
                doc.chosen_enzymes.clear()
            });
        }
    });
    ui.add_space(4.0);

    let mut items = filtered_enzymes(app);
    let sort_col = app.enzyme_sort.column;
    let dir = app.enzyme_sort.dir;
    if let Some(dir) = dir {
        items.sort_by(|a, b| {
            let ord = match sort_col {
                1 => a.0.name.to_lowercase().cmp(&b.0.name.to_lowercase()),
                2 => a.0.site.len().cmp(&b.0.site.len()),
                3 => a.1.cmp(&b.1),
                _ => std::cmp::Ordering::Equal,
            };
            if dir == SortDir::Desc {
                ord.reverse()
            } else {
                ord
            }
        });
    } else {
        items.sort_by_key(|a| a.0.name.to_lowercase());
    }

    let mut toggles: Vec<String> = Vec::new();
    let mut clicked: Option<(String, usize)> = None;
    let mut sort_request: Option<usize> = None;

    let table = TableBuilder::new(ui)
        .striped(true)
        .resizable(true)
        .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
        .column(Column::auto().at_least(34.0))
        .column(Column::auto().at_least(110.0))
        .column(Column::auto().at_least(130.0))
        .column(Column::auto().at_least(60.0))
        .column(Column::auto().at_least(130.0))
        .column(Column::remainder().at_least(160.0))
        .header(26.0, |mut row| {
            row.col(|ui| {
                ui.label(egui::RichText::new("✓").size(12.0).color(theme::MUTED));
            });
            for (i, label) in [(1, "Enzyme"), (2, "Recognition"), (3, "Cuts")] {
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
                    egui::RichText::new("Overhang")
                        .size(12.0)
                        .strong()
                        .color(theme::MUTED),
                );
            });
            row.col(|ui| {
                ui.label(
                    egui::RichText::new("Cut positions")
                        .size(12.0)
                        .strong()
                        .color(theme::MUTED),
                );
            });
        });

    table.body(|body| {
        body.rows(26.0, items.len(), |mut row| {
            let idx = row.index();
            let (e, n) = &items[idx];
            let in_set = app.doc.chosen_enzymes.contains(&e.name);
            row.set_selected(in_set && *n > 0);
            row.col(|ui| {
                let mut v = in_set;
                if ui.checkbox(&mut v, "").changed() {
                    toggles.push(e.name.clone());
                }
            });
            row.col(|ui| {
                let col = if *n == 0 {
                    theme::FAINT
                } else if *n == 1 {
                    theme::TEXT
                } else {
                    theme::MUTED
                };
                ui.label(egui::RichText::new(&e.name).strong().color(col));
            });
            row.col(|ui| {
                ui.label(
                    egui::RichText::new(&e.site)
                        .monospace()
                        .size(12.0)
                        .color(theme::TEAL),
                );
            });
            row.col(|ui| {
                let col = match n {
                    0 => theme::FAINT,
                    1 => theme::OK,
                    2 => theme::WARN,
                    _ => theme::DANGER,
                };
                ui.label(
                    egui::RichText::new(format!("{n}"))
                        .monospace()
                        .strong()
                        .color(col),
                );
            });
            row.col(|ui| {
                let oh = e.overhang();
                let outside = e.cut_top < 0 || e.cut_top > e.site_len() as i32;
                let text = if outside {
                    format!("{} · outside site", oh.label())
                } else {
                    oh.label()
                };
                ui.label(egui::RichText::new(text).size(12.0).color(theme::MUTED));
            });
            row.col(|ui| {
                if *n == 0 {
                    ui.label(egui::RichText::new("—").color(theme::FAINT));
                } else {
                    let empty = Vec::new();
                    let sites = app.analysis.all_sites.get(&e.name).unwrap_or(&empty);
                    let mut pos: Vec<String> = sites
                        .iter()
                        .take(8)
                        .map(|s| format!("{}", s.cut_top + 1))
                        .collect();
                    if sites.len() > 8 {
                        pos.push(format!("+{} more", sites.len() - 8));
                    }
                    ui.add(
                        egui::Label::new(
                            egui::RichText::new(pos.join(", "))
                                .monospace()
                                .size(11.5)
                                .color(theme::MUTED),
                        )
                        .truncate(),
                    );
                }
            });
            let resp = row.response();
            if resp.clicked() && *n > 0 {
                if let Some(s) = app.analysis.all_sites.get(&e.name).and_then(|s| s.first()) {
                    clicked = Some((e.name.clone(), s.position));
                }
            }
        });
    });

    if let Some(i) = sort_request {
        if app.enzyme_sort.column == i {
            app.enzyme_sort.dir = match app.enzyme_sort.dir {
                Some(SortDir::Asc) => Some(SortDir::Desc),
                Some(SortDir::Desc) => None,
                None => Some(SortDir::Asc),
            };
        } else {
            app.enzyme_sort.column = i;
            app.enzyme_sort.dir = Some(SortDir::Asc);
        }
    }
    if !toggles.is_empty() {
        app.mutate("Change enzyme set", Vec::new(), |doc| {
            for name in toggles {
                if let Some(pos) = doc.chosen_enzymes.iter().position(|n| *n == name) {
                    doc.chosen_enzymes.remove(pos);
                } else {
                    doc.chosen_enzymes.push(name);
                }
            }
        });
    }
    if let Some((name, pos)) = clicked {
        app.select(Some(domain::Range::new(pos, 1)));
        app.scroll_to = Some(pos);
        app.toast(format!("{name} cut at {}", pos + 1));
    }
    let _ = l;
}

fn current_set_name(app: &AppState, sets: &[domain::EnzymeSet]) -> String {
    let mut chosen = app.doc.chosen_enzymes.clone();
    chosen.sort();
    for s in sets {
        let mut names = s.enzymes.clone();
        names.sort();
        if names == chosen {
            return s.name.clone();
        }
    }
    "Custom".to_string()
}

fn passes_filter(app: &AppState, e: &domain::Enzyme, n: usize) -> bool {
    let f = &app.enzyme_filter;
    if !f.name.is_empty()
        && !e
            .name
            .to_ascii_lowercase()
            .contains(&f.name.to_ascii_lowercase())
    {
        return false;
    }
    let ok = match f.cut_count {
        0 => true,
        1 => n == 0,
        2 => n == 1,
        3 => n == 2,
        4 => n <= 2,
        _ => n >= 3,
    };
    if !ok {
        return false;
    }
    let outside = e.cut_top < 0 || e.cut_top > e.site_len() as i32;
    let ok = match f.overhang {
        0 => true,
        1 => matches!(e.overhang(), Overhang::FivePrime(_)) && !outside,
        2 => matches!(e.overhang(), Overhang::ThreePrime(_)) && !outside,
        3 => matches!(e.overhang(), Overhang::Blunt) && !outside,
        _ => outside,
    };
    if !ok {
        return false;
    }
    let sl = e.site_len();
    match f.site_len {
        0 => true,
        1 => sl == 4,
        2 => sl == 5,
        3 => sl == 6,
        _ => sl >= 7,
    }
}

fn filtered_enzymes(app: &AppState) -> Vec<(domain::Enzyme, usize)> {
    domain::enzyme_db::all_enzymes()
        .into_iter()
        .filter_map(|e| {
            let n = app.analysis.counts.get(&e.name).copied().unwrap_or(0);
            if passes_filter(app, &e, n) {
                Some((e, n))
            } else {
                None
            }
        })
        .collect()
}

fn filtered_names(app: &AppState) -> Vec<String> {
    filtered_enzymes(app)
        .into_iter()
        .map(|(e, _)| e.name)
        .collect()
}
