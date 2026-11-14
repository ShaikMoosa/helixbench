//! History view: the operation log with history colors and revert.

use egui::Stroke;

use crate::state::{AppState, ColorsMode};
use crate::theme;

pub fn show(app: &mut AppState, ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        let can_undo = !app.history.is_empty();
        let can_redo = !app.future.is_empty();
        if ui.add_enabled(can_undo, egui::Button::new("↶ Undo")).clicked() {
            app.undo();
        }
        if ui.add_enabled(can_redo, egui::Button::new("↷ Redo")).clicked() {
            app.redo();
        }
        let sel = app.history_highlight;
        if ui.add_enabled(sel.is_some(), egui::Button::new("⏮ Revert to Before Selected")).clicked() {
            if let Some(i) = sel {
                app.revert_to(i);
                app.toast("Reverted to earlier state");
            }
        }
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.label(egui::RichText::new("Click an operation to highlight its affected regions in Map and Sequence views.").color(theme::MUTED).size(12.0));
        });
    });
    ui.add_space(6.0);

    if app.history.is_empty() {
        ui.vertical_centered(|ui| {
            ui.add_space(60.0);
            ui.label(egui::RichText::new("↺").size(40.0).color(theme::FAINT));
            ui.add_space(8.0);
            ui.label(
                egui::RichText::new("No operations yet")
                    .size(15.0)
                    .color(theme::MUTED),
            );
            ui.label(
                egui::RichText::new("Edits, annotations and enzyme changes are recorded here.")
                    .size(12.0)
                    .color(theme::FAINT),
            );
        });
        return;
    }

    let mut highlight: Option<usize> = None;
    let mut clear = false;
    let mut revert: Option<(usize, String)> = None;
    egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
        // Current state marker
        egui::Frame::new()
            .fill(theme::SURFACE)
            .stroke(Stroke::new(1.0, theme::BORDER))
            .corner_radius(6)
            .inner_margin(egui::Margin::symmetric(10, 7))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("●").color(theme::OK));
                    ui.label(egui::RichText::new("Current state").strong());
                    ui.label(egui::RichText::new(format!("{} bp · {} features · {} primers", app.doc.sequence.len(), app.doc.features.len(), app.doc.primers.len())).color(theme::MUTED).size(12.0));
                });
            });
        ui.add_space(4.0);

        for (i, entry) in app.history.iter().enumerate().rev() {
            let selected = app.history_highlight == Some(i);
            let fill = if selected { theme::SURFACE_2 } else { theme::PANEL };
            let stroke = if selected { Stroke::new(1.5, theme::ACCENT) } else { Stroke::new(1.0, theme::BORDER) };
            let resp = egui::Frame::new()
                .fill(fill)
                .stroke(stroke)
                .corner_radius(6)
                .inner_margin(egui::Margin::symmetric(10, 7))
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new(format!("{}", i + 1)).monospace().color(theme::FAINT).size(12.0));
                        ui.label(egui::RichText::new(&entry.label).color(theme::TEXT));
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if entry.regions.is_empty() {
                                ui.label(egui::RichText::new("whole document").color(theme::FAINT).size(11.0));
                            } else {
                                let ranges: Vec<String> = entry.regions.iter().take(3).map(|r| format!("{}..{}", r.range.start + 1, r.range.start + r.range.len)).collect();
                                let more = if entry.regions.len() > 3 { format!(" +{}", entry.regions.len() - 3) } else { String::new() };
                                ui.label(egui::RichText::new(format!("{}{}", ranges.join(", "), more)).monospace().color(theme::MUTED).size(11.0));
                            }
                        });
                    });
                })
                .response;
            let resp = resp.interact(egui::Sense::click());
            if resp.clicked() {
                highlight = Some(i);
            }
            if resp.double_clicked() {
                revert = Some((i, entry.label.clone()));
                clear = true;
            }
            resp.on_hover_text("Click to show history colors · double-click to revert to before this operation");
            ui.add_space(3.0);
        }

        // Click on empty area clears history colors (per the guide).
        let rest = ui.available_size_before_wrap();
        if rest.y > 4.0 {
            let r = ui.allocate_response(rest, egui::Sense::click());
            if r.clicked() {
                clear = true;
            }
        }
    });

    if let Some(i) = highlight {
        app.history_highlight = Some(i);
        app.colors_mode = ColorsMode::History;
    }
    if clear {
        app.history_highlight = None;
    }
    if let Some((i, label)) = revert {
        app.revert_to(i);
        app.toast(format!("Reverted to before \"{label}\""));
    }
}
