# HelixBench UX v4 Implementation Plan

> **For agentic workers:** Use superpowers:executing-plans to implement this plan task-by-task. Do not delegate without user authorization. Steps use checkbox tracking.

**Goal:** Implement UX v4 while preserving every existing scientific capability and local-data compatibility.

**Architecture:** Retain Rust domain → appcore/storage → GPUI boundaries. Add typed identity/context and durable operation/result contracts before wiring the new shell and module workflows. Algorithms remain headless and keep their regression fixtures.

**Tech Stack:** Rust 2021; GPUI 0.2.2 from the inspected workspace; serde/serde_json local versioned files.

**Spec:** This bundle README.md and chapters 01–09; canonical repository path docs/implementation-ux-v4/README.md.

## Global constraints

- Every command carries stable source/target IDs and relevant revision IDs. Never bind a mutation to a display name, row index or currently active record at completion time.
- UI coordinates are 1-based inclusive. Domain Range is zero-based start plus length, with documented circular wrap. Conversions occur at a typed boundary and are tested once.
- Keep original import/paste text and produce a transformation report before meaningful changes. Invalid input cannot silently disappear.
- A protein tool cannot choose a chain because DNA is active; a well cannot become A1 because H12 was clicked; no selection yields a prerequisite state.
- View switches retain the same draft. Destructive replacement, document close, file open and app close guard dirty data. Save failure/cancel never performs the pending intent.
- The UI reports saved/submitted/deleted only after the required persistence boundary succeeds. Partial/uncertain outcomes are not called success.
- Result, submitted-entry and plate-snapshot provenance is immutable. New source edits do not rewrite old outputs.
- Deleting a selection clears its inspector and actions. Filters/sorts retain the selected ID or explicitly clear it; they never select an unrelated row silently.
- Each job has an operation ID and captured inputs. Closing a view does not retarget its completion; stale results remain tied to their captured source.
- The local application does not claim remote delivery, authenticated review or cloud synchronization without an implemented service.
- Multi-record and cross-store actions validate a complete plan, commit predictably, and reconcile on restart without repeating successful writes.
- Visible labels, focus, keyboard navigation, readable scientific alternatives and announced status are part of the interaction, not optional decoration.

## Review focus

- Same-name/cross-store IDs must never retarget an action — P01/P02/P12.
- Lossy or wrong-molecule import must retain original input — P03.
- Ignored persistence failure or a retry must never create false success/duplicate output — P04/P13/P16.
- A source revision or submitted snapshot must not change behind a result/reviewer — P01/P09/P11.
- Keyboard/text editing, 1280 layout and large scientific data must remain operable — P07/P19.

## Suggested new-file responsibilities

- `crates/domain/src/identity.rs` — owned by P01: Stable identity, revision provenance and migrations.
- `crates/domain/src/results.rs` — owned by P01: Stable identity, revision provenance and migrations.
- `crates/storage/src/migrations.rs` — owned by P01: Stable identity, revision provenance and migrations.
- `crates/storage/src/results.rs` — owned by P01: Stable identity, revision provenance and migrations.
- `crates/appcore/src/navigation.rs` — owned by P02: Navigation intents, selection and dirty-work guards.
- `crates/appcore/src/import.rs` — owned by P03: Safe import interpretation and commit.
- `crates/domain/src/operation.rs` — owned by P04: Operation controller, persistence errors and feedback.
- `crates/appcore/src/operations.rs` — owned by P04: Operation controller, persistence errors and feedback.
- `crates/storage/src/operations.rs` — owned by P04: Operation controller, persistence errors and feedback.
- `crates/appcore/src/commands.rs` — owned by P05: Command registry and scoped workspace search.
- `crates/appcore/src/workspace_search.rs` — owned by P05: Command registry and scoped workspace search.
- `crates/appcore/src/first_use.rs` — owned by P06: First-use Home, samples and resume.
- `crates/appcore/src/results.rs` — owned by P09: Saved analysis result lifecycle.
- `apps/desktop_gpui/src/views/results.rs` — owned by P09: Saved analysis result lifecycle.
- `crates/appcore/src/record_refs.rs` — owned by P12: Typed Registry, relationships and deletion.
- `crates/appcore/src/inventory_ops.rs` — owned by P13: Inventory assignment, transfer and snapshots.
- `crates/storage/src/csv.rs` — owned by P16: Automation targets, journals and recovery.
- `crates/appcore/src/automation_ops.rs` — owned by P16: Automation targets, journals and recovery.

Tasks are ordered by dependency. Each task is a reviewable slice, not a promise of a fixed duration. Domain/API behavior precedes UI bindings. Proposed test names below are new tests to write; they are not baseline test results.

## P00 · Baseline and scientific regression inventory

**Goal:** Record the current command/test baseline and golden scientific fixtures without changing production behavior.

**Prerequisites:** None; read the baseline and authority chapters first.

**Files:**

- Modify/review: `crates/domain/src`
- Modify/review: `crates/storage/src`
- Modify/review: `crates/appcore/src`
- Modify/review: `apps/desktop_gpui/src`
- Create tests: `crates/appcore/tests/ux_v4_p00.rs`

**Interfaces:**

- Consumes: Inspected repository and specification evidence.
- Produces: Existing scientific fixtures, source manifest and recorded baseline results.

**Required tests:**

- `baseline_existing_suites`
- `legacy_native_round_trip`

- [ ] Step 1: Read the owning screen/feature/state contracts and the cited current APIs; record any newly discovered contradiction before changing scientific behavior.
- [ ] Step 2: Add the listed behavior tests and representative valid/invalid/cancel/stale-ID fixtures in the proposed test file. Confirm the new expectation fails for the intended reason; P00 instead records the existing baseline.
- [ ] Step 3: Run `cargo test -p domain`, `cargo test -p storage`, `cargo test -p appcore` and `cargo check -p desktop_gpui`; record actual baseline results. A new failing behavior test is not required for this read-only baseline task.
- [ ] Step 4: Implement the minimal domain/storage/appcore contract and error propagation before wiring its GPUI controls. Reuse the current algorithm entry points; no fixture ID or result value may become a production default.
- [ ] Step 5: Wire the UI states, prerequisites, labeled entry point and return context. Validate the success path plus the task-specific edge conditions and one persistence/cancel failure.
- [ ] Step 6: Re-run the focused tests, `cargo test -p appcore` and `cargo check -p desktop_gpui` for changed bindings; run formatting and relevant scientific regression suites. Record actual outputs and update feature status.
- [ ] Step 7: Review the diff against the acceptance IDs, stage only this task’s files and create a local checkpoint commit after checks pass. Publishing or external delivery follows the user’s authorization.

**Completion gate:** Existing scientific fixtures, source manifest and recorded baseline results. The listed tests pass and the owning feature traceability entries include implementation/test evidence.

## P01 · Stable identity, revision provenance and migrations

**Goal:** Introduce typed durable IDs, SourceRef and native v2 migration before UI routing depends on them. Verify dependency compatibility and MSRV before adding the proposed UUID and SHA-256 crates; wire normal module exports and serialization.

**Prerequisites:** P00.

**Files:**

- Modify/review: `Cargo.toml`
- Modify/review: `crates/domain/Cargo.toml`
- Modify/review: `crates/domain/src/document.rs`
- Modify/review: `crates/domain/src/registry.rs`
- Modify/review: `crates/domain/src/inventory.rs`
- Modify/review: `crates/storage/src/lib.rs`
- Modify/review: `crates/storage/src/records.rs`
- Create: `crates/domain/src/identity.rs`
- Create: `crates/domain/src/results.rs`
- Create: `crates/storage/src/migrations.rs`
- Create: `crates/storage/src/results.rs`
- Create tests: `crates/domain/tests/ux_v4_p01.rs`

**Interfaces:**

- Consumes: Existing scientific fixtures, source manifest and recorded baseline results.
- Produces: DocumentId / RevisionId / ResultId / OperationId; SourceRef; RecordRef; v1→v2 migration and immutable input snapshot serialization.

**Required tests:**

- `migration_v1_assigns_ids_once`
- `copy_remints_document_identity`
- `result_snapshot_survives_history_retention`

- [ ] Step 1: Read the owning screen/feature/state contracts and the cited current APIs; record any newly discovered contradiction before changing scientific behavior.
- [ ] Step 2: Add the listed behavior tests and representative valid/invalid/cancel/stale-ID fixtures in the proposed test file. Confirm the new expectation fails for the intended reason; P00 instead records the existing baseline.
- [ ] Step 3: Run `cargo test -p domain --test ux_v4_p01` and retain the initial result. Register the integration-test harness normally; do not claim an absent test file passed.
- [ ] Step 4: Implement the minimal domain/storage/appcore contract and error propagation before wiring its GPUI controls. Reuse the current algorithm entry points; no fixture ID or result value may become a production default.
- [ ] Step 5: Wire the UI states, prerequisites, labeled entry point and return context. Validate the success path plus the task-specific edge conditions and one persistence/cancel failure.
- [ ] Step 6: Re-run the focused tests, `cargo test -p domain` and `cargo check -p desktop_gpui` for changed bindings; run formatting and relevant scientific regression suites. Record actual outputs and update feature status.
- [ ] Step 7: Review the diff against the acceptance IDs, stage only this task’s files and create a local checkpoint commit after checks pass. Publishing or external delivery follows the user’s authorization.

