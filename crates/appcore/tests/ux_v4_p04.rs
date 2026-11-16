use appcore::{operations, state::AppState};
use domain::{operation::OperationState, Document, Topology};
use std::{fs, path::PathBuf};
use storage::operations::{Journal, RecoveryDisposition};

fn fixture() -> (AppState, PathBuf, Journal) {
    let root = std::env::temp_dir().join(format!("helixbench-p04-{}", domain::OperationId::new()));
    fs::create_dir_all(&root).unwrap();
    let journal = Journal::new(root.join("journal"));
    let app = AppState::new(
        Document::new("captured".into(), b"ATGC".to_vec(), Topology::Linear),
        storage::config::Config::default(),
    );
    (app, root, journal)
}

#[test]
fn save_rename_failure_is_not_success() {
    let (mut app, root, journal) = fixture();
    let target = root.join("directory.hbx");
    fs::create_dir(&target).unwrap();
    app.dirty = true;
    assert!(!operations::save_to(
        &mut app,
        &target,
        &journal,
        &root.join("config.json")
    ));
    assert!(app.dirty);
    assert!(app.operation_error.is_some());
    assert!(target.is_dir());
}

#[test]
fn operation_uses_captured_input() {
    let (mut app, root, journal) = fixture();
    let plan = journal
        .prepare_native(&app.doc, &root.join("out.hbx"))
        .unwrap();
    app.mutate("rename", vec![], |doc| doc.name = "later edit".into());
    let receipt = journal.execute(plan.id()).unwrap();
    assert_eq!(receipt.state, OperationState::Completed);
    assert_eq!(
        storage::load_document(&root.join("out.hbx")).unwrap().name,
        "captured"
    );
    operations::dispatch_saved(&mut app, &plan, &receipt);
    assert!(app.dirty);
    assert_eq!(app.doc.name, "later edit");
}

#[test]
fn restart_reconciles_committed_output() {
    let (app, root, journal) = fixture();
    let path = root.join("out.hbx");
    let plan = journal.prepare_native(&app.doc, &path).unwrap();
    // Simulate a process exit after the output rename, before the completion marker.
    storage::save_document(&app.doc, &path).unwrap();
    let restarted = Journal::new(root.join("journal"));
    let report = restarted.recovery();
    assert_eq!(report.items.len(), 1);
    assert_eq!(report.items[0].disposition, RecoveryDisposition::Committed);
    let receipt = restarted.execute(plan.id()).unwrap();
    assert_eq!(receipt.state, OperationState::Completed);
    assert!(restarted.recovery().items.is_empty());
}

#[test]
fn routine_success_does_not_block() {
    let (mut app, root, journal) = fixture();
    assert!(operations::save_to(
        &mut app,
        &root.join("out.hbx"),
        &journal,
        &root.join("config.json")
    ));
    assert!(app.dialog.is_none());
    assert!(app.navigation.pending.is_none());
    assert!(app.toast.is_some());
    assert!(app.operation_error.is_none());
    assert!(!app.dirty);
}

#[test]
fn settings_failure_does_not_claim_document_failure() {
    let (mut app, root, journal) = fixture();
    let config = root.join("config.json");
    fs::create_dir(&config).unwrap();
    app.dirty = true;
    assert!(operations::save_to(
        &mut app,
        &root.join("out.hbx"),
        &journal,
        &config
    ));
    assert!(!app.dirty);
    assert!(app.operation_error.as_ref().unwrap().contains("settings"));
    assert!(storage::load_document(&root.join("out.hbx")).is_ok());
}

#[test]
fn changed_destination_blocks_retry() {
    let (app, root, journal) = fixture();
    let path = root.join("out.hbx");
    let plan = journal.prepare_native(&app.doc, &path).unwrap();
    fs::write(&path, b"external work").unwrap();
    assert_eq!(
        journal.recovery().items[0].disposition,
        RecoveryDisposition::Conflict
    );
    assert!(journal.execute(plan.id()).is_err());
    assert_eq!(fs::read(path).unwrap(), b"external work");
}

#[test]
fn cancel_before_commit_has_no_output() {
    let (app, root, journal) = fixture();
    let path = root.join("out.hbx");
    let plan = journal.prepare_native(&app.doc, &path).unwrap();
    journal.cancel(plan.id()).unwrap();
    assert!(journal.execute(plan.id()).is_err());
    assert!(!path.exists());
    assert!(journal.recovery().items.is_empty());
}

#[test]
fn completed_retry_cannot_overwrite_newer_output() {
    let (app, root, journal) = fixture();
    let path = root.join("out.hbx");
    let plan = journal.prepare_native(&app.doc, &path).unwrap();
    let first = journal.execute(plan.id()).unwrap();
    fs::write(&path, b"newer work").unwrap();
    assert_eq!(journal.execute(plan.id()).unwrap(), first);
    assert_eq!(fs::read(path).unwrap(), b"newer work");
}

