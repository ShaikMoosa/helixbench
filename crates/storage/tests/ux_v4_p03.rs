#[test]
fn protein_fasta_never_cleaned_as_dna() {
    let protein = b">protein\nMKWVTFISLLFLFSSAYSRGVFRR\n";
    assert!(
        storage::fasta::parse_all(protein, "protein").is_err(),
        "protein residues must not disappear through DNA cleaning"
    );
}

#[test]
fn rna_requires_review_instead_of_silent_u_to_t() {
    assert!(storage::fasta::parse_all(b">rna\nAUGCUUACG\n", "rna").is_err());
}

#[test]
fn invalid_fasta_record_is_not_silently_skipped() {
    assert!(storage::fasta::parse_all(b">valid\nATGC\n>empty\n>valid2\nGGCC\n", "input").is_err());
}

#[test]
fn first_record_loader_does_not_silently_discard_other_records() {
    assert!(storage::fasta::parse_first(b">first\nATGC\n>second\nGGCC\n", "input").is_err());
}

#[test]
fn malformed_raw_input_is_not_silently_cleaned() {
    let path = std::env::temp_dir().join(format!("hbx-p03-invalid-{}.txt", std::process::id()));
    std::fs::write(&path, b"ATG?C").unwrap();
    let result = storage::load_any(&path);
    std::fs::remove_file(path).unwrap();
    assert!(result.is_err());
}

#[test]
fn genbank_declared_rna_requires_explicit_conversion_review() {
    let text = b"LOCUS       RNA_TEST 6 bp RNA linear\nORIGIN\n        1 augcuu\n//\n";
    assert!(storage::genbank::parse(text, "rna").is_err());
}

use storage::imports::{preview_import, ImportOptions, ImportSource, Molecule};

#[test]
fn import_reports_u_to_t() {
    let source = ImportSource::text("rna.fasta", ">rna\nAUGCUUACG\n");
    let original = source.original_bytes().to_vec();
    let options = std::collections::HashMap::from([(
        1,
        ImportOptions {
            molecule: Some(Molecule::Dna),
            convert_rna_to_dna: true,
            ..Default::default()
        },
    )]);
    let preview = preview_import(source, &options).unwrap();
    assert_eq!(preview.source.original_bytes(), original);
    let record = &preview.records[0];
    assert_eq!(record.detected, Molecule::Rna);
    assert!(record.requires_acknowledgment);
    assert!(record.transformations.iter().any(|s| s.contains("3 U")));
    assert_eq!(
        record.document.as_ref().unwrap().sequence.bases,
        b"ATGCTTACG"
    );
}

#[test]
fn multi_record_partial_import_retains_indices() {
    let source = ImportSource::text("batch.fasta", ">first\nATGC\n>empty\n>last\nGGCC\n");
    let preview = preview_import(source, &Default::default()).unwrap();
    assert_eq!(
        preview.records.iter().map(|r| r.index).collect::<Vec<_>>(),
        vec![1, 2, 3]
    );
    assert!(preview.records[0].document.is_some());
    assert!(!preview.records[1].errors.is_empty());
    assert!(preview.records[2].document.is_some());
}

#[test]
fn declared_protein_cannot_be_inferred_as_dna_from_ambiguous_letters() {
    let preview = preview_import(
        ImportSource::text("protein.faa", ">protein\nMKWV\n"),
        &Default::default(),
    )
    .unwrap();
    assert_eq!(preview.records[0].detected, Molecule::Protein);
    assert!(preview.records[0].document.is_none());
    assert_eq!(preview.records[0].original_sequence, "MKWV\n");
}

#[test]
fn original_newlines_and_invalid_symbol_position_are_retained() {
    let text = ">name\r\nATG?C\r\n";
    let preview =
        preview_import(ImportSource::text("input.fa", text), &Default::default()).unwrap();
    assert_eq!(preview.source.original_bytes(), text.as_bytes());
    assert_eq!(preview.records[0].original_record, text);
    assert!(preview.records[0]
        .errors
        .iter()
        .any(|s| s.contains("character 4")));
    assert!(preview.records[0].document.is_none());
}

