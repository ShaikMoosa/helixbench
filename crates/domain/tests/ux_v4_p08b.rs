use domain::{annotations as a, Direction, Document, Feature, Range, Topology};
fn doc() -> Document {
    Document::new(
        "construct".into(),
        b"ATGCGTACATGC".to_vec(),
        Topology::Circular,
    )
}
#[test]
fn feature_edit_retains_id_after_sort() {
    let mut d = doc();
    let id = d.add_feature(Feature::new(
        0,
        "Z".into(),
        "gene".into(),
        vec![Range::new(1, 3)],
    ));
    d.add_feature(Feature::new(
        0,
        "A".into(),
        "gene".into(),
        vec![Range::new(5, 2)],
    ));
    let source = d.source_ref();
    let mut edited = d.feature(id).unwrap().clone();
    edited.name = "Edited".into();
    edited.direction = Direction::Reverse;
    edited.product = "protein".into();
    d.features.sort_by_key(|f| f.name.clone());
    a::replace_feature(&mut d, source, edited).unwrap();
    assert_eq!(d.feature(id).unwrap().name, "Edited");
    assert_eq!(d.feature(id).unwrap().product, "protein");
    assert_eq!(d.features.iter().filter(|f| f.name == "A").count(), 1);
}
#[test]
fn annotation_preview_cancel_is_noop() {
    let d = doc();
    let before = d.clone();
    let p = a::preview_library(
        &d,
        &[domain::feature_lib::LibFeature {
            name: "motif".into(),
            kind: "gene".into(),
            motif: b"ATGC".to_vec(),
            color: [1, 2, 3],
        }],
    );
    assert!(!p.candidates.is_empty());
    drop(p);
    assert_eq!(d, before);
}
#[test]
fn bulk_annotate_reports_per_document_outcome() {
    let mut docs = vec![doc(), doc()];
    let mut previews: Vec<_> = docs
        .iter()
        .map(|d| {
            a::preview_library(
                d,
                &[domain::feature_lib::LibFeature {
                    name: "motif".into(),
                    kind: "gene".into(),
                    motif: b"ATGC".to_vec(),
                    color: [1, 2, 3],
                }],
            )
        })
        .collect();
    previews[0].checked.fill(true);
    previews[1].checked.fill(true);
    docs[1].advance_revision();
    let result = a::apply_batch(&mut docs, &previews);
    assert!(result[0].added > 0);
    assert!(result[1].error.is_some());
    assert!(docs[1].features.is_empty());
}
#[test]
fn segments_validate_origin_and_bounds() {
    let d = doc();
    assert_eq!(
        a::parse_segments("11..2", 12, true).unwrap(),
        vec![Range::new(10, 4)]
    );
    assert!(a::parse_segments("13..14", 12, true).is_err());
    assert!(a::parse_segments("11..2", 12, false).is_err());
    assert!(a::parse_segments("1..4, 3..6", 12, false).is_ok());
    assert!(a::parse_segments("1", 0, true).is_err());
    assert_eq!(
        a::parse_segments("11..14", d.sequence.len(), true).unwrap(),
        vec![Range::new(10, 4)]
    );
}
#[test]
fn stale_or_missing_feature_is_rejected_without_mutation() {
    let mut d = doc();
    let mut f = Feature::new(99, "gone".into(), "gene".into(), vec![Range::new(0, 3)]);
    let before = d.clone();
    let source = d.source_ref();
    assert!(a::replace_feature(&mut d, source, f.clone()).is_err());
    assert_eq!(d, before);
    f.id = d.add_feature(f.clone());
    let old = d.source_ref();
    d.advance_revision();
    let before = d.clone();
    assert!(a::replace_feature(&mut d, old, f).is_err());
    assert_eq!(d, before);
}
#[test]
fn annotation_invalid_selection_and_invalid_motif_are_noop() {
    let mut d = doc();
    let p = a::preview_library(
        &d,
        &[domain::feature_lib::LibFeature {
            name: "invalid".into(),
            kind: "gene".into(),
            motif: b"AT?C".to_vec(),
            color: [1, 2, 3],
        }],
    );
    let before = d.clone();
    assert!(p.error.is_some());
    assert!(a::apply_preview(&mut d, &p).error.is_some());
    assert_eq!(d, before);
}
#[test]
fn invalid_translation_frame_is_rejected() {
    let mut d = doc();
    let id = d.add_feature(Feature::new(
        0,
        "feature".into(),
        "CDS".into(),
        vec![Range::new(0, 6)],
    ));
    let mut f = d.feature(id).unwrap().clone();
    f.frame_offset = 3;
    let source = d.source_ref();
    let before = d.clone();
    assert!(a::replace_feature(&mut d, source, f).is_err());
    assert_eq!(d, before);
}

#[test]
fn overlapping_proposals_are_visible_and_require_selection() {
    let d = doc();
    let lib = vec![
        domain::feature_lib::LibFeature {
            name: "first".into(),
            kind: "gene".into(),
            motif: b"ATGC".to_vec(),
            color: [1, 2, 3],
        },
        domain::feature_lib::LibFeature {
            name: "second".into(),
            kind: "gene".into(),
            motif: b"TGCG".to_vec(),
            color: [1, 2, 3],
        },
    ];
    let p = a::preview_library(&d, &lib);
    assert!(p.candidates.len() >= 2);
    assert!(p.candidates[1].conflict.is_some());
    assert!(!p.checked[1]);
}

#[test]
fn duplicate_library_names_cannot_mislabel_the_source_motif() {
    let d = doc();
    let lib = vec![
        domain::feature_lib::LibFeature {
            name: "same".into(),
            kind: "gene".into(),
            motif: b"GGGG".to_vec(),
            color: [1, 2, 3],
        },
        domain::feature_lib::LibFeature {
            name: "same".into(),
            kind: "gene".into(),
            motif: b"ATGC".to_vec(),
            color: [1, 2, 3],
        },
    ];
    let p = a::preview_library(&d, &lib);
    assert!(p.error.is_some());
    assert!(p.candidates.is_empty());
}

#[test]
fn existing_joined_overlaps_are_preserved_when_editing_metadata() {
    let mut d = doc();
    let id = d.add_feature(Feature::new(
        0,
        "joined".into(),
        "CDS".into(),
        vec![Range::new(0, 4), Range::new(3, 4)],
    ));
    let source = d.source_ref();
    let mut f = d.feature(id).unwrap().clone();
    f.note = "retained joined annotation".into();
    a::replace_feature(&mut d, source, f).unwrap();
    assert_eq!(
        d.feature(id).unwrap().segments,
        vec![Range::new(0, 4), Range::new(3, 4)]
    );
}
