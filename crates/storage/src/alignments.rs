//! Persistence of saved sequence alignments (alignments.json).

use std::fs;
use std::path::{Path, PathBuf};

use domain::align::Alignment;
use serde::{Deserialize, Serialize};

use crate::config::config_dir;
use crate::{atomic_write, Result};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SavedAlignment {
    pub id: u64,
    pub name: String,
    pub created: u64,
    pub labels: Vec<String>,
    pub result: Alignment,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Alignments {
    pub items: Vec<SavedAlignment>,
    next_id: u64,
}

impl Alignments {
    pub fn add(&mut self, name: &str, labels: Vec<String>, result: Alignment, created: u64) -> u64 {
        self.next_id += 1;
        let id = self.next_id;
        self.items.push(SavedAlignment {
            id,
            name: name.into(),
            created,
            labels,
            result,
        });
        id
    }

    pub fn get(&self, id: u64) -> Option<&SavedAlignment> {
        self.items.iter().find(|a| a.id == id)
    }

    pub fn remove(&mut self, id: u64) -> bool {
        let before = self.items.len();
        self.items.retain(|a| a.id != id);
        self.items.len() != before
    }
}

#[derive(Serialize, Deserialize)]
struct File {
    version: u32,
    alignments: Alignments,
}

pub fn path() -> PathBuf {
    config_dir().join("alignments.json")
}

pub fn load() -> Alignments {
    load_from(&path())
}

pub fn load_from(p: &Path) -> Alignments {
    let Ok(bytes) = fs::read(p) else {
        return Alignments::default();
    };
    serde_json::from_slice::<File>(&bytes)
        .map(|f| f.alignments)
        .unwrap_or_default()
}

pub fn save(a: &Alignments) -> Result<()> {
    save_to(a, &path())
}

pub fn save_to(a: &Alignments, p: &Path) -> Result<()> {
    let file = File {
        version: 1,
        alignments: a.clone(),
    };
    let json = serde_json::to_vec_pretty(&file)?;
    atomic_write(p, &json)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn result() -> Alignment {
        Alignment {
            rows: vec![b"ATG-".to_vec(), b"ATGA".to_vec()],
            consensus: b"ATGA".to_vec(),
            identity: 75.0,
        }
    }

    #[test]
    fn add_get_remove_roundtrip() {
        let dir = std::env::temp_dir().join(format!("hbx-al-{}", std::process::id()));
        let p = dir.join("alignments.json");
        let mut a = Alignments::default();
        let id = a.add("run1", vec!["a".into(), "b".into()], result(), 42);
        save_to(&a, &p).unwrap();
        let loaded = load_from(&p);
        assert_eq!(loaded, a);
        assert_eq!(loaded.get(id).unwrap().name, "run1");
        let mut loaded = loaded;
        assert!(loaded.remove(id));
        assert!(loaded.get(id).is_none());
        assert!(!loaded.remove(id));
        fs::remove_dir_all(&dir).unwrap();
    }
}
