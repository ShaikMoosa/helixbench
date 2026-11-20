use appcore::{
    import::{self, ImportDestination, ImportOptions, ImportSource, Molecule},
    state::{AppState, Dialog},
};
use domain::{Document, Topology};
use storage::config::Config;

fn begin(text: &str) -> AppState {
    let mut app = AppState::new(
        Document::new("Keep my work".into(), b"ATGC".to_vec(), Topology::Linear),
        Config::default(),
    );
    import::begin_review(
        &mut app,
        ImportSource::text("batch.fa", text),
        ImportDestination::NewDocuments,
    )
    .unwrap();
    app
}
fn draft(app: &mut AppState) -> &mut import::ImportReviewDraft {
    let Some(Dialog::ImportReview(draft)) = &mut app.dialog else {
        panic!("review expected")
    };
    draft
}

#[test]
fn mixed_batch_summary_explains_exclusions_before_confirmation() {
    let mut app = begin(">valid\nATGC\n>invalid\nAT?G\n>protein\nMKWVTFISLL\n");
    let d = draft(&mut app);
    let summary = d.summary();
    assert_eq!(
        (
            summary.total,
            summary.selected,
            summary.excluded,
            summary.invalid
        ),
        (3, 1, 2, 2)
    );
    assert!(summary.needs_skipped_ack);
    assert!(!summary.needs_changes_ack);
    assert!(d.validate().unwrap_err().contains("unselected"));
    d.acknowledge_skipped = true;
    assert!(d.validate().is_ok());
    let ids = import::confirm_import(&mut app).unwrap();
    assert_eq!(ids.len(), 1);
    assert_eq!(app.doc.sequence.bases, b"ATGC");
    assert!(app.dirty);
}

#[test]
fn adding_a_converted_record_revokes_prior_acknowledgments() {
    let mut app = begin(">dna\nATGC\n>rna\nAUGCUU\n");
    let d = draft(&mut app);
    d.options.insert(
        2,
        ImportOptions {
            molecule: Some(Molecule::Dna),
            convert_rna_to_dna: true,
            ..Default::default()
        },
    );
    d.refresh().unwrap();
    assert!(
        !d.summary().needs_changes_ack,
        "unselected conversions do not require acceptance"
    );
    d.acknowledge_skipped = true;
    d.acknowledge_changes = true;
    let revision = d.revision;
    d.set_selected(1, true).unwrap();
    assert!(d.revision > revision);
    assert!(!d.acknowledge_changes);
    assert!(!d.acknowledge_skipped);
    assert!(d.summary().needs_changes_ack);
    assert!(!d.summary().needs_skipped_ack);
    assert!(d.validate().unwrap_err().contains("acknowledge"));
    d.acknowledge_changes = true;
    assert!(d.validate().is_ok());
}

#[test]
fn invalid_selection_is_rejected_without_changing_the_review() {
    let mut app = begin(">dna\nATGC\n>bad\nAT?G\n");
    let d = draft(&mut app);
    let revision = d.revision;
    let selected = d.selected.clone();
    assert!(d.set_selected(1, true).is_err());
    assert!(d.set_selected(100, true).is_err());
    assert_eq!(d.selected, selected);
    assert_eq!(d.revision, revision);
    d.set_selected(0, false).unwrap();
    assert!(d.validate().unwrap_err().contains("at least one"));
}

#[test]
fn validation_checks_names_and_insertion_without_constructing_a_plan() {
    let mut app = begin(">dna\nATGC\n");
    draft(&mut app).names[0] = " ".into();
    assert!(draft(&mut app).validate().unwrap_err().contains("name"));
    draft(&mut app).names[0] = "Reviewed name".into();
    assert!(draft(&mut app).validate().is_ok());
    import::cancel_import(&mut app).unwrap();
    import::review_paste(&mut app, ">one\nATGC\n>two\nGGCC\n").unwrap();
    assert!(draft(&mut app)
        .validate()
        .unwrap_err()
        .contains("exactly one"));
    import::cancel_import(&mut app).unwrap();
    assert_eq!(app.doc.name, "Keep my work");
    assert!(app.library.is_empty());
}

#[test]
fn synchronizing_a_name_immediately_updates_validation_and_invalidates_old_plan() {
    let mut app = begin(">dna\nATGC\n");
    let plan = import::reviewed_plan(draft(&mut app)).unwrap();
    draft(&mut app).sync_name(0, " ").unwrap();
    assert!(draft(&mut app).validate().unwrap_err().contains("name"));
    assert!(import::commit_import(&mut app, &plan).is_err());
    draft(&mut app).sync_name(0, "Corrected name").unwrap();
    assert!(draft(&mut app).validate().is_ok());
    import::confirm_import(&mut app).unwrap();
    assert_eq!(app.doc.name, "Corrected name");
}

#[test]
fn original_sequence_flags_survive_conversion_and_remain_record_specific() {
    let mut app = begin(">plain\nATGC\n>rna\naug-c.u\n");
    let d = draft(&mut app);
    let plain = d.source_facts(0).unwrap();
    let rna = d.source_facts(1).unwrap();
    assert!(!plain.contains_u && !plain.contains_gaps);
    assert!(rna.contains_u && rna.contains_gaps);
    assert_eq!(plain.residue_count, 4);
    assert_eq!(rna.residue_count, 7);
    assert!(d.source_facts(99).is_none());
    d.options.insert(
        2,
        ImportOptions {
            molecule: Some(Molecule::Dna),
            convert_rna_to_dna: true,
            remove_gaps: true,
            ..Default::default()
        },
    );
    d.refresh().unwrap();
    assert_eq!(d.source_facts(0), Some(plain));
    assert_eq!(d.source_facts(1), Some(rna));
    assert!(d.preview.records[1].document.is_some());
}
