//! Typed application actions: mutations, clipboard, file IO.

use std::path::PathBuf;

use domain::document::StrandMask;
use domain::feature::{Direction, Feature};
use domain::primer::Primer;
use domain::sequence::{clean_sequence, reverse_complement};
use domain::{ColorRegion, Document, Range, Topology};

use crate::state::{
    AppState, ColorDraft, ColorsMode, Dialog, FeatureDraft, NewSeqState, PendingAction, PrimerDraft,
};

pub fn region(range: Range) -> Vec<ColorRegion> {
    vec![ColorRegion {
        range,
        strands: StrandMask(StrandMask::BOTH),
        color: [0xf8, 0x51, 0x49],
    }]
}

// ---------------------------------------------------------------------------
// Features & primers
// ---------------------------------------------------------------------------

pub fn delete_feature(app: &mut AppState, id: u64) {
    let Some(f) = app.doc.feature(id).cloned() else {
        return;
    };
    let span = f.span(app.doc.sequence.len());
    app.mutate(
        &format!("Delete feature \"{}\"", f.name),
        region(span),
        |doc| {
            doc.delete_feature(id);
        },
    );
    app.selected_feature = None;
    app.toast(format!("Deleted feature \"{}\" — Ctrl+Z to undo", f.name));
}

pub fn delete_primer(app: &mut AppState, id: u64) {
    let Some(p) = app.doc.primer(id).cloned() else {
        return;
    };
    app.mutate(
        &format!("Delete primer \"{}\"", p.name),
        Vec::new(),
        |doc| {
            doc.delete_primer(id);
        },
    );
    app.selected_primer = None;
    app.toast(format!("Deleted primer \"{}\" — Ctrl+Z to undo", p.name));
}

pub fn parse_segments(text: &str, l: usize, circular: bool) -> Result<Vec<Range>, String> {
    let mut out = Vec::new();
    for part in text.split([',', ';']) {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }
        let (a, b) = if let Some((a, b)) = part.split_once("..") {
            (a.trim(), b.trim())
        } else if let Some((a, b)) = part.split_once('-') {
            (a.trim(), b.trim())
        } else {
            (part, part)
        };
        let a: usize = a.parse().map_err(|_| format!("Invalid position \"{a}\""))?;
        let b: usize = b.parse().map_err(|_| format!("Invalid position \"{b}\""))?;
        if a < 1 || b < a {
            return Err(format!(
                "Invalid range \"{part}\" (positions are 1-based, start ≤ end)"
            ));
        }
        let len = b - a + 1;
        if !circular && b > l {
            return Err(format!(
                "Position {b} is past the end of the {l} bp sequence"
            ));
        }
        if circular && len > l {
            return Err(format!("Range \"{part}\" is longer than the sequence"));
        }
        out.push(Range::new(a - 1, len));
    }
    if out.is_empty() {
        return Err("Enter at least one range, e.g. 100..200".into());
    }
    out.sort_by_key(|r| r.start);
    Ok(out)
}

pub fn save_feature_draft(app: &mut AppState, draft: &FeatureDraft) -> Result<(), String> {
    let l = app.doc.sequence.len();
    if draft.name.trim().is_empty() {
        return Err("Give the feature a name".into());
    }
    let segments = parse_segments(&draft.segments_text, l, app.doc.sequence.is_circular())?;
    let mut f = Feature::new(
        0,
        draft.name.trim().to_string(),
        draft.kind.clone(),
        segments.clone(),
    );
    f.direction = draft.direction;
    f.color = draft.color;
    f.note = draft.note.clone();
    f.product = draft.product.clone();
    f.translated = draft.translated;
    f.genetic_code = draft.genetic_code;
    f.first_codon_met = draft.first_codon_met;
    f.frame_offset = draft.frame_offset;
    let span = f.span(l);
    let label = if draft.id.is_some() {
        format!("Edit feature \"{}\"", f.name)
    } else {
        format!("Add feature \"{}\"", f.name)
    };
    let id = draft.id;
    app.mutate(&label, region(span), |doc| {
        if let Some(id) = id {
            if let Some(existing) = doc.feature_mut(id) {
                f.id = id;
                f.visible = existing.visible;
                f.cleavage_arrows = existing.cleavage_arrows.clone();
                *existing = f;
            }
        } else {
            doc.add_feature(f);
        }
    });
    Ok(())
}

