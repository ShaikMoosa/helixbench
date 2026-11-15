# Verified code-to-target gaps

## Import cleaning

storage::fasta::parse_all builds Document::from_sequence_text; clean_sequence uppercases supported characters, converts U to T and filters unsupported symbols. load_any uses parse_first for FASTA. Add interpretation/validation reporting before these mutation-producing paths; offer all records and never silently clean protein/RNA into DNA.

## Stable source identity

Document/Version do not currently supply durable IDs; Registry.linked_doc and Inventory.entity use names/strings. Figma source identity must become typed persisted data, not a UI-only string.

## Result lifecycle

Existing cached Analysis and saved alignments are not a general durable catalog for all tool results. Add a result store with source snapshots, method/parameters, stale-input comparison and refind routes. Preserve the existing reloadable alignment behavior.

## Dirty intent coverage

PendingAction currently covers CloseApp, NewDemo, NewEmpty, Open and OpenRecent. Extend the typed intent/return model for document close/switch and relevant replacement operations; preserve existing dirty history semantics.

## Notebook locking

Entry::locked currently locks Approved/Archived. The target requires a read-only submitted revision snapshot, not blanket mutation of the same in-review Entry data. Row submission is presently represented by a table-level submitted flag; target per-row outcomes need stable submitted-result identities.

## Registry union

Registry::Kind contains Plasmid, Strain and Primer. Figma protein/RNA rows require a typed federated view over the specialist stores or an explicit schema extension/migration, not arbitrary Kind indexes.

## Inventory transfer

Current Inventory has add/place/set_contents/fill_wells/snapshot but no validated transactional transfer API. fill_wells appends children without comprehensive collision/unit validation. Implement explicit reviewed assignment/transfer plans and snapshot-preserving commit behavior.

## Automation state

Current RunStatus is Completed/Failed and Run lacks a durable source/template revision/writeback ledger. Running/cancel/partial/recovery screens require new state and persistence, not only UI branches.

## Automation destination bug

Current appcore::automation_run registers rows with a Name column as plasmids after a Completed run and ignores save results. It does not adequately dispatch writeback according to OutputRegistry versus OutputResults. Add tests proving output-target isolation, truthful persistence success and idempotent recovery.

## CSV semantics

The inspected automation execution uses line and comma splitting. Replace ingestion with a quoting-aware parser at the storage/appcore boundary and pass a typed table into domain execution. Preserve supported existing pipelines and reject malformed input before writeback.

## Scoring claims

CRISPR code comments describe heuristic/Rule-Set-2-style scoring; chemistry explicitly supports a SMILES subset. Preserve computations but document actual method coverage and tests. Do not promote UI labels into claims of equivalence with validated external tools.

## Theme baseline

Existing DESIGN.md and theme.rs use older tokens, system typography and tighter layout. UX v4 uses Soft Gallery foundations and a contextual right panel. Centralize the new token mapping while retaining semantic feature colors and high-density scientific views.

## Prototype versus code

The 686-frame state catalog documents authored examples. A matching prototype link does not mean a schema, command, result store, file export, cancellation mechanism or authenticated reviewer already exists. Feature status in the traceability registry is explicitly scoped.