**Completion gate:** DocumentId / RevisionId / ResultId / OperationId; SourceRef; RecordRef; v1→v2 migration and immutable input snapshot serialization. The listed tests pass and the owning feature traceability entries include implementation/test evidence.

## P02 · Navigation intents, selection and dirty-work guards

**Goal:** Implement typed Route, ReturnLocation, SelectionContext and a destination-aware pending-intent controller.

**Prerequisites:** P01.

**Files:**

- Modify/review: `crates/appcore/src/state.rs`
- Modify/review: `crates/appcore/src/actions.rs`
- Modify/review: `apps/desktop_gpui/src/app.rs`
- Create: `crates/appcore/src/navigation.rs`
- Create tests: `crates/appcore/tests/ux_v4_p02.rs`

**Interfaces:**

- Consumes: DocumentId / RevisionId / ResultId / OperationId; SourceRef; RecordRef; v1→v2 migration and immutable input snapshot serialization.
- Produces: Route, ReturnLocation, SelectionContext, NavigationIntent, request_navigation and resolve_guard.

**Required tests:**

- `guard_save_failure_retains_intent`
- `stay_preserves_invoking_view`
- `view_switch_retains_draft`
- `same_name_documents_do_not_swap`

- [ ] Step 1: Read the owning screen/feature/state contracts and the cited current APIs; record any newly discovered contradiction before changing scientific behavior.
- [ ] Step 2: Add the listed behavior tests and representative valid/invalid/cancel/stale-ID fixtures in the proposed test file. Confirm the new expectation fails for the intended reason; P00 instead records the existing baseline.
- [ ] Step 3: Run `cargo test -p appcore --test ux_v4_p02` and retain the initial result. Register the integration-test harness normally; do not claim an absent test file passed.
- [ ] Step 4: Implement the minimal domain/storage/appcore contract and error propagation before wiring its GPUI controls. Reuse the current algorithm entry points; no fixture ID or result value may become a production default.
- [ ] Step 5: Wire the UI states, prerequisites, labeled entry point and return context. Validate the success path plus the task-specific edge conditions and one persistence/cancel failure.
- [ ] Step 6: Re-run the focused tests, `cargo test -p appcore` and `cargo check -p desktop_gpui` for changed bindings; run formatting and relevant scientific regression suites. Record actual outputs and update feature status.
- [ ] Step 7: Review the diff against the acceptance IDs, stage only this task’s files and create a local checkpoint commit after checks pass. Publishing or external delivery follows the user’s authorization.

**Completion gate:** Route, ReturnLocation, SelectionContext, NavigationIntent, request_navigation and resolve_guard. The listed tests pass and the owning feature traceability entries include implementation/test evidence.

## P03 · Safe import interpretation and commit

**Goal:** Parse into an ImportPreview with original input and a transformation/error report, then commit only reviewed records.

**Prerequisites:** P01, P02.

**Files:**

- Modify/review: `crates/storage/src/fasta.rs`
- Modify/review: `crates/storage/src/genbank.rs`
- Modify/review: `crates/storage/src/lib.rs`
- Modify/review: `crates/appcore/src/actions.rs`
- Modify/review: `apps/desktop_gpui/src/dialogs.rs`
- Create: `crates/appcore/src/import.rs`
- Create tests: `crates/storage/tests/ux_v4_p03.rs`

**Interfaces:**

- Consumes: DocumentId / RevisionId / ResultId / OperationId; SourceRef; RecordRef; v1→v2 migration and immutable input snapshot serialization.; Route, ReturnLocation, SelectionContext, NavigationIntent, request_navigation and resolve_guard.
- Produces: ImportSource / ImportOptions / ImportPreview / ReviewedImportPlan; preview_import and commit_import.

**Required tests:**

- `import_reports_u_to_t`
- `protein_fasta_never_cleaned_as_dna`
- `multi_record_partial_import_retains_indices`
- `duplicate_import_copy_has_new_id`

- [ ] Step 1: Read the owning screen/feature/state contracts and the cited current APIs; record any newly discovered contradiction before changing scientific behavior.
- [ ] Step 2: Add the listed behavior tests and representative valid/invalid/cancel/stale-ID fixtures in the proposed test file. Confirm the new expectation fails for the intended reason; P00 instead records the existing baseline.
- [ ] Step 3: Run `cargo test -p storage --test ux_v4_p03` and retain the initial result. Register the integration-test harness normally; do not claim an absent test file passed.
- [ ] Step 4: Implement the minimal domain/storage/appcore contract and error propagation before wiring its GPUI controls. Reuse the current algorithm entry points; no fixture ID or result value may become a production default.
- [ ] Step 5: Wire the UI states, prerequisites, labeled entry point and return context. Validate the success path plus the task-specific edge conditions and one persistence/cancel failure.
- [ ] Step 6: Re-run the focused tests, `cargo test -p storage` and `cargo check -p desktop_gpui` for changed bindings; run formatting and relevant scientific regression suites. Record actual outputs and update feature status.
- [ ] Step 7: Review the diff against the acceptance IDs, stage only this task’s files and create a local checkpoint commit after checks pass. Publishing or external delivery follows the user’s authorization.

**Completion gate:** ImportSource / ImportOptions / ImportPreview / ReviewedImportPlan; preview_import and commit_import. The listed tests pass and the owning feature traceability entries include implementation/test evidence.

## P04 · Operation controller, persistence errors and feedback

**Goal:** Add durable operation state/journal orchestration and truthful, non-blocking routine feedback.

**Prerequisites:** P01, P02.

**Files:**

- Modify/review: `crates/appcore/src/actions.rs`
- Modify/review: `crates/appcore/src/state.rs`
- Modify/review: `crates/storage/src/lib.rs`
- Modify/review: `apps/desktop_gpui/src/widgets.rs`
- Create: `crates/domain/src/operation.rs`
- Create: `crates/appcore/src/operations.rs`
- Create: `crates/storage/src/operations.rs`
- Create tests: `crates/appcore/tests/ux_v4_p04.rs`

**Interfaces:**

- Consumes: DocumentId / RevisionId / ResultId / OperationId; SourceRef; RecordRef; v1→v2 migration and immutable input snapshot serialization.; Route, ReturnLocation, SelectionContext, NavigationIntent, request_navigation and resolve_guard.
- Produces: OperationState / OperationReceipt / RecoveryPlan; operation journal, completion dispatch and surfaced persistence errors.

**Required tests:**

- `save_rename_failure_is_not_success`
- `operation_uses_captured_input`
- `restart_reconciles_committed_output`
- `routine_success_does_not_block`

- [ ] Step 1: Read the owning screen/feature/state contracts and the cited current APIs; record any newly discovered contradiction before changing scientific behavior.
- [ ] Step 2: Add the listed behavior tests and representative valid/invalid/cancel/stale-ID fixtures in the proposed test file. Confirm the new expectation fails for the intended reason; P00 instead records the existing baseline.
- [ ] Step 3: Run `cargo test -p appcore --test ux_v4_p04` and retain the initial result. Register the integration-test harness normally; do not claim an absent test file passed.
- [ ] Step 4: Implement the minimal domain/storage/appcore contract and error propagation before wiring its GPUI controls. Reuse the current algorithm entry points; no fixture ID or result value may become a production default.
- [ ] Step 5: Wire the UI states, prerequisites, labeled entry point and return context. Validate the success path plus the task-specific edge conditions and one persistence/cancel failure.
- [ ] Step 6: Re-run the focused tests, `cargo test -p appcore` and `cargo check -p desktop_gpui` for changed bindings; run formatting and relevant scientific regression suites. Record actual outputs and update feature status.
- [ ] Step 7: Review the diff against the acceptance IDs, stage only this task’s files and create a local checkpoint commit after checks pass. Publishing or external delivery follows the user’s authorization.

**Completion gate:** OperationState / OperationReceipt / RecoveryPlan; operation journal, completion dispatch and surfaced persistence errors. The listed tests pass and the owning feature traceability entries include implementation/test evidence.

## P05 · Command registry and scoped workspace search

**Goal:** Centralize tool availability/context and implement typed scoped search independent of sequence Find.

**Prerequisites:** P02.

**Files:**

