//! Bottom find bar: DNA / protein / name search with live results and navigation.

use egui::{RichText, Stroke};

use crate::state::{AppState, SearchMode};
use crate::theme;

pub fn show(app: &mut AppState, ui: &mut egui::Ui) {
    egui::Panel::bottom("hb_find")
        .frame(
            egui::Frame::new()
                .fill(theme::PANEL)
                .stroke(Stroke::new(1.0, theme::BORDER))
                .inner_margin(egui::Margin::symmetric(12, 7)),
        )
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                let before = app.search.mode;
                egui::ComboBox::from_id_salt("find_mode")
                    .selected_text(before.label())
                    .width(185.0)
                    .show_ui(ui, |ui| {
                        for m in [SearchMode::Dna, SearchMode::Protein, SearchMode::Name] {
                            ui.selectable_value(&mut app.search.mode, m, m.label());
                        }
                    });
                if app.search.mode != before {
                    app.run_search();
                    if app.search.hit_count() > 0 {
                        app.goto_current_hit();
                    }
                }

                let hint = match app.search.mode {
                    SearchMode::Dna => "e.g. GAATTC or GGTCTRCTTC",
                    SearchMode::Protein => "e.g. MKWG or MXWG",
                    SearchMode::Name => "e.g. *promoter or Eco*",
                };
                let resp = ui.add(
                    egui::TextEdit::singleline(&mut app.search.query)
                        .desired_width(260.0)
                        .font(egui::FontId::monospace(13.0))
                        .hint_text(hint)
                        .text_color(if app.search.error.is_some() {
                            theme::DANGER
                        } else {
                            theme::TEXT
                        }),
                );
                if app.search.focus_request {
                    resp.request_focus();
                    app.search.focus_request = false;
                }
                if resp.changed() {
                    app.run_search();
                    if app.search.hit_count() > 0 {
                        app.goto_current_hit();
                    }
                }
                if resp.has_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    let back = ui.input(|i| i.modifiers.shift);
                    app.search_next(!back);
                }

                recents_menu(app, ui);

                let n = app.search.hit_count();
                if ui
                    .add_enabled(n > 0, egui::Button::new("◀"))
                    .on_hover_text("Previous match (Shift+Enter)")
                    .clicked()
                {
                    app.search_next(false);
                }
                if ui
                    .add_enabled(n > 0, egui::Button::new("▶"))
                    .on_hover_text("Next match (Enter)")
                    .clicked()
                {
                    app.search_next(true);
                }

                status_label(app, ui, n);

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("✕").on_hover_text("Close (Esc)").clicked() {
                        app.search.open = false;
                    }
                    if ui.button("?").on_hover_text("Search syntax tips").clicked() {
                        app.dialog = Some(crate::state::Dialog::SearchTips);
                    }
                });
            });
        });
}

fn recents_menu(app: &mut AppState, ui: &mut egui::Ui) {
    let resp = ui.menu_button(RichText::new("Recent ▾").size(12.0), |ui| {
        ui.set_min_width(260.0);
        if app.search.recents.is_empty() {
            ui.label(
                RichText::new("No recent searches")
                    .color(theme::FAINT)
                    .size(12.0),
            );
            return;
        }
        let recents = app.search.recents.clone();
        for (m, q) in recents {
            let tag = match m {
                SearchMode::Dna => "DNA",
                SearchMode::Protein => "AA",
                SearchMode::Name => "Name",
            };
            let label = format!("[{tag}]  {q}");
            if ui
                .button(RichText::new(label).monospace().size(12.0))
                .clicked()
            {
                app.search.mode = m;
                app.search.query = q;
                app.run_search();
                if app.search.hit_count() > 0 {
                    app.goto_current_hit();
                }
                ui.close();
            }
        }
    });
    resp.response.on_hover_text("Recent searches");
}

fn status_label(app: &AppState, ui: &mut egui::Ui, n: usize) {
    if let Some(e) = &app.search.error {
        ui.label(
            RichText::new(format!("⚠ {e}"))
                .color(theme::DANGER)
                .size(12.0),
        );
        return;
    }
    if app.search.query.trim().is_empty() {
        ui.label(
            RichText::new("Type to search both strands")
                .color(theme::FAINT)
                .size(12.0),
        );
        return;
    }
    if n == 0 {
        ui.label(RichText::new("No matches").color(theme::WARN).size(12.5));
        return;
    }
    ui.label(
        RichText::new(format!("{} of {}", app.search.current + 1, n))
            .monospace()
            .strong()
            .color(theme::TEAL)
            .size(12.5),
    );
    if let Some((start, len, strand)) = current_hit_locus(app) {
        let end = start + len;
        let txt = if len == 1 {
            format!("at {}", start + 1)
        } else {
            format!("at {}..{}", start + 1, end)
        };
        ui.label(
            RichText::new(format!("{txt} · {strand}"))
                .monospace()
                .size(11.5)
                .color(theme::MUTED),
        );
    }
}

fn current_hit_locus(app: &AppState) -> Option<(usize, usize, &'static str)> {
    match app.search.mode {
        SearchMode::Dna => app.search.dna_hits.get(app.search.current).map(|h| {
            (
                h.start,
                h.len,
                if h.strand == domain::Strand::Top {
                    "top strand"
                } else {
                    "bottom strand"
                },
            )
        }),
        SearchMode::Protein => app.search.protein_hits.get(app.search.current).map(|h| {
            (
                h.start,
                h.len,
                if h.strand == domain::Strand::Top {
                    "top strand"
                } else {
                    "bottom strand"
                },
            )
        }),
        SearchMode::Name => None,
    }
}
