# UX v6 Scientific Viewers Implementation Plan

> Use superpowers:executing-plans inline, TDD, and one final whole-branch review.

**Goal:** Match the approved viewer layout with working zoom, fit, rotation, layers and palette controls.
**Architecture:** Appcore owns display settings keyed by exact DocumentId and viewer surface. GPUI canvases consume those settings; the native toolbar and continuous sliders change display state only. Circular paint and hit testing share the same transformation. Sequence zoom uses the existing bounded genomic window.
**Tech stack:** Rust, GPUI 0.2.2, embedded Figma SVGs; no new dependencies.
**Spec:** docs/implementation-ux-v6/README.md, Figma Map199:2197 and toolbar201:5143.

## Constraints and decisions

- Preserve DNA/topology, revisions, dirty status, undo, captured results and biological coordinates.
- Preserve the legacy map/sequence preview as an explicit View options choice; Map defaults to the larger canvas.
- Circular map zoom50â€“250%, linear map100â€“1000%, sequence100% through a minimum20-base window (or the whole sequence if shorter). Rotation only on circular maps, wrapped0â€“360 degrees; buttons step15 degrees.
- Layers are local to a document/view; unconfigured viewers inherit existing display preferences. Color choices affect display only. Reset restores view transforms and defaults, not sequence edits or selection.
- Toolbar32px targets, white surface,10px radius, native text/hover/focus; palettes/layers open a small contextual panel rather than a text modal.
- +/- and0 act only with viewer focus. R rotates only the Map; typing R in Sequence remains valid DNA editing.
- Keep existing preview/metadata, minimap, range selection and scientific commands reachable. No invented values/default results.
- Branch stacked on reviewed V6-02; no merge or main updates.

## Review focus

- A rotated/zoomed feature must hit its original ID and exact genomic coordinates.
- Two same-name documents and switching circular/linear views cannot share settings.
- Empty/short sequences, NaN/infinite values and zoom endpoints must remain finite and safe.
- Slider drag leaving the track, switching context or releasing outside must stop safely.
- Keyboard shortcuts cannot steal bases from sequence editing or text forms.

### Task1 â€” Display state and transformation contract

- [x] Add failing tests for scope, finite/clamped settings, resets without mutations, and inverse geometry.
- [x] Implement viewer settings/capabilities and pure transforms in appcore; add AppState field.
- [x] Run focused tests; record RED/GREEN evidence.

### Task2 â€” Native canvases and controls

- [x] Import exact seven toolbar SVGs with provenance/root sizes/checksums.
- [x] Use common display settings for Map paint/hit testing and Sequence rendering.
- [x] Add floating toolbar, native continuous zoom/rotation sliders, labeled layers/palette panel and Reset view.
- [x] Default Map to the Figma canvas; retain explicit legacy preview and metadata.
- [x] Add focus-safe keyboard controls and transformed hit/selection regressions.

### Task3 â€” Verify and publish

- [x] Format, Clippy, full tests/build; final whole-branch review.
- [ ] Native1600/1280 inspection and asset slot/geometry verification; record any runtime limitations precisely.
- [ ] Update design contract, roadmap/progress and publish a reviewable stacked PR with CI evidence.

## Ledger

Base08841095e2812817a18d5fa8dcbd8bfa312c0378. Pre-flight: native controls consume Task1 settings and transform values; linear and circular paint/hit functions must use the same surface. Sequence keeps existing zoom_start/zoom_width for search/selection compatibility.

Task1: complete. Five core tests passed after missing viewer module RED; native rotated-feature hit test was observed failing the exact-ID assertion, then passed after paint/hit rotation was connected. Logs: work/v603-red.log, v603-hit-red.log, v603-hit-green.log (outside repository).

Ruling: Map zoom limits and 15-degree rotation step are implementation defaults, not user-supplied measurements. Sequence retains the existing 20-base minimum window. A wrong map bound limits display reach but cannot change scientific data.

Task2: implemented. Ten core and four new native regressions pass. Review fixes cover rotated enzyme hits, layer capability gating, tab stops and one native keyboard activation, context/release-gated drag, metadata gating, occluding floating controls, per-property defaults and legacy-menu parity, positive circular tracks, linear window clamping and one-base keyboard zoom increments. Reset affects only its own surface.

Native1280 Home/Map, single Enter toggle and Tab focus observed. Short-canvas clipping was corrected with a bounded scroll body, persistent header/footer and explicit full-width slider parent; final rebuilt confirmation is pending.

Final source verification: 441 tests, format, all-target/all-feature Clippy and build passed. Final1280x720 anchored panel, continuous150% zoom/88-degree rotation, checkable layers and independent Sequence controls were observed. Initial1600 check predates final fixes; final wide-screen acceptance remains pending because the larger window extended off the available desktop. See progress.md; keep PR draft.
