use appcore::{enzyme_workflow as e, results as r, state::AppState};
use domain::{Document, Topology};
fn app() -> AppState {
    let dir = std::env::temp_dir().join(format!("p09-{}", domain::OperationId::new()));
    AppState::new_in(
        Document::new(
            "template".into(),
            b"AAAAGAATTCTTTTGGATCCAAAA".to_vec(),
            Topology::Linear,
        ),
        Default::default(),
        dir,
    )
}
fn result(a: &mut AppState) -> domain::results::AnalysisResult {
    a.doc.chosen_enzymes = vec!["EcoRI".into()];
    let mut d = e::draft(a);
    e::run(a, &mut d).unwrap();
    r::from_digest(d.result.as_ref().unwrap()).unwrap()
}
#[test]
fn new_source_marks_old_result_older_input() {
    let mut a = app();
    let old = result(&mut a);
    a.mutate("edit", vec![], |d| d.name = "changed".into());
    assert_eq!(
        r::source_status(&a, &old.inputs[0]),
        r::SourceStatus::OlderInput
    );
    assert_eq!(old.inputs[0].document().name, "template");
}
#[test]
fn rerun_allocates_new_result_id() {
    let mut a = app();
    let old = result(&mut a);
    let plan = r::prepare_rerun(&a, &old, false).unwrap();
    let new = r::rerun(&a, &plan).unwrap();
    assert_ne!(old.id, new.id);
    assert_ne!(old.operation, new.operation);
    assert_eq!(old.output, new.output);
    assert_eq!(old.inputs, new.inputs);
}
#[test]
fn deleted_source_opens_tombstone() {
    let mut a = app();
    let old = result(&mut a);
    r::save_analysis(&mut a, &old).unwrap();
    a.set_document(
        Document::new("other".into(), b"ACTG".to_vec(), Topology::Linear),
        None,
    );
    assert_eq!(
        r::source_status(&a, &old.inputs[0]),
        r::SourceStatus::Unavailable
    );
    r::open(&mut a, old.id).unwrap();
    assert_eq!(a.first_use.selected_result, Some(old.id));
    assert_eq!(a.doc.name, "other");
    assert_eq!(old.inputs[0].document().name, "template");
}
#[test]
fn export_contains_input_and_method() {
    let mut a = app();
    let old = result(&mut a);
    let text = r::export_text(std::slice::from_ref(&old), r::ExportFormat::Json).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&text).unwrap();
    assert_eq!(parsed["results"][0]["method_version"], "restriction-v1");
    assert_eq!(
        parsed["results"][0]["inputs"][0]["source"]["revision"],
        old.inputs[0].source().revision.to_string()
    );
    assert!(text.contains("cleavage"));
    let csv = r::export_text(&[old], r::ExportFormat::Csv).unwrap();
    assert!(csv.contains("input_snapshots"));
    assert!(csv.contains("restriction-v1"));
}
#[test]
fn repeated_save_is_idempotent_and_restart_refinds_output() {
    let mut a = app();
    let old = result(&mut a);
    r::save_analysis(&mut a, &old).unwrap();
    r::save_analysis(&mut a, &old).unwrap();
    appcore::first_use::refresh_results(&mut a);
    assert_eq!(a.first_use.results.results.len(), 1);
    assert_eq!(a.first_use.results.results[0], old);
    assert!(a
        .config
        .recent_work
        .iter()
        .any(|w| w.target == storage::config::RecentTarget::Result(old.id)));
}
#[test]
fn failed_save_preserves_catalog_and_pending_result() {
    let mut a = app();
    let old = result(&mut a);
    let p = a.profile_path(storage::results::path());
    std::fs::create_dir_all(&p).unwrap();
    assert!(r::save_analysis(&mut a, &old).is_err());
    assert!(a.first_use.results.results.is_empty());
    assert_eq!(old.inputs[0].document().name, "template");
}
#[test]
fn rerun_current_requires_explicit_matching_source_and_revalidates() {
    let mut a = app();
    let old = result(&mut a);
    a.mutate("edit", vec![], |d| d.sequence.bases.push(b'C'));
    let plan = r::prepare_rerun(&a, &old, true).unwrap();
    a.mutate("later", vec![], |d| d.name = "later".into());
    assert!(r::rerun(&a, &plan).is_err());
    let captured = r::prepare_rerun(&a, &old, false).unwrap();
    assert_eq!(r::rerun(&a, &captured).unwrap().inputs, old.inputs);
}
#[test]
fn save_old_input_does_not_mark_edited_document_saved() {
    let mut a = app();
    let old = result(&mut a);
    a.mutate("edit", vec![], |d| d.name = "edited".into());
    r::save_analysis(&mut a, &old).unwrap();
    assert!(a.dirty);
    assert!(a.path.is_none());
    assert_eq!(a.doc.name, "edited");
}
#[test]
fn corrupt_catalog_is_not_overwritten() {
    let mut a = app();
    let old = result(&mut a);
    let p = a.profile_path(storage::results::path());
    std::fs::create_dir_all(p.parent().unwrap()).unwrap();
    std::fs::write(&p, b"not json").unwrap();
    assert!(r::save_analysis(&mut a, &old).is_err());
    assert_eq!(std::fs::read(&p).unwrap(), b"not json");
}
#[test]
fn comparison_keeps_distinct_results_and_methods() {
    let mut a = app();
    let old = result(&mut a);
    let newer = r::rerun(&a, &r::prepare_rerun(&a, &old, false).unwrap()).unwrap();
    let text = r::export_text(&[old.clone(), newer.clone()], r::ExportFormat::Json).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&text).unwrap();
    assert_eq!(parsed["results"].as_array().unwrap().len(), 2);
    assert_ne!(parsed["results"][0]["id"], parsed["results"][1]["id"]);
    assert_eq!(old.output, newer.output);
}

