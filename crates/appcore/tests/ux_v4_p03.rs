use appcore::{
    actions,
    state::{AppState, NewSeqState},
};
use appcore::{
    import::{self, ImportDestination, ImportOptions, ImportSource, Molecule},
    state::Dialog,
};
use domain::{Document, Topology};
use storage::config::Config;

fn begin(app: &mut AppState, text: &str) {
    import::begin_review(
        app,
        ImportSource::text("input.fasta", text),
        ImportDestination::NewDocuments,
    )
    .unwrap();
}
fn draft(app: &mut AppState) -> &mut import::ImportReviewDraft {
    match &mut app.dialog {
        Some(Dialog::ImportReview(draft)) => draft,
        _ => panic!("expected import review"),
    }
}

fn app() -> AppState {
    AppState::new(
        Document::new("retained input".into(), b"ATGC".to_vec(), Topology::Linear),
        Config::default(),
    )
}

#[test]
fn invalid_new_sequence_cannot_be_silently_cleaned() {
    let mut app = app();
    let original = app.doc.clone();
    assert!(actions::create_from_dialog(
        &mut app,
        &NewSeqState {
            name: "invalid".into(),
            text: "ATG?C".into(),
            topology: Topology::Linear,
            error: None
        }
    )
    .is_err());
    assert_eq!(app.doc, original);
}

#[test]
fn unsafe_insertion_retains_source_unchanged() {
    let mut app = app();
    let original = app.doc.clone();
    actions::insert_bases(&mut app, "AUG?C", true);
    assert_eq!(app.doc, original);
    assert!(!app.dirty);
}

#[test]
fn opening_interchange_input_requires_review_before_mutation() {
    let mut app = app();
    let original = app.doc.clone();
    let path = std::env::temp_dir().join(format!("hbx-p03-review-{}.fasta", std::process::id()));
    std::fs::write(&path, b">new\nGGCC\n").unwrap();
    actions::open_path(&mut app, &path);
    std::fs::remove_file(path).unwrap();
    assert_eq!(app.doc, original);
    assert!(
        app.dialog.is_some(),
        "a non-mutating review must be visible"
    );
}

#[test]
fn canceled_review_preserves_dirty_document_and_selection() {
    let mut app = app();
    app.mutate("edit", vec![], |doc| doc.name.push('x'));
    app.selection = Some(domain::Range::new(1, 2));
    let original = app.doc.clone();
    begin(&mut app, ">new\nGGCC\n");
    import::cancel_import(&mut app).unwrap();
    assert_eq!(app.doc, original);
    assert_eq!(app.selection, Some(domain::Range::new(1, 2)));
    assert!(app.dirty);
    assert!(app.library.is_empty());
}

#[test]
fn valid_batch_import_preserves_prior_session_and_all_record_identities() {
    let mut app = app();
    app.mutate("edit", vec![], |doc| doc.name = "unsaved original".into());
    let original = app.doc.id;
    begin(&mut app, ">one\nATGC\n>two\nGGCC\n");
    let plan = import::reviewed_plan(draft(&mut app)).unwrap();
    let ids = import::commit_import(&mut app, &plan).unwrap();
    assert_eq!(ids.len(), 2);
    assert_ne!(ids[0], ids[1]);
    assert!(app.dirty);
    assert_eq!(app.library.len(), 2);
    assert!(app.library.iter().all(|doc| doc.dirty));
    assert_eq!(import::commit_import(&mut app, &plan).unwrap(), ids);
    assert_eq!(
        app.library.len(),
        2,
        "repeated confirmation cannot duplicate output"
    );
    actions::switch_document(&mut app, original);
    assert_eq!(app.doc.name, "unsaved original");
    assert!(app.undo());
    assert_eq!(app.doc.name, "retained input");
}

#[test]
fn selected_subset_requires_explicit_skipped_record_acknowledgment() {
    let mut app = app();
    begin(&mut app, ">one\nATGC\n>invalid\nATG?C\n>three\nGGCC\n");
    assert!(import::confirm_import(&mut app).is_err());
    assert!(app.library.is_empty());
    let form = draft(&mut app);
    assert_eq!(form.selected, vec![true, false, true]);
    form.acknowledge_skipped = true;
    form.invalidate();
    let ids = import::confirm_import(&mut app).unwrap();
    assert_eq!(ids.len(), 2);
    assert_eq!(app.doc.import_source.as_ref().unwrap().record_index, 1);
    let third = app.library.iter().find(|doc| doc.doc.id == ids[1]).unwrap();
    assert_eq!(third.doc.import_source.as_ref().unwrap().record_index, 3);
}

