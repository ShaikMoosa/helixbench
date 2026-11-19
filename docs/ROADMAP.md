# HelixBench — Detailed Product Roadmap

## Active delivery priority — UX v6 (17 November 2026)

The user adopted [Soft Gallery UX v6](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=199-1316)
and requested implementation. The historical release coverage below does not imply the new UX
is implemented. P00–P09 preservation work is on main `d80460d`; P10A–P20 remain in scope.

V6-01 shared foundations is merged. V6-02 object-centered routes/navigation is implemented
on `codex/ux-v6-navigation`; see live progress for verification and remaining visual checks.
V6-03 scientific viewer controls is implemented on `codex/ux-v6-viewers`, stacked on
V6-02. Final wide-screen visual acceptance remains in its draft review scope.
Next implementation: V6-04 import/digest/PCR workspaces and contextual edit drawers.
V6-05 carries the remaining P10A–P18 functionality through the new patterns; V6-06 combines
P19/P20 acceptance, researcher validation and release evidence. P10A resumes after V6-02.

See [the v6 adoption specification](implementation-ux-v6/README.md),
[first implementation plan](superpowers/plans/2026-11-17-helixbench-ux-v6-foundations.md)
and [live progress](implementation-ux-v6/progress.md). Preserve existing scientific capabilities;
do not replace their engines with Figma sample values or infer cloud capabilities from the design.

---

**Grounded in:** `reference-features-ui-research.docx` + `reference-research/screens/`
(15 capability areas, 82 screen groups, 178 public UI assets, 84 sources; research date 14 Nov 2026).
**Baseline:** HelixBench v0.6.0 (local-first, pure-Rust, offline, single-user Windows DNA workbench).
**Current status:** v1.14.0 shipped. Phases 1–5 complete (all five specialist modules delivered).
Remaining: the governed AI/Agents wizard + Phase 6 optional intelligence/integration, and
housekeeping (Figma sync, deprecated checkout action).

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

## 2. Current coverage (v1.8.0) vs the 15 research areas

| # | Reference area | HelixBench v1.8.0 | Status |
|---|---|---|---|
| 1 | Notebook | entries + sections + result tables, templates, submit→approve review, audit | **Shipped** (v0.7) |
| 2 | Registry | typed catalog (plasmid/strain/primer), link-to-doc, Entity Map, archive, CSV import w/ mapping + per-row errors | **Shipped** (v0.5/v0.7/v1.7) |
| 3 | Inventory | storage tree, plates/wells, fill-wells, immutable snapshots, entity lookup | **Shipped** (v0.8) |
| 4 | Molecular Biology | map/sequence/primers/digest/alignment/assembly/CRISPR/PCR/codon/BLAST/protein + view tabs, split workspace, saved alignments, secondary structure, find-by-cuts/region | **Shipped + deepened** (v0.2–v1.5) |
| 5 | Workflows | tasks, requests (draft→sent), local flowchart executor | **Shipped** (v0.9) |
| 6 | Development/Bioprocess | — | Optional module (Phase 5) |
| 7 | In Vivo | — | Optional module (Phase 5) |
| 8 | Insights | local datasets + group-by dashboards + CSV export | **Shipped** (v1.0) |
| 9 | Automation/Connect | CSV ingest→transform→writeback pipelines, draft→published, run history | **Shipped** (v1.0) |
| 10 | AI/Agents/Model Hub | — | Optional (Phase 6, governed) |
| 11 | Biologics/PipeBio | protein analysis only | Optional module (Phase 5) |
| 12 | Chemistry | — | Optional module (Phase 5) |
| 13 | RNA/modified oligos | RNA translation only | Optional (Phase 5) |
| 14 | Developer Platform | — (no API/apps) | Optional (Phase 6, local API/CLI) |
| 15 | Validated Cloud/admin | local config + audit log + CSV export | **Shipped (audit)**; GxP/multi-user out-of-core |

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

### Phase 1 — v0.7 "Connected local records" (Notebook-lite + Registry depth + projects) — SHIPPED (v0.7.0)
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
- Shipped scope: projects/tags, notebook entries + result tables + templates + submit/approve review,
  registry schemas + Entity Map + archive, per-object/global audit with CSV export, Home hub.

