use appcore::{
    commands::{self, Availability, CommandId},
    state::{AppState, Dialog, OpenDoc, View},
    workspace_search::{self as search, SearchQuery, SearchScope, SearchTarget},
};
use domain::{Document, Range, RecordRef, StoreKind, Topology};

fn app() -> AppState {
    AppState::new(
        Document::new("alpha".into(), b"ATGCATGC".to_vec(), Topology::Linear),
        storage::config::Config::default(),
    )
}
fn query(text: &str, scope: SearchScope) -> SearchQuery {
    SearchQuery {
        text: text.into(),
        scope,
        include_archived: false,
    }
}

#[test]
fn owning_page_tool_entry_within_two_clicks() {
    let mut app = app();
    for descriptor in commands::tools() {
        app.view = descriptor.owner;
        assert!(commands::owning_page(app.view)
            .iter()
            .any(|d| d.id == descriptor.id));
    }
    app.view = View::Sequence;
    commands::execute(&mut app, CommandId::Pcr).unwrap();
    assert!(matches!(app.dialog, Some(Dialog::Pcr(_))));
}
#[test]
fn search_back_restores_scope() {
    let mut app = app();
    app.view = View::Sequence;
    app.selection = Some(Range::new(1, 3));
    app.search.query = "ATG".into();
    search::open(&mut app).unwrap();
    search::run(&mut app, query("alpha", SearchScope::Documents));
    let target = app.workspace_search.results[0].target;
    app.workspace_search.selected = Some(target);
    search::open_target(&mut app, target).unwrap();
    search::back_to_search(&mut app).unwrap();
    assert_eq!(
        app.workspace_search.query,
        query("alpha", SearchScope::Documents)
    );
    assert_eq!(app.workspace_search.selected, Some(target));
    assert_eq!(app.search.query, "ATG");
    search::close(&mut app).unwrap();
    assert_eq!(app.selection, Some(Range::new(1, 3)));
    assert_eq!(app.view, View::Sequence);
}
#[test]
fn late_search_reply_is_ignored() {
    let mut app = app();
    search::open(&mut app).unwrap();
    let old = app
        .workspace_search
        .begin(query("alpha", SearchScope::Documents));
    let fresh = app
        .workspace_search
        .begin(query("other", SearchScope::Registry));
    assert!(!app.workspace_search.accept(&old, vec![]));
    assert!(app.workspace_search.accept(&fresh, vec![]));
    search::close(&mut app).unwrap();
    assert!(!app.workspace_search.accept(&fresh, vec![]));
}
#[test]
fn deleted_record_is_not_reopened() {
    let mut app = app();
    app.records
        .entries
        .push(domain::notebook::Entry::new(7, "experiment".into(), 1));
    search::open(&mut app).unwrap();
    search::run(&mut app, query("experiment", SearchScope::Notebook));
    let target = app.workspace_search.results[0].target;
    app.records.entries.clear();
    assert!(search::open_target(&mut app, target).is_err());
    assert!(app.workspace_search.open);
    assert!(app.dialog.is_none());
    assert!(app.records.entries.is_empty());
}
#[test]
fn duplicate_names_route_by_document_id() {
    let mut app = app();
    let doc = app.doc.independent_copy("alpha".into());
    let id = doc.id;
    app.library.push(OpenDoc {
        doc,
        path: None,
        dirty: true,
    });
    search::open(&mut app).unwrap();
    search::run(&mut app, query("alpha", SearchScope::Documents));
    assert_eq!(app.workspace_search.results.len(), 2);
    search::open_target(&mut app, SearchTarget::Document(id)).unwrap();
    assert_eq!(app.doc.id, id);
}
#[test]
fn disabled_command_explains_prerequisite_and_does_not_open() {
    let mut app = app();
    assert!(matches!(
        commands::availability(&app, CommandId::Assembly),
        Availability::Disabled(_)
    ));
    assert!(commands::execute(&mut app, CommandId::Assembly).is_err());
    assert!(app.dialog.is_none());
    assert!(matches!(
        commands::availability(&app, CommandId::Rotate),
        Availability::Disabled(_)
    ));
}
#[test]
fn commands_preserve_existing_form() {
    let mut app = app();
    commands::execute(&mut app, CommandId::Pcr).unwrap();
    assert!(commands::execute(&mut app, CommandId::Digest).is_err());
    assert!(search::open(&mut app).is_err());
    assert!(matches!(app.dialog, Some(Dialog::Pcr(_))));
}
#[test]
fn record_store_identity_is_not_just_local_id() {
    let mut app = app();
    app.records
        .entries
        .push(domain::notebook::Entry::new(1, "entry".into(), 1));
    search::open(&mut app).unwrap();
    let wrong = SearchTarget::Record(RecordRef {
        store: StoreKind::Registry,
        local_id: 1,
    });
    assert!(search::open_target(&mut app, wrong).is_err());
    assert!(app.workspace_search.open);
}
#[test]
fn search_scopes_and_archived_are_explicit() {
    let mut app = app();
    let mut entry = domain::notebook::Entry::new(1, "alpha".into(), 1);
    entry.status = domain::notebook::EntryStatus::Archived;
    app.records.entries.push(entry);
    search::open(&mut app).unwrap();
    search::run(&mut app, query("alpha", SearchScope::Notebook));
    assert!(app.workspace_search.results.is_empty());
    let mut q = query("alpha", SearchScope::Notebook);
    q.include_archived = true;
    search::run(&mut app, q);
    assert_eq!(app.workspace_search.results.len(), 1);
}
#[test]
fn opening_record_is_read_only_and_back_preserves_search() {
    let mut app = app();
    app.records
        .entries
        .push(domain::notebook::Entry::new(1, "experiment".into(), 1));
    let original = app.records.clone();
    search::open(&mut app).unwrap();
    search::run(&mut app, query("experiment", SearchScope::Notebook));
    let target = app.workspace_search.results[0].target;
    search::open_target(&mut app, target).unwrap();
    assert_eq!(app.view, View::Notebook);
    assert_eq!(app.workspace_search.inspected, Some(target));
    assert_eq!(app.records, original);
    assert!(app.dialog.is_none());
    search::back_to_search(&mut app).unwrap();
    assert_eq!(app.workspace_search.query.text, "experiment");
}

