<p align="center">
  <img src="apps/desktop_gpui/assets/helixbench.png" alt="HelixBench logo" width="120" />
</p>

<h1 align="center">HelixBench</h1>

<p align="center">
  A local-first DNA &amp; plasmid workbench for Windows, written in pure Rust.<br />
  Restriction mapping, features, primers, ORFs and 6-frame translation — native, offline, no account.
</p>

---

HelixBench is a desktop sequence editor in the spirit of classic plasmid tools: open a sequence,
see it annotated on a circular map and as a double-stranded sequence view, cut it with restriction
enzymes, design primers, translate open reading frames, and search DNA or protein — all without
leaving your machine. Documents are plain files on disk; saves are atomic and versioned. There is
no cloud, no telemetry, and no sign-in.

Two native shells share one framework-agnostic core:

| Shell | Crate | Role |
| --- | --- | --- |
| **GPUI** (Zed's UI framework) | `apps/desktop_gpui` | Primary shell — night-lab instrument design |
| **egui / eframe** | `apps/desktop` | Lightweight fallback shell |

Both dispatch the same typed commands from `appcore` over the same `domain` + `storage` crates, so
behavior and file formats are identical across shells.

## Features

- **Circular map** — features, enzyme cut sites, primers and ORFs drawn on a plasmid map with a
  ruler, zoom window and topology (linear / circular).
- **Sequence view** — annotated double-stranded rows with complement strand, per-base colors,
  selection, zoom + minimap, and inline translations.
- **Restriction analysis** — curated enzyme database, enzyme sets, cut-site filters (cutters,
  methylation-sensitive, unique / double / non-cutters), and per-enzyme cut positions.
- **Features** — create / edit / delete annotations, translated CDS products, segments, and
  one-click **Detect Common Features** (promoters, terminators, origins, markers, …).
- **Primers** — design and list primers with length, GC%, melting temperature (Tm) and direction;
  phosphorylation flag.
- **Translation & ORFs** — 6-frame translation, ORF detection with a minimum-length threshold, and
  **7 genetic codes** (Standard, Vertebrate / Yeast / Mold / Invertebrate Mitochondrial,
  Ciliate, Bacterial / Plastid).
- **Search** — DNA, protein or feature-name search with IUPAC ambiguity codes, match navigation and
  on-sequence highlighting.
- **Sequence colors** — None, History (edit provenance) or Custom DNA color regions.
- **History** — full undo / redo with an audit list; history coloring shows what changed.
- **Six views** — Map, Sequence, Features, Primers, Enzymes, History.

## File formats

| Direction | Formats |
| --- | --- |
| Open / import | `.hbx` (native), GenBank (`.gb` `.gbk` `.genbank` `.gp`), FASTA (`.fa` `.fasta` `.fna` `.faa`), raw sequence text |
| Save | `.hbx` — versioned JSON, written atomically (temp file + rename) |
| Export | FASTA, GenBank, plain text |

## Getting started

Requirements: **Windows 10/11 (x64)** and a **Rust stable toolchain (MSVC)** with the Visual Studio
C++ build tools. No Node.js, no webview, no runtime downloads.

```bash
# primary GPUI shell
cargo run -p desktop_gpui

# fallback egui shell
cargo run -p desktop

# optimized release binary
cargo build --release -p desktop_gpui
# -> target/release/helixbench_gpui.exe
```

On first launch HelixBench opens a seeded demo plasmid (`pHelixBench-Demo`, 3021 bp) so every view
is populated and explorable immediately.

### Keyboard shortcuts

| Keys | Action |
| --- | --- |
| `Ctrl+1` … `Ctrl+6` | Switch view (Map, Sequence, Features, Primers, Enzymes, History) |
| `Ctrl+N` / `Ctrl+O` | New / Open |
| `Ctrl+S` / `Ctrl+Shift+S` | Save / Save As |
| `Ctrl+Z` / `Ctrl+Y` | Undo / Redo |
| `Ctrl+F` | Find (DNA / protein / name) |
| `Ctrl+A` / `Ctrl+C` / `Ctrl+X` / `Ctrl+V` | Select all / copy / cut / paste |

Zoom is enabled via **View → Zoom controls** (or the sidebar / Options toggle), then driven by the
zoom bar (`+`, `−`, Fit, center-on-selection) and the draggable minimap in the Sequence view.

## Architecture

```
helixbench/
├── crates/
│   ├── domain/      # pure entities & algorithms: sequence, codes, translation, ORFs,
│   │                # enzymes, features, primers, search. No UI / OS / network deps.
│   ├── storage/     # versioned .hbx persistence, FASTA / GenBank IO, atomic writes, config
│   └── appcore/     # framework-agnostic AppState + typed commands (actions), undo/redo
└── apps/
    ├── desktop_gpui/  # GPUI shell: views, dialogs, theme, icons (primary)
    └── desktop/       # egui/eframe shell (fallback)
```

Dependency rule: shells depend inward on `appcore` → `domain` / `storage`. The core crates never
import UI, OS, or network code, which keeps the logic testable headlessly and identical across
shells.

## Design

The GPUI shell follows a light, Benchling-style laboratory editor: a white canvas and chrome with a
single dark top application bar, ink text, one teal accent for selection, and pastel feature fills
so annotations read softly on light. The editor splits into a circular map (left) and an annotated
double-stranded sequence panel (right), with description / properties / legend cards below. A left
icon rail switches views and actions; monochrome stroke icons, nested radii (8/6/4) and a 4px grid
keep the chrome quiet so the sequence stays the focus.

## Development

```bash
cargo fmt --all -- --check
cargo test --workspace --all-features
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo build --workspace --all-targets --all-features
```

The domain, storage and appcore crates are covered by unit tests (translation, ORFs, enzyme
cutting, search, `.hbx` round-trips, atomic-write behavior).

## License

MIT — see the workspace `Cargo.toml`.