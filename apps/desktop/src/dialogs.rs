//! Modal dialogs: editors, choosers, reference tables and options.

use egui::{Align2, Color32, FontId, RichText, Stroke};

use domain::feature::{kind_color, Direction, Feature, FEATURE_KINDS};
use domain::primer::{binding_sites, melting_temperature, HybridizationParams, Primer};
use domain::sequence::clean_sequence;
use domain::{Range, StrandMask, Topology};

use crate::state::{
    AppState, ColorDraft, ColorsMode, Dialog, FeatureDraft, PendingAction, PrimerDraft,
};
use crate::theme;

// ---------------------------------------------------------------------------
// Openers
// ---------------------------------------------------------------------------

pub fn open_feature_editor(app: &mut AppState, id: Option<u64>) {
    let draft = if let Some(f) = id.and_then(|i| app.doc.feature(i)) {
        FeatureDraft {
            id: Some(f.id),
            name: f.name.clone(),
            kind: f.kind.clone(),
            segments_text: f
                .segments
                .iter()
                .map(|s| format!("{}..{}", s.start + 1, s.start + s.len))
                .collect::<Vec<_>>()
                .join(", "),
            direction: f.direction,
            color: f.color,
            note: f.note.clone(),
            product: f.product.clone(),
            translated: f.translated,
            genetic_code: f.genetic_code,
            first_codon_met: f.first_codon_met,
            frame_offset: f.frame_offset,
            error: None,
        }
    } else {
        let (seg_text, direction) = match app.selection {
            Some(sel) if sel.len > 0 => (
                format!("{}..{}", sel.start + 1, sel.start + sel.len),
                Direction::Forward,
            ),
            _ => (String::new(), Direction::None),
        };
        FeatureDraft {
            id: None,
            name: String::new(),
            kind: "misc_feature".into(),
            segments_text: seg_text,
            direction,
            color: kind_color("misc_feature"),
            note: String::new(),
            product: String::new(),
            translated: false,
            genetic_code: 0,
            first_codon_met: false,
            frame_offset: 0,
            error: None,
        }
    };
    app.dialog = Some(Dialog::FeatureEditor(draft));
}

pub fn open_primer_editor(app: &mut AppState, id: Option<u64>) {
    let draft = if let Some(p) = id.and_then(|i| app.doc.primer(i)) {
        PrimerDraft {
            id: Some(p.id),
            name: p.name.clone(),
            sequence: String::from_utf8_lossy(&p.sequence).into_owned(),
            description: p.description.clone(),
            color: p.color,
            phosphorylated: p.phosphorylated,
            error: None,
        }
    } else {
        PrimerDraft {
            id: None,
            name: format!("Primer {}", app.doc.primers.len() + 1),
            sequence: String::new(),
            description: String::new(),
            color: [0x79, 0xc0, 0xff],
            phosphorylated: false,
            error: None,
        }
    };
    app.dialog = Some(Dialog::PrimerEditor(draft));
}

pub fn open_detect_common(app: &mut AppState) {
    app.ensure_analysis();
    let detected = domain::common_features::detect(&app.doc.sequence);
    let checked: Vec<bool> = detected
        .iter()
        .map(|d| !domain::common_features::is_duplicate(&app.doc.features, d))
        .collect();
    if detected.is_empty() {
        app.toast("No known common features found in this sequence");
        return;
    }
    app.dialog = Some(Dialog::DetectCommon { detected, checked });
}

// ---------------------------------------------------------------------------
// Rendering
// ---------------------------------------------------------------------------

