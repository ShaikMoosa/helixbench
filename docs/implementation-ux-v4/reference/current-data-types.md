# Current-code data and draft reference

These declarations are captured from the inspected repository. Proposed changes are in chapter 03 and are not silently substituted below. The complete structured source index is reference/source-index.json.

## apps/desktop_gpui/src/app.rs · MenuKind

[apps/desktop_gpui/src/app.rs:56](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/apps/desktop_gpui/src/app.rs:56)

```rust
pub enum MenuKind {
    File,
    Edit,
    View,
    Features,
    Primers,
    Enzymes,
    Tools,
    Lab,
    Help,
}
```

## apps/desktop_gpui/src/app.rs · HelixBench

[apps/desktop_gpui/src/app.rs:68](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/apps/desktop_gpui/src/app.rs:68)

```rust
pub struct HelixBench {
    pub app: AppState,
    pub focus_handle: FocusHandle,
    initialized: bool,
    last_title: String,
    toast_armed: Option<std::time::Instant>,

    // Map interaction
    pub map_bounds: Option<Bounds<Pixels>>,
    pub map_press: Option<Point<Pixels>>,
    pub map_dragged: bool,

    // Sequence interaction (manual scroll)
    pub seq_scroll: f32,
    pub seq_view_h: f32,
    pub seq_total_h: f32,
    pub seq_press: Option<Point<Pixels>>,
    pub seq_dragged: bool,
    pub seq_layout: Option<crate::views::sequence::SeqLayout>,
    pub seq_mini_bounds: Option<Bounds<Pixels>>,

    // Light editor split interaction
    pub editor_scroll: f32,
    pub editor_seq_bounds: Option<Bounds<Pixels>>,
    pub editor_seq_anchor: Option<usize>,
    pub editor_slider_bounds: Option<Bounds<Pixels>>,

    // Menus
    pub menu: Option<MenuKind>,
    pub menu_anchor: Point<Pixels>,

    // Enzyme view dropdown popups
    pub combo: Option<crate::views::enzymes::ComboKind>,
    pub combo_anchor: Point<Pixels>,
    /// Which list table's column/view options popup is open (table id).
    pub opts: Option<String>,
    pub opts_anchor: Point<Pixels>,
    /// Home calendar displayed month + selected day.
    pub cal_year: i64,
    pub cal_month: u32,
    pub cal_day: Option<u32>,

    // Find bar recents popup
    pub find_recents_open: bool,
    pub find_recents_anchor: Point<Pixels>,

    // Dialog dropdown popups (kind / genetic code selectors)
    pub dlg_combo: Option<crate::dialogs::DlgComboKind>,
    pub dlg_combo_anchor: Point<Pixels>,

    // Text fields (keyed, persistent entities)
    pub inputs: HashMap<String, Entity<TextInput>>,
    pub last_dialog_sig: String,

    // Dialog chrome: window size for the scrim, scroll handle for the body
    pub viewport: Size<Pixels>,
    pub dlg_scroll: ScrollHandle,

    // Table scroll handles
    pub feat_scroll: UniformListScrollHandle,
    pub primer_scroll: UniformListScrollHandle,
    pub enzyme_scroll: UniformListScrollHandle,
    pub history_scroll: UniformListScrollHandle,
    pub detect_scroll: UniformListScrollHandle,
}
```

## apps/desktop_gpui/src/dialogs.rs · DlgComboKind

[apps/desktop_gpui/src/dialogs.rs:125](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/apps/desktop_gpui/src/dialogs.rs:125)

```rust
pub enum DlgComboKind {
    FeatureKind,
    FeatureCode,
    OptionsCode,
    CodesTable,
}
```

## apps/desktop_gpui/src/dialogs.rs · ColorTarget

[apps/desktop_gpui/src/dialogs.rs:523](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/apps/desktop_gpui/src/dialogs.rs:523)

```rust
enum ColorTarget {
    Feature,
    Primer,
    ColorSel,
}
```

## apps/desktop_gpui/src/dialogs.rs · DetectRow

[apps/desktop_gpui/src/dialogs.rs:1908](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/apps/desktop_gpui/src/dialogs.rs:1908)

```rust
struct DetectRow {
    name: String,
    range: String,
    strand: String,
    dup: bool,
    checked: bool,
}
```

## apps/desktop_gpui/src/draw.rs · Align

[apps/desktop_gpui/src/draw.rs:23](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/apps/desktop_gpui/src/draw.rs:23)

```rust
pub enum Align {
    Left,
    Center,
    Right,
}
```

## apps/desktop_gpui/src/editor.rs · SeqDraw

[apps/desktop_gpui/src/editor.rs:35](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/apps/desktop_gpui/src/editor.rs:35)

```rust
struct SeqDraw {
    seq: Vec<u8>,
    features: Vec<(String, [u8; 3], usize, usize)>,
    selection: Option<Range>,
    scroll: f32,
}
```

## apps/desktop_gpui/src/editor.rs · MiniDraw

[apps/desktop_gpui/src/editor.rs:192](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/apps/desktop_gpui/src/editor.rs:192)

```rust
struct MiniDraw {
    l: usize,
    features: Vec<(usize, usize, [u8; 3])>,
    zoom_start: usize,
    zoom_width: usize,
    selection: Option<Range>,
}
```

## apps/desktop_gpui/src/icons.rs · Assets

[apps/desktop_gpui/src/icons.rs:63](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/apps/desktop_gpui/src/icons.rs:63)

```rust
pub struct Assets;

impl AssetSource for Assets {
    fn load(&self, path: &str) -> gpui::Result<Option<Cow<'static, [u8]>>> {
        if path == "logo.png" {
            return Ok(Some(Cow::Borrowed(include_bytes!(
                "../assets/helixbench.png"
            ))));
        }
        if path == "logo.svg" {
            return Ok(Some(Cow::Borrowed(include_bytes!(
                "../assets/helixbench.svg"
            ))));
        }
        let name = path
            .strip_prefix("icons/")
            .and_then(|p| p.strip_suffix(".svg"));
        if name == Some("logo") {
            return Ok(Some(Cow::Borrowed(include_bytes!(
                "../assets/helixbench.svg"
            ))));
        }
        Ok(name.and_then(body).map(|b| {
            let mut s = String::with_capacity(HEAD.len() + b.len() + TAIL.len());
            s.push_str(HEAD);
            s.push_str(b);
            s.push_str(TAIL);
            Cow::Owned(s.into_bytes())
        }))
    }

    fn list(&self, path: &str) -> gpui::Result<Vec<SharedString>> {
        if path == "icons" {
            Ok(names())
        } else {
            Ok(vec![])
        }
    }
}
```

## apps/desktop_gpui/src/text_input.rs · TextInput

[apps/desktop_gpui/src/text_input.rs:52](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/apps/desktop_gpui/src/text_input.rs:52)

```rust
pub struct TextInput {
    focus_handle: FocusHandle,
    content: SharedString,
    placeholder: SharedString,
    selected_range: Range<usize>,
    selection_reversed: bool,
    marked_range: Option<Range<usize>>,
    last_layout: Option<ShapedLine>,
    last_bounds: Option<Bounds<Pixels>>,
    is_selecting: bool,
    scroll_x: Pixels,
    mono: bool,
    font_size: f32,
}
```

## apps/desktop_gpui/src/text_input.rs · TextElement

[apps/desktop_gpui/src/text_input.rs:422](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/apps/desktop_gpui/src/text_input.rs:422)

```rust
struct TextElement {
    input: Entity<TextInput>,
}
```

## apps/desktop_gpui/src/text_input.rs · PrepaintState

[apps/desktop_gpui/src/text_input.rs:426](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/apps/desktop_gpui/src/text_input.rs:426)

```rust
struct PrepaintState {
    line: Option<ShapedLine>,
    cursor: Option<PaintQuad>,
    selection: Option<PaintQuad>,
    scroll_x: Pixels,
}
```

## apps/desktop_gpui/src/views/enzymes.rs · ComboKind

[apps/desktop_gpui/src/views/enzymes.rs:34](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/apps/desktop_gpui/src/views/enzymes.rs:34)

```rust
pub enum ComboKind {
    Set,
    CutCount,
    Overhang,
    SiteLen,
    Region,
}
```

## apps/desktop_gpui/src/views/enzymes.rs · Row

[apps/desktop_gpui/src/views/enzymes.rs:42](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/apps/desktop_gpui/src/views/enzymes.rs:42)

```rust
struct Row {
    name: String,
    site: String,
    n: usize,
    overhang: String,
    positions: String,
    in_set: bool,
    first_site: Option<usize>,
}
```

## apps/desktop_gpui/src/views/features.rs · Row

[apps/desktop_gpui/src/views/features.rs:32](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/apps/desktop_gpui/src/views/features.rs:32)

```rust
struct Row {
    id: u64,
    name: String,
    kind: String,
    dir: &'static str,
    total: usize,
    loc: String,
    trans: u8, // 0 = no, 1 = yes, 2 = yes with internal stop
    product: String,
    color: gpui::Hsla,
    visible: bool,
    selected: bool,
}
```

## apps/desktop_gpui/src/views/history.rs · HRow

[apps/desktop_gpui/src/views/history.rs:18](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/apps/desktop_gpui/src/views/history.rs:18)

```rust
struct HRow {
    orig: Option<usize>,
    number: usize,
    label: String,
    regions: String,
    selected: bool,
}
```

## apps/desktop_gpui/src/views/map.rs · Geom

