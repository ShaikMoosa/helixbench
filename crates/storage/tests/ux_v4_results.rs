use domain::results::{AnalysisResult, InputSnapshot};
use domain::{Document, OperationId, Range, ResultId, Topology};
use storage::results::ResultCatalog;

fn result() -> AnalysisResult {
    let doc = Document::new("captured".into(), b"ATGC".to_vec(), Topology::Linear);
    AnalysisResult {
        id: ResultId::new(),
        operation: OperationId::new(),
        tool: "digest".into(),
        method_version: "test-fixture-v1".into(),
        created: 1,
        inputs: vec![InputSnapshot::capture(&doc, Some(Range::new(0, 4))).unwrap()],
        parameters: serde_json::json!({"enzyme":"EcoRI"}),
        output: serde_json::json!({"sites":0}),
    }
}

#[test]
fn result_snapshot_survives_history_retention() {
    let mut doc = Document::new("original".into(), b"ATGC".to_vec(), Topology::Linear);
    let captured = InputSnapshot::capture(&doc, None).unwrap();
    let source = doc.source_ref();
    for i in 0..25 {
        doc.sequence.bases[0] = if i % 2 == 0 { b'C' } else { b'A' };
        doc.advance_revision();
        doc.save_version(format!("v{i}"), i);
    }
    assert_eq!(doc.versions.len(), 20);
    assert_eq!(captured.source(), source);
    assert_eq!(captured.document().sequence.bases, b"ATGC");
    assert!(captured.document().versions.is_empty());
    captured.validate().unwrap();
}

#[test]
fn copy_remints_document_identity_and_preserves_annotations() {
    let mut doc = Document::new("original".into(), b"ATGC".to_vec(), Topology::Linear);
    doc.add_feature(domain::Feature::new(
        0,
        "gene".into(),
        "CDS".into(),
        vec![Range::new(0, 3)],
    ));
    doc.save_version("v1".into(), 1);
    let copy = doc.independent_copy("copy".into());
    assert_ne!(copy.id, doc.id);
    assert_ne!(copy.revision_id, doc.revision_id);
    assert_eq!(copy.features, doc.features);
    assert_eq!(copy.versions[0].doc.id, copy.id);
    assert_ne!(
        copy.versions[0].doc.revision_id,
        doc.versions[0].doc.revision_id
    );
    assert_eq!(doc.clone(), doc);
}

#[test]
fn result_round_trip_rejects_mutating_a_persisted_result() {
    let path = std::env::temp_dir().join(format!("hbx-p01-results-{}.json", std::process::id()));
    let original = ResultCatalog {
        results: vec![result()],
    };
    storage::results::save_to(&original, &path).unwrap();
    assert_eq!(storage::results::load_from(&path).unwrap(), original);
    let mut changed = original.clone();
    changed.results[0].output = serde_json::json!({"sites":42});
    let attempt = storage::results::save_to(&changed, &path);
    let retained = storage::results::load_from(&path).unwrap();
    std::fs::remove_file(path).unwrap();
    assert!(attempt.is_err(), "rerun must allocate a new result ID");
    assert_eq!(retained, original);
}

#[test]
fn damaged_or_newer_result_store_is_not_treated_as_empty() {
    let path = std::env::temp_dir().join(format!("hbx-p01-broken-{}.json", std::process::id()));
    let original = br#"{"version":999,"catalog":{}}"#;
    std::fs::write(&path, original).unwrap();
    assert!(storage::results::load_from(&path).is_err());
    assert!(storage::results::save_to(&ResultCatalog::default(), &path).is_err());
    assert_eq!(std::fs::read(&path).unwrap(), original);
    std::fs::remove_file(path).unwrap();
}

#[test]
fn tampered_snapshot_is_rejected() {
    let snapshot = result().inputs.remove(0);
    let mut value = serde_json::to_value(snapshot).unwrap();
    value["document"]["name"] = "tampered".into();
    let changed: InputSnapshot = serde_json::from_value(value).unwrap();
    assert!(changed.validate().is_err());
}

#[test]
fn invalid_selection_is_not_captured() {
    let doc = Document::new("linear".into(), b"ATGC".to_vec(), Topology::Linear);
    assert!(InputSnapshot::capture(&doc, Some(Range::new(3, 2))).is_err());
    assert!(InputSnapshot::capture(&doc, Some(Range::new(0, 0))).is_err());
}
