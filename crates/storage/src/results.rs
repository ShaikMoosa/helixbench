//! Result persistence preserves captured input independently of live documents.
use crate::{atomic_write, Result, StorageError};
use domain::results::AnalysisResult;
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, fs, path::Path};

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ResultCatalog {
    pub results: Vec<AnalysisResult>,
}

#[derive(Serialize, Deserialize)]
struct File {
    version: u32,
    catalog: ResultCatalog,
}

fn validate(catalog: &ResultCatalog) -> Result<()> {
    let mut ids = HashSet::new();
    for result in &catalog.results {
        result.validate().map_err(StorageError::Format)?;
        if !ids.insert(result.id) {
            return Err(StorageError::Format(
                "Duplicate analysis result identity".into(),
            ));
        }
    }
    Ok(())
}

pub fn load_from(path: &Path) -> Result<ResultCatalog> {
    Ok(load_captured(path)?.0)
}

/// Returns the hash of the exact bytes parsed, for a checked read/modify/write.
pub fn load_captured(path: &Path) -> Result<(ResultCatalog, Option<String>)> {
    let bytes = match fs::read(path) {
        Ok(bytes) => bytes,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            return Ok((ResultCatalog::default(), None))
        }
        Err(e) => return Err(e.into()),
    };
    let header: serde_json::Value = serde_json::from_slice(&bytes)?;
    if header.get("version").and_then(|v| v.as_u64()) != Some(1) {
        return Err(StorageError::Format(
            "Unsupported analysis-result catalog version".into(),
        ));
    }
    let file: File = serde_json::from_slice(&bytes)?;
    validate(&file.catalog)?;
    Ok((file.catalog, Some(crate::content_hash(&bytes))))
}

pub fn save_to(catalog: &ResultCatalog, path: &Path) -> Result<()> {
    validate(catalog)?;
    // An unreadable/newer catalog is not silently replaced by an empty one.
    if path.exists() {
        let previous = load_from(path)?;
        for result in &catalog.results {
            if previous
                .results
                .iter()
                .any(|old| old.id == result.id && old != result)
            {
                return Err(StorageError::Format(
                    "Saved analysis results are immutable; rerun with a new result ID".into(),
                ));
            }
        }
    }
    let bytes = serde_json::to_vec_pretty(&File {
        version: 1,
        catalog: catalog.clone(),
    })?;
    atomic_write(path, &bytes)
}

/// Shared durable catalog location; P09 owns creation and lifecycle UI.
pub fn path() -> std::path::PathBuf {
    crate::config::config_dir().join("results.json")
}

/// Validated catalog encoding for the shared atomic operation journal.
pub fn catalog_text(catalog: &ResultCatalog) -> Result<String> {
    validate(catalog)?;
    Ok(serde_json::to_string_pretty(&File {
        version: 1,
        catalog: catalog.clone(),
    })?)
}