[apps/desktop_gpui/src/views/map.rs:20](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/apps/desktop_gpui/src/views/map.rs:20)

```rust
pub struct Geom {
    pub center: Point<Pixels>,
    pub r: f32,
    pub bounds: Bounds<Pixels>,
    pub circular: bool,
}
```

## apps/desktop_gpui/src/views/map.rs · FeatDraw

[apps/desktop_gpui/src/views/map.rs:128](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/apps/desktop_gpui/src/views/map.rs:128)

```rust
struct FeatDraw {
    name: String,
    color: Hsla,
    dir: Direction,
    span: Range,
    multi: bool,
    selected: bool,
}
```

## apps/desktop_gpui/src/views/map.rs · PrimerDraw

[apps/desktop_gpui/src/views/map.rs:137](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/apps/desktop_gpui/src/views/map.rs:137)

```rust
struct PrimerDraw {
    name: String,
    color: Hsla,
    seq_len: usize,
    sites: Vec<(usize, usize, bool)>, // start, len, top
}
```

## apps/desktop_gpui/src/views/map.rs · EnzDraw

[apps/desktop_gpui/src/views/map.rs:144](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/apps/desktop_gpui/src/views/map.rs:144)

```rust
struct EnzDraw {
    name: String,
    position: usize,
}
```

## apps/desktop_gpui/src/views/map.rs · MapDraw

[apps/desktop_gpui/src/views/map.rs:149](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/apps/desktop_gpui/src/views/map.rs:149)

```rust
pub struct MapDraw {
    l: usize,
    circular: bool,
    name: String,
    gc: f32,
    selection: Option<Range>,
    show_orfs: bool,
    show_enzymes: bool,
    show_primers: bool,
    show_zoom: bool,
    zoom_start: usize,
    zoom_width: usize,
    orfs: Vec<(usize, usize, bool)>,
    enzymes: Vec<EnzDraw>,
    primers: Vec<PrimerDraw>,
    features: Vec<FeatDraw>,
    regions: Vec<(Range, Hsla)>,
    hits: Vec<Range>,
    hit_current: usize,
    has_hits: bool,
}
```

## apps/desktop_gpui/src/views/map.rs · Hit

[apps/desktop_gpui/src/views/map.rs:990](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/apps/desktop_gpui/src/views/map.rs:990)

```rust
pub enum Hit {
    Feature(u64),
    Primer(u64),
    Enzyme(usize),
}
```

## apps/desktop_gpui/src/views/mod.rs · ScrollbarDecoration

[apps/desktop_gpui/src/views/mod.rs:26](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/apps/desktop_gpui/src/views/mod.rs:26)

```rust
pub struct ScrollbarDecoration;

impl UniformListDecoration for ScrollbarDecoration {
    fn compute(
        &self,
        _visible_range: Range<usize>,
        bounds: Bounds<Pixels>,
        scroll_offset: gpui::Point<Pixels>,
        item_height: Pixels,
        item_count: usize,
        _window: &mut Window,
        _cx: &mut App,
    ) -> AnyElement {
        let viewport = bounds.size.height.v();
        let content = item_height.v() * item_count as f32;
        let offset = -scroll_offset.y.v();
        canvas(
            move |_b, _w, _cx| {},
            move |b, _s, window, _cx| {
                let rail_w = px(8.0);
                let track = Bounds::new(
                    point(b.right() - rail_w - px(2.0), b.top() + px(2.0)),
                    size(rail_w, (b.size.height - px(4.0)).max(px(0.0))),
                );
                draw::vscrollbar(window, track, offset, viewport, content);
            },
        )
        .size_full()
        .into_any_element()
    }
}
```

## apps/desktop_gpui/src/views/primers.rs · Row

[apps/desktop_gpui/src/views/primers.rs:32](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/apps/desktop_gpui/src/views/primers.rs:32)

```rust
struct Row {
    id: u64,
    name: String,
    seq: String,
    len: usize,
    gc: f32,
    tm: Option<f32>,
    sites: usize,
    description: String,
    color: Hsla,
    visible: bool,
    phospho: bool,
    selected: bool,
}
```

## apps/desktop_gpui/src/views/sequence.rs · LaneAssign

[apps/desktop_gpui/src/views/sequence.rs:43](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/apps/desktop_gpui/src/views/sequence.rs:43)

```rust
struct LaneAssign {
    feat_lanes: Vec<(u64, usize)>,
    orf_top: Vec<(usize, usize)>,
    orf_bot: Vec<(usize, usize)>,
    primer_lanes: Vec<(u64, usize)>,
    translated: Vec<u64>,
}
```

## apps/desktop_gpui/src/views/sequence.rs · PrimerSeqDraw

[apps/desktop_gpui/src/views/sequence.rs:182](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/apps/desktop_gpui/src/views/sequence.rs:182)

```rust
struct PrimerSeqDraw {
    id: u64,
    name: String,
    color: Hsla,
    seq_len: usize,
    sites: Vec<(usize, usize, bool)>, // start, len, top strand
}
```

## apps/desktop_gpui/src/views/sequence.rs · RowLayout

[apps/desktop_gpui/src/views/sequence.rs:194](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/apps/desktop_gpui/src/views/sequence.rs:194)

```rust
struct RowLayout {
    y: f32,
    h: f32,
    top_h: f32,
    mid_h: f32,
    feat: Vec<(u64, usize)>,
    orf_top: Vec<(usize, usize)>,
    orf_bot: Vec<(usize, usize)>,
    primers: Vec<(u64, usize)>,
    trans: Vec<usize>,
    enz: bool,
}
```

## apps/desktop_gpui/src/views/sequence.rs · SeqDraw

[apps/desktop_gpui/src/views/sequence.rs:228](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/apps/desktop_gpui/src/views/sequence.rs:228)

```rust
pub struct SeqDraw {
    l: usize,
    circular: bool,
    seq: domain::DnaSequence,
    positions: Vec<usize>,
    disp_len: usize,
    n_rows: usize,
    features: Vec<Feature>,
    orfs: Vec<Orf>,
    primers: Vec<PrimerSeqDraw>,
    translations: Vec<(u64, Vec<u8>, Vec<usize>)>,
    regions: Vec<(Range, Hsla)>,
    dna_hits: Vec<Range>,
    prot_hits: Vec<Range>,
    hit_current: usize,
    sel: Option<Range>,
    selected_feature: Option<u64>,
    enz_marks: Vec<(usize, usize, String, usize)>, // pos, len, name, stagger
    show_complement: bool,
    show_features: bool,
    show_orfs: bool,
    show_primers: bool,
    colorize: bool,
    rows: Vec<RowLayout>,
    total_h: f32,
}
```

## apps/desktop_gpui/src/views/sequence.rs · SeqLayout

[apps/desktop_gpui/src/views/sequence.rs:477](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/apps/desktop_gpui/src/views/sequence.rs:477)

```rust
pub struct SeqLayout {
    pub bounds: Bounds<Pixels>,
    pub scroll: f32,
    rows: Vec<RowLayout>,
    disp_len: usize,
    positions: Vec<usize>,
}
```

## apps/desktop_gpui/src/views/sequence.rs · MiniDraw

[apps/desktop_gpui/src/views/sequence.rs:989](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/apps/desktop_gpui/src/views/sequence.rs:989)

```rust
struct MiniDraw {
    l: usize,
    circular: bool,
    feats: Vec<(usize, usize, Hsla)>,
    enzymes: Vec<usize>,
    sel: Option<Range>,
    zoom_start: usize,
    zoom_width: usize,
}
```

## apps/desktop_gpui/src/widgets.rs · Btn

[apps/desktop_gpui/src/widgets.rs:9](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/apps/desktop_gpui/src/widgets.rs:9)

```rust
pub enum Btn {
    Primary,
    Outline,
    Ghost,
    Danger,
    Subtle,
}
```

## crates/appcore/src/state.rs · View

[crates/appcore/src/state.rs:14](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/appcore/src/state.rs:14)

```rust
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
    Rna,
    InVivo,
    Bioprocess,
}
```

## crates/appcore/src/state.rs · ColorsMode

[crates/appcore/src/state.rs:112](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/appcore/src/state.rs:112)

```rust
pub enum ColorsMode {
    None,
    History,
    Custom,
}
```

## crates/appcore/src/state.rs · HistoryEntry

[crates/appcore/src/state.rs:131](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/appcore/src/state.rs:131)

```rust
pub struct HistoryEntry {
    pub label: String,
    pub snapshot: Document,
    pub regions: Vec<ColorRegion>,
}
```

## crates/appcore/src/state.rs · SearchMode

[crates/appcore/src/state.rs:138](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/appcore/src/state.rs:138)

```rust
pub enum SearchMode {
    #[default]
    Dna,
    Protein,
    Name,
}
```

## crates/appcore/src/state.rs · NameHit

[crates/appcore/src/state.rs:156](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/appcore/src/state.rs:156)

```rust
pub enum NameHit {
    Feature(u64),
    Primer(u64),
    Enzyme(String),
}
```

## crates/appcore/src/state.rs · SearchState

[crates/appcore/src/state.rs:163](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/appcore/src/state.rs:163)

```rust
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
```

## crates/appcore/src/state.rs · SortDir

[crates/appcore/src/state.rs:187](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/appcore/src/state.rs:187)

```rust
pub enum SortDir {
    Asc,
    Desc,
}
```

## crates/appcore/src/state.rs · TableSort

[crates/appcore/src/state.rs:193](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/appcore/src/state.rs:193)

```rust
pub struct TableSort {
    pub column: usize,
    pub dir: Option<SortDir>,
}
```

