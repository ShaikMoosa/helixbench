//! Notebook-lite: connected experimental entries with prose sections, structured
//! result tables (a submission persistence boundary), templates, and staged review.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntryStatus {
    Draft,
    InReview,
    Approved,
    Archived,
}

impl EntryStatus {
    pub fn label(self) -> &'static str {
        match self {
            EntryStatus::Draft => "Draft",
            EntryStatus::InReview => "In review",
            EntryStatus::Approved => "Approved",
            EntryStatus::Archived => "Archived",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Section {
    pub title: String,
    pub body: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResultTable {
    pub name: String,
    pub columns: Vec<String>,
    pub rows: Vec<Vec<String>>,
    /// Submitted rows become reusable records (persistence boundary).
    pub submitted: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReviewEvent {
    pub ts: u64,
    pub actor: String,
    pub action: String,
    pub comment: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Entry {
    pub id: u64,
    pub title: String,
    pub project: Option<u64>,
    pub folder: String,
    pub sections: Vec<Section>,
    pub tables: Vec<ResultTable>,
    pub status: EntryStatus,
    pub template: Option<String>,
    pub reviews: Vec<ReviewEvent>,
    pub created: u64,
    pub updated: u64,
}

impl Entry {
    pub fn new(id: u64, title: String, now: u64) -> Self {
        Self {
            id,
            title,
            project: None,
            folder: String::new(),
            sections: vec![Section {
                title: "Objective".into(),
                body: String::new(),
            }],
            tables: Vec::new(),
            status: EntryStatus::Draft,
            template: None,
            reviews: Vec::new(),
            created: now,
            updated: now,
        }
    }

    pub fn from_template(id: u64, t: &Template, now: u64) -> Self {
        let mut e = Self::new(id, t.name.clone(), now);
        e.sections = t.sections.clone();
        e.tables = t.tables.clone();
        e.template = Some(t.name.clone());
        e
    }

    pub fn locked(&self) -> bool {
        matches!(self.status, EntryStatus::Approved | EntryStatus::Archived)
    }

    pub fn submit_table(&mut self, idx: usize, now: u64) -> bool {
        if self.locked() {
            return false;
        }
        if let Some(t) = self.tables.get_mut(idx) {
            t.submitted = true;
            self.updated = now;
            true
        } else {
            false
        }
    }

    pub fn submit_review(&mut self, actor: &str, now: u64) -> bool {
        if self.status != EntryStatus::Draft {
            return false;
        }
        self.status = EntryStatus::InReview;
        self.reviews.push(ReviewEvent {
            ts: now,
            actor: actor.into(),
            action: "submitted".into(),
            comment: String::new(),
        });
        self.updated = now;
        true
    }

    pub fn approve(&mut self, actor: &str, comment: &str, now: u64) -> bool {
        if self.status != EntryStatus::InReview {
            return false;
        }
        self.status = EntryStatus::Approved;
        self.reviews.push(ReviewEvent {
            ts: now,
            actor: actor.into(),
            action: "approved".into(),
            comment: comment.into(),
        });
        self.updated = now;
        true
    }

    pub fn reject(&mut self, actor: &str, comment: &str, now: u64) -> bool {
        if self.status != EntryStatus::InReview {
            return false;
        }
        self.status = EntryStatus::Draft;
        self.reviews.push(ReviewEvent {
            ts: now,
            actor: actor.into(),
            action: "rejected".into(),
            comment: comment.into(),
        });
        self.updated = now;
        true
    }

    pub fn archive(&mut self, now: u64) {
        self.status = EntryStatus::Archived;
        self.updated = now;
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Template {
    pub name: String,
    pub sections: Vec<Section>,
    pub tables: Vec<ResultTable>,
}

pub fn default_templates() -> Vec<Template> {
    vec![
        Template {
            name: "Cloning experiment".into(),
            sections: vec![
                Section {
                    title: "Objective".into(),
                    body: String::new(),
                },
                Section {
                    title: "Materials".into(),
                    body: String::new(),
                },
                Section {
                    title: "Protocol".into(),
                    body: String::new(),
                },
                Section {
                    title: "Results".into(),
                    body: String::new(),
                },
            ],
            tables: vec![ResultTable {
                name: "Constructs".into(),
                columns: vec![
                    "Name".into(),
                    "Backbone".into(),
                    "Insert".into(),
                    "Verified".into(),
                ],
                rows: Vec::new(),
                submitted: false,
            }],
        },
        Template {
            name: "PCR validation".into(),
            sections: vec![
                Section {
                    title: "Objective".into(),
                    body: String::new(),
                },
                Section {
                    title: "Primers".into(),
                    body: String::new(),
                },
                Section {
                    title: "Results".into(),
                    body: String::new(),
                },
            ],
            tables: vec![ResultTable {
                name: "Amplicons".into(),
                columns: vec![
                    "Name".into(),
                    "Size (bp)".into(),
                    "Tm (C)".into(),
                    "Band".into(),
                ],
                rows: Vec::new(),
                submitted: false,
            }],
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn review_lifecycle_locks() {
        let mut e = Entry::new(1, "test".into(), 1);
        assert!(!e.locked());
        assert!(e.submit_review("alice", 2));
        assert_eq!(e.status, EntryStatus::InReview);
        assert!(!e.submit_table(0, 3)); // no tables yet
        assert!(e.approve("bob", "ok", 4));
        assert!(e.locked());
        assert!(!e.submit_review("alice", 5)); // locked
        assert_eq!(e.reviews.len(), 2);
    }

    #[test]
    fn reject_returns_to_draft() {
        let mut e = Entry::new(1, "t".into(), 1);
        e.submit_review("a", 2);
        assert!(e.reject("b", "fix", 3));
        assert_eq!(e.status, EntryStatus::Draft);
    }

    #[test]
    fn table_submit_boundary() {
        let mut e = Entry::new(1, "t".into(), 1);
        e.tables.push(ResultTable {
            name: "R".into(),
            columns: vec!["Name".into()],
            rows: vec![vec!["x".into()]],
            submitted: false,
        });
        assert!(e.submit_table(0, 2));
        assert!(e.tables[0].submitted);
    }

    #[test]
    fn template_seeds_entry() {
        let t = &default_templates()[0];
        let e = Entry::from_template(1, t, 1);
        assert_eq!(e.template.as_deref(), Some("Cloning experiment"));
        assert_eq!(e.tables.len(), 1);
    }
}
