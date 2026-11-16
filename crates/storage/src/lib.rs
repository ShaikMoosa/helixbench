//! HelixBench storage: versioned local persistence and sequence-file interchange.
//!
//! Native documents use the `.hbx` format (versioned JSON). Writes are atomic:
//! a temporary sibling file is flushed, then renamed over the destination.

pub mod alignments;
pub mod analytics;
pub mod biologics;
pub mod bioprocess;
pub mod chemistry;
pub mod config;
pub mod fasta;
pub mod feature_lib;
pub mod genbank;
pub mod imports;
pub mod inventory;
pub mod invivo;
pub mod migrations;
pub mod operations;
pub mod records;
pub mod registry;
pub mod results;
pub mod rna;
pub mod views;
pub mod workflow;

use std::fs;
use std::io::Write;
use std::path::Path;

use domain::Document;
use serde::{Deserialize, Serialize};

pub const FORMAT_NAME: &str = "helixbench";
// v3 adds retained import/paste provenance. Older v2 writers must not drop it.
pub const FORMAT_VERSION: u32 = 3;
pub const NATIVE_EXT: &str = "hbx";

#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Invalid document JSON: {0}")]
    Json(#[from] serde_json::Error),
    #[error("{0}")]
    Format(String),
}

pub type Result<T> = std::result::Result<T, StorageError>;

#[derive(Serialize, Deserialize)]
struct Envelope {
    format: String,
    version: u32,
    document: Document,
}

// Serializes app writers; checks and rename share the same critical section.
// This is process-local, not a cross-process transaction or network-filesystem lock.
static WRITES: std::sync::Mutex<()> = std::sync::Mutex::new(());

