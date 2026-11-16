//! Captured primer-design and PCR workflows; scientific engines stay in domain.
use crate::state::{AppState, OpenDoc, PcrDraft, PrimerDesignDraft, SearchState, View};
use domain::primer::{HybridizationParams, Primer};
use domain::primer_provenance::{
    BindingProvenance, PcrProvenance, PrimerInput, PrimerPairProvenance,
};
use domain::{Document, OperationId, Range, SourceRef, Strand};
#[derive(Clone, Debug, PartialEq)]
pub struct DesignRequest {
    pub source: SourceRef,
    pub template_sha256: String,
    pub target: Range,
    pub target_tm: f32,
    pub overhang: String,
    pub max_len: usize,
}
#[derive(Clone)]
pub struct DesignCapture {
    pub operation: OperationId,
    pub request: DesignRequest,
    pub forward: Vec<domain::primer_design::Candidate>,
    pub reverse: Vec<domain::primer_design::Candidate>,
}
#[derive(Clone)]
pub struct PcrRun {
    pub input: Box<domain::results::InputSnapshot>,
    pub result_id: domain::ResultId,
    pub created: u64,
    pub operation: OperationId,
    pub source: SourceRef,
    pub template_sha256: String,
    pub forward: PrimerInput,
    pub reverse: PrimerInput,
    pub pair: Option<OperationId>,
    pub params: HybridizationParams,
    pub copy_features: bool,
    pub products: Vec<domain::pcr::PcrProduct>,
    pub diagnostic: Option<String>,
}
fn source(app: &AppState, expected: Option<SourceRef>) -> Result<SourceRef, String> {
    let expected = expected.ok_or("Reopen this tool to capture its template")?;
    if expected != app.doc.source_ref() {
        return Err("The template changed; reopen the tool and review its current version".into());
    }
    if app.workspace_search.open || app.navigation.pending.is_some() {
        return Err("Return to the sequence and finish the pending decision first".into());
    }
    Ok(expected)
}
fn dna(text: &str, allow_empty: bool) -> Result<String, String> {
    let s = text.trim().to_ascii_uppercase();
    if !allow_empty && s.is_empty() {
        return Err("Enter a primer sequence or choose a saved primer".into());
    }
    if s.bytes().any(|b| domain::iupac::dna_mask(b) == 0) {
        return Err(
            "Use supported DNA symbols only; invalid characters are retained for correction".into(),
        );
    }
    Ok(s)
}
pub fn design_request(app: &AppState, draft: &PrimerDesignDraft) -> Result<DesignRequest, String> {
    let source = source(app, draft.source)?;
    let l = app.doc.sequence.len();
    if l < 18 {
        return Err("Primer design requires a template of at least 18 bases".into());
    }
    let start = draft
        .start
        .trim()
        .parse::<usize>()
        .map_err(|_| "Enter a numeric 1-based target start")?;
    let len = draft
        .len
        .trim()
        .parse::<usize>()
        .map_err(|_| "Enter a numeric target length")?;
    if start == 0
        || start > l
        || len == 0
        || len > l
        || !app.doc.sequence.is_circular() && start - 1 + len > l
    {
        return Err(
            "Target must fit the template; origin-spanning targets require circular DNA".into(),
        );
    }
    let tm = draft
        .target_tm
        .trim()
        .parse::<f32>()
        .map_err(|_| "Enter a numeric target Tm")?;
    if !tm.is_finite() || tm <= 0.0 || tm > 100.0 {
        return Err("Target Tm must be finite and between 0 and 100°C".into());
    }
    Ok(DesignRequest {
        source,
        template_sha256: storage::content_hash(&app.doc.sequence.bases),
        target: Range::new(start - 1, len),
        target_tm: tm,
        overhang: dna(&draft.overhang, true)?,
        max_len: 25.min(l),
    })
}
pub fn run_design(app: &AppState, draft: &mut PrimerDesignDraft) -> Result<(), String> {
    draft.capture = None;
    draft.fwd.clear();
    draft.rev.clear();
    draft.fwd_pick = None;
    draft.rev_pick = None;
    draft.error = None;
    let request = design_request(app, draft)?;
    let params = domain::primer_design::DesignParams {
        target_tm: request.target_tm,
        overhang: request.overhang.clone(),
        max_len: request.max_len,
        ..Default::default()
    };
    let (mut f, mut r) = domain::primer_design::design(
        &app.doc.sequence,
        request.target.start,
        request.target.len,
        &params,
    );
    let len = app.doc.sequence.len();
    for c in f.iter_mut().chain(r.iter_mut()) {
        c.start %= len;
    }
    draft.fwd = f.clone();
    draft.rev = r.clone();
    draft.capture = Some(DesignCapture {
        operation: OperationId::new(),
        request,
        forward: f,
        reverse: r,
    });
    Ok(())
}
pub fn add_pair(app: &mut AppState, draft: &PrimerDesignDraft) -> Result<(u64, u64), String> {
    let capture = draft.capture.as_ref().ok_or("Run primer design first")?;
    if design_request(app, draft)? != capture.request
        || draft.fwd != capture.forward
        || draft.rev != capture.reverse
    {
        return Err("Design inputs changed; run design again before adding candidates".into());
    }
    let f = capture
        .forward
        .get(
            draft
                .fwd_pick
                .ok_or("Select a forward candidate explicitly")?,
        )
        .ok_or("The forward candidate is unavailable")?;
    let r = capture
        .reverse
        .get(
            draft
                .rev_pick
                .ok_or("Select a reverse candidate explicitly")?,
        )
        .ok_or("The reverse candidate is unavailable")?;
    let mut ids = (0, 0);
    let anchor = draft.anchor;
    let req = &capture.request;
    let run = capture.operation;
    app.mutate("Add reviewed primer pair", Vec::new(), |doc| {
        let fseq = String::from_utf8_lossy(&f.sequence);
        let rseq = String::from_utf8_lossy(&r.sequence);
        let mut fp = Primer::new(0, format!("Design-F{}", doc.primers.len() + 1), &fseq);
        fp.color = [0x79, 0xc0, 0xff];
        let mut rp = Primer::new(0, format!("Design-R{}", doc.primers.len() + 2), &rseq);
        rp.color = [0xf8, 0x51, 0x49];
        fp.description = format!(
            "Design {run}; template {}@{}; target {}–{}; annealing {} nt; 5′ tail {}",
            req.source.document,
            req.source.revision,
            req.target.start + 1,
            req.target.start + req.target.len,
            f.len,
            req.overhang
        );
        rp.description = fp.description.clone();
        ids = (doc.add_primer(fp), doc.add_primer(rp));
        doc.primer_pairs.push(PrimerPairProvenance {
            pair: run,
            source: req.source,
            template_length: doc.sequence.len(),
            anchor_strand: draft.anchor_strand,
            forward_body: Range::new(f.start, f.len),
            reverse_body: Range::new(r.start, r.len),
            method: format!(
                "SantaLucia nearest-neighbor; 50 mM Na+; 250 nM oligo; 18–{} nt; GC40–60%; Tm±6°C",
                req.max_len
            ),
            anchor,
            target: req.target,
            target_tm: req.target_tm.to_string(),
            overhang: req.overhang.clone(),
            forward: PrimerInput {
                id: Some(ids.0),
                name: doc.primer(ids.0).unwrap().name.clone(),
                sequence: f.sequence.clone(),
            },
            reverse: PrimerInput {
                id: Some(ids.1),
                name: doc.primer(ids.1).unwrap().name.clone(),
                sequence: r.sequence.clone(),
            },
        });
    });
    app.select(Some(Range::new(f.start, f.len)));
    app.selected_feature = None;
    app.selected_primer = Some(ids.0);
    app.toast("Selected primer pair added — Ctrl+Z to undo");
    Ok(ids)
}
pub fn resolve_primer(app: &AppState, text: &str, id: Option<u64>) -> Result<PrimerInput, String> {
    let text = text.trim();
    if let Some(id) = id {
        let p = app
            .doc
            .primer(id)
            .ok_or("The chosen saved primer was deleted")?;
        if !p.name.eq_ignore_ascii_case(text) {
            return Err("Primer text changed; choose it again or enter its full sequence".into());
        }
        return Ok(PrimerInput {
            id: Some(id),
            name: p.name.clone(),
            sequence: p.sequence.clone(),
        });
    }
    let matches: Vec<_> = app
        .doc
        .primers
        .iter()
        .filter(|p| p.name.eq_ignore_ascii_case(text))
        .collect();
    if matches.len() > 1 {
        return Err("Several saved primers share this name; use the labeled ID picker".into());
    }
    if let Some(p) = matches.first() {
        return Ok(PrimerInput {
            id: Some(p.id),
            name: p.name.clone(),
            sequence: p.sequence.clone(),
        });
    }
    Ok(PrimerInput {
        id: None,
        name: "Entered sequence".into(),
        sequence: dna(text, false)?.into_bytes(),
    })
}
pub fn choose_primer(
    app: &AppState,
    draft: &mut PcrDraft,
    forward: bool,
    id: u64,
) -> Result<(), String> {
    source(app, draft.source)?;
    let p = app.doc.primer(id).ok_or("Primer is no longer available")?;
    if forward {
        draft.fwd = p.name.clone();
        draft.fwd_id = Some(id);
    } else {
        draft.rev = p.name.clone();
        draft.rev_id = Some(id);
    }
    draft.input_epoch = draft.input_epoch.wrapping_add(1);
    draft.result = None;
    draft.product_pick = None;
    Ok(())
}
pub fn run_pcr(app: &AppState, draft: &mut PcrDraft) -> Result<(), String> {
    draft.result = None;
    draft.product_pick = None;
    draft.error = None;
    let source = source(app, draft.source)?;
    let forward = resolve_primer(app, &draft.fwd, draft.fwd_id)?;
    let reverse = resolve_primer(app, &draft.rev, draft.rev_id)?;
    let params = HybridizationParams::default();
    let products = predict(&app.doc, &forward, &reverse, &params)?;
    let diagnostic=products.is_empty().then(||"Run completed: no predicted product. Check binding length/Tm, primer directions and template positions".into());
    let pair = app
        .doc
        .primer_pairs
        .iter()
        .rev()
        .find(|p| {
            p.forward.id == forward.id
                && p.reverse.id == reverse.id
                && p.forward.sequence == forward.sequence
                && p.reverse.sequence == reverse.sequence
        })
        .map(|p| p.pair);
    draft.result = Some(PcrRun {
        input: Box::new(domain::results::InputSnapshot::capture(&app.doc, None)?),
        result_id: domain::ResultId::new(),
        created: crate::actions::unix_now(),
        operation: OperationId::new(),
        source,
        template_sha256: storage::content_hash(&app.doc.sequence.bases),
        forward,
        reverse,
        pair,
        params,
        copy_features: draft.copy_features,
        products,
        diagnostic,
    });
    Ok(())
}
pub fn validate_pcr(app: &AppState, draft: &PcrDraft) -> Result<(), String> {
    let run = draft
        .result
        .as_ref()
        .ok_or("Run PCR and select a predicted product first")?;
    if source(app, draft.source)? != run.source
        || storage::content_hash(&app.doc.sequence.bases) != run.template_sha256
        || resolve_primer(app, &draft.fwd, draft.fwd_id)? != run.forward
        || resolve_primer(app, &draft.rev, draft.rev_id)? != run.reverse
        || draft.copy_features != run.copy_features
    {
        return Err("PCR inputs changed; run again before creating a product".into());
    }
    Ok(())
}
pub fn create_product(app: &mut AppState, draft: &PcrDraft) -> Result<(), String> {
    validate_pcr(app, draft)?;
    let run = draft.result.as_ref().unwrap();
    let p = run
        .products
        .get(
            draft
                .product_pick
                .ok_or("Select a predicted product explicitly")?,
        )
        .ok_or("The predicted product is unavailable")?;
    let parent = &app.doc;
    let length = parent.sequence.len();
    let f_tail = run.forward.sequence.len() - p.fwd_site.len;
    let r_tail = run.reverse.sequence.len() - p.rev_site.len;
    let mut doc = Document::new(
        format!("PCR {}", parent.name),
        p.bases.clone(),
        domain::Topology::Linear,
    );
    doc.default_genetic_code = parent.default_genetic_code;
    if run.copy_features {
        for feature in &parent.features {
            if !feature.visible {
                continue;
            }
            if feature.segments.is_empty()
                || feature
                    .segments
                    .iter()
                    .any(|range| !crate::shell::valid_range(app, *range))
            {
                continue;
            }
            let span = feature.span(length);
            let relative = (span.start + length - p.start) % length;
            if span.len > length || relative + span.len > p.template_len {
                continue;
            }
            let mut segments = Vec::new();
            let mut contained = true;
            for segment in &feature.segments {
                let rel = (segment.start + length - p.start) % length;
                if rel + segment.len > p.template_len {
                    contained = false;
                    break;
                }
                segments.push(Range::new(rel + f_tail, segment.len));
            }
            if contained && !segments.is_empty() {
                let mut copied = feature.clone();
                copied.id = 0;
                if segments
                    .windows(2)
                    .any(|pair| pair[0].start > pair[1].start)
                {
                    continue;
                }
                copied.segments = segments;
                copied.cleavage_arrows = feature
                    .cleavage_arrows
                    .iter()
                    .filter_map(|position| {
                        if *position >= length {
                            return None;
                        }
                        let relative = (*position + length - p.start) % length;
                        (relative < p.template_len).then_some(relative + f_tail)
                    })
                    .collect();
                doc.add_feature(copied);
            }
        }
    }
    let binding = |site: &domain::primer::BindingSite| BindingProvenance {
        range: Range::new(site.start, site.len),
        strand: site.strand,
        tm: site.tm.to_string(),
        matched_3prime: site.matched_3prime,
    };
    doc.pcr_source = Some(PcrProvenance {
        operation: run.operation,
        parent: run.source,
        template_length: parent.sequence.len(),
        template_topology: parent.sequence.topology,
        template_sha256: run.template_sha256.clone(),
        pair: run.pair,
        forward: run.forward.clone(),
        reverse: run.reverse.clone(),
        forward_site: binding(&p.fwd_site),
        reverse_site: binding(&p.rev_site),
        template_range: Range::new(p.start, p.template_len),
        forward_tail: run.forward.sequence[..f_tail].to_vec(),
        reverse_tail: run.reverse.sequence[..r_tail].to_vec(),
        min_3prime_match: run.params.min_3prime_match,
        minimum_tm: run.params.min_tm.to_string(),
        salt_mm: run.params.salt_mm.to_string(),
        primer_nm: run.params.primer_nm.to_string(),
        copy_features: run.copy_features,
    });
    doc.description=format!("PCR product from {} · document {} · revision {}. Forward {} ({} nt); reverse {} ({} nt). Body {} bp; full product {} bp including 5′ tails. Copy contained visible features: {}.",parent.name,run.source.document,run.source.revision,run.forward.name,run.forward.sequence.len(),run.reverse.name,run.reverse.sequence.len(),p.template_len,p.len,run.copy_features);
    // Do not suspend the PCR modal into the parent session.
    app.dialog = None;
    crate::navigation::suspend_session(app);
    let old = OpenDoc {
        doc: std::mem::replace(&mut app.doc, doc),
        path: app.path.take(),
        dirty: app.dirty,
    };
    app.library.push(old);
    app.dirty = true;
    app.saved_depth = None;
    app.navigation.saved_document = None;
    app.history.clear();
    app.future.clear();
    app.selection = None;
    app.selection_strand = Strand::Top;
    app.selected_feature = None;
    app.selected_primer = None;
    app.search = SearchState::default();
    app.zoom_start = 0;
    app.zoom_width = app.doc.sequence.len().max(1);
    app.analysis_dirty = true;
    app.view = View::Map;
    app.navigation.context_epoch = app.navigation.context_epoch.wrapping_add(1);
    app.toast("Independent PCR product created; save it to retain its provenance");
    Ok(())
}

