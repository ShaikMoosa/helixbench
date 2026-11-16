# UX v4 implementation progress

P00/P01 merged into `main` at `908788b` on 15 November 2026.
P00–P05 are merged into `main` at `97c43f3` on 16 November 2026. P06 continues on `codex/ux-v4-p06`.
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


## P05 — implemented and verified by automated checks

16 November 2026. Branch: `codex/ux-v4-p05`; base: merged P04 (`054d249`).

- Added a shared catalog of 22 existing tools/actions and 19 page commands. The Tools menu, owning-page labeled tool entries and workspace search use the same command IDs, labels and availability checks. Known prerequisites explain missing DNA, extra assembly inputs, circular topology, or an open form/decision; availability is rechecked on activation.
- Owning-page tool entries are directly visible and activate in one click. This fulfills the two-click access limit for the registered tools; complete feature controls on the remaining module screens still belong to P08–P17. Existing algorithm entry points and scientific behavior are reused.
- Added a visible **Search workspace · Ctrl+K** entry, separate from **Ctrl+F** sequence Find. Search scopes are All, Open documents, Notebook, Registry, Inventory, Workflows, Projects, and Tools & pages. Archive inclusion is explicit; matching supports names, IDs, available notes/fields and tool keywords.
- Results use `DocumentId`, store-qualified `RecordRef`, project ID or `CommandId`, never a display name alone. Duplicate names remain distinct. Result activation revalidates live existence and tool availability. Deleted records cannot fall through to legacy create-on-missing editors.
- Record results open current read-only detail on their owning page. Notebook/Registry have explicit edit entry points; locked notebook entries and archived Registry entities remain read-only with a reason. Other module editors and deeper relationships remain owned by their later milestones.
- Explicit **Back to search** preserves scope, query, archive choice, selected target, visible-result count and the native scroll handle. Closing search restores its captured document/view, sequence Find, selection, zoom, filter and sort context via P02. If its original document was closed, search closes safely on the current page and explains the missing source.
- Request generations and query/scope matching reject late replies and replies received after closing search. Ordinary page/document navigation, already-open native-file handoff and successful import leave search; invalid or canceled import preserves search. Keyboard focus returns from an unmounted query input to the current document shell.
- Arrow keys select search results and Enter opens them; Escape closes search. Query text editing stays inside its input. The search page suppresses sequence edit shortcuts and Edit/Features/Primers/Enzymes menu actions, with core Cut/Delete/Insert and undo/redo guards preventing hidden DNA changes from stale edit events.
- Search displays empty/no-match guidance, explicit refresh, live deletion feedback, archived status, tool prerequisites and progressive result disclosure instead of silently truncating matches.

### Verification and traceability

- `cargo test -p domain -p storage -p appcore --locked`: **297 passed, 0 failed** (domain 145; storage 67; appcore 85). P05 contributes 21 integration cases.
- Required tests in `crates/appcore/tests/ux_v4_p05.rs`: `owning_page_tool_entry_within_two_clicks`, `search_back_restores_scope`, `late_search_reply_is_ignored`, `deleted_record_is_not_reopened`.
- Additional observed-failing regressions were fixed for activation after closing search, closed-origin recovery, ordinary page navigation, hidden-sequence editing, locked notebook editing, already-loaded native file handoff, successful import handoff, and preserving canceled import.
- Strict core Clippy, native executable build, formatting, diff checks and source/manifest integrity checks passed. A Windows LNK1318 PDB error on the final incremental link required a non-incremental native rebuild; no product source change was used to bypass the linker.
- Independent review identified hidden Edit-menu DNA mutation and two successful-open handoffs; fixes and their regression cases were reviewed again. No remaining important finding was reported within P05 scope.
- UX-06 and UX-07 have implementation/test evidence in this milestone and the feature traceability catalog. Live native-window interaction and visual-layout QA remain unverified; compiled bindings and core tests do not prove those checks.

### Scope and follow-up

Search covers loaded local workspace data. It does not crawl unopened files or remote services. Saved-result search awaits the actual P09 catalog/lifecycle; specialist entities await P17. Search currently runs synchronously; generation-aware reply handling prepares for P19 scheduling/performance work. Full module detail/edit flows, application-wide accessibility, screen-reader announcements and large-data budgets retain their later milestone ownership. No complete Soft Gallery shell migration is claimed before P07.

Next: P06 — first-use Home, samples and resume.


## P06 — implemented and verified by automated checks

16 November 2026. Branch: `codex/ux-v4-p06`; base: merged P05 (`97c43f3`).

