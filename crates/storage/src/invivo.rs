//! Persistence of in-vivo studies (invivo.json).

use std::fs;
use std::path::{Path, PathBuf};

use domain::invivo::{Animal, Cohort, Measurement, Study};
use serde::{Deserialize, Serialize};

use crate::config::config_dir;
use crate::{atomic_write, Result};

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Invivo {
    pub studies: Vec<Study>,
    pub cohorts: Vec<Cohort>,
    pub animals: Vec<Animal>,
    pub measurements: Vec<Measurement>,
    next_id: u64,
}

impl Invivo {
    pub fn add_study(&mut self, name: &str, species: &str, strain: &str, now: u64) -> u64 {
        self.next_id += 1;
        let id = self.next_id;
        self.studies.push(Study {
            id,
            name: name.into(),
            species: species.into(),
            strain: strain.into(),
            created: now,
        });
        id
    }

    pub fn add_cohort(&mut self, study: u64, name: &str, cage: &str) -> u64 {
        self.next_id += 1;
        let id = self.next_id;
        self.cohorts.push(Cohort {
            id,
            study,
            name: name.into(),
            cage: cage.into(),
        });
        id
    }

    pub fn add_animal(&mut self, cohort: u64, tag: &str, sex: char, weight_g: f32) -> u64 {
        self.next_id += 1;
        let id = self.next_id;
        self.animals.push(Animal {
            id,
            cohort,
            tag: tag.into(),
            sex,
            group: 0,
            baseline_weight_g: weight_g,
        });
        id
    }

    pub fn add_measurement(&mut self, m: Measurement) {
        self.measurements.push(m);
    }

    pub fn remove_study(&mut self, id: u64) -> bool {
        let before = self.studies.len();
        self.studies.retain(|s| s.id != id);
        let cohorts: Vec<u64> = self
            .cohorts
            .iter()
            .filter(|c| c.study == id)
            .map(|c| c.id)
            .collect();
        self.cohorts.retain(|c| c.study != id);
        let animals: Vec<u64> = self
            .animals
            .iter()
            .filter(|a| cohorts.contains(&a.cohort))
            .map(|a| a.id)
            .collect();
        self.animals.retain(|a| !cohorts.contains(&a.cohort));
        self.measurements.retain(|m| !animals.contains(&m.animal));
        self.studies.len() != before
    }
}

#[derive(Serialize, Deserialize)]
struct File {
    version: u32,
    invivo: Invivo,
}

pub fn path() -> PathBuf {
    config_dir().join("invivo.json")
}

pub fn load() -> Invivo {
    load_from(&path())
}

pub fn load_from(p: &Path) -> Invivo {
    let Ok(bytes) = fs::read(p) else {
        return Invivo::default();
    };
    serde_json::from_slice::<File>(&bytes)
        .map(|f| f.invivo)
        .unwrap_or_default()
}

pub fn save(v: &Invivo) -> Result<()> {
    save_to(v, &path())
}

pub fn save_to(v: &Invivo, p: &Path) -> Result<()> {
    let file = File {
        version: 1,
        invivo: v.clone(),
    };
    let json = serde_json::to_vec_pretty(&file)?;
    atomic_write(p, &json)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn study_cascade_roundtrip() {
        let dir = std::env::temp_dir().join(format!("hbx-iv-{}", std::process::id()));
        let p = dir.join("invivo.json");
        let mut v = Invivo::default();
        let s = v.add_study("S1", "mouse", "BALB/c", 1);
        let c = v.add_cohort(s, "C1", "cage-1");
        let a = v.add_animal(c, "M1", 'F', 20.0);
        v.add_measurement(Measurement {
            animal: a,
            day: 0,
            weight_g: 20.0,
            tumor_mm3: None,
        });
        save_to(&v, &p).unwrap();
        let loaded = load_from(&p);
        assert_eq!(loaded, v);
        let mut loaded = loaded;
        assert!(loaded.remove_study(s));
        assert!(loaded.animals.is_empty());
        assert!(loaded.measurements.is_empty());
        fs::remove_dir_all(&dir).unwrap();
    }
}
