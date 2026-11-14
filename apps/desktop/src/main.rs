#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod dialogs;
mod find;
mod theme;
mod views;

pub use appcore::{actions, analysis, state};

use std::time::Duration;

use eframe::egui;
use egui::{Color32, RichText, Stroke};

use domain::Topology;
use state::{AppState, ColorsMode, Dialog, PendingAction, View};

fn main() -> eframe::Result {
    let config = storage::config::load();
    let doc = initial_document(&config);
    let size = config.window_size.unwrap_or([1360.0, 860.0]);
    let maximized = config.window_maximized;

    let mut viewport = egui::ViewportBuilder::default()
        .with_title("HelixBench")
        .with_inner_size(size)
        .with_min_inner_size([980.0, 640.0])
        .with_icon(std::sync::Arc::new(theme::icon()));
    if maximized {
        viewport = viewport.with_maximized(true);
    }
    let options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };

    eframe::run_native(
        "HelixBench",
        options,
        Box::new(move |cc| {
            theme::install_fonts(&cc.egui_ctx);
            theme::apply(&cc.egui_ctx);
            Ok(Box::new(App {
                state: AppState::new(doc, config),
                closing: false,
                last_title: String::new(),
            }))
        }),
    )
}

/// Open the most recent existing file, else the seeded demo plasmid.
fn initial_document(config: &storage::config::Config) -> domain::Document {
    for p in &config.recent_files {
        if p.exists() {
            if let Ok(doc) = storage::load_any(p) {
                return doc;
            }
        }
    }
    domain::demo::build_demo()
}

struct App {
    state: AppState,
    closing: bool,
    last_title: String,
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();
        self.state.ensure_analysis();

        // Window-close guard: intercept the OS close while there are unsaved edits.
        let rect = ctx.viewport_rect();
        self.state.config.window_size = Some([rect.width(), rect.height()]);
        self.state.config.window_maximized = ctx.input(|i| i.viewport().maximized.unwrap_or(false));
        if ctx.input(|i| i.viewport().close_requested()) && !self.closing {
            ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
            if self.state.dirty {
                actions::guard(&mut self.state, PendingAction::CloseApp);
            } else {
                actions::persist_config(&mut self.state);
                self.closing = true;
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }
        }
        if self.state.exit_requested {
            self.state.exit_requested = false;
            self.closing = true;
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }

        self.shortcuts(&ctx);

        // Track window geometry in memory; flushed to disk on close/save.
        let rect = ctx.viewport_rect();
        self.state.config.window_size = Some([rect.width(), rect.height()]);
        self.state.config.window_maximized = ctx.input(|i| i.viewport().maximized.unwrap_or(false));

        self.menu_bar(ui);
        self.tab_bar(ui);
        self.status_bar(ui);
        if self.state.search.open {
            find::show(&mut self.state, ui);
        }
        self.side_toolbar(ui);

        egui::CentralPanel::default()
            .frame(
                egui::Frame::new()
                    .fill(theme::BG)
                    .inner_margin(egui::Margin::same(0)),
            )
            .show(ui, |ui| {
                let app = &mut self.state;
                match app.view {
                    View::Map => views::map::show(app, ui),
                    View::Sequence => views::sequence::show(app, ui),
                    View::Features => views::features::show(app, ui),
                    View::Primers => views::primers::show(app, ui),
                    View::Enzymes => views::enzymes::show(app, ui),
                    View::History => views::history::show(app, ui),
                }
            });

        dialogs::show(&mut self.state, &ctx);
        self.toast(&ctx);
        self.update_title(&ctx);
    }
}

impl App {
    fn shortcuts(&mut self, ctx: &egui::Context) {
        let app = &mut self.state;

        if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            match &app.dialog {
                Some(Dialog::Unsaved { .. }) => {}
                Some(_) => app.dialog = None,
                None => {
                    if app.search.open {
                        app.search.open = false;
                    } else {
                        app.select(None);
                    }
                }
            }
        }
        if app.dialog.is_some() {
            return;
        }