pub fn show(app: &mut AppState, ctx: &egui::Context) {
    if app.dialog.is_none() {
        return;
    }
    // Scrim behind the modal.
    let screen = ctx.viewport_rect();
    let scrim_id = egui::Id::unique("hb_modal_scrim");
    let scrim = egui::Area::new(scrim_id)
        .order(egui::Order::Middle)
        .fixed_pos(screen.min)
        .show(ctx, |ui| {
            let (rect, resp) = ui.allocate_exact_size(screen.size(), egui::Sense::click());
            ui.painter()
                .rect_filled(rect, 0.0, Color32::from_black_alpha(150));
            resp
        })
        .inner;
    let mut close = scrim.clicked();

    let mut keep = true;
    let dialog = app.dialog.take();
    match dialog {
        Some(Dialog::NewSequence(mut s)) => {
            window(ctx, "New Sequence", 520.0, |ui| {
                new_sequence_ui(app, ui, &mut s, &mut keep);
            });
            if keep {
                app.dialog = Some(Dialog::NewSequence(s));
            }
        }
        Some(Dialog::FeatureEditor(mut d)) => {
            window(
                ctx,
                if d.id.is_some() {
                    "Edit Feature"
                } else {
                    "Add Feature"
                },
                560.0,
                |ui| {
                    feature_editor_ui(app, ui, &mut d, &mut keep);
                },
            );
            if keep {
                app.dialog = Some(Dialog::FeatureEditor(d));
            }
        }
        Some(Dialog::PrimerEditor(mut d)) => {
            window(
                ctx,
                if d.id.is_some() {
                    "Edit Primer"
                } else {
                    "Add Primer"
                },
                560.0,
                |ui| {
                    primer_editor_ui(app, ui, &mut d, &mut keep);
                },
            );
            if keep {
                app.dialog = Some(Dialog::PrimerEditor(d));
            }
        }
        Some(Dialog::GeneticCodes { mut selected }) => {
            window(ctx, "Genetic Codes", 640.0, |ui| {
                genetic_codes_ui(app, ui, &mut selected, &mut keep);
            });
            if keep {
                app.dialog = Some(Dialog::GeneticCodes { selected });
            }
        }
        Some(Dialog::LetterCodes) => {
            window(ctx, "Letter Codes", 520.0, |ui| {
                letter_codes_ui(ui);
                ui.add_space(6.0);
                ui.horizontal(|ui| {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("Close").clicked() {
                            keep = false;
                        }
                    });
                });
            });
            if keep {
                app.dialog = Some(Dialog::LetterCodes);
            }
        }
        Some(Dialog::SearchTips) => {
            window(ctx, "Search Tips", 560.0, |ui| {
                search_tips_ui(ui);
                ui.add_space(6.0);
                ui.horizontal(|ui| {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("Close").clicked() {
                            keep = false;
                        }
                    });
                });
            });
            if keep {
                app.dialog = Some(Dialog::SearchTips);
            }
        }
        Some(Dialog::About) => {
            window(ctx, "About HelixBench", 480.0, |ui| {
                about_ui(ui);
                ui.add_space(6.0);
                ui.horizontal(|ui| {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("Close").clicked() {
                            keep = false;
                        }
                    });
                });
            });
            if keep {
                app.dialog = Some(Dialog::About);
            }
        }
        Some(Dialog::Options) => {
            window(ctx, "Map & Sequence Options", 480.0, |ui| {
                options_ui(app, ui);
                ui.add_space(6.0);
                ui.horizontal(|ui| {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("Done").clicked() {
                            keep = false;
                        }
                    });
                });
            });
            if keep && app.dialog.is_none() {
                app.dialog = Some(Dialog::Options);
            }
        }
        Some(Dialog::Rotate {
            mut position,
            mut error,
        }) => {
            window(ctx, "Rotate Circular Sequence", 400.0, |ui| {
                rotate_ui(app, ui, &mut position, &mut error, &mut keep);
            });
            if keep {
                app.dialog = Some(Dialog::Rotate { position, error });
            }
        }
        Some(Dialog::Unsaved { action }) => {
            close = false; // must choose explicitly
            window(ctx, "Unsaved Changes", 420.0, |ui| {
                unsaved_ui(app, ui, action.clone(), &mut keep);
            });
            if keep {
                app.dialog = Some(Dialog::Unsaved { action });
            }
        }
        Some(Dialog::DetectCommon {
            detected,
            mut checked,
        }) => {
            window(ctx, "Detect Common Features", 560.0, |ui| {
                detect_common_ui(app, ui, &detected, &mut checked, &mut keep);
            });
            if keep {
                app.dialog = Some(Dialog::DetectCommon { detected, checked });
            }
        }
        Some(Dialog::ColorSelection(mut d)) => {
            window(ctx, "Color Selection", 400.0, |ui| {
                color_selection_ui(app, ui, &mut d, &mut keep);
            });
            if keep {
                app.dialog = Some(Dialog::ColorSelection(d));
            }
        }
        Some(Dialog::Pcr(mut d)) => {
            window(ctx, "In-silico PCR", 460.0, |ui| {
                ui.label("Forward primer (name or 5'-3'):");
                ui.text_edit_singleline(&mut d.fwd);
                ui.label("Reverse primer (name or 5'-3'):");
                ui.text_edit_singleline(&mut d.rev);
                ui.checkbox(&mut d.copy_features, "Copy contained features into product");
                if let Some(p) = appcore::actions::pcr_preview(app, &d) {
                    ui.label(format!(
                        "Amplicon {} bp · {}..{} · fwd Tm {:.1} · rev Tm {:.1}",
                        p.len,
                        p.start + 1,
                        p.end,
                        p.fwd_site.tm,
                        p.rev_site.tm
                    ));
                } else {
                    ui.label("No amplicon yet — primers must bind in opposite orientations.");
                }
                if let Some(e) = &d.error {
                    ui.colored_label(egui::Color32::RED, e);
                }
                ui.horizontal(|ui| {
                    if ui.button("Cancel").clicked() {
                        keep = false;
                    }
                    if ui.button("Create product").clicked() {
                        match appcore::actions::create_pcr_product(app, &d) {
                            Ok(()) => keep = false,
                            Err(e) => d.error = Some(e),
                        }
                    }
                });
            });
            if keep {
                app.dialog = Some(Dialog::Pcr(d));
            }
        }
        Some(Dialog::PrimerDesign(mut d)) => {
            window(ctx, "Primer Design", 560.0, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Start");
                    ui.text_edit_singleline(&mut d.start);
                    ui.label("Length");
                    ui.text_edit_singleline(&mut d.len);
                    ui.label("Tm");
                    ui.text_edit_singleline(&mut d.target_tm);
                });
                ui.horizontal(|ui| {
                    ui.label("5' overhang");
                    ui.text_edit_singleline(&mut d.overhang);
                    if ui.button("Design").clicked() {
                        appcore::actions::run_primer_design(app, &mut d);
                    }
                });
                if let Some(e) = &d.error {
                    ui.colored_label(egui::Color32::RED, e);
                }
                for (label, cands, pick, is_fwd) in [
                    ("Forward", &d.fwd, &mut d.fwd_pick, true),
                    ("Reverse", &d.rev, &mut d.rev_pick, false),
                ] {
                    ui.label(label);
                    for (i, c) in cands.iter().enumerate() {
                        let seq = String::from_utf8_lossy(&c.sequence).into_owned();
                        let txt = format!("{}  ({}bp Tm {:.1} GC {:.0}%)", seq, c.len, c.tm, c.gc);
                        if ui.radio(*pick == i, txt).clicked() {
                            *pick = i;
                        }
                        let _ = is_fwd;
                    }
                }
                ui.horizontal(|ui| {
                    if ui.button("Cancel").clicked() {
                        keep = false;
                    }
                    if ui.button("Add primer pair").clicked() {
                        match appcore::actions::add_designed_primers(app, &d) {
                            Ok(()) => keep = false,
                            Err(e) => d.error = Some(e),
                        }
                    }
                });
            });
            if keep {
                app.dialog = Some(Dialog::PrimerDesign(d));
            }
        }
        Some(Dialog::Digest) => {
            window(ctx, "Virtual Digest & Gel", 460.0, |ui| {
                let frags = appcore::actions::digest_fragments(app);
                let total: usize = frags.iter().map(|f| f.size).sum();
                ui.label(format!("{} fragment(s) · {} bp total", frags.len(), total));
                egui::ScrollArea::vertical()
                    .max_height(300.0)
                    .show(ui, |ui| {
                        for (i, f) in frags.iter().enumerate() {
                            ui.label(format!(
                                "#{}  {} bp  ({}..{})",
                                i + 1,
                                f.size,
                                f.start + 1,
                                f.end
                            ));
                        }
                    });
                if ui.button("Close").clicked() {
                    keep = false;
                }
            });
            if keep {
                app.dialog = Some(Dialog::Digest);
            }
        }
        Some(Dialog::Alignment(mut d)) => {
            window(ctx, "Sequence Alignment", 640.0, |ui| {
                ui.label("Reference document:");
                let mut names = vec![app.doc.name.clone()];
                for od in &app.library {
                    names.push(od.doc.name.clone());
                }
                for (i, n) in names.iter().enumerate() {
                    if ui.radio(d.reference == i, n).clicked() {
                        d.reference = i;
                    }
                }
                ui.checkbox(&mut d.reindex, "Auto-reindex circular sequences");
                if ui.button("Align").clicked() {
                    appcore::actions::run_alignment(app, &mut d);
                }
                if let Some(e) = &d.error {
                    ui.colored_label(egui::Color32::RED, e);
                }
                if let Some(al) = &d.result {
                    ui.label(format!(
                        "{} columns · identity {:.1}%",
                        al.consensus.len(),
                        al.identity
                    ));
                    egui::ScrollArea::both().max_height(300.0).show(ui, |ui| {
                        for (i, row) in al.rows.iter().enumerate() {
                            ui.monospace(format!(
                                "{}: {}",
                                names.get(i).cloned().unwrap_or_default(),
                                String::from_utf8_lossy(row)
                            ));
                        }
                        ui.monospace(format!(
                            "Consensus: {}",
                            String::from_utf8_lossy(&al.consensus)
                        ));
                    });
                }
                if ui.button("Close").clicked() {
                    keep = false;
                }
            });
            if keep {
                app.dialog = Some(Dialog::Alignment(d));
            }
        }
        Some(Dialog::Assembly(mut d)) => {
            window(ctx, "Assembly Wizard", 480.0, |ui| {
                ui.horizontal(|ui| {
                    for (i, label) in ["Digest & Ligate", "Gibson", "Golden Gate"]
                        .iter()
                        .enumerate()
                    {
                        if ui.radio(d.strategy == i, *label).clicked() {
                            d.strategy = i;
                        }
                    }
                });
                ui.horizontal(|ui| {
                    ui.label("Cut position");
                    ui.text_edit_singleline(&mut d.cut);
                });
                ui.label("Inserts:");
                for (i, od) in app.library.iter().enumerate() {
                    let mut on = d.inserts.get(i).copied().unwrap_or(false);
                    if ui.checkbox(&mut on, &od.doc.name).changed() {
                        if let Some(b) = d.inserts.get_mut(i) {
                            *b = on;
                        }
                    }
                }
                if let Some(e) = &d.error {
                    ui.colored_label(egui::Color32::RED, e);
                }
                ui.horizontal(|ui| {
                    if ui.button("Cancel").clicked() {
                        keep = false;
                    }
                    if ui.button("Assemble").clicked() {
                        match appcore::actions::create_assembly(app, &d) {
                            Ok(()) => keep = false,
                            Err(e) => d.error = Some(e),
                        }
                    }
                });
            });
            if keep {
                app.dialog = Some(Dialog::Assembly(d));
            }
        }
        Some(Dialog::Crispr(mut d)) => {
            window(ctx, "CRISPR Guides", 620.0, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Max guides");
                    ui.text_edit_singleline(&mut d.limit);
                    if ui.button("Scan").clicked() {
                        appcore::actions::run_crispr(app, &mut d);
                    }
                });
                if let Some(e) = &d.error {
                    ui.colored_label(egui::Color32::RED, e);
                }
                egui::ScrollArea::vertical()
                    .max_height(300.0)
                    .show(ui, |ui| {
                        for g in &d.guides {
                            ui.monospace(format!(
                                "{} {} {} score {:.2} off:{}",
                                String::from_utf8_lossy(&g.sequence),
                                g.pam,
                                if g.strand == domain::Strand::Top {
                                    "+"
                                } else {
                                    "-"
                                },
                                g.score,
                                g.off_targets
                            ));
                        }
                    });
                ui.horizontal(|ui| {
                    if ui.button("Close").clicked() {
                        keep = false;
                    }
                    if ui.button("Add as features").clicked() {
                        appcore::actions::add_crispr_features(app, &d);
                        keep = false;
                    }
                });
            });
            if keep {
                app.dialog = Some(Dialog::Crispr(d));
            }
        }
        Some(Dialog::Codon(mut d)) => {
            window(ctx, "Codon Optimization", 480.0, |ui| {
                ui.horizontal(|ui| {
                    for (i, label) in ["E. coli", "Human"].iter().enumerate() {
                        if ui.radio(d.host == i, *label).clicked() {
                            d.host = i;
                        }
                    }
                });
                if ui.button("Optimize").clicked() {
                    appcore::actions::run_codon(app, &mut d);
                }
                if let Some(e) = &d.error {
                    ui.colored_label(egui::Color32::RED, e);
                }
                if d.result.is_some() {
                    ui.label(format!(
                        "CAI before {:.2} → after {:.2}",
                        d.cai_before, d.cai_after
                    ));
                }
                ui.horizontal(|ui| {
                    if ui.button("Close").clicked() {
                        keep = false;
                    }
                    if ui.button("Create optimized product").clicked() {
                        match appcore::actions::apply_codon(app, &d) {
                            Ok(()) => keep = false,
                            Err(e) => d.error = Some(e),
                        }
                    }
                });
            });
            if keep {
                app.dialog = Some(Dialog::Codon(d));
            }
        }
        Some(Dialog::Blast(mut d)) => {
            window(ctx, "Local BLAST", 620.0, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Word size");
                    ui.text_edit_singleline(&mut d.k);
                    ui.label("Max hits");
                    ui.text_edit_singleline(&mut d.max);
                    if ui.button("Search").clicked() {
                        appcore::actions::run_blast(app, &mut d);
                    }
                });
                if let Some(e) = &d.error {
                    ui.colored_label(egui::Color32::RED, e);
                }
                egui::ScrollArea::vertical()
                    .max_height(300.0)
                    .show(ui, |ui| {
                        for h in &d.hits {
                            ui.monospace(format!(
                                "{} q{}..{} s{}..{} score {} id {:.0}%",
                                h.name,
                                h.qstart + 1,
                                h.qend,
                                h.sstart + 1,
                                h.send,
                                h.score,
                                h.identity
                            ));
                        }
                    });
                if ui.button("Close").clicked() {
                    keep = false;
                }
            });
            if keep {
                app.dialog = Some(Dialog::Blast(d));
            }
        }
        Some(Dialog::Combinatorial(mut d)) => {
            window(ctx, "Combinatorial Assembly", 620.0, |ui| {
                ui.horizontal(|ui| {
                    for (i, label) in ["Digest & Ligate", "Gibson", "Golden Gate"]
                        .iter()
                        .enumerate()
                    {
                        if ui.radio(d.strategy == i, *label).clicked() {
                            d.strategy = i;
                        }
                    }
                });
                ui.horizontal(|ui| {
                    ui.label("Cut");
                    ui.text_edit_singleline(&mut d.cut);
                    ui.label("Cap");
                    ui.text_edit_singleline(&mut d.cap);
                    if ui.button("Generate").clicked() {
                        appcore::actions::run_combinatorial(app, &mut d);
                    }
                });
                if let Some(e) = &d.error {
                    ui.colored_label(egui::Color32::RED, e);
                }
                egui::ScrollArea::vertical()
                    .max_height(300.0)
                    .show(ui, |ui| {
                        for (i, asm) in d.results.iter().enumerate() {
                            ui.horizontal(|ui| {
                                ui.label(d.labels.get(i).cloned().unwrap_or_default());
                                ui.label(format!("{} bp", asm.bases.len()));
                                if ui.button("Create").clicked() {
                                    let _ = appcore::actions::create_combinatorial(app, &d, i);
                                    keep = false;
                                }
                            });
                        }
                    });
                if ui.button("Close").clicked() {
                    keep = false;
                }
            });
            if keep {
                app.dialog = Some(Dialog::Combinatorial(d));
            }
        }
        Some(Dialog::Registry(mut d)) => {
            window(ctx, "Local Registry", 700.0, |ui| {
                ui.horizontal(|ui| {
                    for (i, label) in ["All", "Plasmid", "Strain", "Primer"].iter().enumerate() {
                        if ui.radio(d.filter == i, *label).clicked() {
                            d.filter = i;
                        }
                    }
                    if ui.button("Add entry").clicked() {
                        let kind = d.filter.saturating_sub(1);
                        appcore::actions::open_registry_editor(app, kind, None);
                    }
                });
                egui::ScrollArea::vertical()
                    .max_height(360.0)
                    .show(ui, |ui| {
                        let entries: Vec<domain::registry::Entry> = app
                            .registry
                            .entries
                            .iter()
                            .filter(|e| d.filter == 0 || (e.kind as usize) + 1 == d.filter)
                            .cloned()
                            .collect();
                        for e in entries {
                            ui.horizontal(|ui| {
                                ui.label(e.kind.label());
                                ui.label(e.name());
                                ui.label(e.linked_doc.clone().unwrap_or_default());
                                if ui.button("Edit").clicked() {
                                    appcore::actions::open_registry_editor(
                                        app,
                                        e.kind as usize,
                                        Some(e.id),
                                    );
                                }
                                if ui.button("Link").clicked() {
                                    appcore::actions::link_registry_entry(app, e.id);
                                }
                                if ui.button("Open").clicked() {
                                    appcore::actions::open_linked_entry(app, e.id);
                                }
                                if ui.button("Delete").clicked() {
                                    appcore::actions::delete_registry_entry(app, e.id);
                                }
                            });
                        }
                    });
                if ui.button("Close").clicked() {
                    keep = false;
                }
            });
            if keep {
                app.dialog = Some(Dialog::Registry(d));
            }
        }
        Some(Dialog::RegistryEditor(mut d)) => {
            window(ctx, "Registry Entry", 520.0, |ui| {
                if d.id.is_none() {
                    ui.horizontal(|ui| {
                        for (i, k) in domain::registry::Kind::ALL.iter().enumerate() {
                            if ui.radio(d.kind == i, k.label()).clicked() {
                                d.kind = i;
                                d.values = vec![String::new(); k.fields().len()];
                            }
                        }
                    });
                }
                let fields = domain::registry::Kind::ALL[d.kind.min(2)].fields();
                for (i, f) in fields.iter().enumerate() {
                    ui.horizontal(|ui| {
                        ui.label(*f);
                        if let Some(v) = d.values.get_mut(i) {
                            ui.text_edit_singleline(v);
                        }
                    });
                }
                if let Some(e) = &d.error {
                    ui.colored_label(egui::Color32::RED, e);
                }
                ui.horizontal(|ui| {
                    if ui.button("Cancel").clicked() {
                        appcore::actions::open_registry(app);
                    }
                    if ui.button("Save").clicked() {
                        match appcore::actions::save_registry_entry(app, &d) {
                            Ok(()) => appcore::actions::open_registry(app),
                            Err(e) => d.error = Some(e),
                        }
                    }
                });
            });
            if keep {
                app.dialog = Some(Dialog::RegistryEditor(d));
            }
        }
        None => {}
    }
    if close {
        app.dialog = None;
    }
}

