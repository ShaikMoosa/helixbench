//! Typed application actions: mutations, clipboard, file IO.

use std::path::PathBuf;

use domain::document::StrandMask;
use domain::feature::{Direction, Feature};
use domain::primer::Primer;
use domain::sequence::{clean_sequence, reverse_complement};
use domain::{ColorRegion, Document, Range, Topology};

use crate::state::{
    AlignmentDraft, AppState, AssemblyDraft, AuditDraft, BiologicsDraft, BlastDraft, BpRecipeDraft,
    BpReplicateDraft, ChemAddDraft, ChemLotDraft, ChemReactionDraft, CodonDraft, ColorDraft,
    ColorsMode, CombDraft, CrisprDraft, Dialog, EntityMapDraft, EntryEditDraft, FeatureDraft,
    FeatureLibDraft, HomeDraft, InventoryDraft, IvAnimalDraft, IvMeasureDraft, IvStudyDraft,
    NewSeqState, NotebookDraft, OpenDoc, PcrDraft, PendingAction, PrimerDesignDraft, PrimerDraft,
    ProjectsDraft, ProteinDraft, RegistryDraft, RegistryEditDraft, RegistryImportDraft, RnaDraft,
    WorkflowDraft, WorkspaceDraft,
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
    if app.selected_feature == Some(id) {
        app.selected_feature = None;
        app.selected_primer = None;
        app.select(None);
    }
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
    domain::annotations::parse_segments(text, l, circular)
}

pub fn save_feature_draft(app: &mut AppState, draft: &FeatureDraft) -> Result<(), String> {
    if draft.source != app.doc.source_ref() {
        return Err(
            "The source sequence changed; reopen this feature to review the current fields".into(),
        );
    }
    if let Some(id) = draft.id {
        if app.doc.feature(id).is_none() {
            return Err("The feature was deleted; this draft cannot be saved".into());
        }
    }
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
    domain::annotations::validate_feature(&app.doc, &f)?;
    if let Some(id) = draft.id {
        f.id = id;
        if let Some(old) = app.doc.feature(id) {
            f.visible = old.visible;
            f.cleavage_arrows = old.cleavage_arrows.clone();
            if *old == f {
                return Ok(());
            }
        }
    }
    let span = f.span(l);
    let label = if draft.id.is_some() {
        format!("Edit feature \"{}\"", f.name)
    } else {
        format!("Add feature \"{}\"", f.name)
    };
    let mut selected = draft.id;
    app.mutate(&label, region(span), |doc| {
        if let Some(id) = draft.id {
            *doc.feature_mut(id).expect("validated feature identity") = f;
        } else {
            selected = Some(doc.add_feature(f));
        }
    });
    if let Some(id) = selected {
        app.select_feature(id);
    }
    app.toast("Feature updated — Ctrl+Z to undo");
    Ok(())
}

pub fn save_primer_draft(app: &mut AppState, draft: &PrimerDraft) -> Result<(), String> {
    if draft.source != app.doc.source_ref() {
        return Err("The source sequence changed; reopen this primer before saving".into());
    }
    if draft.id.is_some_and(|id| app.doc.primer(id).is_none()) {
        return Err("The primer was deleted; this draft cannot replace it".into());
    }
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
        source: app.doc.source_ref(),
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

// ---------------------------------------------------------------------------
// Sequence editing
// ---------------------------------------------------------------------------

pub fn delete_selection(app: &mut AppState) {
    if app.workspace_search.open {
        app.toast("Return to the sequence before editing DNA");
        return;
    }
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
    if app.workspace_search.open {
        app.toast("Return to the sequence before editing DNA");
        return None;
    }
    if app.selection.is_some() {
        let text = copy_selection(app);
        delete_selection(app);
        Some(text)
    } else {
        None
    }
}

pub fn insert_bases(app: &mut AppState, text: &str, replace_selection: bool) {
    if app.workspace_search.open {
        app.toast("Return to the sequence before editing DNA");
        return;
    }
    let bases = match storage::imports::strict_dna(text) {
        Ok(bases) => bases,
        Err(error) => {
            app.toast(format!("Input needs review: {error}"));
            return;
        }
    };
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
    if let Err(error) = crate::first_use::try_sample(app) {
        app.toast(error);
    }
}

pub fn open_file_dialog(app: &mut AppState) {
    if let Some(path) = crate::fs_dialogs::pick_open(app.config.last_directory.as_ref()) {
        open_path(app, &path);
    }
}

pub fn open_path(app: &mut AppState, path: &std::path::Path) {
    if let Err(error) = crate::navigation::request_navigation(
        app,
        crate::navigation::NavigationIntent::OpenFile(path.to_owned()),
    ) {
        app.toast(error);
    }
}

pub(crate) fn install_opened_document(app: &mut AppState, doc: Document, path: &std::path::Path) {
    let name = doc.name.clone();
    let len = doc.sequence.len();
    let native = path
        .extension()
        .and_then(|ext| ext.to_str())
        .is_some_and(|ext| ext.eq_ignore_ascii_case(storage::NATIVE_EXT));
    app.set_document(doc, native.then(|| path.to_path_buf()));
    if !native {
        crate::navigation::mark_imported(app);
        app.config.add_recent(path);
    }
    if native {
        crate::first_use::remember(
            app,
            storage::config::RecentTarget::Document {
                document: app.doc.id,
                path: Some(path.to_path_buf()),
            },
            app.doc.name.clone(),
        );
    }
    app.view = crate::state::View::Map;
    app.toast(if native {
        format!("Opened {name} ({len} bp)")
    } else {
        format!("Imported {name} ({len} bp) — Save As a HelixBench document")
    });
}

pub fn save(app: &mut AppState) -> bool {
    if crate::first_use::is_sample(app) {
        app.operation_error = Some(
            "Use Make personal copy in the sample banner before saving your own document".into(),
        );
        return false;
    }
    match app.path.clone() {
        Some(path) => save_to(app, &path),
        None => save_as(app),
    }
}

pub fn save_as(app: &mut AppState) -> bool {
    if crate::first_use::is_sample(app) {
        app.operation_error = Some(
            "Use Make personal copy in the sample banner before saving your own document".into(),
        );
        return false;
    }
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
    let journal = app.operation_journal();
    let config_path = app.profile_path(storage::config::config_path());
    crate::operations::save_to(app, path, &journal, &config_path)
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
        let journal = app.operation_journal();
        crate::operations::export_to(app, &path, text, label, &journal);
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
    if let Err(error) = crate::navigation::request_navigation(app, action.into()) {
        app.toast(error);
    }
}

pub fn perform(app: &mut AppState, action: PendingAction) {
    guard(app, action);
}

pub fn persist_config(app: &mut AppState) -> bool {
    app.config.ui = app.prefs.clone();
    match storage::config::save_to(
        &app.config,
        &app.profile_path(storage::config::config_path()),
    ) {
        Ok(()) => {
            app.exit_settings_failed = false;
            true
        }
        Err(error) => {
            app.operation_error = Some(format!("Could not save settings: {error}. Retry Quit after fixing the destination, or exit without saving settings."));
            false
        }
    }
}

pub fn create_from_dialog(app: &mut AppState, state: &NewSeqState) -> Result<(), String> {
    let bases = storage::imports::strict_dna(&state.text).map_err(|e| e.to_string())?;
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
    crate::navigation::mark_imported(app);
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
    let mut draft = PcrDraft {
        source: Some(app.doc.source_ref()),
        copy_features: true,
        ..Default::default()
    };
    if let Some(id) = app.selected_primer {
        if let Some(pair) = app
            .doc
            .primer_pairs
            .iter()
            .rev()
            .find(|p| p.forward.id == Some(id) || p.reverse.id == Some(id))
            .cloned()
        {
            for (forward, input) in [(true, pair.forward), (false, pair.reverse)] {
                if let Some(id) = input.id {
                    if app
                        .doc
                        .primer(id)
                        .is_some_and(|p| p.sequence == input.sequence)
                    {
                        let _ = crate::primer_workflow::choose_primer(app, &mut draft, forward, id);
                    } else {
                        draft.error =
                            Some("The saved pair changed; choose its primers again".into());
                    }
                }
            }
        }
    }
    app.dialog = Some(Dialog::Pcr(draft));
}
pub fn pcr_preview(app: &AppState, draft: &PcrDraft) -> Option<domain::pcr::PcrProduct> {
    crate::primer_workflow::validate_pcr(app, draft).ok()?;
    draft
        .result
        .as_ref()?
        .products
        .get(draft.product_pick?)
        .cloned()
}
pub fn create_pcr_product(app: &mut AppState, draft: &PcrDraft) -> Result<(), String> {
    crate::primer_workflow::create_product(app, draft)
}
pub fn switch_doc(app: &mut AppState, idx: usize) {
    if let Some(id) = app.library.get(idx).map(|doc| doc.doc.id) {
        switch_document(app, id);
    }
}

pub fn switch_document(app: &mut AppState, id: domain::DocumentId) {
    if let Err(error) = crate::navigation::request_navigation(
        app,
        crate::navigation::NavigationIntent::SwitchDocument(id),
    ) {
        app.toast(error);
    }
}

pub fn open_primer_design(app: &mut AppState) {
    let (start, len) = app
        .selection
        .map(|s| (s.start, s.len))
        .unwrap_or((0, app.doc.sequence.len().min(500)));
    app.dialog = Some(Dialog::PrimerDesign(PrimerDesignDraft {
        source: Some(app.doc.source_ref()),
        anchor: app.selection,
        anchor_strand: app.selection_strand,
        capture: None,
        start: (start + 1).to_string(),
        len: len.to_string(),
        target_tm: "60".into(),
        overhang: String::new(),
        fwd: Vec::new(),
        rev: Vec::new(),
        fwd_pick: None,
        rev_pick: None,
        error: None,
    }));
}
pub fn run_primer_design(app: &mut AppState, draft: &mut PrimerDesignDraft) {
    if let Err(error) = crate::primer_workflow::run_design(app, draft) {
        draft.error = Some(error);
    }
}
pub fn add_designed_primers(app: &mut AppState, draft: &PrimerDesignDraft) -> Result<(), String> {
    crate::primer_workflow::add_pair(app, draft)?;
    app.dialog = None;
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
        labels: Vec::new(),
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
    // Auto-reindex circular sequences against the reference before aligning.
    let mut oriented: Vec<Vec<u8>> = Vec::new();
    for (i, s) in seqs.iter().enumerate() {
        let mut bases = s.bases.clone();
        if draft.reindex && i != ref_idx {
            let off = domain::align::best_rotation(s, &seqs[ref_idx].bases);
            if off > 0 {
                bases.rotate_left(off);
            }
        }
        oriented.push(bases);
    }
    let rows = domain::msa::progressive(&oriented);
    let consensus = domain::msa::consensus(&rows);
    let identity = domain::align::identity(&rows);
    draft.result = Some(domain::align::Alignment {
        rows,
        consensus,
        identity,
    });
    draft.labels = {
        let mut n = vec![app.doc.name.clone()];
        for od in &app.library {
            n.push(od.doc.name.clone());
        }
        n
    };
    draft.error = None;
}

/// Persist the current alignment result as a named, reloadable snapshot.
pub fn alignment_save(app: &mut AppState, name: &str) {
    let result = match &app.dialog {
        Some(Dialog::Alignment(d)) => d.result.clone(),
        _ => None,
    };
    let Some(result) = result else {
        app.toast("Run an alignment first");
        return;
    };
    let mut labels = vec![app.doc.name.clone()];
    for od in &app.library {
        labels.push(od.doc.name.clone());
    }
    let name = if name.trim().is_empty() {
        format!("Alignment {}", app.alignments.items.len() + 1)
    } else {
        name.trim().to_string()
    };
    app.alignments.add(&name, labels, result, unix_now());
    let _ = storage::alignments::save_to(
        &app.alignments,
        &app.profile_path(storage::alignments::path()),
    );
    app.toast(format!("Saved alignment \"{name}\""));
}

/// Reopen a saved alignment in the Alignment dialog.
pub fn alignment_load_saved(app: &mut AppState, id: u64) {
    let Some(saved) = app.alignments.get(id).cloned() else {
        return;
    };
    app.dialog = Some(Dialog::Alignment(AlignmentDraft {
        reference: 0,
        reindex: true,
        result: Some(saved.result),
        labels: saved.labels,
        error: None,
    }));
    app.toast(format!("Loaded alignment \"{}\"", saved.name));
}

pub fn alignment_delete_saved(app: &mut AppState, id: u64) {
    if app.alignments.remove(id) {
        let _ = storage::alignments::save_to(
            &app.alignments,
            &app.profile_path(storage::alignments::path()),
        );
        app.toast("Alignment deleted");
    }
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
    crate::navigation::suspend_session(app);
    let cur = OpenDoc {
        doc: std::mem::replace(&mut app.doc, doc),
        path: app.path.take(),
        dirty: app.dirty,
    };
    app.library.push(cur);
    app.dirty = true;
    app.saved_depth = None;
    app.navigation.saved_document = None;
    app.search = crate::state::SearchState::default();
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
        primers: Vec::new(),
        gg_issues: Vec::new(),
        error: None,
    }));
}

