//! Bioprocess (ISA-88-lite): recipes as unit-operation sequences, run planning
//! (draft → executing → completed/aborted), operator worksheets, replicates with
//! conditions/titer, and cross-run comparison. Pure Rust, offline, local-first.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OpKind {
    Inoculate,
    Feed,
    Induce,
    Harvest,
    Clarify,
    Purify,
}

impl OpKind {
    pub const ALL: [OpKind; 6] = [
        OpKind::Inoculate,
        OpKind::Feed,
        OpKind::Induce,
        OpKind::Harvest,
        OpKind::Clarify,
        OpKind::Purify,
    ];
    pub fn label(self) -> &'static str {
        match self {
            OpKind::Inoculate => "Inoculate",
            OpKind::Feed => "Feed",
            OpKind::Induce => "Induce",
            OpKind::Harvest => "Harvest",
            OpKind::Clarify => "Clarify",
            OpKind::Purify => "Purify",
        }
    }
}

/// A unit operation (ISA-88 operation/phase level) with parameters + conditions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnitOp {
    pub name: String,
    pub kind: OpKind,
    pub duration_h: f32,
    pub params: BTreeMap<String, String>,
    pub done: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Recipe {
    pub id: u64,
    pub name: String,
    pub product: String,
    pub ops: Vec<UnitOp>,
}

impl Recipe {
    /// A default upstream→downstream template.
    pub fn template(id: u64, name: &str, product: &str) -> Recipe {
        let mk = |n: &str, k: OpKind, h: f32| UnitOp {
            name: n.into(),
            kind: k,
            duration_h: h,
            params: BTreeMap::new(),
            done: false,
        };
        Recipe {
            id,
            name: name.into(),
            product: product.into(),
            ops: vec![
                mk("Inoculate", OpKind::Inoculate, 0.5),
                mk("Feed / grow", OpKind::Feed, 48.0),
                mk("Induce", OpKind::Induce, 0.5),
                mk("Harvest", OpKind::Harvest, 2.0),
                mk("Clarify", OpKind::Clarify, 1.0),
                mk("Purify", OpKind::Purify, 4.0),
            ],
        }
    }

    pub fn total_duration_h(&self) -> f32 {
        self.ops.iter().map(|o| o.duration_h).sum()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RunStatus {
    Draft,
    Executing,
    Completed,
    Aborted,
}

impl RunStatus {
    pub fn label(self) -> &'static str {
        match self {
            RunStatus::Draft => "Draft",
            RunStatus::Executing => "Executing",
            RunStatus::Completed => "Completed",
            RunStatus::Aborted => "Aborted",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Replicate {
    pub id: u64,
    pub label: String,
    pub conditions: BTreeMap<String, String>,
    pub titer_mg_l: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Run {
    pub id: u64,
    pub recipe: u64,
    pub name: String,
    pub status: RunStatus,
    pub ops: Vec<UnitOp>,
    pub replicates: Vec<Replicate>,
    pub created: u64,
}

impl Run {
    pub fn from_recipe(id: u64, r: &Recipe, name: &str, now: u64) -> Run {
        Run {
            id,
            recipe: r.id,
            name: name.into(),
            status: RunStatus::Draft,
            ops: r.ops.clone(),
            replicates: Vec::new(),
            created: now,
        }
    }

    /// draft → executing.
    pub fn start(&mut self) -> bool {
        if self.status != RunStatus::Draft {
            return false;
        }
        self.status = RunStatus::Executing;
        true
    }

    /// executing → completed (all ops done).
    pub fn complete(&mut self) -> bool {
        if self.status != RunStatus::Executing {
            return false;
        }
        self.status = RunStatus::Completed;
        true
    }

    pub fn abort(&mut self) -> bool {
        if matches!(self.status, RunStatus::Completed | RunStatus::Aborted) {
            return false;
        }
        self.status = RunStatus::Aborted;
        true
    }

    pub fn progress(&self) -> f32 {
        if self.ops.is_empty() {
            return 0.0;
        }
        self.ops.iter().filter(|o| o.done).count() as f32 / self.ops.len() as f32
    }

    pub fn mean_titer(&self) -> f32 {
        if self.replicates.is_empty() {
            return 0.0;
        }
        self.replicates.iter().map(|r| r.titer_mg_l).sum::<f32>() / self.replicates.len() as f32
    }
}

/// Cross-run mean-titer series for completed runs (for comparison charts).
pub fn titer_series(runs: &[Run]) -> Vec<(String, f32)> {
    runs.iter()
        .filter(|r| r.status == RunStatus::Completed && !r.replicates.is_empty())
        .map(|r| (r.name.clone(), r.mean_titer()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_lifecycle_and_progress() {
        let r = Recipe::template(1, "mAb upstream", "mAb");
        assert_eq!(r.ops.len(), 6);
        let mut run = Run::from_recipe(1, &r, "R1", 0);
        assert_eq!(run.status, RunStatus::Draft);
        assert!(!run.complete()); // not executing
        assert!(run.start());
        assert!(!run.start()); // already executing
        run.ops[0].done = true;
        run.ops[1].done = true;
        assert!((run.progress() - 2.0 / 6.0).abs() < 1e-6);
        assert!(run.complete());
        assert!(!run.abort()); // completed
    }

    #[test]
    fn titer_series_and_mean() {
        let r = Recipe::template(1, "p", "x");
        let mut run = Run::from_recipe(1, &r, "R1", 0);
        run.start();
        run.replicates.push(Replicate {
            id: 1,
            label: "A".into(),
            conditions: BTreeMap::new(),
            titer_mg_l: 100.0,
        });
        run.replicates.push(Replicate {
            id: 2,
            label: "B".into(),
            conditions: BTreeMap::new(),
            titer_mg_l: 200.0,
        });
        assert!((run.mean_titer() - 150.0).abs() < 1e-6);
        run.complete();
        let s = titer_series(&[run]);
        assert_eq!(s.len(), 1);
        assert!((s[0].1 - 150.0).abs() < 1e-6);
    }
}
