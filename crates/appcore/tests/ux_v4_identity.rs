use appcore::state::AppState;
use domain::{Document, Topology};
use storage::config::Config;

fn app() -> AppState {
    AppState::new(
        Document::new("source".into(), b"ATGC".to_vec(), Topology::Linear),
        Config::default(),
    )
}

#[test]
fn mutation_and_undo_keep_exact_revision_provenance() {
    let mut app = app();
    let initial = app.doc.source_ref();
    app.mutate("edit", vec![], |doc| doc.name = "renamed".into());
    let edited = app.doc.source_ref();
    assert_eq!(initial.document, edited.document);
    assert_ne!(initial.revision, edited.revision);
    assert!(app.undo());
    assert_eq!(app.doc.source_ref(), initial);
    assert!(app.redo());
    assert_eq!(app.doc.source_ref(), edited);
}

#[test]
fn saving_checkpoint_does_not_change_captured_scientific_revision() {
    let mut app = app();
    let initial = app.doc.source_ref();
    appcore::actions::save_version(&mut app, "checkpoint");
    assert_eq!(app.doc.source_ref(), initial);
    assert_eq!(app.doc.versions[0].doc.source_ref(), initial);
}

#[test]
fn editing_tags_advances_revision_and_is_undoable() {
    let mut app = app();
    let initial = app.doc.source_ref();
    appcore::actions::set_tags(&mut app, vec!["tag1".into(), "tag2".into()]);
    assert_ne!(app.doc.source_ref().revision, initial.revision);
    assert!(app.undo());
    assert!(app.doc.tags.is_empty());
    assert_eq!(app.doc.source_ref(), initial);
}

#[test]
fn bulk_tag_updates_revisions_and_dirty_flags_for_only_changed_documents() {
    let mut app = app();
    app.library.push(appcore::state::OpenDoc {
        doc: Document::new("other".into(), b"ATGC".to_vec(), Topology::Linear),
        path: None,
        dirty: false,
    });
    let active = app.doc.source_ref();
    let other = app.library[0].doc.source_ref();
    let draft = appcore::state::WorkspaceDraft {
        selected: vec![true, true],
        tag: "shared".into(),
        error: None,
    };
    appcore::actions::bulk_tag(&mut app, &draft);
    assert_ne!(app.doc.source_ref(), active);
    assert_ne!(app.library[0].doc.source_ref(), other);
    assert!(app.library[0].dirty);
    let after = app.library[0].doc.source_ref();
    appcore::actions::bulk_tag(&mut app, &draft);
    assert_eq!(
        app.library[0].doc.source_ref(),
        after,
        "no-op cannot invent a revision"
    );
    assert!(app.undo());
    assert_eq!(app.doc.source_ref(), active);
}

#[test]
fn bulk_annotation_marks_library_revision_and_dirty_flag() {
    let mut app = app();
    app.library.push(appcore::state::OpenDoc {
        doc: Document::new("other".into(), b"ATGC".to_vec(), Topology::Linear),
        path: None,
        dirty: false,
    });
    app.feature_lib = vec![domain::feature_lib::LibFeature {
        name: "motif".into(),
        kind: "CDS".into(),
        motif: b"ATGC".to_vec(),
        color: [1, 2, 3],
    }];
    let previous = app.library[0].doc.source_ref();
    appcore::actions::bulk_annotate(&mut app);
    assert!(
        app.library[0].doc.features.is_empty(),
        "Preview cannot mutate before explicit apply"
    );
    let review = match &app.dialog {
        Some(appcore::state::Dialog::AnnotationReview(d)) => d.clone(),
        _ => panic!("annotation review expected"),
    };
    let outcomes = appcore::annotation_review::apply(&mut app, &review).unwrap();
    assert!(outcomes.iter().all(|o| o.error.is_none()));
    assert!(!app.library[0].doc.features.is_empty());
    assert_ne!(app.library[0].doc.source_ref(), previous);
    assert!(app.library[0].dirty);
}

#[test]
fn opening_non_native_file_requires_native_save_as_and_retains_original() {
    let mut app = app();
    let path = std::env::temp_dir().join(format!("hbx-p01-import-{}.fasta", std::process::id()));
    let original = b">example\nATGC\n";
    std::fs::write(&path, original).unwrap();
    appcore::actions::open_path(&mut app, &path);
    assert!(
        matches!(app.dialog, Some(appcore::state::Dialog::ImportReview(_))),
        "interchange input must be reviewed before commit"
    );
    appcore::import::confirm_import(&mut app).unwrap();
    assert_eq!(app.doc.sequence.bases, b"ATGC");
    assert!(
        app.path.is_none(),
        "import must choose a native Save As destination"
    );
    assert!(
        app.dirty,
        "imported data is not yet saved as a native document"
    );
    assert_eq!(std::fs::read(&path).unwrap(), original);
    std::fs::remove_file(path).unwrap();
}
