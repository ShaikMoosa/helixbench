# HelixBench — Product Improvement Plan (from reference resource analysis)

**Date:** 2026-11-14
**Scope:** Analyze the reference training / molecular-biology resources in
`reference-resources/_static/files/` and derive a prioritized improvement roadmap for
HelixBench (local-first, pure-Rust Windows DNA & plasmid workbench).
**Constraint:** every recommendation must stay implementable in pure Rust, offline, local-first
(no cloud, no account), consistent with the existing `domain / storage / appcore / apps` architecture.

---

## 1. Sources analyzed

| File | What it covers | Relevance |
| --- | --- | --- |
| `MolBio-Tools-Guide.pdf` | Full Molecular Biology suite guide: index + §§1–10 (visualization, annotation, codon opt, primer design, in-silico PCR, virtual digest, assembly wizard, alignment, CRISPR, tips) | **Primary** feature reference |
| `2026_Molecular_Biology_Tools_HandsOn_v4.pdf` | Newest hands-on deck, same workflow set as the guide | **Primary** (confirms current toolset) |
| `2025_Molecular_Biology_Tools_HandsOn_v4.pdf` / `_v3.pdf` / `2025_...HandsOn.pdf` | Earlier revisions of the hands-on training | Secondary (evolution of toolset) |
| `2025_02_10_...Introduction.pdf`, `2025_05_13_...Introduction.pdf`, `2026_02_02_...INTRODUCTION_Presentation.pdf` | Platform intro: Projects/folders, ELN entries, Registry entities (strains/plasmids/media), samples, requests, worklists, sharing | Organizational / future scope |
| `2026_Updated_Purchasing_Training_Users.pdf`, `material_order_guide.pdf` | Material orders / purchasing requests, order types & filters | Lab logistics — out of core scope |
| `Strain_Template_YourName.xlsx` | Strain registry import template (fields for strain entities) | Future registry schema reference |

---

## 2. Reference capability inventory (extracted)

**Sequence visualization & annotation**
- Plasmid map + linear view; ORF customization; topology & tags.
- Annotations / features libraries; **bulk auto-annotation**; feature library creation.
- Sequence search; auto-fill / back / bulk translations.

**Construct design**
- **Primer design** (manual + parameter-driven; adjustable thermodynamic defaults; add restriction
  sites / overhangs to primers; Tm / GC acceptable-range checks).
- **In-silico PCR** (primer pair → PCR product; choose which features to copy into the new DNA).
- **Virtual digestion** (customizable enzyme lists; digest visualization / fragment preview).

**Construct assembly**
- **Assembly Wizard**: Digest & Ligate (RE cloning), **Gibson**, **Golden Gate**.
- **Combinatorial / bulk assembly**; worklists integration.

**Sequence alignment**
- Template-based alignment, **consensus alignment**, **BLAST**, bulk auto-alignment.
- Powered by MAFFT / ClustalOmega; **auto-reindex circular sequences** on alignment.

**CRISPR**
- Guide RNA design; **on / off-target scoring**.

**Codon optimization**
- Host-aware codon optimization during sequence editing.

**AA / protein analysis**
- AA alignment; electrochemical / physicochemical properties overview.

**History & organization**
- Sequence **version history with restore of old versions**.
- Projects / subfolders, ELN entries, Registry (strains, plasmids, media), samples, requests,
  worklists; **bulk / expanded workspace view** (edit, move, archive, attach/detach primers,
  auto-annotate in bulk); starring; sharing.

**UX tips worth borrowing**
- Bulk operations via an expanded table view; auto-indexing for circular alignments; clear
  parameter defaults with "PRO TIP" guidance; wizard-style multi-step construct building.

---

## 3. HelixBench current state (baseline)

Already implemented: circular/linear map (pastel features, enzyme leader-lines, minimap, zoom
slider); double-stranded sequence view (complement, translations, ORFs, selection, colors);
Features CRUD + Detect Common + translated CDS; Primers (Tm, GC%, direction, phosphorylation);
Enzymes (DB, sets, filters, cut sites); undo/redo history; DNA/protein/name IUPAC search;
`.hbx` / FASTA / GenBank / raw import + FASTA/GenBank/plain export; genetic codes (7), ORF threshold,
topology; light modern editor; installer + release.

