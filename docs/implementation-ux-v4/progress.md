# UX v4 implementation progress

P00/P01 merged into `main` at `908788b` on 15 November 2026.
P02 merged into `main` at `9c6802d`. P03/P04 are pushed implementation branches; no P03/P04 merge is claimed.
Baseline: `0474cb94e456094dc4e501a9351ca7587a149361`.
Authority: UX v4 interface; preserve existing scientific capabilities and local data.

## P00 — complete

15 November 2026, Windows, Rust/Cargo 1.100.0.

- `cargo test -p domain -p storage -p appcore --locked`: **181 passed, 0 failed** (domain 145; storage 30; appcore 6). Doc tests: 0 cases.
- `cargo check -p desktop_gpui --locked`: **passed**. Upstream `proc-macro-error2 2.0.1` emits a future compatibility notice; no baseline compiler error.
- Scientific fixtures retained in the existing domain tests: translation/ORFs, primer/PCR, digest, alignment, assembly, CRISPR, codon, protein, RNA and chemistry.
- Native-file baseline: `.hbx` v1, atomic round-trip and rejection of unsupported newer formats. Legacy fixture is in `fixtures/legacy-v1.hbx`.
- Specification, screen images, field/state/feature catalogs and the ordered plan are committed alongside this progress record.

P00 changes no scientific behavior. The repeated generic UI/test checklist in the plan does not apply to this baseline-only task.

## P01 — complete

15 November 2026. Durable identity and persistence foundations:

- Added opaque document, revision, result and operation IDs. Normal clones preserve identity; explicit independent copies remint it while preserving scientific annotations.
- Captured immutable input snapshots include source identity, selection, original document content and a SHA-256 digest. Snapshots are independent of the 20-checkpoint retention cap.
- Added a versioned analysis-result catalog that rejects corrupted/newer data, duplicate IDs, snapshot tampering and in-place edits to persisted results.
- Native v1 reads remain read-only and deterministic for an unchanged source path/content. Explicit save writes v2 and preserves exact original bytes in a sibling `.hbx.v1.bak`; conflicting backups and unsupported newer destinations block overwrite. Unsaved legacy identities are source-scoped until the first v2 save; moving/changing an unmigrated source can change its inferred identity.
- Added typed Registry/Inventory reference fields without discarding legacy labels. Legacy target edits clear conflicting typed references. Automatic name-based link resolution is deliberately deferred to P12's reviewed routing, because the open document set may not be the complete catalog.
- Active and library bulk edits advance revision IDs and mark changed documents dirty. Undo/redo preserves the exact historical revision. Tag edits use the undo boundary.
- Imported FASTA/GenBank/raw files are retained as original sources. Save uses a new native destination instead of overwriting them.

Verification:

- `cargo test -p domain -p storage -p appcore --locked`: **201 passed, 0 failed** (181 existing + 20 added).
- `cargo check -p desktop_gpui --locked`: **passed**.
- `cargo clippy -p domain -p storage -p appcore --all-targets --locked -- -D warnings`: **passed**.
- `cargo fmt --all -- --check` and `git diff --check`: **passed**.
- New regression cases were observed failing before the corresponding fixes: identity/migration/backup/newer-version safety, result mutation, typed-link support/setters, tags/bulk revision tracking and interchange-file save routing.
- Independent code review: two Important and one Minor finding addressed with regression coverage. No Critical finding.

P01 does not introduce the new application shell or claim the result lifecycle UI is implemented. P09 wires actual analysis outputs into the catalog; P12 wires typed cross-store record routing.

## Next

P04: durable operation state, persistence errors and non-blocking feedback.

## P02 — implemented and verified by automated checks

15 November 2026. Native navigation and unsaved-work behavior:

- Added typed routes, document navigation intents, source/selection contexts and ReturnLocation. Document tabs and Home switch by durable document ID rather than a captured row index.
- Added a visible Back action. Returning restores query, selected search match, filters, selection and sequence zoom. Native input caches are reseeded using document identity and a context revision so stale controls cannot overwrite restored state.
- Document switches retain undo/redo history, saved depth, unsaved status, edit-dialog drafts and saved source snapshots. PCR/assembly product creation also suspends the source session. A library edit made while a document is inactive invalidates obsolete undo state when that document returns.
- Added a destination-aware Save/Discard/Stay controller. The pending intent is retained on save failure/cancel and cannot be replaced by another navigation request. Stale source revisions block execution until the user returns with Stay. Escape invokes Stay and restores the invoking view/dialog.
- File input is parsed before a destructive decision and held for that exact intent. An invalid/missing file cannot discard the current input. Opening an already-open native ID focuses its existing draft instead of creating duplicate instances or replacing unsaved edits.
- Discard restores the saved source; a never-saved draft is discarded in full. Document close acts on the requested ID and clears its session. The tab close control now closes the current document instead of opening New Sequence.
- Application exit reviews each dirty document, including inactive library documents. Unapplied forms in any affected document block destructive navigation and reveal their owning tab for Apply/cancel: saving a document must never claim to save unapplied form values.
- Typed linked documents open by ID. Ambiguous legacy name links report a repair requirement instead of choosing the first match; complete record-link editing/routing remains P12.

