//! HelixBench storage: versioned local persistence and sequence-file interchange.
//!
//! Native documents use the `.hbx` format (versioned JSON). Writes are atomic:
//! a temporary sibling file is flushed, then renamed over the destination.

pub mod config;
pub mod fasta;
pub mod feature_lib;
pub mod genbank;
pub mod registry;

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use domain::Document;
use serde::{Deserialize, Serialize};

pub const FORMAT_NAME: &str = "helixbench";
pub const FORMAT_VERSION: u32 = 1;
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

/// Write bytes atomically: temp sibling, flush, rename over the destination.
pub fn atomic_write(path: &Path, bytes: &[u8]) -> Result<()> {
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)?;
        }
    }
    let tmp = temp_sibling(path);
    let write_result = (|| -> Result<()> {
        let mut f = fs::File::create(&tmp)?;
        f.write_all(bytes)?;
        f.sync_all()?;
        Ok(())
    })();
    if let Err(e) = write_result {
        let _ = fs::remove_file(&tmp);
        return Err(e);
    }
    fs::rename(&tmp, path)?;
    Ok(())
}

fn temp_sibling(path: &Path) -> PathBuf {
    let mut name = path
        .file_name()
        .map(|n| n.to_os_string())
        .unwrap_or_default();
    name.push(".tmp");
    path.with_file_name(name)
}

/// Serialize a document to the native `.hbx` format and save it atomically.
pub fn save_document(doc: &Document, path: &Path) -> Result<()> {
    let env = Envelope {
        format: FORMAT_NAME.to_string(),
        version: FORMAT_VERSION,
        document: doc.clone(),
    };
    let mut json = serde_json::to_vec_pretty(&env)?;
    json.push(b'\n');
    atomic_write(path, &json)
}

/// Load a native `.hbx` document.
pub fn load_document(path: &Path) -> Result<Document> {
    let bytes = fs::read(path)?;
    parse_document(&bytes)
}

pub fn parse_document(bytes: &[u8]) -> Result<Document> {
    let env: Envelope = serde_json::from_slice(bytes)?;
    if env.format != FORMAT_NAME {
        return Err(StorageError::Format(format!(
            "not a HelixBench file (format {:?})",
            env.format
        )));
    }
    if env.version > FORMAT_VERSION {
        return Err(StorageError::Format(format!(
            "file version {} is newer than this app supports ({}); update HelixBench",
            env.version, FORMAT_VERSION
        )));
    }
    Ok(env.document)
}

/// Open any supported file: `.hbx`, GenBank, FASTA, or raw sequence text.
/// The format is chosen by extension first, then by content sniffing.
pub fn load_any(path: &Path) -> Result<Document> {
    let bytes = fs::read(path)?;
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
        "hbx" => return parse_document(&bytes),
        "gb" | "gbk" | "genbank" | "gp" => return genbank::parse(&bytes, &stem),
        "fa" | "fasta" | "fna" | "faa" => return fasta::parse_first(&bytes, &stem),
        _ => {}
    }
    sniff_and_parse(&bytes, &stem)
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
    let doc = Document::from_sequence_text(stem.to_string(), text, domain::Topology::Linear);
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
