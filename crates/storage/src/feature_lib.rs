//! Persistence of the reusable feature library (config dir / feature_lib.json).

use std::fs;
use std::path::{Path, PathBuf};

use domain::feature_lib::LibFeature;
use serde::{Deserialize, Serialize};

use crate::config::config_dir;
use crate::{atomic_write, Result};

#[derive(Serialize, Deserialize)]
struct File {
    version: u32,
    library: Vec<LibFeature>,
}

pub fn path() -> PathBuf {
    config_dir().join("feature_lib.json")
}

pub fn load() -> Vec<LibFeature> {
    load_from(&path())
}

pub fn load_from(p: &Path) -> Vec<LibFeature> {
    let Ok(bytes) = fs::read(p) else {
        return domain::feature_lib::default_library();
    };
    serde_json::from_slice::<File>(&bytes)
        .map(|f| f.library)
        .unwrap_or_else(|_| domain::feature_lib::default_library())
}

pub fn save(library: &[LibFeature]) -> Result<()> {
    save_to(library, &path())
}

pub fn save_to(library: &[LibFeature], p: &Path) -> Result<()> {
    let file = File {
        version: 1,
        library: library.to_vec(),
    };
    let json = serde_json::to_vec_pretty(&file)?;
    atomic_write(p, &json)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip() {
        let dir = std::env::temp_dir().join(format!("hbx-fl-{}", std::process::id()));
        let p = dir.join("feature_lib.json");
        let lib = domain::feature_lib::default_library();
        save_to(&lib, &p).unwrap();
        assert_eq!(load_from(&p), lib);
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn missing_returns_default() {
        let dir = std::env::temp_dir().join(format!("hbx-fl-miss-{}", std::process::id()));
        let p = dir.join("nope.json");
        assert_eq!(load_from(&p), domain::feature_lib::default_library());
    }
}
