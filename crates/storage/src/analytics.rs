//! Persistence of insights blocks + automation pipelines/runs (analytics.json).

use std::fs;
use std::path::{Path, PathBuf};

use domain::automation::Automation;
use domain::insights::Block;
use serde::{Deserialize, Serialize};

use crate::config::config_dir;
use crate::{atomic_write, Result};

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Analytics {
    pub blocks: Vec<Block>,
    pub automation: Automation,
}

#[derive(Serialize, Deserialize)]
struct File {
    version: u32,
    analytics: Analytics,
}

pub fn path() -> PathBuf {
    config_dir().join("analytics.json")
}

pub fn load() -> Analytics {
    load_from(&path())
}

pub fn load_from(p: &Path) -> Analytics {
    let Ok(bytes) = fs::read(p) else {
        return Analytics::default();
    };
    serde_json::from_slice::<File>(&bytes)
        .map(|f| f.analytics)
        .unwrap_or_default()
}

pub fn save(a: &Analytics) -> Result<()> {
    save_to(a, &path())
}

pub fn save_to(a: &Analytics, p: &Path) -> Result<()> {
    let file = File {
        version: 1,
        analytics: a.clone(),
    };
    let json = serde_json::to_vec_pretty(&file)?;
    atomic_write(p, &json)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip() {
        let dir = std::env::temp_dir().join(format!("hbx-an-{}", std::process::id()));
        let p = dir.join("analytics.json");
        let mut a = Analytics::default();
        a.automation
            .add_pipeline("p", vec![domain::automation::Step::IngestCsv]);
        save_to(&a, &p).unwrap();
        assert_eq!(load_from(&p), a);
        fs::remove_dir_all(&dir).unwrap();
    }
}