- Modify/review: `apps/desktop_gpui/src/app.rs`
- Modify/review: `apps/desktop_gpui/src/find.rs`
- Create: `crates/appcore/src/commands.rs`
- Create: `crates/appcore/src/workspace_search.rs`
- Create tests: `crates/appcore/tests/ux_v4_p05.rs`

**Interfaces:**

- Consumes: Route, ReturnLocation, SelectionContext, NavigationIntent, request_navigation and resolve_guard.
- Produces: CommandId / command descriptors / availability; SearchQuery and typed search result/return state.

**Required tests:**

- `owning_page_tool_entry_within_two_clicks`
- `search_back_restores_scope`
- `late_search_reply_is_ignored`
- `deleted_record_is_not_reopened`

- [ ] Step 1: Read the owning screen/feature/state contracts and the cited current APIs; record any newly discovered contradiction before changing scientific behavior.
- [ ] Step 2: Add the listed behavior tests and representative valid/invalid/cancel/stale-ID fixtures in the proposed test file. Confirm the new expectation fails for the intended reason; P00 instead records the existing baseline.
- [ ] Step 3: Run `cargo test -p appcore --test ux_v4_p05` and retain the initial result. Register the integration-test harness normally; do not claim an absent test file passed.
- [ ] Step 4: Implement the minimal domain/storage/appcore contract and error propagation before wiring its GPUI controls. Reuse the current algorithm entry points; no fixture ID or result value may become a production default.
- [ ] Step 5: Wire the UI states, prerequisites, labeled entry point and return context. Validate the success path plus the task-specific edge conditions and one persistence/cancel failure.
- [ ] Step 6: Re-run the focused tests, `cargo test -p appcore` and `cargo check -p desktop_gpui` for changed bindings; run formatting and relevant scientific regression suites. Record actual outputs and update feature status.
- [ ] Step 7: Review the diff against the acceptance IDs, stage only this task’s files and create a local checkpoint commit after checks pass. Publishing or external delivery follows the user’s authorization.

**Completion gate:** CommandId / command descriptors / availability; SearchQuery and typed search result/return state. The listed tests pass and the owning feature traceability entries include implementation/test evidence.

## P06 · First-use Home, samples and resume

**Goal:** Implement explicit Import/Try sample, isolated sample origin, typed recent-work resume and saved-result refind.

**Prerequisites:** P02, P03, P05.

**Files:**

- Modify/review: `apps/desktop_gpui/src/lab.rs`
- Modify/review: `crates/appcore/src/actions.rs`
- Modify/review: `crates/storage/src/config.rs`
- Create: `crates/appcore/src/first_use.rs`
- Create tests: `crates/appcore/tests/ux_v4_p06.rs`

**Interfaces:**

- Consumes: Route, ReturnLocation, SelectionContext, NavigationIntent, request_navigation and resolve_guard.; ImportSource / ImportOptions / ImportPreview / ReviewedImportPlan; preview_import and commit_import.; CommandId / command descriptors / availability; SearchQuery and typed search result/return state.
- Produces: First-use preference, sample origin and personal-copy policy; typed recent-work routes.

**Required tests:**

- `existing_workspace_not_replaced_by_demo`
- `sample_copy_is_personal_and_independent`
- `resume_opens_exact_entry_task_or_result`

- [ ] Step 1: Read the owning screen/feature/state contracts and the cited current APIs; record any newly discovered contradiction before changing scientific behavior.
- [ ] Step 2: Add the listed behavior tests and representative valid/invalid/cancel/stale-ID fixtures in the proposed test file. Confirm the new expectation fails for the intended reason; P00 instead records the existing baseline.
- [ ] Step 3: Run `cargo test -p appcore --test ux_v4_p06` and retain the initial result. Register the integration-test harness normally; do not claim an absent test file passed.
- [ ] Step 4: Implement the minimal domain/storage/appcore contract and error propagation before wiring its GPUI controls. Reuse the current algorithm entry points; no fixture ID or result value may become a production default.
- [ ] Step 5: Wire the UI states, prerequisites, labeled entry point and return context. Validate the success path plus the task-specific edge conditions and one persistence/cancel failure.
- [ ] Step 6: Re-run the focused tests, `cargo test -p appcore` and `cargo check -p desktop_gpui` for changed bindings; run formatting and relevant scientific regression suites. Record actual outputs and update feature status.
- [ ] Step 7: Review the diff against the acceptance IDs, stage only this task’s files and create a local checkpoint commit after checks pass. Publishing or external delivery follows the user’s authorization.

**Completion gate:** First-use preference, sample origin and personal-copy policy; typed recent-work routes. The listed tests pass and the owning feature traceability entries include implementation/test evidence.

## P07 · Soft Gallery shell and contextual tools

**Goal:** Apply central tokens/layout and the selected-context right panel while retaining a readable scientific workspace.

**Prerequisites:** P02, P05.

**Files:**

- Modify/review: `apps/desktop_gpui/src/theme.rs`
- Modify/review: `apps/desktop_gpui/src/widgets.rs`
- Modify/review: `apps/desktop_gpui/src/app.rs`
- Modify/review: `apps/desktop_gpui/src/editor.rs`
- Modify/review: `DESIGN.md`
- Create tests: `crates/appcore/tests/ux_v4_p07.rs`

**Interfaces:**

- Consumes: Route, ReturnLocation, SelectionContext, NavigationIntent, request_navigation and resolve_guard.; CommandId / command descriptors / availability; SearchQuery and typed search result/return state.
- Produces: Tokenized GPUI shell, contextual panel and reusable feedback/form/table surfaces.

**Required tests:**

- `panel_close_returns_focus`
- `header_actions_do_not_overflow_1280`
- `view_representation_does_not_change_topology`

- [ ] Step 1: Read the owning screen/feature/state contracts and the cited current APIs; record any newly discovered contradiction before changing scientific behavior.
- [ ] Step 2: Add the listed behavior tests and representative valid/invalid/cancel/stale-ID fixtures in the proposed test file. Confirm the new expectation fails for the intended reason; P00 instead records the existing baseline.
- [ ] Step 3: Run `cargo test -p appcore --test ux_v4_p07` and retain the initial result. Register the integration-test harness normally; do not claim an absent test file passed.
- [ ] Step 4: Implement the minimal domain/storage/appcore contract and error propagation before wiring its GPUI controls. Reuse the current algorithm entry points; no fixture ID or result value may become a production default.
- [ ] Step 5: Wire the UI states, prerequisites, labeled entry point and return context. Validate the success path plus the task-specific edge conditions and one persistence/cancel failure.
- [ ] Step 6: Re-run the focused tests, `cargo test -p appcore` and `cargo check -p desktop_gpui` for changed bindings; run formatting and relevant scientific regression suites. Record actual outputs and update feature status.
- [ ] Step 7: Review the diff against the acceptance IDs, stage only this task’s files and create a local checkpoint commit after checks pass. Publishing or external delivery follows the user’s authorization.

**Completion gate:** Tokenized GPUI shell, contextual panel and reusable feedback/form/table surfaces. The listed tests pass and the owning feature traceability entries include implementation/test evidence.

## P08A · Map, sequence and coordinate synchronization

**Goal:** Wire map/sequence views to typed selection and preserve rendering, zoom/minimap, clipboard, search, ORFs and translation.

**Prerequisites:** P02, P07.

**Files:**

- Modify/review: `apps/desktop_gpui/src/views/map.rs`
- Modify/review: `apps/desktop_gpui/src/views/sequence.rs`
- Modify/review: `apps/desktop_gpui/src/editor.rs`
- Modify/review: `crates/appcore/src/state.rs`
- Create tests: `crates/appcore/tests/ux_v4_p08a.rs`

**Interfaces:**

- Consumes: Route, ReturnLocation, SelectionContext, NavigationIntent, request_navigation and resolve_guard.; Tokenized GPUI shell, contextual panel and reusable feedback/form/table surfaces.
- Produces: Stable document/selection bindings and explicit coordinate conversions for Map/Sequence.

**Required tests:**

- `circular_wrap_selection_survives_view_switch`
- `reverse_strand_selection_keeps_coordinates`
- `display_toggle_does_not_mark_dirty`

- [ ] Step 1: Read the owning screen/feature/state contracts and the cited current APIs; record any newly discovered contradiction before changing scientific behavior.
- [ ] Step 2: Add the listed behavior tests and representative valid/invalid/cancel/stale-ID fixtures in the proposed test file. Confirm the new expectation fails for the intended reason; P00 instead records the existing baseline.
- [ ] Step 3: Run `cargo test -p appcore --test ux_v4_p08a` and retain the initial result. Register the integration-test harness normally; do not claim an absent test file passed.
- [ ] Step 4: Implement the minimal domain/storage/appcore contract and error propagation before wiring its GPUI controls. Reuse the current algorithm entry points; no fixture ID or result value may become a production default.
- [ ] Step 5: Wire the UI states, prerequisites, labeled entry point and return context. Validate the success path plus the task-specific edge conditions and one persistence/cancel failure.
- [ ] Step 6: Re-run the focused tests, `cargo test -p appcore` and `cargo check -p desktop_gpui` for changed bindings; run formatting and relevant scientific regression suites. Record actual outputs and update feature status.
- [ ] Step 7: Review the diff against the acceptance IDs, stage only this task’s files and create a local checkpoint commit after checks pass. Publishing or external delivery follows the user’s authorization.