#[derive(Clone, Copy)]
pub struct PrimerRowTarget {
    pub source: SourceRef,
    pub id: u64,
}
impl PrimerRowTarget {
    pub fn select(self, app: &mut AppState) -> Result<(), String> {
        crate::selection::primer_from_list(app, self.source, self.id)
    }
}

pub fn predict(
    doc: &Document,
    forward: &PrimerInput,
    reverse: &PrimerInput,
    params: &HybridizationParams,
) -> Result<Vec<domain::pcr::PcrProduct>, String> {
    if params.min_3prime_match == 0
        || !params.min_tm.is_finite()
        || !params.salt_mm.is_finite()
        || params.salt_mm <= 0.0
        || !params.primer_nm.is_finite()
        || params.primer_nm <= 0.0
    {
        return Err("Recorded binding parameters are invalid".into());
    }
    if forward.sequence.is_empty()
        || reverse.sequence.is_empty()
        || forward
            .sequence
            .iter()
            .chain(&reverse.sequence)
            .any(|b| domain::iupac::dna_mask(*b) == 0)
    {
        return Err("Recorded primers contain unsupported DNA".into());
    }
    let fp = Primer::new(
        0,
        forward.name.clone(),
        &String::from_utf8_lossy(&forward.sequence),
    );
    let rp = Primer::new(
        0,
        reverse.name.clone(),
        &String::from_utf8_lossy(&reverse.sequence),
    );
    let nf = domain::primer::binding_sites(&fp, &doc.sequence, params)
        .iter()
        .filter(|s| s.strand == Strand::Top)
        .count();
    let nr = domain::primer::binding_sites(&rp, &doc.sequence, params)
        .iter()
        .filter(|s| s.strand == Strand::Bottom)
        .count();
    let pairs = nf.saturating_mul(nr);
    if pairs > 512 || pairs.saturating_mul(doc.sequence.len()) > 20_000_000 {
        return Err("Too many binding-site pairs for an interactive prediction. Choose more specific primers; no partial product list was created".into());
    }
    Ok(domain::pcr::products(&doc.sequence, &fp, &rp, params))
}