---

## 4. Gap analysis (current status, v0.6.0)

The table below was originally a pre-implementation snapshot. As of **v0.6.0** every compute /
organization row is **shipped**; only the enterprise lab-logistics row remains open (deliberately
out-of-core for a local-first, single-user product).

| Capability | the reference platform | HelixBench v0.6.0 | Status | Released |
| --- | --- | --- | --- | --- |
| In-silico PCR | yes | amplicon preview + product w/ feature copy | **Shipped** | v0.2.0 |
| Primer design assistant | yes | Tm/GC/clamp scoring + overhangs | **Shipped** | v0.2.0 |
| Virtual digest / gel preview | yes | fragments + agarose-gel simulation | **Shipped** | v0.2.0 |
| Pairwise / consensus alignment | yes | progressive MSA (UPGMA+profile) + consensus + auto-reindex + bulk | **Shipped** | v0.2.0 / v0.6.0 |
| Version history + restore | yes | persistent named snapshots + restore | **Shipped** | v0.3.0 |
| Assembly wizard (RE/Gibson/Golden Gate) | yes | assemblies + Gibson primer design + GG overhang validation | **Shipped** | v0.3.0 / v0.6.0 |
| CRISPR gRNA + on/off-target | yes | PAM scan + Doench-2 on-target + CFD off-target list | **Shipped** | v0.3.0 / v0.6.0 |
| Codon optimization | yes | host tables + CAI + GC-window + site avoidance + harmonization | **Shipped** | v0.3.0 / v0.6.0 |
| Features library / bulk auto-annotation | yes | reusable library + bulk apply across docs | **Shipped** | v0.6.0 |
| Protein / AA analysis | yes | BLOSUM62 alignment + pI/MW/GRAVY/charge/extinction | **Shipped** | v0.6.0 |
| Bulk / expanded workspace + projects/tags | yes | multi-doc tabs + tags + bulk tag/export/close | **Shipped** | v0.5.0 / v0.6.0 |
| BLAST (local DB) | yes | seed-and-extend vs loaded FASTA or open docs | **Shipped** | v0.4.0 / v0.6.0 |
| Combinatorial / bulk assembly | yes | all insert orderings (capped) | **Shipped** | v0.4.0 |
| Registry | yes | offline plasmid/strain/primer catalog + link-to-doc | **Shipped** | v0.5.0 |
| ELN / worklists / purchasing | yes | not implemented | **Open (out-of-core)** | — |
| Multi-user / collaboration / cloud sync | yes | not implemented (local-first by design) | **Open (out-of-core)** | — |

### Remaining gaps (scope decisions, not compute)
- **ELN / worklists / purchasing** and **multi-user / cloud sync** are enterprise lab-logistics
  features. They conflict with the local-first, single-user, offline design; adding them is a
  product-direction change (accounts, sync, permissions), not a compute gap.
- Depth nuance: MSA uses a progressive heuristic (not MAFFT/ClustalOmega), CRISPR off-target is
  within-open-documents (not genome-wide), BLAST is local (no NCBI). These are reasonable offline
  equivalents; upgrading to external databases/genomes would require network or large bundled data.

---

## 5. Prioritized roadmap

### P0 — core molecular-biology parity (pure Rust, high value)

1. **In-silico PCR** (`domain::pcr`)
   - Input: forward + reverse primer (from primer list or typed), template.
   - Compute amplicon (position, length, Tm, GC), handle circular templates and reverse-strand primers.
   - Output: new sequence document; **copy selected features** into the product (reference parity).
   - UI: PCR dialog (pick primers, preview amplicon, feature-copy checkboxes) → creates doc tab.
   - Tests: known primer pair → expected amplicon coords/length on linear + circular.

2. **Primer design assistant** (`domain::primer_design`)
   - Auto-design primers flanking a selected region / target: length, Tm (nearest-neighbor), GC%,
    3' GC-clamp, self-dimer / hairpin sanity checks; optional 5' overhangs / restriction sites.
   - Expose adjustable thermodynamic defaults (reference "PRO TIP").
   - UI: "Design primers" from selection → candidate list with scores → add to primer list.
   - Tests: Tm/GC math vs reference values; clamp/overhang placement.

