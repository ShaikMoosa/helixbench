use appcore::{
    first_use as first,
    state::{AppState, Dialog, ProjectsDraft, View},
};
use domain::{Document, RecordRef, StoreKind, Topology};
use std::{fs, path::PathBuf};
use storage::config::{Config, RecentTarget};
fn root() -> PathBuf {
    let p = std::env::temp_dir().join(format!("helixbench-p06-{}", domain::OperationId::new()));
    fs::create_dir_all(&p).unwrap();
    p
}
fn app() -> AppState {
    AppState::new_in(
        Document::new("personal".into(), b"ATGCGG".to_vec(), Topology::Linear),
        Config::default(),
        root(),
    )
}
#[test]
fn existing_workspace_not_replaced_by_demo() {
    let root = root();
    let mut records = storage::records::Records::default();
    records.workspace.add("real project", None);
    storage::records::save_to(&records, &root.join("records.json")).unwrap();
    let bytes = fs::read(root.join("records.json")).unwrap();
    let app = first::startup_in(Config::default(), root.clone());
    assert_eq!(app.view, View::Home);
    assert!(app.doc.sequence.is_empty());
    assert_eq!(app.records.workspace.projects[0].name, "real project");
    assert_eq!(fs::read(root.join("records.json")).unwrap(), bytes);
}
#[test]
fn sample_copy_is_personal_and_independent() {
    let mut app = app();
    app.mutate("rename", vec![], |d| d.name = "unsaved personal".into());
    let personal = app.doc.clone();
    first::try_sample(&mut app).unwrap();
    let sample = app.doc.clone();
    assert!(first::is_sample(&app));
    app.mutate("sample edit", vec![], |d| d.name = "my sample work".into());
    first::request_personal_copy(&mut app).unwrap();
    first::confirm_personal_copy(&mut app, "personal copy".into()).unwrap();
    assert!(!first::is_sample(&app));
    assert_ne!(app.doc.id, sample.id);
    assert_ne!(app.doc.revision_id, sample.revision_id);
    assert_eq!(app.doc.sequence, sample.sequence);
    assert_eq!(app.doc.name, "personal copy");
    assert!(app.dirty);
    assert!(app.path.is_none());
    assert!(app.library.iter().any(|d| d.doc == personal && d.dirty));
}
#[test]
fn resume_opens_exact_entry_task_or_result() {
    let mut app = app();
    app.records
        .entries
        .push(domain::notebook::Entry::new(8, "same title".into(), 1));
    let task = app.workflows.add_task("same title", "test", "me", 1);
    let entry = RecentTarget::Record(RecordRef {
        store: StoreKind::Notebook,
        local_id: 8,
    });
    first::resume(&mut app, entry).unwrap();
    assert_eq!(app.view, View::Notebook);
    assert_eq!(
        app.workspace_search.inspected,
        Some(appcore::workspace_search::SearchTarget::Record(RecordRef {
            store: StoreKind::Notebook,
            local_id: 8
        }))
    );
    first::resume(
        &mut app,
        RecentTarget::Record(RecordRef {
            store: StoreKind::Workflow,
            local_id: task,
        }),
    )
    .unwrap();
    assert_eq!(app.view, View::Workflows);
    let id = domain::ResultId::new();
    let result = domain::results::AnalysisResult {
        id,
        operation: domain::OperationId::new(),
        tool: "digest".into(),
        method_version: "fixture-v1".into(),
        created: 1,
        inputs: vec![domain::results::InputSnapshot::capture(&app.doc, None).unwrap()],
        parameters: Default::default(),
        output: Default::default(),
    };
    storage::results::save_to(
        &storage::results::ResultCatalog {
            results: vec![result],
        },
        &app.profile_path(storage::results::path()),
    )
    .unwrap();
    first::resume(&mut app, RecentTarget::Result(id)).unwrap();
    assert_eq!(app.first_use.selected_result, Some(id));
    assert_eq!(app.view, View::Home);
}
#[test]
fn sample_store_writes_cannot_touch_personal_records() {
    let mut app = app();
    let path = app.profile_path(storage::records::path());
    appcore::actions::add_project(
        &mut app,
        &ProjectsDraft {
            name: "real project".into(),
            ..Default::default()
        },
    );
    let before = fs::read(&path).unwrap();
    first::try_sample(&mut app).unwrap();
    assert!(app.records.workspace.projects.is_empty());
    appcore::actions::add_project(
        &mut app,
        &ProjectsDraft {
            name: "sample project".into(),
            ..Default::default()
        },
    );
    assert_eq!(fs::read(path).unwrap(), before);
    assert!(app.profile_path(storage::records::path()).exists());
    first::leave_sample(&mut app, true).unwrap();
    assert_eq!(app.records.workspace.projects[0].name, "real project");
}
#[test]
fn leaving_sample_preserves_personal_dirty_history() {
    let mut app = app();
    app.mutate("edit", vec![], |d| d.name = "edited".into());
    let id = app.doc.id;
    first::try_sample(&mut app).unwrap();
    first::leave_sample(&mut app, true).unwrap();
    assert_eq!(app.doc.id, id);
    assert!(app.dirty);
    assert!(app.undo());
    assert_eq!(app.doc.name, "personal");
}
#[test]
fn missing_resume_target_never_opens_demo_or_new_record() {
    let mut app = app();
    let original = app.doc.clone();
    assert!(first::resume(
        &mut app,
        RecentTarget::Record(RecordRef {
            store: StoreKind::Notebook,
            local_id: 999
        })
    )
    .is_err());
    assert_eq!(app.doc, original);
    assert!(app.dialog.is_none());
    assert!(app.records.entries.is_empty());
}
#[test]
fn native_resume_rejects_wrong_document_at_same_path() {
    let mut app = app();
    let path = root().join("moved.hbx");
    let other = app.doc.independent_copy("other".into());
    storage::save_document(&other, &path).unwrap();
    let id = domain::DocumentId::new();
    assert!(first::resume(
        &mut app,
        RecentTarget::Document {
            document: id,
            path: Some(path)
        }
    )
    .is_err());
    assert_eq!(app.doc.name, "personal");
}
#[test]
fn closing_sample_cannot_skip_suspended_personal_unsaved_guard() {
    let mut app = app();
    app.mutate("edit", vec![], |d| d.name = "unsaved".into());
    first::try_sample(&mut app).unwrap();
    appcore::navigation::request_navigation(
        &mut app,
        appcore::navigation::NavigationIntent::ExitApplication,
    )
    .unwrap();
    assert!(matches!(app.dialog, Some(Dialog::SampleExit)));
    first::leave_sample(&mut app, true).unwrap();
    assert!(!first::is_sample(&app));
    assert!(app.navigation.pending.is_some());
    assert!(!app.exit_requested);
}
#[test]
fn sample_save_requires_explicit_personal_copy() {
    let mut app = app();
    first::try_sample(&mut app).unwrap();
    let root = root();
    let path = root.join("sample.hbx");
    let journal = storage::operations::Journal::new(root.join("journal"));
    assert!(!appcore::operations::save_to(
        &mut app,
        &path,
        &journal,
        &root.join("config.json")
    ));
    assert!(!path.exists());
}
#[test]
fn guidance_preference_survives_restart() {
    let mut app = app();
    let path = app.profile_path(storage::config::config_path());
    first::dismiss_guidance(&mut app).unwrap();
    assert!(storage::config::load_from(&path).first_use_dismissed);
}

