//! Local workflows: assigned, trackable tasks with a status lifecycle, request
//! intake (draft → sent), and a simple flowchart with data passing + failed-halt.
//! Single-user / offline adaptation of coordinated lab work.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
    Skipped,
}

impl TaskStatus {
    pub fn label(self) -> &'static str {
        match self {
            TaskStatus::Pending => "Pending",
            TaskStatus::InProgress => "In progress",
            TaskStatus::Completed => "Completed",
            TaskStatus::Failed => "Failed",
            TaskStatus::Skipped => "Skipped",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Task {
    pub id: u64,
    pub title: String,
    pub schema: String,
    pub status: TaskStatus,
    pub assignee: String,
    pub inputs: BTreeMap<String, String>,
    pub outputs: BTreeMap<String, String>,
    pub created: u64,
    pub updated: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RequestStatus {
    Draft,
    Sent,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Request {
    pub id: u64,
    pub name: String,
    pub kind: String,
    pub status: RequestStatus,
    pub task_ids: Vec<u64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Stage {
    pub id: u64,
    pub name: String,
    pub task_id: u64,
    pub next: Vec<u64>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Workflows {
    pub tasks: Vec<Task>,
    pub requests: Vec<Request>,
    pub stages: Vec<Stage>,
    next_id: u64,
}

impl Workflows {
    fn nid(&mut self) -> u64 {
        self.next_id += 1;
        self.next_id
    }

    pub fn add_task(&mut self, title: &str, schema: &str, assignee: &str, now: u64) -> u64 {
        let id = self.nid();
        self.tasks.push(Task {
            id,
            title: title.into(),
            schema: schema.into(),
            status: TaskStatus::Pending,
            assignee: assignee.into(),
            inputs: BTreeMap::new(),
            outputs: BTreeMap::new(),
            created: now,
            updated: now,
        });
        id
    }

    pub fn task(&self, id: u64) -> Option<&Task> {
        self.tasks.iter().find(|t| t.id == id)
    }

    pub fn set_status(&mut self, id: u64, status: TaskStatus, now: u64) -> bool {
        if let Some(t) = self.tasks.iter_mut().find(|t| t.id == id) {
            t.status = status;
            t.updated = now;
            true
        } else {
            false
        }
    }

    pub fn submit_outputs(&mut self, id: u64, outputs: BTreeMap<String, String>, now: u64) -> bool {
        if let Some(t) = self.tasks.iter_mut().find(|t| t.id == id) {
            t.outputs = outputs;
            t.status = TaskStatus::Completed;
            t.updated = now;
            true
        } else {
            false
        }
    }

    pub fn my_tasks(&self, assignee: &str) -> Vec<&Task> {
        self.tasks
            .iter()
            .filter(|t| t.assignee == assignee)
            .collect()
    }

    pub fn add_request(&mut self, name: &str, kind: &str) -> u64 {
        let id = self.nid();
        self.requests.push(Request {
            id,
            name: name.into(),
            kind: kind.into(),
            status: RequestStatus::Draft,
            task_ids: Vec::new(),
        });
        id
    }

    /// Send a request: draft → sent (read-only), creating its tasks.
    pub fn send_request(&mut self, id: u64, task_titles: &[String], now: u64) -> bool {
        let kind = {
            let Some(r) = self.requests.iter_mut().find(|r| r.id == id) else {
                return false;
            };
            if r.status != RequestStatus::Draft {
                return false;
            }
            r.status = RequestStatus::Sent;
            r.kind.clone()
        };
        let mut ids = Vec::new();
        for t in task_titles {
            ids.push(self.add_task(t, &kind, "local", now));
        }
        if let Some(r) = self.requests.iter_mut().find(|r| r.id == id) {
            r.task_ids = ids;
        }
        true
    }

    pub fn add_stage(&mut self, name: &str, task_id: u64, next: Vec<u64>) -> u64 {
        let id = self.nid();
        self.stages.push(Stage {
            id,
            name: name.into(),
            task_id,
            next,
        });
        id
    }

    /// Next executable stage: a stage whose task is Pending and all upstream
    /// stages are Completed. Failed upstream halts progression (returns None).
    pub fn next_pending(&self) -> Option<u64> {
        for s in &self.stages {
            let Some(t) = self.task(s.task_id) else {
                continue;
            };
            if t.status == TaskStatus::Failed {
                return None; // failed halts
            }
            if t.status != TaskStatus::Pending {
                continue;
            }
            // upstream = stages that list s.id in next
            let upstream_ok = self
                .stages
                .iter()
                .filter(|u| u.next.contains(&s.id))
                .all(|u| {
                    self.task(u.task_id)
                        .map(|t| t.status == TaskStatus::Completed)
                        .unwrap_or(false)
                });
            if upstream_ok {
                return Some(s.id);
            }
        }
        None
    }

    /// Execute a flow step: copy upstream outputs into the stage task's inputs,
    /// mark InProgress.
    pub fn execute_step(&mut self, stage_id: u64, now: u64) -> bool {
        let Some(s) = self.stages.iter().find(|s| s.id == stage_id).cloned() else {
            return false;
        };
        let mut inputs = BTreeMap::new();
        for u in self.stages.iter().filter(|u| u.next.contains(&stage_id)) {
            if let Some(t) = self.task(u.task_id) {
                for (k, v) in &t.outputs {
                    inputs.insert(k.clone(), v.clone());
                }
            }
        }
        if let Some(t) = self.tasks.iter_mut().find(|t| t.id == s.task_id) {
            t.inputs = inputs;
            t.status = TaskStatus::InProgress;
            t.updated = now;
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task_lifecycle_and_outputs() {
        let mut w = Workflows::default();
        let t = w.add_task("PCR", "pcr", "local", 1);
        assert_eq!(w.task(t).unwrap().status, TaskStatus::Pending);
        let mut out = BTreeMap::new();
        out.insert("band".into(), "yes".into());
        assert!(w.submit_outputs(t, out, 2));
        assert_eq!(w.task(t).unwrap().status, TaskStatus::Completed);
        assert_eq!(w.my_tasks("local").len(), 1);
    }

    #[test]
    fn request_draft_to_sent_creates_tasks() {
        let mut w = Workflows::default();
        let r = w.add_request("Clone request", "cloning");
        assert!(w.send_request(r, &["Digest".into(), "Ligate".into()], 1));
        assert!(!w.send_request(r, &["x".into()], 2)); // already sent
        assert_eq!(w.requests[0].task_ids.len(), 2);
        assert_eq!(w.tasks.len(), 2);
    }

    #[test]
    fn flowchart_data_passing_and_failed_halt() {
        let mut w = Workflows::default();
        let t1 = w.add_task("Step1", "s", "local", 1);
        let t2 = w.add_task("Step2", "s", "local", 1);
        let s1 = w.add_stage("S1", t1, vec![]);
        let s2 = w.add_stage("S2", t2, vec![]);
        // link s1 -> s2
        w.stages
            .iter_mut()
            .find(|s| s.id == s1)
            .unwrap()
            .next
            .push(s2);
        assert_eq!(w.next_pending(), Some(s1));
        assert!(w.execute_step(s1, 2));
        let mut out = BTreeMap::new();
        out.insert("product".into(), "p1".into());
        w.submit_outputs(t1, out, 3);
        assert_eq!(w.next_pending(), Some(s2));
        assert!(w.execute_step(s2, 4));
        assert_eq!(
            w.task(t2)
                .unwrap()
                .inputs
                .get("product")
                .map(|s| s.as_str()),
            Some("p1")
        );
        // failed halt
        w.set_status(t2, TaskStatus::Failed, 5);
        assert_eq!(w.next_pending(), None);
    }
}
