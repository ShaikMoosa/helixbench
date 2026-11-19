use appcore::{
    enzyme_workflow, navigation, results,
    state::{AppState, View},
};
use domain::{Document, OperationId, Topology};

fn app() -> AppState {
    AppState::new_in(
        Document::new(
            "template".into(),
            b"AAAAGAATTCTTTTGGATCCAAAA".to_vec(),
            Topology::Linear,
        ),
        Default::default(),
        std::env::temp_dir().join(format!("v6-navigation-{}", OperationId::new())),
    )
}
fn result(app: &mut AppState) -> domain::results::AnalysisResult {
    app.doc.chosen_enzymes = vec!["EcoRI".into()];
    let mut draft = enzyme_workflow::draft(app);
    enzyme_workflow::run(app, &mut draft).unwrap();
    results::from_digest(draft.result.as_ref().unwrap()).unwrap()
}
#[test]
fn results_catalog_has_its_own_destination_and_keeps_source_edits() {
    let mut a = app();
    let source = a.doc.id;
    a.mutate("unsaved name", vec![], |d| {
        d.name = "edited template".into()
    });
    results::browse(&mut a).unwrap();
    assert_eq!(a.view.label(), "Results");
    assert_eq!(a.doc.id, source);
    assert!(a.dirty);
    results::close(&mut a).unwrap();
    assert_eq!(a.view, View::Map);
    assert_eq!(a.doc.name, "edited template");
}
#[test]
fn preview_uses_results_route_and_back_restores_exact_task() {
    let mut a = app();
    let run = result(&mut a);
    let id = run.id;
    a.view = View::Enzymes;
    results::open_preview(&mut a, run).unwrap();
    assert_eq!(a.view.label(), "Results");
    assert_eq!(a.results.pending.as_ref().unwrap().id, id);
    results::close(&mut a).unwrap();
    assert_eq!(a.view, View::Enzymes);
}
#[test]
fn saved_result_source_round_trip_retains_exact_result_identity() {
    let mut a = app();
    let run = result(&mut a);
    results::save_analysis(&mut a, &run).unwrap();
    results::open(&mut a, run.id).unwrap();
    assert_eq!(a.view.label(), "Results");
    results::open_current_source(&mut a, &run).unwrap();
    assert_eq!(a.view, View::Map);
    navigation::go_back(&mut a).unwrap();
    assert_eq!(a.view.label(), "Results");
    assert_eq!(a.first_use.selected_result, Some(run.id));
}

#[test]
fn same_name_sequence_rows_open_by_identity_and_preserve_edits() {
    let mut a = app();
    a.doc.name = "same name".into();
    let first = a.doc.id;
    let second = Document::new("same name".into(), b"CCCCGGGG".to_vec(), Topology::Circular);
    let second_id = second.id;
    a.library.push(appcore::state::OpenDoc {
        doc: second,
        path: None,
        dirty: false,
    });
    a.mutate("draft edit", vec![], |d| d.name = "same name".into());
    let rows = appcore::workspace::sequences(&a, "same name");
    assert_eq!(rows.len(), 2);
    assert_ne!(rows[0].id, rows[1].id);
    assert!(rows.iter().all(|r| r.persistence() != "Saved locally"));
    appcore::workspace::open_sequence(&mut a, second_id).unwrap();
    assert_eq!(a.doc.id, second_id);
    appcore::workspace::open_sequence(&mut a, first).unwrap();
    assert!(a.dirty);
}
#[test]
fn global_results_navigation_does_not_reopen_a_hidden_preview() {
    let mut a = app();
    let run = result(&mut a);
    let id = run.id;
    results::open_preview(&mut a, run).unwrap();
    appcore::workspace::enter(&mut a, View::Home).unwrap();
    assert_eq!(a.view, View::Home);
    appcore::workspace::enter(&mut a, View::Results).unwrap();
    assert!(a.results.list);
    assert!(a.results.pending.is_none());
    results::close(&mut a).unwrap();
    assert_eq!(a.view, View::Home);
    assert_eq!(a.results.pending.as_ref().unwrap().id, id);
}
#[test]
fn global_navigation_is_blocked_by_an_unfinished_form() {
    let mut a = app();
    appcore::commands::execute(&mut a, appcore::commands::CommandId::NewEntry).unwrap();
    assert!(a.dialog.is_some());
    assert!(appcore::workspace::enter(&mut a, View::Sequences).is_err());
    assert!(a.dialog.is_some());
}
#[test]
fn experiments_creation_does_not_create_a_sequence() {
    let mut a = app();
    let source = a.doc.clone();
    appcore::workspace::enter(&mut a, View::Notebook).unwrap();
    appcore::commands::execute(&mut a, appcore::commands::CommandId::NewEntry).unwrap();
    assert_eq!(a.doc, source);
    assert!(a.dialog.is_some());
    assert_eq!(View::Notebook.label(), "Experiments");
    assert_eq!(appcore::workspace::owner(View::Split), View::Sequences);
}