## crates/appcore/src/state.rs · FeatureDraft

[crates/appcore/src/state.rs:199](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/appcore/src/state.rs:199)

```rust
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
```

## crates/appcore/src/state.rs · PrimerDraft

[crates/appcore/src/state.rs:216](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/appcore/src/state.rs:216)

```rust
pub struct PrimerDraft {
    pub id: Option<u64>,
    pub name: String,
    pub sequence: String,
    pub description: String,
    pub color: [u8; 3],
    pub phosphorylated: bool,
    pub error: Option<String>,
}
```

## crates/appcore/src/state.rs · NewSeqState

[crates/appcore/src/state.rs:226](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/appcore/src/state.rs:226)

```rust
pub struct NewSeqState {
    pub name: String,
    pub text: String,
    pub topology: Topology,
    pub error: Option<String>,
}
```

## crates/appcore/src/state.rs · PendingAction

[crates/appcore/src/state.rs:245](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/appcore/src/state.rs:245)

```rust
pub enum PendingAction {
    CloseApp,
    NewDemo,
    NewEmpty,
    Open,
    OpenRecent(PathBuf),
}
```

## crates/appcore/src/state.rs · Dialog

[crates/appcore/src/state.rs:253](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/appcore/src/state.rs:253)

```rust
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
    Rna(RnaDraft),
    IvStudy(IvStudyDraft),
    IvAnimal(IvAnimalDraft),
    IvMeasure(IvMeasureDraft),
    BpRecipe(BpRecipeDraft),
    BpReplicate(BpReplicateDraft),
}
```

## crates/appcore/src/state.rs · ColorDraft

[crates/appcore/src/state.rs:312](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/appcore/src/state.rs:312)

```rust
pub struct ColorDraft {
    pub range: Range,
    pub color: [u8; 3],
    pub strands: StrandMask,
}
```

## crates/appcore/src/state.rs · OpenDoc

[crates/appcore/src/state.rs:320](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/appcore/src/state.rs:320)

```rust
pub struct OpenDoc {
    pub doc: Document,
    pub path: Option<PathBuf>,
    pub dirty: bool,
}
```

## crates/appcore/src/state.rs · PcrDraft

[crates/appcore/src/state.rs:327](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/appcore/src/state.rs:327)

```rust
pub struct PcrDraft {
    pub fwd: String,
    pub rev: String,
    pub copy_features: bool,
    pub error: Option<String>,
}
```

## crates/appcore/src/state.rs · PrimerDesignDraft

[crates/appcore/src/state.rs:335](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/appcore/src/state.rs:335)

```rust
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
```

## crates/appcore/src/state.rs · AlignmentDraft

[crates/appcore/src/state.rs:348](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/appcore/src/state.rs:348)

```rust
pub struct AlignmentDraft {
    /// Index into `AppState::library` used as the reference.
    pub reference: usize,
    pub reindex: bool,
    pub result: Option<domain::align::Alignment>,
    /// Row labels for the current result (saved alignments carry their own).
    pub labels: Vec<String>,
    pub error: Option<String>,
}
```

## crates/appcore/src/state.rs · AssemblyDraft

[crates/appcore/src/state.rs:359](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/appcore/src/state.rs:359)

```rust
pub struct AssemblyDraft {
    pub strategy: usize,
    pub cut: String,
    pub inserts: Vec<bool>,
    pub primers: Vec<domain::assembly::JunctionPrimers>,
    pub gg_issues: Vec<String>,
    pub error: Option<String>,
}
```

## crates/appcore/src/state.rs · CrisprDraft

[crates/appcore/src/state.rs:369](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/appcore/src/state.rs:369)

```rust
pub struct CrisprDraft {
    pub limit: String,
    pub guides: Vec<domain::crispr::Guide>,
    pub off: Vec<domain::crispr::OffTarget>,
    pub error: Option<String>,
}
```

## crates/appcore/src/state.rs · CodonDraft

[crates/appcore/src/state.rs:377](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/appcore/src/state.rs:377)

```rust
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
```

## crates/appcore/src/state.rs · BlastDraft

[crates/appcore/src/state.rs:390](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/appcore/src/state.rs:390)

```rust
pub struct BlastDraft {
    pub k: String,
    pub max: String,
    pub db_count: usize,
    pub hits: Vec<domain::blast::Hit>,
    pub error: Option<String>,
}
```

## crates/appcore/src/state.rs · ProteinDraft

[crates/appcore/src/state.rs:399](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/appcore/src/state.rs:399)

```rust
pub struct ProteinDraft {
    pub stats: Option<domain::protein::ProteinStats>,
    pub align: Option<(Vec<u8>, Vec<u8>, i32)>,
    pub error: Option<String>,
}
```

## crates/appcore/src/state.rs · FeatureLibDraft

[crates/appcore/src/state.rs:406](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/appcore/src/state.rs:406)

```rust
pub struct FeatureLibDraft {
    pub name: String,
    pub kind: String,
    pub motif: String,
    pub error: Option<String>,
}
```

## crates/appcore/src/state.rs · WorkspaceDraft

[crates/appcore/src/state.rs:414](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/appcore/src/state.rs:414)

```rust
pub struct WorkspaceDraft {
    pub selected: Vec<bool>,
    pub tag: String,
    pub error: Option<String>,
}
```

## crates/appcore/src/state.rs · NotebookDraft

[crates/appcore/src/state.rs:421](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/appcore/src/state.rs:421)

```rust
pub struct NotebookDraft {
    pub filter: usize,
    pub error: Option<String>,
}
```

## crates/appcore/src/state.rs · EntryEditDraft

[crates/appcore/src/state.rs:427](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/appcore/src/state.rs:427)

```rust
pub struct EntryEditDraft {
    pub id: Option<u64>,
    pub title: String,
    pub project: Option<u64>,
    pub template: usize,
    pub sections: Vec<(String, String)>,
    pub tables: Vec<domain::notebook::ResultTable>,
    pub error: Option<String>,
}
```

## crates/appcore/src/state.rs · ProjectsDraft

[crates/appcore/src/state.rs:438](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/appcore/src/state.rs:438)

```rust
pub struct ProjectsDraft {
    pub name: String,
    pub parent: Option<u64>,
    pub error: Option<String>,
}
```

## crates/appcore/src/state.rs · AuditDraft

[crates/appcore/src/state.rs:445](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/appcore/src/state.rs:445)

```rust
pub struct AuditDraft {
    pub filter: String,
}
```

## crates/appcore/src/state.rs · HomeDraft

[crates/appcore/src/state.rs:450](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/appcore/src/state.rs:450)

```rust
pub struct HomeDraft {}
```

## crates/appcore/src/state.rs · EntityMapDraft

[crates/appcore/src/state.rs:453](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/appcore/src/state.rs:453)

```rust
pub struct EntityMapDraft {}
```

## crates/appcore/src/state.rs · BiologicsDraft

[crates/appcore/src/state.rs:458](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/appcore/src/state.rs:458)

```rust
pub struct BiologicsDraft {
    pub name: String,
    pub format: usize,
    pub numbering: usize,
    pub seqs: Vec<String>,
    pub error: Option<String>,
}
```

## crates/appcore/src/state.rs · ChemAddDraft

[crates/appcore/src/state.rs:468](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/appcore/src/state.rs:468)

```rust
pub struct ChemAddDraft {
    pub name: String,
    pub smiles: String,
    pub error: Option<String>,
}
```

## crates/appcore/src/state.rs · ChemReactionDraft

[crates/appcore/src/state.rs:476](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/appcore/src/state.rs:476)

```rust
pub struct ChemReactionDraft {
    pub name: String,
    pub reactant: usize,
    pub product: usize,
    pub req: String,
    pub peq: String,
    pub error: Option<String>,
}
```

## crates/appcore/src/state.rs · ChemLotDraft

[crates/appcore/src/state.rs:487](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/appcore/src/state.rs:487)

```rust
pub struct ChemLotDraft {
    pub form: usize,
    pub quantity: String,
    pub unit: String,
    pub error: Option<String>,
}
```

## crates/appcore/src/state.rs · RnaDraft

[crates/appcore/src/state.rs:497](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/appcore/src/state.rs:497)

```rust
pub struct RnaDraft {
    pub name: String,
    pub seq: String,
    pub pattern: usize,
    pub mod_code: usize,
    pub helm: String,
    pub error: Option<String>,
}
```

## crates/appcore/src/state.rs · IvStudyDraft

[crates/appcore/src/state.rs:508](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/appcore/src/state.rs:508)

```rust
pub struct IvStudyDraft {
    pub name: String,
    pub species: String,
    pub strain: String,
    pub error: Option<String>,
}
```

## crates/appcore/src/state.rs · IvAnimalDraft

[crates/appcore/src/state.rs:517](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/appcore/src/state.rs:517)

```rust
pub struct IvAnimalDraft {
    pub tag: String,
    pub sex: usize, // 0 F, 1 M
    pub weight: String,
    pub error: Option<String>,
}
```

## crates/appcore/src/state.rs · IvMeasureDraft

[crates/appcore/src/state.rs:526](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/appcore/src/state.rs:526)

```rust
pub struct IvMeasureDraft {
    pub animal: usize,
    pub day: String,
    pub weight: String,
    pub tumor: String,
    pub error: Option<String>,
}
```

## crates/appcore/src/state.rs · BpRecipeDraft

[crates/appcore/src/state.rs:536](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/appcore/src/state.rs:536)