pub fn save_primer_draft(app: &mut AppState, draft: &PrimerDraft) -> Result<(), String> {
    let raw = draft.sequence.trim();
    if raw.is_empty() {
        return Err("Enter the primer sequence (5′→3′)".into());
    }
    let upper = raw.to_ascii_uppercase();
    if upper.bytes().any(|b| domain::iupac::dna_mask(b) == 0) {
        return Err("Primer contains invalid DNA characters".into());
    }
    let seq = clean_sequence(raw);
    if seq.len() < 6 {
        return Err("Primer must be at least 6 bases".into());
    }
    let name = if draft.name.trim().is_empty() {
        format!("Primer {}", app.doc.primers.len() + 1)
    } else {
        draft.name.trim().to_string()
    };
    let mut p = Primer::new(0, name.clone(), &String::from_utf8_lossy(&seq));
    p.description = draft.description.clone();
    p.color = draft.color;
    p.phosphorylated = draft.phosphorylated;
    let sites = domain::primer::binding_sites(
        &p,
        &app.doc.sequence,
        &domain::primer::HybridizationParams::default(),
    );
    let regions: Vec<ColorRegion> = sites
        .iter()
        .take(4)
        .map(|s| ColorRegion {
            range: Range::new(s.start, s.len),
            strands: StrandMask(StrandMask::BOTH),
            color: [0xf8, 0x51, 0x49],
        })
        .collect();
    let label = if draft.id.is_some() {
        format!("Edit primer \"{name}\"")
    } else {
        format!("Add primer \"{name}\"")
    };
    let id = draft.id;
    app.mutate(&label, regions, |doc| {
        if let Some(id) = id {
            if let Some(existing) = doc.primer_mut(id) {
                p.id = id;
                p.visible = existing.visible;
                *existing = p;
            }
        } else {
            doc.add_primer(p);
        }
    });
    Ok(())
}

pub fn primer_from_selection(app: &mut AppState) {
    let Some(sel) = app.selection else {
        app.toast("Select bases in Map or Sequence view first");
        return;
    };
    let bases = app.doc.sequence.region(sel);
    let draft = PrimerDraft {
        id: None,
        name: format!("Primer {}", app.doc.primers.len() + 1),
        sequence: String::from_utf8_lossy(&bases).into_owned(),
        description: format!("From selection {}..{}", sel.start + 1, sel.start + sel.len),
        color: [0x79, 0xc0, 0xff],
        phosphorylated: false,
        error: None,
    };
    app.dialog = Some(Dialog::PrimerEditor(draft));
}

pub fn apply_detected(
    app: &mut AppState,
    detected: &[domain::common_features::DetectedFeature],
    checked: &[bool],
) {
    let mut to_add = Vec::new();
    for (d, &c) in detected.iter().zip(checked) {
        if c && !domain::common_features::is_duplicate(&app.doc.features, d) {
            to_add.push(d.clone());
        }
    }
    if to_add.is_empty() {
        app.toast("No new common features to add");
        return;
    }
    let n = to_add.len();
    let regions: Vec<ColorRegion> = to_add
        .iter()
        .map(|d| ColorRegion {
            range: d.range,
            strands: StrandMask(StrandMask::BOTH),
            color: [0xf8, 0x51, 0x49],
        })
        .collect();
    app.mutate(
        &format!("Detect common features ({n} added)"),
        regions,
        |doc| {
            for d in to_add {
                let f = d.to_feature(0);
                doc.add_feature(f);
            }
        },
    );
    app.toast(format!("Added {n} common features"));
}

// ---------------------------------------------------------------------------
// Sequence editing
// ---------------------------------------------------------------------------

pub fn delete_selection(app: &mut AppState) {
    let Some(sel) = app.selection else { return };
    if sel.len == 0 {
        return;
    }
    let n = sel.len;
    app.mutate(&format!("Delete {n} bp"), region(sel), |doc| {
        doc.delete_range(sel)
    });
    app.select(None);
}

pub fn copy_selection(app: &AppState) -> String {
    match app.selection {
        Some(sel) => String::from_utf8_lossy(&app.doc.sequence.region(sel)).into_owned(),
        None => String::from_utf8_lossy(&app.doc.sequence.bases).into_owned(),
    }
}

pub fn cut_selection(app: &mut AppState) -> Option<String> {
    if app.selection.is_some() {
        let text = copy_selection(app);
        delete_selection(app);
        Some(text)
    } else {
        None
    }
}

pub fn insert_bases(app: &mut AppState, text: &str, replace_selection: bool) {
    let bases = clean_sequence(text);
    if bases.is_empty() {
        return;
    }
    let l = app.doc.sequence.len();
    let n = bases.len();
    if let Some(sel) = app.selection.filter(|_| replace_selection && l > 0) {
        let new_range = Range::new(sel.start, n);
        app.mutate(
            &format!("Replace {} bp with {n} bp", sel.len),
            region(new_range),
            |doc| {
                doc.replace_range(sel, &bases);
            },
        );
        app.select(Some(new_range));
    } else {
        let pos = app.selection.map(|s| s.start).unwrap_or(l);
        app.mutate(
            &format!("Insert {n} bp"),
            region(Range::new(pos, n)),
            |doc| {
                doc.insert_bases(pos, &bases);
            },
        );
        app.select(Some(Range::new(pos + n, 0)));
        app.selection = None;
        app.scroll_to = Some(pos);
    }
}

