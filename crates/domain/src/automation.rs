//! Automation-lite: local file-ingestion → transform → writeback pipelines with
//! reusable templates (draft → published) and run history + error states.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Step {
    IngestCsv,
    Filter { col: String, contains: String },
    Count,
    OutputRegistry,
    OutputResults,
}

impl Step {
    pub fn label(&self) -> String {
        match self {
            Step::IngestCsv => "Ingest CSV".into(),
            Step::Filter { col, contains } => format!("Filter {col} contains '{contains}'"),
            Step::Count => "Count rows".into(),
            Step::OutputRegistry => "Write to Registry".into(),
            Step::OutputResults => "Write to Results".into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Pipeline {
    pub id: u64,
    pub name: String,
    pub steps: Vec<Step>,
    pub published: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RunStatus {
    Completed,
    Failed,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Run {
    pub id: u64,
    pub pipeline: String,
    pub status: RunStatus,
    pub log: Vec<String>,
    pub columns: Vec<String>,
    pub output: Vec<Vec<String>>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Automation {
    pub pipelines: Vec<Pipeline>,
    pub runs: Vec<Run>,
    next_id: u64,
}

impl Automation {
    pub fn add_pipeline(&mut self, name: &str, steps: Vec<Step>) -> u64 {
        self.next_id += 1;
        let id = self.next_id;
        self.pipelines.push(Pipeline {
            id,
            name: name.into(),
            steps,
            published: false,
        });
        id
    }

    pub fn publish(&mut self, id: u64) -> bool {
        if let Some(p) = self.pipelines.iter_mut().find(|p| p.id == id) {
            p.published = true;
            true
        } else {
            false
        }
    }

    /// Execute a pipeline over CSV input. Pure transform; writeback targets are
    /// recorded in the log and returned rows (appcore performs the writeback).
    pub fn execute(&mut self, id: u64, input_csv: &str) -> Option<Run> {
        let p = self.pipelines.iter().find(|p| p.id == id)?.clone();
        let mut log = Vec::new();
        let mut columns: Vec<String> = Vec::new();
        let mut rows: Vec<Vec<String>> = Vec::new();
        let mut failed = false;
        for step in &p.steps {
            match step {
                Step::IngestCsv => {
                    let mut lines = input_csv.lines().filter(|l| !l.trim().is_empty());
                    if let Some(h) = lines.next() {
                        columns = h.split(',').map(|s| s.trim().to_string()).collect();
                        rows = lines
                            .map(|l| l.split(',').map(|s| s.trim().to_string()).collect())
                            .collect();
                        log.push(format!("Ingested {} rows", rows.len()));
                    } else {
                        failed = true;
                        log.push("Ingest failed: empty CSV".into());
                    }
                }
                Step::Filter { col, contains } => {
                    if let Some(idx) = columns.iter().position(|c| c == col) {
                        rows.retain(|r| {
                            r.get(idx)
                                .map(|v| v.contains(contains.as_str()))
                                .unwrap_or(false)
                        });
                        log.push(format!("Filter -> {} rows", rows.len()));
                    } else {
                        failed = true;
                        log.push(format!("Filter failed: no column {col}"));
                    }
                }
                Step::Count => {
                    log.push(format!("Count = {}", rows.len()));
                }
                Step::OutputRegistry | Step::OutputResults => {
                    log.push(format!("{}: {} rows staged", step.label(), rows.len()));
                }
            }
            if failed {
                break;
            }
        }
        self.next_id += 1;
        let run = Run {
            id: self.next_id,
            pipeline: p.name,
            status: if failed {
                RunStatus::Failed
            } else {
                RunStatus::Completed
            },
            log,
            columns,
            output: rows,
        };
        self.runs.push(run.clone());
        Some(run)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pipeline_ingest_filter_output() {
        let mut a = Automation::default();
        let id = a.add_pipeline(
            "p",
            vec![
                Step::IngestCsv,
                Step::Filter {
                    col: "Kind".into(),
                    contains: "Plasmid".into(),
                },
                Step::OutputRegistry,
            ],
        );
        assert!(!a.pipelines[0].published);
        assert!(a.publish(id));
        let run = a
            .execute(id, "Name,Kind\np1,Plasmid\ns1,Strain\np2,Plasmid\n")
            .unwrap();
        assert_eq!(run.status, RunStatus::Completed);
        assert_eq!(run.output.len(), 2);
        assert!(run.log.iter().any(|l| l.contains("Write to Registry")));
    }

    #[test]
    fn failed_filter_halts() {
        let mut a = Automation::default();
        let id = a.add_pipeline(
            "p",
            vec![
                Step::IngestCsv,
                Step::Filter {
                    col: "Nope".into(),
                    contains: "x".into(),
                },
                Step::Count,
            ],
        );
        let run = a.execute(id, "Name\na\n").unwrap();
        assert_eq!(run.status, RunStatus::Failed);
    }
}
