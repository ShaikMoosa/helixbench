//! Persistence of the local inventory (config dir / inventory.json).

use std::fs;
use std::path::{Path, PathBuf};

use domain::inventory::Inventory;
use serde::{Deserialize, Serialize};

use crate::config::config_dir;
use crate::{atomic_write, Result};

#[derive(Serialize, Deserialize)]
struct File {
    version: u32,
    inventory: Inventory,
}

pub fn path() -> PathBuf {
    config_dir().join("inventory.json")
}

pub fn load() -> Inventory {
    load_from(&path())
}

pub fn load_from(p: &Path) -> Inventory {
    let Ok(bytes) = fs::read(p) else {
        return Inventory::default();
    };
    serde_json::from_slice::<File>(&bytes)
        .map(|f| f.inventory)
        .unwrap_or_default()
}

pub fn save(inv: &Inventory) -> Result<()> {
    save_to(inv, &path())
}

pub fn save_to(inv: &Inventory, p: &Path) -> Result<()> {
    let file = File {
        version: 1,
        inventory: inv.clone(),
    };
    let json = serde_json::to_vec_pretty(&file)?;
    atomic_write(p, &json)
}

#[cfg(test)]
mod tests {
    use super::*;
    use domain::inventory::ContainerKind;

    #[test]
    fn roundtrip() {
        let dir = std::env::temp_dir().join(format!("hbx-inv-{}", std::process::id()));
        let p = dir.join("inventory.json");
        let mut inv = Inventory::default();
        inv.add("Freezer", ContainerKind::Location, None);
        save_to(&inv, &p).unwrap();
        assert_eq!(load_from(&p), inv);
        fs::remove_dir_all(&dir).unwrap();
    }
}
