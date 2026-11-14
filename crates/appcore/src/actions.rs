//! Typed application actions: mutations, clipboard, file IO.

use std::path::PathBuf;

use domain::document::StrandMask;
use domain::feature::{Direction, Feature};
use domain::primer::Primer;
use domain::sequence::{clean_sequence, reverse_complement};
use domain::{ColorRegion, Document, Range, Topology};

use crate::state::{
    AlignmentDraft, AppState, AssemblyDraft, BlastDraft, CodonDraft, ColorDraft, ColorsMode,
    CombDraft, CrisprDraft, Dialog, FeatureDraft, NewSeqState, OpenDoc, PcrDraft, PendingAction,
    PrimerDesignDraft, PrimerDraft, RegistryDraft, RegistryEditDraft,
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

// --- In-silico PCR / primer design / virtual digest -------------------------

pub fn open_pcr(app: &mut AppState) {
    app.dialog = Some(Dialog::Pcr(PcrDraft {
        copy_features: true,
        ..Default::default()
    }));
}

fn primer_seq(app: &AppState, text: &str) -> Vec<u8> {
    let t = text.trim();
    if let Some(p) = app
        .doc
        .primers
        .iter()
        .find(|p| p.name.eq_ignore_ascii_case(t))
    {
        return p.sequence.clone();
    }
    clean_sequence(t)
}

pub fn pcr_preview(app: &AppState, draft: &PcrDraft) -> Option<domain::pcr::PcrProduct> {
    let f = primer_seq(app, &draft.fwd);
    let r = primer_seq(app, &draft.rev);
    if f.is_empty() || r.is_empty() {
        return None;
    }
    let fp = Primer::new(0, "F".into(), &String::from_utf8_lossy(&f));
    let rp = Primer::new(0, "R".into(), &String::from_utf8_lossy(&r));
    domain::pcr::simulate(
        &app.doc.sequence,
        &fp,
        &rp,
        &domain::primer::HybridizationParams::default(),
    )
}

fn reset_doc_view(app: &mut AppState) {
    app.history.clear();
    app.future.clear();
    app.selection = None;
    app.selected_feature = None;
    app.selected_primer = None;
    app.zoom_start = 0;
    app.zoom_width = app.doc.sequence.len().max(1);
    app.analysis_dirty = true;
}

pub fn create_pcr_product(app: &mut AppState, draft: &PcrDraft) -> Result<(), String> {
    let Some(prod) = pcr_preview(app, draft) else {
        return Err("No amplicon: primers must bind in opposite orientations".into());
    };
    let l = app.doc.sequence.len();
    let name = format!(
        "PCR_{}",
        sanitize_filename(&format!("{}-{}", draft.fwd.trim(), draft.rev.trim()))
    );
    let mut doc = Document::new(name, prod.bases.clone(), Topology::Linear);
    if draft.copy_features {
        for f in &app.doc.features {
            if !f.visible {
                continue;
            }
            let span = f.span(l);
            let rel = (span.start + l - prod.start) % l;
            let relend = (span.end() + l - prod.start) % l;
            if rel < relend && relend <= prod.len {
                let mut nf = f.clone();
                nf.id = 0;
                nf.segments = vec![Range::new(rel, relend - rel)];
                doc.features.push(nf);
            }
        }
    }
    let cur = OpenDoc {
        doc: std::mem::replace(&mut app.doc, doc),
        path: app.path.take(),
        dirty: app.dirty,
    };
    app.library.push(cur);
    app.dirty = true;
    app.saved_depth = None;
    reset_doc_view(app);
    app.toast(format!("PCR product created ({} bp)", prod.len));
    Ok(())
}

pub fn switch_doc(app: &mut AppState, idx: usize) {
    if idx >= app.library.len() {
        return;
    }
    let od = app.library.remove(idx);
    let cur = OpenDoc {
        doc: std::mem::replace(&mut app.doc, od.doc),
        path: app.path.take(),
        dirty: app.dirty,
    };
    app.library.push(cur);
    app.path = od.path;
    app.dirty = od.dirty;
    app.saved_depth = if od.dirty { None } else { Some(0) };
    reset_doc_view(app);
}

pub fn open_primer_design(app: &mut AppState) {
    let (start, len) = app
        .selection
        .map(|s| (s.start, s.len))
        .unwrap_or((0, app.doc.sequence.len().min(500)));
    app.dialog = Some(Dialog::PrimerDesign(PrimerDesignDraft {
        start: (start + 1).to_string(),
        len: len.to_string(),
        target_tm: "60".into(),
        overhang: String::new(),
        fwd: Vec::new(),
        rev: Vec::new(),
        fwd_pick: 0,
        rev_pick: 0,
        error: None,
    }));
}

pub fn run_primer_design(app: &mut AppState, draft: &mut PrimerDesignDraft) {
    let start = draft
        .start
        .trim()
        .parse::<usize>()
        .unwrap_or(1)
        .saturating_sub(1);
    let len = draft.len.trim().parse::<usize>().unwrap_or(0);
    let tm = draft.target_tm.trim().parse::<f32>().unwrap_or(60.0);
    if len == 0 {
        draft.error = Some("Enter a target length".into());
        return;
    }
    let p = domain::primer_design::DesignParams {
        target_tm: tm,
        overhang: draft.overhang.clone(),
        ..Default::default()
    };
    let (f, r) = domain::primer_design::design(&app.doc.sequence, start, len, &p);
    draft.error = if f.is_empty() || r.is_empty() {
        Some("No primers meet the Tm/GC window; adjust target Tm".into())
    } else {
        None
    };
    draft.fwd = f;
    draft.rev = r;
    draft.fwd_pick = 0;
    draft.rev_pick = 0;
}

pub fn add_designed_primers(app: &mut AppState, draft: &PrimerDesignDraft) -> Result<(), String> {
    let f = draft
        .fwd
        .get(draft.fwd_pick)
        .ok_or("Pick a forward primer")?;
    let r = draft
        .rev
        .get(draft.rev_pick)
        .ok_or("Pick a reverse primer")?;
    let fs = String::from_utf8_lossy(&f.sequence).into_owned();
    let rs = String::from_utf8_lossy(&r.sequence).into_owned();
    app.mutate("Add designed primer pair", Vec::new(), |doc| {
        let mut fp = Primer::new(0, format!("Design-F{}", doc.primers.len() + 1), &fs);
        fp.color = [0x79, 0xc0, 0xff];
        let mut rp = Primer::new(0, format!("Design-R{}", doc.primers.len() + 2), &rs);
        rp.color = [0xf8, 0x51, 0x49];
        doc.add_primer(fp);
        doc.add_primer(rp);
    });
    app.dialog = None;
    app.toast("Designed primer pair added");
    Ok(())
}

pub fn open_digest(app: &mut AppState) {
    app.dialog = Some(Dialog::Digest);
}

pub fn digest_fragments(app: &AppState) -> Vec<domain::digest::Fragment> {
    let mut cuts = Vec::new();
    for (_e, sites) in &app.analysis.chosen {
        for s in sites {
            cuts.push(s.position);
        }
    }
    domain::digest::fragments(
        &cuts,
        app.doc.sequence.len(),
        app.doc.sequence.is_circular(),
    )
}

// --- Alignment + version history -------------------------------------------

pub fn open_alignment(app: &mut AppState) {
    app.dialog = Some(Dialog::Alignment(AlignmentDraft {
        reference: if app.library.is_empty() { 0 } else { 1 },
        reindex: true,
        result: None,
        error: None,
    }));
}

pub fn run_alignment(app: &mut AppState, draft: &mut AlignmentDraft) {
    let mut seqs = vec![app.doc.sequence.clone()];
    for od in &app.library {
        seqs.push(od.doc.sequence.clone());
    }
    if seqs.len() < 2 {
        draft.error = Some("Open a second document (e.g. a PCR product) to align against.".into());
        draft.result = None;
        return;
    }
    let ref_idx = draft.reference.min(seqs.len() - 1);
    draft.result = Some(domain::align::align_many(&seqs, ref_idx, draft.reindex));
    draft.error = None;
}

fn unix_now() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

pub fn save_version(app: &mut AppState, label: &str) {
    let label = if label.trim().is_empty() {
        format!("Version {}", app.doc.versions.len() + 1)
    } else {
        label.trim().to_string()
    };
    app.doc.save_version(label.clone(), unix_now());
    app.dirty = true;
    app.toast(format!("Saved version \"{label}\""));
}

pub fn restore_version(app: &mut AppState, idx: usize) {
    if app.doc.restore_version(idx) {
        app.history.clear();
        app.future.clear();
        app.selection = None;
        app.selected_feature = None;
        app.zoom_start = 0;
        app.zoom_width = app.doc.sequence.len().max(1);
        app.analysis_dirty = true;
        app.dirty = true;
        app.toast("Version restored");
    }
}

// --- P1: assembly / CRISPR / codon optimization -----------------------------

fn push_product(app: &mut AppState, doc: Document) {
    let cur = OpenDoc {
        doc: std::mem::replace(&mut app.doc, doc),
        path: app.path.take(),
        dirty: app.dirty,
    };
    app.library.push(cur);
    app.dirty = true;
    app.saved_depth = None;
    app.history.clear();
    app.future.clear();
    app.selection = None;
    app.selected_feature = None;
    app.zoom_start = 0;
    app.zoom_width = app.doc.sequence.len().max(1);
    app.analysis_dirty = true;
}

pub fn open_assembly(app: &mut AppState) {
    app.dialog = Some(Dialog::Assembly(AssemblyDraft {
        strategy: 0,
        cut: "1".into(),
        inserts: vec![true; app.library.len()],
        error: None,
    }));
}

pub fn create_assembly(app: &mut AppState, draft: &AssemblyDraft) -> Result<(), String> {
    let strategy = match draft.strategy {
        1 => domain::assembly::Strategy::Gibson,
        2 => domain::assembly::Strategy::GoldenGate,
        _ => domain::assembly::Strategy::DigestLigate,
    };
    let cut = draft
        .cut
        .trim()
        .parse::<usize>()
        .unwrap_or(1)
        .saturating_sub(1);
    let inserts: Vec<Vec<u8>> = app
        .library
        .iter()
        .enumerate()
        .filter(|(i, _)| draft.inserts.get(*i).copied().unwrap_or(false))
        .map(|(_, od)| od.doc.sequence.bases.clone())
        .collect();
    if inserts.is_empty() {
        return Err("Select at least one insert document.".into());
    }
    let asm = domain::assembly::assemble(&app.doc.sequence, &inserts, cut, strategy);
    let name = format!("ASM_{}", sanitize_filename(&app.doc.name));
    let doc = Document::new(name, asm.bases, Topology::Circular);
    push_product(app, doc);
    app.toast(format!(
        "{} assembly created ({} junctions)",
        strategy.label(),
        asm.junctions.len()
    ));
    Ok(())
}

pub fn open_crispr(app: &mut AppState) {
    app.dialog = Some(Dialog::Crispr(CrisprDraft {
        limit: "20".into(),
        guides: Vec::new(),
        error: None,
    }));
}

pub fn run_crispr(app: &mut AppState, draft: &mut CrisprDraft) {
    let limit = draft
        .limit
        .trim()
        .parse::<usize>()
        .unwrap_or(20)
        .clamp(1, 100);
    draft.guides = domain::crispr::scan(&app.doc.sequence, limit);
    draft.error = if draft.guides.is_empty() {
        Some("No NGG PAM sites found.".into())
    } else {
        None
    };
}

pub fn add_crispr_features(app: &mut AppState, draft: &CrisprDraft) {
    let guides = draft.guides.clone();
    app.mutate("Add CRISPR guide features", Vec::new(), |doc| {
        for g in &guides {
            let mut f = Feature::new(
                0,
                format!("gRNA {}", g.position + 1),
                "crispr".into(),
                vec![Range::new(g.position, g.sequence.len())],
            );
            f.direction = if g.strand == domain::Strand::Top {
                Direction::Forward
            } else {
                Direction::Reverse
            };
            f.color = [0x8b, 0x5c, 0xf6];
            doc.add_feature(f);
        }
    });
    app.toast(format!("Added {} guide features", guides.len()));
}

pub fn open_codon(app: &mut AppState) {
    app.dialog = Some(Dialog::Codon(CodonDraft {
        host: 0,
        result: None,
        cai_before: 0.0,
        cai_after: 0.0,
        error: None,
    }));
}

fn host_of(idx: usize) -> domain::codon::Host {
    if idx == 1 {
        domain::codon::Host::Human
    } else {
        domain::codon::Host::EColi
    }
}

pub fn run_codon(app: &mut AppState, draft: &mut CodonDraft) {
    let host = host_of(draft.host);
    let dna = &app.doc.sequence.bases;
    if dna.len() < 3 {
        draft.error = Some("Sequence too short to optimize.".into());
        draft.result = None;
        return;
    }
    let opt = domain::codon::optimize(dna, host);
    draft.cai_before = domain::codon::cai(dna, host);
    draft.cai_after = domain::codon::cai(&opt, host);
    draft.result = Some(opt);
    draft.error = None;
}

pub fn apply_codon(app: &mut AppState, draft: &CodonDraft) -> Result<(), String> {
    let Some(opt) = &draft.result else {
        return Err("Run optimization first.".into());
    };
    let host = host_of(draft.host);
    let name = format!(
        "CODON_{}_{}",
        host.label().replace(['.', ' '], ""),
        sanitize_filename(&app.doc.name)
    );
    let doc = Document::new(name, opt.clone(), app.doc.sequence.topology);
    push_product(app, doc);
    app.toast(format!(
        "Codon-optimized product created (CAI {:.2})",
        draft.cai_after
    ));
    Ok(())
}

// --- P2: local BLAST + combinatorial assembly -------------------------------

pub fn open_blast(app: &mut AppState) {
    app.dialog = Some(Dialog::Blast(BlastDraft {
        k: "11".into(),
        max: "20".into(),
        hits: Vec::new(),
        error: None,
    }));
}

pub fn run_blast(app: &mut AppState, draft: &mut BlastDraft) {
    let k = draft.k.trim().parse::<usize>().unwrap_or(11);
    let max = draft
        .max
        .trim()
        .parse::<usize>()
        .unwrap_or(20)
        .clamp(1, 100);
    let db: Vec<(String, Vec<u8>)> = app
        .library
        .iter()
        .map(|od| (od.doc.name.clone(), od.doc.sequence.bases.clone()))
        .collect();
    if db.is_empty() {
        draft.error =
            Some("Open a second document (or FASTA) to serve as the BLAST database.".into());
        draft.hits = Vec::new();
        return;
    }
    draft.hits = domain::blast::search(&app.doc.sequence.bases, &db, k, max);
    draft.error = if draft.hits.is_empty() {
        Some("No significant hits.".into())
    } else {
        None
    };
}

pub fn open_combinatorial(app: &mut AppState) {
    app.dialog = Some(Dialog::Combinatorial(CombDraft {
        strategy: 0,
        cut: "1".into(),
        cap: "24".into(),
        results: Vec::new(),
        labels: Vec::new(),
        error: None,
    }));
}

fn permutations(items: &[usize], cap: usize) -> Vec<Vec<usize>> {
    let mut out = Vec::new();
    let mut a: Vec<usize> = items.to_vec();
    let n = a.len();
    if n == 0 {
        return out;
    }
    let mut c = vec![0usize; n];
    out.push(a.clone());
    let mut i = 0;
    while i < n && out.len() < cap {
        if c[i] < i {
            if i % 2 == 0 {
                a.swap(0, i);
            } else {
                a.swap(c[i], i);
            }
            out.push(a.clone());
            c[i] += 1;
            i = 0;
        } else {
            c[i] = 0;
            i += 1;
        }
    }
    out.truncate(cap);
    out
}

pub fn run_combinatorial(app: &mut AppState, draft: &mut CombDraft) {
    let strategy = match draft.strategy {
        1 => domain::assembly::Strategy::Gibson,
        2 => domain::assembly::Strategy::GoldenGate,
        _ => domain::assembly::Strategy::DigestLigate,
    };
    let cut = draft
        .cut
        .trim()
        .parse::<usize>()
        .unwrap_or(1)
        .saturating_sub(1);
    let cap = draft
        .cap
        .trim()
        .parse::<usize>()
        .unwrap_or(24)
        .clamp(1, 120);
    let parts: Vec<(String, Vec<u8>)> = app
        .library
        .iter()
        .map(|od| (od.doc.name.clone(), od.doc.sequence.bases.clone()))
        .collect();
    if parts.len() < 2 {
        draft.error = Some("Open at least two insert documents to combine.".into());
        draft.results = Vec::new();
        draft.labels = Vec::new();
        return;
    }
    let idx: Vec<usize> = (0..parts.len()).collect();
    let perms = permutations(&idx, cap);
    let mut results = Vec::new();
    let mut labels = Vec::new();
    for p in perms {
        let inserts: Vec<Vec<u8>> = p.iter().map(|&i| parts[i].1.clone()).collect();
        let label = p
            .iter()
            .map(|&i| parts[i].0.clone())
            .collect::<Vec<_>>()
            .join(" + ");
        results.push(domain::assembly::assemble(
            &app.doc.sequence,
            &inserts,
            cut,
            strategy,
        ));
        labels.push(label);
    }
    draft.results = results;
    draft.labels = labels;
    draft.error = None;
}

pub fn create_combinatorial(
    app: &mut AppState,
    draft: &CombDraft,
    idx: usize,
) -> Result<(), String> {
    let Some(asm) = draft.results.get(idx) else {
        return Err("Generate combinations first.".into());
    };
    let name = format!("COMB{}_{}", idx + 1, sanitize_filename(&app.doc.name));
    let doc = Document::new(name, asm.bases.clone(), Topology::Circular);
    push_product(app, doc);
    app.toast(format!("Combinatorial construct {} created", idx + 1));
    Ok(())
}

// --- Local registry ---------------------------------------------------------

pub fn open_registry(app: &mut AppState) {
    app.dialog = Some(Dialog::Registry(RegistryDraft::default()));
}

pub fn open_registry_editor(app: &mut AppState, kind: usize, id: Option<u64>) {
    let k = domain::registry::Kind::ALL[kind.min(2)];
    let values = match id.and_then(|i| app.registry.get(i)) {
        Some(e) => k
            .fields()
            .iter()
            .map(|f| e.values.get(*f).cloned().unwrap_or_default())
            .collect(),
        None => vec![String::new(); k.fields().len()],
    };
    app.dialog = Some(Dialog::RegistryEditor(RegistryEditDraft {
        id,
        kind,
        values,
        error: None,
    }));
}

pub fn save_registry_entry(app: &mut AppState, draft: &RegistryEditDraft) -> Result<(), String> {
    let k = domain::registry::Kind::ALL[draft.kind.min(2)];
    let fields = k.fields();
    let mut values = std::collections::BTreeMap::new();
    for (i, f) in fields.iter().enumerate() {
        values.insert(
            (*f).to_string(),
            draft.values.get(i).cloned().unwrap_or_default(),
        );
    }
    if values
        .get("Name")
        .map(|s| s.trim().is_empty())
        .unwrap_or(true)
    {
        return Err("Name is required.".into());
    }
    let now = unix_now();
    match draft.id {
        Some(id) => {
            app.registry.update(id, values, now);
        }
        None => {
            app.registry.add(k, values, now);
        }
    }
    let _ = storage::registry::save(&app.registry);
    app.toast("Registry entry saved");
    Ok(())
}

pub fn delete_registry_entry(app: &mut AppState, id: u64) {
    if app.registry.delete(id) {
        let _ = storage::registry::save(&app.registry);
        app.toast("Registry entry deleted");
    }
}

pub fn link_registry_entry(app: &mut AppState, id: u64) {
    let current = app.doc.name.clone();
    let already = app.registry.get(id).and_then(|e| e.linked_doc.clone()) == Some(current.clone());
    let target = if already { None } else { Some(current) };
    app.registry.link(id, target.clone(), unix_now());
    let _ = storage::registry::save(&app.registry);
    app.toast(match target {
        Some(d) => format!("Linked to {d}"),
        None => "Unlinked".into(),
    });
}

pub fn open_linked_entry(app: &mut AppState, id: u64) {
    let Some(docname) = app.registry.get(id).and_then(|e| e.linked_doc.clone()) else {
        app.toast("Entry has no linked document");
        return;
    };
    if app.doc.name == docname {
        return;
    }
    if let Some(i) = app.library.iter().position(|od| od.doc.name == docname) {
        switch_doc(app, i);
    } else {
        app.toast(format!("Linked document \"{docname}\" is not open"));
    }
}
