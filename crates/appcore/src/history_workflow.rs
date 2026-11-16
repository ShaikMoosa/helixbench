//! Named checkpoints are persistent metadata; preview never changes the document.
use crate::state::AppState;
use domain::{document::Version, Document, SourceRef};
#[derive(Clone)]
pub struct CheckpointDraft {
    pub source: SourceRef,
    pub label: String,
    pub error: Option<String>,
}
#[derive(Clone)]
pub struct HistoryReview {
    pub source: SourceRef,
    pub base: Document,
    pub target: Document,
    pub label: String,
    pub checkpoint: Option<Version>,
    pub edit_depth: Option<usize>,
    pub error: Option<String>,
}
impl HistoryReview {
    pub fn summary(&self) -> Vec<String> {
        let a = &self.base;
        let b = &self.target;
        let changed = a
            .sequence
            .bases
            .iter()
            .zip(&b.sequence.bases)
            .filter(|(x, y)| x != y)
            .count()
            + a.sequence.len().abs_diff(b.sequence.len());
        vec![
            format!(
                "Sequence: {} → {} bp; {} differing base positions",
                a.sequence.len(),
                b.sequence.len(),
                changed
            ),
            format!(
                "Topology: {:?} → {:?}",
                a.sequence.topology, b.sequence.topology
            ),
            format!(
                "Features: {} → {} ({} content)",
                a.features.len(),
                b.features.len(),
                if a.features == b.features {
                    "same"
                } else {
                    "changed"
                }
            ),
            format!(
                "Primers: {} → {} ({} content)",
                a.primers.len(),
                b.primers.len(),
                if a.primers == b.primers {
                    "same"
                } else {
                    "changed"
                }
            ),
            format!(
                "Chosen enzymes: {} → {}; metadata and provenance are restored with the snapshot",
                a.chosen_enzymes.len(),
                b.chosen_enzymes.len()
            ),
        ]
    }
}
pub fn save(app: &mut AppState, source: SourceRef, label: &str) -> Result<(), String> {
    crate::enzyme_workflow::validate_source(app, source)?;
    let label = label.trim();
    if label.is_empty() {
        return Err("Enter a name for this checkpoint".into());
    }
    app.doc
        .save_version(label.into(), crate::actions::unix_now());
    app.dirty = true;
    app.toast("Checkpoint created locally — save the document to retain it on disk");
    Ok(())
}
pub fn preview_version(
    app: &AppState,
    source: SourceRef,
    revision: SourceRef,
) -> Result<HistoryReview, String> {
    crate::enzyme_workflow::validate_source(app, source)?;
    let matching = app
        .doc
        .versions
        .iter()
        .filter(|v| v.doc.source_ref() == revision)
        .collect::<Vec<_>>();
    let v = *matching
        .first()
        .ok_or("This checkpoint is no longer available")?;
    if matching.iter().any(|other| other.doc != v.doc) {
        return Err("These legacy checkpoints share an identity but have different content; reopen the original document for review".into());
    }
    preview_checkpoint(app, source, v)
}
pub fn preview_checkpoint(
    app: &AppState,
    source: SourceRef,
    v: &Version,
) -> Result<HistoryReview, String> {
    crate::enzyme_workflow::validate_source(app, source)?;
    if !app.doc.versions.contains(v) {
        return Err("The exact checkpoint is no longer available; select it again".into());
    }
    Ok(HistoryReview {
        source,
        base: app.doc.clone(),
        target: v.doc.clone(),
        label: v.label.clone(),
        checkpoint: Some(v.clone()),
        edit_depth: None,
        error: None,
    })
}
pub fn preview_edit(
    app: &AppState,
    source: SourceRef,
    index: usize,
    revision: SourceRef,
) -> Result<HistoryReview, String> {
    crate::enzyme_workflow::validate_source(app, source)?;
    let entry = app
        .history
        .get(index)
        .filter(|e| e.snapshot.source_ref() == revision)
        .ok_or("The edit history changed; select the operation again")?;
    Ok(HistoryReview {
        source,
        base: app.doc.clone(),
        target: entry.snapshot.clone(),
        label: format!("Before {}", entry.label),
        checkpoint: None,
        edit_depth: Some(index),
        error: None,
    })
}
pub fn restore(app: &mut AppState, review: &HistoryReview) -> Result<(), String> {
    crate::enzyme_workflow::validate_source(app, review.source)?;
    if app.doc != review.base {
        return Err("The document or checkpoints changed; review the impact again".into());
    }
    if let Some(v) = &review.checkpoint {
        if !app.doc.versions.contains(v) {
            return Err("The captured checkpoint is no longer available".into());
        }
    }
    if let Some(i) = review.edit_depth {
        if app
            .history
            .get(i)
            .is_none_or(|e| e.snapshot != review.target)
        {
            return Err("The captured edit is no longer available".into());
        }
    }
    app.mutate(&format!("Restore {}", review.label), vec![], |d| {
        d.restore_snapshot(&review.target)
    });
    app.selection = None;
    app.selected_feature = None;
    app.selected_primer = None;
    app.scroll_to = None;
    app.zoom_start = 0;
    app.zoom_width = app.doc.sequence.len().max(1);
    app.analysis_dirty = true;
    app.toast("Snapshot restored as a new revision — Undo is available; save to retain it");
    Ok(())
}
pub fn cancel(app: &mut AppState) {
    app.dialog = None;
}
