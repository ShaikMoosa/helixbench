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
    let bytes = match fs::read(path) {
        Ok(bytes) => bytes,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(ResultCatalog::default()),
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
    Ok(file.catalog)
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