Verification:

- `cargo test -p domain -p storage -p appcore --locked`: **222 passed, 0 failed** (201 prior + 21 P02 regression tests).
- `cargo clippy -p domain -p storage -p appcore --all-targets --locked -- -D warnings`: **passed**.
- `cargo check -p desktop_gpui --locked` and final `cargo build -p desktop_gpui --locked`: **passed**. The existing upstream future-compatibility notice remains.
- `cargo fmt --all -- --check` and `git diff --check`: **passed**.
- Independent review found hidden-form loss, duplicate document identity, stale native text inputs and search-hit restoration issues. All four were addressed; regression cases were observed failing before the fixes.
- Live UI smoke test: the built executable did not expose a targetable window in this session. No successful interactive/visual verification is claimed. Native window behavior and layout should be checked on a running desktop before release.

Scope: the context types support later command/record/well/pane bindings; module-specific record routes reject unsupported dispatch explicitly. P05/P10/P12/P13 wire their corresponding command, split-pane, record and well selections. P02 does not claim those later workflows are implemented.

## Implementation rulings

- Migration tests belong to storage integration tests; domain tests must not depend on storage.
- Checkpoint commits are pushed to this branch; `main` is preserved until integration is requested.
- Prototype validations in the specification describe design wiring, not application test results. The source manifest describes the original baseline and will differ after implementation.
- UUID v4/v5 and SHA-256 versions already existed in the workspace lockfile. Direct dependency features were added and validated with the installed toolchain; v5 is used only for deterministic legacy-source IDs, not security decisions.

## P03 — implemented and verified by automated checks

16 November 2026. Branch: `codex/ux-v4-p03`; base: merged P02 (`9c6802d`).

- Added a non-mutating review for interchange-file Open, New Sequence and sequence clipboard paste. Native files continue through the identity-aware open flow. Original bytes remain available throughout review.
- Review shows all FASTA/GenBank records, preserving their one-based source indices, including invalid/empty records. Checked valid records can be imported together; excluding any record requires acknowledgment. Invalid records cannot be silently skipped.
- DNA/RNA/protein interpretation is explicit. Protein input is retained and recoverably blocked from the DNA workspace, with Biologics guidance. RNA remains blocked unless the user deliberately chooses DNA interpretation and acknowledges conversion. RNA subtype declarations (`mRNA`, `tRNA`, `rRNA`, `ss-RNA`) use the same safeguards.
- U→T conversion, gap removal, topology overrides and unrepresented GenBank metadata require review acknowledgment. Unknown residues cannot disappear through generic sequence cleaning. GenBank gap removal is blocked because annotations would require remapping.
- Annotation coordinates are validated before commit for circular and linear records; malformed source positions are rejected rather than clamped or wrapped onto unrelated residues. Supported joined/complement annotations are preserved.
- Confirmed batches open as separately identified unsaved DNA documents while retaining the previous editing session. Duplicate display names create independent copies. A reviewed operation cannot be confirmed twice to create duplicate output; a changed review invalidates its prior plan.
- Paste captures the exact destination document/revision, range and replacement mode. A stale target is rejected; a confirmed paste is one undoable mutation. Annotated source input must open as a new document to avoid losing annotations.
- Saved provenance retains original per-record text, source filename, full-source SHA-256, source index and transformation report. Paste provenance also persists the captured target revision/range/mode/position. Whole multi-record source files are not duplicated into every imported document.
- The review list uses GPUI virtualization. Error messages are bounded while the complete original text remains available. Cancel/Escape restores the invoking form and context without modifying scientific data.

### Native format compatibility

Current native format is **v3**. Provenance fields require a version boundary so a prior v2 writer refuses the new file instead of silently deleting those fields on save. Existing v1/v2 documents remain readable; opening does not change their bytes. Explicit save upgrades them to v3 and preserves the exact original as `.hbx.v1.bak` or `.hbx.v2.bak`. Conflicting backups and unsupported newer destinations block overwrite. P01's v2 entries above describe that historical milestone; v3 is the current implementation.

### Verification