fn window(ctx: &egui::Context, title: &str, width: f32, add: impl FnOnce(&mut egui::Ui)) {
    egui::Window::new(title)
        .id(egui::Id::unique(("hb_dialog", title)))
        .order(egui::Order::Foreground)
        .collapsible(false)
        .resizable(false)
        .default_width(width)
        .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
        .show(ctx, add);
}

fn heading(ui: &mut egui::Ui, text: &str) {
    ui.label(RichText::new(text).size(12.0).strong().color(theme::ACCENT));
}

fn error_label(ui: &mut egui::Ui, err: &Option<String>) {
    if let Some(e) = err {
        ui.label(
            RichText::new(format!("⚠ {e}"))
                .color(theme::DANGER)
                .size(12.0),
        );
    }
}

// ---------------------------------------------------------------------------
// New sequence
// ---------------------------------------------------------------------------

fn new_sequence_ui(
    app: &mut AppState,
    ui: &mut egui::Ui,
    s: &mut crate::state::NewSeqState,
    keep: &mut bool,
) {
    egui::Grid::new("new_seq_grid")
        .num_columns(2)
        .spacing([10.0, 8.0])
        .show(ui, |ui| {
            ui.label("Name");
            ui.add(egui::TextEdit::singleline(&mut s.name).desired_width(320.0));
            ui.end_row();
            ui.label("Topology");
            ui.horizontal(|ui| {
                ui.selectable_value(&mut s.topology, Topology::Linear, "Linear");
                ui.selectable_value(&mut s.topology, Topology::Circular, "Circular");
            });
            ui.end_row();
        });
    ui.add_space(4.0);
    heading(ui, "SEQUENCE (PASTE OR TYPE — LETTERS ONLY)");
    let resp = ui.add(
        egui::TextEdit::multiline(&mut s.text)
            .font(FontId::monospace(12.5))
            .desired_rows(8)
            .desired_width(f32::INFINITY)
            .hint_text("ATGCCC…  (whitespace and numbers are ignored)"),
    );
    if !resp.has_focus() && s.text.is_empty() {
        // keep hint visible
    }
    let cleaned = clean_sequence(&s.text);
    ui.horizontal(|ui| {
        ui.label(
            RichText::new(format!("{} bp", cleaned.len()))
                .monospace()
                .color(theme::MUTED),
        );
        if !cleaned.is_empty() {
            let gc = cleaned.iter().filter(|&&b| b == b'G' || b == b'C').count() as f32
                / cleaned.len() as f32
                * 100.0;
            ui.label(
                RichText::new(format!("GC {gc:.1}%"))
                    .monospace()
                    .color(theme::MUTED),
            );
        }
    });
    error_label(ui, &s.error);
    ui.add_space(6.0);
    ui.horizontal(|ui| {
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui
                .add(egui::Button::new(RichText::new("Create").strong()))
                .clicked()
            {
                match crate::actions::create_from_dialog(app, s) {
                    Ok(()) => *keep = false,
                    Err(e) => s.error = Some(e),
                }
            }
            if ui.button("Cancel").clicked() {
                *keep = false;
            }
        });
    });
}

