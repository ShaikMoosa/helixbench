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

## V6-02 — Object-centered navigation (20 November 2026)

Implemented on `codex/ux-v6-navigation`, based on main `c2006eb`. The shell now uses
224px grouped navigation, persistent workspace breadcrumbs/search, exact local v6 SVGs,
and first-class Sequences, Results, Experiments and Activity destinations. Scientific views
use underline tabs, including while Split layout is active. Header actions belong to the
current object: sequence import/create/save/analysis and experiment entry creation.

Sequences lists **open documents**, not a recursively scanned file repository. Rows open by
DocumentId, preserving edits, selection and undo state; pathless documents cannot claim a
successful local save. Search matches name/path, with clear/reset and empty states. Import
continues through the existing native file picker and input review.

Results is a dedicated route rather than an alternate Home renderer. Saved rows expose
actual method, captured source revision, short run ID, UTC date/time and source availability.
Opening sources and Back retain exact ResultId and captured inputs. Completed unsaved runs
remain reachable through **Unsaved in this session → Resume output**, even after visiting
Home and the catalog. The section explicitly states that session outputs do not survive an
app restart. Saved runs are excluded from that section. No new persistence schema is added.

Catalogs hide the unrelated object inspector and its Page tools trigger. Activity reports
actual local recovery items/errors; it does not imply cloud jobs or background scheduling.
Settings continues to open the existing options form. Advanced viewer controls, complete
analysis workspaces, module forms and all P10A–P20 capabilities retain their owning milestones.

Verification:

- Format, all-target/all-feature Clippy with `-D warnings`, full workspace/all-feature tests
  and all-target/all-feature build pass offline with isolated test APPDATA/TEMP.
- **427 tests passed**, including 14 navigation regressions. Coverage includes real digest
  outputs, source return, duplicate names, stale IDs, search, unfinished forms, scoped entry
  creation, repeated navigation and resuming/saving a previously hidden completed run.
- Independent whole-branch review and fix review report no remaining actionable findings.
  Review identified the inactive catalog Page tools control and stranded session output;
  both are fixed, with a failing-then-passing regression for session resume.
- All 16 imported SVG checksums match their manifest; source integrity and diff checks pass.
- Native 1600px observation covered Home, sample Map, sequence catalog, Results empty state,
  and Experiments creation before the final review fixes. The rebuilt 1280px check and native
  Resume output confirmation remain pending: Windows window discovery timed out twice and
  again after its runtime reset. Automated width/layout tests pass; these do not replace
  visual inspection. Keep the PR draft until those two checks are completed.

Environment recovery: two uncommitted result source files resumed as zero-filled files.
They were reconstructed from the committed baseline and saved implementation scripts;
all other edits were preserved. The final tree passed the checks above with incremental
compilation disabled after Rust reported damaged incremental cache metadata.

Next implementation increment: V6-03 real scientific viewer controls; P10A–P20 remain in scope.