```rust
pub struct BpRecipeDraft {
    pub name: String,
    pub product: String,
    pub error: Option<String>,
}
```

## crates/appcore/src/state.rs · BpReplicateDraft

[crates/appcore/src/state.rs:544](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/appcore/src/state.rs:544)

```rust
pub struct BpReplicateDraft {
    pub label: String,
    pub titer: String,
    pub temp: String,
    pub ph: String,
    pub error: Option<String>,
}
```

## crates/appcore/src/state.rs · RegistryImportDraft

[crates/appcore/src/state.rs:553](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/appcore/src/state.rs:553)

```rust
pub struct RegistryImportDraft {
    pub csv: String,
    pub kind: usize,
    /// Column mapped to Name; None = auto-detect a "Name" header (else first).
    pub name_col: Option<usize>,
    pub row_errors: Vec<String>,
    pub error: Option<String>,
    pub imported: usize,
}
```

## crates/appcore/src/state.rs · InventoryDraft

[crates/appcore/src/state.rs:564](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/appcore/src/state.rs:564)

```rust
pub struct InventoryDraft {
    pub name: String,
    pub kind: usize,
    pub parent: Option<u64>,
    pub selected_plate: Option<u64>,
    pub lookup: String,
    pub error: Option<String>,
}
```

## crates/appcore/src/state.rs · WorkflowDraft

[crates/appcore/src/state.rs:574](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/appcore/src/state.rs:574)

```rust
pub struct WorkflowDraft {
    pub title: String,
    pub schema: String,
    pub request_name: String,
    pub output_key: String,
    pub output_val: String,
    pub selected_task: Option<u64>,
    pub error: Option<String>,
}
```

## crates/appcore/src/state.rs · CombDraft

[crates/appcore/src/state.rs:585](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/appcore/src/state.rs:585)

```rust
pub struct CombDraft {
    pub strategy: usize,
    pub cut: String,
    pub cap: String,
    pub results: Vec<domain::assembly::Assembly>,
    pub labels: Vec<String>,
    pub error: Option<String>,
}
```

## crates/appcore/src/state.rs · RegistryDraft

[crates/appcore/src/state.rs:595](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/appcore/src/state.rs:595)

```rust
pub struct RegistryDraft {
    /// 0 = all, 1..=3 = Kind index + 1.
    pub filter: usize,
    pub error: Option<String>,
}
```

## crates/appcore/src/state.rs · RegistryEditDraft

[crates/appcore/src/state.rs:602](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/appcore/src/state.rs:602)

```rust
pub struct RegistryEditDraft {
    pub id: Option<u64>,
    pub kind: usize,
    pub values: Vec<String>,
    pub error: Option<String>,
}
```

## crates/appcore/src/state.rs · EnzymeFilterState

[crates/appcore/src/state.rs:610](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/appcore/src/state.rs:610)

```rust
pub struct EnzymeFilterState {
    pub name: String,
    pub cut_count: usize, // index into CUT_COUNT_OPTIONS
    pub overhang: usize,  // index into OVERHANG_OPTIONS
    pub site_len: usize,  // index into SITE_LEN_OPTIONS
    pub region: usize,    // index into REGION_OPTIONS
    pub region_start: String,
    pub region_end: String,
}
```

## crates/appcore/src/state.rs · MapMode

[crates/appcore/src/state.rs:641](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/appcore/src/state.rs:641)

```rust
pub enum MapMode {
    Auto,
    Plasmid,
    Linear,
    Metadata,
}
```

## crates/appcore/src/state.rs · AppState

[crates/appcore/src/state.rs:648](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/appcore/src/state.rs:648)

```rust
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
    /// Phase 5 RNA / modified oligos.
    pub rna: storage::rna::Rna,
    /// Selected oligo in the RNA view.
    pub rna_sel: Option<u64>,
    /// Phase 5 in-vivo studies.
    pub invivo: storage::invivo::Invivo,
    /// Selected study in the In Vivo view.
    pub iv_sel: Option<u64>,
    /// Dosing calculator inputs (mg/kg, mg/mL).
    pub iv_dose: String,
    pub iv_conc: String,
    /// Phase 5 bioprocess recipes / runs.
    pub bioprocess: storage::bioprocess::Bioprocess,
    /// Selected run in the Bioprocess view.
    pub bp_sel: Option<u64>,
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
```

## crates/appcore/src/state.rs · Analysis

[crates/appcore/src/state.rs:737](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/appcore/src/state.rs:737)

```rust
pub struct Analysis {
    pub chosen: Vec<(Enzyme, Vec<CutSite>)>,
    pub counts: HashMap<String, usize>,
    pub all_sites: HashMap<String, Vec<CutSite>>,
    pub sets: Vec<domain::EnzymeSet>,
    pub orfs: Vec<Orf>,
    pub primer_sites: HashMap<u64, Vec<BindingSite>>,
}
```

## crates/domain/src/align.rs · Alignment

[crates/domain/src/align.rs:147](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/domain/src/align.rs:147)

```rust
pub struct Alignment {
    pub rows: Vec<Vec<u8>>,
    pub consensus: Vec<u8>,
    pub identity: f32,
}
```

## crates/domain/src/align.rs · Map

[crates/domain/src/align.rs:237](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/domain/src/align.rs:237)

```rust
    struct Map {
        cols: Vec<u8>,
        ins: Vec<Vec<u8>>,
    }
```

## crates/domain/src/assembly.rs · Strategy

[crates/domain/src/assembly.rs:7](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/domain/src/assembly.rs:7)

```rust
pub enum Strategy {
    DigestLigate,
    Gibson,
    GoldenGate,
}
```

## crates/domain/src/assembly.rs · Assembly

[crates/domain/src/assembly.rs:24](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/domain/src/assembly.rs:24)

```rust
pub struct Assembly {
    pub bases: Vec<u8>,
    /// Top-strand positions where inserts join the backbone / each other.
    pub junctions: Vec<usize>,
    pub strategy: Strategy,
}
```

## crates/domain/src/assembly.rs · JunctionPrimers

[crates/domain/src/assembly.rs:85](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/domain/src/assembly.rs:85)

```rust
pub struct JunctionPrimers {
    pub name: String,
    pub forward: Vec<u8>,
    pub reverse: Vec<u8>,
}
```

## crates/domain/src/audit.rs · AuditEvent

[crates/domain/src/audit.rs:6](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/domain/src/audit.rs:6)

```rust
pub struct AuditEvent {
    pub ts: u64,
    pub actor: String,
    pub object: String,
    pub action: String,
    pub detail: String,
}
```

## crates/domain/src/audit.rs · AuditLog

[crates/domain/src/audit.rs:15](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/domain/src/audit.rs:15)

```rust
pub struct AuditLog {
    pub events: Vec<AuditEvent>,
}
```

## crates/domain/src/automation.rs · Step

[crates/domain/src/automation.rs:7](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/domain/src/automation.rs:7)

```rust
pub enum Step {
    IngestCsv,
    Filter { col: String, contains: String },
    Count,
    OutputRegistry,
    OutputResults,
}
```

## crates/domain/src/automation.rs · Pipeline

[crates/domain/src/automation.rs:28](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/domain/src/automation.rs:28)

```rust
pub struct Pipeline {
    pub id: u64,
    pub name: String,
    pub steps: Vec<Step>,
    pub published: bool,
}
```

## crates/domain/src/automation.rs · RunStatus

[crates/domain/src/automation.rs:36](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/domain/src/automation.rs:36)

```rust
pub enum RunStatus {
    Completed,
    Failed,
}
```

## crates/domain/src/automation.rs · Run

[crates/domain/src/automation.rs:42](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/domain/src/automation.rs:42)

```rust
pub struct Run {
    pub id: u64,
    pub pipeline: String,
    pub status: RunStatus,
    pub log: Vec<String>,
    pub columns: Vec<String>,
    pub output: Vec<Vec<String>>,
}
```

## crates/domain/src/automation.rs · Automation

[crates/domain/src/automation.rs:52](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/domain/src/automation.rs:52)

```rust
pub struct Automation {
    pub pipelines: Vec<Pipeline>,
    pub runs: Vec<Run>,
    next_id: u64,
}
```

## crates/domain/src/biologics.rs · Format

[crates/domain/src/biologics.rs:12](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/domain/src/biologics.rs:12)

```rust
pub enum Format {
    IgG,
    Bispecific,
    Nanobody,
    Tcr,
    Fab,
    Other,
}
```

## crates/domain/src/biologics.rs · ChainKind

[crates/domain/src/biologics.rs:56](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/domain/src/biologics.rs:56)

```rust
pub enum ChainKind {
    Heavy,
    Light,
    Alpha,
    Beta,
    Single,
    Other,
}
```

## crates/domain/src/biologics.rs · Chain

[crates/domain/src/biologics.rs:88](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/domain/src/biologics.rs:88)

```rust
pub struct Chain {
    pub name: String,
    pub kind: ChainKind,
    /// Amino-acid sequence (one-letter).
    pub sequence: String,
}
```

## crates/domain/src/biologics.rs · Domain

[crates/domain/src/biologics.rs:97](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/domain/src/biologics.rs:97)

```rust
pub struct Domain {
    pub chain: usize,
    pub name: String,
    pub kind: String,
    pub start: usize,
    pub end: usize,
}
```

## crates/domain/src/biologics.rs · Numbering

[crates/domain/src/biologics.rs:106](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/domain/src/biologics.rs:106)

```rust
pub enum Numbering {
    Sequential,
    Kabat,
    Imgt,
}
```

