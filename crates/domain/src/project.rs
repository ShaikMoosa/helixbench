//! Local project / folder tree for organizing documents, entries and registry items.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Project {
    pub id: u64,
    pub name: String,
    pub parent: Option<u64>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Workspace {
    pub projects: Vec<Project>,
    next_id: u64,
}

impl Workspace {
    pub fn add(&mut self, name: &str, parent: Option<u64>) -> u64 {
        self.next_id += 1;
        let id = self.next_id;
        self.projects.push(Project {
            id,
            name: name.into(),
            parent,
        });
        id
    }

    pub fn get(&self, id: u64) -> Option<&Project> {
        self.projects.iter().find(|p| p.id == id)
    }

    pub fn children(&self, parent: Option<u64>) -> Vec<&Project> {
        self.projects
            .iter()
            .filter(|p| p.parent == parent)
            .collect()
    }

    pub fn rename(&mut self, id: u64, name: &str) -> bool {
        if let Some(p) = self.projects.iter_mut().find(|p| p.id == id) {
            p.name = name.into();
            true
        } else {
            false
        }
    }

    /// Delete a node; its children are reparented to its parent.
    pub fn delete(&mut self, id: u64) -> bool {
        let Some(parent) = self.get(id).map(|p| p.parent) else {
            return false;
        };
        for p in self.projects.iter_mut() {
            if p.parent == Some(id) {
                p.parent = parent;
            }
        }
        self.projects.retain(|p| p.id != id);
        true
    }

    /// Breadcrumb path from root to `id`.
    pub fn path(&self, id: u64) -> String {
        let mut parts = Vec::new();
        let mut cur = Some(id);
        while let Some(c) = cur {
            if let Some(p) = self.get(c) {
                parts.push(p.name.clone());
                cur = p.parent;
            } else {
                break;
            }
        }
        parts.reverse();
        parts.join(" / ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tree_and_path() {
        let mut w = Workspace::default();
        let root = w.add("Project A", None);
        let sub = w.add("Sub", Some(root));
        let leaf = w.add("Leaf", Some(sub));
        assert_eq!(w.path(leaf), "Project A / Sub / Leaf");
        assert_eq!(w.children(Some(root)).len(), 1);
    }

    #[test]
    fn delete_reparents() {
        let mut w = Workspace::default();
        let root = w.add("R", None);
        let mid = w.add("M", Some(root));
        let leaf = w.add("L", Some(mid));
        assert!(w.delete(mid));
        assert_eq!(w.get(leaf).unwrap().parent, Some(root));
    }
}