        let ctrl = egui::Modifiers::CTRL;
        let wants_text = ctx.egui_wants_keyboard_input();

        if ctx.input_mut(|i| i.consume_key(ctrl, egui::Key::O)) {
            actions::guard(app, PendingAction::Open);
        }
        if ctx.input_mut(|i| i.consume_key(ctrl | egui::Modifiers::SHIFT, egui::Key::S)) {
            actions::save_as(app);
        } else if ctx.input_mut(|i| i.consume_key(ctrl, egui::Key::S)) {
            actions::save(app);
        }
        if ctx.input_mut(|i| i.consume_key(ctrl, egui::Key::N)) {
            actions::guard(app, PendingAction::NewEmpty);
        }
        if ctx.input_mut(|i| i.consume_key(ctrl, egui::Key::Z)) && app.undo() {
            app.toast("Undo");
        }
        if ctx.input_mut(|i| i.consume_key(ctrl, egui::Key::Y)) && app.redo() {
            app.toast("Redo");
        }
        if ctx.input_mut(|i| i.consume_key(ctrl, egui::Key::F)) {
            app.search.open = true;
            app.search.focus_request = true;
        }
        let num = [
            egui::Key::Num1,
            egui::Key::Num2,
            egui::Key::Num3,
            egui::Key::Num4,
            egui::Key::Num5,
            egui::Key::Num6,
        ];
        for (i, v) in View::ALL.iter().enumerate() {
            if ctx.input_mut(|inp| inp.consume_key(ctrl, num[i])) {
                app.view = *v;
            }
        }

