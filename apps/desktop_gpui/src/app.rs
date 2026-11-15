//! HelixBench GPUI shell: root entity, chrome (menu bar, tabs, side toolbar,
//! status bar), keyboard shortcuts, overlays and view dispatch. All document
//! logic lives in `appcore`; this file is presentation + interaction only.

use std::collections::HashMap;

use gpui::{
    actions, anchored, deferred, div, point, prelude::*, px, size, AnyElement, App, Bounds,
    Context, Corner, Entity, FocusHandle, Focusable, KeyDownEvent, MouseButton, MouseDownEvent,
    Pixels, Point, ScrollHandle, SharedString, Size, UniformListScrollHandle, Window,
};

use appcore::actions as act;
use appcore::state::{AppState, ColorsMode, Dialog, PendingAction, View};
use domain::Topology;

use crate::text_input::TextInput;
use crate::theme;
use crate::widgets;

actions!(
    hb,
    [
        New, Open, Save, SaveAs, Undo, Redo, Find, Escape, SelectAll, Copy, Cut, Paste, Delete,
        View1, View2, View3, View4, View5, View6,
    ]
);

pub fn bind_keys(cx: &mut App) {
    use gpui::KeyBinding;
    cx.bind_keys([
        KeyBinding::new("ctrl-n", New, Some("HelixBench")),
        KeyBinding::new("ctrl-o", Open, Some("HelixBench")),
        KeyBinding::new("ctrl-s", Save, Some("HelixBench")),
        KeyBinding::new("ctrl-shift-s", SaveAs, Some("HelixBench")),
        KeyBinding::new("ctrl-z", Undo, Some("HelixBench")),
        KeyBinding::new("ctrl-y", Redo, Some("HelixBench")),
        KeyBinding::new("ctrl-f", Find, Some("HelixBench")),
        KeyBinding::new("escape", Escape, Some("HelixBench")),
        KeyBinding::new("ctrl-a", SelectAll, Some("HelixBench")),
        KeyBinding::new("ctrl-c", Copy, Some("HelixBench")),
        KeyBinding::new("ctrl-x", Cut, Some("HelixBench")),
        KeyBinding::new("ctrl-v", Paste, Some("HelixBench")),
        KeyBinding::new("delete", Delete, Some("HelixBench")),
        KeyBinding::new("backspace", Delete, Some("HelixBench")),
        KeyBinding::new("ctrl-1", View1, Some("HelixBench")),
        KeyBinding::new("ctrl-2", View2, Some("HelixBench")),
        KeyBinding::new("ctrl-3", View3, Some("HelixBench")),
        KeyBinding::new("ctrl-4", View4, Some("HelixBench")),
        KeyBinding::new("ctrl-5", View5, Some("HelixBench")),
        KeyBinding::new("ctrl-6", View6, Some("HelixBench")),
    ]);
}

#[derive(Clone, Copy, PartialEq, Eq)]
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

impl HelixBench {
    pub fn new(app: AppState, cx: &mut Context<Self>) -> Self {
        Self {
            app,
            focus_handle: cx.focus_handle(),
            initialized: false,
            last_title: String::new(),
            toast_armed: None,
            map_bounds: None,
            map_press: None,
            map_dragged: false,
            seq_scroll: 0.0,
            seq_view_h: 0.0,
            seq_total_h: 0.0,
            seq_press: None,
            seq_dragged: false,
            seq_layout: None,
            seq_mini_bounds: None,
            editor_scroll: 0.0,
            editor_seq_bounds: None,
            editor_seq_anchor: None,
            editor_slider_bounds: None,
            menu: None,
            menu_anchor: point(px(0.0), px(0.0)),
            combo: None,
            combo_anchor: point(px(0.0), px(0.0)),
            opts: None,
            opts_anchor: point(px(0.0), px(0.0)),
            cal_year: crate::util::civil_from_unix(crate::util::now_unix()).0,
            cal_month: crate::util::civil_from_unix(crate::util::now_unix()).1,
            cal_day: None,
            find_recents_open: false,
            find_recents_anchor: point(px(0.0), px(0.0)),
            dlg_combo: None,
            dlg_combo_anchor: point(px(0.0), px(0.0)),
            inputs: HashMap::new(),
            last_dialog_sig: String::new(),
            viewport: size(px(0.0), px(0.0)),
            dlg_scroll: ScrollHandle::new(),
            feat_scroll: UniformListScrollHandle::new(),
            primer_scroll: UniformListScrollHandle::new(),
            enzyme_scroll: UniformListScrollHandle::new(),
            history_scroll: UniformListScrollHandle::new(),
            detect_scroll: UniformListScrollHandle::new(),
        }
    }

    // --- Text field management -------------------------------------------

    pub fn input_entity(
        &mut self,
        cx: &mut Context<Self>,
        key: &str,
        seed: &str,
        placeholder: &str,
        mono: bool,
    ) -> Entity<TextInput> {
        if let Some(e) = self.inputs.get(key) {
            return e.clone();
        }
        let ph = placeholder.to_owned();
        let seed = seed.to_owned();
        let e = cx.new(|cx| {
            let mut ti = TextInput::new(cx, &ph, mono, theme::T_BASE);
            ti.set_text(seed);
            ti
        });
        // Re-render the shell whenever a field changes so the pull-model filters
        // and live search stay in sync with what the user typed.
        cx.observe(&e, |_, _, cx| cx.notify()).detach();
        self.inputs.insert(key.to_owned(), e.clone());
        e
    }

    pub fn set_input(&mut self, cx: &mut Context<Self>, key: &str, text: String) {
        if let Some(e) = self.inputs.get(key) {
            e.update(cx, |ti, _| ti.set_text(text));
        }
    }

    fn dialog_signature(&self) -> String {
        match &self.app.dialog {
            None => "none".into(),
            Some(Dialog::FeatureEditor(d)) => format!("feat:{:?}", d.id),
            Some(Dialog::PrimerEditor(d)) => format!("primer:{:?}", d.id),
            Some(Dialog::NewSequence(_)) => "newseq".into(),
            Some(Dialog::Rotate { .. }) => "rotate".into(),
            Some(Dialog::ColorSelection(d)) => format!("color:{}:{}", d.range.start, d.range.len),
            Some(Dialog::GeneticCodes { .. }) => "codes".into(),
            Some(Dialog::LetterCodes) => "letters".into(),
            Some(Dialog::SearchTips) => "tips".into(),
            Some(Dialog::About) => "about".into(),
            Some(Dialog::Options) => "options".into(),
            Some(Dialog::Unsaved { action }) => format!("unsaved:{action:?}"),
            Some(Dialog::DetectCommon { detected, .. }) => format!("detect:{}", detected.len()),
            Some(Dialog::Pcr(_)) => "pcr".into(),
            Some(Dialog::PrimerDesign(_)) => "pdesign".into(),
            Some(Dialog::Digest) => "digest".into(),
            Some(Dialog::Alignment(_)) => "alignment".into(),
            Some(Dialog::Assembly(_)) => "assembly".into(),
            Some(Dialog::Crispr(_)) => "crispr".into(),
            Some(Dialog::Codon(_)) => "codon".into(),
            Some(Dialog::Blast(_)) => "blast".into(),
            Some(Dialog::Combinatorial(_)) => "combinatorial".into(),
            Some(Dialog::Registry(_)) => "registry".into(),
            Some(Dialog::RegistryEditor(_)) => "registry-editor".into(),
            Some(Dialog::Protein(_)) => "protein".into(),
            Some(Dialog::FeatureLib(_)) => "feature-lib".into(),
            Some(Dialog::Workspace(_)) => "workspace".into(),
            Some(Dialog::Notebook(_)) => "notebook".into(),
            Some(Dialog::EntryEditor(_)) => "entry-editor".into(),
            Some(Dialog::Projects(_)) => "projects".into(),
            Some(Dialog::Audit(_)) => "audit".into(),
            Some(Dialog::Home(_)) => "home".into(),
            Some(Dialog::EntityMap(_)) => "entity-map".into(),
            Some(Dialog::RegistryImport(_)) => "registry-import".into(),
            Some(Dialog::Inventory(_)) => "inventory".into(),
            Some(Dialog::Workflows(_)) => "workflows".into(),
        }
    }

