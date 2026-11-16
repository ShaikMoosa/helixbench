use domain::{Document, Topology};
use serde_json::{json, Value};
use std::{fs, path::PathBuf};

struct TestDir(PathBuf);
impl TestDir {
    fn new(name: &str) -> Self {
        let path =
            std::env::temp_dir().join(format!("helixbench-p01-{}-{name}", std::process::id()));
        fs::create_dir_all(&path).unwrap();
        Self(path)
    }
}
impl Drop for TestDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
fn legacy_bytes() -> Vec<u8> {
    include_bytes!("../../../docs/implementation-ux-v4/fixtures/legacy-v1.hbx").to_vec()
}
fn value(doc: &Document) -> Value {
    serde_json::to_value(doc).unwrap()
}

#[test]
fn new_documents_with_same_name_have_distinct_durable_identities() {
    let a = value(&Document::new(
        "same".into(),
        b"ATGC".to_vec(),
        Topology::Linear,
    ));
    let b = value(&Document::new(
        "same".into(),
        b"ATGC".to_vec(),
        Topology::Linear,
    ));
    assert!(a["id"].is_string(), "documents need a durable identity");
    assert_ne!(a["id"], b["id"]);
    assert_ne!(a["revision_id"], b["revision_id"]);
}

#[test]
fn migration_v1_assigns_ids_once_and_preserves_original_until_save() {
    let dir = TestDir::new("migration");
    let path = dir.0.join("legacy.hbx");
    let original = legacy_bytes();
    fs::write(&path, &original).unwrap();
    let first = storage::load_document(&path).unwrap();
    let second = storage::load_document(&path).unwrap();
    assert!(value(&first)["id"].is_string());
    assert_eq!(value(&first)["id"], value(&second)["id"]);
    assert_eq!(value(&first)["revision_id"], value(&second)["revision_id"]);
    assert_eq!(
        fs::read(&path).unwrap(),
        original,
        "opening must be read-only"
    );
    storage::save_document(&first, &path).unwrap();
    let saved: Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
    assert_eq!(saved["version"], storage::FORMAT_VERSION);
    assert_eq!(fs::read(dir.0.join("legacy.hbx.v1.bak")).unwrap(), original);
    assert_eq!(storage::load_document(&path).unwrap(), first);
}

#[test]
fn identical_legacy_files_at_different_paths_remain_distinct() {
    let dir = TestDir::new("paths");
    let a = dir.0.join("a.hbx");
    let b = dir.0.join("b.hbx");
    fs::write(&a, legacy_bytes()).unwrap();
    fs::write(&b, legacy_bytes()).unwrap();
    let a = value(&storage::load_document(&a).unwrap());
    let b = value(&storage::load_document(&b).unwrap());
    assert!(a["id"].is_string());
    assert_ne!(a["id"], b["id"]);
}

#[test]
fn save_refuses_to_overwrite_newer_native_file() {
    let dir = TestDir::new("newer");
    let path = dir.0.join("newer.hbx");
    let original = json!({"format":"helixbench","version":999,"document":{}}).to_string();
    fs::write(&path, &original).unwrap();
    let doc = Document::new("new".into(), b"ATGC".to_vec(), Topology::Linear);
    assert!(storage::save_document(&doc, &path).is_err());
    assert_eq!(fs::read_to_string(path).unwrap(), original);
}

#[test]
fn conflicting_migration_backup_blocks_overwrite() {
    let dir = TestDir::new("backup");
    let path = dir.0.join("legacy.hbx");
    let original = legacy_bytes();
    fs::write(&path, &original).unwrap();
    fs::write(dir.0.join("legacy.hbx.v1.bak"), b"different original").unwrap();
    let doc = storage::load_document(&path).unwrap();
    assert!(storage::save_document(&doc, &path).is_err());
    assert_eq!(fs::read(path).unwrap(), original);
}

#[test]
fn rejects_missing_or_nil_identity_in_v2() {
    let missing = json!({"format":"helixbench","version":2,
        "document":serde_json::from_slice::<Value>(&legacy_bytes()).unwrap()["document"]});
    assert!(storage::parse_document(missing.to_string().as_bytes()).is_err());
}

#[test]
fn cross_store_records_expose_typed_links_without_losing_legacy_labels() {
    let mut registry = domain::registry::Registry::default();
    let id = registry.add(domain::registry::Kind::Plasmid, Default::default(), 1);
    registry.link(id, Some("legacy label".into()), 1);
    let entry = serde_json::to_value(registry.get(id).unwrap()).unwrap();
    assert!(entry.as_object().unwrap().contains_key("linked_document"));
    assert_eq!(entry["linked_doc"], "legacy label");
    let mut inventory = domain::inventory::Inventory::default();
    let id = inventory.add("tube", domain::inventory::ContainerKind::Tube, None);
    let tube = serde_json::to_value(inventory.get(id).unwrap()).unwrap();
    assert!(tube.as_object().unwrap().contains_key("entity_ref"));
}

#[test]
fn legacy_link_edits_clear_conflicting_typed_targets() {
    let mut registry = domain::registry::Registry::default();
    let id = registry.add(domain::registry::Kind::Plasmid, Default::default(), 1);
    registry.entries[0].linked_document = Some(domain::DocumentId::new());
    registry.link(id, None, 2);
    assert!(registry.entries[0].linked_document.is_none());
    let mut inventory = domain::inventory::Inventory::default();
    let id = inventory.add("tube", domain::inventory::ContainerKind::Tube, None);
    inventory.containers[0].entity_ref = Some(domain::RecordRef {
        store: domain::StoreKind::Registry,
        local_id: 42,
    });
    inventory.set_contents(id, Some("another".into()), 5.0, 1.0);
    assert!(inventory.containers[0].entity_ref.is_none());
}