- Startup now opens Home with an empty document placeholder and the existing local workspace stores. It neither silently opens a recent file nor falls back to a seeded demo. Existing documents/records remain unchanged until the user chooses work.
- Home exposes Import sequence, New sequence, Try sample, New notebook entry, explicit document Resume, typed recent work, recent-file recovery, and saved-result refind. The welcome guidance can be hidden through a persisted preference without removing these actions.
- Samples use an independent AppState and unique temporary storage directory under `HelixBench-samples/<operation ID>`. The complete personal AppState is suspended and restored intact; sample projects, records, analysis stores, settings, recents and recovery journals route to the sample profile.
- Per-instance profile routing covers all 44 existing appcore store/config write sites and the journal UI. Normal personal profiles retain their existing filenames and storage location. No global environment-variable override is used.
- A persistent sample banner explains the temporary workspace. Native Save/Save As require an explicit reviewed personal copy. Copy review names the active sequence and retained sequence/annotation/primer/tag/note/history scope; sample results, other sample documents and lab records are excluded. Confirmation checks the full captured document, allocates independent document/revision IDs including history snapshots, restores personal work and opens the copy as unsaved work.
- Leaving a sample asks for a consequential choice. Quit/window-close first leaves the sample, restores the personal workspace and then runs its existing dirty-document/form guards; suspended unsaved work cannot be skipped.
- Native UI input caches and scroll/calendar state follow the same suspend/restore boundary. Sample search text and dosing drafts cannot overwrite restored personal values; discarded sample UI state is not reused by another sample session.
- Resume targets are typed as document ID plus optional file path, store-qualified record ID, result ID, or a legacy file link. A known-document resume validates identity against the single captured native read before invoking any dirty-work decision. Wrong-file, missing-record and unavailable-unsaved-draft cases never open a demo or create a replacement record.
- Home current/library documents, calendar entries, recent entries and worklist tasks use identity-qualified routes. Record resume opens the exact current read-only detail; later module lifecycle/edit work remains owned by P11–P14.
- Recent work deduplicates document identity across Save As path changes. Missing legacy file links expose persistent Home feedback, explicit alternate-file Open and Forget; known document links additionally offer Locate with ID validation. Failed preference/Forget writes preserve the prior settings state instead of partially removing links.
- Existing immutable result catalogs use the shared `results.json` path. Home refinds exact result IDs and displays the recorded method, input identity/revision, parameters and output read-only, without changing the active sequence. Corrupt/newer catalogs remain preserved and surfaced. No fabricated analysis result is inserted.

### Verification and traceability

- `cargo test -p domain -p storage -p appcore --locked`: **314 passed, 0 failed** (domain 145; storage 67; appcore 102). P06 contributes 17 integration cases.
- Required cases in `crates/appcore/tests/ux_v4_p06.rs`: `existing_workspace_not_replaced_by_demo`, `sample_copy_is_personal_and_independent`, `resume_opens_exact_entry_task_or_result`.
- Additional checks cover real project writes in the sample namespace, preserved personal undo/dirty state, sample Quit guards, native Save refusal, exact moved-file identity, dirty-guard resume, stale copy checkpoints, invalid result catalogs, preference/Forget failures, recent Save As deduplication and sample/personal cached-input restoration.
- Strict core Clippy, final native executable build, formatting/diff checks and source/manifest integrity checks passed. The native build used `CARGO_INCREMENTAL=0` to avoid the previously observed Windows PDB cache failure. The existing upstream future-compatibility notice remains.
- Independent review found cached-input crossover and a calendar entry handler bypassing typed resume. Both were corrected and reviewed again; the reviewer independently passed the 17 focused tests and reported no remaining important issue. Final Home resume/file handlers were also spot-checked.
- Feature traceability updates cover Home F-001/F-002/F-003/F-004/F-005/F-006/F-007 and UX-01/UX-02/UX-19 at their implemented entry/routing scope. Complete later module workflows and live visual validation are not implied.
- Live native-window interaction and visual-layout QA remain unverified. A compiled UI and core tests do not establish successful visual acceptance.

### Scope and follow-up

The sample profile is temporary and is not automatically resumed on restart; profile files may remain in the OS temporary directory. Explicit exports still write the user-chosen destination. Personal copying transfers the reviewed active document only; other sample artifacts stay isolated. Saved-result refind/inspection is read-only here; P09 retains creation, tool-specific presentation, archive, rerun and comparison. Existing module persistence-error behavior beyond the routed namespace still belongs to its owning milestones. P07 owns the complete Soft Gallery shell/right panel, and P19 owns application-wide accessibility, performance and background execution.

