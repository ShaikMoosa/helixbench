# Acceptance, edge cases and verification strategy

## Baseline first

Run and record the existing domain/storage/appcore suites before changing behavior. The source index lists discovered tests but documentation generation does not establish that they pass. If the baseline fails, preserve evidence and resolve/triage it before attributing a failure to the UI migration.

## Required commands

From repository root use cargo test -p domain; cargo test -p storage; cargo test -p appcore; cargo check -p desktop_gpui; cargo fmt --all -- --check. Run workspace-wide tests/build checks after the relevant slices pass and when the Windows/GPUI environment supports them. Record exact command, toolchain, result and revision; do not fabricate expected timings.

## Identity tests

Use two records with the same display name, two store kinds with the same u64 ID, two documents with different revisions and two plates with A1. Sort/filter/reopen/delete while actions are pending. Assert the exact typed identity in every write, result and return route.

## Persistence fault tests

Inject failure at temp creation, write, flush, rename, journal stage and each store commit. The last good file remains readable; false success is impossible; retry/restart cannot create duplicate outputs. Use temporary test directories and never real user config files.

## Async tests

Change selection/document/revision while work runs; issue repeated Run/Cancel; close the view/app at a supported boundary; deliver an obsolete completion after a newer request. Assert captured input provenance, stage-aware cancellation and safe recovery.

## Scientific fixtures

Pin domain-engine regression fixtures by molecule type, topology, coordinate convention, method and expected outputs/tolerances. Use circular wrap, no matches, ambiguous bases, primer overhangs, feature transformations, MSA reindex, assembly overhang errors, invalid SMILES/HELM and unit boundaries.

## Native UI tests

Exercise text-input shortcut precedence, all clickable targets, tab order, visible focus, Escape and focus return. Inspect Windows scaling/text enlargement, high-density map labels, readable alternatives, 1280 layout and right-panel closing. Native assistive-technology exposure needs runtime inspection; a Figma frame cannot establish it.

## First-user study

Observe 6–8 molecular biologists new to HelixBench performing import → interpretation → analysis → save → refind without navigation hints. Record unassisted completion, first correct control, wrong turns, identity/scope mistakes, help requests and recovery. Use the measured baseline to agree quantitative targets; a two-click count alone is not usability evidence.

## Release gate

All mandatory acceptance IDs and selected edge cases pass, legacy data round trips without silent loss, no critical identity/lost-work defects remain, and all retained scientific capabilities are reachable. Unimplemented design-only features are disabled or explicitly explained rather than represented by false-success buttons.

