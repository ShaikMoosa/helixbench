//! Persistence of RNA / modified oligos (rna.json).

use std::fs;
use std::path::{Path, PathBuf};

use domain::rna::Oligo;
use serde::{Deserialize, Serialize};

use crate::config::config_dir;
use crate::{atomic_write, Result};

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Rna {
    pub oligos: Vec<Oligo>,
    next_id: u64,
}

impl Rna {
    pub fn add(&mut self, mut o: Oligo) -> u64 {
        self.next_id += 1;
        o.id = self.next_id;
        let id = o.id;
        self.oligos.push(o);
        id
    }

    pub fn get(&self, id: u64) -> Option<&Oligo> {
        self.oligos.iter().find(|o| o.id == id)
    }

    pub fn get_mut(&mut self, id: u64) -> Option<&mut Oligo> {
        self.oligos.iter_mut().find(|o| o.id == id)
    }

    pub fn remove(&mut self, id: u64) -> bool {
        let before = self.oligos.len();
        self.oligos.retain(|o| o.id != id);
        self.oligos.len() != before
    }
}

#[derive(Serialize, Deserialize)]
struct File {
    version: u32,
    rna: Rna,
}

pub fn path() -> PathBuf {
    config_dir().join("rna.json")
}

pub fn load() -> Rna {
    load_from(&path())
}

pub fn load_from(p: &Path) -> Rna {
    let Ok(bytes) = fs::read(p) else {
        return Rna::default();
    };
    serde_json::from_slice::<File>(&bytes)
        .map(|f| f.rna)
        .unwrap_or_default()
}

pub fn save(r: &Rna) -> Result<()> {
    save_to(r, &path())
}

pub fn save_to(r: &Rna, p: &Path) -> Result<()> {
    let file = File {
        version: 1,
        rna: r.clone(),
    };
    let json = serde_json::to_vec_pretty(&file)?;
    atomic_write(p, &json)
}

#[cfg(test)]
mod tests {
    use super::*;
    use domain::rna::Mod;

    #[test]
    fn oligo_roundtrip() {
        let dir = std::env::temp_dir().join(format!("hbx-rna-{}", std::process::id()));
        let p = dir.join("rna.json");
        let mut r = Rna::default();
        let id = r.add(Oligo {
            id: 0,
            name: "gapmer".into(),
            seq: "AUGCA".into(),
            mods: vec![Mod {
                pos: 0,
                code: "mA".into(),
            }],
        });
        save_to(&r, &p).unwrap();
        let loaded = load_from(&p);
        assert_eq!(loaded, r);
        assert_eq!(loaded.get(id).unwrap().mods.len(), 1);
        let mut loaded = loaded;
        assert!(loaded.remove(id));
        assert!(loaded.get(id).is_none());
        fs::remove_dir_all(&dir).unwrap();
    }
}
