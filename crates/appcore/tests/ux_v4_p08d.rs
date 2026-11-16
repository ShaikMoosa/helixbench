use appcore::{actions, enzyme_workflow as e, history_workflow as h, state::AppState};
use domain::{Document, Range, Topology};
fn app() -> AppState {
    AppState::new(
        Document::new(
            "template".into(),
            b"AAAAGAATTCTTTTGGATCCAAAA".to_vec(),
            Topology::Linear,
        ),
        Default::default(),
    )
}
#[test]
fn uncut_linear_input_is_completed_no_sites() {
    let mut a = app();
    a.doc.chosen_enzymes = vec!["NotI".into()];
    let mut d = e::draft(&a);
    e::run(&a, &mut d).unwrap();
    let r = d.result.unwrap();
    assert!(r.completed_no_sites);
    assert_eq!(r.fragments.len(), 1);
    assert_eq!(r.fragments[0].size, 24);
    assert_eq!(r.input.source(), a.doc.source_ref());
}
#[test]
fn composed_filter_reset_restores_scope() {
    let mut a = app();
    a.doc.chosen_enzymes = vec!["EcoRI".into()];
    let before = a.doc.clone();
    a.enzyme_filter.name = "Eco".into();
    a.enzyme_filter.cut_count = 2;
    a.enzyme_filter.site_len = 3;
    a.enzyme_filter.region = 2;
    a.enzyme_filter.region_start = "1".into();
    a.enzyme_filter.region_end = "12".into();
    assert_eq!(
        e::filtered(&a)
            .unwrap()
            .iter()
            .map(|x| x.0.name.as_str())
            .collect::<Vec<_>>(),
        vec!["EcoRI"]
    );
    e::reset(&mut a);
    assert!(e::filtered(&a).unwrap().len() > 1);
    assert_eq!(e::region(&a).unwrap(), None);
    assert_eq!(a.doc, before);
    assert!(!a.dirty);
}
#[test]
fn restore_cancel_leaves_current_revision() {
    let mut a = app();
    actions::save_version(&mut a, "before");
    a.mutate("name", vec![], |d| d.name = "edited".into());
    a.select(Some(Range::new(2, 4)));
    let doc = a.doc.clone();
    let sel = a.selection;
    let history = a.history.len();
    let review =
        h::preview_version(&a, a.doc.source_ref(), a.doc.versions[0].doc.source_ref()).unwrap();
    assert_ne!(review.target.name, a.doc.name);
    h::cancel(&mut a);
    assert_eq!(a.doc, doc);
    assert_eq!(a.selection, sel);
    assert_eq!(a.history.len(), history);
}
#[test]
fn digest_uses_cleavage_offsets_and_whole_template_despite_filter() {
    let mut a = app();
    a.doc.chosen_enzymes = vec!["EcoRI".into()];
    a.enzyme_filter.name = "NotI".into();
    a.select(Some(Range::new(0, 2)));
    a.enzyme_filter.region = 1;
    let mut d = e::draft(&a);
    e::run(&a, &mut d).unwrap();
    let r = d.result.unwrap();
    assert_eq!(r.sites[0].cut_top, 5);
    assert_eq!(
        r.fragments.iter().map(|f| f.size).collect::<Vec<_>>(),
        vec![5, 19]
    );
    assert_eq!(r.input.selection(), None);
}
#[test]
fn invalid_region_does_not_fall_back_to_whole_sequence() {
    let mut a = app();
    a.enzyme_filter.region = 2;
    for (s, t) in [("bad", "12"), ("0", "12"), ("1", "99"), ("20", "3")] {
        a.enzyme_filter.region_start = s.into();
        a.enzyme_filter.region_end = t.into();
        assert!(e::filtered(&a).is_err());
    }
    a.doc.sequence.topology = Topology::Circular;
    a.enzyme_filter.region_start = "20".into();
    a.enzyme_filter.region_end = "3".into();
    assert_eq!(e::region(&a).unwrap(), Some(Range::new(19, 8)));
    a.enzyme_filter.region = 1;
    assert!(e::filtered(&a).is_err());
}
#[test]
fn unknown_enzyme_and_stale_digest_are_errors() {
    let mut a = app();
    a.doc.chosen_enzymes = vec!["missing".into()];
    let mut d = e::draft(&a);
    assert!(e::run(&a, &mut d).is_err());
    assert_eq!(a.doc.chosen_enzymes, vec!["missing"]);
    a.doc.chosen_enzymes = vec!["EcoRI".into()];
    let mut d = e::draft(&a);
    a.mutate("edit", vec![], |d| d.name = "changed".into());
    assert!(e::run(&a, &mut d).is_err());
    assert!(d.result.is_none());
}
#[test]
fn enzyme_membership_checks_rendered_source_and_undo() {
    let mut a = app();
    let source = a.doc.source_ref();
    e::edit_set(&mut a, source, &["EcoRI".into()], e::SetEdit::Add).unwrap();
    assert_eq!(a.doc.chosen_enzymes, vec!["EcoRI"]);
    assert!(e::edit_set(&mut a, source, &[], e::SetEdit::Replace).is_err());
    assert!(a.undo());
    assert!(a.doc.chosen_enzymes.is_empty());
}
#[test]
fn named_restore_is_new_revision_and_undoable() {
    let mut a = app();
    actions::save_version(&mut a, "before");
    let checkpoint = a.doc.versions[0].doc.source_ref();
    a.mutate("edit", vec![], |d| d.name = "edited".into());
    let before = a.doc.clone();
    let r = h::preview_version(&a, a.doc.source_ref(), checkpoint).unwrap();
    h::restore(&mut a, &r).unwrap();
    assert_eq!(a.doc.name, "template");
    assert_ne!(a.doc.source_ref(), checkpoint);
    assert_eq!(a.doc.versions, before.versions);
    assert!(a.undo());
    assert_eq!(a.doc, before);
}
#[test]
fn old_restore_review_cannot_overwrite_later_edits_or_another_document() {
    let mut a = app();
    actions::save_version(&mut a, "saved");
    let r = h::preview_version(&a, a.doc.source_ref(), a.doc.versions[0].doc.source_ref()).unwrap();
    a.mutate("edit", vec![], |d| d.name = "changed".into());
    assert!(h::restore(&mut a, &r).is_err());
    a = app();
    assert!(h::restore(&mut a, &r).is_err());
}
#[test]
fn checkpoints_survive_edit_undo_and_keep_metadata_dirty() {
    let mut a = app();
    a.mutate("edit", vec![], |d| d.name = "edited".into());
    actions::save_version(&mut a, "keep");
    assert!(a.undo());
    assert_eq!(a.doc.name, "template");
    assert_eq!(a.doc.versions.len(), 1);
    assert!(a.dirty);
    assert!(a.redo());
    assert_eq!(a.doc.versions.len(), 1);
}
#[test]
fn saving_checkpoint_does_not_prune_old_inputs_or_change_scientific_revision() {
    let mut a = app();
    let original = a.doc.source_ref();
    actions::save_version(&mut a, "same");
    assert_eq!(a.doc.source_ref(), original);
    for i in 0..24 {
        a.mutate("edit", vec![], |d| d.name = format!("edit{i}"));
        actions::save_version(&mut a, "same");
    }
    assert_eq!(a.doc.versions.len(), 25);
    assert!(h::preview_version(&a, a.doc.source_ref(), original).is_ok());
}
#[test]
fn saving_checkpoint_to_disk_and_undo_preserves_saved_dirty_truth() {
    let mut a = app();
    a.mutate("edit", vec![], |d| d.name = "edited".into());
    actions::save_version(&mut a, "keep");
    let dir = std::env::temp_dir().join(format!("p08d-{}", domain::OperationId::new()));
    std::fs::create_dir_all(&dir).unwrap();
    let journal = storage::operations::Journal::new(dir.join("journal"));
    assert!(appcore::operations::save_to(
        &mut a,
        &dir.join("saved.hxb"),
        &journal,
        &dir.join("config.json")
    ));
    assert!(!a.dirty);
    assert!(a.undo());
    assert!(a.dirty);
    assert_eq!(a.doc.versions.len(), 1);
    assert!(a.redo());
    assert!(!a.dirty);
}

