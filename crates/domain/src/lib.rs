//! HelixBench domain: pure molecular-biology logic with no UI, OS, or network dependencies.

pub mod common_features;
pub mod demo;
pub mod digest;
pub mod document;
pub mod enzyme;
pub mod enzyme_db;
pub mod enzyme_set;
pub mod feature;
pub mod genetic_code;
pub mod iupac;
pub mod pcr;
pub mod primer;
pub mod primer_design;
pub mod search;
pub mod sequence;
pub mod translate;

pub use document::{ColorRegion, Document, StrandMask};
pub use enzyme::{CutSite, Enzyme, Overhang};
pub use enzyme_set::EnzymeSet;
pub use feature::{Direction, Feature, FEATURE_KINDS};
pub use genetic_code::{GeneticCode, CODES};
pub use primer::{BindingSite, HybridizationParams, Primer};
pub use sequence::{DnaSequence, Range, Strand, Topology};
pub use translate::Orf;