**Completion gate:** Stable document/selection bindings and explicit coordinate conversions for Map/Sequence. The listed tests pass and the owning feature traceability entries include implementation/test evidence.

## P08B · Feature editing and annotation review

**Goal:** Preserve feature fields/segments and apply selected annotation proposals through validated document mutations.

**Prerequisites:** P08A.

**Files:**

- Modify/review: `crates/appcore/src/actions.rs`
- Modify/review: `apps/desktop_gpui/src/views/features.rs`
- Modify/review: `apps/desktop_gpui/src/dialogs.rs`
- Create tests: `crates/domain/tests/ux_v4_p08b.rs`

**Interfaces:**

- Consumes: Stable document/selection bindings and explicit coordinate conversions for Map/Sequence.
- Produces: Validated feature edit/detection/bulk-annotation commands preserving document/feature identities.

**Required tests:**

- `feature_edit_retains_id_after_sort`
- `annotation_preview_cancel_is_noop`
- `bulk_annotate_reports_per_document_outcome`

- [ ] Step 1: Read the owning screen/feature/state contracts and the cited current APIs; record any newly discovered contradiction before changing scientific behavior.
- [ ] Step 2: Add the listed behavior tests and representative valid/invalid/cancel/stale-ID fixtures in the proposed test file. Confirm the new expectation fails for the intended reason; P00 instead records the existing baseline.
- [ ] Step 3: Run `cargo test -p domain --test ux_v4_p08b` and retain the initial result. Register the integration-test harness normally; do not claim an absent test file passed.
- [ ] Step 4: Implement the minimal domain/storage/appcore contract and error propagation before wiring its GPUI controls. Reuse the current algorithm entry points; no fixture ID or result value may become a production default.
- [ ] Step 5: Wire the UI states, prerequisites, labeled entry point and return context. Validate the success path plus the task-specific edge conditions and one persistence/cancel failure.
- [ ] Step 6: Re-run the focused tests, `cargo test -p domain` and `cargo check -p desktop_gpui` for changed bindings; run formatting and relevant scientific regression suites. Record actual outputs and update feature status.
- [ ] Step 7: Review the diff against the acceptance IDs, stage only this task’s files and create a local checkpoint commit after checks pass. Publishing or external delivery follows the user’s authorization.

**Completion gate:** Validated feature edit/detection/bulk-annotation commands preserving document/feature identities. The listed tests pass and the owning feature traceability entries include implementation/test evidence.

## P08C · Primer design and PCR context

**Goal:** Bind actual template/pair/candidate identity and parameters to design, diagnostics and independent PCR product creation.

**Prerequisites:** P08A, P04.

**Files:**

- Modify/review: `crates/appcore/src/actions.rs`
- Modify/review: `apps/desktop_gpui/src/views/primers.rs`
- Modify/review: `apps/desktop_gpui/src/dialogs.rs`
- Create tests: `crates/appcore/tests/ux_v4_p08c.rs`

**Interfaces:**

- Consumes: Stable document/selection bindings and explicit coordinate conversions for Map/Sequence.; OperationState / OperationReceipt / RecoveryPlan; operation journal, completion dispatch and surfaced persistence errors.
- Produces: Captured primer/PCR requests and independent product provenance.

**Required tests:**

- `primer_target_is_distinct_from_anchor`
- `stale_candidate_cannot_be_added`
- `pcr_product_keeps_parent_provenance`

- [x] Step 1: Read the owning screen/feature/state contracts and the cited current APIs; record any newly discovered contradiction before changing scientific behavior.
- [x] Step 2: Add the listed behavior tests and representative valid/invalid/cancel/stale-ID fixtures in the proposed test file. Confirm the new expectation fails for the intended reason; P00 instead records the existing baseline.
- [x] Step 3: Run `cargo test -p appcore --test ux_v4_p08c` and retain the initial result. Register the integration-test harness normally; do not claim an absent test file passed.
- [x] Step 4: Implement the minimal domain/storage/appcore contract and error propagation before wiring its GPUI controls. Reuse the current algorithm entry points; no fixture ID or result value may become a production default.
- [x] Step 5: Wire the UI states, prerequisites, labeled entry point and return context. Validate the success path plus the task-specific edge conditions and one persistence/cancel failure.
- [x] Step 6: Re-run the focused tests, `cargo test -p appcore` and `cargo check -p desktop_gpui` for changed bindings; run formatting and relevant scientific regression suites. Record actual outputs and update feature status.
- [x] Step 7: Review the diff against the acceptance IDs, stage only this task’s files and create a local checkpoint commit after checks pass. Publishing or external delivery follows the user’s authorization.

**Completion gate:** Captured primer/PCR requests and independent product provenance. The listed tests pass and the owning feature traceability entries include implementation/test evidence.

## P08D · Enzyme filters, digest and history

**Goal:** Preserve composable enzyme filters, digest accounting, Undo/Redo and named-version compare/restore semantics.

**Prerequisites:** P08A, P04.

**Files:**

- Modify/review: `apps/desktop_gpui/src/views/enzymes.rs`
- Modify/review: `apps/desktop_gpui/src/views/history.rs`
- Modify/review: `crates/appcore/src/actions.rs`
- Create tests: `crates/appcore/tests/ux_v4_p08d.rs`

**Interfaces:**

- Consumes: Stable document/selection bindings and explicit coordinate conversions for Map/Sequence.; OperationState / OperationReceipt / RecoveryPlan; operation journal, completion dispatch and surfaced persistence errors.
- Produces: Validated enzyme scope/results and named-version/Undo distinction.

**Required tests:**

- `uncut_linear_input_is_completed_no_sites`
- `composed_filter_reset_restores_scope`
- `restore_cancel_leaves_current_revision`

- [x] Step 1: Read the owning screen/feature/state contracts and the cited current APIs; record any newly discovered contradiction before changing scientific behavior.
- [x] Step 2: Add the listed behavior tests and representative valid/invalid/cancel/stale-ID fixtures in the proposed test file. Confirm the new expectation fails for the intended reason; P00 instead records the existing baseline.
- [x] Step 3: Run `cargo test -p appcore --test ux_v4_p08d` and retain the initial result. Register the integration-test harness normally; do not claim an absent test file passed.
- [x] Step 4: Implement the minimal domain/storage/appcore contract and error propagation before wiring its GPUI controls. Reuse the current algorithm entry points; no fixture ID or result value may become a production default.
- [x] Step 5: Wire the UI states, prerequisites, labeled entry point and return context. Validate the success path plus the task-specific edge conditions and one persistence/cancel failure.
- [x] Step 6: Re-run the focused tests, `cargo test -p appcore` and `cargo check -p desktop_gpui` for changed bindings; run formatting and relevant scientific regression suites. Record actual outputs and update feature status.
- [x] Step 7: Review the diff against the acceptance IDs, stage only this task’s files and create a local checkpoint commit after checks pass. Publishing or external delivery follows the user’s authorization.

**Completion gate:** Validated enzyme scope/results and named-version/Undo distinction. The listed tests pass and the owning feature traceability entries include implementation/test evidence.

## P09 · Saved analysis result lifecycle

**Goal:** Persist actual tool outputs, input snapshots and parameters, then implement save/refind/older-input/rerun/compare/export.

**Prerequisites:** P01, P04, P08D.

**Files:**

- Modify/review: `crates/appcore/src/analysis.rs`
- Modify/review: `crates/appcore/src/actions.rs`
- Modify/review: `crates/storage/src/config.rs`
- Create: `crates/appcore/src/results.rs`
- Create: `apps/desktop_gpui/src/views/results.rs`
- Create tests: `crates/appcore/tests/ux_v4_p09.rs`

**Interfaces:**

- Consumes: DocumentId / RevisionId / ResultId / OperationId; SourceRef; RecordRef; v1→v2 migration and immutable input snapshot serialization.; OperationState / OperationReceipt / RecoveryPlan; operation journal, completion dispatch and surfaced persistence errors.; Validated enzyme scope/results and named-version/Undo distinction.
- Produces: AnalysisResult catalog, save_analysis, source snapshot resolution, stale/rerun/export routes.

**Required tests:**

- `new_source_marks_old_result_older_input`
- `rerun_allocates_new_result_id`
- `deleted_source_opens_tombstone`
- `export_contains_input_and_method`

