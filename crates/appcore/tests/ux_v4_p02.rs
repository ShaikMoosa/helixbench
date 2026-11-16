use appcore::navigation::{self, GuardDecision, NavigationIntent, NavigationOutcome, Route};
use appcore::{
    actions,
    state::{AppState, Dialog, OpenDoc, PendingAction, View},
};
use domain::{Document, Range, Topology};
use storage::config::Config;

fn app() -> AppState {
    AppState::new(
        Document::new(
            "same name".into(),
            b"ATGCGTACGTAA".to_vec(),
            Topology::Linear,
        ),
        Config::default(),
    )
}
fn add_other(app: &mut AppState) -> domain::DocumentId {
    let doc = Document::new("same name".into(), b"CCCCGGGG".to_vec(), Topology::Linear);
    let id = doc.id;
    app.library.push(OpenDoc {
        doc,
        path: None,
        dirty: false,
    });
    id
}

#[test]
fn switching_documents_retains_undo_selection_and_query() {
    let mut app = app();
    let original = app.doc.id;
    add_other(&mut app);
    app.mutate("rename", vec![], |doc| doc.name = "edited".into());
    app.view = View::Sequence;
    app.selection = Some(Range::new(2, 4));
    app.zoom_start = 2;
    app.zoom_width = 6;
    app.search.query = "GTAC".into();
    actions::switch_doc(&mut app, 0);
    let index = app
        .library
        .iter()
        .position(|doc| doc.doc.id == original)
        .unwrap();
    actions::switch_doc(&mut app, index);
    assert_eq!(app.selection, Some(Range::new(2, 4)));
    assert_eq!(app.search.query, "GTAC");
    assert_eq!(app.view, View::Sequence);
    assert_eq!(app.zoom_start, 2);
    assert_eq!(app.zoom_width, 6);
    assert!(app.undo());
    assert_eq!(app.doc.name, "same name");
}

#[test]
fn second_intent_cannot_replace_unanswered_guard() {
    let mut app = app();
    app.mutate("edit", vec![], |doc| doc.name.push('x'));
    actions::guard(&mut app, PendingAction::NewEmpty);
    actions::guard(&mut app, PendingAction::NewDemo);
    assert!(matches!(
        app.dialog,
        Some(Dialog::Unsaved {
            action: PendingAction::NewEmpty
        })
    ));
}

#[test]
fn same_name_legacy_link_is_reported_as_ambiguous() {
    let mut app = app();
    add_other(&mut app);
    let id = app
        .registry
        .add(domain::registry::Kind::Plasmid, Default::default(), 1);
    app.registry.link(id, Some("same name".into()), 1);
    actions::open_linked_entry(&mut app, id);
    assert!(app
        .toast
        .as_ref()
        .is_some_and(|(text, _)| text.to_lowercase().contains("ambiguous")));
}

#[test]
fn guard_save_failure_retains_intent() {
    let mut app = app();
    let path = std::env::temp_dir().join(format!("hbx-p02-save-fail-{}", std::process::id()));
    std::fs::create_dir_all(&path).unwrap();
    app.path = Some(path.clone());
    app.mutate("edit", vec![], |doc| doc.name.push('x'));
    let source = app.doc.source_ref();
    navigation::request_navigation(&mut app, NavigationIntent::NewEmpty).unwrap();
    assert_eq!(
        navigation::resolve_guard(&mut app, GuardDecision::Save).unwrap(),
        NavigationOutcome::AwaitingDecision
    );
    assert!(app.dirty);
    assert_eq!(
        app.navigation.pending.as_ref().unwrap().intent,
        NavigationIntent::NewEmpty
    );
    assert_eq!(app.doc.source_ref(), source);
    assert!(matches!(app.dialog, Some(Dialog::Unsaved { .. })));
    navigation::resolve_guard(&mut app, GuardDecision::Stay).unwrap();
    std::fs::remove_dir_all(path).unwrap();
}

#[test]
fn stay_preserves_invoking_view_and_read_only_dialog() {
    let mut app = app();
    app.view = View::Sequence;
    app.selection = Some(Range::new(1, 3));
    app.search.query = "ATGC".into();
    app.dialog = Some(Dialog::About);
    app.mutate("edit", vec![], |doc| doc.name.push('x'));
    navigation::request_navigation(&mut app, NavigationIntent::NewEmpty).unwrap();
    assert_eq!(
        navigation::resolve_guard(&mut app, GuardDecision::Stay).unwrap(),
        NavigationOutcome::Canceled
    );
    assert_eq!(app.view, View::Sequence);
    assert_eq!(app.selection, Some(Range::new(1, 3)));
    assert_eq!(app.search.query, "ATGC");
    assert!(matches!(app.dialog, Some(Dialog::About)));
    assert!(app.navigation.pending.is_none());
    assert!(app.dirty);
}

