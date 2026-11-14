//! Append-only local audit log with CSV export (traceability pillar).

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuditEvent {
    pub ts: u64,
    pub actor: String,
    pub object: String,
    pub action: String,
    pub detail: String,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AuditLog {
    pub events: Vec<AuditEvent>,
}

impl AuditLog {
    pub fn append(&mut self, ts: u64, actor: &str, object: &str, action: &str, detail: &str) {
        self.events.push(AuditEvent {
            ts,
            actor: actor.into(),
            object: object.into(),
            action: action.into(),
            detail: detail.into(),
        });
    }

    pub fn for_object(&self, object: &str) -> Vec<&AuditEvent> {
        self.events.iter().filter(|e| e.object == object).collect()
    }

    pub fn to_csv(&self) -> String {
        let mut out = String::from("ts,actor,object,action,detail\n");
        for e in &self.events {
            out.push_str(&format!(
                "{},{},{},{},{}\n",
                e.ts,
                csv_escape(&e.actor),
                csv_escape(&e.object),
                csv_escape(&e.action),
                csv_escape(&e.detail)
            ));
        }
        out
    }
}

fn csv_escape(s: &str) -> String {
    if s.contains(',') || s.contains('"') || s.contains('\n') {
        format!("\"{}\"", s.replace('"', "\"\""))
    } else {
        s.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn append_and_filter() {
        let mut log = AuditLog::default();
        log.append(1, "alice", "entry:1", "created", "");
        log.append(2, "bob", "entry:2", "created", "");
        log.append(3, "alice", "entry:1", "approved", "ok");
        assert_eq!(log.for_object("entry:1").len(), 2);
        assert_eq!(log.events.len(), 3);
    }

    #[test]
    fn csv_escapes() {
        let mut log = AuditLog::default();
        log.append(1, "a,b", "obj", "act", "he said \"hi\"");
        let csv = log.to_csv();
        assert!(csv.contains("\"a,b\""));
        assert!(csv.contains("\"he said \"\"hi\"\"\""));
    }
}
