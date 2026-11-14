//! Persistence of local workflows (config dir / workflows.json).

use std::fs;
use std::path::{Path, PathBuf};

use domain::workflow::Workflows;
use serde::{Deserialize, Serialize};

use crate::config::config_dir;
use crate::{atomic_write, Result};

#[derive(Serialize, Deserialize)]
struct File {
    version: u32,
    workflows: Workflows,
}

pub fn path() -> PathBuf {
    config_dir().join("workflows.json")
}

pub fn load() -> Workflows {
    load_from(&path())
}

pub fn load_from(p: &Path) -> Workflows {
    let Ok(bytes) = fs::read(p) else {
        return Workflows::default();
    };
    serde_json::from_slice::<File>(&bytes)
        .map(|f| f.workflows)
        .unwrap_or_default()
}

pub fn save(w: &Workflows) -> Result<()> {
    save_to(w, &path())
}

pub fn save_to(w: &Workflows, p: &Path) -> Result<()> {
    let file = File {
        version: 1,
        workflows: w.clone(),
    };
    let json = serde_json::to_vec_pretty(&file)?;
    atomic_write(p, &json)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip() {
        let dir = std::env::temp_dir().join(format!("hbx-wf-{}", std::process::id()));
        let p = dir.join("workflows.json");
        let mut w = Workflows::default();
        w.add_task("T", "s", "local", 1);
        save_to(&w, &p).unwrap();
        assert_eq!(load_from(&p), w);
        fs::remove_dir_all(&dir).unwrap();
    }
}