#[test]
fn view_switch_retains_draft() {
    let mut app = app();
    let id = app.doc.id;
    app.dialog = Some(Dialog::NewSequence(appcore::state::NewSeqState {
        name: "draft".into(),
        text: "ATGC".into(),
        topology: Topology::Linear,
        error: None,
    }));
    app.mutate("edit", vec![], |doc| doc.name.push('x'));
    navigation::request_navigation(
        &mut app,
        NavigationIntent::Navigate(Route {
            view: View::Primers,
            document: id,
            record: None,
        }),
    )
    .unwrap();
    assert!(app.navigation.pending.is_none());
    assert!(matches!(&app.dialog, Some(Dialog::NewSequence(draft)) if draft.text == "ATGC"));
    assert!(app.dirty);
    navigation::go_back(&mut app).unwrap();
    assert_eq!(app.view, View::Map);
}

#[test]
fn same_name_documents_do_not_swap() {
    let mut app = app();
    let original = app.doc.id;
    let target = add_other(&mut app);
    add_other(&mut app);
    app.library.reverse();
    navigation::request_navigation(&mut app, NavigationIntent::SwitchDocument(target)).unwrap();
    assert_eq!(app.doc.id, target);
    navigation::go_back(&mut app).unwrap();
    assert_eq!(app.doc.id, original);
}

#[test]
fn discard_restores_saved_source_before_opening_new_form() {
    let mut app = app();
    let original = app.doc.clone();
    app.mutate("edit", vec![], |doc| doc.name = "discard me".into());
    navigation::request_navigation(&mut app, NavigationIntent::NewEmpty).unwrap();
    navigation::resolve_guard(&mut app, GuardDecision::Discard).unwrap();
    assert_eq!(app.doc, original);
    assert!(!app.dirty);
    assert!(matches!(app.dialog, Some(Dialog::NewSequence(_))));
}

#[test]
fn closing_never_saved_document_does_not_leave_it_open() {
    let mut app = app();
    actions::create_from_dialog(
        &mut app,
        &appcore::state::NewSeqState {
            name: "draft".into(),
            text: "ATGC".into(),
            topology: Topology::Linear,
            error: None,
        },
    )
    .unwrap();
    let id = app.doc.id;
    let target = add_other(&mut app);
    navigation::request_navigation(&mut app, NavigationIntent::CloseDocument(id)).unwrap();
    navigation::resolve_guard(&mut app, GuardDecision::Discard).unwrap();
    assert_ne!(app.doc.id, id);
    assert_eq!(app.doc.id, target);
    assert!(app.library.is_empty());
    assert!(!app.library.iter().any(|doc| doc.doc.id == id));
}

#[test]
fn library_bulk_edit_cannot_restore_an_obsolete_undo_stack() {
    let mut app = app();
    let source = app.doc.id;
    add_other(&mut app);
    app.mutate("rename", vec![], |doc| doc.name = "edited".into());
    let other = app.library[0].doc.id;
    navigation::request_navigation(&mut app, NavigationIntent::SwitchDocument(other)).unwrap();
    actions::bulk_tag(
        &mut app,
        &appcore::state::WorkspaceDraft {
            selected: vec![false, true],
            tag: "external".into(),
            error: None,
        },
    );
    navigation::request_navigation(&mut app, NavigationIntent::SwitchDocument(source)).unwrap();
    assert!(
        !app.undo(),
        "cached undo state predating an inactive document edit must be invalidated"
    );
    assert_eq!(app.doc.tags, vec!["external"]);
    assert!(app.dirty);
}

#[test]
fn exit_checks_dirty_library_and_stay_returns_to_origin() {
    let mut app = app();
    let origin = app.doc.id;
    let other = add_other(&mut app);
    app.library[0].dirty = true;
    app.view = View::Sequence;
    app.selection = Some(Range::new(1, 3));
    assert_eq!(
        navigation::request_navigation(&mut app, NavigationIntent::ExitApplication).unwrap(),
        NavigationOutcome::AwaitingDecision
    );
    assert_eq!(app.doc.id, other);
    assert!(!app.exit_requested);
    navigation::resolve_guard(&mut app, GuardDecision::Stay).unwrap();
    assert_eq!(app.doc.id, origin);
    assert_eq!(app.view, View::Sequence);
    assert_eq!(app.selection, Some(Range::new(1, 3)));
    assert!(app
        .library
        .iter()
        .any(|doc| doc.doc.id == other && doc.dirty));
}