#[test]
fn genbank_review_preserves_supported_annotations_and_original_record() {
    let text = include_str!("../../../docs/implementation-ux-v4/fixtures/annotated.gb");
    let preview = preview_import(
        ImportSource::text("annotated.gb", text),
        &Default::default(),
    )
    .unwrap();
    let doc = preview.records[0]
        .document
        .as_ref()
        .expect("valid GenBank preview");
    assert!(!doc.features.is_empty());
    assert_eq!(doc.import_source.as_ref().unwrap().original_record, text);
    assert_eq!(
        doc.import_source.as_ref().unwrap().source_sha256,
        preview.source_sha256
    );
}

#[test]
fn genbank_unrepresented_metadata_requires_acknowledgment() {
    let text = "LOCUS       TEST 6 bp DNA linear\nACCESSION   ABC123\nFEATURES             Location/Qualifiers\n     CDS             1..6\n                     /experiment=\"source evidence\"\nORIGIN\n        1 atgcaa\n//\n";
    let preview =
        preview_import(ImportSource::text("input.gb", text), &Default::default()).unwrap();
    assert!(preview.records[0].requires_acknowledgment);
    assert!(preview.records[0]
        .transformations
        .iter()
        .any(|s| s.contains("metadata")));
}

#[test]
fn reviewed_gap_removal_is_explicit_and_retains_original() {
    let options = std::collections::HashMap::from([(
        1,
        ImportOptions {
            remove_gaps: true,
            ..Default::default()
        },
    )]);
    let preview = preview_import(
        ImportSource::text("input.fasta", ">gapped\nATG--C\n"),
        &options,
    )
    .unwrap();
    let record = &preview.records[0];
    assert!(record.requires_acknowledgment);
    assert_eq!(record.document.as_ref().unwrap().sequence.bases, b"ATGC");
    assert!(record.original_sequence.contains("--"));
}

#[test]
fn data_before_first_fasta_header_is_not_discarded() {
    assert!(storage::fasta::parse_all(b"ATGC\n>first\nGGCC\n", "input").is_err());
}

#[test]
fn provenance_uses_a_new_native_version_and_preserves_v2_original() {
    let preview = preview_import(
        ImportSource::text("input.fa", ">source\nATGC\n"),
        &Default::default(),
    )
    .unwrap();
    let doc = preview.records[0].document.as_ref().unwrap();
    let path = std::env::temp_dir().join(format!("hbx-p03-format-{}.hbx", std::process::id()));
    let legacy = serde_json::json!({"format":"helixbench", "version":2,
        "document":domain::Document::new("old v2".into(), b"GGCC".to_vec(), domain::Topology::Linear)}).to_string();
    std::fs::write(&path, &legacy).unwrap();
    storage::save_document(doc, &path).unwrap();
    let saved: serde_json::Value = serde_json::from_slice(&std::fs::read(&path).unwrap()).unwrap();
    assert_eq!(
        saved["version"],
        storage::FORMAT_VERSION,
        "older v2 apps must refuse a file whose provenance they cannot preserve"
    );
    let backup = path.with_file_name(format!(
        "{}.v2.bak",
        path.file_name().unwrap().to_string_lossy()
    ));
    assert_eq!(std::fs::read_to_string(&backup).unwrap(), legacy);
    assert_eq!(storage::load_document(&path).unwrap(), *doc);
    std::fs::remove_file(path).unwrap();
    std::fs::remove_file(backup).unwrap();
}

#[test]
fn invalid_input_has_bounded_error_messages_and_complete_original() {
    let text = "?".repeat(10_000);
    let preview = preview_import(
        ImportSource::text("input.txt", text.clone()),
        &Default::default(),
    )
    .unwrap();
    assert!(preview.records[0].errors.len() <= 20);
    assert_eq!(preview.source.original_bytes(), text.as_bytes());
    assert!(preview.records[0].document.is_none());
}