- [x] Step 1: Read the owning screen/feature/state contracts and the cited current APIs; record any newly discovered contradiction before changing scientific behavior.
- [x] Step 2: Add the listed behavior tests and representative valid/invalid/cancel/stale-ID fixtures in the proposed test file. Confirm the new expectation fails for the intended reason; P00 instead records the existing baseline.
- [x] Step 3: Run `cargo test -p appcore --test ux_v4_p09` and retain the initial result. Register the integration-test harness normally; do not claim an absent test file passed.
- [x] Step 4: Implement the minimal domain/storage/appcore contract and error propagation before wiring its GPUI controls. Reuse the current algorithm entry points; no fixture ID or result value may become a production default.
- [x] Step 5: Wire the UI states, prerequisites, labeled entry point and return context. Validate the success path plus the task-specific edge conditions and one persistence/cancel failure.
- [x] Step 6: Re-run the focused tests, `cargo test -p appcore` and `cargo check -p desktop_gpui` for changed bindings; run formatting and relevant scientific regression suites. Record actual outputs and update feature status.
- [x] Step 7: Review the diff against the acceptance IDs, stage only this task’s files and create a local checkpoint commit after checks pass. Publishing or external delivery follows the user’s authorization.

**Completion gate:** AnalysisResult catalog, save_analysis, source snapshot resolution, stale/rerun/export routes. The listed tests pass and the owning feature traceability entries include implementation/test evidence.

## P10A · Alignment, split workspace and saved alignments

**Goal:** Keep pair and multiple alignment behavior with explicit pane/input scope and durable saved-result provenance.

**Prerequisites:** P08A, P09.

**Files:**

- Modify/review: `crates/appcore/src/actions.rs`
- Modify/review: `crates/storage/src/alignments.rs`
- Modify/review: `apps/desktop_gpui/src/lab.rs`
- Modify/review: `apps/desktop_gpui/src/dialogs.rs`
- Create tests: `crates/appcore/tests/ux_v4_p10a.rs`

**Interfaces:**

- Consumes: Stable document/selection bindings and explicit coordinate conversions for Map/Sequence.; AnalysisResult catalog, save_analysis, source snapshot resolution, stale/rerun/export routes.
- Produces: Saved alignment/split-pane routes bound to actual source revisions.

**Required tests:**

- `split_active_pane_controls_input`
- `saved_alignment_load_retains_sources`
- `circular_reindex_retains_mapping`

- [ ] Step 1: Read the owning screen/feature/state contracts and the cited current APIs; record any newly discovered contradiction before changing scientific behavior.
- [ ] Step 2: Add the listed behavior tests and representative valid/invalid/cancel/stale-ID fixtures in the proposed test file. Confirm the new expectation fails for the intended reason; P00 instead records the existing baseline.
- [ ] Step 3: Run `cargo test -p appcore --test ux_v4_p10a` and retain the initial result. Register the integration-test harness normally; do not claim an absent test file passed.
- [ ] Step 4: Implement the minimal domain/storage/appcore contract and error propagation before wiring its GPUI controls. Reuse the current algorithm entry points; no fixture ID or result value may become a production default.
- [ ] Step 5: Wire the UI states, prerequisites, labeled entry point and return context. Validate the success path plus the task-specific edge conditions and one persistence/cancel failure.
- [ ] Step 6: Re-run the focused tests, `cargo test -p appcore` and `cargo check -p desktop_gpui` for changed bindings; run formatting and relevant scientific regression suites. Record actual outputs and update feature status.
- [ ] Step 7: Review the diff against the acceptance IDs, stage only this task’s files and create a local checkpoint commit after checks pass. Publishing or external delivery follows the user’s authorization.

**Completion gate:** Saved alignment/split-pane routes bound to actual source revisions. The listed tests pass and the owning feature traceability entries include implementation/test evidence.

## P10B · Assembly and combinatorial products

**Goal:** Expose strategy-specific inputs/junction checks and preserve all existing assembly products and limits.

**Prerequisites:** P08C, P09.

**Files:**

- Modify/review: `crates/appcore/src/actions.rs`
- Modify/review: `apps/desktop_gpui/src/dialogs.rs`
- Modify/review: `crates/domain/src/assembly.rs`
- Create tests: `crates/domain/tests/ux_v4_p10b.rs`

**Interfaces:**

- Consumes: Captured primer/PCR requests and independent product provenance.; AnalysisResult catalog, save_analysis, source snapshot resolution, stale/rerun/export routes.
- Produces: Reviewed assembly product creation and strategy-specific diagnostics.

**Required tests:**

- `assembly_invalid_overhang_is_reviewable`
- `assembly_product_has_new_id`
- `combinatorial_cap_is_explicit`

- [ ] Step 1: Read the owning screen/feature/state contracts and the cited current APIs; record any newly discovered contradiction before changing scientific behavior.
- [ ] Step 2: Add the listed behavior tests and representative valid/invalid/cancel/stale-ID fixtures in the proposed test file. Confirm the new expectation fails for the intended reason; P00 instead records the existing baseline.
- [ ] Step 3: Run `cargo test -p domain --test ux_v4_p10b` and retain the initial result. Register the integration-test harness normally; do not claim an absent test file passed.
- [ ] Step 4: Implement the minimal domain/storage/appcore contract and error propagation before wiring its GPUI controls. Reuse the current algorithm entry points; no fixture ID or result value may become a production default.
- [ ] Step 5: Wire the UI states, prerequisites, labeled entry point and return context. Validate the success path plus the task-specific edge conditions and one persistence/cancel failure.
- [ ] Step 6: Re-run the focused tests, `cargo test -p domain` and `cargo check -p desktop_gpui` for changed bindings; run formatting and relevant scientific regression suites. Record actual outputs and update feature status.
- [ ] Step 7: Review the diff against the acceptance IDs, stage only this task’s files and create a local checkpoint commit after checks pass. Publishing or external delivery follows the user’s authorization.

**Completion gate:** Reviewed assembly product creation and strategy-specific diagnostics. The listed tests pass and the owning feature traceability entries include implementation/test evidence.

## P10C · CRISPR, codon optimization, BLAST and protein tools

**Goal:** Preserve engines while adding explicit input/parameter review and accurate method/coverage labels.

**Prerequisites:** P09, P05.

**Files:**

- Modify/review: `crates/appcore/src/actions.rs`
- Modify/review: `apps/desktop_gpui/src/dialogs.rs`
- Create tests: `crates/appcore/tests/ux_v4_p10c.rs`

**Interfaces:**

- Consumes: AnalysisResult catalog, save_analysis, source snapshot resolution, stale/rerun/export routes.; CommandId / command descriptors / availability; SearchQuery and typed search result/return state.
- Produces: Typed scientific tool inputs and method-specific parameter/result contracts.

**Required tests:**

- `protein_tool_requires_typed_protein`
- `blast_uses_reviewed_database`
- `codon_apply_retains_protein_in_fixture`
- `guide_result_keeps_method_and_scope`

- [ ] Step 1: Read the owning screen/feature/state contracts and the cited current APIs; record any newly discovered contradiction before changing scientific behavior.
- [ ] Step 2: Add the listed behavior tests and representative valid/invalid/cancel/stale-ID fixtures in the proposed test file. Confirm the new expectation fails for the intended reason; P00 instead records the existing baseline.
- [ ] Step 3: Run `cargo test -p appcore --test ux_v4_p10c` and retain the initial result. Register the integration-test harness normally; do not claim an absent test file passed.
- [ ] Step 4: Implement the minimal domain/storage/appcore contract and error propagation before wiring its GPUI controls. Reuse the current algorithm entry points; no fixture ID or result value may become a production default.
- [ ] Step 5: Wire the UI states, prerequisites, labeled entry point and return context. Validate the success path plus the task-specific edge conditions and one persistence/cancel failure.
- [ ] Step 6: Re-run the focused tests, `cargo test -p appcore` and `cargo check -p desktop_gpui` for changed bindings; run formatting and relevant scientific regression suites. Record actual outputs and update feature status.
- [ ] Step 7: Review the diff against the acceptance IDs, stage only this task’s files and create a local checkpoint commit after checks pass. Publishing or external delivery follows the user’s authorization.

**Completion gate:** Typed scientific tool inputs and method-specific parameter/result contracts. The listed tests pass and the owning feature traceability entries include implementation/test evidence.

## P11 · Notebook creation, submission and local review

**Goal:** Implement real blank/template drafts, stable submitted-result records and immutable local review revisions.

**Prerequisites:** P01, P04, P06.

**Files:**

- Modify/review: `crates/domain/src/notebook.rs`
- Modify/review: `crates/storage/src/records.rs`
- Modify/review: `crates/appcore/src/actions.rs`
- Modify/review: `apps/desktop_gpui/src/lab.rs`
- Modify/review: `apps/desktop_gpui/src/dialogs.rs`
- Create tests: `crates/appcore/tests/ux_v4_p11.rs`

