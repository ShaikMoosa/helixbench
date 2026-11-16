use appcore::{
    actions, primer_workflow as w,
    state::{AppState, Dialog, PcrDraft, PrimerDesignDraft},
};
use domain::{Document, Range, Topology};
const TEMPLATE: &str =
    "ATGCGTACGATCGGATCCGAATTCGCTAGCTAACGGTACCAAGGTTCCGATCGATTAAGCGTACCGATTCGGAATC";
fn app() -> AppState {
    AppState::new(
        Document::new(
            "template".into(),
            TEMPLATE.as_bytes().to_vec(),
            Topology::Linear,
        ),
        Default::default(),
    )
}
fn design(a: &mut AppState) -> PrimerDesignDraft {
    actions::open_primer_design(a);
    match a.dialog.take().unwrap() {
        Dialog::PrimerDesign(d) => d,
        _ => unreachable!(),
    }
}
fn pcr(a: &mut AppState) -> PcrDraft {
    actions::open_pcr(a);
    let mut d = match a.dialog.take().unwrap() {
        Dialog::Pcr(d) => d,
        _ => unreachable!(),
    };
    d.fwd = TEMPLATE[..20].into();
    d.rev = String::from_utf8(domain::sequence::reverse_complement(
        &TEMPLATE.as_bytes()[TEMPLATE.len() - 23..TEMPLATE.len() - 3],
    ))
    .unwrap();
    d
}
#[test]
fn primer_target_is_distinct_from_anchor() {
    let mut a = app();
    a.select(Some(Range::new(2, 8)));
    let mut d = design(&mut a);
    d.start = "11".into();
    d.len = "30".into();
    let request = w::design_request(&a, &d).unwrap();
    assert_eq!(d.anchor, Some(Range::new(2, 8)));
    assert_eq!(request.target, Range::new(10, 30));
    assert_eq!(a.selection, Some(Range::new(2, 8)));
}
#[test]
fn stale_candidate_cannot_be_added() {
    let mut a = app();
    let mut d = design(&mut a);
    d.start = "11".into();
    d.len = "30".into();
    w::run_design(&a, &mut d).unwrap();
    assert!(!d.fwd.is_empty() && !d.rev.is_empty());
    assert!(d.fwd_pick.is_none() && d.rev_pick.is_none());
    assert!(w::add_pair(&mut a, &d).is_err());
    d.fwd_pick = Some(0);
    d.rev_pick = Some(0);
    d.overhang = "GAATTC".into();
    assert!(w::add_pair(&mut a, &d).is_err());
    d.overhang.clear();
    a.doc.advance_revision();
    assert!(w::add_pair(&mut a, &d).is_err());
    assert!(a.doc.primers.is_empty());
}
#[test]
fn pcr_product_keeps_parent_provenance() {
    let mut a = app();
    let parent = a.doc.source_ref();
    let mut d = pcr(&mut a);
    w::run_pcr(&a, &mut d).unwrap();
    assert!(!d.result.as_ref().unwrap().products.is_empty());
    assert!(w::create_product(&mut a, &d).is_err());
    d.product_pick = Some(0);
    w::create_product(&mut a, &d).unwrap();
    assert_ne!(a.doc.id, parent.document);
    let provenance = a.doc.pcr_source.as_ref().unwrap();
    assert_eq!(provenance.parent, parent);
    assert_eq!(provenance.forward.sequence, &TEMPLATE.as_bytes()[..20]);
    assert!(a.library.iter().any(|old| old.doc.source_ref() == parent));
    let json = storage::document_bytes(&a.doc).unwrap();
    assert_eq!(
        storage::parse_document(&json).unwrap().pcr_source,
        a.doc.pcr_source
    );
}
#[test]
fn invalid_design_fields_never_default_or_clip() {
    let mut a = app();
    let mut d = design(&mut a);
    for (start, len, tm, tail) in [
        ("0", "30", "60", ""),
        ("abc", "30", "60", ""),
        ("60", "30", "60", ""),
        ("1", "30", "NaN", ""),
        ("1", "30", "60", "AT?G"),
    ] {
        d.start = start.into();
        d.len = len.into();
        d.target_tm = tm.into();
        d.overhang = tail.into();
        assert!(w::run_design(&a, &mut d).is_err());
        assert!(d.capture.is_none());
    }
}
#[test]
fn changed_pcr_input_or_template_cannot_create_old_product() {
    let mut a = app();
    let mut d = pcr(&mut a);
    w::run_pcr(&a, &mut d).unwrap();
    d.product_pick = Some(0);
    d.copy_features = !d.copy_features;
    assert!(w::create_product(&mut a, &d).is_err());
    d.copy_features = !d.copy_features;
    a.doc.advance_revision();
    assert!(w::create_product(&mut a, &d).is_err());
    assert!(a.library.is_empty());
}
#[test]
fn no_binding_is_a_completed_diagnostic() {
    let mut a = app();
    let mut d = pcr(&mut a);
    d.fwd = "AAAAAAAAAAAAAAAAAAAA".into();
    d.rev = "AAAAAAAAAAAAAAAAAAAA".into();
    w::run_pcr(&a, &mut d).unwrap();
    let result = d.result.unwrap();
    assert!(result.products.is_empty());
    assert!(result.diagnostic.is_some());
}
#[test]
fn invalid_pcr_text_is_not_silently_cleaned() {
    let mut a = app();
    let mut d = pcr(&mut a);
    d.fwd = "AT?GC".into();
    assert!(w::run_pcr(&a, &mut d).is_err());
    assert!(d.result.is_none());
    assert!(a.library.is_empty());
}
#[test]
fn selected_design_pair_is_independent_and_undoable() {
    let mut a = app();
    let mut d = design(&mut a);
    d.start = "11".into();
    d.len = "30".into();
    w::run_design(&a, &mut d).unwrap();
    d.fwd_pick = Some(0);
    d.rev_pick = Some(0);
    let (f, r) = w::add_pair(&mut a, &d).unwrap();
    assert_ne!(f, r);
    assert_eq!(a.doc.primer_pairs.len(), 1);
    assert!(a.undo());
    assert!(a.doc.primers.is_empty());
    assert!(a.doc.primer_pairs.is_empty());
}

