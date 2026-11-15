//! Immutable, owned input snapshots are independent of document history retention.
use crate::{Document, OperationId, Range, ResultId, SourceRef};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InputSnapshot {
    source: SourceRef,
    document: Document,
    selection: Option<Range>,
    sha256: String,
}
impl InputSnapshot {
    pub fn capture(doc: &Document, selection: Option<Range>) -> Result<Self, String> {
        if doc.id.is_nil() || doc.revision_id.is_nil() {
            return Err("Input needs a durable identity".into());
        }
        if let Some(range) = selection {
            let len = doc.sequence.len();
            if range.len == 0
                || range.start >= len
                || range.len > len
                || (!doc.sequence.is_circular()
                    && range
                        .start
                        .checked_add(range.len)
                        .is_none_or(|end| end > len))
            {
                return Err("Selection is outside the input sequence".into());
            }
        }
        let mut document = doc.clone();
        document.versions.clear();
        let bytes = serde_json::to_vec(&(&document, selection)).map_err(|e| e.to_string())?;
        let sha256 = Sha256::digest(bytes)
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect();
        Ok(Self {
            source: doc.source_ref(),
            document,
            selection,
            sha256,
        })
    }
    pub fn source(&self) -> SourceRef {
        self.source
    }
    pub fn document(&self) -> &Document {
        &self.document
    }
    pub fn selection(&self) -> Option<Range> {
        self.selection
    }
    pub fn sha256(&self) -> &str {
        &self.sha256
    }
    pub fn validate(&self) -> Result<(), String> {
        let expected = Self::capture(&self.document, self.selection)?;
        if self.source != expected.source || self.sha256 != expected.sha256 {
            return Err("Input snapshot identity or digest does not match its content".into());
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnalysisResult {
    pub id: ResultId,
    pub operation: OperationId,
    pub tool: String,
    pub method_version: String,
    pub created: u64,
    pub inputs: Vec<InputSnapshot>,
    pub parameters: serde_json::Value,
    pub output: serde_json::Value,
}
impl AnalysisResult {
    pub fn validate(&self) -> Result<(), String> {
        if self.id.is_nil()
            || self.operation.is_nil()
            || self.tool.trim().is_empty()
            || self.method_version.trim().is_empty()
            || self.inputs.is_empty()
        {
            return Err("Result needs identity, method and captured inputs".into());
        }
        for input in &self.inputs {
            input.validate()?;
        }
        Ok(())
    }
}
