# Connected journeys and state machines

## J-01 · First meaningful sequence result

1. Home → Import source
2. Recognized-content and transformation review
3. Open the exact sequence ID/revision
4. Choose restriction analysis with explicit scope
5. Review settings → run against captured input
6. Inspect completed/no-match/failed outcome
7. Save named output with provenance
8. Leave → find by recent work/search → reopen the same result

**Owning screens:** home, sequence, enzymes, history.

## J-02 · Blank sequence

1. Create → enter supported DNA and topology
2. Review valid input or correct field errors
3. Create a newly identified document
4. Edit/save/analyze its own sequence
5. Refind the new document; do not return to the demo

**Owning screens:** home, sequence.

## J-03 · Sample to personal work

1. Choose labeled sample
2. Inspect/run within sample namespace
3. Exit safely or review personal copy
4. Allocate independent document ID/revision
5. Run a new personal analysis with a new result ID

**Owning screens:** home, map, enzymes.

## J-04 · Notebook and results

1. Create a blank entry or deliberately apply a template
2. Link a reviewed source revision
3. Write notes and add a structured table
4. Validate/submit result rows
5. Capture a submitted local-review revision
6. Record changes requested → amend draft → resubmit
7. Inspect the correct read-only submitted revision

**Owning screens:** notebook, registry.

## J-05 · Physical sample custody

1. Choose storage/container and inspect an exact well
2. Choose a single assignment or an explicit multi-well set
3. Review existing/proposed contents and units
4. Validate source/destination transfer mappings
5. Commit a recoverable operation
6. Refresh live state and retain immutable notebook snapshots

**Owning screens:** inventory, notebook, registry.

## J-06 · Archive, delete and retained history

1. Select exact typed record
2. Archive/review/restore using the same identity
3. For permanent delete, confirm ID and revalidate dependencies
4. Block live/unrecoverable dependencies or commit deletion
5. Clear selection and caches
6. Open retained historical result via its snapshot/tombstone

**Owning screens:** registry, history.

## J-07 · Automation recovery

1. Review input, immutable pipeline version and output target
2. Validate parsed rows before writeback
3. Start a uniquely identified operation
4. Cancel only at a supported boundary
5. Inspect completed/failed/partial outcome
6. Reconcile retained output IDs before recovery
7. Write remaining outputs without duplication and refind by run ID

**Owning screens:** automation, notebook, registry.

## J-08 · Source change and scientific results

1. Inspect the saved result’s captured input
2. Edit/save a newer source revision
3. Label older-input result without overwriting it
4. Review rerun inputs and allocate a new result ID
5. Compare method/parameters/outputs and export provenance

**Owning screens:** history, enzymes, split.

## SM-IMPORT · Import review

**Initial state:** Idle.

**States:** Idle, Parsing, ReviewReady, ReviewIssues, DuplicateDecision, Committing, Opened, CommitFailed.

- Idle → choose source → Parsing; invariant: existing work retained.
- Parsing → recognized valid content → ReviewReady; invariant: original retained.
- Parsing → issues → ReviewIssues; invariant: no silent cleaning.
- ReviewIssues → correct and revalidate → ReviewReady; invariant: diff reviewed.
- ReviewReady → matching document → DuplicateDecision; invariant: choose open/skip/copy.
- ReviewReady → confirm selected records → Committing; invariant: typed plan.
- Committing → persist/open succeeds → Opened; invariant: exact new IDs.
- Committing → write fails → CommitFailed; invariant: no false success.

## SM-GUARD · Unsaved navigation

**Initial state:** Editing.

**States:** Editing, Decision, Saving, SaveFailed, Continuing.

- Editing → loss-producing intent → Decision; invariant: capture exact intent/source.
- Decision → Stay or Escape → Editing; invariant: return invoking view.
- Decision → Save → Saving; invariant: retain intent.
- Saving → cancel or failure → SaveFailed; invariant: do not continue.
- Saving → save succeeds → Continuing; invariant: perform intent once.
- Decision → Discard → Continuing; invariant: retain saved checkpoint.

