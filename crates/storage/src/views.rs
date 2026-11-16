//! Persistence of saved table views (views.json): column visibility + sort +
//! filter presets per list table.

use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::config::config_dir;
use crate::{atomic_write, Result};

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct EnzymeFilter {
    pub name: String,
    pub cut_count: usize,
    pub overhang: usize,
    pub site_len: usize,
    pub region: usize,
    pub region_start: String,
    pub region_end: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SavedView {
    pub id: u64,
    pub table: String,
    pub name: String,
    pub hidden: Vec<usize>,
    pub sort_column: usize,
    /// None = unsorted, Some(true) = desc, Some(false) = asc.
    pub sort_desc: Option<bool>,
    pub filter: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enzyme_filter: Option<EnzymeFilter>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Views {
    pub items: Vec<SavedView>,
    next_id: u64,
}

impl Views {
    pub fn add(
        &mut self,
        table: &str,
        name: &str,
        hidden: Vec<usize>,
        sort_column: usize,
        sort_desc: Option<bool>,
        filter: String,
    ) -> u64 {
        self.next_id += 1;
        let id = self.next_id;
        self.items.push(SavedView {
            id,
            table: table.into(),
            name: name.into(),
            hidden,
            sort_column,
            sort_desc,
            filter,
            enzyme_filter: None,
        });
        id
    }

    pub fn get(&self, id: u64) -> Option<&SavedView> {
        self.items.iter().find(|v| v.id == id)
    }

    pub fn remove(&mut self, id: u64) -> bool {
        let before = self.items.len();
        self.items.retain(|v| v.id != id);
        self.items.len() != before
    }

    pub fn for_table(&self, table: &str) -> Vec<&SavedView> {
        self.items.iter().filter(|v| v.table == table).collect()
    }
}

#[derive(Serialize, Deserialize)]
struct File {
    version: u32,
    views: Views,
}

pub fn path() -> PathBuf {
    config_dir().join("views.json")
}

pub fn load() -> Views {
    load_from(&path())
}

pub fn load_from(p: &Path) -> Views {
    let Ok(bytes) = fs::read(p) else {
        return Views::default();
    };
    serde_json::from_slice::<File>(&bytes)
        .map(|f| f.views)
        .unwrap_or_default()
}

pub fn save(v: &Views) -> Result<()> {
    save_to(v, &path())
}

pub fn save_to(v: &Views, p: &Path) -> Result<()> {
    let file = File {
        version: 2,
        views: v.clone(),
    };
    let json = serde_json::to_vec_pretty(&file)?;
    atomic_write(p, &json)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_for_table_remove_roundtrip() {
        let dir = std::env::temp_dir().join(format!("hbx-views-{}", std::process::id()));
        let p = dir.join("views.json");
        let mut v = Views::default();
        let id = v.add("features", "mine", vec![2, 6], 1, Some(false), "cds".into());
        v.add("primers", "other", vec![], 0, None, String::new());
        save_to(&v, &p).unwrap();
        let loaded = load_from(&p);
        assert_eq!(loaded, v);
        assert_eq!(loaded.for_table("features").len(), 1);
        assert_eq!(loaded.get(id).unwrap().hidden, vec![2, 6]);
        let mut loaded = loaded;
        assert!(loaded.remove(id));
        assert!(loaded.for_table("features").is_empty());
        fs::remove_dir_all(&dir).unwrap();
    }
}
