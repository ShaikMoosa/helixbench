# HelixBench — Detailed Product Roadmap

**Grounded in:** `reference-features-ui-research.docx` + `reference-research/screens/`
(15 capability areas, 82 screen groups, 178 public UI assets, 84 sources; research date 14 Nov 2026).
**Baseline:** HelixBench v0.6.0 (local-first, pure-Rust, offline, single-user Windows DNA workbench).

This roadmap translates the reference platform research into a phased plan for HelixBench. It keeps
HelixBench's identity — **local-first, offline, single-user, pure Rust** — and adapts the reference platform's
*connected-record* ideas to local equivalents rather than copying its multi-tenant cloud model.

---

## 1. What the research says (key UX findings)

1. **Shared scientific identity.** The same entity (sequence / protein / molecule) is reused across
   Registry → Inventory → Notebook → Results → Analysis. Traceability and controlled handoffs are the
   core experience. *Implication: HelixBench needs a local entity/lineage graph, not isolated files.*
2. **Registry identity ≠ Inventory custody.** Registry describes the scientific entity; Inventory
   tracks physical containers, quantities, location. *Implication: two linked but distinct local stores.*
3. **Notebook is an operational workspace.** Structured tables are interfaces into entities/results/
   inventory; submission is a persistence boundary; templates + staged review matter.
4. **Progressive configuration.** Admin schemas/templates generate the forms scientists execute.
   *Implication: schema-driven UI, separate "configure" vs "execute" modes.*
5. **Bulk work & recovery.** Mapping errors, duplicate identity, validation, partial selection,
   async creation, corrected re-submission are first-class states.
6. **Lifecycle & permission states.** draft / published / locked / archived / view-only / no-access
   must be visually distinct.
7. **Workflows vs Automation are different metaphors.** Workflows = assigned human tasks/requests;
   Automation = instrument/data transformation pipelines. Don't conflate them.
8. **AI is governed.** Unified Chat + reusable Agents + human approval of edits; Model Hub for tracked
   scientific-model execution. *Implication: any AI/automation must have explicit approval + audit.*

---

## 2. Current coverage (v0.6.0) vs the 15 research areas

| # | Reference area | HelixBench v0.6.0 | Status |
|---|---|---|---|
| 1 | Notebook | — (no entries/templates/review) | **Gap → Phase 1** |
| 2 | Registry | local catalog (plasmid/strain/primer) + link-to-doc | Partial → Phase 1 (schemas, lineage, import) |
| 3 | Inventory | — | **Gap → Phase 2** |
| 4 | Molecular Biology | map/sequence/primers/digest/alignment/assembly/CRISPR/PCR/codon/BLAST/protein | **Shipped** (deepen: split workspace, saved alignments, secondary structure) |
| 5 | Workflows | — | **Gap → Phase 3** |
| 6 | Development/Bioprocess | — | Optional module (Phase 5) |
| 7 | In Vivo | — | Optional module (Phase 5) |
| 8 | Insights | — | **Gap → Phase 4** (local dashboards) |
| 9 | Automation/Connect | — | **Gap → Phase 4** (local pipelines) |
| 10 | AI/Agents/Model Hub | — | Optional (Phase 6, governed) |
| 11 | Biologics/PipeBio | protein analysis only | Optional module (Phase 5) |
| 12 | Chemistry | — | Optional module (Phase 5) |
| 13 | RNA/modified oligos | RNA translation only | Optional (Phase 5) |
| 14 | Developer Platform | — (no API/apps) | Optional (Phase 6, local API/CLI) |
| 15 | Validated Cloud/admin | local config + audit-lite | Partial → audit in Phase 1; GxP/multi-user out-of-core |

---

## 3. Cross-cutting UX pillars (apply to every phase)

- **P1 Lineage everywhere:** every object shows "derived from / used in" links; preserve IDs across
  PCR → assembly → registry → inventory → notebook → result.
- **P2 High-density workspaces:** resizable split views, column config, saved views, realistic data
  volumes, virtualized tables.
