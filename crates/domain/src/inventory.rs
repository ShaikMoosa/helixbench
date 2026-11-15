//! Inventory: physical storage as nested locations / boxes / plates / containers.
//! Registry identity is distinct from custody: a container links to an entity and
//! tracks quantity, concentration, placement and well role. Plate snapshots are
//! immutable once recorded in a notebook entry.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContainerKind {
    Location,
    Box,
    Plate,
    Tube,
}

impl ContainerKind {
    pub fn label(self) -> &'static str {
        match self {
            ContainerKind::Location => "Location",
            ContainerKind::Box => "Box",
            ContainerKind::Plate => "Plate",
            ContainerKind::Tube => "Tube",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WellRole {
    Sample,
    Control,
    Standard,
    Blank,
}

impl WellRole {
    pub fn label(self) -> &'static str {
        match self {
            WellRole::Sample => "Sample",
            WellRole::Control => "Control",
            WellRole::Standard => "Standard",
            WellRole::Blank => "Blank",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Container {
    pub id: u64,
    pub name: String,
    pub kind: ContainerKind,
    pub parent: Option<u64>,
    /// Well position (e.g. "A1") for tubes inside a plate/box.
    pub position: Option<String>,
    pub role: Option<WellRole>,
    /// Linked registry entity name (scientific identity).
    pub entity: Option<String>,
    /// Cross-store identity. Keep the legacy label for display and repair.
    #[serde(default)]
    pub entity_ref: Option<crate::RecordRef>,
    pub quantity: f32,
    pub unit: String,
    pub concentration: f32,
    /// Plate dimensions (rows x cols), only for Plate.
    pub rows: u8,
    pub cols: u8,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlateSnapshot {
    pub plate_name: String,
    pub taken: u64,
    pub wells: Vec<(String, Option<String>, Option<WellRole>, f32)>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Inventory {
    pub containers: Vec<Container>,
    next_id: u64,
}

impl Inventory {
    pub fn add(&mut self, name: &str, kind: ContainerKind, parent: Option<u64>) -> u64 {
        self.next_id += 1;
        let id = self.next_id;
        let (rows, cols) = if kind == ContainerKind::Plate {
            (8, 12)
        } else {
            (0, 0)
        };
        self.containers.push(Container {
            id,
            name: name.into(),
            kind,
            parent,
            position: None,
            role: None,
            entity: None,
            entity_ref: None,
            quantity: 0.0,
            unit: "uL".into(),
            concentration: 0.0,
            rows,
            cols,
        });
        id
    }

    pub fn get(&self, id: u64) -> Option<&Container> {
        self.containers.iter().find(|c| c.id == id)
    }

    pub fn children(&self, parent: Option<u64>) -> Vec<&Container> {
        self.containers
            .iter()
            .filter(|c| c.parent == parent)
            .collect()
    }

    pub fn place(&mut self, id: u64, parent: Option<u64>, position: Option<String>) -> bool {
        if let Some(c) = self.containers.iter_mut().find(|c| c.id == id) {
            c.parent = parent;
            c.position = position;
            true
        } else {
            false
        }
    }

    pub fn set_contents(
        &mut self,
        id: u64,
        entity: Option<String>,
        quantity: f32,
        concentration: f32,
    ) -> bool {
        if let Some(c) = self.containers.iter_mut().find(|c| c.id == id) {
            if entity.is_none() || c.entity != entity {
                c.entity_ref = None;
            }
            c.entity = entity;
            c.quantity = quantity;
            c.concentration = concentration;
            true
        } else {
            false
        }
    }

    /// Fill wells of a plate: create tube children at positions with entity + role.
    pub fn fill_wells(
        &mut self,
        plate: u64,
        wells: &[(String, Option<String>, WellRole, f32)],
    ) -> usize {
        let mut n = 0;
        for (pos, entity, role, qty) in wells {
            self.next_id += 1;
            let id = self.next_id;
            self.containers.push(Container {
                id,
                name: format!(
                    "{} {}",
                    self.get(plate).map(|p| p.name.clone()).unwrap_or_default(),
                    pos
                ),
                kind: ContainerKind::Tube,
                parent: Some(plate),
                position: Some(pos.clone()),
                role: Some(*role),
                entity: entity.clone(),
                entity_ref: None,
                quantity: *qty,
                unit: "uL".into(),
                concentration: 0.0,
                rows: 0,
                cols: 0,
            });
            n += 1;
        }
        n
    }

    pub fn wells(&self, plate: u64) -> Vec<&Container> {
        self.containers
            .iter()
            .filter(|c| c.parent == Some(plate) && c.position.is_some())
            .collect()
    }

    /// Every container holding a given entity (contents-grouped lookup).
    pub fn find_by_entity(&self, entity: &str) -> Vec<&Container> {
        self.containers
            .iter()
            .filter(|c| c.entity.as_deref() == Some(entity))
            .collect()
    }

    /// Immutable snapshot of a plate's current wells.
    pub fn snapshot(&self, plate: u64, now: u64) -> Option<PlateSnapshot> {
        let p = self.get(plate)?;
        if p.kind != ContainerKind::Plate {
            return None;
        }
        Some(PlateSnapshot {
            plate_name: p.name.clone(),
            taken: now,
            wells: self
                .wells(plate)
                .iter()
                .map(|w| {
                    (
                        w.position.clone().unwrap_or_default(),
                        w.entity.clone(),
                        w.role,
                        w.quantity,
                    )
                })
                .collect(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hierarchy_and_lookup() {
        let mut inv = Inventory::default();
        let freezer = inv.add("Freezer -80", ContainerKind::Location, None);
        let boxid = inv.add("Box 1", ContainerKind::Box, Some(freezer));
        let tube = inv.add("Tube A", ContainerKind::Tube, Some(boxid));
        inv.set_contents(tube, Some("pUC19".into()), 50.0, 100.0);
        assert_eq!(inv.children(Some(freezer)).len(), 1);
        assert_eq!(inv.find_by_entity("pUC19").len(), 1);
        assert_eq!(inv.find_by_entity("pUC19")[0].id, tube);
    }

    #[test]
    fn fill_wells_and_snapshot() {
        let mut inv = Inventory::default();
        let plate = inv.add("Plate 1", ContainerKind::Plate, None);
        let n = inv.fill_wells(
            plate,
            &[
                ("A1".into(), Some("pUC19".into()), WellRole::Sample, 10.0),
                ("A2".into(), None, WellRole::Blank, 10.0),
            ],
        );
        assert_eq!(n, 2);
        assert_eq!(inv.wells(plate).len(), 2);
        let snap = inv.snapshot(plate, 42).unwrap();
        assert_eq!(snap.wells.len(), 2);
        // mutating live inventory does not change the snapshot
        inv.set_contents(inv.wells(plate)[0].id, Some("changed".into()), 0.0, 0.0);
        assert_eq!(snap.wells[0].1.as_deref(), Some("pUC19"));
    }

    #[test]
    fn place_moves_container() {
        let mut inv = Inventory::default();
        let a = inv.add("A", ContainerKind::Box, None);
        let b = inv.add("B", ContainerKind::Box, None);
        let t = inv.add("T", ContainerKind::Tube, Some(a));
        assert!(inv.place(t, Some(b), Some("C3".into())));
        assert_eq!(inv.get(t).unwrap().parent, Some(b));
        assert_eq!(inv.get(t).unwrap().position.as_deref(), Some("C3"));
    }
}