#[test]
fn invalid_file_does_not_discard_dirty_input() {
    let mut app = app();
    app.mutate("edit", vec![], |doc| doc.name.push('x'));
    let source = app.doc.clone();
    let path = std::env::temp_dir().join("helixbench-p02-definitely-missing-file.hbx");
    assert!(navigation::request_navigation(&mut app, NavigationIntent::OpenFile(path)).is_err());
    assert_eq!(app.doc, source);
    assert!(app.dirty);
    assert!(app.navigation.pending.is_none());
}

#[test]
fn changed_guard_source_cannot_execute_destination() {
    let mut app = app();
    app.mutate("edit", vec![], |doc| doc.name.push('x'));
    navigation::request_navigation(&mut app, NavigationIntent::NewEmpty).unwrap();
    app.mutate("late edit", vec![], |doc| doc.name.push('y'));
    assert!(navigation::resolve_guard(&mut app, GuardDecision::Discard).is_err());
    assert!(app.doc.name.ends_with("xy"));
    assert!(app.navigation.pending.is_some());
    navigation::resolve_guard(&mut app, GuardDecision::Stay).unwrap();
}

#[test]
fn source_session_survives_creating_a_pcr_product() {
    let template = "ATGCGTACGATCGGATCCGAATTCGCTAGCTAACGGTACCAAGGTTCCGATCGATTAAGCGTACCGATTCGGAATC";
    let mut app = AppState::new(
        Document::new(
            "template".into(),
            template.as_bytes().to_vec(),
            Topology::Linear,
        ),
        Config::default(),
    );
    let original = app.doc.id;
    app.mutate("rename", vec![], |doc| doc.name = "edited template".into());
    app.selection = Some(Range::new(2, 8));
    let reverse = domain::sequence::reverse_complement(
        &template.as_bytes()[template.len() - 23..template.len() - 3],
    );
    actions::open_pcr(&mut app);
    let mut draft = match app.dialog.take().unwrap() {
        appcore::state::Dialog::Pcr(d) => d,
        _ => unreachable!(),
    };
    draft.fwd = template[..20].into();
    draft.rev = String::from_utf8(reverse).unwrap();
    appcore::primer_workflow::run_pcr(&app, &mut draft).unwrap();
    draft.product_pick = Some(0);
    actions::create_pcr_product(&mut app, &draft).unwrap();
    assert_ne!(app.doc.id, original);
    navigation::request_navigation(&mut app, NavigationIntent::SwitchDocument(original)).unwrap();
    assert_eq!(app.selection, Some(Range::new(2, 8)));
    assert!(app.undo());
    assert_eq!(app.doc.name, "template");
}

#[test]
fn dirty_edit_form_is_not_misrepresented_as_document_save() {
    let mut app = app();
    app.dialog = Some(Dialog::NewSequence(appcore::state::NewSeqState {
        name: "typed draft".into(),
        text: "ATGC".into(),
        topology: Topology::Linear,
        error: None,
    }));
    assert!(navigation::has_unsaved_work(&app));
    assert!(navigation::request_navigation(&mut app, NavigationIntent::ExitApplication).is_err());
    assert!(matches!(&app.dialog, Some(Dialog::NewSequence(draft)) if draft.name == "typed draft"));
    assert!(!app.exit_requested);
}

#[test]
fn save_runs_original_destination_only_after_successful_persistence() {
    let mut app = app();
    let path = std::env::temp_dir().join(format!("hbx-p02-save-ok-{}.hbx", std::process::id()));
    app.path = Some(path.clone());
    app.mutate("rename", vec![], |doc| doc.name = "persist this".into());
    let persisted = app.doc.clone();
    navigation::request_navigation(&mut app, NavigationIntent::NewEmpty).unwrap();
    assert_eq!(
        navigation::resolve_guard(&mut app, GuardDecision::Save).unwrap(),
        NavigationOutcome::Completed
    );
    assert_eq!(storage::load_document(&path).unwrap(), persisted);
    assert!(matches!(app.dialog, Some(Dialog::NewSequence(_))));
    assert!(app.navigation.pending.is_none());
    std::fs::remove_file(path).unwrap();
}