    fn focus_root(&mut self, window: &mut Window) {
        self.focus_handle.focus(window);
    }

    // --- Menus ------------------------------------------------------------

    pub fn open_menu(&mut self, kind: MenuKind, at: Point<Pixels>, cx: &mut Context<Self>) {
        self.menu = Some(kind);
        self.menu_anchor = at;
        cx.notify();
    }

    pub fn close_menu(&mut self, cx: &mut Context<Self>) {
        if self.menu.is_some() {
            self.menu = None;
            cx.notify();
        }
    }

    fn render_menu(&mut self, cx: &mut Context<Self>) -> Option<AnyElement> {
        let kind = self.menu?;
        let mut rows: Vec<AnyElement> = Vec::new();
        let sel_feature = self.app.selected_feature;
        let sel_primer = self.app.selected_primer;
        let has_sel = self.app.selection.is_some();
        let can_undo = !self.app.history.is_empty();
        let can_redo = !self.app.future.is_empty();
        let circular = self.app.doc.sequence.is_circular();

        match kind {
            MenuKind::File => {
                rows.push(
                    widgets::menu_row("m-new", "New Sequence…", "Ctrl+N", true)
                        .on_click(cx.listener(|t, _, _, cx| {
                            t.menu = None;
                            act::guard(&mut t.app, PendingAction::NewEmpty);
                            cx.notify();
                        }))
                        .into_any_element(),
                );
                rows.push(
                    widgets::menu_row("m-demo", "Open Demo Plasmid", "", true)
                        .on_click(cx.listener(|t, _, _, cx| {
                            t.menu = None;
                            act::guard(&mut t.app, PendingAction::NewDemo);
                            cx.notify();
                        }))
                        .into_any_element(),
                );
                rows.push(widgets::divider().into_any_element());
                rows.push(
                    widgets::menu_row("m-open", "Open…", "Ctrl+O", true)
                        .on_click(cx.listener(|t, _, _, cx| {
                            t.menu = None;
                            act::guard(&mut t.app, PendingAction::Open);
                            cx.notify();
                        }))
                        .into_any_element(),
                );
                rows.push(widgets::menu_section("Recent Files").into_any_element());
                if self.app.config.recent_files.is_empty() {
                    rows.push(
                        widgets::label("No recent files", theme::T_SM, theme::FAINT)
                            .px(px(10.0))
                            .py(px(3.0))
                            .into_any_element(),
                    );
                } else {
                    for (i, p) in self.app.config.recent_files.clone().into_iter().enumerate() {
                        let name = p
                            .file_name()
                            .map(|s| s.to_string_lossy().into_owned())
                            .unwrap_or_else(|| p.display().to_string());
                        let path = p.clone();
                        rows.push(
                            widgets::menu_row(
                                SharedString::from(format!("m-recent-{i}")),
                                &name,
                                "",
                                true,
                            )
                            .on_click(cx.listener(move |t, _, _, cx| {
                                t.menu = None;
                                act::guard(&mut t.app, PendingAction::OpenRecent(path.clone()));
                                cx.notify();
                            }))
                            .into_any_element(),
                        );
                    }
                }
                rows.push(widgets::divider().into_any_element());
                rows.push(
                    widgets::menu_row("m-save", "Save", "Ctrl+S", true)
                        .on_click(cx.listener(|t, _, _, cx| {
                            t.menu = None;
                            act::save(&mut t.app);
                            cx.notify();
                        }))
                        .into_any_element(),
                );
                rows.push(
                    widgets::menu_row("m-saveas", "Save As…", "Ctrl+Shift+S", true)
                        .on_click(cx.listener(|t, _, _, cx| {
                            t.menu = None;
                            act::save_as(&mut t.app);
                            cx.notify();
                        }))
                        .into_any_element(),
                );
                rows.push(widgets::menu_section("Export").into_any_element());
                for (id, label, which) in [
                    ("m-exp-fasta", "FASTA…", 0),
                    ("m-exp-gb", "GenBank…", 1),
                    ("m-exp-txt", "Plain sequence…", 2),
                ] {
                    rows.push(
                        widgets::menu_row(id, label, "", true)
                            .on_click(cx.listener(move |t, _, _, cx| {
                                t.menu = None;
                                match which {
                                    0 => act::export_fasta(&mut t.app),
                                    1 => act::export_genbank(&mut t.app),
                                    _ => act::export_plain(&mut t.app),
                                }
                                cx.notify();
                            }))
                            .into_any_element(),
                    );
                }
                rows.push(widgets::divider().into_any_element());
                rows.push(
                    widgets::menu_row("m-exit", "Exit", "", true)
                        .on_click(cx.listener(|t, _, _, cx| {
                            t.menu = None;
                            act::guard(&mut t.app, PendingAction::CloseApp);
                            cx.notify();
                        }))
                        .into_any_element(),
                );
            }
            MenuKind::Edit => {
                let undo_label = self
                    .app
                    .history
                    .last()
                    .map(|e| format!("Undo {}", e.label))
                    .unwrap_or_else(|| "Undo".into());
                let redo_label = self
                    .app
                    .future
                    .last()
                    .map(|e| format!("Redo {}", e.label))
                    .unwrap_or_else(|| "Redo".into());
                let mut u = widgets::menu_row("m-undo", &undo_label, "Ctrl+Z", can_undo);
                if can_undo {
                    u = u.on_click(cx.listener(|t, _, _, cx| {
                        t.menu = None;
                        if t.app.undo() {
                            t.app.toast("Undo");
                        }
                        cx.notify();
                    }));
                }
                rows.push(u.into_any_element());
                let mut r = widgets::menu_row("m-redo", &redo_label, "Ctrl+Y", can_redo);
                if can_redo {
                    r = r.on_click(cx.listener(|t, _, _, cx| {
                        t.menu = None;
                        if t.app.redo() {
                            t.app.toast("Redo");
                        }
                        cx.notify();
                    }));
                }
                rows.push(r.into_any_element());
                rows.push(widgets::divider().into_any_element());
                rows.push(
                    widgets::menu_row("m-cut", "Cut", "Ctrl+X", has_sel)
                        .on_click(cx.listener(|t, _, _, cx| {
                            t.menu = None;
                            if let Some(text) = act::cut_selection(&mut t.app) {
                                if !text.is_empty() {
                                    cx.write_to_clipboard(gpui::ClipboardItem::new_string(text));
                                }
                            }
                            cx.notify();
                        }))
                        .into_any_element(),
                );
                rows.push(
                    widgets::menu_row("m-copy", "Copy", "Ctrl+C", true)
                        .on_click(cx.listener(|t, _, _, cx| {
                            t.menu = None;
                            let text = act::copy_selection(&t.app);
                            if !text.is_empty() {
                                cx.write_to_clipboard(gpui::ClipboardItem::new_string(text));
                            }
                            cx.notify();
                        }))
                        .into_any_element(),
                );
                rows.push(
                    widgets::menu_row("m-paste", "Paste", "Ctrl+V", true)
                        .on_click(cx.listener(|t, _, _, cx| {
                            t.menu = None;
                            if let Some(text) = cx.read_from_clipboard().and_then(|i| i.text()) {
                                let dna: String = text
                                    .chars()
                                    .filter(|c| {
                                        c.is_ascii()
                                            && domain::iupac::dna_mask(c.to_ascii_uppercase() as u8)
                                                != 0
                                    })
                                    .collect();
                                if !dna.is_empty() {
                                    act::insert_bases(&mut t.app, &dna, true);
                                }
                            }
                            cx.notify();
                        }))
                        .into_any_element(),
                );
                rows.push(
                    widgets::menu_row("m-del", "Delete", "Del", has_sel)
                        .on_click(cx.listener(|t, _, _, cx| {
                            t.menu = None;
                            act::delete_selection(&mut t.app);
                            cx.notify();
                        }))
                        .into_any_element(),
                );
                rows.push(
                    widgets::menu_row("m-selall", "Select All", "Ctrl+A", true)
                        .on_click(cx.listener(|t, _, _, cx| {
                            t.menu = None;
                            act::select_all(&mut t.app);
                            cx.notify();
                        }))
                        .into_any_element(),
                );
                rows.push(widgets::divider().into_any_element());
                rows.push(
                    widgets::menu_row("m-color", "Color Selection…", "", has_sel)
                        .on_click(cx.listener(|t, _, _, cx| {
                            t.menu = None;
                            act::open_color_selection(&mut t.app);
                            cx.notify();
                        }))
                        .into_any_element(),
                );
                rows.push(
                    widgets::menu_row(
                        "m-clearcolor",
                        "Clear Custom Colors",
                        "",
                        !self.app.doc.color_regions.is_empty(),
                    )
                    .on_click(cx.listener(|t, _, _, cx| {
                        t.menu = None;
                        act::clear_custom_colors(&mut t.app);
                        cx.notify();
                    }))
                    .into_any_element(),
                );
                rows.push(widgets::divider().into_any_element());
                rows.push(
                    widgets::menu_row("m-find", "Find…", "Ctrl+F", true)
                        .on_click(cx.listener(|t, _, _, cx| {
                            t.menu = None;
                            t.app.search.open = true;
                            t.app.search.focus_request = true;
                            cx.notify();
                        }))
                        .into_any_element(),
                );
            }
            MenuKind::View => {
                for (key, label, on) in [
                    ("show_features", "Features", self.app.prefs.show_features),
                    ("show_enzymes", "Enzymes", self.app.prefs.show_enzymes),
                    ("show_primers", "Primers", self.app.prefs.show_primers),
                    ("show_orfs", "ORFs", self.app.prefs.show_orfs),
                    (
                        "show_translations",
                        "Translations",
                        self.app.prefs.show_translations,
                    ),
                    (
                        "show_complement",
                        "Complement strand",
                        self.app.prefs.show_complement,
                    ),
                    ("show_colors", "Base colors", self.app.prefs.show_colors),
                    (
                        "show_zoom_controls",
                        "Zoom controls",
                        self.app.prefs.show_zoom_controls,
                    ),
                ] {
                    rows.push(
                        widgets::menu_check(SharedString::from(format!("mv-{key}")), label, on)
                            .on_click(cx.listener(move |t, _, _, cx| {
                                t.toggle_pref(key);
                                cx.notify();
                            }))
                            .into_any_element(),
                    );
                }
                rows.push(widgets::divider().into_any_element());
                rows.push(widgets::menu_section("Show colors").into_any_element());
                for (mode, label) in [
                    (ColorsMode::None, "No colors"),
                    (ColorsMode::History, "History colors"),
                    (ColorsMode::Custom, "Custom DNA colors"),
                ] {
                    rows.push(
                        widgets::menu_check(
                            SharedString::from(format!("mvc-{}", mode.label())),
                            label,
                            self.app.colors_mode == mode,
                        )
                        .on_click(cx.listener(move |t, _, _, cx| {
                            t.app.colors_mode = mode;
                            cx.notify();
                        }))
                        .into_any_element(),
                    );
                }
                rows.push(widgets::divider().into_any_element());
                rows.push(
                    widgets::menu_row("mv-options", "Map & Sequence Options…", "", true)
                        .on_click(cx.listener(|t, _, _, cx| {
                            t.menu = None;
                            t.app.dialog = Some(Dialog::Options);
                            cx.notify();
                        }))
                        .into_any_element(),
                );
            }
            MenuKind::Features => {
                rows.push(
                    widgets::menu_row("mf-add", "Add Feature…", "", true)
                        .on_click(cx.listener(|t, _, _, cx| {
                            t.menu = None;
                            crate::dialogs::open_feature_editor(&mut t.app, None);
                            cx.notify();
                        }))
                        .into_any_element(),
                );
                let mut e = widgets::menu_row(
                    "mf-edit",
                    "Edit Selected Feature…",
                    "",
                    sel_feature.is_some(),
                );
                if sel_feature.is_some() {
                    e = e.on_click(cx.listener(move |t, _, _, cx| {
                        t.menu = None;
                        crate::dialogs::open_feature_editor(&mut t.app, sel_feature);
                        cx.notify();
                    }));
                }
                rows.push(e.into_any_element());
                let mut d = widgets::menu_row(
                    "mf-del",
                    "Delete Selected Feature",
                    "",
                    sel_feature.is_some(),
                );
                if sel_feature.is_some() {
                    d = d.on_click(cx.listener(move |t, _, _, cx| {
                        t.menu = None;
                        if let Some(id) = sel_feature {
                            act::delete_feature(&mut t.app, id);
                        }
                        cx.notify();
                    }));
                }
                rows.push(d.into_any_element());
                rows.push(widgets::divider().into_any_element());
                rows.push(
                    widgets::menu_row("mf-detect", "Detect Common Features…", "", true)
                        .on_click(cx.listener(|t, _, _, cx| {
                            t.menu = None;
                            crate::dialogs::open_detect_common(&mut t.app);
                            cx.notify();
                        }))
                        .into_any_element(),
                );
            }
            MenuKind::Primers => {
                rows.push(
                    widgets::menu_row("mp-add", "Add Primer…", "", true)
                        .on_click(cx.listener(|t, _, _, cx| {
                            t.menu = None;
                            crate::dialogs::open_primer_editor(&mut t.app, None);
                            cx.notify();
                        }))
                        .into_any_element(),
                );
                rows.push(
                    widgets::menu_row("mp-fromsel", "Add Primer from Selection…", "", has_sel)
                        .on_click(cx.listener(|t, _, _, cx| {
                            t.menu = None;
                            act::primer_from_selection(&mut t.app);
                            cx.notify();
                        }))
                        .into_any_element(),
                );
                let mut e =
                    widgets::menu_row("mp-edit", "Edit Selected Primer…", "", sel_primer.is_some());
                if sel_primer.is_some() {
                    e = e.on_click(cx.listener(move |t, _, _, cx| {
                        t.menu = None;
                        crate::dialogs::open_primer_editor(&mut t.app, sel_primer);
                        cx.notify();
                    }));
                }
                rows.push(e.into_any_element());
                let mut d =
                    widgets::menu_row("mp-del", "Delete Selected Primer", "", sel_primer.is_some());
                if sel_primer.is_some() {
                    d = d.on_click(cx.listener(move |t, _, _, cx| {
                        t.menu = None;
                        if let Some(id) = sel_primer {
                            act::delete_primer(&mut t.app, id);
                        }
                        cx.notify();
                    }));
                }
                rows.push(d.into_any_element());
            }
            MenuKind::Enzymes => {
                rows.push(
                    widgets::menu_row("me-choose", "Choose Enzymes…", "", true)
                        .on_click(cx.listener(|t, _, _, cx| {
                            t.set_view(View::Enzymes, cx);
                        }))
                        .into_any_element(),
                );
                rows.push(widgets::menu_section("Apply Set").into_any_element());
                for (i, s) in self.app.analysis.sets.clone().into_iter().enumerate() {
                    let label = format!("{} ({})", s.name, s.enzymes.len());
                    let names = s.enzymes.clone();
                    let set_name = s.name.clone();
                    rows.push(
                        widgets::menu_row(
                            SharedString::from(format!("me-set-{i}")),
                            &label,
                            "",
                            true,
                        )
                        .on_click(cx.listener(move |t, _, _, cx| {
                            t.menu = None;
                            let n = names.len();
                            let nm = set_name.clone();
                            t.app.mutate(
                                &format!("Apply enzyme set \"{nm}\""),
                                Vec::new(),
                                |doc| doc.chosen_enzymes = names.clone(),
                            );
                            t.app.toast(format!("Enzyme set: {nm} ({n} enzymes)"));
                            cx.notify();
                        }))
                        .into_any_element(),
                    );
                }
            }
            MenuKind::Tools => {
                rows.push(
                    widgets::menu_row("mt-pcr", "In-silico PCR…", "", true)
                        .on_click(cx.listener(|t, _, _, cx| {
                            t.menu = None;
                            act::open_pcr(&mut t.app);
                            cx.notify();
                        }))
                        .into_any_element(),
                );
                rows.push(
                    widgets::menu_row("mt-pdesign", "Primer Design…", "", true)
                        .on_click(cx.listener(|t, _, _, cx| {
                            t.menu = None;
                            act::open_primer_design(&mut t.app);
                            cx.notify();
                        }))
                        .into_any_element(),
                );
                rows.push(
                    widgets::menu_row("mt-digest", "Virtual Digest & Gel…", "", true)
                        .on_click(cx.listener(|t, _, _, cx| {
                            t.menu = None;
                            act::open_digest(&mut t.app);
                            cx.notify();
                        }))
                        .into_any_element(),
                );
                rows.push(
                    widgets::menu_row("mt-align", "Sequence Alignment…", "", true)
                        .on_click(cx.listener(|t, _, _, cx| {
                            t.menu = None;
                            act::open_alignment(&mut t.app);
                            cx.notify();
                        }))
                        .into_any_element(),
                );
                rows.push(
                    widgets::menu_row("mt-assembly", "Assembly Wizard…", "", true)
                        .on_click(cx.listener(|t, _, _, cx| {
                            t.menu = None;
                            act::open_assembly(&mut t.app);
                            cx.notify();
                        }))
                        .into_any_element(),
                );
                rows.push(
                    widgets::menu_row("mt-crispr", "CRISPR Guides…", "", true)
                        .on_click(cx.listener(|t, _, _, cx| {
                            t.menu = None;
                            act::open_crispr(&mut t.app);
                            cx.notify();
                        }))
                        .into_any_element(),
                );
                rows.push(
                    widgets::menu_row("mt-codon", "Codon Optimization…", "", true)
                        .on_click(cx.listener(|t, _, _, cx| {
                            t.menu = None;
                            act::open_codon(&mut t.app);
                            cx.notify();
                        }))
                        .into_any_element(),
                );
                rows.push(
                    widgets::menu_row("mt-blast", "Local BLAST…", "", true)
                        .on_click(cx.listener(|t, _, _, cx| {
                            t.menu = None;
                            act::open_blast(&mut t.app);
                            cx.notify();
                        }))
                        .into_any_element(),
                );
                rows.push(
                    widgets::menu_row("mt-comb", "Combinatorial Assembly…", "", true)
                        .on_click(cx.listener(|t, _, _, cx| {
                            t.menu = None;
                            act::open_combinatorial(&mut t.app);
                            cx.notify();
                        }))
                        .into_any_element(),
                );
                rows.push(
                    widgets::menu_row("mt-registry", "Local Registry…", "", true)
                        .on_click(cx.listener(|t, _, _, cx| {
                            t.menu = None;
                            act::open_registry(&mut t.app);
                            cx.notify();
                        }))
                        .into_any_element(),
                );
                rows.push(
                    widgets::menu_row("mt-protein", "Protein Analysis…", "", true)
                        .on_click(cx.listener(|t, _, _, cx| {
                            t.menu = None;
                            act::open_protein(&mut t.app);
                            cx.notify();
                        }))
                        .into_any_element(),
                );
                rows.push(
                    widgets::menu_row("mt-featlib", "Feature Library…", "", true)
                        .on_click(cx.listener(|t, _, _, cx| {
                            t.menu = None;
                            act::open_feature_lib(&mut t.app);
                            cx.notify();
                        }))
                        .into_any_element(),
                );
                rows.push(
                    widgets::menu_row("mt-workspace", "Workspace / Bulk…", "", true)
                        .on_click(cx.listener(|t, _, _, cx| {
                            t.menu = None;
                            act::open_workspace(&mut t.app);
                            cx.notify();
                        }))
                        .into_any_element(),
                );
                rows.push(widgets::divider().into_any_element());
                rows.push(
                    widgets::menu_row("mt-codes", "Genetic Codes…", "", true)
                        .on_click(cx.listener(|t, _, _, cx| {
                            t.menu = None;
                            t.app.dialog = Some(Dialog::GeneticCodes {
                                selected: t.app.doc.default_genetic_code,
                            });
                            cx.notify();
                        }))
                        .into_any_element(),
                );
                rows.push(
                    widgets::menu_row("mt-letters", "Letter Codes…", "", true)
                        .on_click(cx.listener(|t, _, _, cx| {
                            t.menu = None;
                            t.app.dialog = Some(Dialog::LetterCodes);
                            cx.notify();
                        }))
                        .into_any_element(),
                );
                rows.push(
                    widgets::menu_row("mt-tips", "Search Tips…", "", true)
                        .on_click(cx.listener(|t, _, _, cx| {
                            t.menu = None;
                            t.app.dialog = Some(Dialog::SearchTips);
                            cx.notify();
                        }))
                        .into_any_element(),
                );
                rows.push(widgets::divider().into_any_element());
                let mut rot =
                    widgets::menu_row("mt-rotate", "Rotate Circular Sequence…", "", circular);
                if circular {
                    rot = rot.on_click(cx.listener(|t, _, _, cx| {
                        t.menu = None;
                        t.app.dialog = Some(Dialog::Rotate {
                            position: "1".into(),
                            error: None,
                        });
                        cx.notify();
                    }));
                }
                rows.push(rot.into_any_element());
                rows.push(
                    widgets::menu_row("mt-revcomp", "Reverse Complement Sequence", "", true)
                        .on_click(cx.listener(|t, _, _, cx| {
                            t.menu = None;
                            act::reverse_complement_sequence(&mut t.app);
                            cx.notify();
                        }))
                        .into_any_element(),
                );
                rows.push(widgets::divider().into_any_element());
                rows.push(widgets::menu_section("Topology").into_any_element());
                for (topo, label) in [
                    (Topology::Linear, "Linear"),
                    (Topology::Circular, "Circular"),
                ] {
                    rows.push(
                        widgets::menu_check(
                            SharedString::from(format!("mt-topo-{label}")),
                            label,
                            self.app.doc.sequence.topology == topo,
                        )
                        .on_click(cx.listener(move |t, _, _, cx| {
                            t.menu = None;
                            act::set_topology(&mut t.app, topo);
                            cx.notify();
                        }))
                        .into_any_element(),
                    );
                }
            }
            MenuKind::Lab => {
                for (id, label, act_fn) in [
                    ("lb-home", "Home / My Work", 0),
                    ("lb-notebook", "Notebook", 1),
                    ("lb-projects", "Projects & Folders", 2),
                    ("lb-entitymap", "Entity Map", 3),
                    ("lb-import", "Registry Import (CSV)", 4),
                    ("lb-audit", "Audit Log", 5),
                    ("lb-inv", "Inventory", 6),
                    ("lb-wf", "Workflows", 7),
                ] {
                    rows.push(
                        widgets::menu_row(id, label, "", true)
                            .on_click(cx.listener(move |t, _, _, cx| {
                                t.menu = None;
                                match act_fn {
                                    0 => act::open_home(&mut t.app),
                                    1 => act::open_notebook(&mut t.app),
                                    2 => act::open_projects(&mut t.app),
                                    3 => act::open_entity_map(&mut t.app),
                                    4 => act::open_registry_import(&mut t.app),
                                    6 => act::open_inventory(&mut t.app),
                                    7 => act::open_workflows(&mut t.app),
                                    _ => act::open_audit(&mut t.app),
                                }
                                cx.notify();
                            }))
                            .into_any_element(),
                    );
                }
            }
            MenuKind::Help => {
                rows.push(
                    widgets::menu_row("mh-about", "About HelixBench", "", true)
                        .on_click(cx.listener(|t, _, _, cx| {
                            t.menu = None;
                            t.app.dialog = Some(Dialog::About);
                            cx.notify();
                        }))
                        .into_any_element(),
                );
                rows.push(
                    widgets::menu_row("mh-tips", "Search Tips…", "", true)
                        .on_click(cx.listener(|t, _, _, cx| {
                            t.menu = None;
                            t.app.dialog = Some(Dialog::SearchTips);
                            cx.notify();
                        }))
                        .into_any_element(),
                );
            }
        }

        let mut card = div()
            .id("menu-card")
            .flex()
            .flex_col()
            .min_w(px(232.0))
            .max_w(px(340.0))
            .py(px(4.0))
            .bg(theme::OVERLAY)
            .border_1()
            .border_color(theme::BORDER_STRONG)
            .rounded(px(8.0))
            .shadow_lg()
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|_, _, _, cx| cx.stop_propagation()),
            )
            .on_mouse_down_out(cx.listener(|t, _, _, cx| t.close_menu(cx)));
        for row in rows {
            card = card.child(row);
        }
        Some(
            deferred(
                anchored()
                    .anchor(Corner::TopLeft)
                    .position(self.menu_anchor)
                    .offset(point(px(-4.0), px(6.0)))
                    .snap_to_window()
                    .child(card),
            )
            .with_priority(40)
            .into_any_element(),
        )
    }

    // --- Action handlers --------------------------------------------------

    fn on_new(&mut self, _: &New, _: &mut Window, cx: &mut Context<Self>) {
        act::guard(&mut self.app, PendingAction::NewEmpty);
        cx.notify();
    }
    fn on_open(&mut self, _: &Open, _: &mut Window, cx: &mut Context<Self>) {
        act::guard(&mut self.app, PendingAction::Open);
        cx.notify();
    }
    fn on_save(&mut self, _: &Save, _: &mut Window, cx: &mut Context<Self>) {
        act::save(&mut self.app);
        cx.notify();
    }
    fn on_save_as(&mut self, _: &SaveAs, _: &mut Window, cx: &mut Context<Self>) {
        act::save_as(&mut self.app);
        cx.notify();
    }
    fn on_undo(&mut self, _: &Undo, _: &mut Window, cx: &mut Context<Self>) {
        if self.app.undo() {
            self.app.toast("Undo");
        }
        cx.notify();
    }
    fn on_redo(&mut self, _: &Redo, _: &mut Window, cx: &mut Context<Self>) {
        if self.app.redo() {
            self.app.toast("Redo");
        }
        cx.notify();
    }
    fn on_find(&mut self, _: &Find, window: &mut Window, cx: &mut Context<Self>) {
        self.app.search.open = true;
        self.app.search.focus_request = true;
        self.menu = None;
        let _ = window;
        cx.notify();
    }
    fn on_escape(&mut self, _: &Escape, _: &mut Window, cx: &mut Context<Self>) {
        if self.menu.is_some() {
            self.menu = None;
        } else if self.dlg_combo.is_some() {
            self.dlg_combo = None;
        } else {
            match &self.app.dialog {
                Some(Dialog::Unsaved { .. }) => {}
                Some(_) => self.app.dialog = None,
                None => {
                    if self.app.search.open {
                        self.app.search.open = false;
                    } else {
                        self.app.select(None);
                        self.app.history_highlight = None;
                    }
                }
            }
        }
        cx.notify();
    }
    fn on_select_all(&mut self, _: &SelectAll, _: &mut Window, cx: &mut Context<Self>) {
        act::select_all(&mut self.app);
        cx.notify();
    }
    fn on_copy(&mut self, _: &Copy, _: &mut Window, cx: &mut Context<Self>) {
        let text = act::copy_selection(&self.app);
        if !text.is_empty() {
            cx.write_to_clipboard(gpui::ClipboardItem::new_string(text));
        }
    }
    fn on_cut(&mut self, _: &Cut, _: &mut Window, cx: &mut Context<Self>) {
        if let Some(text) = act::cut_selection(&mut self.app) {
            if !text.is_empty() {
                cx.write_to_clipboard(gpui::ClipboardItem::new_string(text));
            }
        }
        cx.notify();
    }
    fn on_paste(&mut self, _: &Paste, _: &mut Window, cx: &mut Context<Self>) {
        if let Some(text) = cx.read_from_clipboard().and_then(|i| i.text()) {
            let dna: String = text
                .chars()
                .filter(|c| {
                    c.is_ascii() && domain::iupac::dna_mask(c.to_ascii_uppercase() as u8) != 0
                })
                .collect();
            if !dna.is_empty() {
                act::insert_bases(&mut self.app, &dna, true);
            }
        }
        cx.notify();
    }
    fn on_delete(&mut self, _: &Delete, _: &mut Window, cx: &mut Context<Self>) {
        act::delete_selection(&mut self.app);
        cx.notify();
    }
    pub(crate) fn set_view(&mut self, v: View, cx: &mut Context<Self>) {
        self.app.view = v;
        self.menu = None;
        cx.notify();
    }

    fn on_key_down(&mut self, ev: &KeyDownEvent, window: &mut Window, cx: &mut Context<Self>) {
        if self.app.dialog.is_some() || self.menu.is_some() {
            return;
        }
        if !self.focus_handle.is_focused(window) || self.app.view != View::Sequence {
            return;
        }
        let k = &ev.keystroke;
        if k.modifiers.control || k.modifiers.alt || k.modifiers.platform {
            return;
        }
        if let Some(ch) = k.key_char.as_ref() {
            let dna: String = ch
                .chars()
                .filter(|c| {
                    c.is_ascii() && domain::iupac::dna_mask(c.to_ascii_uppercase() as u8) != 0
                })
                .collect();
            if !dna.is_empty() {
                act::insert_bases(&mut self.app, &dna, true);
                cx.notify();
            }
        }
    }

    // --- Chrome -----------------------------------------------------------

    fn menu_bar(&mut self, cx: &mut Context<Self>) -> AnyElement {
        let items = [
            (MenuKind::File, "File"),
            (MenuKind::Edit, "Edit"),
            (MenuKind::View, "View"),
            (MenuKind::Features, "Features"),
            (MenuKind::Primers, "Primers"),
            (MenuKind::Enzymes, "Enzymes"),
            (MenuKind::Tools, "Tools"),
            (MenuKind::Lab, "Lab"),
            (MenuKind::Help, "Help"),
        ];
        let mut bar = div()
            .flex()
            .flex_row()
            .items_center()
            .gap(px(2.0))
            .h(px(40.0))
            .px(px(8.0))
            .bg(theme::TOPBAR);
        // Brand mark
        bar = bar.child(
            div()
                .flex()
                .items_center()
                .gap(px(7.0))
                .px(px(8.0))
                .mr(px(6.0))
                .child(
                    div()
                        .w(px(18.0))
                        .h(px(18.0))
                        .rounded(px(4.0))
                        .bg(theme::ACCENT),
                )
                .child(
                    div()
                        .text_size(px(theme::T_BASE))
                        .text_color(theme::TOPBAR_TEXT)
                        .child("HelixBench"),
                ),
        );
        for (kind, label) in items {
            let active = self.menu == Some(kind);
            let d = div()
                .id(SharedString::from(format!("menu-{label}")))
                .px(px(10.0))
                .h(px(26.0))
                .flex()
                .items_center()
                .rounded(px(5.0))
                .text_size(px(theme::T_BASE))
                .cursor_pointer()
                .child(label.to_owned())
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(move |this, ev: &MouseDownEvent, _, cx| {
                        if this.menu == Some(kind) {
                            this.close_menu(cx);
                        } else {
                            this.open_menu(kind, ev.position, cx);
                        }
                    }),
                );
            bar = bar.child(if active {
                d.bg(theme::TOPBAR_TEXT.opacity(0.14))
                    .text_color(theme::TOPBAR_TEXT)
            } else {
                d.text_color(theme::TOPBAR_MUTED).hover(|s| {
                    s.bg(theme::TOPBAR_TEXT.opacity(0.08))
                        .text_color(theme::TOPBAR_TEXT)
                })
            });
        }
        bar = bar.child(
            div()
                .flex_grow()
                .flex()
                .justify_end()
                .items_center()
                .gap(px(8.0))
                .px(px(8.0))
                .child(
                    div()
                        .w(px(22.0))
                        .h(px(22.0))
                        .rounded_full()
                        .bg(theme::ACCENT)
                        .flex()
                        .items_center()
                        .justify_center()
                        .text_size(px(theme::T_XS))
                        .text_color(theme::TOPBAR_TEXT)
                        .child("L"),
                )
                .child(
                    div()
                        .text_size(px(theme::T_SM))
                        .text_color(theme::TOPBAR_MUTED)
                        .child("Local workspace"),
                ),
        );
        bar.into_any_element()
    }

    fn tab_bar(&mut self, cx: &mut Context<Self>) -> AnyElement {
        let mut row = div()
            .flex()
            .flex_row()
            .items_center()
            .gap(px(4.0))
            .h(px(40.0))
            .px(px(8.0))
            .bg(theme::PANEL)
            .border_b_1()
            .border_color(theme::BORDER);
        let name = self.app.doc.name.clone();
        let dirty = self.app.dirty;
        row = row.child(
            div()
                .id("doc-tab")
                .flex()
                .items_center()
                .gap(px(8.0))
                .h(px(30.0))
                .px(px(12.0))
                .rounded(px(6.0))
                .bg(theme::SURFACE)
                .border_1()
                .border_color(theme::BORDER)
                .child(
                    div()
                        .w(px(8.0))
                        .h(px(8.0))
                        .rounded(px(2.0))
                        .bg(theme::ACCENT),
                )
                .child(
                    div()
                        .text_size(px(theme::T_SM))
                        .text_color(theme::TEXT)
                        .child(format!("{name}{}", if dirty { "  •" } else { "" })),
                )
                .child(
                    div()
                        .id("doc-tab-close")
                        .text_size(px(theme::T_XS))
                        .text_color(theme::FAINT)
                        .cursor_pointer()
                        .hover(|s| s.text_color(theme::DANGER))
                        .child("✕")
                        .on_click(cx.listener(|this, _, window, cx| this.on_new(&New, window, cx))),
                ),
        );
        for (i, od) in self.app.library.iter().enumerate() {
            let lname = od.doc.name.clone();
            let ldirty = od.dirty;
            row = row.child(
                div()
                    .id(SharedString::from(format!("doc-tab-{i}")))
                    .flex()
                    .items_center()
                    .gap(px(8.0))
                    .h(px(30.0))
                    .px(px(12.0))
                    .rounded(px(6.0))
                    .cursor_pointer()
                    .hover(|s| s.bg(theme::SURFACE))
                    .child(
                        div()
                            .w(px(8.0))
                            .h(px(8.0))
                            .rounded(px(2.0))
                            .bg(theme::FAINT),
                    )
                    .child(
                        div()
                            .text_size(px(theme::T_SM))
                            .text_color(theme::MUTED)
                            .child(format!("{lname}{}", if ldirty { "  •" } else { "" })),
                    )
                    .on_click(cx.listener(move |this, _, _, cx| {
                        act::switch_doc(&mut this.app, i);
                        cx.notify();
                    })),
            );
        }
        row = row.child(
            widgets::icon_only("doc-new", "plus", widgets::Btn::Subtle)
                .on_click(cx.listener(|this, _, window, cx| this.on_new(&New, window, cx))),
        );
        let topo = self.app.doc.sequence.topology.label();
        row = row.child(
            div()
                .flex_grow()
                .flex()
                .justify_end()
                .items_center()
                .gap(px(10.0))
                .child(
                    div()
                        .text_size(px(theme::T_XS))
                        .text_color(theme::ACCENT)
                        .px(px(7.0))
                        .py(px(2.0))
                        .rounded(px(4.0))
                        .bg(theme::ACCENT.opacity(0.12))
                        .child(topo.to_owned()),
                ),
        );
        row.into_any_element()
    }

    fn left_rail(&mut self, cx: &mut Context<Self>) -> AnyElement {
        let mut col = div()
            .flex()
            .flex_col()
            .gap(px(2.0))
            .w(px(180.0))
            .h_full()
            .py(px(8.0))
            .px(px(8.0))
            .bg(theme::PANEL)
            .border_r_1()
            .border_color(theme::BORDER);
        col = col.child(widgets::label("Workspace", theme::T_XS, theme::FAINT));
        for v in View::LAB {
            col = col.child(self.nav_row(v, cx));
        }
        col = col.child(widgets::divider());
        col = col.child(widgets::label("Sequence", theme::T_XS, theme::FAINT));
        for v in View::ALL {
            col = col.child(self.nav_row(v, cx));
        }
        col = col.child(widgets::divider());
        let can_undo = !self.app.history.is_empty();
        let can_redo = !self.app.future.is_empty();
        let actions: [(&str, &str, u8); 8] = [
            ("rail-new", "plus", 0),
            ("rail-open", "scan", 1),
            ("rail-save", "check", 2),
            ("rail-undo", "undo", 3),
            ("rail-redo", "redo", 4),
            ("rail-find", "search", 5),
            ("rail-zoom-in", "zoom-in", 6),
            ("rail-zoom-out", "zoom-out", 7),
        ];
        let mut tools = div().flex().flex_wrap().gap(px(4.0)).pt(px(2.0));
        for (id, glyph, kind) in actions {
            let disabled = (kind == 3 && !can_undo) || (kind == 4 && !can_redo);
            tools = tools.child(
                div()
                    .id(SharedString::from(id))
                    .w(px(28.0))
                    .h(px(28.0))
                    .flex()
                    .items_center()
                    .justify_center()
                    .rounded(px(6.0))
                    .cursor_pointer()
                    .opacity(if disabled { 0.4 } else { 1.0 })
                    .hover(|s| s.bg(theme::SURFACE))
                    .child(crate::icons::icon(glyph, 16.0, theme::MUTED))
                    .on_click(cx.listener(move |this, _, window, cx| match kind {
                        0 => this.on_new(&New, window, cx),
                        1 => this.on_open(&Open, window, cx),
                        2 => this.on_save(&Save, window, cx),
                        3 => {
                            if this.app.undo() {
                                cx.notify();
                            }
                        }
                        4 => {
                            if this.app.redo() {
                                cx.notify();
                            }
                        }
                        5 => {
                            this.app.search.open = !this.app.search.open;
                            if this.app.search.open {
                                this.app.search.focus_request = true;
                            }
                            cx.notify();
                        }
                        6 => {
                            this.app.zoom_width = (this.app.zoom_width / 2).max(20);
                            this.app.center_zoom_on_selection();
                            cx.notify();
                        }
                        _ => {
                            let l = this.app.doc.sequence.len().max(1);
                            this.app.zoom_width = (this.app.zoom_width * 2).min(l);
                            this.app.center_zoom_on_selection();
                            cx.notify();
                        }
                    })),
            );
        }
        tools = tools.child(
            div()
                .id("rail-options")
                .w(px(28.0))
                .h(px(28.0))
                .flex()
                .items_center()
                .justify_center()
                .rounded(px(6.0))
                .cursor_pointer()
                .hover(|s| s.bg(theme::SURFACE))
                .child(crate::icons::icon("sliders", 16.0, theme::MUTED))
                .on_click(cx.listener(|this, _, _, cx| {
                    this.app.dialog = Some(Dialog::Options);
                    cx.notify();
                })),
        );
        col = col.child(tools);
        col.into_any_element()
    }

    fn nav_row(&mut self, v: View, cx: &mut Context<Self>) -> gpui::Stateful<gpui::Div> {
        let active = self.app.view == v;
        div()
            .id(SharedString::from(format!("nav-{}", v.label())))
            .h(px(26.0))
            .flex()
            .items_center()
            .gap(px(8.0))
            .px(px(8.0))
            .rounded(px(6.0))
            .cursor_pointer()
            .bg(if active {
                theme::SURFACE_2
            } else {
                theme::transparent()
            })
            .hover(|s| s.bg(theme::SURFACE))
            .child(crate::icons::icon(
                crate::icons::view_icon(v),
                14.0,
                if active { theme::ACCENT } else { theme::MUTED },
            ))
            .child(
                div()
                    .text_size(px(theme::T_SM))
                    .text_color(if active { theme::TEXT } else { theme::MUTED })
                    .child(v.label()),
            )
            .on_click(cx.listener(move |this, _, _, cx| this.set_view(v, cx)))
    }

    pub fn toggle_pref(&mut self, key: &str) {
        let p = &mut self.app.prefs;
        match key {
            "show_features" => p.show_features = !p.show_features,
            "show_enzymes" => p.show_enzymes = !p.show_enzymes,
            "show_primers" => p.show_primers = !p.show_primers,
            "show_orfs" => p.show_orfs = !p.show_orfs,
            "show_translations" => p.show_translations = !p.show_translations,
            "show_complement" => p.show_complement = !p.show_complement,
            "show_colors" => p.show_colors = !p.show_colors,
            "show_zoom_controls" => p.show_zoom_controls = !p.show_zoom_controls,
            _ => {}
        }
        self.app.analysis_dirty = true;
    }

    fn status_bar(&mut self, cx: &mut Context<Self>) -> AnyElement {
        let app = &self.app;
        let l = app.doc.sequence.len();
        let gc = gc_percent(&app.doc.sequence.bases);
        let sel_text = if let Some(sel) = app.selection {
            let bases = app.doc.sequence.region(sel);
            let sgc = gc_percent(&bases);
            let (a, b) = (sel.start + 1, sel.start + sel.len);
            let rng = if sel.len == 1 {
                format!("{a}")
            } else {
                format!("{a}..{b}")
            };
            format!("Sel {rng} · {} bp · GC {sgc:.1}%", sel.len)
        } else {
            "No selection".into()
        };
        let sel_color = if app.selection.is_some() {
            theme::ACCENT
        } else {
            theme::FAINT
        };
        let counts = format!(
            "{} features · {} primers · {} enzymes",
            app.doc.features.len(),
            app.doc.primers.len(),
            app.analysis.chosen.len()
        );
        let (save_label, save_color) = if app.dirty {
            ("● Unsaved", theme::WARN)
        } else if app.path.is_some() {
            ("✓ Saved", theme::OK)
        } else {
            ("Demo", theme::FAINT)
        };
        let mut bar = div()
            .flex()
            .items_center()
            .gap(px(14.0))
            .h(px(30.0))
            .px(px(14.0))
            .bg(theme::SURFACE)
            .border_t_1()
            .border_color(theme::BORDER)
            .text_size(px(theme::T_SM))
            .child(div().text_color(sel_color).child(sel_text))
            .child(dot())
            .child(div().text_color(theme::MUTED).child(format!(
                "{l} bp · {} · GC {gc:.1}%",
                app.doc.sequence.topology.label()
            )))
            .child(dot())
            .child(div().text_color(theme::MUTED).child(counts));
        if let Some(p) = &app.path {
            bar = bar.child(
                div()
                    .flex_grow()
                    .flex()
                    .justify_end()
                    .text_color(theme::FAINT)
                    .text_size(px(theme::T_XS))
                    .child(p.display().to_string()),
            );
        } else {
            bar = bar.child(div().flex_grow());
        }
        bar = bar.child(div().text_color(save_color).child(save_label.to_owned()));
        let _ = cx;
        bar.into_any_element()
    }

    fn toast(&mut self, cx: &mut Context<Self>) -> Option<AnyElement> {
        let (msg, when) = self.app.toast.clone()?;
        let elapsed = when.elapsed().as_secs_f32();
        if elapsed > 3.5 {
            self.app.toast = None;
            self.toast_armed = None;
            return None;
        }
        let alpha = if elapsed > 3.0 {
            ((3.5 - elapsed) / 0.5).clamp(0.0, 1.0)
        } else {
            1.0
        };
        // Arm exactly one ticker per toast: it repaints for the fade and then
        // clears itself, so unrelated repaints never spawn extra timers.
        if self.toast_armed != Some(when) {
            self.toast_armed = Some(when);
            cx.spawn(async move |this, cx| {
                // Idle while fully visible: repainting every 50ms here caused
                // continuous re-layout that swallowed mouse clicks elsewhere.
                cx.background_executor()
                    .timer(std::time::Duration::from_millis(3000))
                    .await;
                loop {
                    cx.background_executor()
                        .timer(std::time::Duration::from_millis(50))
                        .await;
                    let cont = this
                        .update(cx, |this, cx| match &this.app.toast {
                            Some((_, w)) if *w == when => {
                                if w.elapsed().as_secs_f32() > 3.5 {
                                    this.app.toast = None;
                                    this.toast_armed = None;
                                }
                                cx.notify();
                                this.app.toast.is_some()
                            }
                            _ => {
                                this.toast_armed = None;
                                false
                            }
                        })
                        .unwrap_or(false);
                    if !cont {
                        break;
                    }
                }
            })
            .detach();
        }
        Some(
            deferred(
                div()
                    .absolute()
                    .bottom(px(56.0))
                    .left_0()
                    .right_0()
                    .flex()
                    .justify_center()
                    .child(
                        div()
                            .max_w(px(460.0))
                            .px(px(16.0))
                            .py(px(10.0))
                            .rounded(px(8.0))
                            .bg(theme::SURFACE_2)
                            .border_1()
                            .border_color(theme::ACCENT.opacity(0.7))
                            .text_size(px(theme::T_BASE))
                            .text_color(theme::TEXT.opacity(alpha))
                            .child(msg),
                    ),
            )
            .with_priority(30)
            .into_any_element(),
        )
    }

    fn central_view(&mut self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        match self.app.view {
            View::Map => crate::editor::render_editor(self, cx),
            View::Sequence => self.render_sequence(cx),
            View::Features => self.render_features(window, cx),
            View::Primers => self.render_primers(window, cx),
            View::Enzymes => self.render_enzymes(window, cx),
            View::History => self.render_history(window, cx),
            View::Home => crate::lab::render_home(self, cx),
            View::Notebook => crate::lab::render_notebook(self, cx),
            View::Registry => crate::lab::render_registry(self, cx),
            View::Inventory => crate::lab::render_inventory(self, cx),
            View::Workflows => crate::lab::render_workflows(self, cx),
            View::Insights => crate::lab::render_insights(self, cx),
            View::Automation => crate::lab::render_automation(self, cx),
            View::Split => crate::lab::render_split(self, cx),
        }
    }

    fn update_title(&mut self, window: &mut Window) {
        let title = format!(
            "HelixBench — {}{}",
            self.app.doc.name,
            if self.app.dirty { " •" } else { "" }
        );
        if title != self.last_title {
            window.set_window_title(&title);
            self.last_title = title;
        }
    }
}