### Phase 2 — v0.8 "Inventory & sample custody" — SHIPPED (v0.8.0)
Goal: physical storage tracking linked to registry identity.
- Local storage hierarchy: locations → boxes/plates → containers; schema-configurable nesting/units.
- Containers: entity link, quantity, concentration, location; plate grid (A1…) with roles
  (Sample/Control/Standard/Blank), fill-wells wizard, review step.
- Notebook Inventory tables for transfers/attribute updates (bulk).
- Entity→container lookup ("where is this plasmid?") grouped-by-contents search.
- Immutable plate snapshots in entries vs live inventory (research finding).
- Acceptance: place/take samples, find all containers of an entity, bulk transfer with audit.
- Shipped scope: storage tree (locations/boxes/plates/containers), plate well grid + fill-wells,
  immutable plate snapshots into entries, entity→container lookup.

### Phase 3 — v0.9 "Workflows & requests (local)" — SHIPPED (v0.9.0)
Goal: assigned, trackable lab work without a server.
- Task schemas (inputs/outputs/status lifecycle/owner), task dashboard + My Work assignments.
- Execution modes: direct task, entry-based (template), and a simple local flowchart with data passing.
- Requests/Forms intake (local), draft→sent (read-only) states, watchers.
- Failed-mapping recovery modal + bulk fix (research 05 06).
- Acceptance: request → task → execute → output linked; failed mapping recoverable.
- Shipped scope: task dashboard + lifecycle, request intake (draft→sent), local flowchart executor
  with data passing; CI (GitHub Actions) added and kept green.

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
- **Biologics — SHIPPED (v1.10.0):** format-aware protein entities (IgG/bispecific/nanobody/TCR/Fab),
  chains + auto-derived domains, Sequential/Kabat/IMGT numbering (simplified display schemes),
  2D glyph, registration wizard, local persistence + Biologics view.
- **Chemistry — SHIPPED (v1.11.0):** SMILES-subset parser (organic subset, branches, rings,
  aromatic, bracket atoms), formula/MW/H-donor/acceptor, substructure search, compound/reaction/lot
  stores, reaction stoichiometry (equivalents), built-in salt & solvate library, Chemistry view.
- **RNA/oligos — SHIPPED (v1.12.0):** RNA sequences, modified-monomer library (2'-OMe / 2'-F / LNA /
  PS / 5mC / pseudouridine), bulk modification patterns, linear HELM subset (emit + parse),
  oligo MW/GC, RNA view with add-oligo dialog + pattern application.
- **In Vivo — SHIPPED (v1.13.0):** studies/cohorts/cages/animals, seeded balanced randomization,
  weight-based dosing (mg/kg × kg / mg/mL), timeseries measurements, mean-weight-by-day graph,
  welfare alerts (≥20% weight loss), TGI helper. Local persistence + In Vivo view.
- **Bioprocess — SHIPPED (v1.14.0):** ISA-88-lite recipes (unit-operation templates), run planning
  (draft → executing → completed/aborted), operator worksheets with per-step done flags + progress,
  replicates with conditions/titer, mean-titer + cross-run comparison chart. Local persistence + view.
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
  labels + ruler; **PLASMID / LINEAR / METADATA** view tabs shipped (v1.1.0); **Split Workspace**
  (compare two sequences side-by-side) shipped (v1.2.0); **saved-alignments list** (save / load /
  delete named alignment snapshots) shipped (v1.3.0); **primer secondary-structure preview**
  (hairpin stem/loop + self-dimer, live in primer editor and as design-candidate badges) shipped
  (v1.4.0); **enzyme find by cuts/region** (cut-count filter + selection / custom-range region
  filter, combinable) shipped (v1.5.0); status bar tied to selection (already present).
- **Notebook/Registry/Inventory screens:** adopt schema-driven forms, structured tables with a clear
  **Submit** persistence boundary, template editor with locked columns, review badges, Entity Map graph,
  plate grid + fill-wells, storage tree.
- **Home/AI screens:** home = resume-work hub (My Work / recent files / worklist / recent entries)
  shipped (v1.6.0); **Home month calendar** (entries/tasks per day, month nav, day drill-down)
  shipped (v1.9.0); the AI/Agents 3-step governed wizard (setup → data access → collaborators)
  with "uses mine / has its own" permissions remains.
- **States to design for every module:** empty, loading, long-running, validation-error, partial-select,
  async-creating, locked, archived, view-only, no-access, review-pending, failed-mapping.
