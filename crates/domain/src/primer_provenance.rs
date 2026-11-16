//! Persisted primer-pair and PCR derivation metadata. UI coordinates remain 1-based;
//! these records store zero-based ranges and exact captured identities/sequences.
use crate::{OperationId, Range, SourceRef, Strand};
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrimerInput {
    pub id: Option<u64>,
    pub name: String,
    pub sequence: Vec<u8>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BindingProvenance {
    pub range: Range,
    pub strand: Strand,
    pub tm: String,
    pub matched_3prime: usize,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrimerPairProvenance {
    pub pair: OperationId,
    pub source: SourceRef,
    pub template_length: usize,
    pub anchor_strand: Strand,
    pub forward_body: Range,
    pub reverse_body: Range,
    pub method: String,
    pub anchor: Option<Range>,
    pub target: Range,
    pub target_tm: String,
    pub overhang: String,
    pub forward: PrimerInput,
    pub reverse: PrimerInput,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PcrProvenance {
    pub operation: OperationId,
    pub parent: SourceRef,
    pub template_length: usize,
    pub template_topology: crate::Topology,
    pub template_sha256: String,
    pub pair: Option<OperationId>,
    pub forward: PrimerInput,
    pub reverse: PrimerInput,
    pub forward_site: BindingProvenance,
    pub reverse_site: BindingProvenance,
    pub template_range: Range,
    pub forward_tail: Vec<u8>,
    pub reverse_tail: Vec<u8>,
    pub min_3prime_match: usize,
    pub minimum_tm: String,
    pub salt_mm: String,
    pub primer_nm: String,
    pub copy_features: bool,
}
