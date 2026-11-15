# UX v4 implementation progress

P00/P01 merged into `main` at `908788b` on 15 November 2026.
P02 branch: `codex/ux-v4-p02`.
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

P03: safe import interpretation, molecule/loss review and multi-record commit.

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