- **Density & recovery:** virtualized tables, bulk bars, undo/redo present; **registry import with
  column mapping + per-row errors** shipped (v1.7.0); **column config + saved views** for the
  Features/Primers/Enzymes tables shipped (v1.8.0); **corrected re-submission** (request-changes →
  author corrects → resubmit with revision history) shipped (v1.9.0).

---

## 6. Release plan & success criteria

| Release | Theme | Exit criteria | Status |
|---|---|---|---|
| v0.7 | Connected records | lineage traceable; registry import validates; review locks; audit export | shipped |
| v0.8 | Inventory | place/find/transfer samples; plate snapshots immutable | shipped |
| v0.9 | Workflows | request→task→execute→output; failed-mapping recovery | shipped |
| v1.0 | Insights+Automation + UI revamp | ingest→transform→writeback; dashboards; template lifecycle; nav sidebar + full-page views | shipped |
| v1.1 | Editor view tabs | Plasmid / Linear / Metadata without changing topology | shipped |
| v1.2 | Split Workspace | side-by-side compare of two open documents | shipped |
| v1.3 | Saved alignments | save / load / delete named alignment snapshots | shipped |
| v1.4 | Primer secondary structure | hairpin + self-dimer preview & design badges | shipped |
| v1.5 | Enzyme find by cuts/region | cut-count + selection/custom-range filters | shipped |
| v1.6 | Home resume hub | My Work / recent files / worklist / recent entries | shipped |
| v1.7 | Import mapping + per-row errors | file-driven CSV, Name-column mapping, row issue report | shipped |
| v1.8 | Column config + saved views | show/hide columns; named column+sort+filter presets | shipped |
| v1.9 | Re-submission + Home calendar | request-changes→correct→resubmit w/ revisions; month calendar | shipped |
| v1.10 | Biologics (Phase 5) | formats, chains/domains, numbering, glyph, registration wizard | shipped |
| v1.11 | Chemistry (Phase 5) | SMILES parse, formula/MW, substructure search, reactions, salts | shipped |
| v1.12 | RNA/oligos (Phase 5) | modified monomers, patterns, HELM, oligo MW/GC | shipped |
| v1.13 | In Vivo (Phase 5) | studies, randomization, dosing, welfare alerts, weight graph | shipped |
| v1.14 | Bioprocess (Phase 5) | ISA-88 recipes, runs, worksheets, replicates, cross-run titer | shipped |
| v1.15 | Design revamp | contrast fix, shared row/section/empty-state widgets, card elevation, nav accent indicator | shipped |
| v1.16 | Design-system upgrade (quiet-md review) | alpha state fills, layered shadows, pressed states, measured neutrals, DESIGN.md contract | shipped |
| v1.17 | Design consistency | unify remaining hover fills to alpha token; DESIGN.md finalised (dark mode not planned) | shipped |
| v2.0 (optional) | Governed AI/integration (Phase 6) | approval+audit on all automated actions | optional |

Every release: `fmt` / `clippy -D warnings` / workspace tests / build green; **GitHub Actions CI watched
to completion (green)**; runtime-verified UI in the running app; installer + portable published;
README updated. (Figma design-system sync remains pending — Figma desktop is unstable on this machine.)

### Remaining work (consolidated)
- **§5 UX:** AI/Agents 3-step governed wizard (Phase 6).
- **Phase 6 (optional):** governed AI assist, Agents-lite, Model Hub-lite, local Developer/CLI API.
- **Housekeeping:** Figma design-system sync; bump deprecated `actions/checkout` Node 20 → newer.
- **Phase 5: COMPLETE** (Biologics, Chemistry, RNA/oligos, In Vivo, Bioprocess all shipped).

## 7. Risks & mitigations
- **Scope creep toward cloud:** keep multi-user/sync/GxP out-of-core; revisit only as a product decision.
- **Schema complexity:** start with a small typed-field set; iterate.
- **Performance (MSA/BLAST/tables):** virtualize + cap + background jobs; benchmark large plasmids.
- **AI safety:** approval + audit + citations mandatory; no autonomous writes.

## 8. Explicitly out-of-core (current direction)
Multi-tenant cloud, real-time collaboration, SSO/MFA/tenant admin, GxP/Validated Cloud, e-signatures,
and hosted instrument gateways. Local equivalents (audit, roles-lite, local pipelines) are in-scope.
