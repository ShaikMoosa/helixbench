# HelixBench · implementation specification

**Target:** Soft Gallery UX v4, preserving existing scientific capabilities. **Code baseline:** 0474cb94e456094dc4e501a9351ca7587a149361. **Snapshot:** 15 November 2026.

This is a code-ready specification and implementation order, not a statement that the target features are implemented. Begin with the shared contracts and foundation tasks; preserve the existing algorithms while replacing ambiguous context/state handling.

Current implementation evidence and format compatibility are tracked in [implementation progress](progress.md). The chapters below retain the original specification baseline.

## Open the document

- [Illustrated, searchable specification](index.html)
- [Step-by-step implementation plan](10-implementation-plan.md)
- [Feature traceability](feature-traceability.csv)
- [Acceptance and edge-case register](acceptance-and-edge-cases.csv)
- [Machine-readable specification](specification.json)
- [Figma flow-completion review](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=128-8269)

## Coverage

- 19 primary module screen blueprints with current images and code ownership.
- 212 preserved feature entries plus 25 explicitly identified target UX additions.
- 686 captured Figma frames classified as primary, linked state, responsive/context variant or reference.
- Field contracts from the current Rust draft types; current data schemas and source/API/test indexes.
- Connected journeys, functional state machines, migration/persistence rules and synthetic fixtures.
- 26 reviewable implementation tasks, each with dependencies, files, proposed interfaces, tests and completion gates.

## Read in order

- [Product scope and implementation authority](01-product-and-authority.md)
- [Shared interaction contracts](02-shared-interaction-contracts.md)
- [Data, state and persistence contracts](03-data-state-and-persistence.md)
- [Verified code-to-target gaps](04-code-to-target-gaps.md)
- [Functional state machines and transitions](05-functional-state-machines.md)
- [Scientific and input-validation contracts](06-scientific-and-validation-contracts.md)
- [Acceptance, edge cases and verification strategy](07-qa-and-edge-case-strategy.md)
- [Implementation risks, resolved policies and remaining validation](08-implementation-risks-and-resolution.md)
- [Journeys and state machines](09-journeys-and-state-machines.md)
- [Implementation plan](10-implementation-plan.md)
- [Form field contracts](11-form-field-contracts.md)
- [Complete screen/state catalog](12-screen-state-catalog.md)
- [Shared state visual references](13-shared-state-visuals.md)
- [Soft Gallery visual foundations](14-visual-foundations.md)
- [Current-code data models](reference/current-data-types.md)
- [Test fixtures](fixtures/README.md)

## Module blueprints

- [SCR-01 · Home](screens/home.md) · 14 preserved feature entries · P06.
- [SCR-02 · Map](screens/map.md) · 20 preserved feature entries · P08A.
- [SCR-03 · Sequence](screens/sequence.md) · 16 preserved feature entries · P08A.
- [SCR-04 · Features](screens/features.md) · 14 preserved feature entries · P08B.
- [SCR-05 · Primers](screens/primers.md) · 13 preserved feature entries · P08C.
- [SCR-06 · Enzymes](screens/enzymes.md) · 13 preserved feature entries · P08D.
- [SCR-07 · History](screens/history.md) · 7 preserved feature entries · P08D.
- [SCR-08 · Notebook](screens/notebook.md) · 12 preserved feature entries · P11.
- [SCR-09 · Registry](screens/registry.md) · 12 preserved feature entries · P12.
- [SCR-10 · Inventory](screens/inventory.md) · 11 preserved feature entries · P13.
- [SCR-11 · Workflows](screens/workflows.md) · 11 preserved feature entries · P14.
- [SCR-12 · Insights](screens/insights.md) · 6 preserved feature entries · P15.
- [SCR-13 · Automation](screens/automation.md) · 10 preserved feature entries · P16.
- [SCR-14 · Split](screens/split.md) · 6 preserved feature entries · P10A.
- [SCR-15 · Biologics](screens/biologics.md) · 7 preserved feature entries · P17.
- [SCR-16 · Chemistry](screens/chemistry.md) · 10 preserved feature entries · P17.
- [SCR-17 · RNA / Oligos](screens/rna-oligos.md) · 8 preserved feature entries · P17.
- [SCR-18 · In Vivo](screens/in-vivo.md) · 10 preserved feature entries · P18.
- [SCR-19 · Bioprocess](screens/bioprocess.md) · 12 preserved feature entries · P18.

## Execution discipline

- Every task begins with its requirements and current source references. New public types/APIs in the plan are labeled proposed.
- Entry click counts are target budgets from the owning page, not measured runtime results. Preparing data and necessary safety review are not equivalent to discovering a tool.
- Do not hardcode a Figma example ID, quantity, sequence or computed result in production.
- Do not interpret every Figma frame as a route or every success-content example as a modal.
- Current-code observations, target requirements and unverified runtime behavior remain distinct.
- Update traceability with actual commit/test evidence; checkboxes alone are not proof.
- No source application code was changed or scientific tests run to create this specification.