## crates/domain/src/biologics.rs · Biologic

[crates/domain/src/biologics.rs:125](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/domain/src/biologics.rs:125)

```rust
pub struct Biologic {
    pub id: u64,
    pub name: String,
    pub format: Format,
    pub chains: Vec<Chain>,
    pub domains: Vec<Domain>,
    pub numbering: Numbering,
    pub created: u64,
    pub updated: u64,
}
```

## crates/domain/src/biologics.rs · NumberedResidue

[crates/domain/src/biologics.rs:210](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/domain/src/biologics.rs:210)

```rust
pub struct NumberedResidue {
    pub label: String,
    pub region: &'static str,
    pub aa: char,
}
```

## crates/domain/src/bioprocess.rs · OpKind

[crates/domain/src/bioprocess.rs:9](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/domain/src/bioprocess.rs:9)

```rust
pub enum OpKind {
    Inoculate,
    Feed,
    Induce,
    Harvest,
    Clarify,
    Purify,
}
```

## crates/domain/src/bioprocess.rs · UnitOp

[crates/domain/src/bioprocess.rs:41](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/domain/src/bioprocess.rs:41)

```rust
pub struct UnitOp {
    pub name: String,
    pub kind: OpKind,
    pub duration_h: f32,
    pub params: BTreeMap<String, String>,
    pub done: bool,
}
```

## crates/domain/src/bioprocess.rs · Recipe

[crates/domain/src/bioprocess.rs:50](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/domain/src/bioprocess.rs:50)

```rust
pub struct Recipe {
    pub id: u64,
    pub name: String,
    pub product: String,
    pub ops: Vec<UnitOp>,
}
```

## crates/domain/src/bioprocess.rs · RunStatus

[crates/domain/src/bioprocess.rs:88](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/domain/src/bioprocess.rs:88)

```rust
pub enum RunStatus {
    Draft,
    Executing,
    Completed,
    Aborted,
}
```

## crates/domain/src/bioprocess.rs · Replicate

[crates/domain/src/bioprocess.rs:107](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/domain/src/bioprocess.rs:107)

```rust
pub struct Replicate {
    pub id: u64,
    pub label: String,
    pub conditions: BTreeMap<String, String>,
    pub titer_mg_l: f32,
}
```

## crates/domain/src/bioprocess.rs · Run

[crates/domain/src/bioprocess.rs:115](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/domain/src/bioprocess.rs:115)

```rust
pub struct Run {
    pub id: u64,
    pub recipe: u64,
    pub name: String,
    pub status: RunStatus,
    pub ops: Vec<UnitOp>,
    pub replicates: Vec<Replicate>,
    pub created: u64,
}
```

## crates/domain/src/blast.rs · Hit

[crates/domain/src/blast.rs:7](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/domain/src/blast.rs:7)

```rust
pub struct Hit {
    pub name: String,
    pub qstart: usize,
    pub qend: usize,
    pub sstart: usize,
    pub send: usize,
    pub score: i32,
    pub identity: f32,
    pub len: usize,
}
```

## crates/domain/src/chemistry.rs · Atom

[crates/domain/src/chemistry.rs:9](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/domain/src/chemistry.rs:9)

```rust
pub struct Atom {
    pub element: String,
    pub aromatic: bool,
    pub charge: i32,
    pub explicit_h: Option<usize>,
}
```

## crates/domain/src/chemistry.rs · Bond

[crates/domain/src/chemistry.rs:18](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/domain/src/chemistry.rs:18)

```rust
pub struct Bond {
    pub a: usize,
    pub b: usize,
    pub order: u8,
}
```

## crates/domain/src/chemistry.rs · Molecule

[crates/domain/src/chemistry.rs:25](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/domain/src/chemistry.rs:25)

```rust
pub struct Molecule {
    pub atoms: Vec<Atom>,
    pub bonds: Vec<Bond>,
}
```

## crates/domain/src/chemistry.rs · Form

[crates/domain/src/chemistry.rs:439](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/domain/src/chemistry.rs:439)

```rust
pub struct Form {
    pub id: u64,
    pub name: String,
    pub formula: String,
    pub mw: f32,
}
```

## crates/domain/src/chemistry.rs · Compound

[crates/domain/src/chemistry.rs:470](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/domain/src/chemistry.rs:470)

```rust
pub struct Compound {
    pub id: u64,
    pub name: String,
    pub smiles: String,
    pub formula: String,
    pub mw: f32,
}
```

## crates/domain/src/chemistry.rs · ReactionComponent

[crates/domain/src/chemistry.rs:479](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/domain/src/chemistry.rs:479)

```rust
pub struct ReactionComponent {
    pub compound: u64,
    pub equivalents: f32,
}
```

## crates/domain/src/chemistry.rs · Reaction

[crates/domain/src/chemistry.rs:485](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/domain/src/chemistry.rs:485)

```rust
pub struct Reaction {
    pub id: u64,
    pub name: String,
    pub reactants: Vec<ReactionComponent>,
    pub products: Vec<ReactionComponent>,
}
```

## crates/domain/src/codon.rs · Host

[crates/domain/src/codon.rs:7](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/domain/src/codon.rs:7)

```rust
pub enum Host {
    EColi,
    Human,
}
```

## crates/domain/src/codon.rs · OptimizeOptions

[crates/domain/src/codon.rs:190](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/domain/src/codon.rs:190)

```rust
pub struct OptimizeOptions {
    pub gc_min: f32,
    pub gc_max: f32,
    pub avoid: Vec<Vec<u8>>,
    pub harmonize: bool,
}
```

## crates/domain/src/common_features.rs · CommonFeature

[crates/domain/src/common_features.rs:11](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/domain/src/common_features.rs:11)

```rust
pub struct CommonFeature {
    pub name: &'static str,
    pub kind: &'static str,
    pub sequence: &'static str,
    pub color: [u8; 3],
    pub note: &'static str,
}
```

## crates/domain/src/common_features.rs · DetectedFeature

[crates/domain/src/common_features.rs:184](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/domain/src/common_features.rs:184)

```rust
pub struct DetectedFeature {
    pub template: usize,
    pub range: Range,
    pub strand: Strand,
}
```

## crates/domain/src/crispr.rs · Guide

[crates/domain/src/crispr.rs:7](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/domain/src/crispr.rs:7)

```rust
pub struct Guide {
    /// Top-strand start of the 20 nt protospacer.
    pub position: usize,
    pub strand: Strand,
    pub sequence: Vec<u8>,
    pub pam: String,
    /// Simplified Doench-like on-target score in [0,1].
    pub score: f32,
    /// Number of other sites with <= 3 mismatches.
    pub off_targets: usize,
}
```

## crates/domain/src/crispr.rs · OffTarget

[crates/domain/src/crispr.rs:122](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/domain/src/crispr.rs:122)

```rust
pub struct OffTarget {
    pub name: String,
    pub position: usize,
    pub strand: Strand,
    pub mismatches: usize,
    pub cfd: f32,
}
```

## crates/domain/src/demo.rs · Rng

[crates/domain/src/demo.rs:11](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/domain/src/demo.rs:11)

```rust
struct Rng(u64);

impl Rng {
    fn next_base(&mut self) -> u8 {
        self.0 = self
            .0
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        match (self.0 >> 33) % 4 {
            0 => b'A',
            1 => b'C',
            2 => b'G',
            _ => b'T',
        }
    }
}
```

## crates/domain/src/digest.rs · Fragment

[crates/domain/src/digest.rs:4](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/domain/src/digest.rs:4)

```rust
pub struct Fragment {
    pub start: usize,
    pub end: usize,
    pub size: usize,
}
```

## crates/domain/src/document.rs · StrandMask

[crates/domain/src/document.rs:8](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/domain/src/document.rs:8)

```rust
pub struct StrandMask(pub u8);

impl StrandMask {
    pub const TOP: u8 = 1;
    pub const BOTTOM: u8 = 2;
    pub const BOTH: u8 = 3;

    pub fn has_top(self) -> bool {
        self.0 & Self::TOP != 0
    }
    pub fn has_bottom(self) -> bool {
        self.0 & Self::BOTTOM != 0
    }
}
```

## crates/domain/src/document.rs · ColorRegion

[crates/domain/src/document.rs:24](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/domain/src/document.rs:24)

```rust
pub struct ColorRegion {
    pub range: Range,
    pub strands: StrandMask,
    pub color: [u8; 3],
}
```

## crates/domain/src/document.rs · Version

[crates/domain/src/document.rs:31](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/domain/src/document.rs:31)

```rust
pub struct Version {
    pub label: String,
    /// Unix seconds when the snapshot was taken.
    pub created: u64,
    pub doc: Document,
}
```

## crates/domain/src/document.rs · Document

[crates/domain/src/document.rs:39](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/domain/src/document.rs:39)

```rust
pub struct Document {
    pub name: String,
    pub description: String,
    pub sequence: DnaSequence,
    pub features: Vec<Feature>,
    pub primers: Vec<Primer>,
    pub chosen_enzymes: Vec<String>,
    pub color_regions: Vec<ColorRegion>,
    /// NCBI genetic code id used for ORFs and features without an override.
    pub default_genetic_code: u8,
    /// Saved snapshots (newest last); each snapshot's own `versions` is empty.
    #[serde(default)]
    pub versions: Vec<Version>,
    /// Free-form organization tags.
    #[serde(default)]
    pub tags: Vec<String>,
    next_id: u64,
}
```

## crates/domain/src/enzyme.rs · Enzyme