#[test]
fn hidden_unapplied_form_blocks_exit_and_target_close() {
    let mut app = app();
    let form_owner = app.doc.id;
    let other = add_other(&mut app);
    app.dialog = Some(Dialog::NewSequence(appcore::state::NewSeqState {
        name: "hidden draft".into(),
        text: "ATGC".into(),
        topology: Topology::Linear,
        error: None,
    }));
    navigation::request_navigation(&mut app, NavigationIntent::SwitchDocument(other)).unwrap();
    assert!(app.dialog.is_none());
    assert!(navigation::has_unsaved_work(&app));
    assert!(navigation::request_navigation(&mut app, NavigationIntent::ExitApplication).is_err());
    assert!(!app.exit_requested);
    assert_eq!(app.doc.id, form_owner);
    assert!(
        navigation::request_navigation(&mut app, NavigationIntent::CloseDocument(form_owner))
            .is_err()
    );
    navigation::request_navigation(&mut app, NavigationIntent::SwitchDocument(form_owner)).unwrap();
    assert!(
        matches!(&app.dialog, Some(Dialog::NewSequence(draft)) if draft.name == "hidden draft")
    );
}

#[test]
fn reopening_same_native_document_focuses_existing_draft() {
    let mut app = app();
    let path = std::env::temp_dir().join(format!("hbx-p02-reopen-{}.hbx", std::process::id()));
    storage::save_document(&app.doc, &path).unwrap();
    let id = app.doc.id;
    let other = add_other(&mut app);
    app.mutate("unsaved rename", vec![], |doc| {
        doc.name = "retained draft".into()
    });
    navigation::request_navigation(&mut app, NavigationIntent::SwitchDocument(other)).unwrap();
    navigation::request_navigation(&mut app, NavigationIntent::OpenFile(path.clone())).unwrap();
    assert_eq!(app.doc.id, id);
    assert_eq!(app.doc.name, "retained draft");
    assert!(app.dirty);
    assert!(!app.library.iter().any(|doc| doc.doc.id == id));
    std::fs::remove_file(path).unwrap();
}

#[test]
fn returning_to_search_preserves_the_selected_match() {
    let mut app = app();
    app.doc = Document::new("matches".into(), b"ATGCATGCATGC".to_vec(), Topology::Linear);
    let id = app.doc.id;
    app.search.open = true;
    app.search.query = "ATGC".into();
    app.run_search();
    assert!(app.search.hit_count() > 1);
    app.search.current = 1;
    app.goto_current_hit();
    let selection = app.selection;
    navigation::request_navigation(
        &mut app,
        NavigationIntent::Navigate(Route {
            view: View::Home,
            document: id,
            record: None,
        }),
    )
    .unwrap();
    navigation::go_back(&mut app).unwrap();
    assert_eq!(app.search.current, 1);
    assert_eq!(app.selection, selection);
}

#[test]
fn exit_does_not_complete_until_every_dirty_document_is_resolved() {
    let mut app = app();
    app.mutate("edit", vec![], |doc| doc.name.push('x'));
    let other = add_other(&mut app);
    app.library[0].dirty = true;
    navigation::request_navigation(&mut app, NavigationIntent::ExitApplication).unwrap();
    assert_eq!(
        navigation::resolve_guard(&mut app, GuardDecision::Discard).unwrap(),
        NavigationOutcome::AwaitingDecision
    );
    assert_eq!(app.doc.id, other);
    assert!(!app.exit_requested);
    assert_eq!(
        navigation::resolve_guard(&mut app, GuardDecision::Discard).unwrap(),
        NavigationOutcome::Completed
    );
    assert!(app.exit_requested);
    assert!(!app.dirty);
    assert!(app.library.iter().all(|doc| !doc.dirty));
}

#[test]
fn restored_context_signals_native_inputs_to_reseed() {
    let mut app = app();
    let id = app.doc.id;
    app.search.query = "ATGC".into();
    navigation::request_navigation(
        &mut app,
        NavigationIntent::Navigate(Route {
            view: View::Primers,
            document: id,
            record: None,
        }),
    )
    .unwrap();
    let before = app.navigation.context_epoch;
    app.search.query = "changed in next view".into();
    navigation::go_back(&mut app).unwrap();
    assert_eq!(app.search.query, "ATGC");
    assert_ne!(app.navigation.context_epoch, before);
}
