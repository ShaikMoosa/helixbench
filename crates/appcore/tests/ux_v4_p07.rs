use appcore::{
    shell::{self, FocusReturn, InspectorPlacement, PanelContext, PanelMode, ShellLayout},
    state::{AppState, MapMode, View},
};
use domain::{Document, Feature, Range, Topology};
fn app() -> AppState {
    AppState::new(
        Document::new(
            "real construct".into(),
            b"ATGCGTAC".to_vec(),
            Topology::Circular,
        ),
        storage::config::Config::default(),
    )
}
#[test]
fn panel_close_returns_focus() {
    let mut app = app();
    app.shell.open(FocusReturn::InspectorToggle);
    assert_eq!(app.shell.close(), FocusReturn::InspectorToggle);
    assert_eq!(app.shell.mode, PanelMode::Closed);
    assert_eq!(
        app.shell.layout(1600.0).inspector,
        InspectorPlacement::Hidden
    );
}
#[test]
fn header_actions_do_not_overflow_1280() {
    for width in [980.0, 1280.0, 1600.0] {
        let layout = ShellLayout::new(width, PanelMode::Auto);
        assert!(
            layout.header_actions_width + layout.title_min_width + layout.gap
                <= layout.content_width
        );
        assert!(layout.content_width >= 600.0);
        assert_eq!(layout.navigation_width, 224.0);
    }
    let narrow = ShellLayout::new(1280.0, PanelMode::Open);
    assert_eq!(narrow.inspector, InspectorPlacement::Drawer);
    assert_eq!(narrow.workspace_width, 1056.0);
}
#[test]
fn view_representation_does_not_change_topology() {
    let mut app = app();
    let original = app.doc.clone();
    for mode in [
        MapMode::Linear,
        MapMode::Plasmid,
        MapMode::Auto,
        MapMode::Metadata,
    ] {
        shell::set_representation(&mut app, mode);
        assert_eq!(app.doc, original);
        assert!(!app.dirty);
    }
}
#[test]
fn panel_context_tracks_actual_selected_feature() {
    let mut app = app();
    let id = app.doc.add_feature(Feature::new(
        0,
        "promoter".into(),
        "promoter".into(),
        vec![Range::new(1, 3)],
    ));
    app.selected_feature = Some(id);
    let context = shell::context(&app);
    assert!(matches!(context,PanelContext::Feature{feature,..} if feature==id));
    app.doc.features.clear();
    assert!(shell::validate_context(&app, &context).is_err());
}
#[test]
fn workspace_search_does_not_expose_hidden_dna_context() {
    let mut app = app();
    app.selection = Some(Range::new(1, 4));
    app.workspace_search.open = true;
    assert_eq!(shell::context(&app), PanelContext::WorkspaceSearch);
}
#[test]
fn panel_follows_page_and_does_not_retarget_stale_actions() {
    let mut app = app();
    let selected = shell::context(&app);
    app.doc.advance_revision();
    assert!(shell::validate_context(&app, &selected).is_err());
    app.view = View::Home;
    assert_eq!(shell::context(&app), PanelContext::Page(View::Home));
}
#[test]
fn manual_close_survives_resize_and_reopen_is_explicit() {
    let mut app = app();
    assert_eq!(
        app.shell.layout(1600.0).inspector,
        InspectorPlacement::Docked
    );
    assert_eq!(
        app.shell.layout(1280.0).inspector,
        InspectorPlacement::Hidden
    );
    app.shell.close();
    assert_eq!(
        app.shell.layout(1600.0).inspector,
        InspectorPlacement::Hidden
    );
    app.shell.open(FocusReturn::InspectorToggle);
    assert_eq!(
        app.shell.layout(1280.0).inspector,
        InspectorPlacement::Drawer
    );
}

#[test]
fn primers_page_inspects_the_primer_instead_of_a_previous_feature() {
    let mut app = app();
    let feature = app.doc.add_feature(Feature::new(
        0,
        "previous".into(),
        "gene".into(),
        vec![Range::new(0, 3)],
    ));
    app.doc
        .primers
        .push(domain::Primer::new(7, "current primer".into(), "ATGC"));
    app.selected_feature = Some(feature);
    app.selected_primer = Some(7);
    app.view = View::Primers;
    assert!(matches!(
        shell::context(&app),
        PanelContext::Primer { primer: 7, .. }
    ));
}
#[test]
fn invalid_selection_never_becomes_a_primer_action_context() {
    let mut app = app();
    app.selection = Some(Range::new(100, 3));
    assert!(matches!(shell::context(&app), PanelContext::Document(_)));
}

#[test]
fn cleared_feature_range_cannot_copy_the_whole_construct_as_a_primer() {
    let mut app = app();
    let id = app.doc.add_feature(Feature::new(
        0,
        "feature".into(),
        "gene".into(),
        vec![Range::new(1, 3)],
    ));
    app.selected_feature = Some(id);
    app.selection = None;
    let selected = shell::context(&app);
    assert!(shell::primer_range(&app, &selected).is_err());
    assert!(app.dialog.is_none());
    app.selection = Some(Range::new(1, 3));
    assert_eq!(
        shell::primer_range(&app, &shell::context(&app)).unwrap(),
        Range::new(1, 3)
    );
}
#[test]
fn panel_close_remembers_page_tools_opener() {
    let mut app = app();
    app.shell.open(FocusReturn::PageTools);
    assert_eq!(app.shell.close(), FocusReturn::PageTools);
}