#[test]
fn hidden_search_result_cannot_execute_after_close() {
    let mut app = app();
    search::open(&mut app).unwrap();
    search::close(&mut app).unwrap();
    assert!(search::open_target(&mut app, SearchTarget::Command(CommandId::Pcr)).is_err());
    assert!(app.dialog.is_none());
}
#[test]
fn closed_origin_does_not_trap_user_in_search() {
    let mut app = app();
    let original = app.doc.id;
    let other = app.doc.independent_copy("other".into());
    let other_id = other.id;
    app.library.push(OpenDoc {
        doc: other,
        path: None,
        dirty: false,
    });
    search::open(&mut app).unwrap();
    search::open_target(&mut app, SearchTarget::Document(other_id)).unwrap();
    app.library.retain(|d| d.doc.id != original);
    search::back_to_search(&mut app).unwrap();
    search::close(&mut app).unwrap();
    assert!(!app.workspace_search.open);
    assert_eq!(app.doc.id, other_id);
    assert!(app.toast.as_ref().unwrap().0.contains("no longer open"));
}
#[test]
fn saved_query_cannot_replace_newly_opened_editor() {
    let mut app = app();
    app.records
        .entries
        .push(domain::notebook::Entry::new(1, "entry".into(), 1));
    search::open(&mut app).unwrap();
    commands::execute(&mut app, CommandId::Pcr).unwrap();
    assert!(search::open_target(
        &mut app,
        SearchTarget::Record(RecordRef {
            store: StoreKind::Notebook,
            local_id: 1
        })
    )
    .is_err());
    assert!(matches!(app.dialog, Some(Dialog::Pcr(_))));
}
#[test]
fn record_removed_after_inspection_cannot_create_new_editor() {
    let mut app = app();
    app.records
        .entries
        .push(domain::notebook::Entry::new(1, "entry".into(), 1));
    search::open(&mut app).unwrap();
    search::open_target(
        &mut app,
        SearchTarget::Record(RecordRef {
            store: StoreKind::Notebook,
            local_id: 1,
        }),
    )
    .unwrap();
    app.records.entries.clear();
    assert!(search::edit_inspected(&mut app).is_err());
    assert!(app.dialog.is_none());
}
#[test]
fn command_availability_is_rechecked_at_activation() {
    let mut app = app();
    app.doc.set_topology(Topology::Circular);
    search::open(&mut app).unwrap();
    search::run(&mut app, query("rotate", SearchScope::Tools));
    app.doc.set_topology(Topology::Linear);
    assert!(search::open_target(&mut app, SearchTarget::Command(CommandId::Rotate)).is_err());
    assert!(app.dialog.is_none());
}