3. **Virtual digest + gel preview** (`domain::digest`)
   - From chosen enzyme set: fragment list (sizes, count, ends), single/double digest.
   - **Agarose-gel simulation view** (log-scale migration vs ladder) — strong visual win.
   - UI: Enzymes view → "Simulate digest" → fragments table + gel panel.
   - Tests: fragment sizes sum to template length; circular vs linear cut counts.

4. **Alignment** (`domain::align`)
   - Pairwise (Needleman–Wunsch global / Smith–Waterman local) with affine gaps; template-based
     alignment of a sequence to a reference; **consensus** row for multi-sequence.
   - **Auto-reindex circular** sequences before aligning (reference default), toggleable.
   - UI: Alignment view (rows + consensus + mismatch highlighting), reindex toggle.
   - Tests: known alignments, consensus correctness, circular reindex.

5. **Persistent version history + restore** (`storage::versions`)
   - Snapshot document versions (on save / manual "save version") in `.hbx` or sidecar; list with
     timestamps + labels; **restore** an old version as a new document / current state.
   - UI: History view → version timeline with restore.
   - Tests: snapshot/restore round-trip; version ordering; corrupt-version handling.

### P1 — differentiating tools

6. **Assembly wizard** (`domain::assembly`): Digest&Ligate, Gibson (overlap design), Golden Gate
   (Type IIS overhangs); vector + inserts → assembled construct with carried features; preview junctions.
7. **CRISPR** (`domain::crispr`): PAM (NGG + variants) scan, gRNA candidates, on-target score
   (Doench-style simplified), off-target scan vs the loaded sequence(s) with mismatch counts.
8. **Codon optimization** (`domain::codon`): host codon-usage tables, CAI target, GC-window smoothing,
   avoid chosen restriction sites; preserve protein; show before/after identity.
9. **Features library + bulk auto-annotation** (`domain::feature_lib`): curated reusable library;
   batch-apply across open documents; extend Detect Common.
10. **Protein analysis** (`domain::protein`): AA alignment, MW, pI, charge, hydropathy (GRAVY),
    electrochemical overview panel.
11. **Bulk workspace + organization**: multi-document expanded table (batch edit/move/archive,
    attach/detach primers, auto-annotate), project folders + tags in a local library index.

### P2 — optional / future

12. **Local BLAST** against a user-supplied FASTA database (seed-and-extend, pure Rust).
13. **Combinatorial / bulk assembly** (multi-fragment combinatorics + worklist export).
14. **Registry / ELN-lite / worklists / purchasing** — lab logistics. *Recommendation:* keep out of the
    core DNA editor; if pursued, model as a separate local registry module (strain/plasmid entities,
    using `Strain_Template_YourName.xlsx` fields) rather than bloating the sequence editor.

---

## 6. UX improvements (borrowed from reference tips + reference editor)

- **Wizard-style dialogs** for PCR / assembly / primer design (multi-step, preview before commit).
- **Bulk / expanded view** for primers, features, enzymes (multi-select + batch actions).
- **Auto-index** toggle for circular alignments; sensible parameter defaults with inline "pro tip" help.
- **Gel + alignment + version-timeline** as first-class visual panels in the light editor.
- Keyboard shortcuts and drag-select parity across map / sequence / tables (already partly present).

---

## 7. Architecture & verification notes

- Add pure-Rust modules under `crates/domain` (`pcr`, `primer_design`, `digest`, `align`, `assembly`,
  `crispr`, `codon`, `protein`, `feature_lib`); keep them UI/OS/network-free.
- Expose typed commands via `crates/appcore`; persist versions/library via `crates/storage`.
- GPUI views/dialogs in `apps/desktop_gpui`; egui fallback inherits commands automatically.
- Each P0/P1 feature ships with unit tests (math + round-trips) and a runtime-verified UI flow;
  run `cargo fmt / test / clippy -D warnings / build` before release.
- Update the Figma design system + README preview as new panels land.

---

## 8. Suggested next step

Implement **P0 #1–#3 (in-silico PCR, primer design, virtual digest + gel)** first: they reuse the
existing primer/enzyme/feature model, deliver immediately visible value, and are fully testable in
pure Rust. Then alignment + version history (P0 #4–#5), then the P1 differentiators.