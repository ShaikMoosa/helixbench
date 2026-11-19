# UX v6 Object-Centered Navigation Implementation Plan

> **For agentic workers:** Use superpowers:executing-plans, test-driven-development and a final whole-branch review.

**Goal:** Make Sequences, Results and Experiments recognizable destinations, preserving exact document/result identity and scientific capabilities.
**Architecture:** Extend the existing typed View/navigation model and immutable result lifecycle. A small workspace adapter owns global destination selection and open-document summaries; GPUI renders those routes with existing native controls.
**Tech Stack:** Rust 2021, GPUI 0.2.2, existing domain/storage engines and embedded assets.
**Spec:** `docs/implementation-ux-v6/README.md`; Figma page199:1316, navigation199:2198, Sequences199:1694, Results199:4370.

## Global constraints

- Preserve document/result IDs, unsaved guards, per-document sessions, sample isolation and historical captured inputs.
- Native file pickers stay native. Never turn prototype sample values into production defaults.
- Import exact v6 navigation assets; no network dependency at runtime.
- Keep legacy scientific commands reachable; Split is a layout action rather than a primary destination.
- Settings forms, advanced viewer controls and analysis workspace migration remain their owning increments.

## Review focus

- Same-name documents must open by ID, retaining their draft/undo/selection state.
- Returning from Results to Home cannot render an old hidden result instead of Home.
- A clean imported document without a path must not claim it was saved.
- Leaving a result must retain its captured output and an exact return path.
- Page creation on Experiments cannot create a DNA document.

### Task 1 â€” Typed destinations and result lifecycle

Files: `crates/appcore/src/{state,navigation,results,shell,workspace,commands}.rs`, `crates/appcore/tests/ux_v6_navigation.rs`.

- [x] Add failing real-result tests for catalog/preview destinations and source-return identity.
- [x] Add Sequences/Results views, owning navigation adapter and stable open-sequence summaries.
- [x] Preserve result return stacks while separating their route from Home; guard incomplete forms.
- [x] Test catalog filtering, duplicate names, scoped creation and return to an edited source.

### Task 2 â€” Native shell and catalogs

Files: `apps/desktop_gpui/src/{shell,app,find,icons,theme,widgets}.rs`, `views/{sequences,results,mod}.rs`, `lab.rs`, assets/ux-v6.

- [x] Import and document exact v6 assets with checksums and local dimensions.
- [x] Group Home/Sequences/Results/Experiments, Lab records and Lab operations; retain labeled specialist expansion.
- [x] Add persistent breadcrumbs/search and owner-specific actions. Remove generic DNA New from lab-page headers.
- [x] Render native sequence catalog and direct result catalog with useful metadata and clear empty/reset states.
- [x] Replace scientific view buttons with underline tabs and move Split into a labeled layout control.

### Task 3 â€” Verification and review

- [x] Run focused tests, full workspace tests, format, all-target/all-feature Clippy and build.
- [ ] Check Home â†’ Sequences â†’ Map â†’ Results â†’ source/Back, Experiments creation, and 1280/1600 layouts in the native app.
- [x] Review the complete diff, update progress and publish a reviewable branch/PR. Main is not changed by this implementation step.

## Interface pre-flight

Task 2 consumes Task 1's View variants, workspace summaries and Results routes. ResultsState retains its existing return stack; no new persistence schema is introduced. Existing P06 expectations that saved results live in Home must migrate to the explicit Results contract.

## Verification limit

Native 1600px flow was inspected before final review fixes. Rebuilt 1280px and native session-resume confirmation remain pending because the Windows helper timed out after bounded recovery. See the V6-02 progress entry; keep the PR draft until these checks pass.
