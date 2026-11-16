use domain::{Document, Topology};
#[test]
fn v3_documents_remain_readable_and_upgrade_with_backup() {
    let doc = Document::new("v3".into(), b"ATGC".to_vec(), Topology::Linear);
    let root =
        std::env::temp_dir().join(format!("hbx-p08c-storage-{}", domain::OperationId::new()));
    std::fs::create_dir_all(&root).unwrap();
    let path = root.join("legacy.hbx");
    let original =
        serde_json::json!({"format":"helixbench","version":3,"document":doc}).to_string();
    std::fs::write(&path, &original).unwrap();
    let loaded = storage::load_any(&path).unwrap();
    assert!(loaded.pcr_source.is_none());
    assert!(loaded.primer_pairs.is_empty());
    storage::save_document(&loaded, &path).unwrap();
    let saved: serde_json::Value = serde_json::from_slice(&std::fs::read(&path).unwrap()).unwrap();
    assert_eq!(saved["version"], 4);
    let backup = storage::migrations::backup_path_for_version(&path, 3);
    assert_eq!(std::fs::read_to_string(backup).unwrap(), original);
}
