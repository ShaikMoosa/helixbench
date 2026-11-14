//! Local registry: an offline catalog of plasmids, strains and primers.
//! Schema-driven (strain fields follow the Benchling strain template subset);
//! entries can be linked to an open document. No multi-user / sync.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Kind {
    Plasmid,
    Strain,
    Primer,
}

impl Kind {
    pub const ALL: [Kind; 3] = [Kind::Plasmid, Kind::Strain, Kind::Primer];

    pub fn label(self) -> &'static str {
        match self {
            Kind::Plasmid => "Plasmid",
            Kind::Strain => "Strain",
            Kind::Primer => "Primer",
        }
    }

    /// Ordered field names; the first is always the display name.
    pub fn fields(self) -> &'static [&'static str] {
        match self {
            Kind::Plasmid => &[
                "Name",
                "Backbone",
                "Insert",
                "Resistance",
                "Origin",
                "Promoter",
                "Description",
            ],
            Kind::Strain => &[
                "Name",
                "Organism",
                "Host strain",
                "Parent strain",
                "Antibiotic resistance",
                "Plasmids",
                "Medium",
                "Genotype",
                "Status",
                "Comments",
            ],
            Kind::Primer => &["Name", "Sequence", "Tm (C)", "GC (%)", "Description"],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Entry {
    pub id: u64,
    pub kind: Kind,
    pub values: BTreeMap<String, String>,
    pub created: u64,
    pub updated: u64,
    /// Name of the linked document (if any).
    pub linked_doc: Option<String>,
    /// Archived entries are hidden by default but retained.
    #[serde(default)]
    pub archived: bool,
    /// Related registry entry ids (Entity Map edges).
    #[serde(default)]
    pub relations: Vec<u64>,
}

impl Entry {
    pub fn name(&self) -> &str {
        self.values
            .get("Name")
            .map(|s| s.as_str())
            .unwrap_or("(unnamed)")
    }
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Registry {
    pub entries: Vec<Entry>,
    next_id: u64,
}

impl Registry {
    pub fn add(&mut self, kind: Kind, values: BTreeMap<String, String>, now: u64) -> u64 {
        self.next_id += 1;
        let id = self.next_id;
        self.entries.push(Entry {
            id,
            kind,
            values,
            created: now,
            updated: now,
            linked_doc: None,
            archived: false,
            relations: Vec::new(),
        });
        id
    }

    pub fn set_archived(&mut self, id: u64, archived: bool, now: u64) -> bool {
        if let Some(e) = self.entries.iter_mut().find(|e| e.id == id) {
            e.archived = archived;
            e.updated = now;
            true
        } else {
            false
        }
    }

    pub fn add_relation(&mut self, a: u64, b: u64, now: u64) -> bool {
        if a == b || self.get(a).is_none() || self.get(b).is_none() {
            return false;
        }
        for (x, y) in [(a, b), (b, a)] {
            if let Some(e) = self.entries.iter_mut().find(|e| e.id == x) {
                if !e.relations.contains(&y) {
                    e.relations.push(y);
                }
                e.updated = now;
            }
        }
        true
    }

    /// Entity Map edges (undirected, deduplicated).
    pub fn edges(&self) -> Vec<(u64, u64)> {
        let mut out = Vec::new();
        for e in &self.entries {
            for &r in &e.relations {
                if e.id < r {
                    out.push((e.id, r));
                }
            }
        }
        out
    }

    pub fn get(&self, id: u64) -> Option<&Entry> {
        self.entries.iter().find(|e| e.id == id)
    }

    pub fn update(&mut self, id: u64, values: BTreeMap<String, String>, now: u64) -> bool {
        if let Some(e) = self.entries.iter_mut().find(|e| e.id == id) {
            e.values = values;
            e.updated = now;
            true
        } else {
            false
        }
    }

    pub fn delete(&mut self, id: u64) -> bool {
        let before = self.entries.len();
        self.entries.retain(|e| e.id != id);
        self.entries.len() != before
    }

    pub fn link(&mut self, id: u64, doc: Option<String>, now: u64) -> bool {
        if let Some(e) = self.entries.iter_mut().find(|e| e.id == id) {
            e.linked_doc = doc;
            e.updated = now;
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vals(name: &str) -> BTreeMap<String, String> {
        let mut m = BTreeMap::new();
        m.insert("Name".into(), name.into());
        m
    }

    #[test]
    fn add_update_delete_link() {
        let mut r = Registry::default();
        let id = r.add(Kind::Strain, vals("DH5a"), 1);
        assert_eq!(r.get(id).unwrap().name(), "DH5a");
        assert!(r.update(id, vals("DH5a-2"), 2));
        assert_eq!(r.get(id).unwrap().name(), "DH5a-2");
        assert!(r.link(id, Some("pDemo".into()), 3));
        assert_eq!(r.get(id).unwrap().linked_doc.as_deref(), Some("pDemo"));
        assert!(r.delete(id));
        assert!(r.get(id).is_none());
    }

    #[test]
    fn strain_fields_include_template_columns() {
        let f = Kind::Strain.fields();
        for expect in [
            "Organism",
            "Host strain",
            "Antibiotic resistance",
            "Genotype",
            "Status",
        ] {
            assert!(f.contains(&expect));
        }
    }

    #[test]
    fn ids_unique() {
        let mut r = Registry::default();
        let a = r.add(Kind::Plasmid, vals("p1"), 1);
        let b = r.add(Kind::Plasmid, vals("p2"), 1);
        assert_ne!(a, b);
    }
}
