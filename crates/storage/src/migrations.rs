//! Read-only v1 migration. Identity is stable for an unchanged legacy source.
use crate::{Result, StorageError};
use domain::{Document, DocumentId, RevisionId};
use std::path::{Path, PathBuf};

pub fn migrate_v1(mut doc: Document, scope: &[u8]) -> Document {
    if doc.id.is_nil() {
        doc.id = DocumentId::from_legacy_key(scope);
    }
    if doc.revision_id.is_nil() {
        doc.revision_id = RevisionId::from_legacy_key(scope);
    }
    for (index, version) in doc.versions.iter_mut().enumerate() {
        let mut key = scope.to_vec();
        key.extend_from_slice(format!("/snapshot/{index}").as_bytes());
        version.doc.id = doc.id;
        if version.doc.revision_id.is_nil() {
            version.doc.revision_id = RevisionId::from_legacy_key(&key);
        }
    }
    doc
}

pub fn validate_v2(doc: &Document) -> Result<()> {
    if doc.id.is_nil() || doc.revision_id.is_nil() {
        return Err(StorageError::Format(
            "Native v2 document has a missing identity".into(),
        ));
    }
    for version in &doc.versions {
        if version.doc.id != doc.id
            || version.doc.revision_id.is_nil()
            || !version.doc.versions.is_empty()
        {
            return Err(StorageError::Format(
                "Invalid snapshot identity or nested history".into(),
            ));
        }
    }
    Ok(())
}

pub fn backup_path(path: &Path) -> PathBuf {
    backup_path_for_version(path, 1)
}

pub fn backup_path_for_version(path: &Path, version: u32) -> PathBuf {
    let mut name = path.file_name().unwrap_or_default().to_os_string();
    name.push(format!(".v{version}.bak"));
    path.with_file_name(name)
}

/// Never overwrite an unrelated backup or report a failed backup as a completed migration.
pub fn preserve_v1(path: &Path, bytes: &[u8]) -> Result<()> {
    preserve_original(path, bytes, 1)
}

pub fn preserve_original(path: &Path, bytes: &[u8], version: u32) -> Result<()> {
    use std::{fs, io::Write};
    let backup = backup_path_for_version(path, version);
    let mut file = match fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&backup)
    {
        Ok(file) => file,
        Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
            if fs::read(&backup)? == bytes {
                return Ok(());
            }
            return Err(StorageError::Format(format!(
                "Migration backup {} contains another original; choose another save location",
                backup.display()
            )));
        }
        Err(e) => return Err(e.into()),
    };
    if let Err(e) = file.write_all(bytes).and_then(|_| file.sync_all()) {
        drop(file);
        let _ = fs::remove_file(&backup);
        return Err(e.into());
    }
    Ok(())
}
