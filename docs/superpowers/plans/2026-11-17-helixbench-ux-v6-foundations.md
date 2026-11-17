# HelixBench UX v6 Foundations Implementation Plan

> **For agentic workers:** Use superpowers:executing-plans to implement this first increment task-by-task.

**Goal:** Adopt the user's v6 direction and implement its shared native presentation foundations.

**Architecture:** Extend the existing GPUI theme/widgets/shell. Keep appcore routes and domain/storage
behavior unchanged in this increment. Subsequent shell and viewer work has independent acceptance gates.

**Tech Stack:** Rust 2021, GPUI 0.2.2, existing embedded DM Sans and local assets.

**Spec:** `docs/implementation-ux-v6/README.md`; Figma Map `199:2197`, page `199:1316`.

## Global constraints

- Local-first native Windows GPUI; preserve scientific engines, identities, guards and persistence.
- Existing scientific base/feature colors retain meaning; readable UI text has separate roles.
- No Figma mutation, new runtime dependencies or fixture-derived production defaults.
- Preserve existing routes until V6-02 provides a tested replacement.

## Review focus

- Accent and status text on mint/tinted backgrounds as well as white.
- Selected navigation must not lose its persistent indicator on hover.
- 40px controls must remain scrollable instead of shrinking in narrow inspectors.
- Saving a source, saving an analysis and creating a product must remain distinct.
- A native UI refresh cannot imply implemented viewer controls or validated accessibility.

### Task 1: Adopt and protect the contrast roles

**Files:** `docs/ROADMAP.md`, `DESIGN.md`, `docs/implementation-ux-v6/*`, `apps/desktop_gpui/src/theme.rs`.

**Interfaces:** Existing Hsla theme constants remain compatible. Add named semantic status fills,
navigation text and selected stroke roles. No domain or public appcore interface changes.

- [x] Record v6 authority, preserved milestones and isolated baseline evidence.
- [x] Add contrast regression tests using actual GPUI Hsla→Rgba conversion and sRGB luminance.
- [x] Observe the existing accent fail 4.5:1 on canvas/mint before changing production tokens.
- [x] Implement the supplied v6 roles and rerun the focused tests.

### Task 2: Apply shared native geometry and hierarchy

**Files:** `apps/desktop_gpui/src/widgets.rs`, `apps/desktop_gpui/src/shell.rs`, `apps/desktop_gpui/src/theme.rs`.

**Interfaces:** Preserve widget constructors and route handlers. Extend existing styles, not parallel widgets.

- [x] Apply 40px/8px action geometry, 12px cards and subtle two-layer card elevation.
- [x] Apply 28px page headings, close title/description groups and 40px selected navigation styling.
- [x] Preserve current navigation targets and existing approved icon assets; exact v6 icon adoption is V6-02.
- [x] Verify native layouts at 1600 and 1280; record remaining v4 structure as deferred migration work.

### Task 3: Verify and checkpoint

- [x] Run format, full-workspace/all-target/all-feature Clippy, workspace tests and workspace build.
- [x] Self-review the entire diff, record limitations and update progress against actual evidence.
- [x] Commit this increment on `codex/ux-v6-foundations`. Do not merge main as part of this request.

## Pre-flight

Task 2 consumes Task 1's theme roles; retain existing constant names where possible.
Roadmap V6-02 owns new routes; no new View enum variants belong to this patch.