// ---------------------------------------------------------------------------
// Feature editor
// ---------------------------------------------------------------------------

fn feature_editor_ui(app: &mut AppState, ui: &mut egui::Ui, d: &mut FeatureDraft, keep: &mut bool) {
    let l = app.doc.sequence.len();
    egui::Grid::new("feat_grid")
        .num_columns(2)
        .spacing([10.0, 7.0])
        .show(ui, |ui| {
            ui.label("Name");
            ui.add(
                egui::TextEdit::singleline(&mut d.name)
                    .desired_width(340.0)
                    .hint_text("e.g. GFP, T7 promoter…"),
            );
            ui.end_row();
            ui.label("Kind");
            ui.horizontal(|ui| {
                let before = d.kind.clone();
                egui::ComboBox::from_id_salt("feat_kind")
                    .selected_text(&d.kind)
                    .width(180.0)
                    .show_ui(ui, |ui| {
                        for k in FEATURE_KINDS {
                            ui.selectable_value(&mut d.kind, k.to_string(), *k);
                        }
                    });
                if d.kind != before {
                    d.color = kind_color(&d.kind);
                    d.translated = d.kind == "CDS";
                }
                ui.label("Color");
                egui::widgets::color_picker::color_edit_button_srgb(ui, &mut d.color);
            });
            ui.end_row();
            ui.label("Direction");
            ui.horizontal(|ui| {
                for (dir, label) in [
                    (Direction::None, "— none"),
                    (Direction::Forward, "→ forward"),
                    (Direction::Reverse, "← reverse"),
                    (Direction::Bidirectional, "↔ both"),
                ] {
                    ui.selectable_value(&mut d.direction, dir, label);
                }
            });
            ui.end_row();
            ui.label("Segments");
            ui.add(
                egui::TextEdit::singleline(&mut d.segments_text)
                    .desired_width(340.0)
                    .font(FontId::monospace(12.5))
                    .hint_text("100..200, 300..400  (1-based; comma-separated exons)"),
            );
            ui.end_row();
            ui.label("Product");
            ui.add(egui::TextEdit::singleline(&mut d.product).desired_width(340.0));
            ui.end_row();
            ui.label("Note");
            ui.add(egui::TextEdit::singleline(&mut d.note).desired_width(340.0));
            ui.end_row();
        });
    ui.add_space(6.0);
    ui.separator();
    heading(ui, "TRANSLATION");
    ui.checkbox(&mut d.translated, "Translate this feature");
    if d.translated {
        egui::Grid::new("feat_trans_grid")
            .num_columns(2)
            .spacing([10.0, 6.0])
            .show(ui, |ui| {
                ui.label("Genetic code");
                let current = if d.genetic_code == 0 {
                    format!(
                        "Document default ({})",
                        domain::genetic_code::code_by_id(app.doc.default_genetic_code).name
                    )
                } else {
                    domain::genetic_code::code_by_id(d.genetic_code)
                        .name
                        .to_string()
                };
                egui::ComboBox::from_id_salt("feat_code")
                    .selected_text(current)
                    .width(300.0)
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut d.genetic_code,
                            0u8,
                            format!(
                                "Document default ({})",
                                domain::genetic_code::code_by_id(app.doc.default_genetic_code).name
                            ),
                        );
                        for c in domain::genetic_code::CODES {
                            ui.selectable_value(
                                &mut d.genetic_code,
                                c.id,
                                format!("{} — {}", c.id, c.name),
                            );
                        }
                    });
                ui.end_row();
                ui.label("First codon");
                ui.checkbox(&mut d.first_codon_met, "Translate first codon as M");
                ui.end_row();
                ui.label("Frame offset");
                ui.horizontal(|ui| {
                    for off in 0..3usize {
                        ui.selectable_value(&mut d.frame_offset, off, format!("+{off}"));
                    }
                });
                ui.end_row();
            });
        // Live preview
        if let Ok(segments) =
            crate::actions::parse_segments(&d.segments_text, l, app.doc.sequence.is_circular())
        {
            let mut f = Feature::new(0, d.name.clone(), d.kind.clone(), segments);
            f.direction = d.direction;
            f.translated = true;
            f.genetic_code = d.genetic_code;
            f.first_codon_met = d.first_codon_met;
            f.frame_offset = d.frame_offset;
            let code = domain::genetic_code::code_by_id(app.doc.default_genetic_code);
            let aa = f.translation(&app.doc.sequence, code);
            let stops = aa.iter().filter(|&&a| a == b'*').count();
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new(format!("{} aa", aa.len()))
                        .monospace()
                        .color(theme::MUTED),
                );
                if stops > 0 && f.has_internal_stop(&app.doc.sequence, code) {
                    ui.label(
                        RichText::new(format!("⚠ {} stop codon(s) inside", stops))
                            .color(theme::WARN)
                            .size(12.0),
                    );
                } else if !aa.is_empty() {
                    ui.label(
                        RichText::new("✓ no internal stops")
                            .color(theme::OK)
                            .size(12.0),
                    );
                }
            });
            let text = String::from_utf8_lossy(&aa);
            ui.add(
                egui::Label::new(
                    RichText::new(text.as_ref())
                        .monospace()
                        .size(11.5)
                        .color(theme::MUTED),
                )
                .wrap(),
            );
        }
    }
    error_label(ui, &d.error);
    ui.add_space(6.0);
    ui.horizontal(|ui| {
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui
                .add(egui::Button::new(RichText::new("Save").strong()))
                .clicked()
            {
                match crate::actions::save_feature_draft(app, d) {
                    Ok(()) => *keep = false,
                    Err(e) => d.error = Some(e),
                }
            }
            if ui.button("Cancel").clicked() {
                *keep = false;
            }
        });
    });
}