**Interfaces:**

- Consumes: DocumentId / RevisionId / ResultId / OperationId; SourceRef; RecordRef; v1→v2 migration and immutable input snapshot serialization.; OperationState / OperationReceipt / RecoveryPlan; operation journal, completion dispatch and surfaced persistence errors.; First-use preference, sample origin and personal-copy policy; typed recent-work routes.
- Produces: Immutable SubmittedEntryRevision, reusable submitted result IDs and local-review transitions.

**Required tests:**

- `blank_entry_contains_no_demo_rows`
- `template_does_not_link_sample`
- `submitted_revision_is_immutable`
- `row_submission_retry_does_not_duplicate`
- `review_is_local_not_remote_delivery`

- [ ] Step 1: Read the owning screen/feature/state contracts and the cited current APIs; record any newly discovered contradiction before changing scientific behavior.
- [ ] Step 2: Add the listed behavior tests and representative valid/invalid/cancel/stale-ID fixtures in the proposed test file. Confirm the new expectation fails for the intended reason; P00 instead records the existing baseline.
- [ ] Step 3: Run `cargo test -p appcore --test ux_v4_p11` and retain the initial result. Register the integration-test harness normally; do not claim an absent test file passed.
- [ ] Step 4: Implement the minimal domain/storage/appcore contract and error propagation before wiring its GPUI controls. Reuse the current algorithm entry points; no fixture ID or result value may become a production default.
- [ ] Step 5: Wire the UI states, prerequisites, labeled entry point and return context. Validate the success path plus the task-specific edge conditions and one persistence/cancel failure.
- [ ] Step 6: Re-run the focused tests, `cargo test -p appcore` and `cargo check -p desktop_gpui` for changed bindings; run formatting and relevant scientific regression suites. Record actual outputs and update feature status.
- [ ] Step 7: Review the diff against the acceptance IDs, stage only this task’s files and create a local checkpoint commit after checks pass. Publishing or external delivery follows the user’s authorization.

**Completion gate:** Immutable SubmittedEntryRevision, reusable submitted result IDs and local-review transitions. The listed tests pass and the owning feature traceability entries include implementation/test evidence.

## P12 · Typed Registry, relationships and deletion

**Goal:** Federate typed entity references and implement record-specific archive/restore/dependency-aware deletion with tombstones.

**Prerequisites:** P01, P04, P05.

**Files:**

- Modify/review: `crates/domain/src/registry.rs`
- Modify/review: `crates/storage/src/registry.rs`
- Modify/review: `crates/appcore/src/actions.rs`
- Modify/review: `apps/desktop_gpui/src/lab.rs`
- Create: `crates/appcore/src/record_refs.rs`
- Create tests: `crates/appcore/tests/ux_v4_p12.rs`

**Interfaces:**

- Consumes: DocumentId / RevisionId / ResultId / OperationId; SourceRef; RecordRef; v1→v2 migration and immutable input snapshot serialization.; OperationState / OperationReceipt / RecoveryPlan; operation journal, completion dispatch and surfaced persistence errors.; CommandId / command descriptors / availability; SearchQuery and typed search result/return state.
- Produces: Typed federated Registry record adapter, archive/restore/dependency/tombstone operations.

**Required tests:**

- `archive_primer_never_targets_dna`
- `ambiguous_legacy_name_link_requires_repair`
- `delete_revalidates_dependencies`
- `post_delete_clears_inspector`

- [ ] Step 1: Read the owning screen/feature/state contracts and the cited current APIs; record any newly discovered contradiction before changing scientific behavior.
- [ ] Step 2: Add the listed behavior tests and representative valid/invalid/cancel/stale-ID fixtures in the proposed test file. Confirm the new expectation fails for the intended reason; P00 instead records the existing baseline.
- [ ] Step 3: Run `cargo test -p appcore --test ux_v4_p12` and retain the initial result. Register the integration-test harness normally; do not claim an absent test file passed.
- [ ] Step 4: Implement the minimal domain/storage/appcore contract and error propagation before wiring its GPUI controls. Reuse the current algorithm entry points; no fixture ID or result value may become a production default.
- [ ] Step 5: Wire the UI states, prerequisites, labeled entry point and return context. Validate the success path plus the task-specific edge conditions and one persistence/cancel failure.
- [ ] Step 6: Re-run the focused tests, `cargo test -p appcore` and `cargo check -p desktop_gpui` for changed bindings; run formatting and relevant scientific regression suites. Record actual outputs and update feature status.
- [ ] Step 7: Review the diff against the acceptance IDs, stage only this task’s files and create a local checkpoint commit after checks pass. Publishing or external delivery follows the user’s authorization.

**Completion gate:** Typed federated Registry record adapter, archive/restore/dependency/tombstone operations. The listed tests pass and the owning feature traceability entries include implementation/test evidence.

## P13 · Inventory assignment, transfer and snapshots

**Goal:** Implement reviewed well/selection payloads, explicit units, collision checks and recoverable transfer plans.

**Prerequisites:** P12, P04, P11.

**Files:**

- Modify/review: `crates/domain/src/inventory.rs`
- Modify/review: `crates/storage/src/inventory.rs`
- Modify/review: `crates/appcore/src/actions.rs`
- Modify/review: `apps/desktop_gpui/src/lab.rs`
- Create: `crates/appcore/src/inventory_ops.rs`
- Create tests: `crates/appcore/tests/ux_v4_p13.rs`

**Interfaces:**

- Consumes: Typed federated Registry record adapter, archive/restore/dependency/tombstone operations.; OperationState / OperationReceipt / RecoveryPlan; operation journal, completion dispatch and surfaced persistence errors.; Immutable SubmittedEntryRevision, reusable submitted result IDs and local-review transitions.
- Produces: WellRef / TransferRequest / TransferPlan / ReviewedTransferPlan; plan_transfer and commit_transfer.

**Required tests:**

- `all_96_wells_preserve_coordinate`
- `bulk_plan_has_exact_reviewed_ids`
- `transfer_is_all_or_recoverable`
- `unknown_legacy_concentration_unit_blocks_conversion`
- `snapshot_does_not_change_with_live_inventory`

- [ ] Step 1: Read the owning screen/feature/state contracts and the cited current APIs; record any newly discovered contradiction before changing scientific behavior.
- [ ] Step 2: Add the listed behavior tests and representative valid/invalid/cancel/stale-ID fixtures in the proposed test file. Confirm the new expectation fails for the intended reason; P00 instead records the existing baseline.
- [ ] Step 3: Run `cargo test -p appcore --test ux_v4_p13` and retain the initial result. Register the integration-test harness normally; do not claim an absent test file passed.
- [ ] Step 4: Implement the minimal domain/storage/appcore contract and error propagation before wiring its GPUI controls. Reuse the current algorithm entry points; no fixture ID or result value may become a production default.
- [ ] Step 5: Wire the UI states, prerequisites, labeled entry point and return context. Validate the success path plus the task-specific edge conditions and one persistence/cancel failure.
- [ ] Step 6: Re-run the focused tests, `cargo test -p appcore` and `cargo check -p desktop_gpui` for changed bindings; run formatting and relevant scientific regression suites. Record actual outputs and update feature status.
- [ ] Step 7: Review the diff against the acceptance IDs, stage only this task’s files and create a local checkpoint commit after checks pass. Publishing or external delivery follows the user’s authorization.

**Completion gate:** WellRef / TransferRequest / TransferPlan / ReviewedTransferPlan; plan_transfer and commit_transfer. The listed tests pass and the owning feature traceability entries include implementation/test evidence.

## P14 · Workflows and request handoffs

**Goal:** Make task/request selection and graph execution status explicit and recoverable across module handoffs.

**Prerequisites:** P02, P04, P11, P12.

**Files:**

- Modify/review: `crates/domain/src/workflow.rs`
- Modify/review: `crates/storage/src/workflow.rs`
- Modify/review: `crates/appcore/src/actions.rs`
- Modify/review: `apps/desktop_gpui/src/lab.rs`
- Create tests: `crates/appcore/tests/ux_v4_p14.rs`

**Interfaces:**

- Consumes: Route, ReturnLocation, SelectionContext, NavigationIntent, request_navigation and resolve_guard.; OperationState / OperationReceipt / RecoveryPlan; operation journal, completion dispatch and surfaced persistence errors.; Immutable SubmittedEntryRevision, reusable submitted result IDs and local-review transitions.; Typed federated Registry record adapter, archive/restore/dependency/tombstone operations.
- Produces: Task/request ID routes and recoverable graph cursor/output-commit behavior.

**Required tests:**

- `task_row_opens_exact_id`
- `missing_mapping_blocks_execution`
- `failed_output_commit_does_not_advance_cursor`
- `request_send_records_local_status`