## SM-ANALYSIS · Analysis/result lifecycle

**Initial state:** Prerequisite.

**States:** Prerequisite, Ready, Running, Completed, NoMatches, Failed, Canceled, Saved, OlderInput, UnavailableSource.

- Prerequisite → valid typed input → Ready; invariant: capture scope.
- Ready → Run → Running; invariant: immutable snapshot.
- Running → success with matches → Completed; invariant: method/parameters retained.
- Running → success without matches → NoMatches; invariant: not failure.
- Running → supported cancellation → Canceled; invariant: stage truthful.
- Completed → persist output → Saved; invariant: new result ID.
- Saved → source changes → OlderInput; invariant: old output unchanged.
- Saved → source deleted and snapshot retained → UnavailableSource; invariant: tombstone; no resurrection.

## SM-NOTEBOOK · Local notebook review

**Initial state:** Draft.

**States:** Draft, InReview, ChangesRequested, Approved, Archived.

- Draft → submit reviewed revision → InReview; invariant: required results persisted.
- InReview → request changes → ChangesRequested; invariant: comment and snapshot retained.
- ChangesRequested → resubmit next revision → InReview; invariant: same entry ID.
- InReview → approve locally → Approved; invariant: immutable approved snapshot.
- Approved → archive → Archived; invariant: retain historical references.

## SM-RECORD · Archive and delete

**Initial state:** Active.

**States:** Active, Archived, DeleteReview, Blocked, Deleted, Failed.

- Active → confirm archive → Archived; invariant: correct record ID.
- Archived → restore → Active; invariant: same identity.
- Active → request permanent delete → DeleteReview; invariant: dependencies reviewed.
- DeleteReview → live dependency or missing snapshot → Blocked; invariant: no mutation.
- DeleteReview → matching confirmation and valid plan → Deleted; invariant: selection cleared; tombstones retained.
- DeleteReview → persistence fails → Failed; invariant: original retained.

## SM-TRANSFER · Inventory transfer

**Initial state:** Draft.

**States:** Draft, Validated, Review, Committing, Completed, RecoverableFailure.

- Draft → validate all mappings/units → Validated; invariant: whole ID set.
- Validated → present impact → Review; invariant: source/destination quantities.
- Review → confirm → Committing; invariant: journal operation.
- Committing → all stores committed → Completed; invariant: source and destination updated.
- Committing → interrupted/failed → RecoverableFailure; invariant: reconcile before retry.

## SM-AUTOMATION · Automation run

**Initial state:** Reviewing.

**States:** Reviewing, Queued, Running, CancelRequested, Canceled, Completed, Failed, PartialOutcome, Reconciled.

- Reviewing → validated input/target → Queued; invariant: immutable template revision.
- Queued → start → Running; invariant: unique run ID.
- Running → request supported cancel → CancelRequested; invariant: do not claim canceled yet.
- CancelRequested → safe boundary reached → Canceled; invariant: written outputs explained.
- Running → all output commits succeed → Completed; invariant: truthful row counts.
- Running → validation fails before write → Failed; invariant: no records written.
- Running → some outputs committed → PartialOutcome; invariant: retain output IDs.
- PartialOutcome → reconcile journal → Reconciled; invariant: retry remaining only.

## SM-SEARCH · Scoped search

**Initial state:** Recent.

**States:** Recent, Loading, Results, NoMatches, Failed.

- Recent → query/scope changes → Loading; invariant: new request identity.
- Loading → matching rows → Results; invariant: typed IDs.
- Loading → no matching rows → NoMatches; invariant: source is not necessarily empty.
- Loading → load error → Failed; invariant: query retained.
- Results → clear query → Recent; invariant: clear explicit scope as requested.