Next: P07 — Soft Gallery shell and contextual tools.


## P07 — Soft Gallery shell and contextual tools

16 November 2026. Branch `codex/ux-v4-p07`; base merged P06 `2edc483`.

- Shared chrome now uses the approved Home/Map/inspector design roles, embedded
  DM Sans, full-color logo, warm center, labeled 200px navigation, pill actions,
  sequence tabs and reusable surfaces. Scientific colors and dynamic canvases remain.
- A typed context inspector follows actual document/revision, feature, primer,
  valid range, record and result identities. Search shows its own scope and query.
  Actions revalidate context; stale IDs or open forms cannot retarget a mutation.
- Page tools opens a shared catalog in one click; choosing a tool is the second.
  Missing prerequisites are visible. Edit feature/primer and related-record routes
  use current IDs. Add primer from range requires an explicit valid nonempty range.
- At1600px the320px inspector docks; at1280px it opens as an explicit drawer.
  Close persists across resize and returns focus to the originating control.
  Ctrl+Shift+I toggles it. Circular/linear representation does not change topology.
- Native visual checks covered Home1600, live sample Map, linear representation
  with circular topology preserved, Close→Enter reopening Page tools, Home1280,
  drawer opening and scrolling through PCR/primer tools. The visual pass found
  clipped inspector content and compressed controls; width bounds and a separate
  scroll viewport corrected both. These observations do not establish complete
  accessibility or every module's visual acceptance.
- Automated verification: 325 core tests passed (11 P07 cases), strict core Clippy,
  non-incremental native executable build, formatting and source/diff checks.
  Upstream proc-macro-error2 future-compatibility notice remains.
- Fresh branch review found cleared-range primer fallback and incorrect Close
  focus target. Regression cases and validated range extraction corrected the
  former; the native Close handler now consumes the stored opener and the native
  Page tools→Close→Enter check verified reopening. No additional reviewer is claimed.

Scope decisions: approved fixture names/arcs are replaced with live data; module
screens retain P08–P17 ownership. The empty Home reference has no illustration
slot. Darker secondary/selected text improves contrast. Native menus remain to
preserve existing capabilities. P19 retains full accessibility/performance work.

Follow-up: P08A — map, sequence and coordinate synchronization. The legacy linear
canvas caption says “linear” for the representation; clarify its view/topology
wording alongside coordinate/selection UX. Inspector topology and document status
remain authoritative and unchanged. Detailed scientific label contrast/overlap,
zoom fitting and lower Home content density also retain their owning screen work.


## P08A — map, sequence and coordinate synchronization

16 November 2026. Branch `codex/ux-v4-p08a`, stacked on P07 `d0583ab`.
Main remains at P06 until an explicit merge. Figma Sequence107:29677 was inspected
with code and screenshot; shared Soft Gallery widgets and existing embedded assets
are reused, with live scientific data replacing reference fixtures.

- Added validated source/revision-bound selection and 1-based inclusive conversion,
  ordered circular segments and independent strand. Invalid coordinates, stale source,
  open forms and workspace search cannot replace a valid selection silently.
- Navigation captures/restores strand; reverse features and DNA/protein search hits
  retain top-coordinate ranges plus the selected strand. Circular/linear representation
  captions distinguish the view from actual DNA topology.
- Direct Find, Annotation list, full-sequence and zoom controls improve discoverability.
  Shared feedback and the inspector agree on origin crossing, length and strand.
  Map/Sequence clear actions remove dependent feature/primer identities.
- Complement-row drags use the pressed strand. Whitespace/translation rows do not
  retag selections; selecting the opposite strand at the same base does not clear it.
  Map primer clicks carry exact binding-site range/strand. The primer list explicitly
  chooses its first site, preserving strand; an unbound primer clears previous DNA range.
- Labeled display toggles preserve dirty/history/document state. Features Off now
  suppresses map rendering and hit testing. Circular selections paint both ordered
  segments in a linear view and sequence minimap. Short/empty zoom bounds are safe.
- The scientific canvas receives space previously occupied by persistent footer cards;
  metadata cards remain in Metadata and the annotation list is directly accessible.
  Annotation text uses readable ink while scientific feature/base colors remain intact.

Verification:339 core tests passed, including14 P08A integration cases;1 new native
renderer/hit-test case passed. Strict core Clippy, non-incremental native build,
format/diff checks and manifest/source integrity checks passed. The upstream
proc-macro-error2 future-compatibility notice remains.