pub fn content_hash(bytes: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    Sha256::digest(bytes)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}
pub fn file_hash(path: &Path) -> Result<Option<String>> {
    match fs::read(path) {
        Ok(bytes) => Ok(Some(content_hash(&bytes))),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(e) => Err(e.into()),
    }
}
fn check_before(path: &Path, before: &Option<String>) -> Result<()> {
    if &file_hash(path)? != before {
        return Err(StorageError::Format(
            "Destination changed since review; no output was replaced".into(),
        ));
    }
    Ok(())
}
/// Unique, exclusively created staging file; failed writes/renames remove only that file.
fn atomic_write_unlocked(path: &Path, bytes: &[u8]) -> Result<()> {
    if let Some(parent) = path.parent().filter(|p| !p.as_os_str().is_empty()) {
        fs::create_dir_all(parent)?;
    }
    let mut name = path.file_name().unwrap_or_default().to_os_string();
    name.push(format!(".{}.tmp", domain::OperationId::new()));
    let tmp = path.with_file_name(name);
    let mut f = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&tmp)?;
    let result = (|| -> Result<()> {
        f.write_all(bytes)?;
        f.sync_all()?;
        drop(f);
        fs::rename(&tmp, path)?;
        Ok(())
    })();
    if result.is_err() {
        let _ = fs::remove_file(&tmp);
    }
    result
}
pub fn atomic_write(path: &Path, bytes: &[u8]) -> Result<()> {
    let _lock = WRITES
        .lock()
        .map_err(|_| StorageError::Format("Write controller is unavailable".into()))?;
    atomic_write_unlocked(path, bytes)
}
pub fn atomic_write_checked(path: &Path, bytes: &[u8], before: &Option<String>) -> Result<()> {
    let _lock = WRITES
        .lock()
        .map_err(|_| StorageError::Format("Write controller is unavailable".into()))?;
    check_before(path, before)?;
    atomic_write_unlocked(path, bytes)
}
pub fn document_bytes(doc: &Document) -> Result<Vec<u8>> {
    migrations::validate_v2(doc)?;
    let env = Envelope {
        format: FORMAT_NAME.to_string(),
        version: FORMAT_VERSION,
        document: doc.clone(),
    };
    let mut json = serde_json::to_vec_pretty(&env)?;
    json.push(b'\n');
    Ok(json)
}
fn save_document_unlocked(doc: &Document, path: &Path) -> Result<()> {
    let bytes = document_bytes(doc)?;
    match fs::read(path) {
        Ok(previous) => {
            let version = native_version(&previous)?;
            if version < FORMAT_VERSION {
                migrations::preserve_original(path, &previous, version)?;
            }
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
        Err(e) => return Err(e.into()),
    }
    atomic_write_unlocked(path, &bytes)
}
/// Native persistence retains legacy backups and never replaces newer formats.
pub fn save_document(doc: &Document, path: &Path) -> Result<()> {
    let _lock = WRITES
        .lock()
        .map_err(|_| StorageError::Format("Write controller is unavailable".into()))?;
    save_document_unlocked(doc, path)
}
pub fn save_document_checked(doc: &Document, path: &Path, before: &Option<String>) -> Result<()> {
    let _lock = WRITES
        .lock()
        .map_err(|_| StorageError::Format("Write controller is unavailable".into()))?;
    check_before(path, before)?;
    save_document_unlocked(doc, path)
}

/// Load a native `.hbx` document.
pub fn load_document(path: &Path) -> Result<Document> {
    let bytes = fs::read(path)?;
    parse_document_scoped(&bytes, Some(path))
}

pub fn parse_document(bytes: &[u8]) -> Result<Document> {
    parse_document_scoped(bytes, None)
}

fn native_version(bytes: &[u8]) -> Result<u32> {
    #[derive(Deserialize)]
    struct Header {
        format: String,
        version: u32,
    }
    let header: Header = serde_json::from_slice(bytes)?;
    if header.format != FORMAT_NAME {
        return Err(StorageError::Format(format!(
            "not a HelixBench file (format {:?})",
            header.format
        )));
    }
    if header.version > FORMAT_VERSION {
        return Err(StorageError::Format(format!(
            "file version {} is newer than this app supports ({}); update HelixBench",
            header.version, FORMAT_VERSION
        )));
    }
    if header.version == 0 {
        return Err(StorageError::Format("Unsupported native version 0".into()));
    }
    Ok(header.version)
}

fn parse_document_scoped(bytes: &[u8], path: Option<&Path>) -> Result<Document> {
    let version = native_version(bytes)?;
    let env: Envelope = serde_json::from_slice(bytes)?;
    if version == 1 {
        let mut scope = bytes.to_vec();
        if let Some(path) = path {
            scope.push(0);
            scope.extend_from_slice(fs::canonicalize(path)?.as_os_str().as_encoded_bytes());
        }
        let doc = migrations::migrate_v1(env.document, &scope);
        migrations::validate_v2(&doc)?;
        Ok(doc)
    } else {
        migrations::validate_v2(&env.document)?;
        Ok(env.document)
    }
}

/// Open any supported file: `.hbx`, GenBank, FASTA, or raw sequence text.
/// The format is chosen by extension first, then by content sniffing.
pub fn load_any(path: &Path) -> Result<Document> {
    let bytes = fs::read(path)?;
    load_captured(path, &bytes)
}

/// Parse exactly the bytes captured by the caller, avoiding a second file read.
pub fn load_captured(path: &Path, bytes: &[u8]) -> Result<Document> {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();
    let stem = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("sequence")
        .to_string();
    match ext.as_str() {
        "hbx" => return parse_document_scoped(bytes, Some(path)),
        "gb" | "gbk" | "genbank" | "gp" => return genbank::parse(bytes, &stem),
        "faa" => {
            return Err(StorageError::Format(
                "Protein FASTA requires molecule-aware import review".into(),
            ))
        }
        "fa" | "fasta" | "fna" => return fasta::parse_first(bytes, &stem),
        _ => {}
    }
    if bytes.iter().find(|b| !b.is_ascii_whitespace()) == Some(&b'{') {
        return parse_document_scoped(bytes, Some(path));
    }
    sniff_and_parse(bytes, &stem)
}

fn sniff_and_parse(bytes: &[u8], stem: &str) -> Result<Document> {
    let text = std::str::from_utf8(bytes)
        .map_err(|_| StorageError::Format("file is not valid UTF-8 text".into()))?;
    let head = text.trim_start();
    if head.starts_with('{') {
        return parse_document(bytes);
    }
    if head.starts_with("LOCUS") {
        return genbank::parse(bytes, stem);
    }
    if head.starts_with('>') {
        return fasta::parse_first(bytes, stem);
    }
    let doc = Document::new(
        stem.to_string(),
        imports::strict_dna(text)?,
        domain::Topology::Linear,
    );
    if doc.sequence.is_empty() {
        return Err(StorageError::Format("no DNA sequence found in file".into()));
    }
    Ok(doc)
}

/// Export the top strand as plain text (one line, no header).
pub fn export_plain(doc: &Document) -> String {
    let mut s = String::from_utf8_lossy(&doc.sequence.bases).into_owned();
    s.push('\n');
    s
}

#[cfg(test)]
mod tests {
    use super::*;
    use domain::{Feature, Primer, Range, Topology};

    fn sample_doc() -> Document {
        let mut d = Document::new(
            "test".into(),
            b"ATGAAACCCGGGTTTTAA".to_vec(),
            Topology::Circular,
        );
        d.description = "round trip".into();
        let mut f = Feature::new(0, "cds".into(), "CDS".into(), vec![Range::new(0, 15)]);
        f.direction = domain::Direction::Forward;
        f.note = "hello".into();
        d.add_feature(f);
        d.add_primer(Primer::new(0, "p1".into(), "ATGAAACCC"));
        d.chosen_enzymes = vec!["EcoRI".into(), "BamHI".into()];
        d.color_regions.push(domain::ColorRegion {
            range: Range::new(2, 4),
            strands: domain::document::StrandMask(domain::document::StrandMask::BOTH),
            color: [1, 2, 3],
        });
        d
    }

    #[test]
    fn hbx_roundtrip() {
        let dir = std::env::temp_dir().join(format!("hbx-test-{}", std::process::id()));
        let path = dir.join("doc.hbx");
        let d = sample_doc();
        save_document(&d, &path).unwrap();
        let loaded = load_document(&path).unwrap();
        assert_eq!(loaded, d);
        // Overwrite atomically a second time.
        save_document(&d, &path).unwrap();
        assert_eq!(load_document(&path).unwrap(), d);
        assert!(!path.with_file_name("doc.hbx.tmp").exists());
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn rejects_future_version() {
        let env = serde_json::json!({
            "format": "helixbench", "version": 99,
            "document": sample_doc(),
        });
        let err = parse_document(env.to_string().as_bytes()).unwrap_err();
        assert!(err.to_string().contains("newer"));
    }

    #[test]
    fn rejects_foreign_format() {
        let env = serde_json::json!({ "format": "other", "version": 1, "document": sample_doc() });
        assert!(parse_document(env.to_string().as_bytes()).is_err());
    }

    #[test]
    fn sniff_raw_and_fasta() {
        let d = sniff_and_parse(b"atg ccc\nGGG TTT\n", "raw").unwrap();
        assert_eq!(d.sequence.bases, b"ATGCCCGGGTTT".to_vec());
        let d = sniff_and_parse(b">x desc\nATGCCC\n", "f").unwrap();
        assert_eq!(d.name, "x");
        let d = sniff_and_parse(
            &serde_json::to_vec(&Envelope {
                format: FORMAT_NAME.into(),
                version: 1,
                document: sample_doc(),
            })
            .unwrap(),
            "j",
        )
        .unwrap();
        assert_eq!(d.name, "test");
    }
}
