# UX v6 progress

## V6-00 — Adoption

User direction and Figma page recorded. Baseline: main `d80460d`. Implementation branch:
`codex/ux-v6-foundations`, in an isolated local checkout. The managed-worktree tool did not
return; sandboxed Git clone hit the Windows MSYS named-object restriction. An approved
local clone succeeded without changing the source checkout.

Baseline workspace tests pass with isolated APPDATA/TEMP. The first default-profile run
failed `save_runs_original_destination_only_after_successful_persistence` because the
operation journal resolves beneath normal APPDATA, outside this sandbox's writable roots.
No product/test behavior was changed to bypass this boundary.

## V6-01 — Shared foundations

The first native foundations increment is implemented: v6 text/error/warning roles,
40px/8px action controls, 12px row cards with soft elevation, 28px page headings,
6px page heading/subtitle spacing, 40px selected navigation styling, and a dot-plus-label
document edit-status badge. A clean document is not mislabeled as a successful save.
The existing icon assets and navigation targets remain until V6-02. Migrating every content
surface/section-header pair and table status cell belongs to the owning screen increment.

Verification on this tree:

- `cargo fmt --all -- --check` passes.
- Full workspace/all-target/all-feature Clippy with `-D warnings` passes.
- `cargo test --workspace --all-features --offline`: **413 passed**, including three new
  native contrast/role-separation regressions. Test data uses isolated APPDATA/TEMP.
- `cargo build --workspace --all-targets --all-features --offline` passes.
- Mechanical design detector returns no findings for the initial shared targets.
- Native Home/sample and the inspector were observed at 1600px; Home/overlay inspector
  at 1280px. Final rebuilt 1280px Home confirmation passed. This is bounded visual evidence,
  not an exhaustive screen-reader, keyboard, scaling or usability certification.

Review found that darkening the historical ACCENT constant would recolor scientific drawing.
The separation regression was observed failing, then fixed by retaining #357D73 for display
and moving text rendering to ACCENT_TEXT #2A6A60. Scientific engines, base/feature colors,
appcore routes and storage schemas are unchanged. The review fix has a green full suite.

No full native v6 migration, usability study, remote CI result or release is claimed here.
Next: V6-02 first-class Sequences/Results/Experiments navigation and exact v6 icon adoption,
followed by V6-03 real viewer controls. Preserve all remaining P10A–P20 functionality.