// ---------------------------------------------------------------------------
// Primer editor
// ---------------------------------------------------------------------------

fn primer_editor_ui(app: &mut AppState, ui: &mut egui::Ui, d: &mut PrimerDraft, keep: &mut bool) {
    egui::Grid::new("primer_grid")
        .num_columns(2)
        .spacing([10.0, 7.0])
        .show(ui, |ui| {
            ui.label("Name");
            ui.add(egui::TextEdit::singleline(&mut d.name).desired_width(340.0));
            ui.end_row();
            ui.label("Sequence 5′→3′");
            ui.add(
                egui::TextEdit::multiline(&mut d.sequence)
                    .font(FontId::monospace(13.0))
                    .desired_rows(3)
                    .desired_width(340.0)
                    .hint_text("ATGAAACCC…"),
            );
            ui.end_row();
            ui.label("Description");
            ui.add(egui::TextEdit::singleline(&mut d.description).desired_width(340.0));
            ui.end_row();
            ui.label("Color");
            ui.horizontal(|ui| {
                egui::widgets::color_picker::color_edit_button_srgb(ui, &mut d.color);
                ui.checkbox(&mut d.phosphorylated, "5′ phosphorylated");
            });
            ui.end_row();
        });
    ui.add_space(6.0);
    ui.separator();
    heading(ui, "PROPERTIES & BINDING");
    let cleaned = clean_sequence(&d.sequence);
    let params = HybridizationParams::default();
    if cleaned.is_empty() {
        ui.label(
            RichText::new("Enter a sequence to see Tm and binding sites.")
                .color(theme::FAINT)
                .size(12.0),
        );
    } else {
        let p = Primer::new(0, d.name.clone(), &String::from_utf8_lossy(&cleaned));
        let tm = melting_temperature(&cleaned, &params);
        ui.horizontal(|ui| {
            ui.label(RichText::new(format!("{} nt", cleaned.len())).monospace());
            ui.label(RichText::new(format!("GC {:.1}%", p.gc_percent())).monospace());
            match tm {
                Some(t) => {
                    let col = if t < params.min_tm {
                        theme::WARN
                    } else {
                        theme::OK
                    };
                    ui.label(
                        RichText::new(format!("Tm {t:.1} °C"))
                            .monospace()
                            .strong()
                            .color(col),
                    );
                }
                None => {
                    ui.label(RichText::new("Tm —").monospace().color(theme::FAINT));
                }
            }
            ui.label(
                RichText::new(format!(
                    "({} mM Na⁺, {} nM)",
                    params.salt_mm, params.primer_nm
                ))
                .size(11.0)
                .color(theme::FAINT),
            );
        });
        let sites = binding_sites(&p, &app.doc.sequence, &params);
        if sites.is_empty() {
            ui.label(
                RichText::new(format!(
                    "⚠ No binding sites (≥{} bp 3′ match, Tm ≥ {} °C)",
                    params.min_3prime_match, params.min_tm
                ))
                .color(theme::WARN)
                .size(12.0),
            );
        } else {
            ui.label(
                RichText::new(format!("{} binding site(s):", sites.len()))
                    .size(12.0)
                    .color(theme::MUTED),
            );
            egui::ScrollArea::vertical()
                .max_height(90.0)
                .show(ui, |ui| {
                    for s in sites.iter().take(12) {
                        ui.label(
                            RichText::new(format!(
                                "{}..{} · {} strand · {} bp matched · Tm {:.1} °C",
                                s.start + 1,
                                s.start + s.len,
                                if s.strand == domain::Strand::Top {
                                    "top"
                                } else {
                                    "bottom"
                                },
                                s.matched_3prime,
                                s.tm
                            ))
                            .monospace()
                            .size(11.5)
                            .color(theme::TEXT),
                        );
                    }
                });
        }
    }
    error_label(ui, &d.error);
    ui.add_space(6.0);
    ui.horizontal(|ui| {
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui
                .add(egui::Button::new(RichText::new("Save").strong()))
                .clicked()
            {
                match crate::actions::save_primer_draft(app, d) {
                    Ok(()) => *keep = false,
                    Err(e) => d.error = Some(e),
                }
            }
            if ui.button("Cancel").clicked() {
                *keep = false;
            }
        });
    });
}