pub fn select_all(app: &mut AppState) {
    let l = app.doc.sequence.len();
    if l > 0 {
        app.select(Some(Range::new(0, l)));
    }
}

pub fn reverse_complement_sequence(app: &mut AppState) {
    let l = app.doc.sequence.len();
    if l == 0 {
        return;
    }
    let sel = app.selection;
    app.mutate("Reverse complement sequence", Vec::new(), |doc| {
        doc.sequence.bases = reverse_complement(&doc.sequence.bases);
        let mirror = |r: Range| Range::new((l - r.start - r.len) % l, r.len);
        for f in &mut doc.features {
            f.segments = f.segments.iter().map(|&r| mirror(r)).collect();
            f.segments.reverse();
            f.direction = match f.direction {
                Direction::Forward => Direction::Reverse,
                Direction::Reverse => Direction::Forward,
                d => d,
            };
            f.cleavage_arrows = f.cleavage_arrows.iter().map(|&p| l - 1 - p).collect();
        }
        for c in &mut doc.color_regions {
            c.range = mirror(c.range);
        }
    });
    if let Some(s) = sel {
        app.select(Some(Range::new((l - s.start - s.len) % l, s.len)));
    }
    app.toast("Sequence reverse complemented");
}

pub fn rotate_to(app: &mut AppState, pos: usize) {
    let l = app.doc.sequence.len();
    if l == 0 || !app.doc.sequence.is_circular() {
        return;
    }
    let p = pos % l;
    if p == 0 {
        return;
    }
    app.mutate(&format!("Rotate origin to {}", p + 1), Vec::new(), |doc| {
        doc.rotate(p)
    });
    app.zoom_start = 0;
    app.toast(format!("Origin moved to position {}", p + 1));
}

pub fn set_topology(app: &mut AppState, topology: Topology) {
    if app.doc.sequence.topology == topology {
        return;
    }
    let l = app.doc.sequence.len();
    app.mutate(
        &format!("Set topology to {}", topology.label()),
        Vec::new(),
        |doc| {
            doc.set_topology(topology);
            if topology == Topology::Linear {
                for f in &mut doc.features {
                    let mut split = Vec::new();
                    for seg in &f.segments {
                        if seg.end() > l {
                            split.push(Range::new(seg.start, l - seg.start));
                            split.push(Range::new(0, seg.end() - l));
                        } else {
                            split.push(*seg);
                        }
                    }
                    f.segments = split;
                }
                let mut split = Vec::new();
                for c in &doc.color_regions {
                    if c.range.end() > l {
                        split.push(ColorRegion {
                            range: Range::new(c.range.start, l - c.range.start),
                            strands: c.strands,
                            color: c.color,
                        });
                        split.push(ColorRegion {
                            range: Range::new(0, c.range.end() - l),
                            strands: c.strands,
                            color: c.color,
                        });
                    } else {
                        split.push(c.clone());
                    }
                }
                doc.color_regions = split;
            }
        },
    );
}

// ---------------------------------------------------------------------------
// Custom DNA colors
// ---------------------------------------------------------------------------

pub fn open_color_selection(app: &mut AppState) {
    let Some(sel) = app.selection else {
        app.toast("Select a range in the sequence first");
        return;
    };
    app.dialog = Some(Dialog::ColorSelection(ColorDraft {
        range: sel,
        color: [0x2d, 0xd4, 0xbf],
        strands: StrandMask(StrandMask::BOTH),
    }));
}

pub fn apply_color_selection(app: &mut AppState, draft: ColorDraft) {
    let ColorDraft {
        range,
        color,
        strands,
    } = draft;
    app.mutate("Add custom color", region(range), |doc| {
        doc.color_regions.push(ColorRegion {
            range,
            strands,
            color,
        });
    });
    app.colors_mode = ColorsMode::Custom;
    app.toast("Custom color added");
}

pub fn clear_custom_colors(app: &mut AppState) {
    let n = app.doc.color_regions.len();
    if n == 0 {
        app.toast("No custom colors to clear");
        return;
    }
    app.mutate("Clear custom colors", Vec::new(), |doc| {
        doc.color_regions.clear();
    });
    app.toast(format!("Cleared {n} custom color region(s)"));
}

// ---------------------------------------------------------------------------
// File operations
// ---------------------------------------------------------------------------

pub fn new_demo(app: &mut AppState) {
    app.set_document(domain::demo::build_demo(), None);
    app.view = crate::state::View::Map;
    app.toast("Opened demo plasmid pHelixBench-Demo");
}

pub fn open_file_dialog(app: &mut AppState) {
    if let Some(path) = crate::fs_dialogs::pick_open(app.config.last_directory.as_ref()) {
        open_path(app, &path);
    }
}

