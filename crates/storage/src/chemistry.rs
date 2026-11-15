//! Persistence of chemistry compounds / reactions / lots (chemistry.json).

use std::fs;
use std::path::{Path, PathBuf};

use domain::chemistry::{Compound, Reaction};
use serde::{Deserialize, Serialize};

use crate::config::config_dir;
use crate::{atomic_write, Result};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Lot {
    pub id: u64,
    pub compound: u64,
    pub form: u64,
    pub quantity: f32,
    pub unit: String,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Chemistry {
    pub compounds: Vec<Compound>,
    pub reactions: Vec<Reaction>,
    pub lots: Vec<Lot>,
    next_id: u64,
}

impl Chemistry {
    pub fn add_compound(&mut self, name: &str, smiles: &str) -> std::result::Result<u64, String> {
        let mol = domain::chemistry::Molecule::parse(smiles)?;
        self.next_id += 1;
        let id = self.next_id;
        self.compounds.push(Compound {
            id,
            name: name.into(),
            smiles: smiles.into(),
            formula: mol.formula(),
            mw: mol.molecular_weight(),
        });
        Ok(id)
    }

    pub fn add_reaction(&mut self, r: Reaction) -> u64 {
        self.next_id += 1;
        let id = self.next_id;
        let mut r = r;
        r.id = id;
        self.reactions.push(r);
        id
    }

    pub fn add_lot(&mut self, compound: u64, form: u64, quantity: f32, unit: &str) -> u64 {
        self.next_id += 1;
        let id = self.next_id;
        self.lots.push(Lot {
            id,
            compound,
            form,
            quantity,
            unit: unit.into(),
        });
        id
    }

    pub fn remove_compound(&mut self, id: u64) -> bool {
        let before = self.compounds.len();
        self.compounds.retain(|c| c.id != id);
        self.lots.retain(|l| l.compound != id);
        self.compounds.len() != before
    }

    pub fn remove_reaction(&mut self, id: u64) -> bool {
        let before = self.reactions.len();
        self.reactions.retain(|r| r.id != id);
        self.reactions.len() != before
    }
}

#[derive(Serialize, Deserialize)]
struct File {
    version: u32,
    chemistry: Chemistry,
}

pub fn path() -> PathBuf {
    config_dir().join("chemistry.json")
}

pub fn load() -> Chemistry {
    load_from(&path())
}

pub fn load_from(p: &Path) -> Chemistry {
    let Ok(bytes) = fs::read(p) else {
        return Chemistry::default();
    };
    serde_json::from_slice::<File>(&bytes)
        .map(|f| f.chemistry)
        .unwrap_or_default()
}

pub fn save(c: &Chemistry) -> Result<()> {
    save_to(c, &path())
}

pub fn save_to(c: &Chemistry, p: &Path) -> Result<()> {
    let file = File {
        version: 1,
        chemistry: c.clone(),
    };
    let json = serde_json::to_vec_pretty(&file)?;
    atomic_write(p, &json)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compound_reaction_lot_roundtrip() {
        let dir = std::env::temp_dir().join(format!("hbx-chem-{}", std::process::id()));
        let p = dir.join("chemistry.json");
        let mut c = Chemistry::default();
        let id = c.add_compound("ethanol", "CCO").unwrap();
        assert!(c.add_compound("bad", "C1CC").is_err());
        c.add_lot(id, 1, 100.0, "mL");
        save_to(&c, &p).unwrap();
        let loaded = load_from(&p);
        assert_eq!(loaded, c);
        assert_eq!(loaded.compounds[0].formula, "C2H6O");
        let mut loaded = loaded;
        assert!(loaded.remove_compound(id));
        assert!(loaded.lots.is_empty());
        fs::remove_dir_all(&dir).unwrap();
    }
}
