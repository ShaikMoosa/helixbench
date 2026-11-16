//! App configuration in `%APPDATA%/HelixBench` (recent files, window, UI prefs).

use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::{atomic_write, Result};

pub const MAX_RECENT: usize = 12;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UiPrefs {
    pub show_features: bool,
    pub show_enzymes: bool,
    pub show_primers: bool,
    pub show_orfs: bool,
    pub show_translations: bool,
    pub show_complement: bool,
    pub show_colors: bool,
    pub show_zoom_controls: bool,
    pub orf_min_aa: usize,
    pub genetic_code: u8,
}

impl Default for UiPrefs {
    fn default() -> Self {
        Self {
            show_features: true,
            show_enzymes: true,
            show_primers: true,
            show_orfs: false,
            show_translations: false,
            show_complement: true,
            show_colors: false,
            show_zoom_controls: false,
            orf_min_aa: 50,
            genetic_code: 1,
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub first_use_dismissed: bool,
    pub recent_work: Vec<RecentWork>,
    pub recent_files: Vec<PathBuf>,
    pub last_directory: Option<PathBuf>,
    pub window_size: Option<[f32; 2]>,
    pub window_maximized: bool,
    pub ui: UiPrefs,
}

impl Config {
    pub fn add_recent(&mut self, path: &Path) {
        self.recent_files.retain(|p| p != path);
        self.recent_files.insert(0, path.to_path_buf());
        self.recent_files.truncate(MAX_RECENT);
        if let Some(parent) = path.parent() {
            self.last_directory = Some(parent.to_path_buf());
        }
    }

    pub fn remove_recent(&mut self, path: &Path) {
        self.recent_files.retain(|p| p != path);
    }
}

/// `%APPDATA%/HelixBench`, falling back to `~/.helixbench`.
pub fn config_dir() -> PathBuf {
    if let Ok(appdata) = std::env::var("APPDATA") {
        if !appdata.is_empty() {
            return PathBuf::from(appdata).join("HelixBench");
        }
    }
    let home = std::env::var("USERPROFILE")
        .or_else(|_| std::env::var("HOME"))
        .unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home).join(".helixbench")
}

pub fn config_path() -> PathBuf {
    config_dir().join("config.json")
}

/// Load the config; missing or corrupt files fall back to defaults. A corrupt
/// file is preserved as `config.json.bak` instead of being silently destroyed.
pub fn load() -> Config {
    load_from(&config_path())
}

pub fn load_from(path: &Path) -> Config {
    let Ok(bytes) = fs::read(path) else {
        return Config::default();
    };
    match serde_json::from_slice::<Config>(&bytes) {
        Ok(c) => c,
        Err(_) => {
            let bak = path.with_extension("json.bak");
            let _ = fs::rename(path, bak);
            Config::default()
        }
    }
}

pub fn save(config: &Config) -> Result<()> {
    save_to(config, &config_path())
}

pub fn save_to(config: &Config, path: &Path) -> Result<()> {
    let mut json = serde_json::to_vec_pretty(config)?;
    json.push(b'\n');
    atomic_write(path, &json)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recent_dedup_and_cap() {
        let mut c = Config::default();
        for i in 0..20 {
            c.add_recent(Path::new(&format!("C:/x/f{i}.hbx")));
        }
        assert_eq!(c.recent_files.len(), MAX_RECENT);
        assert_eq!(c.recent_files[0], PathBuf::from("C:/x/f19.hbx"));
        c.add_recent(Path::new("C:/x/f10.hbx"));
        assert_eq!(c.recent_files[0], PathBuf::from("C:/x/f10.hbx"));
        assert_eq!(
            c.recent_files
                .iter()
                .filter(|p| p.as_path() == Path::new("C:/x/f10.hbx"))
                .count(),
            1
        );
        assert_eq!(c.last_directory, Some(PathBuf::from("C:/x")));
        c.remove_recent(Path::new("C:/x/f10.hbx"));
        assert!(!c.recent_files.contains(&PathBuf::from("C:/x/f10.hbx")));
    }

    #[test]
    fn roundtrip_and_corrupt() {
        let dir = std::env::temp_dir().join(format!("hbx-cfg-{}", std::process::id()));
        let path = dir.join("config.json");
        let mut c = Config::default();
        c.ui.show_orfs = true;
        c.window_size = Some([1200.0, 800.0]);
        c.add_recent(Path::new("C:/x/a.hbx"));
        save_to(&c, &path).unwrap();
        assert_eq!(load_from(&path), c);
        fs::write(&path, b"{corrupt").unwrap();
        assert_eq!(load_from(&path), Config::default());
        assert!(dir.join("config.json.bak").exists());
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn partial_json_uses_defaults() {
        let c: Config = serde_json::from_str(r#"{"recent_files":["C:/a.hbx"]}"#).unwrap();
        assert_eq!(c.recent_files.len(), 1);
        assert!(c.ui.show_features);
        assert_eq!(c.ui.orf_min_aa, 50);
    }
}

/// Identity-qualified resume links. Labels are display text, never routing keys.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RecentTarget {
    Document {
        document: domain::DocumentId,
        path: Option<PathBuf>,
    },
    Record(domain::RecordRef),
    Result(domain::ResultId),
    File(PathBuf),
}
impl RecentTarget {
    pub fn same_identity(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Document { document: a, .. }, Self::Document { document: b, .. }) => a == b,
            _ => self == other,
        }
    }
    pub fn kind(&self) -> &'static str {
        match self {
            Self::Document { .. } => "Document",
            Self::Record(r) => match r.store {
                domain::StoreKind::Notebook => "Notebook entry",
                domain::StoreKind::Workflow => "Workflow task",
                _ => "Workspace record",
            },
            Self::Result(_) => "Saved result",
            Self::File(_) => "File",
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecentWork {
    pub target: RecentTarget,
    pub title: String,
}
impl Config {
    pub fn remember_work(&mut self, work: RecentWork) {
        self.recent_work
            .retain(|item| !item.target.same_identity(&work.target));
        self.recent_work.insert(0, work);
        self.recent_work.truncate(20);
    }
}