[crates/domain/src/enzyme.rs:7](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/domain/src/enzyme.rs:7)

```rust
pub struct Enzyme {
    pub name: String,
    /// Recognition sequence, 5'->3', IUPAC codes allowed.
    pub site: String,
    /// Top-strand cut offset relative to the site start (may lie outside the site).
    pub cut_top: i32,
    /// Bottom-strand cut offset in top-strand coordinates.
    pub cut_bottom: i32,
}
```

## crates/domain/src/enzyme.rs · Overhang

[crates/domain/src/enzyme.rs:18](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/domain/src/enzyme.rs:18)

```rust
pub enum Overhang {
    FivePrime(usize),
    ThreePrime(usize),
    Blunt,
}
```

## crates/domain/src/enzyme.rs · CutSite

[crates/domain/src/enzyme.rs:35](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/domain/src/enzyme.rs:35)

```rust
pub struct CutSite {
    pub enzyme: String,
    /// Site start on top-strand coordinates.
    pub position: usize,
    pub strand: Strand,
    pub cut_top: usize,
    pub cut_bottom: usize,
}
```

## crates/domain/src/enzyme_set.rs · EnzymeSet

[crates/domain/src/enzyme_set.rs:8](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/domain/src/enzyme_set.rs:8)

```rust
pub struct EnzymeSet {
    pub name: String,
    pub enzymes: Vec<String>,
    pub builtin: bool,
}
```

## crates/domain/src/enzyme_set.rs · EnzymeFilter

[crates/domain/src/enzyme_set.rs:92](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/domain/src/enzyme_set.rs:92)

```rust
pub struct EnzymeFilter {
    pub name_contains: String,
    /// (min, max) number of cut sites, inclusive.
    pub cut_count: Option<(usize, usize)>,
    pub overhang: Option<OverhangFilter>,
    pub site_len: Option<(usize, usize)>,
}
```

## crates/domain/src/enzyme_set.rs · OverhangFilter

[crates/domain/src/enzyme_set.rs:101](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/domain/src/enzyme_set.rs:101)

```rust
pub enum OverhangFilter {
    FivePrime,
    ThreePrime,
    Blunt,
}
```

## crates/domain/src/feature.rs · Direction

[crates/domain/src/feature.rs:56](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/domain/src/feature.rs:56)

```rust
pub enum Direction {
    #[default]
    None,
    Forward,
    Reverse,
    Bidirectional,
}
```

## crates/domain/src/feature.rs · Feature

[crates/domain/src/feature.rs:84](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/domain/src/feature.rs:84)

```rust
pub struct Feature {
    pub id: u64,
    pub name: String,
    pub kind: String,
    /// Segments in ascending genomic order; gaps between them are introns.
    pub segments: Vec<Range>,
    pub direction: Direction,
    pub color: [u8; 3],
    pub note: String,
    pub product: String,
    pub translated: bool,
    /// 0 means "use the document default genetic code".
    pub genetic_code: u8,
    pub first_codon_met: bool,
    pub frame_offset: usize,
    pub visible: bool,
    pub cleavage_arrows: Vec<usize>,
}
```

## crates/domain/src/feature_lib.rs · LibFeature

[crates/domain/src/feature_lib.rs:11](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/domain/src/feature_lib.rs:11)

```rust
pub struct LibFeature {
    pub name: String,
    pub kind: String,
    pub motif: Vec<u8>,
    pub color: [u8; 3],
}
```

## crates/domain/src/genetic_code.rs · GeneticCode

[crates/domain/src/genetic_code.rs:4](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/domain/src/genetic_code.rs:4)

```rust
pub struct GeneticCode {
    pub id: u8,
    pub name: &'static str,
    pub table: &'static str,
    pub starts: &'static [&'static str],
}
```

## crates/domain/src/insights.rs · Dataset

[crates/domain/src/insights.rs:8](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/domain/src/insights.rs:8)

```rust
pub struct Dataset {
    pub name: String,
    pub columns: Vec<String>,
    pub rows: Vec<Vec<String>>,
}
```

## crates/domain/src/insights.rs · Chart

[crates/domain/src/insights.rs:15](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/domain/src/insights.rs:15)

```rust
pub enum Chart {
    Table,
    Bar,
    Line,
}
```

## crates/domain/src/insights.rs · Block

[crates/domain/src/insights.rs:32](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/domain/src/insights.rs:32)

```rust
pub struct Block {
    pub name: String,
    pub source: String,
    pub chart: Chart,
    pub group_by: Option<String>,
}
```

## crates/domain/src/inventory.rs · ContainerKind

[crates/domain/src/inventory.rs:9](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/domain/src/inventory.rs:9)

```rust
pub enum ContainerKind {
    Location,
    Box,
    Plate,
    Tube,
}
```

## crates/domain/src/inventory.rs · WellRole

[crates/domain/src/inventory.rs:28](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/domain/src/inventory.rs:28)

```rust
pub enum WellRole {
    Sample,
    Control,
    Standard,
    Blank,
}
```

## crates/domain/src/inventory.rs · Container

[crates/domain/src/inventory.rs:47](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/domain/src/inventory.rs:47)

```rust
pub struct Container {
    pub id: u64,
    pub name: String,
    pub kind: ContainerKind,
    pub parent: Option<u64>,
    /// Well position (e.g. "A1") for tubes inside a plate/box.
    pub position: Option<String>,
    pub role: Option<WellRole>,
    /// Linked registry entity name (scientific identity).
    pub entity: Option<String>,
    pub quantity: f32,
    pub unit: String,
    pub concentration: f32,
    /// Plate dimensions (rows x cols), only for Plate.
    pub rows: u8,
    pub cols: u8,
}
```

## crates/domain/src/inventory.rs · PlateSnapshot

[crates/domain/src/inventory.rs:66](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/domain/src/inventory.rs:66)

```rust
pub struct PlateSnapshot {
    pub plate_name: String,
    pub taken: u64,
    pub wells: Vec<(String, Option<String>, Option<WellRole>, f32)>,
}
```

## crates/domain/src/inventory.rs · Inventory

[crates/domain/src/inventory.rs:73](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/domain/src/inventory.rs:73)

```rust
pub struct Inventory {
    pub containers: Vec<Container>,
    next_id: u64,
}
```

## crates/domain/src/invivo.rs · Study

[crates/domain/src/invivo.rs:7](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/domain/src/invivo.rs:7)

```rust
pub struct Study {
    pub id: u64,
    pub name: String,
    pub species: String,
    pub strain: String,
    pub created: u64,
}
```

## crates/domain/src/invivo.rs · Cohort

[crates/domain/src/invivo.rs:16](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/domain/src/invivo.rs:16)

```rust
pub struct Cohort {
    pub id: u64,
    pub study: u64,
    pub name: String,
    pub cage: String,
}
```

## crates/domain/src/invivo.rs · Animal

[crates/domain/src/invivo.rs:24](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/domain/src/invivo.rs:24)

```rust
pub struct Animal {
    pub id: u64,
    pub cohort: u64,
    pub tag: String,
    pub sex: char,
    pub group: u32,
    pub baseline_weight_g: f32,
}
```

## crates/domain/src/invivo.rs · Measurement

[crates/domain/src/invivo.rs:34](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/domain/src/invivo.rs:34)

```rust
pub struct Measurement {
    pub animal: u64,
    pub day: u32,
    pub weight_g: f32,
    pub tumor_mm3: Option<f32>,
}
```

## crates/domain/src/notebook.rs · EntryStatus

[crates/domain/src/notebook.rs:7](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/domain/src/notebook.rs:7)

```rust
pub enum EntryStatus {
    Draft,
    InReview,
    ChangesRequested,
    Approved,
    Archived,
}
```

## crates/domain/src/notebook.rs · Section

[crates/domain/src/notebook.rs:28](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/domain/src/notebook.rs:28)

```rust
pub struct Section {
    pub title: String,
    pub body: String,
}
```

## crates/domain/src/notebook.rs · ResultTable

[crates/domain/src/notebook.rs:34](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/domain/src/notebook.rs:34)

```rust
pub struct ResultTable {
    pub name: String,
    pub columns: Vec<String>,
    pub rows: Vec<Vec<String>>,
    /// Submitted rows become reusable records (persistence boundary).
    pub submitted: bool,
}
```

## crates/domain/src/notebook.rs · ReviewEvent

[crates/domain/src/notebook.rs:43](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/domain/src/notebook.rs:43)

```rust
pub struct ReviewEvent {
    pub ts: u64,
    pub actor: String,
    pub action: String,
    pub comment: String,
}
```

## crates/domain/src/notebook.rs · Entry

[crates/domain/src/notebook.rs:51](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/domain/src/notebook.rs:51)

```rust
pub struct Entry {
    pub id: u64,
    pub title: String,
    pub project: Option<u64>,
    pub folder: String,
    pub sections: Vec<Section>,
    pub tables: Vec<ResultTable>,
    pub status: EntryStatus,
    pub template: Option<String>,
    pub reviews: Vec<ReviewEvent>,
    /// Number of (re)submissions; increments on corrected re-submission.
    #[serde(default)]
    pub revision: u32,
    pub created: u64,
    pub updated: u64,
}
```

## crates/domain/src/notebook.rs · Template

[crates/domain/src/notebook.rs:203](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/domain/src/notebook.rs:203)

```rust
pub struct Template {
    pub name: String,
    pub sections: Vec<Section>,
    pub tables: Vec<ResultTable>,
}
```

## crates/domain/src/pcr.rs · PcrProduct