#[test]
fn page_navigation_leaves_search_and_keeps_return_query() {
    let mut app = app();
    search::open(&mut app).unwrap();
    search::run(&mut app, query("alpha", SearchScope::Documents));
    let doc_id = app.doc.id;
    appcore::navigation::request_navigation(
        &mut app,
        appcore::navigation::NavigationIntent::Navigate(appcore::navigation::Route {
            view: View::Inventory,
            document: doc_id,
            record: None,
        }),
    )
    .unwrap();
    assert!(!app.workspace_search.open);
    assert!(app.workspace_search.can_return);
    assert_eq!(app.view, View::Inventory);
    search::back_to_search(&mut app).unwrap();
    assert_eq!(
        app.workspace_search.query,
        query("alpha", SearchScope::Documents)
    );
}

#[test]
fn workspace_search_cannot_edit_hidden_sequence() {
    let mut app = app();
    app.mutate("rename", vec![], |doc| doc.name = "edited".into());
    app.selection = Some(Range::new(1, 3));
    let original = app.doc.clone();
    let selection = app.selection;
    let history = app.history.len();
    search::open(&mut app).unwrap();
    assert!(appcore::actions::cut_selection(&mut app).is_none());
    appcore::actions::delete_selection(&mut app);
    appcore::actions::insert_bases(&mut app, "GGG", true);
    assert!(!app.undo());
    assert!(!app.redo());
    assert_eq!(app.doc, original);
    assert_eq!(app.selection, selection);
    assert_eq!(app.history.len(), history);
}

#[test]
fn locked_notebook_result_stays_read_only() {
    let mut app = app();
    let mut entry = domain::notebook::Entry::new(1, "approved entry".into(), 1);
    entry.status = domain::notebook::EntryStatus::Approved;
    app.records.entries.push(entry);
    search::open(&mut app).unwrap();
    search::open_target(
        &mut app,
        SearchTarget::Record(RecordRef {
            store: StoreKind::Notebook,
            local_id: 1,
        }),
    )
    .unwrap();
    assert!(search::edit_inspected(&mut app).is_err());
    assert!(app.dialog.is_none());
}

#[test]
fn reopening_loaded_native_document_leaves_search() {
    let mut app = app();
    let path =
        std::env::temp_dir().join(format!("helixbench-p05-{}.hbx", domain::OperationId::new()));
    storage::save_document(&app.doc, &path).unwrap();
    search::open(&mut app).unwrap();
    appcore::navigation::request_navigation(
        &mut app,
        appcore::navigation::NavigationIntent::OpenFile(path),
    )
    .unwrap();
    assert!(!app.workspace_search.open);
    assert!(app.workspace_search.can_return);
}
#[test]
fn confirming_import_leaves_search_and_exposes_imported_document() {
    let mut app = app();
    search::open(&mut app).unwrap();
    appcore::import::begin_review(
        &mut app,
        appcore::import::ImportSource::text("input.fa", ">new\nGGCC\n"),
        appcore::import::ImportDestination::NewDocuments,
    )
    .unwrap();
    let plan = match &app.dialog {
        Some(Dialog::ImportReview(draft)) => appcore::import::reviewed_plan(draft).unwrap(),
        _ => panic!("expected review"),
    };
    appcore::import::commit_import(&mut app, &plan).unwrap();
    assert!(!app.workspace_search.open);
    assert!(app.workspace_search.can_return);
    assert_eq!(app.view, View::Map);
    assert_eq!(app.doc.name, "new");
}
#[test]
fn invalid_or_canceled_import_keeps_search_visible() {
    let mut app = app();
    search::open(&mut app).unwrap();
    appcore::import::begin_review(
        &mut app,
        appcore::import::ImportSource::text("input.fa", ">bad\nAT?G\n"),
        appcore::import::ImportDestination::NewDocuments,
    )
    .unwrap();
    match &app.dialog {
        Some(Dialog::ImportReview(draft)) => {
            assert!(appcore::import::reviewed_plan(draft).is_err())
        }
        _ => panic!("expected review"),
    };
    assert!(app.workspace_search.open);
    appcore::import::cancel_import(&mut app).unwrap();
    assert!(app.workspace_search.open);
    assert_eq!(app.doc.name, "alpha");
}
