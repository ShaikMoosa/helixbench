# Data, state and persistence contracts

## Current architecture

domain owns scientific/domain types; storage owns local formats and persistence; appcore owns application actions/state and orchestrates work; desktop_gpui renders and dispatches. Keep algorithms headlessly testable. Existing AppState has an active Document plus OpenDoc library entries and several independent local stores.

## Identity gaps

Current Document has name, sequence, features, primers and version snapshots but no durable document/revision ID. Registry links documents by name; Inventory links an entity using Option<String>. Introduce typed durable references and migrate name links only when resolution is unique. Preserve unresolved/ambiguous legacy links for user repair rather than selecting the first name match.

## Proposed identity types

Introduce DocumentId, RevisionId, ResultId and OperationId as UUID-backed opaque newtypes. Keep existing u64 IDs within their store and wrap cross-store references with a StoreKind discriminator. Display aliases such as DNA-001 are labels and fixtures, not primary keys. New copies get new identities; reopening the same native document preserves identity.

## Revision and result

AnalysisResult stores tool/method version, immutable typed input references/snapshots, captured selection/topology/units, validated parameters, timestamps, output payload, persistence status and optional current-source comparison. A draft analysis captures a durable draft snapshot; do not call it saved version 7 if it contains unsaved edits.

## Selection and route

SelectionContext is independent of the renderer. Route/ReturnLocation carry typed targets, queries and view state. A pending operation stores the exact original context; completion never reads a newly active document as its input.

## Native-file v2 migration

Read current v1 files, allocate durable IDs once during migration, retain feature/primer IDs, and migrate version snapshots to stable revision references. Save as native v2 only on explicit user save, with a recoverable original backup and a compatibility explanation. Unknown newer format versions are not overwritten. Preserve original input bytes until migration/interpretation review completes.

## Cross-store journal

Per-file atomic rename is not a multi-store transaction. For result submission, transfers and automation writeback, persist an operation plan with unique operation/output IDs and preconditions, write staged changes, atomically advance commit markers and reconcile incomplete journals at restart. Recovery must determine already committed outputs before retrying.

## Atomic save

Keep the existing atomic-write behavior but use operation-unique sibling temp names and serialize competing writes to the same destination. Flush/sync before replace. On failure, preserve the last good file, keep the document dirty and show a recoverable error. Propagate ignored storage errors back to the action result rather than showing success.

## Runtime operation states

Separate the existing domain RunStatus (Completed/Failed) from a richer operation controller: Reviewing, Queued, Running, CancelRequested, Canceled, Completed, Failed and PartialOutcome. Stage-specific cancellation must be truthful. A job works from a captured immutable input and dispatches completion to the UI through its operation ID.

## Dependency and tombstone policy

Archive is reversible and removes a record from active views. Permanent deletion blocks active live dependencies and historical results that cannot retain their inputs. Fully snapshotted historical results may retain an unavailable-source tombstone. Revalidate dependencies immediately before commit; after success clear active selection/actions and invalidate search caches.

## Local review revisions

Preserve current EntryStatus values and local ReviewEvent data. Add immutable submitted revision snapshots and distinguish author draft from the submitted revision. A review revision is read-only even though an independently edited draft may be allowed; never mutate a reviewer’s frozen input in place.

## Inventory migration

Existing quantities are f32 plus a unit string; concentration lacks an explicit unit. Normalize recognized quantity unit spellings, retain raw legacy values and mark unknown concentration units unresolved. Do not infer ng/µL or assign sample fixture values to real records. New transfer calculations use dimensioned validated values and explicit conversion.

## Scientific persistence

Retain outputs by source revision and method. Older results remain valid historical observations even if the current input changed; label them older-input, offer rerun with a new result ID and preserve the previous output. Pin source snapshots independently of the existing 20-checkpoint retention cap.

## Proposed interfaces

All names below are proposed contracts, not assertions about installed APIs. Introduce supporting types in the owning task and retain adapters for current call sites until migrated.

