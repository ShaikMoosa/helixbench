//! Review annotation proposals without mutating documents until explicit apply.
use crate::state::{AppState, Dialog};
use domain::annotations::{self, AnnotationOutcome, AnnotationPreview};
#[derive(Clone)]
pub struct ReviewDraft {
    pub previews: Vec<AnnotationPreview>,
    pub outcomes: Option<Vec<AnnotationOutcome>>,
    pub error: Option<String>,
}
pub fn prepare_library(app: &AppState) -> ReviewDraft {
    let mut previews = vec![annotations::preview_library(&app.doc, &app.feature_lib)];
    previews.extend(
        app.library
            .iter()
            .map(|d| annotations::preview_library(&d.doc, &app.feature_lib)),
    );
    ReviewDraft {
        previews,
        outcomes: None,
        error: None,
    }
}
pub fn prepare_common(app: &AppState) -> ReviewDraft {
    ReviewDraft {
        previews: vec![annotations::preview_common(&app.doc)],
        outcomes: None,
        error: None,
    }
}
pub fn apply(app: &mut AppState, draft: &ReviewDraft) -> Result<Vec<AnnotationOutcome>, String> {
    if !matches!(app.dialog, Some(Dialog::AnnotationReview(_)))
        || app.navigation.pending.is_some()
        || app.workspace_search.open
    {
        return Err("Open the annotation review and finish other decisions first".into());
    }
    if draft.outcomes.is_some() {
        return Err("This review was already applied; create a new preview".into());
    }
    let original = app.doc.id;
    let saved_dialog = app.dialog.take();
    let mut outcomes = Vec::new();
    for preview in &draft.previews {
        if let Err(error) = crate::navigation::switch_inner(app, preview.source.document) {
            outcomes.push(AnnotationOutcome {
                document: preview.source.document,
                name: preview.name.clone(),
                added: 0,
                skipped: 0,
                error: Some(error),
            });
            continue;
        }
        let mut prepared = app.doc.clone();
        let outcome = annotations::apply_preview(&mut prepared, preview);
        if outcome.added > 0 && outcome.error.is_none() {
            app.mutate("Apply reviewed annotations", Vec::new(), |doc| {
                *doc = prepared
            });
        }
        outcomes.push(outcome);
    }
    let restored = crate::navigation::switch_inner(app, original);
    app.dialog = saved_dialog;
    restored?;
    Ok(outcomes)
}