#[test]
fn catalog_pages_do_not_repeat_the_object_inspector() {
    let state = appcore::shell::ShellState::default();
    for view in [View::Home, View::Sequences, View::Results, View::Activity] {
        assert_eq!(
            state.layout_for(view, 1600.0).inspector,
            appcore::shell::InspectorPlacement::Hidden
        );
    }
    assert_eq!(
        state.layout_for(View::Map, 1600.0).inspector,
        appcore::shell::InspectorPlacement::Docked
    );
}

#[test]
fn result_titles_identify_real_digest_settings() {
    let mut a = app();
    let run = result(&mut a);
    let title = results::display_title(&run);
    assert!(title.contains("EcoRI"));
    assert!(title.to_lowercase().contains("digest"));
    assert!(!title.contains(&run.id.to_string()));
}

#[test]
fn repeated_results_selection_returns_to_the_original_page_once() {
    let mut a = app();
    a.view = View::Primers;
    results::browse(&mut a).unwrap();
    results::browse(&mut a).unwrap();
    appcore::workspace::enter(&mut a, View::Results).unwrap();
    results::close(&mut a).unwrap();
    assert_eq!(a.view, View::Primers);
}

#[test]
fn sequence_search_matches_names_and_paths_without_changing_the_document() {
    let mut a = app();
    a.path = Some(std::path::PathBuf::from("C:/research/Construct_A.gb"));
    let before = a.doc.clone();
    assert_eq!(
        appcore::workspace::sequences(&a, "  CONSTRUCT_a  ").len(),
        1
    );
    assert_eq!(appcore::workspace::sequences(&a, "TEMPLATE").len(), 1);
    assert!(appcore::workspace::sequences(&a, "missing sequence").is_empty());
    assert_eq!(a.doc, before);
}

#[test]
fn a_stale_sequence_row_cannot_open_a_different_document() {
    let mut a = app();
    let before = a.doc.clone();
    assert!(appcore::workspace::open_sequence(&mut a, domain::DocumentId::new()).is_err());
    assert_eq!(a.doc, before);
    assert_eq!(a.view, View::Map);
}

#[test]
fn completed_unsaved_output_can_be_resumed_after_visiting_home_and_catalog() {
    let mut a = app();
    let run = result(&mut a);
    results::open_preview(&mut a, run.clone()).unwrap();
    appcore::workspace::enter(&mut a, View::Home).unwrap();
    appcore::workspace::enter(&mut a, View::Results).unwrap();
    assert_eq!(results::unsaved_previews(&a), vec![&run]);
    results::resume_preview(&mut a, run.id).unwrap();
    assert_eq!(a.results.pending.as_deref(), Some(&run));
    results::close(&mut a).unwrap();
    assert_eq!(a.view, View::Results);
    assert!(a.results.list);
    results::resume_preview(&mut a, run.id).unwrap();
    results::save_analysis(&mut a, &run).unwrap();
    assert!(results::unsaved_previews(&a).is_empty());
}

#[test]
fn resuming_a_missing_preview_leaves_the_current_task_unchanged() {
    let mut a = app();
    let before = a.doc.clone();
    assert!(results::resume_preview(&mut a, domain::ResultId::new()).is_err());
    assert_eq!(a.doc, before);
    assert_eq!(a.view, View::Map);
}
