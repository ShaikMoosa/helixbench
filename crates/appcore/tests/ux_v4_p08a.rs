use appcore::{
    navigation::{self, NavigationIntent, ReturnLocation, Route},
    selection::{self, DisplayToggle},
    state::{AppState, MapMode, View},
};
use domain::{Direction, Document, Feature, Range, Strand, Topology};
fn app() -> AppState {
    AppState::new(
        Document::new("construct".into(), b"ATGCGTAC".to_vec(), Topology::Circular),
        Default::default(),
    )
}
#[test]
fn circular_wrap_selection_survives_view_switch() {
    let mut a = app();
    let s = a.doc.source_ref();
    let r = selection::from_inclusive(&a, 7, 2).unwrap();
    assert_eq!(r, Range::new(6, 4));
    selection::apply(&mut a, s, Some(r), Strand::Top).unwrap();
    let id = a.doc.id;
    navigation::request_navigation(
        &mut a,
        NavigationIntent::Navigate(Route {
            view: View::Sequence,
            document: id,
            record: None,
        }),
    )
    .unwrap();
    assert_eq!(a.selection, Some(r));
    assert_eq!(
        selection::segments(&a, r).unwrap(),
        vec![Range::new(6, 2), Range::new(0, 2)]
    );
    assert!(selection::summary(&a).contains("7–2"));
}
#[test]
fn reverse_strand_selection_keeps_coordinates() {
    let mut a = app();
    let s = a.doc.source_ref();
    selection::apply(&mut a, s, Some(Range::new(1, 4)), Strand::Bottom).unwrap();
    let location = ReturnLocation::capture(&a);
    assert_eq!(location.selection.strand, Strand::Bottom);
    a.select(None);
    navigation::restore_return_location(&mut a, location).unwrap();
    assert_eq!(a.selection, Some(Range::new(1, 4)));
    assert_eq!(a.selection_strand, Strand::Bottom);
    assert!(selection::summary(&a).contains("reverse"));
}
#[test]
fn display_toggle_does_not_mark_dirty() {
    let mut a = app();
    let before = a.doc.clone();
    for t in DisplayToggle::ALL {
        selection::toggle_display(&mut a, t);
        selection::toggle_display(&mut a, t);
    }
    appcore::shell::set_representation(&mut a, MapMode::Linear);
    selection::zoom(&mut a, true);
    selection::fit(&mut a);
    assert_eq!(a.doc, before);
    assert!(!a.dirty);
    assert!(a.history.is_empty());
}
#[test]
fn invalid_coordinates_preserve_selection() {
    let mut a = app();
    let s = a.doc.source_ref();
    selection::apply(&mut a, s, Some(Range::new(2, 2)), Strand::Top).unwrap();
    for r in [
        Range::new(0, 0),
        Range::new(8, 1),
        Range::new(0, 9),
        Range::new(usize::MAX, 2),
    ] {
        assert!(selection::apply(&mut a, s, Some(r), Strand::Bottom).is_err());
        assert_eq!(a.selection, Some(Range::new(2, 2)));
    }
    assert!(selection::from_inclusive(&a, 0, 3).is_err());
    a.doc.sequence.topology = Topology::Linear;
    assert!(selection::from_inclusive(&a, 7, 2).is_err());
}
#[test]
fn stale_selection_does_not_retarget_document() {
    let mut a = app();
    let s = a.doc.source_ref();
    a.doc.advance_revision();
    assert!(selection::apply(&mut a, s, Some(Range::new(1, 2)), Strand::Bottom).is_err());
    assert!(a.selection.is_none());
}
#[test]
fn clear_removes_dependent_inspector_actions() {
    let mut a = app();
    let id = a.doc.add_feature(Feature::new(
        0,
        "feature".into(),
        "gene".into(),
        vec![Range::new(1, 3)],
    ));
    a.select_feature(id);
    let s = a.doc.source_ref();
    selection::apply(&mut a, s, None, Strand::Top).unwrap();
    assert!(a.selected_feature.is_none());
    assert!(a.selected_primer.is_none());
    assert!(matches!(
        appcore::shell::context(&a),
        appcore::shell::PanelContext::Document(_)
    ));
}
#[test]
fn short_and_empty_sequence_zoom_never_exceeds_bounds() {
    let mut a = app();
    selection::zoom(&mut a, true);
    assert_eq!(a.zoom_width, 8);
    a.doc.sequence.bases.clear();
    selection::zoom(&mut a, true);
    selection::fit(&mut a);
    assert_eq!((a.zoom_start, a.zoom_width), (0, 1));
    assert!(selection::from_inclusive(&a, 1, 1).is_err());
}
#[test]
fn reverse_feature_selection_sets_strand() {
    let mut a = app();
    let mut f = Feature::new(0, "reverse".into(), "gene".into(), vec![Range::new(2, 3)]);
    f.direction = Direction::Reverse;
    let id = a.doc.add_feature(f);
    a.select_feature(id);
    assert_eq!(a.selection_strand, Strand::Bottom);
    a.select_feature(u64::MAX);
    assert!(a.selected_feature.is_none());
}
#[test]
fn reverse_dna_search_preserves_hit_strand() {
    let mut a = app();
    a.search.query = "GCAT".into();
    a.run_search();
    let i = a
        .search
        .dna_hits
        .iter()
        .position(|h| h.strand == Strand::Bottom)
        .unwrap();
    a.search.current = i;
    let expected = a.search.dna_hits[i].clone();
    a.goto_current_hit();
    assert_eq!(a.selection, Some(Range::new(expected.start, expected.len)));
    assert_eq!(a.selection_strand, Strand::Bottom);
}
#[test]
fn linear_range_cannot_extend_beyond_document() {
    let mut a = app();
    a.doc.sequence.topology = Topology::Linear;
    a.select(Some(Range::new(7, 3)));
    assert!(a.selection.is_none());
}