Fresh review reported three important UI binding issues (Features Off, whitespace
strand changes, primer-list reverse strand). All were fixed in one pass with observed
failing regressions followed by passing focused/full checks. Native1600px checks
confirmed complement-row reverse selection, gutter click preserving it, view-switch
retention and the Features Off result. No final P08A1280 visual acceptance or complete
accessibility/large-data performance claim is made.

Rulings: selection strand is a view attribute on top-strand coordinates. Copy keeps its
existing top-strand behavior, with explicit guidance and reverse-complement commands.
No scientific algorithm is changed to infer reverse output from a display selection.
The branch is stacked on the unmerged P07 milestone to preserve its prerequisite.

Follow-up: P08B feature editing/annotation review, P08C primer design/PCR and P08D
enzyme/digest/history own their detailed action/review flows. Mutation impact previews
and result lifecycle are not claimed complete here. Dense-label layout, application-wide
accessibility and large-data performance retain their later acceptance work.


## P08B — feature editing and annotation review

16 November 2026. Branch `codex/ux-v4-p08b`; base merged main `0a9acca`.
Figma Features107:29886 code and screenshot inspected; existing Soft Gallery tokens,
buttons, modal shell, table and embedded assets reused with live document data.

- Feature drafts capture source document/revision and stable feature ID. Save validates
  name/type, ranges, genetic code and frame, rejects missing/stale targets before mutation,
  preserves all editor fields plus visibility/cleavage metadata, selects the resulting ID
  and keeps Undo. An unchanged edit creates no checkpoint. Delete clears selected range.
- Circular wrap and legacy unrolled spans remain accepted. Joined overlaps are preserved
  with a visible warning instead of being newly rejected; this protects existing editor
  semantics. Table action closures validate their captured source even after sorting.
- Detection and bulk library annotation now prepare review drafts rather than applying
  immediately. Candidates show source motif, strand, coordinates and overlap/duplicate
  warnings; overlap candidates start unchecked. Per-document selection and Apply selected
  produce explicit added/skipped/error outcomes. Cancel leaves data unchanged.
- Bulk application uses the existing document-session controller, retaining dirty state,
  per-document history and Undo; stale sources/closed documents report failures. These
  results describe unsaved document changes. No disk-persistence success is implied.
- Features exposes Review auto-annotation directly. Toolbar wraps at narrower widths and
  table sizing accounts for the actual shell. Empty annotations and no-filter-match copy
  are distinct. Existing filters, sorts, columns, saved views and translation algorithms
  remain; no unverified redesign of those workflows is claimed.
- Reused scientific matching algorithms. Library exact matching keeps its existing-name
  skip rule, disclosed during review. Empty/invalid motifs and duplicate library names
  cannot generate ambiguous source-motif previews. Common detector remains unchanged.

Verification:355 workspace tests passed, including10 domain and5 appcore P08B cases.
Required tests feature_edit_retains_id_after_sort, annotation_preview_cancel_is_noop
and bulk_annotate_reports_per_document_outcome exist in domain/tests/ux_v4_p08b.rs.
Additional cases cover stale/missing features, all editor fields/ranges, frame validation,
proposal overlaps, invalid motifs, duplicate-name provenance, legacy joined overlaps and
undoable per-document application. The previous bulk identity regression now explicitly
applies a review while retaining its revision/dirty assertions.

Fresh review found one important issue: duplicate library names could show the first
entry's motif for a candidate generated by a later entry. A failing regression reproduced
it; duplicate names now block review. Compatibility inspection separately reproduced and
fixed overly strict joined-overlap validation. Native1280px verification confirmed editor
errors, review, conflicts, Apply, outcomes, Undo, Cancel and the rebuilt overlap warning.
Strict core Clippy, native build, format/diff/source integrity and manifest checks passed.
The existing upstream proc-macro-error2 future-compatibility notice remains.

Rulings: preserve scientific matching and legacy joined-segment semantics; provide review
and warnings rather than silently changing them. Duplicate library names must be corrected
before preview so source provenance is reliable. Cost: an ambiguous legacy library needs
cleanup before bulk annotation. Application-wide accessibility, large review performance
and other tool-specific reviews remain in their owning milestones.

Next: P08C — primer design and PCR context. P08B is published separately; main is unchanged
until an explicit merge.


## P08B merge and P08C — primer design and PCR

