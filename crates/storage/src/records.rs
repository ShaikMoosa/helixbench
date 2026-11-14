//! Persistence of Phase-1 connected records: project tree, notebook entries,
//! templates and the audit log, in a single versioned atomic file.

use std::fs;
use std::path::{Path, PathBuf};

use domain::audit::AuditLog;
use domain::notebook::{Entry, Template};
use domain::project::Workspace;
use serde::{Deserialize, Serialize};

use crate::config::config_dir;
use crate::{atomic_write, Result};

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Records {
    pub workspace: Workspace,
    pub entries: Vec<Entry>,
    pub templates: Vec<Template>,
    pub audit: AuditLog,
}

#[derive(Serialize, Deserialize)]
struct File {
    version: u32,
    records: Records,
}

pub fn path() -> PathBuf {
    config_dir().join("records.json")
}

pub fn load() -> Records {
    load_from(&path())
}

#[allow(clippy::field_reassign_with_default)]
pub fn load_from(p: &Path) -> Records {
    let Ok(bytes) = fs::read(p) else {
        return default_records();
    };
    let mut r: Records = serde_json::from_slice::<File>(&bytes)
        .map(|f| f.records)
        .unwrap_or_else(|_| default_records());
    if r.templates.is_empty() {
        r = Records {
            templates: domain::notebook::default_templates(),
            ..r
        };
    }
    r
}

fn default_records() -> Records {
    Records {
        templates: domain::notebook::default_templates(),
        ..Default::default()
    }
}

pub fn save(records: &Records) -> Result<()> {
    save_to(records, &path())
}

pub fn save_to(records: &Records, p: &Path) -> Result<()> {
    let file = File {
        version: 1,
        records: records.clone(),
    };
    let json = serde_json::to_vec_pretty(&file)?;
    atomic_write(p, &json)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_and_default_templates() {
        let dir = std::env::temp_dir().join(format!("hbx-rec-{}", std::process::id()));
        let p = dir.join("records.json");
        let mut r = Records {
            templates: domain::notebook::default_templates(),
            ..Default::default()
        };
        r.workspace.add("P", None);
        r.entries
            .push(domain::notebook::Entry::new(1, "e".into(), 1));
        r.audit.append(1, "a", "o", "act", "");
        save_to(&r, &p).unwrap();
        let back = load_from(&p);
        assert_eq!(back, r);
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn missing_gives_templates() {
        let dir = std::env::temp_dir().join(format!("hbx-rec-miss-{}", std::process::id()));
        let r = load_from(&dir.join("nope.json"));
        assert!(!r.templates.is_empty());
    }
}