#[test]
fn checkpoint_row_uses_exact_label_even_for_same_scientific_revision() {
    let mut a = app();
    actions::save_version(&mut a, "first label");
    actions::save_version(&mut a, "second label");
    let chosen = a.doc.versions[1].clone();
    let r = h::preview_checkpoint(&a, a.doc.source_ref(), &chosen).unwrap();
    assert_eq!(r.label, "second label");
    assert_eq!(r.checkpoint, Some(chosen));
}
#[test]
fn checkpoint_save_failure_keeps_current_state_and_unsaved_snapshot() {
    let mut a = app();
    actions::save_version(&mut a, "retain me");
    let doc = a.doc.clone();
    let dir = std::env::temp_dir().join(format!("p08d-fail-{}", domain::OperationId::new()));
    std::fs::create_dir_all(&dir).unwrap();
    let journal = storage::operations::Journal::new(dir.join("journal"));
    assert!(!appcore::operations::save_to(
        &mut a,
        &dir,
        &journal,
        &dir.join("config.json")
    ));
    assert_eq!(a.doc, doc);
    assert!(a.dirty);
    assert!(a.path.is_none());
}

#[test]
fn linear_outside_cleavage_requires_review_instead_of_silent_truncation() {
    let mut a = AppState::new(
        Document::new("short".into(), b"GGTCTCAAAA".to_vec(), Topology::Linear),
        Default::default(),
    );
    a.doc.chosen_enzymes = vec!["BsaI".into()];
    let mut d = e::draft(&a);
    assert!(e::run(&a, &mut d).is_err());
    assert!(d.result.is_none());
}

