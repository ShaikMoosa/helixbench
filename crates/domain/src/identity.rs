//! Opaque identities. Cloning retains identity; creating a copy is explicit.
use serde::{Deserialize, Serialize};
use uuid::Uuid;

macro_rules! identity {
    ($name:ident) => {
        #[derive(
            Debug,
            Clone,
            Copy,
            Default,
            PartialEq,
            Eq,
            PartialOrd,
            Ord,
            Hash,
            Serialize,
            Deserialize,
        )]
        #[serde(transparent)]
        pub struct $name(Uuid);
        impl $name {
            pub fn new() -> Self {
                Self(Uuid::new_v4())
            }
            /// Reserved for deterministic legacy migration. Normal creation uses new().
            pub fn from_legacy_key(key: &[u8]) -> Self {
                let namespace = Uuid::new_v5(
                    &Uuid::NAMESPACE_URL,
                    concat!("helixbench/", stringify!($name)).as_bytes(),
                );
                Self(Uuid::new_v5(&namespace, key))
            }
            pub fn is_nil(self) -> bool {
                self.0.is_nil()
            }
        }
        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.0.fmt(f)
            }
        }
    };
}
identity!(DocumentId);
identity!(RevisionId);
identity!(ResultId);
identity!(OperationId);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceRef {
    pub document: DocumentId,
    pub revision: RevisionId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum StoreKind {
    Registry,
    Inventory,
    Notebook,
    Workflow,
    Biologics,
    Chemistry,
    Rna,
    InVivo,
    Bioprocess,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct RecordRef {
    pub store: StoreKind,
    pub local_id: u64,
}