pub fn open_path(app: &mut AppState, path: &std::path::Path) {
    match storage::load_any(path) {
        Ok(doc) => {
            let name = doc.name.clone();
            let len = doc.sequence.len();
            app.set_document(doc, Some(path.to_path_buf()));
            app.view = crate::state::View::Map;
            app.toast(format!("Opened {name} ({len} bp)"));
        }
        Err(e) => app.toast(format!("Could not open {}: {e}", path.display())),
    }
}

pub fn save(app: &mut AppState) -> bool {
    match app.path.clone() {
        Some(path) => save_to(app, &path),
        None => save_as(app),
    }
}

pub fn save_as(app: &mut AppState) -> bool {
    let name = format!("{}.hbx", sanitize_filename(&app.doc.name));
    match crate::fs_dialogs::pick_save(
        "Save Sequence As",
        "HelixBench document",
        &["hbx"],
        &name,
        app.config.last_directory.as_ref(),
    ) {
        Some(path) => save_to(app, &path),
        None => false,
    }
}

fn save_to(app: &mut AppState, path: &std::path::Path) -> bool {
    match storage::save_document(&app.doc, path) {
        Ok(()) => {
            app.path = Some(path.to_path_buf());
            app.dirty = false;
            app.saved_depth = Some(app.history.len());
            app.config.add_recent(path);
            let _ = storage::config::save(&app.config);
            app.toast(format!("Saved {}", path.display()));
            true
        }
        Err(e) => {
            app.toast(format!("Save failed: {e}"));
            false
        }
    }
}

pub fn export_fasta(app: &mut AppState) {
    export_with(app, "fasta", "FASTA", storage::fasta::export);
}

pub fn export_genbank(app: &mut AppState) {
    export_with(app, "gb", "GenBank", storage::genbank::export);
}

pub fn export_plain(app: &mut AppState) {
    export_with(app, "txt", "Plain text", storage::export_plain);
}

fn export_with(app: &mut AppState, ext: &str, label: &str, f: impl FnOnce(&Document) -> String) {
    let name = format!("{}.{ext}", sanitize_filename(&app.doc.name));
    if let Some(path) = crate::fs_dialogs::pick_save(
        &format!("Export as {label}"),
        label,
        &[ext],
        &name,
        app.config.last_directory.as_ref(),
    ) {
        let text = f(&app.doc);
        match storage::atomic_write(&path, text.as_bytes()) {
            Ok(()) => app.toast(format!("Exported {label} to {}", path.display())),
            Err(e) => app.toast(format!("Export failed: {e}")),
        }
    }
}

fn sanitize_filename(name: &str) -> String {
    let s: String = name
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '.' {
                c
            } else {
                '_'
            }
        })
        .collect();
    if s.is_empty() {
        "sequence".into()
    } else {
        s
    }
}

// ---------------------------------------------------------------------------
// Unsaved-changes guard
// ---------------------------------------------------------------------------

pub fn guard(app: &mut AppState, action: PendingAction) {
    if app.dirty {
        app.dialog = Some(Dialog::Unsaved { action });
    } else {
        perform(app, action);
    }
}

pub fn perform(app: &mut AppState, action: PendingAction) {
    match action {
        PendingAction::CloseApp => {
            persist_config(app);
            app.exit_requested = true;
        }
        PendingAction::NewDemo => new_demo(app),
        PendingAction::NewEmpty => app.dialog = Some(Dialog::NewSequence(NewSeqState::default())),
        PendingAction::Open => open_file_dialog(app),
        PendingAction::OpenRecent(path) => recent_open(app, path),
    }
}

pub fn persist_config(app: &mut AppState) {
    app.config.ui = app.prefs.clone();
    let _ = storage::config::save(&app.config);
}

pub fn create_from_dialog(app: &mut AppState, state: &NewSeqState) -> Result<(), String> {
    let bases = clean_sequence(&state.text);
    if bases.is_empty() {
        return Err("Paste or type a DNA sequence".into());
    }
    if bases.len() > 5_000_000 {
        return Err("Sequence is too large (max 5 Mb)".into());
    }
    let name = if state.name.trim().is_empty() {
        "Untitled".to_string()
    } else {
        state.name.trim().to_string()
    };
    let doc = Document::new(name, bases, state.topology);
    app.set_document(doc, None);
    app.view = crate::state::View::Map;
    app.toast(format!(
        "Created {} ({} bp)",
        app.doc.name,
        app.doc.sequence.len()
    ));
    Ok(())
}

pub fn recent_open(app: &mut AppState, path: PathBuf) {
    if path.exists() {
        open_path(app, &path);
    } else {
        app.config.remove_recent(&path);
        app.toast(format!("File no longer exists: {}", path.display()));
    }
}