        // egui-winit turns Ctrl+C/X/V into clipboard events rather than key events.
        let (mut copy, mut cut) = (false, false);
        let mut paste = String::new();
        let mut typed = String::new();
        ctx.input(|i| {
            for e in &i.events {
                match e {
                    egui::Event::Copy => copy = true,
                    egui::Event::Cut => cut = true,
                    egui::Event::Paste(s) => paste.push_str(s),
                    egui::Event::Text(s) => typed.push_str(s),
                    _ => {}
                }
            }
        });
        if !wants_text {
            if copy {
                let text = actions::copy_selection(app);
                if !text.is_empty() {
                    ctx.copy_text(text);
                }
            }
            if cut {
                if let Some(text) = actions::cut_selection(app) {
                    if !text.is_empty() {
                        ctx.copy_text(text);
                    }
                }
            }
            let mut insert = paste;
            if app.view == View::Sequence {
                insert.push_str(&typed);
            }
            let dna: String = insert
                .chars()
                .filter(|c| {
                    c.is_ascii() && domain::iupac::dna_mask(c.to_ascii_uppercase() as u8) != 0
                })
                .collect();
            if !dna.is_empty() {
                actions::insert_bases(app, &dna, true);
            }
            if ctx.input_mut(|i| i.consume_key(ctrl, egui::Key::A)) {
                actions::select_all(app);
            }
            if ctx.input_mut(|i| {
                i.consume_key(egui::Modifiers::NONE, egui::Key::Delete)
                    || i.consume_key(egui::Modifiers::NONE, egui::Key::Backspace)
            }) {
                actions::delete_selection(app);
            }
        }
    }

    fn menu_bar(&mut self, ui: &mut egui::Ui) {
        let ctx = ui.ctx().clone();
        let app = &mut self.state;
        egui::Panel::top("hb_menu")
            .frame(
                egui::Frame::new()
                    .fill(theme::PANEL)
                    .inner_margin(egui::Margin::symmetric(6, 2)),
            )
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.menu_button("File", |ui| {
                        if ui.button("New Sequence…\tCtrl+N").clicked() {
                            actions::guard(app, PendingAction::NewEmpty);
                            ui.close();
                        }
                        if ui.button("Open Demo Plasmid").clicked() {
                            actions::guard(app, PendingAction::NewDemo);
                            ui.close();
                        }
                        ui.separator();
                        if ui.button("Open…\tCtrl+O").clicked() {
                            actions::guard(app, PendingAction::Open);
                            ui.close();
                        }
                        ui.menu_button("Recent Files", |ui| {
                            if app.config.recent_files.is_empty() {
                                ui.label(RichText::new("No recent files").color(theme::FAINT));
                            }
                            let recents = app.config.recent_files.clone();
                            for p in recents {
                                let name = p
                                    .file_name()
                                    .map(|s| s.to_string_lossy().into_owned())
                                    .unwrap_or_else(|| p.display().to_string());
                                if ui
                                    .button(name)
                                    .on_hover_text(p.display().to_string())
                                    .clicked()
                                {
                                    actions::guard(app, PendingAction::OpenRecent(p));
                                    ui.close();
                                }
                            }
                        });
                        ui.separator();
                        if ui.button("Save\tCtrl+S").clicked() {
                            actions::save(app);
                            ui.close();
                        }
                        if ui.button("Save As…\tCtrl+Shift+S").clicked() {
                            actions::save_as(app);
                            ui.close();
                        }
                        ui.menu_button("Export", |ui| {
                            if ui.button("FASTA…").clicked() {
                                actions::export_fasta(app);
                                ui.close();
                            }
                            if ui.button("GenBank…").clicked() {
                                actions::export_genbank(app);
                                ui.close();
                            }
                            if ui.button("Plain sequence…").clicked() {
                                actions::export_plain(app);
                                ui.close();
                            }
                        });
                        ui.separator();
                        if ui.button("Exit").clicked() {
                            actions::guard(app, PendingAction::CloseApp);
                            ui.close();
                        }
                    });

                    ui.menu_button("Edit", |ui| {
                        let undo = app
                            .history
                            .last()
                            .map(|e| format!("Undo {}\tCtrl+Z", e.label))
                            .unwrap_or_else(|| "Undo\tCtrl+Z".into());
                        if ui
                            .add_enabled(!app.history.is_empty(), egui::Button::new(undo))
                            .clicked()
                        {
                            app.undo();
                            ui.close();
                        }
                        let redo = app
                            .future
                            .last()
                            .map(|e| format!("Redo {}\tCtrl+Y", e.label))
                            .unwrap_or_else(|| "Redo\tCtrl+Y".into());
                        if ui
                            .add_enabled(!app.future.is_empty(), egui::Button::new(redo))
                            .clicked()
                        {
                            app.redo();
                            ui.close();
                        }
                        ui.separator();
                        if ui.button("Cut\tCtrl+X").clicked() {
                            if let Some(text) = actions::cut_selection(app) {
                                if !text.is_empty() {
                                    ctx.copy_text(text);
                                }
                            }
                            ui.close();
                        }
                        if ui.button("Copy\tCtrl+C").clicked() {
                            let text = actions::copy_selection(app);
                            if !text.is_empty() {
                                ctx.copy_text(text);
                            }
                            ui.close();
                        }
                        if ui.button("Paste\tCtrl+V").clicked() {
                            paste_from_clipboard(app);
                            ui.close();
                        }
                        if ui
                            .add_enabled(app.selection.is_some(), egui::Button::new("Delete\tDel"))
                            .clicked()
                        {
                            actions::delete_selection(app);
                            ui.close();
                        }
                        if ui.button("Select All\tCtrl+A").clicked() {
                            actions::select_all(app);
                            ui.close();
                        }
                        ui.separator();
                        if ui
                            .add_enabled(
                                app.selection.is_some(),
                                egui::Button::new("Color Selection…"),
                            )
                            .clicked()
                        {
                            actions::open_color_selection(app);
                            ui.close();
                        }
                        if ui
                            .add_enabled(
                                !app.doc.color_regions.is_empty(),
                                egui::Button::new("Clear Custom Colors"),
                            )
                            .clicked()
                        {
                            actions::clear_custom_colors(app);
                            ui.close();
                        }
                        ui.separator();
                        if ui.button("Find…\tCtrl+F").clicked() {
                            app.search.open = true;
                            app.search.focus_request = true;
                            ui.close();
                        }
                    });

                    ui.menu_button("View", |ui| {
                        ui.checkbox(&mut app.prefs.show_features, "Features");
                        ui.checkbox(&mut app.prefs.show_enzymes, "Enzymes");
                        ui.checkbox(&mut app.prefs.show_primers, "Primers");
                        ui.checkbox(&mut app.prefs.show_orfs, "ORFs");
                        ui.checkbox(&mut app.prefs.show_translations, "Translations");
                        ui.checkbox(&mut app.prefs.show_complement, "Complement strand");
                        ui.checkbox(&mut app.prefs.show_colors, "Base colors");
                        ui.checkbox(&mut app.prefs.show_zoom_controls, "Zoom controls");
                        ui.separator();
                        ui.menu_button("Show colors", |ui| {
                            ui.selectable_value(
                                &mut app.colors_mode,
                                ColorsMode::None,
                                "No colors",
                            );
                            ui.selectable_value(
                                &mut app.colors_mode,
                                ColorsMode::History,
                                "History colors",
                            );
                            ui.selectable_value(
                                &mut app.colors_mode,
                                ColorsMode::Custom,
                                "Custom DNA colors",
                            );
                        });
                        ui.separator();
                        if ui.button("Map & Sequence Options…").clicked() {
                            app.dialog = Some(Dialog::Options);
                            ui.close();
                        }
                    });

                    ui.menu_button("Features", |ui| {
                        if ui.button("Add Feature…").clicked() {
                            dialogs::open_feature_editor(app, None);
                            ui.close();
                        }
                        let sel = app.selected_feature;
                        if ui
                            .add_enabled(sel.is_some(), egui::Button::new("Edit Selected Feature…"))
                            .clicked()
                        {
                            dialogs::open_feature_editor(app, sel);
                            ui.close();
                        }
                        if ui
                            .add_enabled(
                                sel.is_some(),
                                egui::Button::new("Delete Selected Feature"),
                            )
                            .clicked()
                        {
                            if let Some(id) = sel {
                                actions::delete_feature(app, id);
                            }
                            ui.close();
                        }
                        ui.separator();
                        if ui.button("Detect Common Features…").clicked() {
                            dialogs::open_detect_common(app);
                            ui.close();
                        }
                    });

                    ui.menu_button("Primers", |ui| {
                        if ui.button("Add Primer…").clicked() {
                            dialogs::open_primer_editor(app, None);
                            ui.close();
                        }
                        if ui.button("Add Primer from Selection…").clicked() {
                            actions::primer_from_selection(app);
                            ui.close();
                        }
                        let sel = app.selected_primer;
                        if ui
                            .add_enabled(sel.is_some(), egui::Button::new("Edit Selected Primer…"))
                            .clicked()
                        {
                            dialogs::open_primer_editor(app, sel);
                            ui.close();
                        }
                        if ui
                            .add_enabled(sel.is_some(), egui::Button::new("Delete Selected Primer"))
                            .clicked()
                        {
                            if let Some(id) = sel {
                                actions::delete_primer(app, id);
                            }
                            ui.close();
                        }
                    });

                    ui.menu_button("Enzymes", |ui| {
                        if ui.button("Choose Enzymes…").clicked() {
                            app.view = View::Enzymes;
                            ui.close();
                        }
                        ui.menu_button("Apply Set", |ui| {
                            let sets = app.analysis.sets.clone();
                            for s in sets {
                                let label = format!("{} ({})", s.name, s.enzymes.len());
                                if ui.button(label).clicked() {
                                    let names = s.enzymes.clone();
                                    let n = names.len();
                                    let set_name = s.name.clone();
                                    app.mutate(
                                        &format!("Apply enzyme set \"{set_name}\""),
                                        Vec::new(),
                                        |doc| {
                                            doc.chosen_enzymes = names;
                                        },
                                    );
                                    app.toast(format!("Enzyme set: {set_name} ({n} enzymes)"));
                                    ui.close();
                                }
                            }
                        });
                    });

                    ui.menu_button("Tools", |ui| {
                        if ui.button("Genetic Codes…").clicked() {
                            app.dialog = Some(Dialog::GeneticCodes {
                                selected: app.doc.default_genetic_code,
                            });
                            ui.close();
                        }
                        if ui.button("Letter Codes…").clicked() {
                            app.dialog = Some(Dialog::LetterCodes);
                            ui.close();
                        }
                        if ui.button("Search Tips…").clicked() {
                            app.dialog = Some(Dialog::SearchTips);
                            ui.close();
                        }
                        ui.separator();
                        let circular = app.doc.sequence.is_circular();
                        if ui
                            .add_enabled(circular, egui::Button::new("Rotate Circular Sequence…"))
                            .clicked()
                        {
                            app.dialog = Some(Dialog::Rotate {
                                position: "1".into(),
                                error: None,
                            });
                            ui.close();
                        }
                        if ui.button("Reverse Complement Sequence").clicked() {
                            actions::reverse_complement_sequence(app);
                            ui.close();
                        }
                        ui.separator();
                        ui.label(RichText::new("Topology").size(11.0).color(theme::FAINT));
                        let topo = app.doc.sequence.topology;
                        let mut new = topo;
                        ui.selectable_value(&mut new, Topology::Linear, "Linear");
                        ui.selectable_value(&mut new, Topology::Circular, "Circular");
                        if new != topo {
                            actions::set_topology(app, new);
                        }
                    });

                    ui.menu_button("Help", |ui| {
                        if ui.button("About HelixBench").clicked() {
                            app.dialog = Some(Dialog::About);
                            ui.close();
                        }
                        if ui.button("Search Tips…").clicked() {
                            app.dialog = Some(Dialog::SearchTips);
                            ui.close();
                        }
                    });
                });
            });
    }

    fn tab_bar(&mut self, ui: &mut egui::Ui) {
        let app = &mut self.state;
        egui::Panel::top("hb_tabs")
            .frame(
                egui::Frame::new()
                    .fill(theme::SURFACE)
                    .stroke(Stroke::new(1.0, theme::BORDER))
                    .inner_margin(egui::Margin::symmetric(8, 4)),
            )
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    for v in View::ALL {
                        let active = app.view == v;
                        let text = RichText::new(format!("{}  {}", v.icon(), v.label()))
                            .size(13.0)
                            .strong()
                            .color(if active { theme::TEXT } else { theme::MUTED });
                        let btn = egui::Button::new(text)
                            .fill(if active {
                                theme::SURFACE_2
                            } else {
                                Color32::TRANSPARENT
                            })
                            .stroke(if active {
                                Stroke::new(1.0, theme::ACCENT)
                            } else {
                                Stroke::NONE
                            })
                            .corner_radius(egui::CornerRadius::same(6));
                        if ui.add(btn).clicked() {
                            app.view = v;
                        }
                    }
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let dirty = if app.dirty { " ●" } else { "" };
                        ui.label(
                            RichText::new(format!("{}{dirty}", app.doc.name))
                                .strong()
                                .color(theme::TEXT),
                        );
                        ui.label(
                            RichText::new(app.doc.sequence.topology.label())
                                .size(11.0)
                                .color(theme::TEAL),
                        );
                    });
                });
            });
    }

    fn side_toolbar(&mut self, ui: &mut egui::Ui) {
        let app = &mut self.state;
        egui::Panel::right("hb_toolbar")
            .exact_size(158.0)
            .resizable(false)
            .frame(
                egui::Frame::new()
                    .fill(theme::PANEL)
                    .stroke(Stroke::new(1.0, theme::BORDER))
                    .inner_margin(egui::Margin::same(8)),
            )
            .show(ui, |ui| {
                ui.add_space(2.0);
                ui.label(
                    RichText::new("SHOW")
                        .size(11.0)
                        .strong()
                        .color(theme::FAINT),
                );
                ui.add_space(4.0);
                toggle(ui, &mut app.prefs.show_features, "◧", "Features");
                toggle(ui, &mut app.prefs.show_enzymes, "✂", "Enzymes");
                toggle(ui, &mut app.prefs.show_primers, "➤", "Primers");
                toggle(ui, &mut app.prefs.show_orfs, "▤", "ORFs");
                toggle(ui, &mut app.prefs.show_translations, "Aa", "Translation");
                toggle(ui, &mut app.prefs.show_complement, "≡", "Complement");
                toggle(ui, &mut app.prefs.show_colors, "◐", "Base colors");
                toggle(ui, &mut app.prefs.show_zoom_controls, "⊕", "Zoom controls");
                ui.add_space(8.0);
                ui.separator();
                ui.add_space(6.0);
                ui.label(
                    RichText::new("COLORS")
                        .size(11.0)
                        .strong()
                        .color(theme::FAINT),
                );
                let mut mode = app.colors_mode;
                egui::ComboBox::from_id_salt("tb_colors")
                    .selected_text(mode.label())
                    .width(140.0)
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut mode, ColorsMode::None, "No colors");
                        ui.selectable_value(&mut mode, ColorsMode::History, "History colors");
                        ui.selectable_value(&mut mode, ColorsMode::Custom, "Custom DNA colors");
                    });
                app.colors_mode = mode;
                ui.add_space(8.0);
                ui.separator();
                ui.add_space(6.0);
                if ui
                    .add_sized([142.0, 26.0], egui::Button::new("🔍  Find"))
                    .clicked()
                {
                    app.search.open = !app.search.open;
                    if app.search.open {
                        app.search.focus_request = true;
                    }
                }
                if ui
                    .add_sized([142.0, 26.0], egui::Button::new("⚙  Options"))
                    .clicked()
                {
                    app.dialog = Some(Dialog::Options);
                }
                ui.add_space(8.0);
                ui.separator();
                ui.add_space(6.0);
                ui.horizontal(|ui| {
                    if ui
                        .add_enabled(!app.history.is_empty(), egui::Button::new("↶"))
                        .on_hover_text("Undo (Ctrl+Z)")
                        .clicked()
                    {
                        app.undo();
                    }
                    if ui
                        .add_enabled(!app.future.is_empty(), egui::Button::new("↷"))
                        .on_hover_text("Redo (Ctrl+Y)")
                        .clicked()
                    {
                        app.redo();
                    }
                });
            });
    }

    fn status_bar(&mut self, ui: &mut egui::Ui) {
        let app = &mut self.state;
        egui::Panel::bottom("hb_status")
            .frame(
                egui::Frame::new()
                    .fill(theme::SURFACE)
                    .stroke(Stroke::new(1.0, theme::BORDER))
                    .inner_margin(egui::Margin::symmetric(12, 4)),
            )
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    if let Some(sel) = app.selection {
                        let bases = app.doc.sequence.region(sel);
                        let gc = gc_percent(&bases);
                        let (a, b) = (sel.start + 1, sel.start + sel.len);
                        let rng = if sel.len == 1 {
                            format!("{a}")
                        } else {
                            format!("{a}..{b}")
                        };
                        ui.label(
                            RichText::new(format!("Sel {rng} · {} bp · GC {gc:.1}%", sel.len))
                                .monospace()
                                .size(12.0)
                                .color(theme::ACCENT),
                        );
                    } else {
                        ui.label(RichText::new("No selection").size(12.0).color(theme::FAINT));
                    }
                    ui.separator();
                    let l = app.doc.sequence.len();
                    let gc = gc_percent(&app.doc.sequence.bases);
                    ui.label(
                        RichText::new(format!(
                            "{l} bp · {} · GC {gc:.1}%",
                            app.doc.sequence.topology.label()
                        ))
                        .monospace()
                        .size(12.0)
                        .color(theme::MUTED),
                    );
                    ui.separator();
                    ui.label(
                        RichText::new(format!(
                            "{} features · {} primers · {} enzymes",
                            app.doc.features.len(),
                            app.doc.primers.len(),
                            app.analysis.chosen.len()
                        ))
                        .size(12.0)
                        .color(theme::MUTED),
                    );
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if app.dirty {
                            ui.label(RichText::new("● Unsaved").size(12.0).color(theme::WARN));
                        } else if app.path.is_some() {
                            ui.label(RichText::new("✓ Saved").size(12.0).color(theme::OK));
                        } else {
                            ui.label(RichText::new("Demo").size(12.0).color(theme::FAINT));
                        }
                        if let Some(p) = &app.path {
                            ui.label(
                                RichText::new(p.display().to_string())
                                    .size(11.0)
                                    .color(theme::FAINT),
                            )
                            .on_hover_text("Document path");
                        }
                    });
                });
            });
    }

    fn toast(&mut self, ctx: &egui::Context) {
        let app = &mut self.state;
        let Some((msg, when)) = app.toast.clone() else {
            return;
        };
        let elapsed = when.elapsed().as_secs_f32();
        if elapsed > 3.5 {
            app.toast = None;
            return;
        }
        let alpha = if elapsed > 3.0 {
            ((3.5 - elapsed) / 0.5).clamp(0.0, 1.0)
        } else {
            1.0
        };
        egui::Area::new(egui::Id::unique("hb_toast"))
            .order(egui::Order::Foreground)
            .anchor(egui::Align2::CENTER_BOTTOM, [0.0, -64.0])
            .show(ctx, |ui| {
                egui::Frame::new()
                    .fill(theme::SURFACE_2)
                    .stroke(Stroke::new(1.0, theme::ACCENT))
                    .corner_radius(egui::CornerRadius::same(8))
                    .inner_margin(egui::Margin::symmetric(14, 9))
                    .shadow(egui::epaint::Shadow {
                        offset: [0, 4],
                        blur: 12,
                        spread: 0,
                        color: Color32::from_black_alpha(120),
                    })
                    .show(ui, |ui| {
                        // Anchored areas reuse last frame's width; pin a readable wrap width
                        // so a long message after a short one doesn't collapse to a column.
                        ui.set_max_width(420.0);
                        ui.label(
                            RichText::new(msg)
                                .size(12.5)
                                .color(theme::TEXT.gamma_multiply(alpha)),
                        );
                    });
            });
        ctx.request_repaint_after(Duration::from_secs_f32((3.5 - elapsed).max(0.05)));
    }

    fn update_title(&mut self, ctx: &egui::Context) {
        let app = &self.state;
        let title = format!(
            "HelixBench — {}{}",
            app.doc.name,
            if app.dirty { " •" } else { "" }
        );
        if title != self.last_title {
            ctx.send_viewport_cmd(egui::ViewportCommand::Title(title.clone()));
            self.last_title = title;
        }
    }
}

fn toggle(ui: &mut egui::Ui, on: &mut bool, icon: &str, label: &str) {
    let active = *on;
    let text = RichText::new(format!("{icon}  {label}"))
        .size(12.5)
        .color(if active { theme::TEXT } else { theme::MUTED });
    let btn = egui::Button::new(text)
        .fill(if active {
            theme::SURFACE_2
        } else {
            Color32::TRANSPARENT
        })
        .stroke(if active {
            Stroke::new(1.0, theme::BORDER)
        } else {
            Stroke::NONE
        })
        .corner_radius(egui::CornerRadius::same(5));
    if ui.add_sized([142.0, 24.0], btn).clicked() {
        *on = !*on;
    }
}

fn gc_percent(bases: &[u8]) -> f32 {
    if bases.is_empty() {
        return 0.0;
    }
    let gc = bases.iter().filter(|&&b| b == b'G' || b == b'C').count();
    gc as f32 / bases.len() as f32 * 100.0
}

fn paste_from_clipboard(app: &mut AppState) {
    if let Ok(mut cb) = arboard::Clipboard::new() {
        if let Ok(text) = cb.get_text() {
            actions::insert_bases(app, &text, true);
        }
    }
}