#[test]
fn pcr_feature_copy_retains_joins_and_unique_ids() {
    let mut a = app();
    let mut feature = domain::Feature::new(
        0,
        "joined".into(),
        "gene".into(),
        vec![Range::new(25, 3), Range::new(30, 3)],
    );
    feature.note = "retain".into();
    feature.cleavage_arrows = vec![25];
    a.doc.add_feature(feature);
    let mut d = pcr(&mut a);
    d.fwd = format!("GGGG{}", &TEMPLATE[..20]);
    w::run_pcr(&a, &mut d).unwrap();
    d.product_pick = Some(0);
    w::create_product(&mut a, &d).unwrap();
    assert_eq!(a.doc.features.len(), 1);
    assert_eq!(a.doc.features[0].segments.len(), 2);
    assert_eq!(a.doc.features[0].note, "retain");
    assert_eq!(a.doc.features[0].segments[0].start, 29);
    assert_eq!(a.doc.features[0].cleavage_arrows, vec![29]);
    assert_ne!(a.doc.features[0].id, 0);
}
#[test]
fn ambiguous_names_need_id_and_deleted_ids_never_retarget() {
    let mut a = app();
    let f1 = a.doc.add_primer(domain::Primer::new(
        0,
        "same".into(),
        TEMPLATE.get(..20).unwrap(),
    ));
    let f2 = a.doc.add_primer(domain::Primer::new(
        0,
        "same".into(),
        TEMPLATE.get(3..23).unwrap(),
    ));
    assert!(w::resolve_primer(&a, "same", None).is_err());
    assert_eq!(
        w::resolve_primer(&a, "same", Some(f2)).unwrap().id,
        Some(f2)
    );
    let mut d = pcr(&mut a);
    w::choose_primer(&a, &mut d, true, f1).unwrap();
    a.doc.delete_primer(f1);
    assert!(w::run_pcr(&a, &mut d).is_err());
}
#[test]
fn product_save_failure_preserves_parent_and_provenance() {
    let mut a = app();
    let parent = a.doc.clone();
    let mut d = pcr(&mut a);
    w::run_pcr(&a, &mut d).unwrap();
    d.product_pick = Some(0);
    w::create_product(&mut a, &d).unwrap();
    let product = a.doc.clone();
    let root = std::env::temp_dir().join(format!("hbx-p08c-{}", domain::OperationId::new()));
    std::fs::create_dir_all(&root).unwrap();
    let journal = storage::operations::Journal::new(root.join("journal"));
    assert!(!appcore::operations::save_to(
        &mut a,
        &root,
        &journal,
        &root.join("config.json")
    ));
    assert_eq!(a.doc, product);
    assert!(a.dirty);
    assert_eq!(a.library[0].doc, parent);
    assert!(a.doc.pcr_source.is_some());
}

#[test]
fn highly_repetitive_pcr_requires_more_specific_primers() {
    let mut a = AppState::new(
        Document::new(
            "repeat".into(),
            "GC".repeat(40).into_bytes(),
            Topology::Linear,
        ),
        Default::default(),
    );
    let mut d = pcr(&mut a);
    d.fwd = "GCGCGCGCGCGCGCGCGCGC".into();
    d.rev = d.fwd.clone();
    assert!(w::run_pcr(&a, &mut d).is_err());
    assert!(d.result.is_none());
}

#[test]
fn captured_primer_row_cannot_select_same_numeric_id_in_another_template() {
    let mut a = app();
    let id = a
        .doc
        .add_primer(domain::Primer::new(0, "old".into(), &TEMPLATE[..20]));
    let row = w::PrimerRowTarget {
        source: a.doc.source_ref(),
        id,
    };
    let mut other = Document::new(
        "other".into(),
        TEMPLATE.as_bytes().to_vec(),
        Topology::Linear,
    );
    assert_eq!(
        other.add_primer(domain::Primer::new(0, "new".into(), &TEMPLATE[..20])),
        id
    );
    a.set_document(other, None);
    assert!(row.select(&mut a).is_err());
    assert!(a.selected_primer.is_none());
}

#[test]
fn wrapping_pcr_does_not_copy_a_join_whose_original_order_would_be_reversed() {
    let mut a = app();
    a.doc.sequence.topology = Topology::Circular;
    a.doc.add_feature(domain::Feature::new(
        0,
        "split across excluded span".into(),
        "gene".into(),
        vec![Range::new(15, 5), Range::new(55, 5)],
    ));
    let mut d = pcr(&mut a);
    d.fwd = TEMPLATE[50..70].into();
    d.rev = String::from_utf8(domain::sequence::reverse_complement(
        &TEMPLATE.as_bytes()[10..30],
    ))
    .unwrap();
    w::run_pcr(&a, &mut d).unwrap();
    let index = d
        .result
        .as_ref()
        .unwrap()
        .products
        .iter()
        .position(|p| p.start == 50 && p.rev_site.start == 10)
        .unwrap();
    d.product_pick = Some(index);
    w::create_product(&mut a, &d).unwrap();
    assert!(
        a.doc.features.is_empty(),
        "copy only fully contained spans; don't reorder a joined scientific feature silently"
    );
}
