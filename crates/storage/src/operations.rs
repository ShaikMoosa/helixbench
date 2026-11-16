//! Versioned, per-operation journals. Startup inspection never replays writes.
//! A completed receipt discards its captured payload to bound retained disk usage.
use crate::{Result, StorageError};
use domain::{
    operation::{OperationReceipt, OperationState},
    Document, OperationId,
};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
    sync::Mutex,
};

// Serialize the load/check/commit/receipt sequence across app workers in this process.
static OPERATIONS: Mutex<()> = Mutex::new(());

#[derive(Debug, Clone, Serialize, Deserialize)]
enum Output {
    Native,
    Text(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WritePlan {
    id: OperationId,
    document: Document,
    destination: PathBuf,
    before: Option<String>,
    expected: String,
    output: Output,
}
impl WritePlan {
    pub fn id(&self) -> OperationId {
        self.id
    }
    pub fn document(&self) -> &Document {
        &self.document
    }
    pub fn destination(&self) -> &Path {
        &self.destination
    }
    pub fn matches_native_receipt(&self, receipt: &OperationReceipt) -> bool {
        matches!(self.output, Output::Native)
            && receipt.state == OperationState::Completed
            && receipt.id == self.id
            && receipt.source == self.document.source_ref()
            && receipt.destination == self.destination
            && receipt.sha256 == self.expected
    }
    fn bytes(&self) -> Result<Vec<u8>> {
        match &self.output {
            Output::Native => crate::document_bytes(&self.document),
            Output::Text(text) => Ok(text.as_bytes().to_vec()),
        }
    }
    fn receipt(&self, state: OperationState) -> OperationReceipt {
        OperationReceipt {
            id: self.id,
            source: self.document.source_ref(),
            destination: self.destination.clone(),
            sha256: self.expected.clone(),
            state,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Record {
    version: u32,
    id: OperationId,
    state: OperationState,
    plan: Option<WritePlan>,
    receipt: Option<OperationReceipt>,
    error: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoveryDisposition {
    Committed,
    RetryAvailable,
    Conflict,
}
#[derive(Debug, Clone)]
pub struct RecoveryItem {
    pub id: OperationId,
    pub destination: PathBuf,
    pub disposition: RecoveryDisposition,
}
#[derive(Debug, Default)]
pub struct RecoveryPlan {
    pub items: Vec<RecoveryItem>,
    pub errors: Vec<String>,
}
#[derive(Debug, Clone)]
pub struct Journal {
    root: PathBuf,
}
impl Default for Journal {
    fn default() -> Self {
        Self::new(crate::config::config_dir().join("operations"))
    }
}
impl Journal {
    pub fn new(root: PathBuf) -> Self {
        Self { root }
    }
    fn path(&self, id: OperationId) -> PathBuf {
        self.root.join(format!("{id}.json"))
    }
    fn store(&self, record: &Record) -> Result<()> {
        crate::atomic_write(&self.path(record.id), &serde_json::to_vec(record)?)
    }
    fn load(&self, id: OperationId) -> Result<Record> {
        let bytes = fs::read(self.path(id))?;
        let header: serde_json::Value = serde_json::from_slice(&bytes)?;
        if header.get("version").and_then(|v| v.as_u64()) != Some(1) {
            return Err(StorageError::Format(
                "Unsupported operation journal version; keep the file and update HelixBench".into(),
            ));
        }
        let record: Record = serde_json::from_slice(&bytes)?;
        if record.id != id || id.is_nil() {
            return Err(StorageError::Format("Operation identity mismatch".into()));
        }
        if let Some(plan) = &record.plan {
            crate::migrations::validate_v2(&plan.document)?;
            if plan.id != id
                || !plan.destination.is_absolute()
                || crate::content_hash(&plan.bytes()?) != plan.expected
            {
                return Err(StorageError::Format(
                    "Invalid captured operation input".into(),
                ));
            }
        }
        match record.state {
            OperationState::Completed => {
                let receipt = record
                    .receipt
                    .as_ref()
                    .ok_or_else(|| StorageError::Format("Missing completion receipt".into()))?;
                if receipt.id != id
                    || receipt.state != OperationState::Completed
                    || !receipt.destination.is_absolute()
                    || receipt.source.document.is_nil()
                    || receipt.source.revision.is_nil()
                {
                    return Err(StorageError::Format("Invalid completion receipt".into()));
                }
            }
            OperationState::Canceled => {}
            _ if record.plan.is_none() => {
                return Err(StorageError::Format(
                    "Missing captured operation input".into(),
                ))
            }
            _ => {}
        }
        Ok(record)
    }
    fn prepare(
        &self,
        doc: &Document,
        path: &Path,
        output: Output,
        require_new: bool,
    ) -> Result<WritePlan> {
        let _lock = OPERATIONS
            .lock()
            .map_err(|_| StorageError::Format("Operation controller is unavailable".into()))?;
        crate::migrations::validate_v2(doc)?;
        let destination = std::path::absolute(path)?;
        let before = crate::file_hash(&destination)?;
        if require_new && before.is_some() {
            return Err(StorageError::Format(
                "Choose a new file for the recovery copy".into(),
            ));
        }
        let mut plan = WritePlan {
            id: OperationId::new(),
            document: doc.clone(),
            destination,
            before,
            expected: String::new(),
            output,
        };
        plan.expected = crate::content_hash(&plan.bytes()?);
        self.store(&Record {
            version: 1,
            id: plan.id,
            state: OperationState::Queued,
            plan: Some(plan.clone()),
            receipt: None,
            error: None,
        })?;
        Ok(plan)
    }
    pub fn prepare_native(&self, doc: &Document, path: &Path) -> Result<WritePlan> {
        self.prepare(doc, path, Output::Native, false)
    }
    pub fn prepare_text(&self, doc: &Document, path: &Path, text: String) -> Result<WritePlan> {
        self.prepare(doc, path, Output::Text(text), false)
    }
    fn disposition(plan: &WritePlan) -> Result<RecoveryDisposition> {
        let current = crate::file_hash(&plan.destination)?;
        Ok(if current.as_deref() == Some(&plan.expected) {
            RecoveryDisposition::Committed
        } else if current == plan.before {
            RecoveryDisposition::RetryAvailable
        } else {
            RecoveryDisposition::Conflict
        })
    }
    /// Retry/reconcile only on explicit request. Repeated completion returns its receipt.
    pub fn execute(&self, id: OperationId) -> Result<OperationReceipt> {
        let _lock = OPERATIONS
            .lock()
            .map_err(|_| StorageError::Format("Operation controller is unavailable".into()))?;
        let mut record = self.load(id)?;
        if record.state == OperationState::Canceled {
            return Err(StorageError::Format("Operation was canceled".into()));
        }
        if let Some(receipt) = &record.receipt {
            if record.state == OperationState::Completed {
                return Ok(receipt.clone());
            }
        }
        let plan = record
            .plan
            .as_ref()
            .ok_or_else(|| StorageError::Format("Missing captured input".into()))?
            .clone();
        let disposition = Self::disposition(&plan)?;
        if disposition == RecoveryDisposition::Conflict {
            return Err(StorageError::Format("Destination changed. Keep the current file or recover the captured input to a new file".into()));
        }
        if disposition != RecoveryDisposition::Committed {
            record.state = OperationState::Running;
            record.error = None;
            self.store(&record)?;
            let result = match &plan.output {
                Output::Native => {
                    crate::save_document_checked(&plan.document, &plan.destination, &plan.before)
                }
                Output::Text(text) => {
                    crate::atomic_write_checked(&plan.destination, text.as_bytes(), &plan.before)
                }
            };
            if let Err(error) = result {
                record.state = OperationState::Failed;
                record.error = Some(error.to_string());
                if let Err(journal_error) = self.store(&record) {
                    return Err(StorageError::Format(format!(
                        "Write failed: {error}; journal update also failed: {journal_error}"
                    )));
                }
                return Err(error);
            }
        }
        Ok(self.complete(record, &plan))
    }
    fn complete(&self, mut record: Record, plan: &WritePlan) -> OperationReceipt {
        let receipt = plan.receipt(OperationState::Completed);
        record.state = OperationState::Completed;
        record.receipt = Some(receipt.clone());
        record.plan = None;
        record.error = None;
        if self.store(&record).is_err() {
            // Output exists. Never report an unqualified success or blindly write it again.
            return plan.receipt(OperationState::PartialOutcome);
        }
        receipt
    }
    /// Cancel a queued/failed write, or dismiss a conflict without touching its output.
    pub fn cancel(&self, id: OperationId) -> Result<()> {
        let _lock = OPERATIONS
            .lock()
            .map_err(|_| StorageError::Format("Operation controller is unavailable".into()))?;
        let mut record = self.load(id)?;
        if record.state == OperationState::Completed {
            return Err(StorageError::Format(
                "Output is already committed; it cannot be canceled".into(),
            ));
        }
        if let Some(plan) = &record.plan {
            if Self::disposition(plan)? == RecoveryDisposition::Committed {
                return Err(StorageError::Format(
                    "Output is already committed; reconcile it instead".into(),
                ));
            }
        }
        record.state = OperationState::Canceled;
        record.plan = None;
        self.store(&record)
    }
    /// Preserve the original conflict and create a separately identified recovery copy.
    pub fn recover_copy(&self, id: OperationId, destination: &Path) -> Result<OperationReceipt> {
        let record = self.load(id)?;
        let plan = record
            .plan
            .ok_or_else(|| StorageError::Format("No captured input to recover".into()))?;
        if destination.exists() {
            return Err(StorageError::Format(
                "Choose a new file for the recovery copy".into(),
            ));
        }
        let copy = self.prepare(&plan.document, destination, plan.output, true)?;
        self.execute(copy.id)
    }
    pub fn recovery(&self) -> RecoveryPlan {
        let mut report = RecoveryPlan::default();
        let entries = match fs::read_dir(&self.root) {
            Ok(entries) => entries,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return report,
            Err(e) => {
                report.errors.push(format!(
                    "Cannot read operation journal {}: {e}",
                    self.root.display()
                ));
                return report;
            }
        };
        for entry in entries {
            let item = (|| -> Result<Option<RecoveryItem>> {
                let path = entry?.path();
                if path.extension().is_none_or(|ext| ext != "json") {
                    return Ok(None);
                }
                let name = path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .ok_or_else(|| StorageError::Format("Invalid journal filename".into()))?;
                let id: OperationId =
                    serde_json::from_value(serde_json::Value::String(name.into()))?;
                let record = self.load(id)?;
                if matches!(
                    record.state,
                    OperationState::Completed | OperationState::Canceled
                ) {
                    return Ok(None);
                }
                let plan = record.plan.unwrap(); // validated by load
                Ok(Some(RecoveryItem {
                    id,
                    destination: plan.destination.clone(),
                    disposition: Self::disposition(&plan)?,
                }))
            })();
            match item {
                Ok(Some(item)) => report.items.push(item),
                Ok(None) => {}
                Err(error) => report
                    .errors
                    .push(format!("{}: {error}", self.root.display())),
            }
        }
        report.items.sort_by_key(|item| item.id);
        report
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn completion_marker_failure_returns_partial_and_restart_reconciles() {
        let root = std::env::temp_dir().join(format!("helixbench-marker-{}", OperationId::new()));
        let journal = Journal::new(root.join("journal"));
        let doc = Document::new(
            "captured".into(),
            b"ATGC".to_vec(),
            domain::Topology::Linear,
        );
        let output = root.join("output.hbx");
        let plan = journal.prepare_native(&doc, &output).unwrap();
        let record = journal.load(plan.id()).unwrap();
        crate::save_document(&doc, &output).unwrap();
        let marker = journal.path(plan.id());
        let retained = marker.with_extension("retained");
        fs::rename(&marker, &retained).unwrap();
        fs::create_dir(&marker).unwrap();
        let receipt = journal.complete(record, &plan);
        assert_eq!(receipt.state, OperationState::PartialOutcome);
        assert_eq!(crate::load_document(&output).unwrap(), doc);
        fs::remove_dir(&marker).unwrap();
        fs::rename(retained, marker).unwrap();
        let restarted = Journal::new(root.join("journal"));
        assert_eq!(
            restarted.recovery().items[0].disposition,
            RecoveryDisposition::Committed
        );
        assert_eq!(
            restarted.execute(plan.id()).unwrap().state,
            OperationState::Completed
        );
    }
}