// ---------------------------------------------------------------------------
// Genetic code table
// ---------------------------------------------------------------------------

fn genetic_codes_ui(app: &mut AppState, ui: &mut egui::Ui, selected: &mut u8, keep: &mut bool) {
    ui.horizontal(|ui| {
        ui.label("Genetic code:");
        let code = domain::genetic_code::code_by_id(*selected);
        egui::ComboBox::from_id_salt("gc_table_sel")
            .selected_text(format!("{} — {}", code.id, code.name))
            .width(360.0)
            .show_ui(ui, |ui| {
                for c in domain::genetic_code::CODES {
                    ui.selectable_value(selected, c.id, format!("{} — {}", c.id, c.name));
                }
            });
    });
    ui.add_space(6.0);
    let code = domain::genetic_code::code_by_id(*selected);
    let bases = *b"TCAG";
    egui::Grid::new("codon_grid")
        .num_columns(5)
        .spacing([6.0, 4.0])
        .striped(true)
        .show(ui, |ui| {
            ui.label("");
            for b2 in bases {
                ui.label(
                    RichText::new((b2 as char).to_string())
                        .monospace()
                        .strong()
                        .color(theme::ACCENT),
                );
            }
            ui.end_row();
            for (i, &b1) in bases.iter().enumerate() {
                ui.label(
                    RichText::new((b1 as char).to_string())
                        .monospace()
                        .strong()
                        .color(theme::ACCENT),
                );
                for (j, &b2) in bases.iter().enumerate() {
                    let mut cell = String::new();
                    for (k, &b3) in bases.iter().enumerate() {
                        let idx = i * 16 + j * 4 + k;
                        let aa = code.table.as_bytes()[idx];
                        let codon = format!("{}{}{}", b1 as char, b2 as char, b3 as char);
                        let is_start = code.starts.contains(&codon.as_str());
                        let marker = if aa == b'*' {
                            "■"
                        } else if is_start {
                            "▶"
                        } else {
                            " "
                        };
                        cell.push_str(&format!("{codon} {aa} {marker}\n"));
                    }
                    let text = cell.trim_end();
                    let has_stop = text.contains('*');
                    let col = if has_stop { theme::MUTED } else { theme::TEXT };
                    ui.add(
                        egui::Label::new(RichText::new(text).monospace().size(10.5).color(col))
                            .selectable(false),
                    );
                }
                ui.end_row();
            }
        });
    ui.add_space(4.0);
    ui.horizontal(|ui| {
        ui.label(
            RichText::new("▶ start codon   ■ stop")
                .size(11.0)
                .color(theme::FAINT),
        );
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.button("Close").clicked() {
                *keep = false;
            }
            let is_default = app.doc.default_genetic_code == *selected;
            if ui
                .add_enabled(!is_default, egui::Button::new("Use as document default"))
                .clicked()
            {
                let id = *selected;
                let name = domain::genetic_code::code_by_id(id).name;
                app.mutate(&format!("Set genetic code to {name}"), Vec::new(), |doc| {
                    doc.default_genetic_code = id;
                });
                app.toast(format!("Default genetic code: {name}"));
            }
        });
    });
}

// ---------------------------------------------------------------------------
// Letter codes & search tips
// ---------------------------------------------------------------------------

fn letter_codes_ui(ui: &mut egui::Ui) {
    heading(ui, "DNA LETTER CODES");
    let rows = [
        ("A", "Adenine"),
        ("C", "Cytosine"),
        ("G", "Guanine"),
        ("T", "Thymine"),
        ("R", "A or G (purine)"),
        ("Y", "C or T (pyrimidine)"),
        ("S", "G or C (strong)"),
        ("W", "A or T (weak)"),
        ("K", "G or T (keto)"),
        ("M", "A or C (amino)"),
        ("B", "C, G or T (not A)"),
        ("D", "A, G or T (not C)"),
        ("H", "A, C or T (not G)"),
        ("V", "A, C or G (not T)"),
        ("N", "Any base"),
    ];
    egui::Grid::new("dna_codes")
        .num_columns(2)
        .spacing([14.0, 3.0])
        .show(ui, |ui| {
            for (code, meaning) in rows {
                ui.label(
                    RichText::new(code)
                        .monospace()
                        .strong()
                        .color(theme::ACCENT),
                );
                ui.label(RichText::new(meaning).size(12.0).color(theme::TEXT));
                ui.end_row();
            }
        });
    ui.add_space(8.0);
    heading(ui, "PROTEIN LETTER CODES");
    let rows = [
        ("B", "D or N (Asx)"),
        ("Z", "E or Q (Glx)"),
        ("X", "Any amino acid"),
        ("*", "Stop codon"),
    ];
    egui::Grid::new("prot_codes")
        .num_columns(2)
        .spacing([14.0, 3.0])
        .show(ui, |ui| {
            for (code, meaning) in rows {
                ui.label(
                    RichText::new(code)
                        .monospace()
                        .strong()
                        .color(theme::ACCENT),
                );
                ui.label(RichText::new(meaning).size(12.0).color(theme::TEXT));
                ui.end_row();
            }
        });
    ui.add_space(6.0);
    ui.add(egui::Label::new(RichText::new("The 20 standard amino-acid letters (A, C, D, E, F, G, H, I, K, L, M, N, P, Q, R, S, T, V, W, Y) match themselves.").size(11.5).color(theme::MUTED)).wrap());
}

