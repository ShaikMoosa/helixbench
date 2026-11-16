//! Captured, non-mutating annotation proposals and validated feature edits.
use crate::feature_lib::LibFeature;
use crate::{Document, DocumentId, Feature, Range, SourceRef};
pub fn parse_segments(text: &str, len: usize, circular: bool) -> Result<Vec<Range>, String> {
    if len == 0 {
        return Err("Import a nonempty sequence before adding annotations".into());
    }
    let mut out = Vec::new();
    for part in text.split([',', ';']) {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }
        let (a, b) = part
            .split_once("..")
            .or_else(|| part.split_once('-'))
            .unwrap_or((part, part));
        let a: usize = a
            .trim()
            .parse()
            .map_err(|_| format!("Invalid start in {part}"))?;
        let b: usize = b
            .trim()
            .parse()
            .map_err(|_| format!("Invalid end in {part}"))?;
        if a == 0 || a > len || b == 0 {
            return Err(format!(
                "Start and end must identify bases in the {len} bp sequence"
            ));
        }
        let width = if b >= a {
            b - a + 1
        } else if circular && b <= len {
            len - a + 1 + b
        } else {
            return Err("A linear segment must end at or after its start".into());
        };
        if width > len || (!circular && b > len) {
            return Err("The segment exceeds the sequence bounds".into());
        }
        out.push(Range::new(a - 1, width));
    }
    if out.is_empty() {
        return Err("Enter at least one range, e.g. 100..200".into());
    }
    validate_segments(&out, len, circular)?;
    out.sort_by_key(|r| r.start);
    Ok(out)
}
fn validate_segments(segments: &[Range], len: usize, circular: bool) -> Result<(), String> {
    if len == 0 || segments.is_empty() {
        return Err("A feature needs a nonempty sequence and at least one segment".into());
    }
    for r in segments {
        if r.len == 0
            || r.start >= len
            || r.len > len
            || !circular && r.start.checked_add(r.len).is_none_or(|end| end > len)
        {
            return Err("A feature segment exceeds sequence bounds".into());
        }
    }
    Ok(())
}
pub fn validate_feature(doc: &Document, f: &Feature) -> Result<(), String> {
    if f.name.trim().is_empty() {
        return Err("Give the feature a name".into());
    }
    if f.kind.trim().is_empty() {
        return Err("Choose a feature type".into());
    }
    validate_segments(&f.segments, doc.sequence.len(), doc.sequence.is_circular())?;
    if f.frame_offset > 2 {
        return Err("Frame offset must be 0, 1 or 2".into());
    }
    if f.genetic_code != 0
        && !crate::genetic_code::CODES
            .iter()
            .any(|c| c.id == f.genetic_code)
    {
        return Err("Choose a supported genetic code".into());
    }
    Ok(())
}
pub fn replace_feature(
    doc: &mut Document,
    source: SourceRef,
    mut feature: Feature,
) -> Result<(), String> {
    if doc.source_ref() != source {
        return Err(
            "The source sequence changed; reopen the feature to review its current fields".into(),
        );
    }
    validate_feature(doc, &feature)?;
    let old = doc
        .feature(feature.id)
        .ok_or("The feature was deleted; close this draft and select another feature")?;
    feature.visible = old.visible;
    feature.cleavage_arrows = old.cleavage_arrows.clone();
    let id = feature.id;
    *doc.feature_mut(id).unwrap() = feature;
    Ok(())
}
#[derive(Clone)]
pub struct Candidate {
    pub feature: Feature,
    pub source_motif: String,
    pub source_name: String,
    pub conflict: Option<String>,
}
#[derive(Clone)]
pub struct AnnotationPreview {
    pub source: SourceRef,
    pub name: String,
    pub length: usize,
    pub candidates: Vec<Candidate>,
    pub checked: Vec<bool>,
    pub error: Option<String>,
}
#[derive(Clone, Debug)]
pub struct AnnotationOutcome {
    pub document: DocumentId,
    pub name: String,
    pub added: usize,
    pub skipped: usize,
    pub error: Option<String>,
}
fn overlaps(doc: &Document, f: &Feature) -> bool {
    let l = doc.sequence.len();
    doc.features.iter().any(|old| {
        old.segments.iter().any(|a| {
            f.segments.iter().any(|b| {
                a.contains(b.start, l, doc.sequence.is_circular())
                    || b.contains(a.start, l, doc.sequence.is_circular())
            })
        })
    })
}
fn preview(
    doc: &Document,
    mut candidates: Vec<Candidate>,
    error: Option<String>,
) -> AnnotationPreview {
    for i in 0..candidates.len() {
        let len = doc.sequence.len();
        let overlap = candidates[..i].iter().any(|old| {
            old.feature.segments.iter().any(|a| {
                candidates[i].feature.segments.iter().any(|b| {
                    a.contains(b.start, len, doc.sequence.is_circular())
                        || b.contains(a.start, len, doc.sequence.is_circular())
                })
            })
        });
        if overlap && candidates[i].conflict.is_none() {
            candidates[i].conflict =
                Some("Overlaps another proposal; review before selecting".into());
        }
    }
    let checked = candidates.iter().map(|c| c.conflict.is_none()).collect();
    AnnotationPreview {
        source: doc.source_ref(),
        name: doc.name.clone(),
        length: doc.sequence.len(),
        candidates,
        checked,
        error,
    }
}
pub fn preview_library(doc: &Document, library: &[LibFeature]) -> AnnotationPreview {
    let mut names = std::collections::HashSet::new();
    if library
        .iter()
        .any(|entry| !names.insert(entry.name.trim().to_string()))
    {
        return preview(doc,Vec::new(),Some("Library names must be unique so each candidate identifies its actual source motif. Remove or rename duplicate entries before review".into()));
    }

    if library.iter().any(|f| {
        f.name.trim().is_empty()
            || f.kind.trim().is_empty()
            || f.motif.is_empty()
            || f.motif.iter().any(|b| crate::iupac::dna_mask(*b) == 0)
    }) {
        return preview(doc,vec![],Some("Library contains an empty or invalid motif/name/type; correct it before annotation".into()));
    }
    let candidates = crate::feature_lib::auto_annotate(&doc.sequence, library, &doc.features)
        .into_iter()
        .map(|f| {
            let motif = library
                .iter()
                .find(|l| l.name == f.name)
                .map(|l| String::from_utf8_lossy(&l.motif).into_owned())
                .unwrap_or_default();
            let conflict = overlaps(doc, &f)
                .then(|| "Overlaps an existing annotation; review before selecting".into());
            Candidate {
                feature: f,
                source_motif: motif,
                source_name: "Feature library (exact motif; existing names skipped)".into(),
                conflict,
            }
        })
        .collect();
    preview(doc, candidates, None)
}
pub fn preview_common(doc: &Document) -> AnnotationPreview {
    let candidates = crate::common_features::detect(&doc.sequence)
        .into_iter()
        .map(|d| {
            let template = &crate::common_features::COMMON_FEATURES[d.template];
            let f = d.to_feature(0);
            let conflict = if crate::common_features::is_duplicate(&doc.features, &d) {
                Some("Already annotated at this position".into())
            } else {
                overlaps(doc, &f)
                    .then(|| "Overlaps an existing annotation; review before selecting".into())
            };
            Candidate {
                feature: f,
                source_motif: template.sequence.into(),
                source_name: template.name.into(),
                conflict,
            }
        })
        .collect();
    preview(doc, candidates, None)
}
pub fn apply_preview(doc: &mut Document, p: &AnnotationPreview) -> AnnotationOutcome {
    let mut result = AnnotationOutcome {
        document: p.source.document,
        name: p.name.clone(),
        added: 0,
        skipped: 0,
        error: None,
    };
    if doc.source_ref() != p.source {
        result.error = Some("Document changed since preview; refresh and review again".into());
        return result;
    }
    if let Some(e) = &p.error {
        result.error = Some(e.clone());
        return result;
    }
    if p.checked.len() != p.candidates.len() {
        result.error = Some("Candidate selection changed; refresh preview".into());
        return result;
    }
    let mut additions = Vec::new();
    for (candidate, checked) in p.candidates.iter().zip(&p.checked) {
        if !checked {
            result.skipped += 1;
            continue;
        }
        if let Err(e) = validate_feature(doc, &candidate.feature) {
            result.error = Some(e);
            return result;
        }
        if doc
            .features
            .iter()
            .chain(additions.iter())
            .any(|f: &Feature| {
                f.name == candidate.feature.name
                    && f.segments == candidate.feature.segments
                    && f.direction == candidate.feature.direction
            })
        {
            result.skipped += 1;
        } else {
            additions.push(candidate.feature.clone());
        }
    }
    result.added = additions.len();
    for f in additions {
        doc.add_feature(f);
    }
    if result.added > 0 {
        doc.advance_revision();
    }
    result
}
pub fn apply_batch(
    documents: &mut [Document],
    previews: &[AnnotationPreview],
) -> Vec<AnnotationOutcome> {
    previews
        .iter()
        .map(
            |p| match documents.iter_mut().find(|d| d.id == p.source.document) {
                Some(doc) => apply_preview(doc, p),
                None => AnnotationOutcome {
                    document: p.source.document,
                    name: p.name.clone(),
                    added: 0,
                    skipped: 0,
                    error: Some("Document is no longer open".into()),
                },
            },
        )
        .collect()
}

/// Overlapping joined segments were supported by the existing editor; warn without rewriting them.
pub fn segments_overlap(segments: &[Range], len: usize, circular: bool) -> bool {
    len > 0
        && segments.iter().enumerate().any(|(i, r)| {
            segments[..i].iter().any(|other| {
                r.contains(other.start, len, circular) || other.contains(r.start, len, circular)
            })
        })
}