#[test]
fn failed_forget_preserves_all_recent_links() {
    let mut app = app();
    let file = root().join("work.hbx");
    app.config.add_recent(&file);
    let target = RecentTarget::File(file.clone());
    app.config.remember_work(storage::config::RecentWork {
        target: target.clone(),
        title: "work".into(),
    });
    fs::create_dir(app.profile_path(storage::config::config_path())).unwrap();
    assert!(first::forget(&mut app, &target).is_err());
    assert_eq!(app.config.recent_files, vec![file]);
    assert_eq!(app.config.recent_work.len(), 1);
}
#[test]
fn failed_preference_write_does_not_hide_guidance() {
    let mut app = app();
    fs::create_dir(app.profile_path(storage::config::config_path())).unwrap();
    assert!(first::dismiss_guidance(&mut app).is_err());
    assert!(!app.config.first_use_dismissed);
}
#[test]
fn copy_review_rejects_newer_checkpoint_even_at_same_revision() {
    let mut app = app();
    first::try_sample(&mut app).unwrap();
    first::request_personal_copy(&mut app).unwrap();
    let source = app.doc.source_ref();
    app.doc.save_version("later".into(), 1);
    assert_eq!(app.doc.source_ref(), source);
    assert!(first::confirm_personal_copy(&mut app, "copy".into()).is_err());
    assert!(first::is_sample(&app));
}
#[test]
fn resume_waits_for_dirty_guard_then_opens_validated_document() {
    let mut app = app();
    app.mutate("edit", vec![], |d| d.name = "dirty personal".into());
    let previous = app.doc.id;
    let doc = Document::new("saved target".into(), b"CCCC".to_vec(), Topology::Linear);
    let path = root().join("saved.hbx");
    storage::save_document(&doc, &path).unwrap();
    first::resume(
        &mut app,
        RecentTarget::Document {
            document: doc.id,
            path: Some(path),
        },
    )
    .unwrap();
    assert_eq!(app.doc.id, previous);
    assert!(app.navigation.pending.is_some());
    appcore::navigation::resolve_guard(&mut app, appcore::navigation::GuardDecision::Discard)
        .unwrap();
    assert_eq!(app.doc.id, doc.id);
}
#[test]
fn newer_saved_result_catalog_is_preserved_and_not_opened() {
    let mut app = app();
    let path = app.profile_path(storage::results::path());
    let bytes = br#"{"version":999,"catalog":{}}"#;
    fs::write(&path, bytes).unwrap();
    assert!(first::resume(&mut app, RecentTarget::Result(domain::ResultId::new())).is_err());
    assert!(app.first_use.selected_result.is_none());
    assert_eq!(fs::read(&path).unwrap(), bytes);
}
#[test]
fn recent_document_save_as_updates_path_without_duplicate_identity() {
    let mut config = Config::default();
    let id = domain::DocumentId::new();
    for name in ["old.hbx", "new.hbx"] {
        config.remember_work(storage::config::RecentWork {
            target: RecentTarget::Document {
                document: id,
                path: Some(PathBuf::from(name)),
            },
            title: name.into(),
        });
    }
    assert_eq!(config.recent_work.len(), 1);
    assert_eq!(config.recent_work[0].title, "new.hbx");
}

#[test]
fn sample_cached_inputs_restore_personal_values() {
    let mut session = first::ProfileUiSession::default();
    let mut values = std::collections::HashMap::from([
        ("workspace:query", "personal search"),
        ("iv:dose", "12"),
        ("iv:conc", "7"),
    ]);
    session.sync(true, &mut values);
    assert!(values.is_empty());
    values.insert("workspace:query", "sample search");
    values.insert("iv:dose", "99");
    values.insert("iv:conc", "100");
    session.sync(false, &mut values);
    assert_eq!(values["workspace:query"], "personal search");
    assert_eq!(values["iv:dose"], "12");
    assert_eq!(values["iv:conc"], "7");
    session.sync(true, &mut values);
    assert!(values.is_empty());
    session.sync(false, &mut values);
    assert_eq!(values["iv:dose"], "12");
}
