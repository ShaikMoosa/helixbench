//! Versioned, atomic persistence of the local registry (config dir / registry.json).

use std::fs;
use std::path::{Path, PathBuf};

use domain::registry::Registry;
use serde::{Deserialize, Serialize};

use crate::config::config_dir;
use crate::{atomic_write, Result};

#[derive(Serialize, Deserialize)]
struct File {
    version: u32,
    registry: Registry,
}

pub fn path() -> PathBuf {
    config_dir().join("registry.json")
}

pub fn load() -> Registry {
    load_from(&path())
}

pub fn load_from(p: &Path) -> Registry {
    let Ok(bytes) = fs::read(p) else {
        return Registry::default();
    };
    serde_json::from_slice::<File>(&bytes)
        .map(|f| f.registry)
        .unwrap_or_default()
}

pub fn save(registry: &Registry) -> Result<()> {
    save_to(registry, &path())
}

pub fn save_to(registry: &Registry, p: &Path) -> Result<()> {
    let file = File {
        version: 1,
        registry: registry.clone(),
    };
    let json = serde_json::to_vec_pretty(&file)?;
    atomic_write(p, &json)
}

#[cfg(test)]
mod tests {
    use super::*;
    use domain::registry::Kind;
    use std::collections::BTreeMap;

    #[test]
    fn roundtrip_and_corrupt() {
        let dir = std::env::temp_dir().join(format!("hbx-reg-{}", std::process::id()));
        let p = dir.join("registry.json");
        let mut r = Registry::default();
        let mut m = BTreeMap::new();
        m.insert("Name".into(), "pTest".into());
        r.add(Kind::Plasmid, m, 1);
        save_to(&r, &p).unwrap();
        assert_eq!(load_from(&p), r);
        fs::write(&p, b"{corrupt").unwrap();
        assert_eq!(load_from(&p), Registry::default());
        fs::remove_dir_all(&dir).unwrap();
    }
}
