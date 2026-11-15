# UX v4 implementation progress

Branch: `codex/ux-v4-implementation`.
Baseline: `0474cb94e456094dc4e501a9351ca7587a149361`.
Authority: UX v4 interface; preserve existing scientific capabilities and local data.

## P00 — complete

15 November 2026, Windows, Rust/Cargo 1.100.0.

- `cargo test -p domain -p storage -p appcore --locked`: **181 passed, 0 failed** (domain 145; storage 30; appcore 6). Doc tests: 0 cases.
- `cargo check -p desktop_gpui --locked`: **passed**. Upstream `proc-macro-error2 2.0.1` emits a future compatibility notice; no baseline compiler error.
- Scientific fixtures retained in the existing domain tests: translation/ORFs, primer/PCR, digest, alignment, assembly, CRISPR, codon, protein, RNA and chemistry.
- Native-file baseline: `.hbx` v1, atomic round-trip and rejection of unsupported newer formats. Legacy fixture is in `fixtures/legacy-v1.hbx`.
- Specification, screen images, field/state/feature catalogs and the ordered plan are committed alongside this progress record.

P00 changes no scientific behavior. The repeated generic UI/test checklist in the plan does not apply to this baseline-only task.

## Next

P01: stable document/revision identities, immutable input snapshots, typed cross-store references and native-file v2 migration.

## Implementation rulings

- Migration tests belong to storage integration tests; domain tests must not depend on storage.
- Checkpoint commits are pushed to this branch; `main` is preserved until integration is requested.
- Prototype validations in the specification describe design wiring, not application test results. The source manifest describes the original baseline and will differ after implementation.