[crates/domain/src/pcr.rs:7](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/domain/src/pcr.rs:7)

```rust
pub struct PcrProduct {
    /// Top-strand start of the amplicon (inclusive).
    pub start: usize,
    /// Top-strand end of the amplicon (exclusive, mod len).
    pub end: usize,
    pub len: usize,
    pub bases: Vec<u8>,
    pub fwd_site: BindingSite,
    pub rev_site: BindingSite,
}
```

## crates/domain/src/primer.rs · Primer

[crates/domain/src/primer.rs:7](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/domain/src/primer.rs:7)

```rust
pub struct Primer {
    pub id: u64,
    pub name: String,
    pub sequence: Vec<u8>,
    pub description: String,
    pub color: [u8; 3],
    pub phosphorylated: bool,
    pub visible: bool,
}
```

## crates/domain/src/primer.rs · HybridizationParams

[crates/domain/src/primer.rs:52](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/domain/src/primer.rs:52)

```rust
pub struct HybridizationParams {
    /// Consecutive matching bases required at the primer 3' end.
    pub min_3prime_match: usize,
    /// Minimum melting temperature of the hybridized region (°C).
    pub min_tm: f32,
    /// Monovalent salt concentration (mM).
    pub salt_mm: f32,
    /// Primer concentration (nM).
    pub primer_nm: f32,
}
```

## crates/domain/src/primer.rs · BindingSite

[crates/domain/src/primer.rs:125](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/domain/src/primer.rs:125)

```rust
pub struct BindingSite {
    /// Top-strand start of the hybridized region.
    pub start: usize,
    pub len: usize,
    /// Top: primer sequence equals the top strand, extends rightward.
    /// Bottom: primer equals the bottom strand, extends leftward.
    pub strand: Strand,
    pub tm: f32,
    pub matched_3prime: usize,
}
```

## crates/domain/src/primer_design.rs · DesignParams

[crates/domain/src/primer_design.rs:7](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/domain/src/primer_design.rs:7)

```rust
pub struct DesignParams {
    pub min_len: usize,
    pub max_len: usize,
    pub target_tm: f32,
    pub tm_tol: f32,
    pub min_gc: f32,
    pub max_gc: f32,
    /// Optional 5' overhang (e.g. restriction site) prepended to the annealing region.
    pub overhang: String,
}
```

## crates/domain/src/primer_design.rs · Candidate

[crates/domain/src/primer_design.rs:33](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/domain/src/primer_design.rs:33)

```rust
pub struct Candidate {
    /// Full primer including any 5' overhang.
    pub sequence: Vec<u8>,
    /// Annealing-region start on the top strand.
    pub start: usize,
    pub strand: Strand,
    pub tm: f32,
    pub gc: f32,
    pub len: usize,
    pub clamp: bool,
    pub score: f32,
}
```

## crates/domain/src/primer_struct.rs · Secondary

[crates/domain/src/primer_struct.rs:56](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/domain/src/primer_struct.rs:56)

```rust
pub struct Secondary {
    pub hairpin: Option<(usize, usize)>,
    pub dimer: usize,
    pub dimer_3prime: bool,
}
```

## crates/domain/src/project.rs · Project

[crates/domain/src/project.rs:6](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/domain/src/project.rs:6)

```rust
pub struct Project {
    pub id: u64,
    pub name: String,
    pub parent: Option<u64>,
}
```

## crates/domain/src/project.rs · Workspace

[crates/domain/src/project.rs:13](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/domain/src/project.rs:13)

```rust
pub struct Workspace {
    pub projects: Vec<Project>,
    next_id: u64,
}
```

## crates/domain/src/protein.rs · ProteinStats

[crates/domain/src/protein.rs:167](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/domain/src/protein.rs:167)

```rust
pub struct ProteinStats {
    pub len: usize,
    pub mw: f32,
    pub pi: f32,
    pub gravy: f32,
    pub charge_ph7: f32,
    pub ext_coeff: f32,
    pub composition: Vec<(char, usize)>,
}
```

## crates/domain/src/registry.rs · Kind

[crates/domain/src/registry.rs:10](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/domain/src/registry.rs:10)

```rust
pub enum Kind {
    Plasmid,
    Strain,
    Primer,
}
```

## crates/domain/src/registry.rs · Entry

[crates/domain/src/registry.rs:57](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/domain/src/registry.rs:57)

```rust
pub struct Entry {
    pub id: u64,
    pub kind: Kind,
    pub values: BTreeMap<String, String>,
    pub created: u64,
    pub updated: u64,
    /// Name of the linked document (if any).
    pub linked_doc: Option<String>,
    /// Archived entries are hidden by default but retained.
    #[serde(default)]
    pub archived: bool,
    /// Related registry entry ids (Entity Map edges).
    #[serde(default)]
    pub relations: Vec<u64>,
}
```

## crates/domain/src/registry.rs · Registry

[crates/domain/src/registry.rs:83](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/domain/src/registry.rs:83)

```rust
pub struct Registry {
    pub entries: Vec<Entry>,
    next_id: u64,
}
```

## crates/domain/src/rna.rs · Monomer

[crates/domain/src/rna.rs:9](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/domain/src/rna.rs:9)

```rust
pub struct Monomer {
    pub code: String,
    pub name: String,
    pub base: char,
    pub sugar: String,
    pub phosphate: String,
    /// Residue molecular weight (monophosphate, minus water for polymer).
    pub mw: f32,
}
```

## crates/domain/src/rna.rs · Mod

[crates/domain/src/rna.rs:68](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/domain/src/rna.rs:68)

```rust
pub struct Mod {
    pub pos: usize,
    pub code: String,
}
```

## crates/domain/src/rna.rs · Oligo

[crates/domain/src/rna.rs:74](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/domain/src/rna.rs:74)

```rust
pub struct Oligo {
    pub id: u64,
    pub name: String,
    /// RNA sequence (A/U/G/C).
    pub seq: String,
    pub mods: Vec<Mod>,
}
```

## crates/domain/src/rna.rs · Pattern

[crates/domain/src/rna.rs:84](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/domain/src/rna.rs:84)

```rust
pub enum Pattern {
    All,
    Alternating,
    EveryThird,
    Ends,
}
```

## crates/domain/src/search.rs · DnaMatcher

[crates/domain/src/search.rs:15](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/domain/src/search.rs:15)

```rust
enum DnaMatcher {
    Allowed(u8),
    Excluded(u8),
    Literal(u8),
}
```

## crates/domain/src/search.rs · DnaHit

[crates/domain/src/search.rs:119](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/domain/src/search.rs:119)

```rust
pub struct DnaHit {
    pub start: usize,
    pub len: usize,
    pub strand: Strand,
}
```

## crates/domain/src/search.rs · ProteinHit

[crates/domain/src/search.rs:191](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/domain/src/search.rs:191)

```rust
pub struct ProteinHit {
    /// Top-strand start of the coding region.
    pub start: usize,
    /// Length in bases (3 * query length).
    pub len: usize,
    pub strand: Strand,
    pub frame: u8,
}
```

## crates/domain/src/sequence.rs · Topology

[crates/domain/src/sequence.rs:4](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/domain/src/sequence.rs:4)

```rust
pub enum Topology {
    Linear,
    #[default]
    Circular,
}
```

## crates/domain/src/sequence.rs · Strand

[crates/domain/src/sequence.rs:20](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/domain/src/sequence.rs:20)

```rust
pub enum Strand {
    Top,
    Bottom,
}
```

## crates/domain/src/sequence.rs · Range

[crates/domain/src/sequence.rs:37](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/domain/src/sequence.rs:37)

```rust
pub struct Range {
    pub start: usize,
    pub len: usize,
}
```

## crates/domain/src/sequence.rs · DnaSequence

[crates/domain/src/sequence.rs:80](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/domain/src/sequence.rs:80)

```rust
pub struct DnaSequence {
    pub bases: Vec<u8>,
    pub topology: Topology,
}
```

## crates/domain/src/translate.rs · Orf

[crates/domain/src/translate.rs:16](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/domain/src/translate.rs:16)

```rust
pub struct Orf {
    /// Top-strand coordinate of the first base (start codon for Top, last base for Bottom).
    pub start: usize,
    /// Length in bases, including the stop codon.
    pub len: usize,
    pub strand: Strand,
    pub frame: u8,
    pub aa_len: usize,
}
```

## crates/domain/src/workflow.rs · TaskStatus

[crates/domain/src/workflow.rs:10](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/domain/src/workflow.rs:10)

```rust
pub enum TaskStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
    Skipped,
}
```

## crates/domain/src/workflow.rs · Task

[crates/domain/src/workflow.rs:31](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/domain/src/workflow.rs:31)

```rust
pub struct Task {
    pub id: u64,
    pub title: String,
    pub schema: String,
    pub status: TaskStatus,
    pub assignee: String,
    pub inputs: BTreeMap<String, String>,
    pub outputs: BTreeMap<String, String>,
    pub created: u64,
    pub updated: u64,
}
```

## crates/domain/src/workflow.rs · RequestStatus

[crates/domain/src/workflow.rs:44](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/domain/src/workflow.rs:44)

```rust
pub enum RequestStatus {
    Draft,
    Sent,
}
```

## crates/domain/src/workflow.rs · Request

[crates/domain/src/workflow.rs:50](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/domain/src/workflow.rs:50)

```rust
pub struct Request {
    pub id: u64,
    pub name: String,
    pub kind: String,
    pub status: RequestStatus,
    pub task_ids: Vec<u64>,
}
```