fn search_tips_ui(ui: &mut egui::Ui) {
    egui::ScrollArea::vertical().max_height(420.0).show(ui, |ui| {
        heading(ui, "DNA SEARCHES");
        for (q, desc) in [
            ("GAATTC", "Find an exact sequence on both strands (and across the origin of circular DNA)."),
            ("GGTCTRCTTC", "R allows A or G at that position — finds GGTCTACTTC and GGTCTGCTTC."),
            ("GGTCT(AG)CTTC", "Parentheses allow an explicit set of bases at one position."),
            ("AAA!R", "! excludes bases: AAA not followed by A or G."),
            ("AAA!(AG)", "Exclude an explicit set with !(…)."),
            ("\"R\"", "Quoted letters match only bases literally represented as R."),
        ] {
            ui.horizontal(|ui| {
                ui.label(RichText::new(q).monospace().strong().color(theme::TEAL));
            });
            ui.add(egui::Label::new(RichText::new(desc).size(12.0).color(theme::MUTED)).wrap());
            ui.add_space(4.0);
        }
        heading(ui, "PROTEIN SEARCHES");
        for (q, desc) in [
            ("MKWG", "Searches all six translation frames of visible sequence."),
            ("MXWG", "X allows any amino acid at that position."),
            ("!P", "Exclude an amino acid; !(ST) excludes a set."),
            ("\"X\"", "Quoted X matches only residues literally represented as X. B matches D/N, Z matches E/Q."),
        ] {
            ui.label(RichText::new(q).monospace().strong().color(theme::TEAL));
            ui.add(egui::Label::new(RichText::new(desc).size(12.0).color(theme::MUTED)).wrap());
            ui.add_space(4.0);
        }
        heading(ui, "ENZYME / FEATURE / PRIMER SEARCHES");
        ui.add(egui::Label::new(RichText::new("Queries are not case sensitive. Use * as a wildcard: \"*promoter\" or \"M13*\".").size(12.0).color(theme::MUTED)).wrap());
    });
}

fn about_ui(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.label(RichText::new("🧬").size(34.0));
        ui.vertical(|ui| {
            ui.label(
                RichText::new("HelixBench")
                    .size(22.0)
                    .strong()
                    .color(theme::ACCENT),
            );
            ui.label(
                RichText::new("Local-first DNA workbench · version 0.1.0")
                    .size(12.0)
                    .color(theme::MUTED),
            );
        });
    });
    ui.add_space(8.0);
    ui.add(egui::Label::new(RichText::new("A native Windows desktop app for plasmid and sequence analysis: restriction mapping, feature annotation, primer design, six-frame translation and history-tracked editing. All data stays on this machine.").size(12.5).color(theme::TEXT)).wrap());
    ui.add_space(8.0);
    heading(ui, "KEYBOARD SHORTCUTS");
    egui::Grid::new("about_keys")
        .num_columns(2)
        .spacing([16.0, 3.0])
        .show(ui, |ui| {
            for (k, v) in [
                ("Ctrl+O / Ctrl+S", "Open / Save"),
                ("Ctrl+Z / Ctrl+Y", "Undo / Redo"),
                ("Ctrl+F", "Find"),
                (
                    "Ctrl+A / Ctrl+C / Ctrl+V / Ctrl+X",
                    "Select all / Copy / Paste / Cut",
                ),
                ("Delete", "Delete selection"),
                ("Ctrl+1…6", "Switch views"),
                ("Esc", "Close dialog / find bar"),
            ] {
                ui.label(RichText::new(k).monospace().size(11.5).color(theme::TEAL));
                ui.label(RichText::new(v).size(11.5).color(theme::MUTED));
                ui.end_row();
            }
        });
    ui.add_space(8.0);
    ui.label(
        RichText::new("Built with Rust + egui · feature set derived from the SnapGene User Guide.")
            .size(11.0)
            .color(theme::FAINT),
    );
}

// ---------------------------------------------------------------------------
// Options
// ---------------------------------------------------------------------------

fn options_ui(app: &mut AppState, ui: &mut egui::Ui) {
    heading(ui, "SHOW IN MAP & SEQUENCE");
    egui::Grid::new("options_grid")
        .num_columns(2)
        .spacing([20.0, 5.0])
        .show(ui, |ui| {
            ui.checkbox(&mut app.prefs.show_features, "Features");
            ui.checkbox(&mut app.prefs.show_enzymes, "Enzymes");
            ui.end_row();
            ui.checkbox(&mut app.prefs.show_primers, "Primers");
            ui.checkbox(&mut app.prefs.show_orfs, "ORFs");
            ui.end_row();
            ui.checkbox(&mut app.prefs.show_translations, "Translations");
            ui.checkbox(&mut app.prefs.show_complement, "Complement strand");
            ui.end_row();
            ui.checkbox(&mut app.prefs.show_colors, "Base colors");
            ui.checkbox(&mut app.prefs.show_zoom_controls, "Zoom controls");
            ui.end_row();
        });
    ui.add_space(6.0);
    ui.horizontal(|ui| {
        ui.label("Show colors:");
        let mut mode = app.colors_mode;
        egui::ComboBox::from_id_salt("colors_mode")
            .selected_text(mode.label())
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut mode, ColorsMode::None, "No colors");
                ui.selectable_value(&mut mode, ColorsMode::History, "History colors");
                ui.selectable_value(&mut mode, ColorsMode::Custom, "Custom DNA colors");
            });
        app.colors_mode = mode;
    });
    ui.add_space(6.0);
    ui.separator();
    heading(ui, "ORFS & TRANSLATION");
    ui.horizontal(|ui| {
        ui.label("Minimum ORF length");
        let mut v = app.prefs.orf_min_aa as f32;
        if ui
            .add(egui::Slider::new(&mut v, 10.0..=300.0).text("aa"))
            .changed()
        {
            app.prefs.orf_min_aa = v as usize;
            app.analysis_dirty = true;
        }
    });
    ui.horizontal(|ui| {
        ui.label("Default genetic code:");
        let current = app.doc.default_genetic_code;
        let code = domain::genetic_code::code_by_id(current);
        let mut new_id = current;
        egui::ComboBox::from_id_salt("opt_code")
            .selected_text(format!("{} — {}", code.id, code.name))
            .width(300.0)
            .show_ui(ui, |ui| {
                for c in domain::genetic_code::CODES {
                    ui.selectable_value(&mut new_id, c.id, format!("{} — {}", c.id, c.name));
                }
            });
        if new_id != current {
            let name = domain::genetic_code::code_by_id(new_id).name;
            app.mutate(&format!("Set genetic code to {name}"), Vec::new(), |doc| {
                doc.default_genetic_code = new_id
            });
        }
    });
    ui.add_space(6.0);
    ui.separator();
    heading(ui, "TOPOLOGY");
    ui.horizontal(|ui| {
        let topo = app.doc.sequence.topology;
        let mut new = topo;
        ui.selectable_value(&mut new, Topology::Linear, "Linear");
        ui.selectable_value(&mut new, Topology::Circular, "Circular");
        if new != topo {
            crate::actions::set_topology(app, new);
        }
        if app.doc.sequence.is_circular() && ui.button("Rotate origin…").clicked() {
            app.dialog = Some(Dialog::Rotate {
                position: "1".into(),
                error: None,
            });
        }
    });
}

