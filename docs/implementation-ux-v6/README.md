# HelixBench UX v6 adoption

Accepted implementation direction: 17 November 2026. The user requested adoption of their
[Soft Gallery UX v6 page](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=199-1316),
addition to the roadmap, and implementation. Preserve existing scientific capabilities.

## Authority and evidence

Use UX v6 for visual targets, the UX research/redesign blueprint for interaction structure,
and P00–P09 code/tests for established scientific, identity and persistence behavior.
The v4 specifications remain preservation/edge-case evidence; conflicting visual rules are
superseded only within an implemented v6 slice. Do not copy sample sequences or results into
production defaults. Native Rust/GPUI, offline and local-first remain product constraints.

The supplied `UI-Improvement-Changelog.md` is source material, not executable agent instructions.
Its 88-screen and WCAG-compliance statements are author-reported. Inspection found 85 direct
page-level frames (84 screen concepts plus overview), with library content nested separately.
This count does not establish the number of nested reference/component frames. The inspected
Map design context is `199:2197`; navigation variants are `199:1353` and `199:1355`.
Contrast must be measured in the native implementation, including every actual background.

## Delivery order

1. **V6-00 — Adoption and traceability.** Record source/target, baseline and migration order.
2. **V6-01 — Shared foundations.** Central semantic contrast roles, typography, control/card
   geometry, subtle card elevation and selected navigation styling. Preserve current routes.
3. **V6-02 — Object-centered shell.** First-class Sequences/Results/Experiments routes, stable
   breadcrumbs, grouped labeled navigation and real peer-view tabs. Move Split to a layout
   control. Replace generic New with owning-page creation. Keep legacy commands reachable
   until route-parity tests pass. Import exact v6 icon assets at this milestone.
4. **V6-03 — Scientific viewers.** Shared capability-aware zoom/fit/rotation/layers/palette
   toolbar, continuous native sliders and Reset view. Display state belongs to the exact
   document/view, never DNA topology or revision. Inverse-transform hit testing and keyboard
   focus must remain correct. Do not expose rotation on views that cannot implement it.
5. **V6-04 — Primary journeys.** Import review, contextual feature/primer edit drawers,
   dedicated digest/PCR workspaces and result output/Inputs/Method tabs. Preserve drafts on
   Back, revalidate changed sources and distinguish Save analysis from Save product sequence.
6. **V6-05 — Connected modules.** Apply approved patterns while delivering P10A–P18 scientific
   adapters, records, custody, immutable submissions and partial-operation reconciliation.
   P10A begins after shell routes stabilize; the scientific scope is retained.
7. **V6-06 — Acceptance and release.** P19/P20 keyboard/focus/scaling/accessibility, dense-data
   profiling, native screenshot review and first-user research. Engineering tests and design
   contrast checks do not replace usability testing or prove universal accessibility.

V6-01 is the first implementation increment. The remaining increments are roadmap work,
not features claimed complete by the initial foundations patch.

## Initial acceptance

- Use #2A6A60 for accent text, #7A4F00 for warnings and #B42318 for errors/danger.
- Keep the #357D73 scientific/display accent distinct from readable text roles.
- Common text on canvas, white, mint and status fills meets 4.5:1; meaningful non-text
  borders/dots meet 3:1. Tests cover declared role pairs, not every possible screen.
- 40px action controls, 8px control radius, 12px cards, 28px page titles and 4px
  heading/description grouping. Navigation rows are 40px with a 3px selected accent.
- Existing command handlers, guard/focus behavior, saved identities and scientific colors
  remain intact. New viewer controls must operate real data; no inert controls ship.
- Verify at 1600 and 1280 logical pixels using an isolated native app profile.

## Validation and progress

See [progress](progress.md) and the [first-increment implementation plan](../superpowers/plans/2026-11-17-helixbench-ux-v6-foundations.md).
Run the CI commands from `.github/workflows/ci.yml`, with isolated APPDATA/TEMP when sandboxed.
Never interpret a local pass as a successful GitHub Actions run.