#[test]
fn primer_selection_uses_clicked_binding_site() {
    let mut a = app();
    a.doc
        .primers
        .push(domain::Primer::new(7, "multi-site".into(), "ATGC"));
    a.analysis.primer_sites.insert(
        7,
        vec![
            domain::primer::BindingSite {
                start: 0,
                len: 2,
                strand: Strand::Top,
                tm: 50.0,
                matched_3prime: 2,
            },
            domain::primer::BindingSite {
                start: 6,
                len: 3,
                strand: Strand::Bottom,
                tm: 50.0,
                matched_3prime: 3,
            },
        ],
    );
    let source = a.doc.source_ref();
    selection::primer_site(&mut a, source, 7, Range::new(6, 3), Strand::Bottom).unwrap();
    assert_eq!(a.selection, Some(Range::new(6, 3)));
    assert_eq!(a.selection_strand, Strand::Bottom);
    assert_eq!(a.selected_primer, Some(7));
    assert!(selection::primer_site(&mut a, source, 7, Range::new(2, 2), Strand::Top).is_err());
    assert_eq!(a.selection, Some(Range::new(6, 3)));
}
#[test]
fn open_form_blocks_selection_without_losing_range() {
    let mut a = app();
    let source = a.doc.source_ref();
    selection::apply(&mut a, source, Some(Range::new(2, 3)), Strand::Bottom).unwrap();
    a.dialog = Some(appcore::state::Dialog::NewSequence(Default::default()));
    assert!(selection::apply(&mut a, source, None, Strand::Top).is_err());
    assert_eq!(a.selection, Some(Range::new(2, 3)));
}

#[test]
fn whitespace_and_other_strand_click_do_not_clear_reverse_selection() {
    let mut a = app();
    let source = a.doc.source_ref();
    selection::apply(&mut a, source, Some(Range::new(2, 1)), Strand::Bottom).unwrap();
    selection::click_base(&mut a, source, None).unwrap();
    assert_eq!(a.selection_strand, Strand::Bottom);
    selection::click_base(&mut a, source, Some((2, Strand::Top))).unwrap();
    assert_eq!(a.selection, Some(Range::new(2, 1)));
    assert_eq!(a.selection_strand, Strand::Top);
    selection::click_base(&mut a, source, Some((2, Strand::Top))).unwrap();
    assert!(a.selection.is_none());
}
#[test]
fn primer_list_preserves_first_binding_strand_and_clears_missing_range() {
    let mut a = app();
    a.doc
        .primers
        .push(domain::Primer::new(7, "reverse".into(), "ATGC"));
    a.doc
        .primers
        .push(domain::Primer::new(8, "unbound".into(), "TTTT"));
    a.analysis.primer_sites.insert(
        7,
        vec![domain::primer::BindingSite {
            start: 3,
            len: 3,
            strand: Strand::Bottom,
            tm: 50.0,
            matched_3prime: 3,
        }],
    );
    let source = a.doc.source_ref();
    selection::primer_from_list(&mut a, source, 7).unwrap();
    assert_eq!(a.selection_strand, Strand::Bottom);
    selection::primer_from_list(&mut a, source, 8).unwrap();
    assert_eq!(a.selected_primer, Some(8));
    assert!(a.selection.is_none());
}