- **P3 Schema-driven forms:** a local schema editor generates entry/registry/inventory forms;
  separate Configure vs Execute modes.
- **P4 Bulk & recovery:** multi-select, bulk edit/export/tag, import mapping + validation errors,
  undo/redo, corrected re-submission.
- **P5 Lifecycle states:** draft/published/locked/archived/view-only badges; locked records immutable.
- **P6 Audit & traceability:** local append-only audit log per object + global, exportable CSV.
- **P7 Governed automation:** any automated/AI action requires explicit approval + is audited.

---

## 4. Phased roadmap

### Phase 1 — v0.7 "Connected local records" (Notebook-lite + Registry depth + projects)
Goal: turn isolated documents into a connected local knowledge base.
- **Projects/folders:** local tree organizing documents, entries, registry items; drag/move; star.
- **Registry depth:** typed schemas (field types, dropdowns, uniqueness, name templates), spreadsheet
  import with mapping+validation, Entity Map (local relationship graph), archive states.
- **Notebook-lite:** entries (prose + sections + attachments), structured result/registry/inventory
  tables that write back to entities, templates + template collections, staged review (submit →
  reviewer → approve) with lock-on-submit, review history.
- **Home/My Work:** recent items, my entries/tasks, status filters, calendar-lite.
- **Audit log (P6):** per-object + global, CSV export.
- Acceptance: one entity traceable sequence→registry→entry→result; import validates; review locks.

### Phase 2 — v0.8 "Inventory & sample custody"
Goal: physical storage tracking linked to registry identity.
- Local storage hierarchy: locations → boxes/plates → containers; schema-configurable nesting/units.
- Containers: entity link, quantity, concentration, location; plate grid (A1…) with roles
  (Sample/Control/Standard/Blank), fill-wells wizard, review step.
- Notebook Inventory tables for transfers/attribute updates (bulk).
- Entity→container lookup ("where is this plasmid?") grouped-by-contents search.
- Immutable plate snapshots in entries vs live inventory (research finding).
- Acceptance: place/take samples, find all containers of an entity, bulk transfer with audit.

### Phase 3 — v0.9 "Workflows & requests (local)"
Goal: assigned, trackable lab work without a server.
- Task schemas (inputs/outputs/status lifecycle/owner), task dashboard + My Work assignments.
- Execution modes: direct task, entry-based (template), and a simple local flowchart with data passing.
- Requests/Forms intake (local), draft→sent (read-only) states, watchers.
- Failed-mapping recovery modal + bulk fix (research 05 06).
- Acceptance: request → task → execute → output linked; failed mapping recoverable.

### Phase 4 — v1.0 "Insights + Automation (local)" — SHIPPED (v1.0.0)
Goal: analyze and transform local data; reusable pipelines.
- **Insights-lite:** saved queries over local store (SQL-like or query builder), dashboard blocks
  (table/bar/line/heatmap), parameters with null handling, CSV export, permission-aware (local roles).
- **Automation-lite:** local file ingestion (instrument CSV/FASTA) → transform steps (built-in +
  optional embedded Python) → write back to registry/inventory/results; reusable templates with
  draft→published lifecycle; run history + error states.
- Acceptance: ingest → transform → register/update with audit; dashboard reflects local data.
- Shipped scope: local datasets (registry/inventory/tasks/entries) + group-by dashboard blocks with
  bar charts and CSV export; CSV file-ingestion pipelines (ingest → filter → count → writeback) with
  draft→published lifecycle, run history + error states, registry writeback + audit trail.
  UI revamp: labeled workspace navigation sidebar + full-page Home/Notebook/Registry/Inventory/
  Workflows/Insights/Automation views.

### Phase 5 — v1.x specialist modules (optional, pick by user focus)
- **Biologics:** format-aware protein entities (IgG/bispecific/nanobody/TCR), domains/chains/variable
  pairs, 2D glyphs, registration wizard, numbering schemes.