#[test]
fn source_navigation_and_back_restore_exact_result() {
    let mut a = app();
    let old = result(&mut a);
    r::save_analysis(&mut a, &old).unwrap();
    r::open(&mut a, old.id).unwrap();
    r::open_current_source(&mut a, &old).unwrap();
    assert_eq!(a.view, appcore::state::View::Map);
    appcore::navigation::go_back(&mut a).unwrap();
    assert_eq!(a.first_use.selected_result, Some(old.id));
}

#[test]
fn export_cannot_replace_catalog_through_parent_segments() {
    let mut a = app();
    let old = result(&mut a);
    r::save_analysis(&mut a, &old).unwrap();
    let path = a.profile_path(storage::results::path());
    let before = std::fs::read(&path).unwrap();
    let sub = path.parent().unwrap().join("sub");
    std::fs::create_dir_all(&sub).unwrap();
    assert!(r::export_to(
        &mut a,
        &[old],
        r::ExportFormat::Json,
        &sub.join("..").join("results.json")
    )
    .is_err());
    assert_eq!(std::fs::read(path).unwrap(), before);
}
#[test]
fn csv_export_does_not_create_spreadsheet_formulas() {
    let mut a = app();
    let mut old = result(&mut a);
    old.tool = "=1+1".into();
    let text = r::export_text(&[old], r::ExportFormat::Csv).unwrap();
    assert!(text.contains("\"'=1+1\""));
}
#[test]
fn preview_back_retains_owner_draft_and_captured_run() {
    let mut a = app();
    a.doc.chosen_enzymes = vec!["EcoRI".into()];
    let mut draft = e::draft(&a);
    e::run(&a, &mut draft).unwrap();
    let old = r::from_digest(draft.result.as_ref().unwrap()).unwrap();
    let operation = old.operation;
    a.dialog = Some(appcore::state::Dialog::Digest(draft));
    r::open_preview(&mut a, old).unwrap();
    assert!(a.dialog.is_none());
    r::close(&mut a).unwrap();
    match a.dialog {
        Some(appcore::state::Dialog::Digest(d)) => {
            assert_eq!(d.result.unwrap().operation, operation)
        }
        _ => panic!("owner draft missing"),
    };
}
#[test]
fn pcr_result_keeps_input_and_rerun_preserves_products() {
    let mut a = app();
    let text = "ATGCGTACGATCGGATCCGAATTCGCTAGCTAACGGTACCAAGGTTCCGATCGATTAAGCGTACCGATTCGGAATC";
    a.doc.sequence.bases = text.as_bytes().to_vec();
    let mut d = appcore::state::PcrDraft {
        source: Some(a.doc.source_ref()),
        fwd: text[..20].into(),
        rev: String::from_utf8(domain::sequence::reverse_complement(
            &text.as_bytes()[text.len() - 20..],
        ))
        .unwrap(),
        ..Default::default()
    };
    appcore::primer_workflow::run_pcr(&a, &mut d).unwrap();
    let old = r::from_pcr(d.result.as_ref().unwrap()).unwrap();
    a.mutate("edit", vec![], |d| d.name = "newer".into());
    assert_eq!(old.inputs[0].document().name, "template");
    let newer = r::rerun(&a, &r::prepare_rerun(&a, &old, false).unwrap()).unwrap();
    assert_eq!(newer.output["products"], old.output["products"]);
    assert_ne!(newer.id, old.id);
}

#[test]
fn failed_export_exposes_recovery_without_restart() {
    let mut a = app();
    let old = result(&mut a);
    let path = a
        .profile_path(storage::results::path())
        .with_file_name("readonly-export.json");
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(&path, b"previous").unwrap();
    let original = std::fs::metadata(&path).unwrap().permissions();
    let mut readonly = original.clone();
    readonly.set_readonly(true);
    std::fs::set_permissions(&path, readonly).unwrap();
    let outcome = r::export_to(&mut a, &[old], r::ExportFormat::Json, &path);
    std::fs::set_permissions(&path, original).unwrap();
    assert!(outcome.is_err());
    assert!(a.recovery.items.iter().any(|i| i.destination == path));
}

#[test]
fn catalog_save_rejects_update_since_captured_read() {
    let mut a = app();
    let first = result(&mut a);
    r::save_analysis(&mut a, &first).unwrap();
    let path = a.profile_path(storage::results::path());
    let (mut stale, before) = storage::results::load_captured(&path).unwrap();
    let second = result(&mut a);
    r::save_analysis(&mut a, &second).unwrap();
    let winner = std::fs::read(&path).unwrap();
    stale.results.push(result(&mut a));
    let attempt = a.operation_journal().prepare_text_checked(
        &a.doc,
        &path,
        storage::results::catalog_text(&stale).unwrap(),
        before.as_deref(),
    );
    assert!(attempt.is_err());
    assert_eq!(std::fs::read(&path).unwrap(), winner);
    assert_eq!(storage::results::load_from(&path).unwrap().results.len(), 2);
}
