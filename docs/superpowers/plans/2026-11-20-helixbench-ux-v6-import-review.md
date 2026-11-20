# UX v6 import review implementation plan

> **For agentic workers:** Use superpowers:executing-plans to implement this plan task by task.

**Goal:** Deliver the first V6-04 slice: a readable import-review workspace with visible record status and a persistent, validated confirmation action.

**Architecture:** Retain the existing captured import draft, original bytes, atomic commit and navigation guard. Render file-import reviews in the central workspace, using the existing dialog state as the guarded task owner. Keep reviewed paste and new-sequence forms compatible. Separate overview, interpretation and original-input detail so corrections remain directly accessible.

**Tech stack:** Rust, GPUI 0.2.2, existing embedded assets and semantic theme.

**Spec:** `docs/implementation-ux-v6/README.md`, V6-04; Figma `199:1994` and `199:2107` (inspected 20 November 2026). This is an increment of the already approved v6 adoption, not a new visual direction.

## Constraints

- Preserve scientific behavior, source bytes, document identity, captured insertion targets and save semantics.
- Never silently drop invalid records or convert RNA. Ask only for acknowledgments relevant to the selected subset.
- No native-file overwrite, new dependency, persistence schema or hard-coded sample results.
- User waived exact 1600px acceptance; use available native widths.
- Digest/PCR workspaces, feature/primer drawers and result tabs remain subsequent V6-04 slices.

## Review focus

- Mixed DNA/protein/invalid batches show exclusions and cannot commit an unacknowledged subset.
- Selecting an additional converted record invalidates previous acknowledgments.
- Switching detail tabs and records retains edited names and choices.
- Short windows retain the footer; long lists scroll independently.
- Escape/Cancel and guarded navigation preserve prior work; paste retains its captured target.

## Task 1 — Validated review presentation

- [x] Add core behavior tests for summary counts, selected conversion acknowledgment, selection changes and invalid selections.
- [x] Watch the new tests fail, then implement borrowed validation and selection helpers; keep `reviewed_plan` as the single commit-plan constructor.
- [x] Run import regressions and the existing preservation suite.

## Task 2 — Native review workspace

- [x] Render file review in the central workspace with task progress, record table, detail tabs, scope acknowledgment and fixed footer.
- [x] Reuse current interpretation controls; show original input only in its named tab. Preserve compatibility for paste/new-sequence review.
- [x] Suppress unrelated header/tabs/inspector while the guarded task is active; provide accurate breadcrumbs.
- [x] Confirm keyboard targets and native 1280px/available maximized layout in a bounded visual pass.

## Task 3 — Verification and delivery

- [x] Reconcile roadmap/progress with PR #3/#4 merges and the user's screen-width waiver.
- [x] Run format, Clippy, full workspace tests, build, source integrity and diff checks.
- [x] Conduct one independent review of the complete branch, fix important findings and verify.
- [ ] Commit, push a reviewable PR and check its GitHub Actions result. Do not merge without a new merge instruction.

## Execution record

Task 1: six new regressions, plus 13 existing import tests. Missing helpers were observed
failing before implementation; review-fix tests also failed before the fixes. Final
full suite: 447 passed. Logs are external task evidence (`work/v604-verified-*.log`).

Task 2: native 1280px and available 1536px confirmation, real NCBI pUC19 import, mixed
record exclusion and acknowledgment, retained source/old context, fixed footer and
keyboard tab activation observed. No exact 1600px requirement remains.

Final review: one independent read-only reviewer. Fixed validation timing and repeated
full-sequence scans; raw-length formatting also gained a whitespace-count regression.
Minor (deferred): explicit root focus restoration after Escape cancellation.
Minor (deferred): redundant import activity guards in Find/Escape handlers.

Implementation choice: retain Dialog::ImportReview as the guarded task owner while
rendering file reviews in the workspace. This preserves the existing atomic import
and return-location contracts; a separate choose-input page remains a later slice.
Remote delivery/Actions evidence is maintained in the PR to avoid treating a local
check as a remote pass.
