//! Persistence of biologics entities (biologics.json).

use std::fs;
use std::path::{Path, PathBuf};

use domain::biologics::Biologic;
use serde::{Deserialize, Serialize};

use crate::config::config_dir;
use crate::{atomic_write, Result};

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Biologics {
    pub items: Vec<Biologic>,
    next_id: u64,
}

impl Biologics {
    pub fn add(&mut self, mut b: Biologic) -> u64 {
        self.next_id += 1;
        b.id = self.next_id;
        let id = b.id;
        self.items.push(b);
        id
    }

    pub fn get(&self, id: u64) -> Option<&Biologic> {
        self.items.iter().find(|b| b.id == id)
    }

    pub fn remove(&mut self, id: u64) -> bool {
        let before = self.items.len();
        self.items.retain(|b| b.id != id);
        self.items.len() != before
    }
}

#[derive(Serialize, Deserialize)]
struct File {
    version: u32,
    biologics: Biologics,
}

pub fn path() -> PathBuf {
    config_dir().join("biologics.json")
}

pub fn load() -> Biologics {
    load_from(&path())
}

pub fn load_from(p: &Path) -> Biologics {
    let Ok(bytes) = fs::read(p) else {
        return Biologics::default();
    };
    serde_json::from_slice::<File>(&bytes)
        .map(|f| f.biologics)
        .unwrap_or_default()
}

pub fn save(b: &Biologics) -> Result<()> {
    save_to(b, &path())
}

pub fn save_to(b: &Biologics, p: &Path) -> Result<()> {
    let file = File {
        version: 1,
        biologics: b.clone(),
    };
    let json = serde_json::to_vec_pretty(&file)?;
    atomic_write(p, &json)
}

#[cfg(test)]
mod tests {
    use super::*;
    use domain::biologics::{Chain, ChainKind, Format, Numbering};

    #[test]
    fn add_get_remove_roundtrip() {
        let dir = std::env::temp_dir().join(format!("hbx-bio-{}", std::process::id()));
        let p = dir.join("biologics.json");
        let mut bs = Biologics::default();
        let id = bs.add(Biologic {
            id: 0,
            name: "mAb1".into(),
            format: Format::IgG,
            chains: vec![Chain {
                name: "HC".into(),
                kind: ChainKind::Heavy,
                sequence: "ACD".into(),
            }],
            domains: vec![],
            numbering: Numbering::Kabat,
            created: 1,
            updated: 1,
        });
        save_to(&bs, &p).unwrap();
        let loaded = load_from(&p);
        assert_eq!(loaded, bs);
        assert_eq!(loaded.get(id).unwrap().name, "mAb1");
        let mut loaded = loaded;
        assert!(loaded.remove(id));
        assert!(loaded.get(id).is_none());
        fs::remove_dir_all(&dir).unwrap();
    }
}