fn color_selection_ui(app: &mut AppState, ui: &mut egui::Ui, d: &mut ColorDraft, keep: &mut bool) {
    let end = d.range.start + d.range.len;
    ui.label(
        RichText::new(format!(
            "Color bases {}..{} ({} bp).",
            d.range.start + 1,
            end,
            d.range.len
        ))
        .size(12.5)
        .color(theme::MUTED),
    );
    ui.add_space(8.0);
    egui::Grid::new("color_sel_grid")
        .num_columns(2)
        .spacing([12.0, 8.0])
        .show(ui, |ui| {
            ui.label("Color");
            egui::widgets::color_picker::color_edit_button_srgb(ui, &mut d.color);
            ui.end_row();
            ui.label("Strands");
            ui.horizontal(|ui| {
                ui.selectable_value(&mut d.strands, StrandMask(StrandMask::TOP), "Top");
                ui.selectable_value(&mut d.strands, StrandMask(StrandMask::BOTTOM), "Bottom");
                ui.selectable_value(&mut d.strands, StrandMask(StrandMask::BOTH), "Both");
            });
            ui.end_row();
        });
    ui.add_space(6.0);
    ui.label(
        RichText::new("Shown in the Custom DNA colors mode.")
            .size(11.5)
            .color(theme::FAINT),
    );
    ui.add_space(8.0);
    ui.horizontal(|ui| {
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui
                .add(egui::Button::new(RichText::new("Apply").strong()))
                .clicked()
            {
                crate::actions::apply_color_selection(app, *d);
                *keep = false;
            }
            if ui.button("Cancel").clicked() {
                *keep = false;
            }
        });
    });
}

// ---------------------------------------------------------------------------
// Rotate / unsaved / detect common
// ---------------------------------------------------------------------------

fn rotate_ui(
    app: &mut AppState,
    ui: &mut egui::Ui,
    position: &mut String,
    error: &mut Option<String>,
    keep: &mut bool,
) {
    let l = app.doc.sequence.len();
    ui.add(egui::Label::new(RichText::new(format!("Move the origin (position 1) of this {l} bp circular sequence. All annotations rotate with it.")).size(12.5).color(theme::MUTED)).wrap());
    ui.add_space(6.0);
    ui.horizontal(|ui| {
        ui.label("New position 1 at:");
        ui.add(
            egui::TextEdit::singleline(position)
                .desired_width(100.0)
                .font(FontId::monospace(13.0)),
        );
        ui.label(
            RichText::new(format!("(1..{l})"))
                .color(theme::FAINT)
                .size(12.0),
        );
    });
    error_label(ui, error);
    ui.add_space(6.0);
    ui.horizontal(|ui| {
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui
                .add(egui::Button::new(RichText::new("Rotate").strong()))
                .clicked()
            {
                match position.trim().parse::<usize>() {
                    Ok(p) if p >= 1 && p <= l => {
                        crate::actions::rotate_to(app, p - 1);
                        *keep = false;
                    }
                    _ => *error = Some(format!("Enter a position between 1 and {l}")),
                }
            }
            if ui.button("Cancel").clicked() {
                *keep = false;
            }
        });
    });
}

fn unsaved_ui(app: &mut AppState, ui: &mut egui::Ui, action: PendingAction, keep: &mut bool) {
    ui.label(
        RichText::new(format!("\"{}\" has unsaved changes.", app.doc.name))
            .size(13.5)
            .color(theme::TEXT),
    );
    ui.label(
        RichText::new("Save before continuing?")
            .size(12.5)
            .color(theme::MUTED),
    );
    ui.add_space(10.0);
    ui.horizontal(|ui| {
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.button("Cancel").clicked() {
                *keep = false;
            }
            if ui
                .add(egui::Button::new(
                    RichText::new("Discard").color(theme::DANGER),
                ))
                .clicked()
            {
                *keep = false;
                crate::actions::perform(app, action.clone());
            }
            if ui
                .add(egui::Button::new(RichText::new("Save").strong()))
                .clicked()
                && crate::actions::save(app)
            {
                *keep = false;
                crate::actions::perform(app, action);
            }
        });
    });
}

fn detect_common_ui(
    app: &mut AppState,
    ui: &mut egui::Ui,
    detected: &[domain::common_features::DetectedFeature],
    checked: &mut [bool],
    keep: &mut bool,
) {
    let n_new = detected
        .iter()
        .zip(checked.iter())
        .filter(|(d, &c)| c && !domain::common_features::is_duplicate(&app.doc.features, d))
        .count();
    ui.label(
        RichText::new(format!(
            "{} known element(s) found · {} new",
            detected.len(),
            n_new
        ))
        .size(12.5)
        .color(theme::MUTED),
    );
    ui.add_space(4.0);
    ui.horizontal(|ui| {
        if ui.button("Select all new").clicked() {
            for (d, c) in detected.iter().zip(checked.iter_mut()) {
                *c = !domain::common_features::is_duplicate(&app.doc.features, d);
            }
        }
        if ui.button("Select none").clicked() {
            for c in checked.iter_mut() {
                *c = false;
            }
        }
    });
    ui.add_space(4.0);
    egui::ScrollArea::vertical()
        .max_height(320.0)
        .show(ui, |ui| {
            egui::Grid::new("detect_grid")
                .num_columns(4)
                .spacing([12.0, 4.0])
                .striped(true)
                .show(ui, |ui| {
                    for (i, d) in detected.iter().enumerate() {
                        let t = &domain::common_features::COMMON_FEATURES[d.template];
                        let dup = domain::common_features::is_duplicate(&app.doc.features, d);
                        ui.add_enabled(!dup, egui::Checkbox::new(&mut checked[i], ""));
                        ui.label(RichText::new(t.name).color(if dup {
                            theme::FAINT
                        } else {
                            theme::TEXT
                        }));
                        ui.label(
                            RichText::new(format!(
                                "{}..{}",
                                d.range.start + 1,
                                d.range.start + d.range.len
                            ))
                            .monospace()
                            .size(11.5)
                            .color(theme::MUTED),
                        );
                        ui.label(
                            RichText::new(if dup {
                                "already present"
                            } else {
                                if d.strand == domain::Strand::Top {
                                    "top"
                                } else {
                                    "bottom"
                                }
                            })
                            .size(11.0)
                            .color(if dup {
                                theme::FAINT
                            } else {
                                theme::TEAL
                            }),
                        );
                        ui.end_row();
                    }
                });
        });
    ui.add_space(6.0);
    ui.horizontal(|ui| {
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui
                .add_enabled(
                    n_new > 0,
                    egui::Button::new(RichText::new(format!("Add {n_new} Features")).strong()),
                )
                .clicked()
            {
                crate::actions::apply_detected(app, detected, checked);
                *keep = false;
            }
            if ui.button("Cancel").clicked() {
                *keep = false;
            }
        });
    });
}

/// Unused helper kept for palette consistency in future dialogs.
#[allow(dead_code)]
fn accent_stroke() -> Stroke {
    Stroke::new(1.0, theme::ACCENT)
}

/// Range helper used by editor previews.
#[allow(dead_code)]
fn preview_range(start: usize, len: usize) -> Range {
    Range::new(start, len)
}