- [ ] Step 1: Read the owning screen/feature/state contracts and the cited current APIs; record any newly discovered contradiction before changing scientific behavior.
- [ ] Step 2: Add the listed behavior tests and representative valid/invalid/cancel/stale-ID fixtures in the proposed test file. Confirm the new expectation fails for the intended reason; P00 instead records the existing baseline.
- [ ] Step 3: Run `cargo test -p appcore --test ux_v4_p14` and retain the initial result. Register the integration-test harness normally; do not claim an absent test file passed.
- [ ] Step 4: Implement the minimal domain/storage/appcore contract and error propagation before wiring its GPUI controls. Reuse the current algorithm entry points; no fixture ID or result value may become a production default.
- [ ] Step 5: Wire the UI states, prerequisites, labeled entry point and return context. Validate the success path plus the task-specific edge conditions and one persistence/cancel failure.
- [ ] Step 6: Re-run the focused tests, `cargo test -p appcore` and `cargo check -p desktop_gpui` for changed bindings; run formatting and relevant scientific regression suites. Record actual outputs and update feature status.
- [ ] Step 7: Review the diff against the acceptance IDs, stage only this task’s files and create a local checkpoint commit after checks pass. Publishing or external delivery follows the user’s authorization.

**Completion gate:** Task/request ID routes and recoverable graph cursor/output-commit behavior. The listed tests pass and the owning feature traceability entries include implementation/test evidence.

## P15 · Insights scope, chart alternatives and export

**Goal:** Bind dashboards to reviewed local data scope and preserve table/chart/CSV behavior with distinct empty/failure states.

**Prerequisites:** P05, P09, P12.

**Files:**

- Modify/review: `crates/domain/src/insights.rs`
- Modify/review: `crates/storage/src/analytics.rs`
- Modify/review: `crates/appcore/src/actions.rs`
- Modify/review: `apps/desktop_gpui/src/lab.rs`
- Create tests: `crates/appcore/tests/ux_v4_p15.rs`

**Interfaces:**

- Consumes: CommandId / command descriptors / availability; SearchQuery and typed search result/return state.; AnalysisResult catalog, save_analysis, source snapshot resolution, stale/rerun/export routes.; Typed federated Registry record adapter, archive/restore/dependency/tombstone operations.
- Produces: Scoped dataset/query identity, block invalidation and exact-scope CSV export.

**Required tests:**

- `no_matches_is_not_load_failure`
- `cache_key_includes_parameters`
- `chart_source_link_opens_exact_record`
- `csv_export_quotes_source_values`

- [ ] Step 1: Read the owning screen/feature/state contracts and the cited current APIs; record any newly discovered contradiction before changing scientific behavior.
- [ ] Step 2: Add the listed behavior tests and representative valid/invalid/cancel/stale-ID fixtures in the proposed test file. Confirm the new expectation fails for the intended reason; P00 instead records the existing baseline.
- [ ] Step 3: Run `cargo test -p appcore --test ux_v4_p15` and retain the initial result. Register the integration-test harness normally; do not claim an absent test file passed.
- [ ] Step 4: Implement the minimal domain/storage/appcore contract and error propagation before wiring its GPUI controls. Reuse the current algorithm entry points; no fixture ID or result value may become a production default.
- [ ] Step 5: Wire the UI states, prerequisites, labeled entry point and return context. Validate the success path plus the task-specific edge conditions and one persistence/cancel failure.
- [ ] Step 6: Re-run the focused tests, `cargo test -p appcore` and `cargo check -p desktop_gpui` for changed bindings; run formatting and relevant scientific regression suites. Record actual outputs and update feature status.
- [ ] Step 7: Review the diff against the acceptance IDs, stage only this task’s files and create a local checkpoint commit after checks pass. Publishing or external delivery follows the user’s authorization.

**Completion gate:** Scoped dataset/query identity, block invalidation and exact-scope CSV export. The listed tests pass and the owning feature traceability entries include implementation/test evidence.

## P16 · Automation targets, journals and recovery

**Goal:** Replace naive ingestion, dispatch explicit writeback targets and implement immutable pipeline revisions plus durable operation/recovery state.

**Prerequisites:** P04, P11, P12, P15.

**Files:**

- Modify/review: `crates/domain/src/automation.rs`
- Modify/review: `crates/storage/src/analytics.rs`
- Modify/review: `crates/appcore/src/actions.rs`
- Modify/review: `apps/desktop_gpui/src/lab.rs`
- Create: `crates/storage/src/csv.rs`
- Create: `crates/appcore/src/automation_ops.rs`
- Create tests: `crates/appcore/tests/ux_v4_p16.rs`

**Interfaces:**

- Consumes: OperationState / OperationReceipt / RecoveryPlan; operation journal, completion dispatch and surfaced persistence errors.; Immutable SubmittedEntryRevision, reusable submitted result IDs and local-review transitions.; Typed federated Registry record adapter, archive/restore/dependency/tombstone operations.; Scoped dataset/query identity, block invalidation and exact-scope CSV export.
- Produces: Typed parsed table, immutable PipelineRevision, explicit writeback target and reconciled run recovery.

**Required tests:**

- `output_results_does_not_create_plasmids`
- `quoted_csv_rows_parse_correctly`
- `double_start_does_not_duplicate_outputs`
- `partial_write_reconciliation_skips_committed_rows`
- `cancel_after_commit_is_not_false_rollback`

- [ ] Step 1: Read the owning screen/feature/state contracts and the cited current APIs; record any newly discovered contradiction before changing scientific behavior.
- [ ] Step 2: Add the listed behavior tests and representative valid/invalid/cancel/stale-ID fixtures in the proposed test file. Confirm the new expectation fails for the intended reason; P00 instead records the existing baseline.
- [ ] Step 3: Run `cargo test -p appcore --test ux_v4_p16` and retain the initial result. Register the integration-test harness normally; do not claim an absent test file passed.
- [ ] Step 4: Implement the minimal domain/storage/appcore contract and error propagation before wiring its GPUI controls. Reuse the current algorithm entry points; no fixture ID or result value may become a production default.
- [ ] Step 5: Wire the UI states, prerequisites, labeled entry point and return context. Validate the success path plus the task-specific edge conditions and one persistence/cancel failure.
- [ ] Step 6: Re-run the focused tests, `cargo test -p appcore` and `cargo check -p desktop_gpui` for changed bindings; run formatting and relevant scientific regression suites. Record actual outputs and update feature status.
- [ ] Step 7: Review the diff against the acceptance IDs, stage only this task’s files and create a local checkpoint commit after checks pass. Publishing or external delivery follows the user’s authorization.

**Completion gate:** Typed parsed table, immutable PipelineRevision, explicit writeback target and reconciled run recovery. The listed tests pass and the owning feature traceability entries include implementation/test evidence.

## P17 · Biologics, chemistry and RNA identities

**Goal:** Apply the shared record/selection/result model to specialist inputs without replacing supported scientific methods.

**Prerequisites:** P12, P10C, P09.

**Files:**

- Modify/review: `crates/domain/src/biologics.rs`
- Modify/review: `crates/domain/src/chemistry.rs`
- Modify/review: `crates/domain/src/rna.rs`
- Modify/review: `crates/appcore/src/actions.rs`
- Modify/review: `apps/desktop_gpui/src/lab.rs`
- Create tests: `crates/domain/tests/ux_v4_p17.rs`

**Interfaces:**

- Consumes: Typed federated Registry record adapter, archive/restore/dependency/tombstone operations.; Typed scientific tool inputs and method-specific parameter/result contracts.; AnalysisResult catalog, save_analysis, source snapshot resolution, stale/rerun/export routes.
- Produces: Specialist typed-record adapters preserving chain/compound/oligo input/output identity.

**Required tests:**

- `selected_chain_survives_analysis`
- `unsupported_smiles_is_not_fabricated_structure`
- `helm_round_trip_preserves_supported_modifications`
- `same_name_specialist_records_do_not_swap`

- [ ] Step 1: Read the owning screen/feature/state contracts and the cited current APIs; record any newly discovered contradiction before changing scientific behavior.
- [ ] Step 2: Add the listed behavior tests and representative valid/invalid/cancel/stale-ID fixtures in the proposed test file. Confirm the new expectation fails for the intended reason; P00 instead records the existing baseline.
- [ ] Step 3: Run `cargo test -p domain --test ux_v4_p17` and retain the initial result. Register the integration-test harness normally; do not claim an absent test file passed.
- [ ] Step 4: Implement the minimal domain/storage/appcore contract and error propagation before wiring its GPUI controls. Reuse the current algorithm entry points; no fixture ID or result value may become a production default.
- [ ] Step 5: Wire the UI states, prerequisites, labeled entry point and return context. Validate the success path plus the task-specific edge conditions and one persistence/cancel failure.
- [ ] Step 6: Re-run the focused tests, `cargo test -p domain` and `cargo check -p desktop_gpui` for changed bindings; run formatting and relevant scientific regression suites. Record actual outputs and update feature status.
- [ ] Step 7: Review the diff against the acceptance IDs, stage only this task’s files and create a local checkpoint commit after checks pass. Publishing or external delivery follows the user’s authorization.