impl Focusable for HelixBench {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for HelixBench {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.app.ensure_analysis();

        if !self.initialized {
            self.initialized = true;
            self.focus_root(window);
        }
        if self.app.exit_requested {
            self.app.exit_requested = false;
            act::persist_config(&mut self.app);
            cx.quit();
        }
        // Reseed dialog-scoped text fields whenever the dialog identity changes.
        let sig = self.dialog_signature();
        if sig != self.last_dialog_sig {
            self.inputs.retain(|k, _| !k.starts_with("dlg:"));
            self.dlg_scroll = ScrollHandle::new();
            self.last_dialog_sig = sig;
        }
        self.viewport = window.viewport_size();
        self.update_title(window);

        let central = self.central_view(window, cx);
        let toolbar = self.left_rail(cx);
        let menu_overlay = self.render_menu(cx);
        let dialog_overlay = crate::dialogs::show(self, window, cx);
        let toast = self.toast(cx);
        let find_bar = if self.app.search.open {
            Some(crate::find::show(self, window, cx))
        } else {
            None
        };

        let mut root = div()
            .key_context("HelixBench")
            .track_focus(&self.focus_handle)
            .size_full()
            .flex()
            .flex_col()
            .bg(theme::BG)
            .text_color(theme::TEXT)
            .text_size(px(theme::T_BASE))
            .on_action(cx.listener(Self::on_new))
            .on_action(cx.listener(Self::on_open))
            .on_action(cx.listener(Self::on_save))
            .on_action(cx.listener(Self::on_save_as))
            .on_action(cx.listener(Self::on_undo))
            .on_action(cx.listener(Self::on_redo))
            .on_action(cx.listener(Self::on_find))
            .on_action(cx.listener(Self::on_escape))
            .on_action(cx.listener(Self::on_select_all))
            .on_action(cx.listener(Self::on_copy))
            .on_action(cx.listener(Self::on_cut))
            .on_action(cx.listener(Self::on_paste))
            .on_action(cx.listener(Self::on_delete))
            .on_action(cx.listener(|this, _: &View1, _, cx| this.set_view(View::Map, cx)))
            .on_action(cx.listener(|this, _: &View2, _, cx| this.set_view(View::Sequence, cx)))
            .on_action(cx.listener(|this, _: &View3, _, cx| this.set_view(View::Features, cx)))
            .on_action(cx.listener(|this, _: &View4, _, cx| this.set_view(View::Primers, cx)))
            .on_action(cx.listener(|this, _: &View5, _, cx| this.set_view(View::Enzymes, cx)))
            .on_action(cx.listener(|this, _: &View6, _, cx| this.set_view(View::History, cx)))
            .on_key_down(cx.listener(Self::on_key_down))
            .child(self.menu_bar(cx))
            .child(self.tab_bar(cx));
        if let Some(fb) = find_bar {
            root = root.child(fb);
        }
        root = root
            .child(
                div()
                    .flex_grow()
                    .flex()
                    .flex_row()
                    .overflow_hidden()
                    .child(toolbar)
                    .child(div().flex_grow().h_full().overflow_hidden().child(central)),
            )
            .child(self.status_bar(cx));
        if let Some(m) = menu_overlay {
            root = root.child(m);
        }
        if let Some(d) = dialog_overlay {
            root = root.child(d);
        }
        if let Some(t) = toast {
            root = root.child(t);
        }
        root
    }
}

fn dot() -> gpui::Div {
    div()
        .w(px(3.0))
        .h(px(3.0))
        .rounded(px(2.0))
        .bg(theme::BORDER_STRONG)
}

pub fn gc_percent(bases: &[u8]) -> f32 {
    if bases.is_empty() {
        return 0.0;
    }
    let gc = bases.iter().filter(|&&b| b == b'G' || b == b'C').count();
    gc as f32 / bases.len() as f32 * 100.0
}