#[test]
fn corrupt_or_newer_journals_are_reported_and_preserved() {
    let (_, root, journal) = fixture();
    fs::create_dir_all(root.join("journal")).unwrap();
    let path = root
        .join("journal")
        .join(format!("{}.json", domain::OperationId::new()));
    fs::write(&path, br#"{"version":999}"#).unwrap();
    let report = journal.recovery();
    assert_eq!(report.errors.len(), 1);
    assert_eq!(fs::read(&path).unwrap(), br#"{"version":999}"#);
}

#[test]
fn failed_journal_prevents_output() {
    let (mut app, root, _) = fixture();
    let journal_path = root.join("not-directory");
    fs::write(&journal_path, b"preserve").unwrap();
    let journal = Journal::new(journal_path);
    app.dirty = true;
    assert!(!operations::save_to(
        &mut app,
        &root.join("out.hbx"),
        &journal,
        &root.join("config.json")
    ));
    assert!(!root.join("out.hbx").exists());
    assert!(app.dirty);
}

#[test]
fn atomic_write_does_not_touch_another_writers_temp() {
    let (_, root, _) = fixture();
    let path = root.join("out");
    fs::write(root.join("out.tmp"), b"owned by another writer").unwrap();
    storage::atomic_write(&path, b"ours").unwrap();
    assert_eq!(
        fs::read(root.join("out.tmp")).unwrap(),
        b"owned by another writer"
    );
}

#[test]
fn failed_rename_cleans_only_its_staging_file() {
    let (_, root, _) = fixture();
    let path = root.join("out");
    fs::create_dir(&path).unwrap();
    assert!(storage::atomic_write(&path, b"ours").is_err());
    assert_eq!(fs::read_dir(&root).unwrap().count(), 1);
}

#[test]
fn changed_receipt_cannot_mark_document_clean() {
    let (mut app, root, journal) = fixture();
    app.dirty = true;
    let plan = journal
        .prepare_native(&app.doc, &root.join("out.hbx"))
        .unwrap();
    let mut receipt = journal.execute(plan.id()).unwrap();
    receipt.sha256 = "wrong output".into();
    operations::dispatch_saved(&mut app, &plan, &receipt);
    assert!(app.dirty);
    assert!(app.path.is_none());
}

#[test]
fn captured_save_retains_versions_but_does_not_clean_later_checkpoint() {
    let (mut app, root, journal) = fixture();
    app.doc.save_version("captured checkpoint".into(), 1);
    let plan = journal
        .prepare_native(&app.doc, &root.join("out.hbx"))
        .unwrap();
    app.doc.save_version("newer checkpoint".into(), 2);
    app.dirty = true;
    let receipt = journal.execute(plan.id()).unwrap();
    operations::dispatch_saved(&mut app, &plan, &receipt);
    assert!(app.dirty);
    let saved = storage::load_document(&root.join("out.hbx")).unwrap();
    assert_eq!(saved.versions.len(), 1);
    assert_eq!(saved.versions[0].label, "captured checkpoint");
}

#[test]
fn competing_captured_writes_do_not_replace_each_other() {
    let (app, root, journal) = fixture();
    let path = root.join("out.hbx");
    let first = journal.prepare_native(&app.doc, &path).unwrap();
    let mut changed = app.doc.clone();
    changed.name = "different output".into();
    let second = journal.prepare_native(&changed, &path).unwrap();
    journal.execute(first.id()).unwrap();
    assert!(journal.execute(second.id()).is_err());
    assert_eq!(storage::load_document(&path).unwrap().name, "captured");
}

#[test]
fn conflicting_input_can_be_recovered_as_new_file() {
    let (app, root, journal) = fixture();
    let path = root.join("out.hbx");
    let plan = journal.prepare_native(&app.doc, &path).unwrap();
    fs::write(&path, b"external work").unwrap();
    let copy = root.join("copy.hbx");
    journal.recover_copy(plan.id(), &copy).unwrap();
    assert_eq!(fs::read(&path).unwrap(), b"external work");
    assert_eq!(storage::load_document(&copy).unwrap(), app.doc);
    assert!(journal.recover_copy(plan.id(), &copy).is_err());
}

#[test]
fn terminal_receipt_drops_the_full_payload() {
    let (app, root, journal) = fixture();
    let plan = journal
        .prepare_native(&app.doc, &root.join("out.hbx"))
        .unwrap();
    journal.execute(plan.id()).unwrap();
    let receipt =
        fs::read_to_string(root.join("journal").join(format!("{}.json", plan.id()))).unwrap();
    assert!(receipt.contains("\"plan\":null"));
    assert!(!receipt.contains("sequence"));
}

#[test]
fn skipping_failed_settings_never_skips_new_unsaved_work() {
    let (mut app, _, _) = fixture();
    app.exit_settings_failed = true;
    assert!(operations::can_exit_without_settings(&app));
    app.mutate("new edit after failed quit", vec![], |doc| {
        doc.name = "unsaved".into()
    });
    assert!(!operations::can_exit_without_settings(&app));
    app.dirty = false;
    app.library.push(appcore::state::OpenDoc {
        doc: app.doc.independent_copy("another".into()),
        path: None,
        dirty: true,
    });
    assert!(!operations::can_exit_without_settings(&app));
}