#[test]
fn trailing_genbank_content_is_not_silently_ignored() {
    let text = "LOCUS       TEST 4 bp DNA linear\nORIGIN\n        1 atgc\n//\nGGCC\n";
    let preview =
        preview_import(ImportSource::text("input.gb", text), &Default::default()).unwrap();
    assert!(!preview.records[0].errors.is_empty());
    assert!(preview.records[0].document.is_none());
}

#[test]
fn headerless_fasta_can_still_be_reviewed_as_raw_sequence() {
    let preview = preview_import(
        ImportSource::text("input.fasta", "ATGC\nGGCC\n"),
        &Default::default(),
    )
    .unwrap();
    assert_eq!(
        preview.records[0].document.as_ref().unwrap().sequence.bases,
        b"ATGCGGCC"
    );
}

#[test]
fn rna_subtypes_require_explicit_dna_interpretation() {
    for subtype in ["mRNA", "tRNA", "rRNA", "ss-RNA"] {
        let text = format!("LOCUS       TEST 4 bp {subtype} linear\nORIGIN\n        1 atgc\n//\n");
        let preview = preview_import(
            ImportSource::text("input.gb", text.clone()),
            &Default::default(),
        )
        .unwrap();
        assert_eq!(preview.records[0].detected, Molecule::Rna, "{subtype}");
        assert!(preview.records[0].document.is_none());
        assert!(storage::genbank::parse(text.as_bytes(), "input").is_err());
        let options = std::collections::HashMap::from([(
            1,
            ImportOptions {
                molecule: Some(Molecule::Dna),
                convert_rna_to_dna: true,
                ..Default::default()
            },
        )]);
        let reviewed = preview_import(ImportSource::text("input.gb", text), &options).unwrap();
        assert!(reviewed.records[0].document.is_some());
        assert!(reviewed.records[0].requires_acknowledgment);
    }
}

#[test]
fn omitted_standard_genbank_metadata_always_requires_acknowledgment() {
    for metadata in [
        "SOURCE      synthetic construct\n  ORGANISM  synthetic construct\n",
        "DBLINK      BioProject: TEST\n",
        "KEYWORDS    laboratory record\n",
    ] {
        let text =
            format!("LOCUS       TEST 4 bp DNA linear\n{metadata}ORIGIN\n        1 atgc\n//\n");
        let preview =
            preview_import(ImportSource::text("input.gb", text), &Default::default()).unwrap();
        assert!(preview.records[0].document.is_some());
        assert!(preview.records[0].requires_acknowledgment, "{metadata}");
    }
}

#[test]
fn circular_annotation_coordinates_cannot_wrap_invalid_source_positions() {
    for location in ["10..12", "1..8", "join(3..4,1..8)"] {
        let text = format!("LOCUS       TEST 4 bp DNA circular\nFEATURES             Location/Qualifiers\n     misc_feature    {location}\nORIGIN\n        1 atgc\n//\n");
        let preview = preview_import(
            ImportSource::text("input.gb", text.clone()),
            &Default::default(),
        )
        .unwrap();
        assert!(
            preview.records[0].document.is_none(),
            "invalid coordinate {location}"
        );
        assert!(storage::genbank::parse(text.as_bytes(), "input").is_err());
    }
}

#[test]
fn genbank_terminator_is_not_reported_as_omitted_metadata() {
    let text = "LOCUS       TEST 4 bp DNA linear\nORIGIN\n        1 atgc\n//\n";
    let preview =
        preview_import(ImportSource::text("input.gb", text), &Default::default()).unwrap();
    assert!(preview.records[0].document.is_some());
    assert!(
        !preview.records[0].requires_acknowledgment,
        "the // terminator is a format marker, not omitted metadata"
    );
}