- **Chemistry:** molecule sketcher/import (SMILES), compound/form/lot, reaction tables w/ stoichiometry,
  structure/substructure search, Salt & Solvate library.
- **RNA/oligos:** RNA sequences, modified nucleotides (sugar/base/phosphate), HELM notation, monomer
  library, bulk modification patterns.
- **In Vivo:** studies, cohorts/cages, randomization, scheduling, dosing (weight-based), timeseries
  tables, study graphs, welfare alerts, audit.
- **Bioprocess:** ISA-88 recipes, unit operations, conditions/replicates, operator worksheets,
  run planning (draft→exec), cross-run visualization.
- Each is a self-contained module behind a feature flag; share the Phase 1–4 record/audit foundation.

### Phase 6 — optional, governed intelligence & integration (scope decision)
- **AI assist:** optional local or user-configured LLM for design suggestions / SQL writer / summarization,
  always with human approval + citation + audit (P7). No silent edits.
- **Agents-lite:** local scheduled/triggered automations with explicit permission scope + approval.
- **Model Hub-lite:** optional local structure prediction (e.g. bundled/on-demand) with tracked jobs.
- **Developer/CLI:** local REST/CLI + plugin API for integration (single-user), read-only export warehouse.
- **Explicitly out-of-core unless direction changes:** multi-tenant cloud, real-time collaboration,
  SSO/MFA/tenant admin, GxP/Validated Cloud, electronic signatures. These conflict with local-first.

---

## 5. UI/UX roadmap (grounded in the screens)

- **Molecular Biology (mol-bio screens):** keep light plasmid map w/ concentric pastel rings + rotated
  labels + ruler; add **LINEAR MAP / PLASMID / METADATA** tabs and **Split Workspace** (compare two
  sequences side-by-side); status bar tied to selection (already present). Add saved-alignments list,
  primer secondary-structure preview, enzyme "find by cuts/region".
- **Notebook/Registry/Inventory screens:** adopt schema-driven forms, structured tables with a clear
  **Submit** persistence boundary, template editor with locked columns, review badges, Entity Map graph,
  plate grid + fill-wells, storage tree.
- **Home/AI screens:** home = resume-work hub (My Work/recent/worklists/calendar); AI/Agents uses a
  3-step governed wizard (setup → data access → collaborators) with "uses mine / has its own" permissions.
- **States to design for every module:** empty, loading, long-running, validation-error, partial-select,
  async-creating, locked, archived, view-only, no-access, review-pending, failed-mapping.
- **Density & recovery:** virtualized tables, column config, saved views, bulk bars, import mapping with
  per-row errors, undo/redo, corrected re-submission.

---

## 6. Release plan & success criteria

| Release | Theme | Exit criteria |
|---|---|---|
| v0.7 | Connected records | lineage traceable; registry import validates; review locks; audit export |
| v0.8 | Inventory | place/find/transfer samples; plate snapshots immutable |
| v0.9 | Workflows | request→task→execute→output; failed-mapping recovery |
| v1.0 (shipped) | Insights+Automation | ingest→transform→writeback; dashboards; template lifecycle |
| v1.x | Specialist modules | per-module acceptance (above) |
| v2.0 (optional) | Governed AI/integration | approval+audit on all automated actions |

Every release: `fmt` / `clippy -D warnings` / workspace tests / build green; runtime-verified UI;
installer + portable published; README + Figma design system updated.

## 7. Risks & mitigations
- **Scope creep toward cloud:** keep multi-user/sync/GxP out-of-core; revisit only as a product decision.
- **Schema complexity:** start with a small typed-field set; iterate.
- **Performance (MSA/BLAST/tables):** virtualize + cap + background jobs; benchmark large plasmids.
- **AI safety:** approval + audit + citations mandatory; no autonomous writes.

## 8. Explicitly out-of-core (current direction)
Multi-tenant cloud, real-time collaboration, SSO/MFA/tenant admin, GxP/Validated Cloud, e-signatures,
and hosted instrument gateways. Local equivalents (audit, roles-lite, local pipelines) are in-scope.