## crates/domain/src/workflow.rs · Stage

[crates/domain/src/workflow.rs:59](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/domain/src/workflow.rs:59)

```rust
pub struct Stage {
    pub id: u64,
    pub name: String,
    pub task_id: u64,
    pub next: Vec<u64>,
}
```

## crates/domain/src/workflow.rs · Workflows

[crates/domain/src/workflow.rs:67](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/domain/src/workflow.rs:67)

```rust
pub struct Workflows {
    pub tasks: Vec<Task>,
    pub requests: Vec<Request>,
    pub stages: Vec<Stage>,
    next_id: u64,
}
```

## crates/storage/src/alignments.rs · SavedAlignment

[crates/storage/src/alignments.rs:13](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/storage/src/alignments.rs:13)

```rust
pub struct SavedAlignment {
    pub id: u64,
    pub name: String,
    pub created: u64,
    pub labels: Vec<String>,
    pub result: Alignment,
}
```

## crates/storage/src/alignments.rs · Alignments

[crates/storage/src/alignments.rs:22](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/storage/src/alignments.rs:22)

```rust
pub struct Alignments {
    pub items: Vec<SavedAlignment>,
    next_id: u64,
}
```

## crates/storage/src/alignments.rs · File

[crates/storage/src/alignments.rs:53](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/storage/src/alignments.rs:53)

```rust
struct File {
    version: u32,
    alignments: Alignments,
}
```

## crates/storage/src/analytics.rs · Analytics

[crates/storage/src/analytics.rs:14](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/storage/src/analytics.rs:14)

```rust
pub struct Analytics {
    pub blocks: Vec<Block>,
    pub automation: Automation,
}
```

## crates/storage/src/analytics.rs · File

[crates/storage/src/analytics.rs:20](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/storage/src/analytics.rs:20)

```rust
struct File {
    version: u32,
    analytics: Analytics,
}
```

## crates/storage/src/biologics.rs · Biologics

[crates/storage/src/biologics.rs:13](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/storage/src/biologics.rs:13)

```rust
pub struct Biologics {
    pub items: Vec<Biologic>,
    next_id: u64,
}
```

## crates/storage/src/biologics.rs · File

[crates/storage/src/biologics.rs:39](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/storage/src/biologics.rs:39)

```rust
struct File {
    version: u32,
    biologics: Biologics,
}
```

## crates/storage/src/bioprocess.rs · Bioprocess

[crates/storage/src/bioprocess.rs:13](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/storage/src/bioprocess.rs:13)

```rust
pub struct Bioprocess {
    pub recipes: Vec<Recipe>,
    pub runs: Vec<Run>,
    next_id: u64,
}
```

## crates/storage/src/bioprocess.rs · File

[crates/storage/src/bioprocess.rs:56](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/storage/src/bioprocess.rs:56)

```rust
struct File {
    version: u32,
    bioprocess: Bioprocess,
}
```

## crates/storage/src/chemistry.rs · Lot

[crates/storage/src/chemistry.rs:13](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/storage/src/chemistry.rs:13)

```rust
pub struct Lot {
    pub id: u64,
    pub compound: u64,
    pub form: u64,
    pub quantity: f32,
    pub unit: String,
}
```

## crates/storage/src/chemistry.rs · Chemistry

[crates/storage/src/chemistry.rs:22](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/storage/src/chemistry.rs:22)

```rust
pub struct Chemistry {
    pub compounds: Vec<Compound>,
    pub reactions: Vec<Reaction>,
    pub lots: Vec<Lot>,
    next_id: u64,
}
```

## crates/storage/src/chemistry.rs · File

[crates/storage/src/chemistry.rs:81](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/storage/src/chemistry.rs:81)

```rust
struct File {
    version: u32,
    chemistry: Chemistry,
}
```

## crates/storage/src/config.rs · UiPrefs

[crates/storage/src/config.rs:14](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/storage/src/config.rs:14)

```rust
pub struct UiPrefs {
    pub show_features: bool,
    pub show_enzymes: bool,
    pub show_primers: bool,
    pub show_orfs: bool,
    pub show_translations: bool,
    pub show_complement: bool,
    pub show_colors: bool,
    pub show_zoom_controls: bool,
    pub orf_min_aa: usize,
    pub genetic_code: u8,
}
```

## crates/storage/src/config.rs · Config

[crates/storage/src/config.rs:46](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/storage/src/config.rs:46)

```rust
pub struct Config {
    pub recent_files: Vec<PathBuf>,
    pub last_directory: Option<PathBuf>,
    pub window_size: Option<[f32; 2]>,
    pub window_maximized: bool,
    pub ui: UiPrefs,
}
```

## crates/storage/src/feature_lib.rs · File

[crates/storage/src/feature_lib.rs:13](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/storage/src/feature_lib.rs:13)

```rust
struct File {
    version: u32,
    library: Vec<LibFeature>,
}
```

## crates/storage/src/genbank.rs · Section

[crates/storage/src/genbank.rs:42](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/storage/src/genbank.rs:42)

```rust
    enum Section {
        None,
        Definition,
        Features,
        Origin,
    }
```

## crates/storage/src/genbank.rs · RawFeature

[crates/storage/src/genbank.rs:212](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/storage/src/genbank.rs:212)

```rust
struct RawFeature {
    key: String,
    location: String,
    qualifiers: Vec<(String, String)>,
}
```

## crates/storage/src/genbank.rs · Qualifiers

[crates/storage/src/genbank.rs:218](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/storage/src/genbank.rs:218)

```rust
struct Qualifiers<'a>(&'a [(String, String)]);

impl Qualifiers<'_> {
    fn get(&self, key: &str) -> Option<String> {
        self.0
            .iter()
            .find(|(k, _)| k.eq_ignore_ascii_case(key))
            .map(|(_, v)| unquote(v))
    }

    fn name(&self) -> Option<String> {
        for key in ["label", "gene", "product", "locus_tag", "standard_name"] {
            if let Some(v) = self.get(key) {
                if !v.is_empty() {
                    return Some(v);
                }
            }
        }
        None
    }
}
```

## crates/storage/src/inventory.rs · File

[crates/storage/src/inventory.rs:13](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/storage/src/inventory.rs:13)

```rust
struct File {
    version: u32,
    inventory: Inventory,
}
```

## crates/storage/src/invivo.rs · Invivo

[crates/storage/src/invivo.rs:13](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/storage/src/invivo.rs:13)

```rust
pub struct Invivo {
    pub studies: Vec<Study>,
    pub cohorts: Vec<Cohort>,
    pub animals: Vec<Animal>,
    pub measurements: Vec<Measurement>,
    next_id: u64,
}
```

## crates/storage/src/invivo.rs · File

[crates/storage/src/invivo.rs:88](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/storage/src/invivo.rs:88)

```rust
struct File {
    version: u32,
    invivo: Invivo,
}
```

## crates/storage/src/lib.rs · StorageError

[crates/storage/src/lib.rs:35](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/storage/src/lib.rs:35)

```rust
pub enum StorageError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Invalid document JSON: {0}")]
    Json(#[from] serde_json::Error),
    #[error("{0}")]
    Format(String),
}
```

## crates/storage/src/lib.rs · Envelope

[crates/storage/src/lib.rs:47](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/storage/src/lib.rs:47)

```rust
struct Envelope {
    format: String,
    version: u32,
    document: Document,
}
```

## crates/storage/src/records.rs · Records

[crates/storage/src/records.rs:16](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/storage/src/records.rs:16)

```rust
pub struct Records {
    pub workspace: Workspace,
    pub entries: Vec<Entry>,
    pub templates: Vec<Template>,
    pub audit: AuditLog,
}
```

## crates/storage/src/records.rs · File

[crates/storage/src/records.rs:24](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/storage/src/records.rs:24)

```rust
struct File {
    version: u32,
    records: Records,
}
```

## crates/storage/src/registry.rs · File

[crates/storage/src/registry.rs:13](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/storage/src/registry.rs:13)

```rust
struct File {
    version: u32,
    registry: Registry,
}
```

## crates/storage/src/rna.rs · Rna

[crates/storage/src/rna.rs:13](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/storage/src/rna.rs:13)

```rust
pub struct Rna {
    pub oligos: Vec<Oligo>,
    next_id: u64,
}
```

## crates/storage/src/rna.rs · File

[crates/storage/src/rna.rs:43](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/storage/src/rna.rs:43)

```rust
struct File {
    version: u32,
    rna: Rna,
}
```

## crates/storage/src/views.rs · SavedView

[crates/storage/src/views.rs:13](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/storage/src/views.rs:13)

```rust
pub struct SavedView {
    pub id: u64,
    pub table: String,
    pub name: String,
    pub hidden: Vec<usize>,
    pub sort_column: usize,
    /// None = unsorted, Some(true) = desc, Some(false) = asc.
    pub sort_desc: Option<bool>,
    pub filter: String,
}
```

## crates/storage/src/views.rs · Views

[crates/storage/src/views.rs:25](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/storage/src/views.rs:25)

```rust
pub struct Views {
    pub items: Vec<SavedView>,
    next_id: u64,
}
```

## crates/storage/src/views.rs · File

[crates/storage/src/views.rs:70](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/storage/src/views.rs:70)

```rust
struct File {
    version: u32,
    views: Views,
}
```

## crates/storage/src/workflow.rs · File

[crates/storage/src/workflow.rs:13](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/storage/src/workflow.rs:13)

```rust
struct File {
    version: u32,
    workflows: Workflows,
}
```