/// Compute Gibson junction primers + Golden Gate overhang validation for the
/// selected inserts (does not create a product).
pub fn assembly_extras(app: &mut AppState, draft: &mut AssemblyDraft) {
    let inserts: Vec<Vec<u8>> = app
        .library
        .iter()
        .enumerate()
        .filter(|(i, _)| draft.inserts.get(*i).copied().unwrap_or(false))
        .map(|(_, od)| od.doc.sequence.bases.clone())
        .collect();
    draft.primers = domain::assembly::gibson_primers(&app.doc.sequence.bases, &inserts, 20);
    // Golden Gate overhangs = first 4 bases of each insert (Type IIS fusion sites).
    let overhangs: Vec<Vec<u8>> = inserts
        .iter()
        .map(|i| i[..4.min(i.len())].to_vec())
        .collect();
    draft.gg_issues = domain::assembly::golden_gate_validate(&overhangs);
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
        off: Vec::new(),
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
    draft.off = if let Some(top) = draft.guides.first() {
        let db: Vec<(String, Vec<u8>)> =
            std::iter::once((app.doc.name.clone(), app.doc.sequence.bases.clone()))
                .chain(
                    app.library
                        .iter()
                        .map(|od| (od.doc.name.clone(), od.doc.sequence.bases.clone())),
                )
                .collect();
        domain::crispr::off_target_sites(&top.sequence, &db, 4, 20)
    } else {
        Vec::new()
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
        gc_min: "0.3".into(),
        gc_max: "0.7".into(),
        avoid: String::new(),
        harmonize: false,
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
    let opts = domain::codon::OptimizeOptions {
        gc_min: draft.gc_min.trim().parse().unwrap_or(0.3),
        gc_max: draft.gc_max.trim().parse().unwrap_or(0.7),
        avoid: draft
            .avoid
            .split(',')
            .map(|s| clean_sequence(s.trim()))
            .filter(|v| !v.is_empty())
            .collect(),
        harmonize: draft.harmonize,
    };
    let opt = domain::codon::optimize_with(dna, host, &opts);
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
        db_count: app.blast_db.len(),
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
    let db: Vec<(String, Vec<u8>)> = if !app.blast_db.is_empty() {
        app.blast_db.clone()
    } else {
        app.library
            .iter()
            .map(|od| (od.doc.name.clone(), od.doc.sequence.bases.clone()))
            .collect()
    };
    draft.db_count = db.len();
    if db.is_empty() {
        draft.error =
            Some("Load a FASTA database or open a second document to search against.".into());
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

/// Load an external FASTA file as the BLAST database.
pub fn load_blast_db(app: &mut AppState) {
    if let Some(path) = crate::fs_dialogs::pick_fasta(app.config.last_directory.as_ref()) {
        match std::fs::read(&path) {
            Ok(bytes) => match storage::fasta::parse_all(&bytes, "db") {
                Ok(docs) => {
                    app.blast_db = docs
                        .into_iter()
                        .map(|d| (d.name, d.sequence.bases))
                        .collect();
                    app.toast(format!(
                        "BLAST database loaded ({} sequences)",
                        app.blast_db.len()
                    ));
                }
                Err(e) => app.toast(format!("FASTA parse failed: {e}")),
            },
            Err(e) => app.toast(format!("Read failed: {e}")),
        }
    }
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
    let _ = storage::registry::save_to(&app.registry, &app.profile_path(storage::registry::path()));
    app.toast("Registry entry saved");
    Ok(())
}

pub fn delete_registry_entry(app: &mut AppState, id: u64) {
    if app.registry.delete(id) {
        let _ =
            storage::registry::save_to(&app.registry, &app.profile_path(storage::registry::path()));
        app.toast("Registry entry deleted");
    }
}

pub fn link_registry_entry(app: &mut AppState, id: u64) {
    let current = app.doc.name.clone();
    let already = app.registry.get(id).and_then(|e| e.linked_doc.clone()) == Some(current.clone());
    let target = if already { None } else { Some(current) };
    app.registry.link(id, target.clone(), unix_now());
    let _ = storage::registry::save_to(&app.registry, &app.profile_path(storage::registry::path()));
    app.toast(match target {
        Some(d) => format!("Linked to {d}"),
        None => "Unlinked".into(),
    });
}

pub fn open_linked_entry(app: &mut AppState, id: u64) {
    let Some(entry) = app.registry.get(id) else {
        app.toast("Registry entry is unavailable");
        return;
    };
    if let Some(target) = entry.linked_document {
        switch_document(app, target);
        return;
    }
    let Some(docname) = entry.linked_doc.clone() else {
        app.toast("Entry has no linked document");
        return;
    };
    let mut matches: Vec<_> = app
        .library
        .iter()
        .filter(|doc| doc.doc.name == docname)
        .map(|doc| doc.doc.id)
        .collect();
    if app.doc.name == docname {
        matches.push(app.doc.id);
    }
    matches.sort();
    matches.dedup();
    match matches.as_slice() {
        [target] => switch_document(app, *target),
        [] => app.toast(format!("Linked document \"{docname}\" is not open")),
        _ => app.toast(format!(
            "Ambiguous legacy link \"{docname}\" — choose and relink the intended document"
        )),
    }
}

// --- Protein analysis -------------------------------------------------------

pub fn open_protein(app: &mut AppState) {
    app.dialog = Some(Dialog::Protein(ProteinDraft::default()));
}

pub fn run_protein(app: &mut AppState, draft: &mut ProteinDraft) {
    let code = domain::genetic_code::code_by_id(app.doc.default_genetic_code);
    let protein = domain::translate::translate(&app.doc.sequence.bases, code);
    draft.stats = Some(domain::protein::analyze(&protein));
    // Align against the first library document's translation, if any.
    draft.align = app.library.first().map(|od| {
        let other = domain::translate::translate(&od.doc.sequence.bases, code);
        domain::protein::align_protein(&protein, &other)
    });
    draft.error = None;
}

// --- Feature library + bulk auto-annotation ---------------------------------

pub fn open_feature_lib(app: &mut AppState) {
    app.dialog = Some(Dialog::FeatureLib(FeatureLibDraft::default()));
}

pub fn add_lib_feature(app: &mut AppState, name: &str, kind: &str, motif: &str) {
    let motif = clean_sequence(motif);
    if name.trim().is_empty() || motif.is_empty() {
        app.toast("Name and motif are required");
        return;
    }
    app.feature_lib.push(domain::feature_lib::LibFeature {
        name: name.trim().to_string(),
        kind: kind.trim().to_string(),
        motif,
        color: [0x5a, 0xa2, 0xf0],
    });
    let _ = storage::feature_lib::save_to(
        &app.feature_lib,
        &app.profile_path(storage::feature_lib::path()),
    );
    app.toast("Library feature added");
}

pub fn delete_lib_feature(app: &mut AppState, idx: usize) {
    if idx < app.feature_lib.len() {
        app.feature_lib.remove(idx);
        let _ = storage::feature_lib::save_to(
            &app.feature_lib,
            &app.profile_path(storage::feature_lib::path()),
        );
    }
}

/// Bulk auto-annotate the active document AND all open library documents.
pub fn bulk_annotate(app: &mut AppState) {
    if app.dialog.is_some() && !matches!(app.dialog, Some(Dialog::FeatureLib(_))) {
        app.toast("Apply or cancel the open form first");
        return;
    }
    let draft = crate::annotation_review::prepare_library(app);
    app.dialog = Some(Dialog::AnnotationReview(draft));
}

// --- Workspace: tags + bulk operations --------------------------------------

pub fn open_workspace(app: &mut AppState) {
    let n = app.library.len() + 1;
    app.dialog = Some(Dialog::Workspace(WorkspaceDraft {
        selected: vec![false; n],
        tag: String::new(),
        error: None,
    }));
}

fn workspace_docs(selected: &[bool]) -> Vec<usize> {
    // index 0 = active doc, 1.. = library
    selected
        .iter()
        .enumerate()
        .filter(|(_, s)| **s)
        .map(|(i, _)| i)
        .collect()
}

pub fn bulk_tag(app: &mut AppState, draft: &WorkspaceDraft) {
    let tag = draft.tag.trim().to_string();
    if tag.is_empty() {
        app.toast("Enter a tag");
        return;
    }
    let idxs = workspace_docs(&draft.selected);
    for i in idxs {
        if i == 0 {
            if !app.doc.tags.contains(&tag) {
                app.mutate("Bulk tag", Vec::new(), |doc| doc.tags.push(tag.clone()));
            }
        } else if let Some(od) = app.library.get_mut(i - 1) {
            if !od.doc.tags.contains(&tag) {
                od.doc.tags.push(tag.clone());
                od.doc.advance_revision();
                od.dirty = true;
            }
        }
    }
    app.toast("Tag applied to selected documents");
}

pub fn bulk_export(app: &mut AppState, draft: &WorkspaceDraft) {
    let idxs = workspace_docs(&draft.selected);
    if idxs.is_empty() {
        app.toast("Select documents to export");
        return;
    }
    if let Some(dir) = crate::fs_dialogs::pick_folder() {
        let mut n = 0;
        for i in idxs {
            let doc = if i == 0 {
                &app.doc
            } else {
                &app.library[i - 1].doc
            };
            let path = dir.join(format!("{}.fasta", sanitize_filename(&doc.name)));
            if std::fs::write(&path, storage::fasta::export(doc)).is_ok() {
                n += 1;
            }
        }
        app.toast(format!("Exported {n} documents to {}", dir.display()));
    }
}

pub fn bulk_delete(app: &mut AppState, draft: &WorkspaceDraft) {
    let idxs = workspace_docs(&draft.selected);
    // delete library docs (skip active index 0)
    let mut lib_idx: Vec<usize> = idxs.into_iter().filter(|i| *i > 0).map(|i| i - 1).collect();
    lib_idx.sort_unstable();
    lib_idx.dedup();
    for i in lib_idx.into_iter().rev() {
        if i < app.library.len() {
            app.library.remove(i);
        }
    }
    app.toast("Selected documents closed");
}

pub fn set_tags(app: &mut AppState, tags: Vec<String>) {
    if app.doc.tags != tags {
        app.mutate("Edit tags", Vec::new(), |doc| doc.tags = tags);
    }
}

// --- Phase 1: connected records (projects / notebook / audit / home) --------

fn persist_records(app: &AppState) {
    let _ = storage::records::save_to(&app.records, &app.profile_path(storage::records::path()));
}

fn audit(app: &mut AppState, object: &str, action: &str, detail: &str) {
    app.records
        .audit
        .append(unix_now(), "local", object, action, detail);
    persist_records(app);
}

pub fn open_projects(app: &mut AppState) {
    app.dialog = Some(Dialog::Projects(ProjectsDraft::default()));
}

pub fn add_project(app: &mut AppState, draft: &ProjectsDraft) {
    if draft.name.trim().is_empty() {
        app.toast("Project name required");
        return;
    }
    let id = app.records.workspace.add(draft.name.trim(), draft.parent);
    audit(app, &format!("project:{id}"), "created", &draft.name);
    app.toast("Project created");
}

pub fn delete_project(app: &mut AppState, id: u64) {
    if app.records.workspace.delete(id) {
        audit(app, &format!("project:{id}"), "deleted", "");
        app.toast("Project deleted (children reparented)");
    }
}

pub fn open_notebook(app: &mut AppState) {
    app.dialog = Some(Dialog::Notebook(NotebookDraft::default()));
}

pub fn open_entry_editor(app: &mut AppState, id: Option<u64>, template: usize) {
    let draft = match id.and_then(|i| app.records.entries.iter().find(|e| e.id == i)) {
        Some(e) => EntryEditDraft {
            id: Some(e.id),
            title: e.title.clone(),
            project: e.project,
            template: 0,
            sections: e
                .sections
                .iter()
                .map(|s| (s.title.clone(), s.body.clone()))
                .collect(),
            tables: e.tables.clone(),
            error: None,
        },
        None => {
            let t = app
                .records
                .templates
                .get(template)
                .cloned()
                .unwrap_or_else(|| domain::notebook::Template {
                    name: "Blank".into(),
                    sections: vec![domain::notebook::Section {
                        title: "Objective".into(),
                        body: String::new(),
                    }],
                    tables: Vec::new(),
                });
            EntryEditDraft {
                id: None,
                title: t.name.clone(),
                project: None,
                template,
                sections: t
                    .sections
                    .iter()
                    .map(|s| (s.title.clone(), s.body.clone()))
                    .collect(),
                tables: t.tables.clone(),
                error: None,
            }
        }
    };
    app.dialog = Some(Dialog::EntryEditor(draft));
}

pub fn save_entry(app: &mut AppState, draft: &EntryEditDraft) -> Result<u64, String> {
    if draft.title.trim().is_empty() {
        return Err("Title required".into());
    }
    let now = unix_now();
    let sections: Vec<domain::notebook::Section> = draft
        .sections
        .iter()
        .map(|(t, b)| domain::notebook::Section {
            title: t.clone(),
            body: b.clone(),
        })
        .collect();
    let id = match draft.id {
        Some(id) => {
            if let Some(e) = app.records.entries.iter_mut().find(|e| e.id == id) {
                if e.locked() {
                    return Err("Entry is locked (approved/archived)".into());
                }
                e.title = draft.title.clone();
                e.project = draft.project;
                e.sections = sections;
                e.tables = draft.tables.clone();
                e.updated = now;
            }
            id
        }
        None => {
            let id = app.records.entries.len() as u64 + 1;
            let mut e = domain::notebook::Entry::new(id, draft.title.clone(), now);
            e.project = draft.project;
            e.sections = sections;
            e.tables = draft.tables.clone();
            app.records.entries.push(e);
            id
        }
    };
    audit(app, &format!("entry:{id}"), "saved", &draft.title);
    persist_records(app);
    Ok(id)
}

pub fn entry_action(app: &mut AppState, id: u64, action: &str) {
    let now = unix_now();
    let Some(e) = app.records.entries.iter_mut().find(|e| e.id == id) else {
        return;
    };
    let ok = match action {
        "submit" => e.submit_review("local", now),
        "approve" => e.approve("local", "", now),
        "reject" => e.reject("local", "", now),
        "request_changes" => e.request_changes("local", "", now),
        "resubmit" => e.resubmit("local", "", now),
        "archive" => {
            e.archive(now);
            true
        }
        _ => false,
    };
    if ok {
        audit(app, &format!("entry:{id}"), action, "");
        persist_records(app);
        app.toast(format!("Entry {action}"));
    }
}

/// Submit a result table: persistence boundary + writeback of rows with a Name
/// column into the registry as plasmid entries.
pub fn submit_entry_table(app: &mut AppState, entry_id: u64, table_idx: usize) {
    let now = unix_now();
    let Some(e) = app.records.entries.iter_mut().find(|e| e.id == entry_id) else {
        return;
    };
    if !e.submit_table(table_idx, now) {
        app.toast("Cannot submit (locked or missing table)");
        return;
    }
    let table = e.tables[table_idx].clone();
    let name_col = table
        .columns
        .iter()
        .position(|c| c.eq_ignore_ascii_case("name"));
    let mut created = 0;
    if let Some(nc) = name_col {
        for row in &table.rows {
            if let Some(name) = row.get(nc) {
                if name.trim().is_empty() {
                    continue;
                }
                let mut values = std::collections::BTreeMap::new();
                for (i, col) in table.columns.iter().enumerate() {
                    values.insert(col.clone(), row.get(i).cloned().unwrap_or_default());
                }
                app.registry
                    .add(domain::registry::Kind::Plasmid, values, now);
                created += 1;
            }
        }
        let _ =
            storage::registry::save_to(&app.registry, &app.profile_path(storage::registry::path()));
    }
    audit(
        app,
        &format!("entry:{entry_id}"),
        "table_submitted",
        &format!("{} rows, {created} registered", table.rows.len()),
    );
    persist_records(app);
    app.toast(format!("Table submitted ({created} registered)"));
}

pub fn open_home(app: &mut AppState) {
    app.dialog = Some(Dialog::Home(HomeDraft {}));
}

pub fn open_audit(app: &mut AppState) {
    app.dialog = Some(Dialog::Audit(AuditDraft::default()));
}

pub fn export_audit(app: &mut AppState) {
    if let Some(path) = crate::fs_dialogs::pick_save(
        "Export audit log",
        "CSV",
        &["csv"],
        "audit.csv",
        app.config.last_directory.as_ref(),
    ) {
        match std::fs::write(&path, app.records.audit.to_csv()) {
            Ok(()) => app.toast(format!("Audit exported to {}", path.display())),
            Err(e) => app.toast(format!("Export failed: {e}")),
        }
    }
}

pub fn open_entity_map(app: &mut AppState) {
    app.dialog = Some(Dialog::EntityMap(EntityMapDraft {}));
}

pub fn link_registry_entries(app: &mut AppState, a: u64, b: u64) {
    if app.registry.add_relation(a, b, unix_now()) {
        let _ =
            storage::registry::save_to(&app.registry, &app.profile_path(storage::registry::path()));
        audit(
            app,
            &format!("registry:{a}"),
            "related",
            &format!("registry:{b}"),
        );
        app.toast("Entities linked");
    }
}

pub fn archive_registry_entry(app: &mut AppState, id: u64, archived: bool) {
    if app.registry.set_archived(id, archived, unix_now()) {
        let _ =
            storage::registry::save_to(&app.registry, &app.profile_path(storage::registry::path()));
        audit(
            app,
            &format!("registry:{id}"),
            if archived { "archived" } else { "unarchived" },
            "",
        );
    }
}

pub fn open_registry_import(app: &mut AppState) {
    app.dialog = Some(Dialog::RegistryImport(RegistryImportDraft::default()));
}

/// Load a CSV file from disk as the registry-import source.
pub fn registry_import_load_csv(app: &mut AppState) {
    let Some(path) = crate::fs_dialogs::pick_csv(app.config.last_directory.as_ref()) else {
        return;
    };
    match std::fs::read_to_string(&path) {
        Ok(text) => {
            if let Some(Dialog::RegistryImport(d)) = &mut app.dialog {
                d.csv = text;
                d.row_errors = Vec::new();
                d.imported = 0;
                d.error = None;
            }
            app.toast(format!("Loaded {}", path.display()));
        }
        Err(e) => app.toast(format!("Read failed: {e}")),
    }
}

/// Parse CSV (header + rows) and register entries of the chosen kind.
/// Returns (imported, errors). Validates required Name column.
pub fn run_registry_import(app: &mut AppState, draft: &mut RegistryImportDraft) {
    let kind = domain::registry::Kind::ALL[draft.kind.min(2)];
    let (rows, row_errors) = match parse_registry_rows(&draft.csv, draft.name_col) {
        Ok(v) => v,
        Err(e) => {
            draft.error = Some(e);
            draft.row_errors = Vec::new();
            return;
        }
    };
    let now = unix_now();
    for values in &rows {
        app.registry.add(kind, values.clone(), now);
    }
    let imported = rows.len();
    let _ = storage::registry::save_to(&app.registry, &app.profile_path(storage::registry::path()));
    audit(
        app,
        "registry",
        "imported",
        &format!("{imported} rows, {} row issues", row_errors.len()),
    );
    draft.imported = imported;
    draft.row_errors = row_errors;
    draft.error = if imported == 0 {
        Some("No valid rows imported".into())
    } else {
        None
    };
    app.toast(format!("Imported {imported} entries"));
}

/// Pure CSV parse for registry import: map a chosen column to Name and collect
/// per-row issues (empty name skipped, short rows padded). No persistence.
pub type ImportParse = (Vec<std::collections::BTreeMap<String, String>>, Vec<String>);

pub fn parse_registry_rows(csv: &str, name_col: Option<usize>) -> Result<ImportParse, String> {
    let mut lines = csv.lines().filter(|l| !l.trim().is_empty());
    let Some(header) = lines.next() else {
        return Err("CSV is empty".into());
    };
    let cols: Vec<String> = split_csv(header)
        .iter()
        .map(|s| s.trim().to_string())
        .collect();
    if cols.is_empty() {
        return Err("CSV header is empty".into());
    }
    let nc = match name_col {
        Some(i) if i < cols.len() => i,
        Some(_) => return Err("Selected Name column is out of range".into()),
        None => cols
            .iter()
            .position(|c| c.eq_ignore_ascii_case("name"))
            .unwrap_or(0),
    };
    let mut rows = Vec::new();
    let mut errs = Vec::new();
    for (ln, line) in lines.enumerate() {
        let row = split_csv(line);
        let name = row.get(nc).cloned().unwrap_or_default();
        if name.trim().is_empty() {
            errs.push(format!("row {}: empty name — skipped", ln + 2));
            continue;
        }
        if row.len() < cols.len() {
            errs.push(format!(
                "row {}: {} of {} columns — padded",
                ln + 2,
                row.len(),
                cols.len()
            ));
        }
        let mut values = std::collections::BTreeMap::new();
        for (i, c) in cols.iter().enumerate() {
            let val = row.get(i).cloned().unwrap_or_default();
            if i == nc {
                values.insert("Name".into(), val);
            } else if !c.eq_ignore_ascii_case("name") {
                values.insert(c.clone(), val);
            }
        }
        rows.push(values);
    }
    Ok((rows, errs))
}

/// Header column names of a CSV (trimmed), for import column mapping.
pub fn csv_columns(csv: &str) -> Vec<String> {
    csv.lines()
        .find(|l| !l.trim().is_empty())
        .map(|h| split_csv(h).iter().map(|s| s.trim().to_string()).collect())
        .unwrap_or_default()
}

/// Show/hide a column in a list table (column config).
pub fn table_toggle_column(app: &mut AppState, table: &str, col: usize) {
    let h = app.table_hidden.entry(table.to_string()).or_default();
    if let Some(p) = h.iter().position(|c| *c == col) {
        h.remove(p);
    } else {
        h.push(col);
    }
}

fn table_sort_filter(app: &AppState, table: &str) -> (crate::state::TableSort, String) {
    match table {
        "features" => (app.feature_sort, app.feature_filter.clone()),
        "primers" => (app.primer_sort, app.primer_filter.clone()),
        _ => (app.enzyme_sort, app.enzyme_filter.name.clone()),
    }
}

/// Save the current column config + sort + filter of a table as a named view.
pub fn view_save(app: &mut AppState, table: &str, name: &str) {
    let hidden = app.hidden_cols(table);
    let (sort, filter) = table_sort_filter(app, table);
    let name = if name.trim().is_empty() {
        format!("View {}", app.views.items.len() + 1)
    } else {
        name.trim().to_string()
    };
    app.views.add(
        table,
        &name,
        hidden,
        sort.column,
        sort.dir.map(|d| d == crate::state::SortDir::Desc),
        filter,
    );
    let _ = storage::views::save_to(&app.views, &app.profile_path(storage::views::path()));
    app.toast(format!("Saved view \"{name}\""));
}

/// Apply a saved view (column config + sort + filter) to its table.
pub fn view_load(app: &mut AppState, id: u64) {
    let Some(v) = app.views.get(id).cloned() else {
        return;
    };
    app.table_hidden.insert(v.table.clone(), v.hidden.clone());
    let sort = crate::state::TableSort {
        column: v.sort_column,
        dir: v.sort_desc.map(|d| {
            if d {
                crate::state::SortDir::Desc
            } else {
                crate::state::SortDir::Asc
            }
        }),
    };
    match v.table.as_str() {
        "features" => {
            app.feature_sort = sort;
            app.feature_filter = v.filter.clone();
        }
        "primers" => {
            app.primer_sort = sort;
            app.primer_filter = v.filter.clone();
        }
        _ => {
            app.enzyme_sort = sort;
            app.enzyme_filter.name = v.filter.clone();
        }
    }
    app.toast(format!("Loaded view \"{}\"", v.name));
}

pub fn view_delete(app: &mut AppState, id: u64) {
    if app.views.remove(id) {
        let _ = storage::views::save_to(&app.views, &app.profile_path(storage::views::path()));
        app.toast("View deleted");
    }
}

/// Open the biologics registration wizard.
pub fn open_biologics_register(app: &mut AppState) {
    app.dialog = Some(Dialog::Biologics(BiologicsDraft {
        name: String::new(),
        format: 0,
        numbering: 1,
        seqs: vec![
            String::new();
            domain::biologics::chain_kinds(domain::biologics::Format::IgG).len()
        ],
        error: None,
    }));
}

/// Register a biologic from the wizard draft (chains + auto domains).
pub fn biologics_save(app: &mut AppState, draft: &BiologicsDraft) -> Result<u64, String> {
    if draft.name.trim().is_empty() {
        return Err("Name required".into());
    }
    let format = domain::biologics::Format::ALL[draft.format.min(5)];
    let kinds = domain::biologics::chain_kinds(format);
    let mut chains = Vec::new();
    for (i, k) in kinds.iter().enumerate() {
        let seq = draft.seqs.get(i).cloned().unwrap_or_default();
        let clean: String = seq.chars().filter(|c| c.is_ascii_alphabetic()).collect();
        if clean.is_empty() {
            return Err(format!("Chain {} ({}) sequence required", i + 1, k.label()));
        }
        chains.push(domain::biologics::Chain {
            name: format!("{}{}", k.label(), i + 1),
            kind: *k,
            sequence: clean.to_ascii_uppercase(),
        });
    }
    let domains = domain::biologics::auto_domains(&chains);
    let now = unix_now();
    let b = domain::biologics::Biologic {
        id: 0,
        name: draft.name.trim().to_string(),
        format,
        chains,
        domains,
        numbering: domain::biologics::Numbering::ALL[draft.numbering.min(2)],
        created: now,
        updated: now,
    };
    let id = app.biologics.add(b);
    let _ = storage::biologics::save_to(
        &app.biologics,
        &app.profile_path(storage::biologics::path()),
    );
    audit(app, &format!("biologic:{id}"), "registered", &draft.name);
    app.bio_sel = Some(id);
    app.toast("Biologic registered");
    Ok(id)
}

pub fn biologics_delete(app: &mut AppState, id: u64) {
    if app.biologics.remove(id) {
        let _ = storage::biologics::save_to(
            &app.biologics,
            &app.profile_path(storage::biologics::path()),
        );
        if app.bio_sel == Some(id) {
            app.bio_sel = None;
        }
        audit(app, &format!("biologic:{id}"), "deleted", "");
        app.toast("Biologic deleted");
    }
}

/// Open the chemistry add-compound dialog.
pub fn open_chem_add(app: &mut AppState) {
    app.dialog = Some(Dialog::ChemAdd(ChemAddDraft::default()));
}

pub fn chem_add_save(app: &mut AppState, draft: &ChemAddDraft) -> Result<u64, String> {
    if draft.name.trim().is_empty() {
        return Err("Name required".into());
    }
    let id = app
        .chemistry
        .add_compound(draft.name.trim(), draft.smiles.trim())?;
    let _ = storage::chemistry::save_to(
        &app.chemistry,
        &app.profile_path(storage::chemistry::path()),
    );
    audit(app, &format!("compound:{id}"), "added", &draft.smiles);
    app.chem_sel = Some(id);
    app.toast("Compound added");
    Ok(id)
}

pub fn open_chem_reaction(app: &mut AppState) {
    app.dialog = Some(Dialog::ChemReaction(ChemReactionDraft {
        req: "1.0".into(),
        peq: "1.0".into(),
        ..Default::default()
    }));
}

pub fn chem_reaction_save(app: &mut AppState, draft: &ChemReactionDraft) -> Result<u64, String> {
    if draft.name.trim().is_empty() {
        return Err("Name required".into());
    }
    let n = app.chemistry.compounds.len();
    if n < 2 {
        return Err("Add at least two compounds first".into());
    }
    let r = draft.reactant.min(n - 1);
    let p = draft.product.min(n - 1);
    let req: f32 = draft
        .req
        .trim()
        .parse()
        .map_err(|_| "Invalid reactant eq")?;
    let peq: f32 = draft.peq.trim().parse().map_err(|_| "Invalid product eq")?;
    let rx = domain::chemistry::Reaction {
        id: 0,
        name: draft.name.trim().to_string(),
        reactants: vec![domain::chemistry::ReactionComponent {
            compound: app.chemistry.compounds[r].id,
            equivalents: req,
        }],
        products: vec![domain::chemistry::ReactionComponent {
            compound: app.chemistry.compounds[p].id,
            equivalents: peq,
        }],
    };
    let id = app.chemistry.add_reaction(rx);
    let _ = storage::chemistry::save_to(
        &app.chemistry,
        &app.profile_path(storage::chemistry::path()),
    );
    audit(app, &format!("reaction:{id}"), "added", &draft.name);
    app.toast("Reaction added");
    Ok(id)
}

pub fn open_chem_lot(app: &mut AppState) {
    app.dialog = Some(Dialog::ChemLot(ChemLotDraft {
        quantity: "100".into(),
        unit: "mg".into(),
        ..Default::default()
    }));
}

pub fn chem_lot_save(app: &mut AppState, draft: &ChemLotDraft) -> Result<u64, String> {
    let Some(comp) = app.chem_sel else {
        return Err("Select a compound first".into());
    };
    let qty: f32 = draft
        .quantity
        .trim()
        .parse()
        .map_err(|_| "Invalid quantity")?;
    let lib = domain::chemistry::salt_solvate_library();
    let form = lib
        .get(draft.form.min(lib.len() - 1))
        .map(|f| f.id)
        .unwrap_or(0);
    let id = app.chemistry.add_lot(comp, form, qty, draft.unit.trim());
    let _ = storage::chemistry::save_to(
        &app.chemistry,
        &app.profile_path(storage::chemistry::path()),
    );
    audit(app, &format!("lot:{id}"), "added", "");
    app.toast("Lot added");
    Ok(id)
}

pub fn chem_delete_compound(app: &mut AppState, id: u64) {
    if app.chemistry.remove_compound(id) {
        let _ = storage::chemistry::save_to(
            &app.chemistry,
            &app.profile_path(storage::chemistry::path()),
        );
        if app.chem_sel == Some(id) {
            app.chem_sel = None;
        }
        audit(app, &format!("compound:{id}"), "deleted", "");
        app.toast("Compound deleted");
    }
}

pub fn chem_delete_reaction(app: &mut AppState, id: u64) {
    if app.chemistry.remove_reaction(id) {
        let _ = storage::chemistry::save_to(
            &app.chemistry,
            &app.profile_path(storage::chemistry::path()),
        );
        audit(app, &format!("reaction:{id}"), "deleted", "");
        app.toast("Reaction deleted");
    }
}

/// Open the RNA add-oligo dialog.
pub fn open_rna_add(app: &mut AppState) {
    app.dialog = Some(Dialog::Rna(RnaDraft {
        mod_code: 4, // mA default
        ..Default::default()
    }));
}

pub fn rna_add_save(app: &mut AppState, draft: &RnaDraft) -> Result<u64, String> {
    if draft.name.trim().is_empty() {
        return Err("Name required".into());
    }
    let oligo = if !draft.helm.trim().is_empty() {
        domain::rna::Oligo::from_helm(draft.name.trim(), draft.helm.trim())?
    } else {
        let seq: String = draft
            .seq
            .chars()
            .filter(|c| "AUGCaugc".contains(*c))
            .map(|c| c.to_ascii_uppercase())
            .collect();
        if seq.is_empty() {
            return Err("RNA sequence (A/U/G/C) required".into());
        }
        let lib = domain::rna::monomer_library();
        let code = lib
            .get(draft.mod_code.min(lib.len() - 1))
            .map(|m| m.code.clone())
            .unwrap_or_else(|| "A".into());
        let pattern = domain::rna::Pattern::ALL[draft.pattern.min(3)];
        let mods = pattern
            .positions(seq.len())
            .into_iter()
            .map(|pos| domain::rna::Mod {
                pos,
                code: code.clone(),
            })
            .collect();
        domain::rna::Oligo {
            id: 0,
            name: draft.name.trim().to_string(),
            seq,
            mods,
        }
    };
    let id = app.rna.add(oligo);
    let _ = storage::rna::save_to(&app.rna, &app.profile_path(storage::rna::path()));
    audit(app, &format!("oligo:{id}"), "added", &draft.name);
    app.rna_sel = Some(id);
    app.toast("Oligo added");
    Ok(id)
}

/// Apply a bulk modification pattern to the selected oligo.
pub fn rna_apply_pattern(app: &mut AppState, pattern: usize, mod_code: usize) {
    let Some(sel) = app.rna_sel else { return };
    let lib = domain::rna::monomer_library();
    let code = lib
        .get(mod_code.min(lib.len() - 1))
        .map(|m| m.code.clone())
        .unwrap_or_else(|| "A".into());
    let pat = domain::rna::Pattern::ALL[pattern.min(3)];
    if let Some(o) = app.rna.get_mut(sel) {
        let positions = pat.positions(o.seq.len());
        o.mods.retain(|m| !positions.contains(&m.pos));
        for pos in positions {
            o.mods.push(domain::rna::Mod {
                pos,
                code: code.clone(),
            });
        }
        o.mods.sort_by_key(|m| m.pos);
    }
    let _ = storage::rna::save_to(&app.rna, &app.profile_path(storage::rna::path()));
    app.toast("Modification pattern applied");
}

pub fn rna_delete(app: &mut AppState, id: u64) {
    if app.rna.remove(id) {
        let _ = storage::rna::save_to(&app.rna, &app.profile_path(storage::rna::path()));
        if app.rna_sel == Some(id) {
            app.rna_sel = None;
        }
        audit(app, &format!("oligo:{id}"), "deleted", "");
        app.toast("Oligo deleted");
    }
}

/// Open the in-vivo add-study dialog.
pub fn open_iv_study(app: &mut AppState) {
    app.dialog = Some(Dialog::IvStudy(IvStudyDraft {
        species: "mouse".into(),
        strain: "BALB/c".into(),
        ..Default::default()
    }));
}

pub fn iv_study_save(app: &mut AppState, draft: &IvStudyDraft) -> Result<u64, String> {
    if draft.name.trim().is_empty() {
        return Err("Study name required".into());
    }
    let id = app.invivo.add_study(
        draft.name.trim(),
        draft.species.trim(),
        draft.strain.trim(),
        unix_now(),
    );
    // default cohort/cage for the study
    app.invivo.add_cohort(id, "C1", "cage-1");
    let _ = storage::invivo::save_to(&app.invivo, &app.profile_path(storage::invivo::path()));
    audit(app, &format!("study:{id}"), "created", &draft.name);
    app.iv_sel = Some(id);
    app.toast("Study created");
    Ok(id)
}

pub fn open_iv_animal(app: &mut AppState) {
    app.dialog = Some(Dialog::IvAnimal(IvAnimalDraft {
        weight: "20".into(),
        ..Default::default()
    }));
}

pub fn iv_animal_save(app: &mut AppState, draft: &IvAnimalDraft) -> Result<u64, String> {
    let Some(study) = app.iv_sel else {
        return Err("Select a study first".into());
    };
    let cohort = app
        .invivo
        .cohorts
        .iter()
        .find(|c| c.study == study)
        .map(|c| c.id)
        .ok_or("No cohort")?;
    let weight: f32 = draft.weight.trim().parse().map_err(|_| "Invalid weight")?;
    if draft.tag.trim().is_empty() {
        return Err("Tag required".into());
    }
    let sex = if draft.sex == 1 { 'M' } else { 'F' };
    let id = app.invivo.add_animal(cohort, draft.tag.trim(), sex, weight);
    let _ = storage::invivo::save_to(&app.invivo, &app.profile_path(storage::invivo::path()));
    audit(app, &format!("animal:{id}"), "added", &draft.tag);
    app.toast("Animal added");
    Ok(id)
}

pub fn open_iv_measure(app: &mut AppState) {
    app.dialog = Some(Dialog::IvMeasure(IvMeasureDraft {
        day: "0".into(),
        ..Default::default()
    }));
}

pub fn iv_measure_save(app: &mut AppState, draft: &IvMeasureDraft) -> Result<(), String> {
    let Some(study) = app.iv_sel else {
        return Err("Select a study first".into());
    };
    let animals: Vec<u64> = app
        .invivo
        .animals
        .iter()
        .filter(|a| {
            app.invivo
                .cohorts
                .iter()
                .any(|c| c.id == a.cohort && c.study == study)
        })
        .map(|a| a.id)
        .collect();
    let animal = *animals
        .get(draft.animal.min(animals.len().saturating_sub(1)))
        .ok_or("No animals")?;
    let day: u32 = draft.day.trim().parse().map_err(|_| "Invalid day")?;
    let weight: f32 = draft.weight.trim().parse().map_err(|_| "Invalid weight")?;
    let tumor = if draft.tumor.trim().is_empty() {
        None
    } else {
        Some(
            draft
                .tumor
                .trim()
                .parse::<f32>()
                .map_err(|_| "Invalid tumor")?,
        )
    };
    app.invivo.add_measurement(domain::invivo::Measurement {
        animal,
        day,
        weight_g: weight,
        tumor_mm3: tumor,
    });
    let _ = storage::invivo::save_to(&app.invivo, &app.profile_path(storage::invivo::path()));
    audit(
        app,
        &format!("animal:{animal}"),
        "measured",
        &format!("day {day}"),
    );
    app.toast("Measurement recorded");
    Ok(())
}

/// Randomize the selected study's animals into `groups` balanced groups.
pub fn iv_randomize(app: &mut AppState, groups: u32, seed: u64) {
    let Some(study) = app.iv_sel else { return };
    let ids: Vec<u64> = app
        .invivo
        .animals
        .iter()
        .filter(|a| {
            app.invivo
                .cohorts
                .iter()
                .any(|c| c.id == a.cohort && c.study == study)
        })
        .map(|a| a.id)
        .collect();
    let assign = domain::invivo::randomize(&ids, groups, seed);
    for (id, g) in assign {
        if let Some(a) = app.invivo.animals.iter_mut().find(|a| a.id == id) {
            a.group = g;
        }
    }
    let _ = storage::invivo::save_to(&app.invivo, &app.profile_path(storage::invivo::path()));
    audit(
        app,
        &format!("study:{study}"),
        "randomized",
        &format!("{groups} groups"),
    );
    app.toast("Randomized");
}

pub fn iv_delete_study(app: &mut AppState, id: u64) {
    if app.invivo.remove_study(id) {
        let _ = storage::invivo::save_to(&app.invivo, &app.profile_path(storage::invivo::path()));
        if app.iv_sel == Some(id) {
            app.iv_sel = None;
        }
        audit(app, &format!("study:{id}"), "deleted", "");
        app.toast("Study deleted");
    }
}

/// Open the bioprocess add-recipe dialog.
pub fn open_bp_recipe(app: &mut AppState) {
    app.dialog = Some(Dialog::BpRecipe(BpRecipeDraft::default()));
}

pub fn bp_recipe_save(app: &mut AppState, draft: &BpRecipeDraft) -> Result<u64, String> {
    if draft.name.trim().is_empty() {
        return Err("Recipe name required".into());
    }
    let r = domain::bioprocess::Recipe::template(0, draft.name.trim(), draft.product.trim());
    let id = app.bioprocess.add_recipe(r);
    let _ = storage::bioprocess::save_to(
        &app.bioprocess,
        &app.profile_path(storage::bioprocess::path()),
    );
    audit(app, &format!("recipe:{id}"), "created", &draft.name);
    app.toast("Recipe created");
    Ok(id)
}

/// Create a draft run from the first recipe (or a chosen one).
pub fn bp_run_create(app: &mut AppState, recipe: u64, name: &str) -> Result<u64, String> {
    let r = app
        .bioprocess
        .recipes
        .iter()
        .find(|r| r.id == recipe)
        .or_else(|| app.bioprocess.recipes.first())
        .cloned()
        .ok_or("Create a recipe first")?;
    let run = domain::bioprocess::Run::from_recipe(0, &r, name, unix_now());
    let id = app.bioprocess.add_run(run);
    let _ = storage::bioprocess::save_to(
        &app.bioprocess,
        &app.profile_path(storage::bioprocess::path()),
    );
    audit(app, &format!("run:{id}"), "planned", name);
    app.bp_sel = Some(id);
    app.toast("Run planned (draft)");
    Ok(id)
}

pub fn bp_run_action(app: &mut AppState, id: u64, action: &str) {
    let Some(run) = app.bioprocess.run_mut(id) else {
        return;
    };
    let ok = match action {
        "start" => run.start(),
        "complete" => run.complete(),
        "abort" => run.abort(),
        _ => false,
    };
    if ok {
        let _ = storage::bioprocess::save_to(
            &app.bioprocess,
            &app.profile_path(storage::bioprocess::path()),
        );
        audit(app, &format!("run:{id}"), action, "");
        app.toast(format!("Run {action}"));
    }
}

/// Toggle a worksheet step done flag on the selected run.
pub fn bp_step_toggle(app: &mut AppState, run: u64, step: usize) {
    let Some(r) = app.bioprocess.run_mut(run) else {
        return;
    };
    if let Some(op) = r.ops.get_mut(step) {
        op.done = !op.done;
    }
    let _ = storage::bioprocess::save_to(
        &app.bioprocess,
        &app.profile_path(storage::bioprocess::path()),
    );
}

pub fn open_bp_replicate(app: &mut AppState) {
    app.dialog = Some(Dialog::BpReplicate(BpReplicateDraft {
        temp: "37".into(),
        ph: "7.0".into(),
        ..Default::default()
    }));
}

pub fn bp_replicate_save(app: &mut AppState, draft: &BpReplicateDraft) -> Result<u64, String> {
    let Some(sel) = app.bp_sel else {
        return Err("Select a run first".into());
    };
    let titer: f32 = draft.titer.trim().parse().map_err(|_| "Invalid titer")?;
    let mut conditions = std::collections::BTreeMap::new();
    conditions.insert("temp_C".into(), draft.temp.trim().to_string());
    conditions.insert("pH".into(), draft.ph.trim().to_string());
    let rep = domain::bioprocess::Replicate {
        id: 0,
        label: if draft.label.trim().is_empty() {
            format!(
                "R{}",
                app.bioprocess
                    .run_mut(sel)
                    .map(|r| r.replicates.len() + 1)
                    .unwrap_or(1)
            )
        } else {
            draft.label.trim().to_string()
        },
        conditions,
        titer_mg_l: titer,
    };
    let Some(run) = app.bioprocess.run_mut(sel) else {
        return Err("Run missing".into());
    };
    let id = run.replicates.len() as u64 + 1;
    let mut rep = rep;
    rep.id = id;
    run.replicates.push(rep);
    let _ = storage::bioprocess::save_to(
        &app.bioprocess,
        &app.profile_path(storage::bioprocess::path()),
    );
    audit(app, &format!("run:{sel}"), "replicate", &draft.label);
    app.toast("Replicate added");
    Ok(id)
}

pub fn bp_delete_run(app: &mut AppState, id: u64) {
    if app.bioprocess.remove_run(id) {
        let _ = storage::bioprocess::save_to(
            &app.bioprocess,
            &app.profile_path(storage::bioprocess::path()),
        );
        if app.bp_sel == Some(id) {
            app.bp_sel = None;
        }
        audit(app, &format!("run:{id}"), "deleted", "");
        app.toast("Run deleted");
    }
}

fn split_csv(line: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut in_q = false;
    let mut chars = line.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '"' => {
                if in_q && chars.peek() == Some(&'"') {
                    cur.push('"');
                    chars.next();
                } else {
                    in_q = !in_q;
                }
            }
            ',' if !in_q => {
                out.push(cur.clone());
                cur.clear();
            }
            _ => cur.push(c),
        }
    }
    out.push(cur);
    out
}

// --- Phase 2: inventory -----------------------------------------------------

pub fn open_inventory(app: &mut AppState) {
    app.dialog = Some(Dialog::Inventory(InventoryDraft::default()));
}

pub fn inv_add(app: &mut AppState, draft: &InventoryDraft) {
    if draft.name.trim().is_empty() {
        app.toast("Name required");
        return;
    }
    let kind = match draft.kind {
        1 => domain::inventory::ContainerKind::Box,
        2 => domain::inventory::ContainerKind::Plate,
        3 => domain::inventory::ContainerKind::Tube,
        _ => domain::inventory::ContainerKind::Location,
    };
    let id = app.inventory.add(draft.name.trim(), kind, draft.parent);
    let _ = storage::inventory::save_to(
        &app.inventory,
        &app.profile_path(storage::inventory::path()),
    );
    audit(app, &format!("inv:{id}"), "created", &draft.name);
    app.toast(format!("{} created", kind.label()));
}

/// Fill the first four wells of a plate with the first registry entity (sample)
/// and blanks, as a quick-start demonstration of plate filling.
pub fn inv_fill_demo_wells(app: &mut AppState, plate: u64) {
    let entity = app.registry.entries.first().map(|e| e.name().to_string());
    let wells = vec![
        (
            "A1".to_string(),
            entity.clone(),
            domain::inventory::WellRole::Sample,
            10.0,
        ),
        (
            "A2".to_string(),
            entity.clone(),
            domain::inventory::WellRole::Sample,
            10.0,
        ),
        (
            "A3".to_string(),
            None,
            domain::inventory::WellRole::Control,
            10.0,
        ),
        (
            "A4".to_string(),
            None,
            domain::inventory::WellRole::Blank,
            10.0,
        ),
    ];
    let n = app.inventory.fill_wells(plate, &wells);
    let _ = storage::inventory::save_to(
        &app.inventory,
        &app.profile_path(storage::inventory::path()),
    );
    audit(
        app,
        &format!("inv:{plate}"),
        "filled",
        &format!("{n} wells"),
    );
    app.toast(format!("Filled {n} wells"));
}

/// Record an immutable plate snapshot as a notebook entry result table.
pub fn inv_snapshot_to_entry(app: &mut AppState, plate: u64) {
    let Some(snap) = app.inventory.snapshot(plate, unix_now()) else {
        app.toast("Select a plate");
        return;
    };
    let now = unix_now();
    let id = app.records.entries.len() as u64 + 1;
    let mut e =
        domain::notebook::Entry::new(id, format!("Plate snapshot: {}", snap.plate_name), now);
    let mut table = domain::notebook::ResultTable {
        name: format!("{} wells", snap.plate_name),
        columns: vec![
            "Well".into(),
            "Entity".into(),
            "Role".into(),
            "Volume (uL)".into(),
        ],
        rows: Vec::new(),
        submitted: true,
    };
    for (pos, entity, role, qty) in &snap.wells {
        table.rows.push(vec![
            pos.clone(),
            entity.clone().unwrap_or_default(),
            role.map(|r| r.label().to_string()).unwrap_or_default(),
            format!("{qty}"),
        ]);
    }
    e.tables.push(table);
    app.records.entries.push(e);
    audit(app, &format!("entry:{id}"), "snapshot", &snap.plate_name);
    persist_records(app);
    app.toast("Plate snapshot recorded to notebook");
}

// --- Phase 3: workflows -----------------------------------------------------

pub fn open_workflows(app: &mut AppState) {
    app.dialog = Some(Dialog::Workflows(WorkflowDraft::default()));
}

fn persist_workflows(app: &AppState) {
    let _ =
        storage::workflow::save_to(&app.workflows, &app.profile_path(storage::workflow::path()));
}

pub fn wf_add_task(app: &mut AppState, draft: &WorkflowDraft) {
    if draft.title.trim().is_empty() {
        app.toast("Task title required");
        return;
    }
    let id = app
        .workflows
        .add_task(draft.title.trim(), &draft.schema, "local", unix_now());
    persist_workflows(app);
    audit(app, &format!("task:{id}"), "created", &draft.title);
    app.toast("Task created");
}

pub fn wf_set_status(app: &mut AppState, id: u64, status: domain::workflow::TaskStatus) {
    if app.workflows.set_status(id, status, unix_now()) {
        persist_workflows(app);
        audit(app, &format!("task:{id}"), "status", status.label());
    }
}

pub fn wf_submit_output(app: &mut AppState, draft: &WorkflowDraft) {
    let Some(id) = draft.selected_task else {
        app.toast("Select a task");
        return;
    };
    let mut out = std::collections::BTreeMap::new();
    if !draft.output_key.trim().is_empty() {
        out.insert(
            draft.output_key.trim().to_string(),
            draft.output_val.clone(),
        );
    }
    if app.workflows.submit_outputs(id, out, unix_now()) {
        persist_workflows(app);
        audit(app, &format!("task:{id}"), "completed", "");
        app.toast("Task completed");
    }
}

pub fn wf_add_request(app: &mut AppState, draft: &WorkflowDraft) {
    if draft.request_name.trim().is_empty() {
        app.toast("Request name required");
        return;
    }
    let id = app
        .workflows
        .add_request(draft.request_name.trim(), "request");
    persist_workflows(app);
    audit(
        app,
        &format!("request:{id}"),
        "created",
        &draft.request_name,
    );
    app.toast("Request drafted");
}

pub fn wf_send_request(app: &mut AppState, id: u64) {
    let titles = vec!["Fulfill request".to_string()];
    if app.workflows.send_request(id, &titles, unix_now()) {
        persist_workflows(app);
        audit(app, &format!("request:{id}"), "sent", "");
        app.toast("Request sent (task created)");
    }
}

pub fn wf_execute_next(app: &mut AppState) {
    if let Some(stage) = app.workflows.next_pending() {
        if app.workflows.execute_step(stage, unix_now()) {
            persist_workflows(app);
            app.toast("Flow step started");
        }
    } else {
        app.toast("No pending flow step (or a failed task halts)");
    }
}

// --- v1.0: insights + automation -------------------------------------------

/// Build local datasets from the connected stores for Insights.
pub fn datasets(app: &AppState) -> Vec<domain::insights::Dataset> {
    let mut out = Vec::new();
    out.push(domain::insights::Dataset {
        name: "registry".into(),
        columns: vec!["Name".into(), "Kind".into(), "Archived".into()],
        rows: app
            .registry
            .entries
            .iter()
            .map(|e| {
                vec![
                    e.name().to_string(),
                    e.kind.label().to_string(),
                    e.archived.to_string(),
                ]
            })
            .collect(),
    });
    out.push(domain::insights::Dataset {
        name: "inventory".into(),
        columns: vec!["Name".into(), "Kind".into(), "Entity".into()],
        rows: app
            .inventory
            .containers
            .iter()
            .map(|c| {
                vec![
                    c.name.clone(),
                    c.kind.label().to_string(),
                    c.entity.clone().unwrap_or_default(),
                ]
            })
            .collect(),
    });
    out.push(domain::insights::Dataset {
        name: "tasks".into(),
        columns: vec!["Title".into(), "Status".into(), "Assignee".into()],
        rows: app
            .workflows
            .tasks
            .iter()
            .map(|t| {
                vec![
                    t.title.clone(),
                    t.status.label().to_string(),
                    t.assignee.clone(),
                ]
            })
            .collect(),
    });
    out.push(domain::insights::Dataset {
        name: "entries".into(),
        columns: vec!["Title".into(), "Status".into()],
        rows: app
            .records
            .entries
            .iter()
            .map(|e| vec![e.title.clone(), e.status.label().to_string()])
            .collect(),
    });
    out
}

pub fn insights_add_block(
    app: &mut AppState,
    name: &str,
    source: &str,
    chart: domain::insights::Chart,
    group_by: Option<String>,
) {
    app.analytics.blocks.push(domain::insights::Block {
        name: name.into(),
        source: source.into(),
        chart,
        group_by,
    });
    let _ = storage::analytics::save_to(
        &app.analytics,
        &app.profile_path(storage::analytics::path()),
    );
    app.toast("Insights block added");
}

pub fn insights_export(app: &mut AppState, source: &str) {
    let ds = datasets(app).into_iter().find(|d| d.name == source);
    let Some(ds) = ds else {
        app.toast("Unknown dataset");
        return;
    };
    if let Some(path) = crate::fs_dialogs::pick_save(
        "Export dataset",
        "CSV",
        &["csv"],
        &format!("{source}.csv"),
        app.config.last_directory.as_ref(),
    ) {
        match std::fs::write(&path, domain::insights::to_csv(&ds)) {
            Ok(()) => app.toast(format!("Exported {source} to {}", path.display())),
            Err(e) => app.toast(format!("Export failed: {e}")),
        }
    }
}

pub fn automation_add_pipeline(app: &mut AppState, name: &str) {
    let steps = vec![
        domain::automation::Step::IngestCsv,
        domain::automation::Step::OutputRegistry,
    ];
    app.analytics.automation.add_pipeline(name, steps);
    let _ = storage::analytics::save_to(
        &app.analytics,
        &app.profile_path(storage::analytics::path()),
    );
    app.toast("Pipeline drafted");
}

pub fn automation_publish(app: &mut AppState, id: u64) {
    if app.analytics.automation.publish(id) {
        let _ = storage::analytics::save_to(
            &app.analytics,
            &app.profile_path(storage::analytics::path()),
        );
        app.toast("Pipeline published");
    }
}

/// Load a CSV file from disk as the input for the next automation run.
pub fn automation_load_csv(app: &mut AppState) {
    let Some(path) = crate::fs_dialogs::pick_csv(app.config.last_directory.as_ref()) else {
        return;
    };
    match std::fs::read_to_string(&path) {
        Ok(text) => {
            app.automation_csv_name = path
                .file_name()
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_else(|| "input.csv".into());
            app.automation_csv = text;
            app.toast(format!("Loaded {}", app.automation_csv_name));
        }
        Err(e) => app.toast(format!("Read failed: {e}")),
    }
}

/// Run a pipeline over CSV; perform writeback (Registry / Results).
pub fn automation_run(app: &mut AppState, id: u64, csv: &str) {
    let Some(run) = app.analytics.automation.execute(id, csv) else {
        app.toast("Pipeline not found");
        return;
    };
    if run.status == domain::automation::RunStatus::Completed {
        let name_idx = run
            .columns
            .iter()
            .position(|c| c.eq_ignore_ascii_case("name"));
        let mut registered = 0;
        if let Some(nc) = name_idx {
            for row in &run.output {
                if let Some(name) = row.get(nc) {
                    if name.trim().is_empty() {
                        continue;
                    }
                    let mut values = std::collections::BTreeMap::new();
                    for (i, c) in run.columns.iter().enumerate() {
                        values.insert(c.clone(), row.get(i).cloned().unwrap_or_default());
                    }
                    app.registry
                        .add(domain::registry::Kind::Plasmid, values, unix_now());
                    registered += 1;
                }
            }
            let _ = storage::registry::save_to(
                &app.registry,
                &app.profile_path(storage::registry::path()),
            );
        }
        audit(
            app,
            &format!("pipeline:{id}"),
            "run",
            &format!("{} rows, {registered} registered", run.output.len()),
        );
        app.toast(format!(
            "Run completed: {} rows ({registered} registered)",
            run.output.len()
        ));
    } else {
        app.toast("Run failed (see log)");
    }
    let _ = storage::analytics::save_to(
        &app.analytics,
        &app.profile_path(storage::analytics::path()),
    );
}