**Completion gate:** Specialist typed-record adapters preserving chain/compound/oligo input/output identity. The listed tests pass and the owning feature traceability entries include implementation/test evidence.

## P18 · In-vivo and bioprocess stateful records

**Goal:** Preserve study/measurement/cohort and recipe/run/replicate behavior with explicit IDs, units and lifecycle controls.

**Prerequisites:** P12, P04.

**Files:**

- Modify/review: `crates/domain/src/invivo.rs`
- Modify/review: `crates/domain/src/bioprocess.rs`
- Modify/review: `crates/appcore/src/actions.rs`
- Modify/review: `apps/desktop_gpui/src/lab.rs`
- Create tests: `crates/domain/tests/ux_v4_p18.rs`

**Interfaces:**

- Consumes: Typed federated Registry record adapter, archive/restore/dependency/tombstone operations.; OperationState / OperationReceipt / RecoveryPlan; operation journal, completion dispatch and surfaced persistence errors.
- Produces: Explicit-unit study/measurement and recipe/run/replicate action contracts.

**Required tests:**

- `animal_measurement_uses_exact_study_and_id`
- `dose_helper_rejects_zero_denominator`
- `cohort_seed_and_input_set_are_retained`
- `completed_run_does_not_restart`
- `recipe_edit_does_not_mutate_run_snapshot`

- [ ] Step 1: Read the owning screen/feature/state contracts and the cited current APIs; record any newly discovered contradiction before changing scientific behavior.
- [ ] Step 2: Add the listed behavior tests and representative valid/invalid/cancel/stale-ID fixtures in the proposed test file. Confirm the new expectation fails for the intended reason; P00 instead records the existing baseline.
- [ ] Step 3: Run `cargo test -p domain --test ux_v4_p18` and retain the initial result. Register the integration-test harness normally; do not claim an absent test file passed.
- [ ] Step 4: Implement the minimal domain/storage/appcore contract and error propagation before wiring its GPUI controls. Reuse the current algorithm entry points; no fixture ID or result value may become a production default.
- [ ] Step 5: Wire the UI states, prerequisites, labeled entry point and return context. Validate the success path plus the task-specific edge conditions and one persistence/cancel failure.
- [ ] Step 6: Re-run the focused tests, `cargo test -p domain` and `cargo check -p desktop_gpui` for changed bindings; run formatting and relevant scientific regression suites. Record actual outputs and update feature status.
- [ ] Step 7: Review the diff against the acceptance IDs, stage only this task’s files and create a local checkpoint commit after checks pass. Publishing or external delivery follows the user’s authorization.

**Completion gate:** Explicit-unit study/measurement and recipe/run/replicate action contracts. The listed tests pass and the owning feature traceability entries include implementation/test evidence.

## P19 · Responsive, focus, error and accessibility hardening

**Goal:** Complete runtime interaction states, semantic exposure, keyboard flow, panel behavior and measured dense-data rendering.

**Prerequisites:** P07, P08A, P11, P13, P16, P17, P18.

**Files:**

- Modify/review: `apps/desktop_gpui/src/app.rs`
- Modify/review: `apps/desktop_gpui/src/widgets.rs`
- Modify/review: `apps/desktop_gpui/src/text_input.rs`
- Modify/review: `apps/desktop_gpui/src/theme.rs`
- Create tests: `crates/appcore/tests/ux_v4_p19.rs`

**Interfaces:**

- Consumes: Tokenized GPUI shell, contextual panel and reusable feedback/form/table surfaces.; Stable document/selection bindings and explicit coordinate conversions for Map/Sequence.; Immutable SubmittedEntryRevision, reusable submitted result IDs and local-review transitions.; WellRef / TransferRequest / TransferPlan / ReviewedTransferPlan; plan_transfer and commit_transfer.; Typed parsed table, immutable PipelineRevision, explicit writeback target and reconciled run recovery.; Specialist typed-record adapters preserving chain/compound/oligo input/output identity.; Explicit-unit study/measurement and recipe/run/replicate action contracts.
- Produces: Verified runtime keyboard/focus/panel/empty/error/responsive interaction behaviors.

**Required tests:**

- `input_shortcuts_do_not_edit_sequence`
- `modal_escape_returns_focus`
- `1280_first_use_journey_stays_usable`
- `filtered_empty_has_clear_reset`
- `focused_control_not_hidden_by_panel`

- [ ] Step 1: Read the owning screen/feature/state contracts and the cited current APIs; record any newly discovered contradiction before changing scientific behavior.
- [ ] Step 2: Add the listed behavior tests and representative valid/invalid/cancel/stale-ID fixtures in the proposed test file. Confirm the new expectation fails for the intended reason; P00 instead records the existing baseline.
- [ ] Step 3: Run `cargo test -p appcore --test ux_v4_p19` and retain the initial result. Register the integration-test harness normally; do not claim an absent test file passed.
- [ ] Step 4: Implement the minimal domain/storage/appcore contract and error propagation before wiring its GPUI controls. Reuse the current algorithm entry points; no fixture ID or result value may become a production default.
- [ ] Step 5: Wire the UI states, prerequisites, labeled entry point and return context. Validate the success path plus the task-specific edge conditions and one persistence/cancel failure.
- [ ] Step 6: Re-run the focused tests, `cargo test -p appcore` and `cargo check -p desktop_gpui` for changed bindings; run formatting and relevant scientific regression suites. Record actual outputs and update feature status.
- [ ] Step 7: Review the diff against the acceptance IDs, stage only this task’s files and create a local checkpoint commit after checks pass. Publishing or external delivery follows the user’s authorization.

**Completion gate:** Verified runtime keyboard/focus/panel/empty/error/responsive interaction behaviors. The listed tests pass and the owning feature traceability entries include implementation/test evidence.

## P20 · End-to-end validation and staged release

**Goal:** Run the full regression/edge/UI checklist and first-user study, update traceability and ship only genuinely implemented behavior.

**Prerequisites:** P19, P09, P10A, P10B, P10C, P14, P15.

**Files:**

- Modify/review: `README.md`
- Modify/review: `DESIGN.md`
- Create tests: `crates/appcore/tests/ux_v4_p20.rs`

**Interfaces:**

- Consumes: Verified runtime keyboard/focus/panel/empty/error/responsive interaction behaviors.; AnalysisResult catalog, save_analysis, source snapshot resolution, stale/rerun/export routes.; Saved alignment/split-pane routes bound to actual source revisions.; Reviewed assembly product creation and strategy-specific diagnostics.; Typed scientific tool inputs and method-specific parameter/result contracts.; Task/request ID routes and recoverable graph cursor/output-commit behavior.; Scoped dataset/query identity, block invalidation and exact-scope CSV export.
- Produces: Recorded regression/edge/UI/study results, updated traceability and release evidence.

**Required tests:**

- `first_use_import_analyze_save_refind`
- `legacy_workspace_migrates_without_loss`
- `restart_after_partial_operation_is_recoverable`
- `preserved_feature_catalog_is_reachable`

- [ ] Step 1: Read the owning screen/feature/state contracts and the cited current APIs; record any newly discovered contradiction before changing scientific behavior.
- [ ] Step 2: Add the listed behavior tests and representative valid/invalid/cancel/stale-ID fixtures in the proposed test file. Confirm the new expectation fails for the intended reason; P00 instead records the existing baseline.
- [ ] Step 3: Run `cargo test -p appcore --test ux_v4_p20` and retain the initial result. Register the integration-test harness normally; do not claim an absent test file passed.
- [ ] Step 4: Implement the minimal domain/storage/appcore contract and error propagation before wiring its GPUI controls. Reuse the current algorithm entry points; no fixture ID or result value may become a production default.
- [ ] Step 5: Wire the UI states, prerequisites, labeled entry point and return context. Validate the success path plus the task-specific edge conditions and one persistence/cancel failure.
- [ ] Step 6: Re-run the focused tests, `cargo test -p appcore` and `cargo check -p desktop_gpui` for changed bindings; run formatting and relevant scientific regression suites. Record actual outputs and update feature status.
- [ ] Step 7: Review the diff against the acceptance IDs, stage only this task’s files and create a local checkpoint commit after checks pass. Publishing or external delivery follows the user’s authorization.

**Completion gate:** Recorded regression/edge/UI/study results, updated traceability and release evidence. The listed tests pass and the owning feature traceability entries include implementation/test evidence.

