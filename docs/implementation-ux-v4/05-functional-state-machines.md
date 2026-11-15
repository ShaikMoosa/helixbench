# Functional state machines and transitions

## Import

Idle → SourceChosen → Parsing → ReviewReady or ReviewIssues → Correcting/Selection/DuplicateDecision → Committing → Opened or CommitFailed. No parse review commits data. Cancel returns to the source task with existing documents intact. Partial import explicitly records accepted/rejected records and preserves their original order/index.

## Document draft

Clean → Editing → Dirty. Save success creates/updates the persisted checkpoint then Clean; save cancel/failure remains Dirty. Undo may return to the saved state; editing after Undo clears incompatible redo. A guarded intent adds AwaitingDecision → Saving/Discarding/Stay; execute the original intent once only.

## Analysis

UnavailablePrerequisite → Ready → ReviewingParameters → Queued/Running → Completed, CompletedNoMatches, Failed or Canceled. Saving a result is a separate persistence state. A completed result whose source changed becomes OlderInput, not recomputed in place. A deleted source becomes UnavailableSource with retained snapshot when policy allows.

## Sample

Inactive → ActiveSample → SampleResult. CopyReview → PersonalCopyCreated allocates new identity; a new personal analysis has a new result ID. ExitDecision → Stay or Exit preserves the separation. No sample action silently substitutes a personal record.

## Search

Idle/Recent → Loading → Results, NoMatches or Failed. Scope/query changes produce a new request identity; obsolete replies cannot replace newer results. Opening a row stores its ReturnLocation; Back restores that query, scope, filters and selected record.

## Registry lifecycle

Active → ArchiveReview → Archived → RestoreReview/RestoredActive. DeleteReview → ConfirmationInvalid/Valid → DependencyBlocked or Committing → Deleted or Failed. Cancel returns unchanged. Concurrent dependency changes trigger revalidation. Deleted selection is cleared and retained references use tombstones.

## Notebook

Draft has saved prose and independently unsubmitted tables. SubmitRows validates/stages/persists reusable result records. SubmitReview captures revision 1 and enters InReview. ChangesRequested allows an author draft; Resubmit captures the next revision. Approved/Archived remain protected. All events are local records until a remote service exists.

## Inventory

InspectWell and SelectMany are separate contexts. AssignmentDraft → Validate → OccupiedReplacementReview if needed → Commit → Updated/Failed. TransferDraft → ResolveMappings → ValidateAll → Review → CommitAll → Completed or RecoverableFailure. SnapshotCreated is immutable and does not subscribe to live-container updates.

## Workflow

Keep current TaskStatus Pending/InProgress/Completed/Failed/Skipped and RequestStatus Draft/Sent. Missing mappings are explicit blockers with remedies; do not invent an unsupported Completed state for a failed mapping. Executor input resolution → step execution → output commit → cursor advance; failed commit leaves the cursor recoverable.

## Automation

PipelineDraft → PublishedRevision. RunReview → Queued → Running → Completed/Failed/Canceled/PartialOutcome. CancelRequested is stage-aware. A pre-write validation failure may start a new corrected-input run. PartialOutcome requires reconciliation of written output IDs before a reviewed recovery run; never blind-retry an uncertain commit.

## Bioprocess

Preserve Draft → Executing → Completed/Aborted, with explicit allowed actions and immutable execution source context. Required unit-operation completion and replicate data determine completion availability. Recipe edits do not rewrite an already planned/executing run.

## Shared surface states

Each interactive control specifies default, hover, pressed, focused, disabled and busy when applicable. Each collection specifies loading, populated, first-use empty, filtered empty, failed and unavailable source. Disabled actions include the reason and remedy; routine success is non-blocking.