- `cargo test -p domain -p storage -p appcore --locked --offline`: **257 passed, 0 failed** (222 prior + 35 P03 tests).
- `cargo clippy -p domain -p storage -p appcore --all-targets --locked --offline -- -D warnings`: **passed**.
- `cargo check -p desktop_gpui --locked --offline` and final `cargo build -p desktop_gpui --locked --offline`: **passed**. The prior upstream future-compatibility notice remains.
- `cargo fmt --all -- --check`, `git diff --check` and a scan of all 95 Rust files for NUL content: **passed**.
- Independent review identified RNA subtype classification, persisted paste lineage, metadata reporting and circular-coordinate validation concerns. Fixes and regression tests cover these cases, including removal of false metadata warnings for the GenBank `//` terminator.
- Interactive UI inspection: an isolated-profile launch again did not expose a targetable native window. No successful live-window or visual-layout verification is claimed.

### Scope and follow-up

Protein/RNA specialist import is not fabricated: incompatible DNA imports are blocked with guidance; P17 supplies the later specialist workflows. Batch commit is in-memory and explicitly unsaved; durable operation recovery is P04/P16. The 64 MiB advisory appears in review; streaming, background parsing, performance budgets and large-data benchmarks remain P04/P19 release work. This milestone does not claim those later capabilities are implemented.


## P04 — implemented and verified by automated checks

16 November 2026. Branch: `codex/ux-v4-p04`; base: P03 (`af25311`).

- Native Save and FASTA/GenBank/plain-text exports capture an owned document, full version history, destination fingerprint and expected output hash before writing. A versioned per-operation journal is persisted before the output starts.
- Journals expose operation state, completion receipts and a read-only startup recovery plan. Explicit recovery recognizes already-committed bytes without replay, permits retry only against unchanged input destinations, or preserves a changed file while recovering captured output to a new destination.
- Completed retries return the original receipt without replacing later output. Cancellation before commit produces no output; an already-committed write cannot be described as canceled. A failed completion-marker write returns `PartialOutcome` and remains available for hash-based reconciliation.
- Atomic writes use unique exclusively created staging files, flush before rename, and clean only their own staging file on failure. Destination fingerprint checks and replacement share a process-local critical section. Native legacy backup and newer-version rejection remain intact.
- Save completion checks the captured full document and receipt hash before marking the current editor clean. A later checkpoint or edit cannot be cleared by an older completion. Unrelated/inactive document state is never cleared.
- Routine success uses the existing toast without a modal. Errors persist in an inline banner. Recovery exposes labeled actions for retry, recording completion, recovering to a new file, preserving a changed file, and refreshing inaccessible/corrupt journal feedback.
- Document persistence and settings persistence report distinct outcomes. A settings failure does not dirty an already-saved document or falsely report document failure. Quit and OS window-close retain the window when settings fail; the explicit settings-only exit fallback still checks active/library dirty work, pending decisions and forms before closing.
- Completed/canceled journal records discard full captured payloads; only small receipts/terminal metadata remain. Unfinished records retain captured input until recovered or explicitly dismissed. Corrupt/newer journals are preserved and surfaced rather than silently reset.

### Verification and traceability

- `cargo test -p domain -p storage -p appcore --locked`: **276 passed, 0 failed** (domain 145; storage 67; appcore 64). Includes 18 P04 integration cases and the storage completion-marker failure case.
- Required P04 cases are present in `crates/appcore/tests/ux_v4_p04.rs`: `save_rename_failure_is_not_success`, `operation_uses_captured_input`, `restart_reconciles_committed_output`, `routine_success_does_not_block`.
- Further coverage: failed journal prevents output, native rename staging cleanup, changed-file conflict, cancellation, repeated completed retry, recovery-copy protection, full checkpoint retention, stale completion, forged receipt hash, failed settings and new edits after failed Quit.
- `cargo clippy -p domain -p storage -p appcore --all-targets --locked -- -D warnings`, native `cargo build -p desktop_gpui --locked`, formatting and diff checks pass. The existing upstream future-compatibility notice remains.
- Independent review found two exit-path issues; both were fixed and reviewed again: OS close ignoring settings failure and the settings-only fallback bypassing newly edited documents. No remaining important finding was reported within this milestone scope.
- F-050 persistence foundation is implemented here; P08A retains ownership of the complete Sequence-screen controls and visual validation. Shared contracts: `02-shared-interaction-contracts.md` message taxonomy and `03-data-state-and-persistence.md` durable outcome/recovery rules.
- Interactive native-window and visual-layout verification remains unverified; a successful native build is not a live UI test.

### Scope and follow-up

This slice retains synchronous Save so existing dirty-navigation guards retain their behavior. Non-blocking routine feedback means non-modal feedback; background scheduling, streaming, in-flight cancellation and large-data performance budgets remain P19. The serial write lock coordinates app workers in one process; cross-process/network-filesystem transaction isolation and power-loss durability are not claimed. Recovery acts on captured file output only, does not retarget the currently open editor, and does not automatically resume an old navigation decision after restart. Multi-store automation journals and rollback remain P16. Existing unrelated module stores still require their owning milestone's persistence-error work.

Next: P05 — command registry and scoped workspace search.
