use appcore::{
    state::{AppState, View},
    viewer::{self, Surface},
};
use domain::{Document, Topology};
#[test]
fn zoom_does_not_freeze_defaults_and_legacy_toggles_are_local() {
    let mut a = app();
    viewer::set_zoom(&mut a, Surface::CircularMap, 1.7);
    a.prefs.show_features = false;
    assert!(!viewer::prefs(&a, Surface::CircularMap).show_features);
    let defaults = a.prefs.clone();
    appcore::selection::toggle_display(&mut a, appcore::selection::DisplayToggle::Features);
    assert!(viewer::prefs(&a, Surface::CircularMap).show_features);
    assert_eq!(a.prefs, defaults);
    assert!(!viewer::prefs(&a, Surface::Sequence).show_features);
}
#[test]
fn linear_zoom_clamps_at_both_ends_while_circular_zoom_can_wrap() {
    let mut a = app();
    a.doc.sequence.topology = Topology::Linear;
    a.view = View::Sequence;
    a.selection = Some(domain::Range::new(0, 1));
    viewer::set_zoom(&mut a, Surface::Sequence, 2.0);
    assert_eq!(a.zoom_start, 0);
    a.selection = Some(domain::Range::new(199, 1));
    viewer::set_zoom(&mut a, Surface::Sequence, 2.0);
    assert_eq!(a.zoom_start, 100);
    a.doc.sequence.topology = Topology::Circular;
    a.selection = Some(domain::Range::new(0, 1));
    viewer::set_zoom(&mut a, Surface::Sequence, 2.0);
    assert_eq!(a.zoom_start, 150);
}
#[test]
fn sequence_slider_arrows_always_move_by_at_least_one_base() {
    let mut a = app();
    a.view = View::Sequence;
    viewer::set_zoom(&mut a, Surface::Sequence, 5.0);
    assert_eq!(a.zoom_width, 40);
    viewer::step_zoom(&mut a, Surface::Sequence, true);
    assert_eq!(a.zoom_width, 39);
    viewer::step_zoom(&mut a, Surface::Sequence, false);
    assert_eq!(a.zoom_width, 40);
}
#[test]
fn metadata_has_no_active_viewer_controls() {
    let mut a = app();
    a.map_mode = appcore::state::MapMode::Metadata;
    assert_eq!(viewer::active(&a), None);
}
fn app() -> AppState {
    AppState::new(
        Document::new("template".into(), vec![b'A'; 200], Topology::Circular),
        Default::default(),
    )
}
#[test]
fn layer_controls_only_offer_layers_the_view_can_render() {
    assert!(!viewer::supports_layer(Surface::LinearMap, "orfs"));
    assert!(!viewer::supports_layer(
        Surface::CircularMap,
        "translations"
    ));
    assert!(viewer::supports_layer(Surface::CircularMap, "orfs"));
    assert!(viewer::supports_layer(Surface::Sequence, "translations"));
    assert!(!viewer::supports_layer(Surface::Sequence, "unknown-layer"));
}
#[test]
fn display_controls_do_not_edit_science_or_history() {
    let mut a = app();
    let before = a.doc.clone();
    viewer::set_zoom(&mut a, Surface::CircularMap, 1.73);
    viewer::rotate(&mut a, Surface::CircularMap, 47.0);
    viewer::toggle_layer(&mut a, Surface::CircularMap, "features");
    viewer::set_palette(&mut a, Surface::CircularMap, true);
    assert_eq!(a.doc, before);
    assert!(!a.dirty);
    assert!(a.history.is_empty());
    viewer::reset(&mut a, Surface::CircularMap);
    assert_eq!(viewer::settings(&a, Surface::CircularMap).zoom, 1.0);
    assert_eq!(a.doc, before);
}
#[test]
fn same_name_documents_and_view_surfaces_have_independent_settings() {
    let mut a = app();
    let id = a.doc.id;
    viewer::set_zoom(&mut a, Surface::CircularMap, 1.8);
    viewer::rotate(&mut a, Surface::CircularMap, 95.0);
    assert_eq!(viewer::settings(&a, Surface::LinearMap).zoom, 1.0);
    let second = Document::new("template".into(), vec![b'C'; 200], Topology::Circular);
    let second_id = second.id;
    a.library.push(appcore::state::OpenDoc {
        doc: second,
        path: None,
        dirty: false,
    });
    appcore::workspace::open_sequence(&mut a, second_id).unwrap();
    assert_eq!(viewer::settings(&a, Surface::CircularMap).rotation, 0.0);
    appcore::workspace::open_sequence(&mut a, id).unwrap();
    assert_eq!(viewer::settings(&a, Surface::CircularMap).rotation, 95.0);
}
#[test]
fn transformed_points_round_trip_before_genomic_hit_testing() {
    let transform = viewer::Transform {
        zoom: 1.73,
        rotation: 253.0,
    };
    for p in [[0.0, 0.0], [35.0, -160.0], [-79.0, 25.0]] {
        let q = transform.inverse(transform.forward(p));
        assert!((q[0] - p[0]).abs() < 0.001 && (q[1] - p[1]).abs() < 0.001);
    }
}
#[test]
fn rotation_is_capability_gated_and_invalid_values_cannot_poison_geometry() {
    let mut a = app();
    viewer::rotate(&mut a, Surface::LinearMap, 90.0);
    viewer::rotate(&mut a, Surface::Sequence, 90.0);
    assert_eq!(viewer::settings(&a, Surface::LinearMap).rotation, 0.0);
    assert_eq!(viewer::settings(&a, Surface::Sequence).rotation, 0.0);
    viewer::set_zoom(&mut a, Surface::CircularMap, f32::NAN);
    viewer::rotate(&mut a, Surface::CircularMap, f32::INFINITY);
    assert!(viewer::settings(&a, Surface::CircularMap).zoom.is_finite());
    assert!(viewer::settings(&a, Surface::CircularMap)
        .rotation
        .is_finite());
    viewer::rotate(&mut a, Surface::CircularMap, -15.0);
    assert_eq!(viewer::settings(&a, Surface::CircularMap).rotation, 345.0);
}
#[test]
fn sequence_zoom_uses_real_bases_and_fit_does_not_clear_selection() {
    let mut a = app();
    a.view = View::Sequence;
    a.selection = Some(domain::Range::new(45, 20));
    viewer::set_zoom(&mut a, Surface::Sequence, 2.0);
    assert_eq!(a.zoom_width, 100);
    viewer::fit(&mut a, Surface::Sequence);
    assert_eq!(a.zoom_width, 200);
    assert_eq!(a.selection, Some(domain::Range::new(45, 20)));
    a.doc.sequence = domain::sequence::DnaSequence::new(vec![b'A'; 3], Topology::Linear);
    viewer::set_zoom(&mut a, Surface::Sequence, 10000.0);
    assert_eq!(a.zoom_width, 3);
}