```rust
// PROPOSED interfaces; these do not exist in the baseline code.
pub struct SourceRef { pub document: DocumentId, pub revision: RevisionId }
pub struct RecordRef { pub store: StoreKind, pub local_id: u64 }
pub struct WellRef { pub plate_id: u64, pub coordinate: WellCoordinate }
pub struct SelectionContext {
    pub source: Option<SourceRef>, pub range: Option<domain::Range>,
    pub strand: domain::sequence::Strand, pub active_pane: PaneId,
    pub records: Vec<RecordRef>, pub wells: Vec<WellRef>,
}
pub struct ReturnLocation { pub route: Route, pub query: Option<SearchQuery>, pub view: ViewState }
pub enum NavigationIntent {
    Navigate(Route), OpenFile(std::path::PathBuf), SwitchDocument(DocumentId),
    CloseDocument(DocumentId), ExitApplication,
}
pub fn request_navigation(state: &mut AppState, intent: NavigationIntent) -> NavigationOutcome;
pub fn resolve_guard(state: &mut AppState, decision: GuardDecision) -> Result<NavigationOutcome, AppError>;
pub fn preview_import(source: ImportSource, options: ImportOptions) -> Result<ImportPreview, ImportError>;
pub fn commit_import(state: &mut AppState, plan: ReviewedImportPlan) -> Result<Vec<DocumentId>, AppError>;
pub fn dispatch_command(state: &mut AppState, id: CommandId, context: SelectionContext) -> Result<CommandOutcome, AppError>;
pub fn start_analysis(input: CapturedInput, parameters: AnalysisParameters) -> Result<OperationId, AppError>;
pub fn save_analysis(result: AnalysisResult, destination: ResultDestination) -> Result<ResultId, StorageError>;
pub fn plan_transfer(inventory: &Inventory, request: TransferRequest) -> Result<TransferPlan, Vec<FieldError>>;
pub fn commit_transfer(plan: ReviewedTransferPlan, stores: &mut WorkspaceStores) -> Result<OperationReceipt, AppError>;
pub fn reconcile_operation(id: OperationId, stores: &mut WorkspaceStores) -> Result<RecoveryPlan, AppError>;
```

## Proposed dependency policy

UUID v4/serde support is documented by the [uuid crate](https://docs.rs/uuid/1.29.2/uuid/). SHA-256 is documented by [RustCrypto sha2](https://docs.rs/sha2/0.11.1/sha2/). Proposed versions are uuid 1.29.2 with v4/serde and sha2 0.11.1; verify installed-toolchain/MSRV compatibility before adding them and commit the lockfile. Keep UI aliases separate from opaque IDs. A canonical versioned input digest includes algorithm-relevant sequence/type/topology, scope/strand, primer or chain data and ordered parameters; it never replaces the durable source revision reference.


## P08C native-format v4 extension

Document adds optional `pcr_source: PcrProvenance` and default-empty
`primer_pairs: Vec<PrimerPairProvenance>`. Pair records retain operation/source identity,
anchor/target/strand, actual primer IDs/sequences, body lengths, overhang, Tm and method.
PCR records retain operation/parent/pair identity, template length/topology/SHA-256,
resolved primer sequences/IDs, binding sites and Tm, body range, both tails, binding/Tm
parameters and copy policy. They are immutable historical input snapshots, not live
references whose sequences change when a primer is edited. Inspector marks edited pair
members as needing review. Floating conditions are serialized as readable strings so
Document retains exact equality; the scientific engines still use numeric parameters.

FORMAT_VERSION=4 prevents older version3 writers from silently deleting this metadata.
Version1–3 reads use defaults; normal upgrade makes an exact old-version backup before
atomic replacement. SourceRef is retained even when the parent is no longer open; the
UI never substitutes a similarly named/current revision. Product creation is an unsaved
new document. P04 owns actual disk save, errors and journal reconciliation; P09 owns the
future saved-result catalog. See screens/primers.md for copy policy and verified cases.


## P08D checkpoint and view-preset clarification

Named Version snapshots keep their scientific SourceRef. UI actions capture the exact
Version including label/time/content so same-revision names cannot retarget to another row.
Named lists survive transient Undo/Redo, and dirty calculation compares checkpoint metadata
against the saved native baseline as well as edit depth. Silent20checkpoint eviction is
removed; owned analysis InputSnapshot remains independent even if checkpoints are removed.
Restore copies snapshot content into a new current revision with Undo, preserving record ID,
named list and allocator. No new native-document fields or version bump are introduced.

Profile views.json writes envelope2 and optional typed enzyme_filter with name, cut count,
overhang, site length, scope and bounds. Legacy views without it reset non-name filters.
Atomic save/delete updates live Views only after persistence succeeds; native fields reseed
after Load. Source scientific documents and chosen enzymes do not change when presets load.
