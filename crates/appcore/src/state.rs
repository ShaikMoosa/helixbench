//! Application state: document, undo/redo history, selection, search, dialogs.

use std::collections::HashMap;
use std::path::PathBuf;

use domain::common_features::DetectedFeature;
use domain::enzyme::CutSite;
use domain::primer::BindingSite;
use domain::search::{DnaHit, ProteinHit};
use domain::{ColorRegion, Direction, Document, Enzyme, Orf, Range, StrandMask, Topology};
use storage::config::{Config, UiPrefs};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum View {
    Map,
    Sequence,
    Features,
    Primers,
    Enzymes,
    History,
    Home,
    Notebook,
    Registry,
    Inventory,
    Workflows,
    Insights,
    Automation,
    Split,
    Biologics,
    Chemistry,
}

impl View {
    pub const ALL: [View; 6] = [
        View::Map,
        View::Sequence,
        View::Features,
        View::Primers,
        View::Enzymes,
        View::History,
    ];

    pub const LAB: [View; 10] = [
        View::Home,
        View::Notebook,
        View::Registry,
        View::Inventory,
        View::Workflows,
        View::Insights,
        View::Automation,
        View::Split,
        View::Biologics,
        View::Chemistry,
    ];

    pub fn label(self) -> &'static str {
        match self {
            View::Map => "Map",
            View::Sequence => "Sequence",
            View::Features => "Features",
            View::Primers => "Primers",
            View::Enzymes => "Enzymes",
            View::History => "History",
            View::Home => "Home",
            View::Notebook => "Notebook",
            View::Registry => "Registry",
            View::Inventory => "Inventory",
            View::Workflows => "Workflows",
            View::Insights => "Insights",
            View::Automation => "Automation",
            View::Split => "Split",
            View::Biologics => "Biologics",
            View::Chemistry => "Chemistry",
        }
    }

    pub fn icon(self) -> &'static str {
        match self {
            View::Map => "◉",
            View::Sequence => "≡",
            View::Features => "◧",
            View::Primers => "➤",
            View::Enzymes => "✂",
            View::History => "↺",
            View::Home => "⌂",
            View::Notebook => "✎",
            View::Registry => "▤",
            View::Inventory => "▦",
            View::Workflows => "⇄",
            View::Insights => "◔",
            View::Automation => "⚙",
            View::Split => "⧉",
            View::Biologics => "Y",
            View::Chemistry => "⚗",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorsMode {
    None,
    History,
    Custom,
}

impl ColorsMode {
    pub fn label(self) -> &'static str {
        match self {
            ColorsMode::None => "No colors",
            ColorsMode::History => "History colors",
            ColorsMode::Custom => "Custom DNA colors",
        }
    }
}

/// One undoable operation: the document snapshot from before it ran, plus the
/// regions it touched (used for History colors).
#[derive(Clone)]
pub struct HistoryEntry {
    pub label: String,
    pub snapshot: Document,
    pub regions: Vec<ColorRegion>,
}

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum SearchMode {
    #[default]
    Dna,
    Protein,
    Name,
}

impl SearchMode {
    pub fn label(self) -> &'static str {
        match self {
            SearchMode::Dna => "Find DNA sequence:",
            SearchMode::Protein => "Find protein sequence:",
            SearchMode::Name => "Find enzyme/feature/primer:",
        }
    }
}

#[derive(Clone)]
pub enum NameHit {
    Feature(u64),
    Primer(u64),
    Enzyme(String),
}

#[derive(Default)]
pub struct SearchState {
    pub open: bool,
    pub mode: SearchMode,
    pub query: String,
    pub recents: Vec<(SearchMode, String)>,
    pub dna_hits: Vec<DnaHit>,
    pub protein_hits: Vec<ProteinHit>,
    pub name_hits: Vec<NameHit>,
    pub current: usize,
    pub error: Option<String>,
    pub focus_request: bool,
}