16 November 2026. P08B fast-forward merged and pushed to main at `de3e030`;355 tests
passed on merged main. P08C branch `codex/ux-v4-p08c` starts at that merge.
Figma Primers107:30093 code/screenshot inspected; live native controls reuse Soft Gallery.


## P08C implemented flow and evidence — 16 November 2026

From Primers, Design pair and Simulate PCR are direct labeled actions. Design captures
the template document/revision and sequence digest. Its selection anchor and strand
remain separate from the editable amplification target. Target start is 1-based; length
must fit the linear molecule or one circular traversal. Invalid numeric input, unsupported
DNA symbols, nonfinite/out-of-range Tm and invalid overhangs retain the draft with errors.
No silent default or clipping is applied after invalid input.

Run design uses the existing candidate engine. Results disclose the existing nearest-
neighbor Tm conditions and structure heuristics; each shows body length, 5′ tail, GC,
Tm, position and hairpin/self-dimer indicators. Both candidate choices start unselected.
Adding requires current inputs, exact captured candidate vectors and explicit forward/
reverse picks. One undoable mutation adds unique primer IDs and immutable pair provenance.
Add pair and open PCR transfers those actual IDs, with no prototype fixture defaults.

PCR accepts explicit saved-primer ID choices, unique names or validated entered DNA.
Duplicate names require ID selection. Deleted IDs never fall back to another primer.
Choosing a saved primer updates the native text field and clears the prior prediction.
Run captures both resolved sequences/IDs, template version/digest, binding parameters,
pair identity where applicable and copy policy. No binding or incompatible orientation
is a successful diagnostic. A changed source, sequence, primer or copy policy requires
rerunning before creation. Select an explicit product, then Create independent product.

Product sequence includes unmatched forward 5′ tail, template body and reverse-complement
of the reverse 5′ tail. Linear products may end at the final base; circular products may
cross the origin but cannot anneal through repeated template laps. The product gets new
document/revision IDs, linear topology, parent genetic code and immutable PCR provenance.
The unsaved parent session and its Undo history remain open. Open parent template returns
only to the exact captured open version; unavailable history is reported honestly.

Copy policy retains only visible feature spans fully contained in the product body.
Joined-segment order is preserved; joins that would require reordering across a circular
origin are skipped. Segment and absolute cleavage-marker coordinates shift by the forward
tail; out-of-body cleavage markers are omitted. Partial/hidden features are not copied.
Repeated names remain distinct through document IDs. Creation is unsaved: normal P04 save
and journal semantics apply, and a failed save retains product, provenance and parent.

Native format is now v4 so older v3 writers reject rather than silently drop new metadata.
Versions1–3 remain readable; upgrading a v3 file retains its exact .v3.bak backup. Pair and
PCR provenance round-trip; older files default to no provenance. The change does not add
a P09 saved-result catalog lifecycle or a durable historical-version restore service.

Prediction is synchronous and bounded: more than512 binding-site pairs or an estimated
pair-count × template-length above20,000,000 bytes produces an actionable error before
product materialization. No partial result list is presented as complete. General async
work and large-data performance/accessibility acceptance remain P19.

Evidence:372 workspace tests passed, including14 appcore,2 domain and1 storage P08C tests;
strict core Clippy and native executable build passed. Required regressions are
primer_target_is_distinct_from_anchor, stale_candidate_cannot_be_added and
pcr_product_keeps_parent_provenance. Additional coverage includes invalid inputs, stale
copy policy/source, missing/duplicate primers, no-product diagnostics, undoable pairs,
tails, last-base products, joined features, cleavage markers, repetitive inputs and save
failure. Fresh review identified stale rendered-row source and circular join ordering;
both regressions were observed failing and then passing with the fixes.

Native1280px verification observed direct entries, disabled unselected-pair actions,
candidate metrics, explicit choices, pair-to-PCR handoff, saved-ID text-field reseeding,
prediction invalidation, selected502bp product, independent dirty product/parent tabs,
parent provenance and return to the original dirty parent session. This is bounded sample
verification, not comprehensive keyboard/accessibility or large-data acceptance.


Rulings: retain existing design/Tm/structure algorithms and disclose their conditions.
Fix PCR assembly/accounting for 5′ tails and linear last-base products. Preserve joined
feature semantics by skipping copies that require reordering. Bound synchronous prediction
explicitly rather than returning a truncated list. Native v4 is required for durable
provenance; older apps cannot open v4, while v1–3 files remain readable with upgrade backup.

Next: P08D — enzyme filters, digest and history. P08C is published on its feature branch
for review; merging P08C into main awaits the user's next integration instruction.