#[test]
fn saved_enzyme_view_restores_every_composed_filter_and_keeps_chosen_set() {
    let mut a = app();
    let dir = std::env::temp_dir().join(format!("p08d-view-{}", domain::OperationId::new()));
    std::fs::create_dir_all(&dir).unwrap();
    a = AppState::new_in(a.doc.clone(), Default::default(), dir.clone());
    a.doc.chosen_enzymes = vec!["EcoRI".into()];
    let before = a.doc.clone();
    a.enzyme_filter.name = "Eco".into();
    a.enzyme_filter.cut_count = 2;
    a.enzyme_filter.overhang = 1;
    a.enzyme_filter.site_len = 3;
    a.enzyme_filter.region = 2;
    a.enzyme_filter.region_start = "1".into();
    a.enzyme_filter.region_end = "12".into();
    actions::view_save(&mut a, "enzymes", "scoped");
    let id = a.views.items.last().unwrap().id;
    a.views = storage::views::load_from(&dir.join("views.json"));
    e::reset(&mut a);
    actions::view_load(&mut a, id);
    assert_eq!(a.enzyme_filter.name, "Eco");
    assert_eq!(a.enzyme_filter.cut_count, 2);
    assert_eq!(a.enzyme_filter.overhang, 1);
    assert_eq!(a.enzyme_filter.site_len, 3);
    assert_eq!(a.enzyme_filter.region, 2);
    assert_eq!(a.enzyme_filter.region_start, "1");
    assert_eq!(a.enzyme_filter.region_end, "12");
    assert_eq!(a.doc, before);
}
#[test]
fn failed_view_save_rolls_back_memory_and_never_claims_saved() {
    let mut a = app();
    let dir = std::env::temp_dir().join(format!("p08d-view-fail-{}", domain::OperationId::new()));
    std::fs::create_dir_all(dir.join("views.json")).unwrap();
    a = AppState::new_in(a.doc.clone(), Default::default(), dir);
    let before = a.views.clone();
    actions::view_save(&mut a, "enzymes", "retain draft");
    assert_eq!(a.views, before);
    assert!(!a.toast.as_ref().unwrap().0.starts_with("Saved view"));
    assert!(a.operation_error.is_some());
}

#[test]
fn second_cut_opens_exact_map_position_and_back_restores_enzyme_page() {
    let mut a = AppState::new(
        Document::new(
            "two sites".into(),
            b"AAAAGAATTCAAAAGAATTCAAAA".to_vec(),
            Topology::Linear,
        ),
        Default::default(),
    );
    a.view = appcore::state::View::Enzymes;
    let captured = e::cut_sites(&a, a.doc.source_ref(), "EcoRI").unwrap();
    assert_eq!(captured.sites.len(), 2);
    let second = captured.sites[1].clone();
    assert_eq!(second.cut_top, 15);
    e::show_cut(&mut a, &captured, &second).unwrap();
    assert_eq!(a.view, appcore::state::View::Map);
    assert_eq!(a.selection, Some(Range::new(14, 1)));
    appcore::navigation::go_back(&mut a).unwrap();
    assert_eq!(a.view, appcore::state::View::Enzymes);
    assert_eq!(a.selection, None);
    a.mutate("edit", vec![], |d| d.name = "changed".into());
    assert!(e::show_cut(&mut a, &captured, &second).is_err());
}
