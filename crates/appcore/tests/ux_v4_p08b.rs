use appcore::{
    actions, annotation_review as r,
    state::{AppState, Dialog, FeatureDraft},
};
use domain::{Direction, Document, Feature, Range, Topology};
fn app() -> AppState {
    AppState::new(
        Document::new(
            "sequence".into(),
            b"ATGCGTACATGC".to_vec(),
            Topology::Circular,
        ),
        Default::default(),
    )
}
fn draft(a: &AppState, id: Option<u64>) -> FeatureDraft {
    FeatureDraft {
        source: a.doc.source_ref(),
        id,
        name: "edited".into(),
        kind: "gene".into(),
        segments_text: "11..2".into(),
        direction: Direction::Reverse,
        color: [1, 2, 3],
        note: "note".into(),
        product: "product".into(),
        translated: true,
        genetic_code: 1,
        first_codon_met: true,
        frame_offset: 1,
        error: None,
    }
}
#[test]
fn stale_feature_draft_cannot_edit_another_document() {
    let mut a = app();
    let d = draft(&a, None);
    a.doc.advance_revision();
    let before = a.doc.clone();
    assert!(actions::save_feature_draft(&mut a, &d).is_err());
    assert_eq!(a.doc, before);
    assert!(a.history.is_empty());
}
#[test]
fn deleted_feature_save_never_creates_an_undo_checkpoint() {
    let mut a = app();
    let d = draft(&a, Some(99));
    assert!(actions::save_feature_draft(&mut a, &d).is_err());
    assert!(!a.dirty);
    assert!(a.history.is_empty());
}
#[test]
fn feature_fields_and_undo_survive_save() {
    let mut a = app();
    let id = a.doc.add_feature(Feature::new(
        0,
        "old".into(),
        "gene".into(),
        vec![Range::new(0, 3)],
    ));
    let before = a.doc.clone();
    let d = draft(&a, Some(id));
    actions::save_feature_draft(&mut a, &d).unwrap();
    let f = a.doc.feature(id).unwrap();
    assert_eq!(f.product, "product");
    assert_eq!(f.segments, vec![Range::new(10, 4)]);
    assert_eq!(a.selected_feature, Some(id));
    assert!(a.undo());
    assert_eq!(a.doc, before);
}
#[test]
fn bulk_review_is_nonmutating_until_apply_and_is_undoable_per_document() {
    let mut a = app();
    a.feature_lib = vec![domain::feature_lib::LibFeature {
        name: "motif".into(),
        kind: "gene".into(),
        motif: b"ATGC".to_vec(),
        color: [1, 2, 3],
    }];
    let other = Document::new("other".into(), b"ATGCATGC".to_vec(), Topology::Linear);
    let otherid = other.id;
    a.library.push(appcore::state::OpenDoc {
        doc: other,
        path: None,
        dirty: false,
    });
    let before = a.doc.clone();
    let d = r::prepare_library(&a);
    assert_eq!(a.doc, before);
    a.dialog = Some(Dialog::AnnotationReview(d.clone()));
    let outcomes = r::apply(&mut a, &d).unwrap();
    assert_eq!(outcomes.len(), 2);
    assert!(outcomes.iter().all(|o| o.added > 0));
    assert!(a.undo());
    assert_eq!(a.doc, before);
    appcore::navigation::request_navigation(
        &mut a,
        appcore::navigation::NavigationIntent::SwitchDocument(otherid),
    )
    .unwrap();
    assert!(!a.doc.features.is_empty());
    assert!(a.undo());
    assert!(a.doc.features.is_empty());
}
#[test]
fn stale_annotation_document_reports_failure_without_changing_other_identity() {
    let mut a = app();
    a.feature_lib = vec![domain::feature_lib::LibFeature {
        name: "motif".into(),
        kind: "gene".into(),
        motif: b"ATGC".to_vec(),
        color: [1, 2, 3],
    }];
    let d = r::prepare_library(&a);
    a.doc.advance_revision();
    let before = a.doc.clone();
    a.dialog = Some(Dialog::AnnotationReview(d.clone()));
    let result = r::apply(&mut a, &d).unwrap();
    assert!(result[0].error.is_some());
    assert_eq!(a.doc, before);
}