#[test]
fn conversion_requires_review_and_acknowledgment_before_commit() {
    let mut app = app();
    begin(&mut app, ">rna\nAUGCUU\n");
    let form = draft(&mut app);
    form.options.insert(
        1,
        ImportOptions {
            molecule: Some(Molecule::Dna),
            convert_rna_to_dna: true,
            ..Default::default()
        },
    );
    form.refresh().unwrap();
    form.selected[0] = true;
    assert!(import::reviewed_plan(form).is_err());
    form.acknowledge_changes = true;
    form.invalidate();
    import::confirm_import(&mut app).unwrap();
    assert_eq!(app.doc.sequence.bases, b"ATGCTT");
    assert!(app
        .doc
        .import_source
        .as_ref()
        .unwrap()
        .original_record
        .contains("AUGCUU"));
}

#[test]
fn duplicate_import_copy_has_new_id() {
    let mut app = app();
    let original = app.doc.id;
    begin(&mut app, ">retained_input\nATGC\n>retained_input\nATGC\n");
    let ids = import::confirm_import(&mut app).unwrap();
    assert!(ids.iter().all(|id| *id != original));
    assert_ne!(ids[0], ids[1]);
    assert_eq!(app.doc.sequence.bases, b"ATGC");
}

#[test]
fn changed_review_rejects_an_older_plan() {
    let mut app = app();
    begin(&mut app, ">one\nATGC\n");
    let plan = import::reviewed_plan(draft(&mut app)).unwrap();
    let form = draft(&mut app);
    form.names[0] = "changed".into();
    form.invalidate();
    assert!(import::commit_import(&mut app, &plan).is_err());
    assert!(app.library.is_empty());
    assert_eq!(app.doc.name, "retained input");
}

#[test]
fn reviewed_paste_uses_captured_range_and_is_undoable() {
    let mut app = app();
    app.selection = Some(domain::Range::new(1, 2));
    import::review_paste(&mut app, "GG").unwrap();
    assert_eq!(app.doc.sequence.bases, b"ATGC");
    let plan = import::reviewed_plan(draft(&mut app)).unwrap();
    app.selection = None;
    import::commit_import(&mut app, &plan).unwrap();
    assert_eq!(app.doc.sequence.bases, b"AGGC");
    assert_eq!(app.doc.import_history.len(), 1);
    assert!(app.undo());
    assert_eq!(app.doc.sequence.bases, b"ATGC");
    assert!(app.doc.import_history.is_empty());
}

#[test]
fn stale_paste_target_never_changes_another_revision() {
    let mut app = app();
    import::review_paste(&mut app, "GG").unwrap();
    let plan = import::reviewed_plan(draft(&mut app)).unwrap();
    app.mutate("late edit", vec![], |doc| doc.name.push('x'));
    let changed = app.doc.clone();
    assert!(import::commit_import(&mut app, &plan).is_err());
    assert_eq!(app.doc, changed);
    assert!(matches!(app.dialog, Some(Dialog::ImportReview(_))));
}

#[test]
fn cancel_import_cannot_destroy_an_unrelated_form() {
    let mut app = app();
    app.dialog = Some(Dialog::NewSequence(NewSeqState {
        name: "keep draft".into(),
        text: "ATGC".into(),
        topology: Topology::Linear,
        error: None,
    }));
    assert!(import::cancel_import(&mut app).is_err());
    assert!(matches!(&app.dialog, Some(Dialog::NewSequence(form)) if form.name == "keep draft"));
}

#[test]
fn saved_paste_provenance_retains_captured_target_revision_and_range() {
    let mut app = app();
    app.selection = Some(domain::Range::new(1, 2));
    let original = app.doc.source_ref();
    import::review_paste(&mut app, "GG").unwrap();
    import::confirm_import(&mut app).unwrap();
    let path =
        std::env::temp_dir().join(format!("hbx-p03-paste-lineage-{}.hbx", std::process::id()));
    storage::save_document(&app.doc, &path).unwrap();
    let saved_text = std::fs::read_to_string(&path).unwrap();
    let saved = storage::load_document(&path).unwrap();
    std::fs::remove_file(path).unwrap();
    assert_eq!(saved.import_history.len(), 1);
    assert!(
        saved_text.contains(&original.revision.to_string()),
        "captured target revision must remain available after saving and loading"
    );
    assert!(saved_text.contains("replace_selection"));
    assert!(saved_text.contains("insertion_position"));
}
