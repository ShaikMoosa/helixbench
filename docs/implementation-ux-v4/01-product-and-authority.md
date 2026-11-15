# Product scope and implementation authority

## Product

HelixBench is a local-first, offline Windows molecular-biology workbench in Rust. The existing native GPUI shell renders a framework-independent appcore over domain and storage. Primary users import, inspect, edit and analyze sequences; lab records connect scientific identity, experimental notes and physical samples.

## Implementation authority

The user selected UX v4 as the target while preserving existing scientific capabilities. Apply this specification as the interaction contract, Figma as visual/flow evidence, and the inspected code as the current behavior/API baseline. Existing DESIGN.md describes the older palette/type/layout and must be reconciled in the implementation PR rather than silently treated as the new visual authority.

## Read in order

Start with README.md, this chapter, shared contracts, data/migrations, screen chapters and the implementation plan. Before each task read its feature IDs, relevant state-machine transitions, current code references and acceptance cases. Update the traceability registry and tests with every completed slice.

## State versus screen

There are 19 primary module pages and 686 Figma root frames in the captured v4 page. Frames include forms, reference material, duplicate responsive variants and scenario-specific states. Do not create 686 production routes or hardcode the text/IDs/numbers in these examples. Use typed state, a shared shell and reusable renderers.

## Local review and requests

Existing review events have actor strings and local statuses. A local review/request transition is not authenticated remote delivery. Use local workflow copy until a separately scoped service, account model, permissions and delivery protocol exist. Reviewer A is a prototype persona.

## Baseline

Repository revision: 0474cb94e456094dc4e501a9351ca7587a149361. The working tree was clean at inspection. The reference index captures 82 Rust files, 35,137 Rust lines, 1,133 function declarations, 218 struct/enum declarations and 181 test functions. These are static inspection counts; no application or scientific test suite was executed for this documentation task.

## Completeness boundary

The bundle catalogs the existing curated 212 feature entries, 25 explicit UX additions, all 19 primary screens, all captured Figma frame content/actions and the inspected source API/types. Runtime scientific validity, keyboard/assistive-technology behavior, persistence fault handling and user task performance require implementation/testing. Prototype links are not proof of these behaviors.

## Decision register

### DEC-01 · Authority

**Status:** Accepted by user

UX v4 defines target interaction and visual behavior. Existing scientific capabilities are preserved; code/schema evidence controls what currently exists. Prototype fixtures never define actual numerical results.

### DEC-02 · Runtime

**Status:** Baseline constraint

Keep the Rust 2021 workspace, native GPUI shell and domain/appcore/storage boundaries. Do not replace the UI with a web shell or egui.

### DEC-03 · Local workflow

**Status:** Target default from existing product

Remain local-first, offline and account-free. Review/request/audience states are local workflow records. Real remote delivery, authentication and multi-user authorization require a separate approved scope.

### DEC-04 · Molecular typing

**Status:** Target policy

Keep DnaSequence as DNA. Route RNA/oligo and protein inputs through explicit typed representations/module handoffs. Until a molecule can be represented safely, block its commit with original input retained; never clean protein into DNA.

### DEC-05 · Stable identity

**Status:** Proposed engineering decision

Add UUID-backed document/revision/result/operation identities while preserving existing store-local u64 keys. Persist identities once; importing as a copy remints document identity and retains original provenance.

### DEC-06 · Deletion

**Status:** Target policy resolving example ambiguity

Block active live dependencies and historical outputs lacking an immutable input snapshot. A historical result with complete provenance may retain a tombstone after source deletion. Archive is the safe default.

### DEC-07 · Version retention

**Status:** Target policy

Retain the current checkpoint behavior for unreferenced history, but pin or separately snapshot every source used by a retained result. The current 20-checkpoint cap cannot break provenance.

### DEC-08 · Operation semantics

**Status:** Target policy

Compute from an immutable captured input; validate before writeback; persist a recoverable operation journal for cross-store mutations. Offer cancellation only at a supported stage.

### DEC-09 · Two-click measure

**Status:** Accepted by user

Feature entry is within two navigation clicks from its owning page. Labeled page navigation is one click. Data entry, OS file picking and necessary safety confirmation are measured separately.

### DEC-10 · Presentation

**Status:** Target policy

Use one persistent shell, contextual right panel, modal decisions for consequential operations and inline/toast completion for routine actions. Figma feedback-content frames are not instructions to build hundreds of blocking modals.

### DEC-11 · Schema compatibility

**Status:** Proposed engineering decision

Introduce native schema v2 for durable identities/provenance. Read v1, migrate in memory and save only on explicit user action, with a recoverable backup. Newer unsupported versions remain read-protected and explain the required app update.

### DEC-12 · Scientific claims

**Status:** Release gate

Use engine-specific method names and disclose implemented limitations. CRISPR scores are local Rule-Set-2-style/CFD implementations, local BLAST is a seed-and-extend search, and chemistry supports a SMILES subset. Independent method validation precedes stronger scientific claims.

