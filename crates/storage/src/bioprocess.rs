//! Persistence of bioprocess recipes / runs (bioprocess.json).

use std::fs;
use std::path::{Path, PathBuf};

use domain::bioprocess::{Recipe, Run};
use serde::{Deserialize, Serialize};

use crate::config::config_dir;
use crate::{atomic_write, Result};

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Bioprocess {
    pub recipes: Vec<Recipe>,
    pub runs: Vec<Run>,
    next_id: u64,
}

impl Bioprocess {
    pub fn add_recipe(&mut self, r: Recipe) -> u64 {
        self.next_id += 1;
        let id = self.next_id;
        let mut r = r;
        r.id = id;
        self.recipes.push(r);
        id
    }

    pub fn add_run(&mut self, r: Run) -> u64 {
        self.next_id += 1;
        let id = self.next_id;
        let mut r = r;
        r.id = id;
        self.runs.push(r);
        id
    }

    pub fn run_mut(&mut self, id: u64) -> Option<&mut Run> {
        self.runs.iter_mut().find(|r| r.id == id)
    }

    pub fn remove_recipe(&mut self, id: u64) -> bool {
        let before = self.recipes.len();
        self.recipes.retain(|r| r.id != id);
        self.recipes.len() != before
    }

    pub fn remove_run(&mut self, id: u64) -> bool {
        let before = self.runs.len();
        self.runs.retain(|r| r.id != id);
        self.runs.len() != before
    }
}

#[derive(Serialize, Deserialize)]
struct File {
    version: u32,
    bioprocess: Bioprocess,
}

pub fn path() -> PathBuf {
    config_dir().join("bioprocess.json")
}

pub fn load() -> Bioprocess {
    load_from(&path())
}

pub fn load_from(p: &Path) -> Bioprocess {
    let Ok(bytes) = fs::read(p) else {
        return Bioprocess::default();
    };
    serde_json::from_slice::<File>(&bytes)
        .map(|f| f.bioprocess)
        .unwrap_or_default()
}

pub fn save(b: &Bioprocess) -> Result<()> {
    save_to(b, &path())
}

pub fn save_to(b: &Bioprocess, p: &Path) -> Result<()> {
    let file = File {
        version: 1,
        bioprocess: b.clone(),
    };
    let json = serde_json::to_vec_pretty(&file)?;
    atomic_write(p, &json)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recipe_run_roundtrip() {
        let dir = std::env::temp_dir().join(format!("hbx-bp-{}", std::process::id()));
        let p = dir.join("bioprocess.json");
        let mut b = Bioprocess::default();
        let rid = b.add_recipe(Recipe::template(0, "p", "x"));
        let recipe = b.recipes.iter().find(|r| r.id == rid).unwrap().clone();
        let run_id = b.add_run(Run::from_recipe(0, &recipe, "R1", 0));
        save_to(&b, &p).unwrap();
        let loaded = load_from(&p);
        assert_eq!(loaded, b);
        let mut loaded = loaded;
        assert!(loaded.run_mut(run_id).unwrap().start());
        assert!(loaded.remove_run(run_id));
        assert!(loaded.remove_recipe(rid));
        fs::remove_dir_all(&dir).unwrap();
    }
}