impl SearchState {
    pub fn hit_count(&self) -> usize {
        match self.mode {
            SearchMode::Dna => self.dna_hits.len(),
            SearchMode::Protein => self.protein_hits.len(),
            SearchMode::Name => self.name_hits.len(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SortDir {
    Asc,
    Desc,
}

#[derive(Default, Clone, Copy)]
pub struct TableSort {
    pub column: usize,
    pub dir: Option<SortDir>,
}

#[derive(Clone)]
pub struct FeatureDraft {
    pub id: Option<u64>,
    pub name: String,
    pub kind: String,
    pub segments_text: String,
    pub direction: Direction,
    pub color: [u8; 3],
    pub note: String,
    pub product: String,
    pub translated: bool,
    pub genetic_code: u8,
    pub first_codon_met: bool,
    pub frame_offset: usize,
    pub error: Option<String>,
}

#[derive(Clone)]
pub struct PrimerDraft {
    pub id: Option<u64>,
    pub name: String,
    pub sequence: String,
    pub description: String,
    pub color: [u8; 3],
    pub phosphorylated: bool,
    pub error: Option<String>,
}

pub struct NewSeqState {
    pub name: String,
    pub text: String,
    pub topology: Topology,
    pub error: Option<String>,
}

impl Default for NewSeqState {
    fn default() -> Self {
        Self {
            name: "Untitled".into(),
            text: String::new(),
            topology: Topology::Linear,
            error: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PendingAction {
    CloseApp,
    NewDemo,
    NewEmpty,
    Open,
    OpenRecent(PathBuf),
}

pub enum Dialog {
    NewSequence(NewSeqState),
    FeatureEditor(FeatureDraft),
    PrimerEditor(PrimerDraft),
    GeneticCodes {
        selected: u8,
    },
    LetterCodes,
    SearchTips,
    About,
    Options,
    Rotate {
        position: String,
        error: Option<String>,
    },
    Unsaved {
        action: PendingAction,
    },
    DetectCommon {
        detected: Vec<DetectedFeature>,
        checked: Vec<bool>,
    },
    ColorSelection(ColorDraft),
    Pcr(PcrDraft),
    PrimerDesign(PrimerDesignDraft),
    Digest,
    Alignment(AlignmentDraft),
    Assembly(AssemblyDraft),
    Crispr(CrisprDraft),
    Codon(CodonDraft),
    Blast(BlastDraft),
    Combinatorial(CombDraft),
    Registry(RegistryDraft),
    RegistryEditor(RegistryEditDraft),
    Protein(ProteinDraft),
    FeatureLib(FeatureLibDraft),
    Workspace(WorkspaceDraft),
    Notebook(NotebookDraft),
    EntryEditor(EntryEditDraft),
    Projects(ProjectsDraft),
    Audit(AuditDraft),
    Home(HomeDraft),
    EntityMap(EntityMapDraft),
    RegistryImport(RegistryImportDraft),
    Inventory(InventoryDraft),
    Workflows(WorkflowDraft),
    Biologics(BiologicsDraft),
    ChemAdd(ChemAddDraft),
    ChemReaction(ChemReactionDraft),
    ChemLot(ChemLotDraft),
}

#[derive(Clone, Copy)]
pub struct ColorDraft {
    pub range: Range,
    pub color: [u8; 3],
    pub strands: StrandMask,
}

/// A non-active open document (template kept while viewing a PCR product, etc.).
#[derive(Clone)]
pub struct OpenDoc {
    pub doc: Document,
    pub path: Option<PathBuf>,
    pub dirty: bool,
}

#[derive(Clone, Default)]
pub struct PcrDraft {
    pub fwd: String,
    pub rev: String,
    pub copy_features: bool,
    pub error: Option<String>,
}

#[derive(Clone)]
pub struct PrimerDesignDraft {
    pub start: String,
    pub len: String,
    pub target_tm: String,
    pub overhang: String,
    pub fwd: Vec<domain::primer_design::Candidate>,
    pub rev: Vec<domain::primer_design::Candidate>,
    pub fwd_pick: usize,
    pub rev_pick: usize,
    pub error: Option<String>,
}

#[derive(Clone)]
pub struct AlignmentDraft {
    /// Index into `AppState::library` used as the reference.
    pub reference: usize,
    pub reindex: bool,
    pub result: Option<domain::align::Alignment>,
    /// Row labels for the current result (saved alignments carry their own).
    pub labels: Vec<String>,
    pub error: Option<String>,
}

#[derive(Clone)]
pub struct AssemblyDraft {
    pub strategy: usize,
    pub cut: String,
    pub inserts: Vec<bool>,
    pub primers: Vec<domain::assembly::JunctionPrimers>,
    pub gg_issues: Vec<String>,
    pub error: Option<String>,
}

#[derive(Clone, Default)]
pub struct CrisprDraft {
    pub limit: String,
    pub guides: Vec<domain::crispr::Guide>,
    pub off: Vec<domain::crispr::OffTarget>,
    pub error: Option<String>,
}

#[derive(Clone)]
pub struct CodonDraft {
    pub host: usize,
    pub gc_min: String,
    pub gc_max: String,
    pub avoid: String,
    pub harmonize: bool,
    pub result: Option<Vec<u8>>,
    pub cai_before: f32,
    pub cai_after: f32,
    pub error: Option<String>,
}

#[derive(Clone, Default)]
pub struct BlastDraft {
    pub k: String,
    pub max: String,
    pub db_count: usize,
    pub hits: Vec<domain::blast::Hit>,
    pub error: Option<String>,
}

#[derive(Clone, Default)]
pub struct ProteinDraft {
    pub stats: Option<domain::protein::ProteinStats>,
    pub align: Option<(Vec<u8>, Vec<u8>, i32)>,
    pub error: Option<String>,
}

#[derive(Clone, Default)]
pub struct FeatureLibDraft {
    pub name: String,
    pub kind: String,
    pub motif: String,
    pub error: Option<String>,
}

#[derive(Clone, Default)]
pub struct WorkspaceDraft {
    pub selected: Vec<bool>,
    pub tag: String,
    pub error: Option<String>,
}

#[derive(Clone, Default)]
pub struct NotebookDraft {
    pub filter: usize,
    pub error: Option<String>,
}

#[derive(Clone)]
pub struct EntryEditDraft {
    pub id: Option<u64>,
    pub title: String,
    pub project: Option<u64>,
    pub template: usize,
    pub sections: Vec<(String, String)>,
    pub tables: Vec<domain::notebook::ResultTable>,
    pub error: Option<String>,
}

#[derive(Clone, Default)]
pub struct ProjectsDraft {
    pub name: String,
    pub parent: Option<u64>,
    pub error: Option<String>,
}

#[derive(Clone, Default)]
pub struct AuditDraft {
    pub filter: String,
}

#[derive(Clone, Default)]
pub struct HomeDraft {}

#[derive(Clone, Default)]
pub struct EntityMapDraft {}

/// Phase 5 biologics registration wizard draft. `seqs` holds one amino-acid
/// sequence per expected chain of the chosen format.
#[derive(Clone, Default)]
pub struct BiologicsDraft {
    pub name: String,
    pub format: usize,
    pub numbering: usize,
    pub seqs: Vec<String>,
    pub error: Option<String>,
}

/// Phase 5 chemistry: add-compound dialog draft.
#[derive(Clone, Default)]
pub struct ChemAddDraft {
    pub name: String,
    pub smiles: String,
    pub error: Option<String>,
}

/// Phase 5 chemistry: add-reaction dialog draft (compound indices + equivalents).
#[derive(Clone, Default)]
pub struct ChemReactionDraft {
    pub name: String,
    pub reactant: usize,
    pub product: usize,
    pub req: String,
    pub peq: String,
    pub error: Option<String>,
}

/// Phase 5 chemistry: add-lot dialog draft (form index into salt library).
#[derive(Clone, Default)]
pub struct ChemLotDraft {
    pub form: usize,
    pub quantity: String,
    pub unit: String,
    pub error: Option<String>,
}

#[derive(Clone, Default)]
pub struct RegistryImportDraft {
    pub csv: String,
    pub kind: usize,
    /// Column mapped to Name; None = auto-detect a "Name" header (else first).
    pub name_col: Option<usize>,
    pub row_errors: Vec<String>,
    pub error: Option<String>,
    pub imported: usize,
}

#[derive(Clone, Default)]
pub struct InventoryDraft {
    pub name: String,
    pub kind: usize,
    pub parent: Option<u64>,
    pub selected_plate: Option<u64>,
    pub lookup: String,
    pub error: Option<String>,
}

#[derive(Clone, Default)]
pub struct WorkflowDraft {
    pub title: String,
    pub schema: String,
    pub request_name: String,
    pub output_key: String,
    pub output_val: String,
    pub selected_task: Option<u64>,
    pub error: Option<String>,
}

#[derive(Clone)]
pub struct CombDraft {
    pub strategy: usize,
    pub cut: String,
    pub cap: String,
    pub results: Vec<domain::assembly::Assembly>,
    pub labels: Vec<String>,
    pub error: Option<String>,
}

#[derive(Clone, Default)]
pub struct RegistryDraft {
    /// 0 = all, 1..=3 = Kind index + 1.
    pub filter: usize,
    pub error: Option<String>,
}

#[derive(Clone)]
pub struct RegistryEditDraft {
    pub id: Option<u64>,
    pub kind: usize,
    pub values: Vec<String>,
    pub error: Option<String>,
}

#[derive(Default)]
pub struct EnzymeFilterState {
    pub name: String,
    pub cut_count: usize, // index into CUT_COUNT_OPTIONS
    pub overhang: usize,  // index into OVERHANG_OPTIONS
    pub site_len: usize,  // index into SITE_LEN_OPTIONS
    pub region: usize,    // index into REGION_OPTIONS
    pub region_start: String,
    pub region_end: String,
}

pub const REGION_OPTIONS: [&str; 3] = ["Anywhere", "In selection", "Custom range"];

pub const CUT_COUNT_OPTIONS: [&str; 6] = [
    "Any cuts",
    "Noncutters (0)",
    "Unique (1)",
    "Dual (2)",
    "≤ 2 cuts",
    "≥ 3 cuts",
];
pub const OVERHANG_OPTIONS: [&str; 5] = [
    "Any overhang",
    "5′ overhang",
    "3′ overhang",
    "Blunt",
    "Type IIS (outside)",
];
pub const SITE_LEN_OPTIONS: [&str; 5] = ["Any length", "4 bp", "5 bp", "6 bp", "≥ 7 bp"];

/// Editor map presentation: follow topology, force a shape, or show metadata.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MapMode {
    Auto,
    Plasmid,
    Linear,
    Metadata,
}

pub struct AppState {
    pub doc: Document,
    /// Other open documents (e.g. PCR products); `doc` is the active one.
    pub library: Vec<OpenDoc>,
    /// Local offline catalog of plasmids / strains / primers.
    pub registry: domain::registry::Registry,
    /// Reusable feature library for bulk auto-annotation.
    pub feature_lib: Vec<domain::feature_lib::LibFeature>,
    /// External BLAST database (loaded FASTA); empty = use open documents.
    pub blast_db: Vec<(String, Vec<u8>)>,
    /// Phase-1 connected records: projects, notebook entries, templates, audit.
    pub records: storage::records::Records,
    /// Phase-2 physical inventory (locations/boxes/plates/containers).
    pub inventory: domain::inventory::Inventory,
    /// Phase-3 local workflows (tasks/requests/flowchart).
    pub workflows: domain::workflow::Workflows,
    /// v1.0 insights blocks + automation pipelines/runs.
    pub analytics: storage::analytics::Analytics,
    /// v1.3 saved sequence alignments.
    pub alignments: storage::alignments::Alignments,
    /// CSV loaded from disk for the next automation run (file-ingestion).
    pub automation_csv: String,
    pub automation_csv_name: String,
    pub path: Option<PathBuf>,
    pub dirty: bool,
    /// History depth at the last save/open; `None` once a branch makes that state unreachable.
    pub saved_depth: Option<usize>,
    pub history: Vec<HistoryEntry>,
    pub future: Vec<HistoryEntry>,
    pub history_highlight: Option<usize>,
    pub colors_mode: ColorsMode,
    pub selection: Option<Range>,
    pub selected_feature: Option<u64>,
    pub selected_primer: Option<u64>,
    pub view: View,
    pub map_mode: MapMode,
    /// Split Workspace: index into `library` of the document in the right pane.
    pub split_sel: usize,
    pub prefs: UiPrefs,
    pub config: Config,
    pub search: SearchState,
    pub dialog: Option<Dialog>,
    pub toast: Option<(String, std::time::Instant)>,
    pub zoom_start: usize,
    pub zoom_width: usize,
    pub scroll_to: Option<usize>,
    pub enzyme_filter: EnzymeFilterState,
    /// Per-table hidden column indices (column config).
    pub table_hidden: HashMap<String, Vec<usize>>,
    /// Saved table views (column config + sort + filter presets).
    pub views: storage::views::Views,
    /// Phase 5 biologics entities.
    pub biologics: storage::biologics::Biologics,
    /// Selected biologic in the Biologics view.
    pub bio_sel: Option<u64>,
    /// Phase 5 chemistry compounds / reactions / lots.
    pub chemistry: storage::chemistry::Chemistry,
    /// Selected compound in the Chemistry view.
    pub chem_sel: Option<u64>,
    /// Substructure SMILES filter in the Chemistry view.
    pub chem_search: String,
    pub feature_sort: TableSort,
    pub primer_sort: TableSort,
    pub enzyme_sort: TableSort,
    pub feature_filter: String,
    pub primer_filter: String,
    pub analysis: Analysis,
    pub analysis_dirty: bool,
    pub map_drag_anchor: Option<usize>,
    pub seq_drag_anchor: Option<usize>,
    pub exit_requested: bool,
}

#[derive(Default)]
pub struct Analysis {
    pub chosen: Vec<(Enzyme, Vec<CutSite>)>,
    pub counts: HashMap<String, usize>,
    pub all_sites: HashMap<String, Vec<CutSite>>,
    pub sets: Vec<domain::EnzymeSet>,
    pub orfs: Vec<Orf>,
    pub primer_sites: HashMap<u64, Vec<BindingSite>>,
}

impl AppState {
    pub fn new(doc: Document, config: Config) -> Self {
        let prefs = config.ui.clone();
        let zoom_width = doc.sequence.len().max(1);
        let mut s = Self {
            doc,
            library: Vec::new(),
            registry: storage::registry::load(),
            feature_lib: storage::feature_lib::load(),
            blast_db: Vec::new(),
            records: storage::records::load(),
            inventory: storage::inventory::load(),
            workflows: storage::workflow::load(),
            analytics: storage::analytics::load(),
            alignments: storage::alignments::load(),
            automation_csv: String::new(),
            automation_csv_name: String::new(),
            path: None,
            dirty: false,
            saved_depth: Some(0),
            history: Vec::new(),
            future: Vec::new(),
            history_highlight: None,
            colors_mode: ColorsMode::None,
            selection: None,
            selected_feature: None,
            selected_primer: None,
            view: View::Map,
            map_mode: MapMode::Auto,
            split_sel: 0,
            prefs,
            config,
            search: SearchState::default(),
            dialog: None,
            toast: None,
            zoom_start: 0,
            zoom_width,
            scroll_to: None,
            enzyme_filter: EnzymeFilterState::default(),
            table_hidden: HashMap::new(),
            views: storage::views::load(),
            biologics: storage::biologics::load(),
            bio_sel: None,
            chemistry: storage::chemistry::load(),
            chem_sel: None,
            chem_search: String::new(),
            feature_sort: TableSort::default(),
            primer_sort: TableSort::default(),
            enzyme_sort: TableSort::default(),
            feature_filter: String::new(),
            primer_filter: String::new(),
            analysis: Analysis::default(),
            analysis_dirty: true,
            map_drag_anchor: None,
            seq_drag_anchor: None,
            exit_requested: false,
        };
        s.recompute();
        s
    }

    pub fn recompute(&mut self) {
        self.analysis = crate::analysis::compute(&self.doc, &self.prefs);
        self.analysis_dirty = false;
    }

    /// Effective map shape for the current view mode (Auto follows topology).
    pub fn map_circular(&self) -> bool {
        match self.map_mode {
            MapMode::Plasmid => true,
            MapMode::Linear => false,
            _ => self.doc.sequence.topology == domain::Topology::Circular,
        }
    }

    /// Hidden column indices for a list table (column config).
    pub fn hidden_cols(&self, table: &str) -> Vec<usize> {
        self.table_hidden.get(table).cloned().unwrap_or_default()
    }

    pub fn col_hidden(&self, table: &str, col: usize) -> bool {
        self.table_hidden
            .get(table)
            .is_some_and(|h| h.contains(&col))
    }

    /// Region used by the enzyme "find by region" filter, if any.
    /// Custom bounds are 1-based inclusive; selection is used as-is.
    pub fn enzyme_region(&self) -> Option<Range> {
        let l = self.doc.sequence.len();
        match self.enzyme_filter.region {
            1 => self.selection,
            2 => {
                let start = self
                    .enzyme_filter
                    .region_start
                    .trim()
                    .parse::<usize>()
                    .unwrap_or(1);
                let end = self
                    .enzyme_filter
                    .region_end
                    .trim()
                    .parse::<usize>()
                    .unwrap_or(l);
                if l == 0 || start == 0 || end == 0 {
                    return None;
                }
                let start = start.min(l);
                let end = end.min(l);
                if end < start {
                    return None;
                }
                Some(Range::new(start - 1, end - start + 1))
            }
            _ => None,
        }
    }

    pub fn ensure_analysis(&mut self) {
        if self.analysis_dirty {
            self.recompute();
        }
    }

    /// Run a mutating operation with undo tracking and history colors.
    pub fn mutate(
        &mut self,
        label: &str,
        regions: Vec<ColorRegion>,
        f: impl FnOnce(&mut Document),
    ) {
        if self.saved_depth.is_some_and(|s| self.history.len() < s) {
            self.saved_depth = None;
        }
        let snapshot = self.doc.clone();
        f(&mut self.doc);
        self.history.push(HistoryEntry {
            label: label.to_string(),
            snapshot,
            regions,
        });
        if self.history.len() > 200 {
            self.history.remove(0);
            self.saved_depth = self.saved_depth.and_then(|s| s.checked_sub(1));
        }
        self.future.clear();
        self.refresh_dirty();
        self.analysis_dirty = true;
        self.history_highlight = None;
        let l = self.doc.sequence.len();
        if let Some(sel) = self.selection {
            if l == 0 {
                self.selection = None;
            } else {
                self.selection = Some(sel.normalized(l));
            }
        }
        self.zoom_width = self.zoom_width.min(l.max(1));
        self.zoom_start = if l > 0 { self.zoom_start % l } else { 0 };
        if self.search.open && !self.search.query.is_empty() {
            self.run_search();
        }
    }

    fn refresh_dirty(&mut self) {
        self.dirty = self.saved_depth != Some(self.history.len());
    }

    pub fn undo(&mut self) -> bool {
        let Some(entry) = self.history.pop() else {
            return false;
        };
        let current = self.doc.clone();
        self.future.push(HistoryEntry {
            label: entry.label.clone(),
            snapshot: current,
            regions: entry.regions.clone(),
        });
        self.doc = entry.snapshot;
        self.after_time_travel();
        true
    }

    pub fn redo(&mut self) -> bool {
        let Some(entry) = self.future.pop() else {
            return false;
        };
        let current = self.doc.clone();
        self.history.push(HistoryEntry {
            label: entry.label.clone(),
            snapshot: current,
            regions: entry.regions.clone(),
        });
        self.doc = entry.snapshot;
        self.after_time_travel();
        true
    }

    /// Undo back to just before history entry `index` (0 = oldest).
    pub fn revert_to(&mut self, index: usize) {
        while self.history.len() > index {
            if !self.undo() {
                break;
            }
        }
    }

    fn after_time_travel(&mut self) {
        self.refresh_dirty();
        self.analysis_dirty = true;
        self.history_highlight = None;
        let l = self.doc.sequence.len();
        self.selection = self.selection.map(|s| s.normalized(l.max(1)));
        if l == 0 {
            self.selection = None;
        }
        self.zoom_width = self.zoom_width.min(l.max(1));
        self.zoom_start = if l > 0 { self.zoom_start % l } else { 0 };
        if self.search.open && !self.search.query.is_empty() {
            self.run_search();
        }
    }

    pub fn toast(&mut self, msg: impl Into<String>) {
        self.toast = Some((msg.into(), std::time::Instant::now()));
    }

    pub fn set_document(&mut self, doc: Document, path: Option<PathBuf>) {
        self.doc = doc;
        self.path = path;
        self.dirty = false;
        self.saved_depth = Some(0);
        self.history.clear();
        self.future.clear();
        self.history_highlight = None;
        self.selection = None;
        self.selected_feature = None;
        self.selected_primer = None;
        self.zoom_start = 0;
        self.zoom_width = self.doc.sequence.len().max(1);
        self.search = SearchState {
            open: self.search.open,
            recents: self.search.recents.clone(),
            ..Default::default()
        };
        self.analysis_dirty = true;
        self.dialog = None;
        if let Some(p) = &self.path {
            self.config.add_recent(p);
        }
    }

    pub fn select(&mut self, range: Option<Range>) {
        self.selection = range.map(|r| r.normalized(self.doc.sequence.len().max(1)));
    }

    pub fn select_feature(&mut self, id: u64) {
        self.selected_feature = Some(id);
        self.selected_primer = None;
        if let Some(f) = self.doc.feature(id) {
            let span = f.span(self.doc.sequence.len());
            self.select(Some(span));
        }
    }

    pub fn center_zoom_on_selection(&mut self) {
        if let Some(sel) = self.selection {
            let l = self.doc.sequence.len();
            if l == 0 {
                return;
            }
            let w = self.zoom_width.min(l);
            let center = sel.start + sel.len / 2;
            self.zoom_start = (center + l - w / 2) % l;
        }
    }

    // ------------------------------------------------------------------
    // Search
    // ------------------------------------------------------------------

    pub fn run_search(&mut self) {
        let q = self.search.query.trim().to_string();
        self.search.dna_hits.clear();
        self.search.protein_hits.clear();
        self.search.name_hits.clear();
        self.search.error = None;
        self.search.current = 0;
        if q.is_empty() {
            return;
        }
        match self.search.mode {
            SearchMode::Dna => match domain::search::find_dna(&self.doc.sequence, &q) {
                Ok(hits) => self.search.dna_hits = hits,
                Err(e) => self.search.error = Some(e),
            },
            SearchMode::Protein => {
                let code = domain::genetic_code::code_by_id(self.doc.default_genetic_code);
                match domain::search::find_protein(&self.doc.sequence, &q, code) {
                    Ok(hits) => self.search.protein_hits = hits,
                    Err(e) => self.search.error = Some(e),
                }
            }
            SearchMode::Name => {
                self.search.name_hits = self.find_names(&q);
            }
        }
        if !self
            .search
            .recents
            .iter()
            .any(|(m, s)| *m == self.search.mode && s == &q)
        {
            self.search.recents.insert(0, (self.search.mode, q));
            self.search.recents.truncate(12);
        }
    }

    fn find_names(&self, q: &str) -> Vec<NameHit> {
        let mut hits = Vec::new();
        let matches = |name: &str| -> bool {
            if q.contains('*') {
                let parts: Vec<&str> = q.split('*').filter(|p| !p.is_empty()).collect();
                let lower = name.to_ascii_lowercase();
                let mut pos = 0;
                let anchored_start = !q.starts_with('*');
                let anchored_end = !q.ends_with('*');
                for (i, part) in parts.iter().enumerate() {
                    let p = part.to_ascii_lowercase();
                    let found = if i == 0 && anchored_start {
                        if lower.starts_with(&p) {
                            Some(0)
                        } else {
                            None
                        }
                    } else {
                        lower[pos..].find(&p).map(|x| x + pos)
                    };
                    match found {
                        Some(at) => pos = at + p.len(),
                        None => return false,
                    }
                }
                if anchored_end {
                    if let Some(last) = parts.last() {
                        return lower.ends_with(&last.to_ascii_lowercase());
                    }
                }
                true
            } else {
                name.to_ascii_lowercase().contains(&q.to_ascii_lowercase())
            }
        };
        for f in &self.doc.features {
            if matches(&f.name) {
                hits.push(NameHit::Feature(f.id));
            }
        }
        for p in &self.doc.primers {
            if matches(&p.name) {
                hits.push(NameHit::Primer(p.id));
            }
        }
        for e in domain::enzyme_db::all_enzymes() {
            if matches(&e.name) && self.analysis.counts.get(&e.name).copied().unwrap_or(0) > 0 {
                hits.push(NameHit::Enzyme(e.name.clone()));
            }
        }
        hits
    }

    pub fn search_next(&mut self, forward: bool) {
        let n = self.search.hit_count();
        if n == 0 {
            return;
        }
        self.search.current = if forward {
            (self.search.current + 1) % n
        } else {
            (self.search.current + n - 1) % n
        };
        self.goto_current_hit();
    }

    pub fn goto_current_hit(&mut self) {
        let l = self.doc.sequence.len();
        // Resolve an owned selection target first so no immutable borrow of
        // `self` is still live when we mutate it below.
        let mut feature_target: Option<u64> = None;
        let mut primer_target: Option<u64> = None;
        let target: Option<(Range, usize)> = match self.search.mode {
            SearchMode::Dna => self
                .search
                .dna_hits
                .get(self.search.current)
                .map(|h| (Range::new(h.start, h.len), h.start)),
            SearchMode::Protein => self
                .search
                .protein_hits
                .get(self.search.current)
                .map(|h| (Range::new(h.start, h.len), h.start)),
            SearchMode::Name => match self.search.name_hits.get(self.search.current).cloned() {
                Some(NameHit::Feature(id)) => {
                    feature_target = Some(id);
                    self.doc.feature(id).map(|f| {
                        let span = f.span(self.doc.sequence.len());
                        let start = span.start;
                        (span, start)
                    })
                }
                Some(NameHit::Primer(id)) => {
                    primer_target = Some(id);
                    self.analysis
                        .primer_sites
                        .get(&id)
                        .and_then(|sites| sites.first())
                        .map(|s| (Range::new(s.start, s.len), s.start))
                }
                Some(NameHit::Enzyme(name)) => {
                    let chosen = self
                        .analysis
                        .chosen
                        .iter()
                        .find(|(e, _)| e.name == name)
                        .and_then(|(_, sites)| sites.first())
                        .map(|s| s.position);
                    let pos = match chosen {
                        Some(p) => Some(p),
                        None => domain::enzyme_db::enzyme_by_name(&name).and_then(|e| {
                            e.find_sites(&self.doc.sequence).first().map(|s| s.position)
                        }),
                    };
                    pos.map(|p| (Range::new(p, 1), p))
                }
                None => None,
            },
        };
        if let Some(id) = feature_target {
            self.select_feature(id);
        }
        if let Some(id) = primer_target {
            self.selected_primer = Some(id);
        }
        if let Some((range, scroll)) = target {
            self.select(Some(range));
            self.scroll_to = Some(scroll);
        }
        if self.prefs.show_zoom_controls && l > 0 {
            self.center_zoom_on_selection();
        }
    }

    // ------------------------------------------------------------------
    // History colors
    // ------------------------------------------------------------------

    pub fn history_regions(&self) -> Vec<ColorRegion> {
        match self.colors_mode {
            ColorsMode::History => self
                .history_highlight
                .and_then(|i| self.history.get(i))
                .map(|e| e.regions.clone())
                .unwrap_or_default(),
            ColorsMode::Custom => self.doc.color_regions.clone(),
            ColorsMode::None => Vec::new(),
        }
    }

    /// Base RGB for a color region in the current mode; shells apply their own alpha.
    pub fn region_rgb(&self, r: &ColorRegion) -> [u8; 3] {
        match self.colors_mode {
            ColorsMode::History => [0x6b, 0x21, 0x14],
            _ => r.color.map(|c| (c as f32 * 0.55) as u8),
        }
    }

    pub fn region_alpha(&self) -> u8 {
        match self.colors_mode {
            ColorsMode::History => 0xc8,
            _ => 0xff,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn app() -> AppState {
        AppState::new(domain::demo::build_demo(), Config::default())
    }

    fn edit(a: &mut AppState, tag: char) {
        a.mutate("edit", Vec::new(), |doc| doc.name.push(tag));
    }

    #[test]
    fn undo_back_to_opened_state_is_clean() {
        let mut a = app();
        assert!(!a.dirty);
        edit(&mut a, 'x');
        assert!(a.dirty);
        assert!(a.undo());
        assert!(!a.dirty);
        assert!(a.redo());
        assert!(a.dirty);
    }

    #[test]
    fn undo_back_to_saved_depth_is_clean() {
        let mut a = app();
        edit(&mut a, 'x');
        edit(&mut a, 'y');
        a.saved_depth = Some(a.history.len());
        a.dirty = false;
        assert!(a.undo());
        assert!(a.dirty);
        assert!(a.undo());
        assert!(a.dirty);
        assert!(a.redo());
        assert!(a.redo());
        assert!(!a.dirty);
    }

    #[test]
    fn branching_below_save_never_reports_clean() {
        let mut a = app();
        edit(&mut a, 'x');
        edit(&mut a, 'y');
        a.saved_depth = Some(a.history.len());
        a.dirty = false;
        assert!(a.undo());
        assert!(a.undo());
        edit(&mut a, 'z');
        edit(&mut a, 'w');
        assert!(
            a.dirty,
            "diverged branch must stay dirty at old saved depth"
        );
        assert!(a.undo());
        assert!(a.undo());
        assert!(a.dirty, "saved state is unreachable after branching");
    }

    #[test]
    fn set_document_resets_clean() {
        let mut a = app();
        edit(&mut a, 'x');
        a.set_document(domain::demo::build_demo(), None);
        assert!(!a.dirty);
        assert!(a.history.is_empty());
        assert!(!a.undo());
    }

    #[test]
    fn import_auto_detects_name_and_reports_row_issues() {
        let csv = "Name,Organism\np1,E. coli\n,E. coli\np2\n";
        let (rows, errs) = crate::actions::parse_registry_rows(csv, None).unwrap();
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].get("Name").unwrap(), "p1");
        assert_eq!(errs.len(), 2); // empty name skipped + short row padded
        assert!(errs[0].contains("row 3"));
        assert!(errs[1].contains("row 4"));
    }

    #[test]
    fn import_maps_chosen_column_to_name() {
        let csv = "Id,Label\n1,alpha\n2,beta\n";
        let (rows, errs) = crate::actions::parse_registry_rows(csv, Some(1)).unwrap();
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].get("Name").unwrap(), "alpha");
        assert!(!rows[0].contains_key("Label"));
        assert!(errs.is_empty());
        assert!(crate::actions::parse_registry_rows(csv, Some(9)).is_err());
        assert_eq!(crate::actions::csv_columns(csv), vec!["Id", "Label"]);
    }
}
