//! Durable write lifecycle. Completion describes persisted output, not UI selection.
use crate::{OperationId, SourceRef};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OperationState {
    Reviewing,
    Queued,
    Running,
    CancelRequested,
    Canceled,
    Completed,
    Failed,
    PartialOutcome,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OperationReceipt {
    pub id: OperationId,
    pub source: SourceRef,
    pub destination: PathBuf,
    pub sha256: String,
    pub state: OperationState,
}
