//! Modal dialogs. Openers seed drafts into `AppState`; `show` presents them over
//! a scrim as centered cards. Text fields use the shell's pull-model inputs;
//! dropdowns reuse the anchored-popup pattern; long lists scroll with the
//! palette-matched scrollbar. All document logic lives in `appcore::actions`.

use gpui::{
    anchored, canvas, deferred, div, point, prelude::*, px, size, uniform_list, AnyElement, Bounds,
    Context, Corner, Div, Entity, FontWeight, MouseButton, MouseDownEvent, Pixels, SharedString,
    Stateful, Window,
};

use appcore::actions as act;
use appcore::state::{
    AppState, ColorsMode, Dialog, FeatureDraft, PcrDraft, PendingAction, PrimerDraft,
};
use domain::feature::{kind_color, Direction, Feature, FEATURE_KINDS};
use domain::genetic_code::{code_by_id, CODES};
use domain::primer::{binding_sites, melting_temperature, HybridizationParams, Primer};
use domain::sequence::clean_sequence;
use domain::{Strand, StrandMask, Topology};

use crate::app::HelixBench;
use crate::draw::PxVal;
use crate::theme;
use crate::views::{check_box, ScrollbarDecoration};
use crate::widgets::{self, Btn};

// ---------------------------------------------------------------------------
// Openers
// ---------------------------------------------------------------------------

pub fn open_feature_editor(app: &mut AppState, id: Option<u64>) {
    let draft = if let Some(f) = id.and_then(|i| app.doc.feature(i)) {
        FeatureDraft {
            id: Some(f.id),
            name: f.name.clone(),
            kind: f.kind.clone(),
            segments_text: f
                .segments
                .iter()
                .map(|s| format!("{}..{}", s.start + 1, s.start + s.len))
                .collect::<Vec<_>>()
                .join(", "),
            direction: f.direction,
            color: f.color,
            note: f.note.clone(),
            product: f.product.clone(),
            translated: f.translated,
            genetic_code: f.genetic_code,
            first_codon_met: f.first_codon_met,
            frame_offset: f.frame_offset,
            error: None,
        }
    } else {
        let (seg_text, direction) = match app.selection {
            Some(sel) if sel.len > 0 => (
                format!("{}..{}", sel.start + 1, sel.start + sel.len),
                Direction::Forward,
            ),
            _ => (String::new(), Direction::None),
        };
        FeatureDraft {
            id: None,
            name: String::new(),
            kind: "misc_feature".into(),
            segments_text: seg_text,
            direction,
            color: kind_color("misc_feature"),
            note: String::new(),
            product: String::new(),
            translated: false,
            genetic_code: 0,
            first_codon_met: false,
            frame_offset: 0,
            error: None,
        }
    };
    app.dialog = Some(Dialog::FeatureEditor(draft));
}

pub fn open_primer_editor(app: &mut AppState, id: Option<u64>) {
    let draft = if let Some(p) = id.and_then(|i| app.doc.primer(i)) {
        PrimerDraft {
            id: Some(p.id),
            name: p.name.clone(),
            sequence: String::from_utf8_lossy(&p.sequence).into_owned(),
            description: p.description.clone(),
            color: p.color,
            phosphorylated: p.phosphorylated,
            error: None,
        }
    } else {
        PrimerDraft {
            id: None,
            name: format!("Primer {}", app.doc.primers.len() + 1),
            sequence: String::new(),
            description: String::new(),
            color: [0x79, 0xc0, 0xff],
            phosphorylated: false,
            error: None,
        }
    };
    app.dialog = Some(Dialog::PrimerEditor(draft));
}

pub fn open_detect_common(app: &mut AppState) {
    app.ensure_analysis();
    let detected = domain::common_features::detect(&app.doc.sequence);
    let checked: Vec<bool> = detected
        .iter()
        .map(|d| !domain::common_features::is_duplicate(&app.doc.features, d))
        .collect();
    if detected.is_empty() {
        app.toast("No known common features found in this sequence");
        return;
    }
    app.dialog = Some(Dialog::DetectCommon { detected, checked });
}

// ---------------------------------------------------------------------------
// Dispatch
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum DlgComboKind {
    FeatureKind,
    FeatureCode,
    OptionsCode,
    CodesTable,
}

pub fn show(
    app: &mut HelixBench,
    window: &mut Window,
    cx: &mut Context<HelixBench>,
) -> Option<AnyElement> {
    if app.app.dialog.is_none() {
        app.dlg_combo = None;
        return None;
    }
    let max_h = (window.viewport_size().height - px(64.0)).max(px(360.0));

    let card = if matches!(app.app.dialog, Some(Dialog::NewSequence(_))) {
        new_sequence(app, cx, max_h)
    } else if matches!(app.app.dialog, Some(Dialog::FeatureEditor(_))) {
        feature_editor(app, cx, max_h)
    } else if matches!(app.app.dialog, Some(Dialog::PrimerEditor(_))) {
        primer_editor(app, cx, max_h)
    } else if matches!(app.app.dialog, Some(Dialog::GeneticCodes { .. })) {
        genetic_codes(app, cx, max_h)
    } else if matches!(app.app.dialog, Some(Dialog::LetterCodes)) {
        letter_codes(app, cx, max_h)
    } else if matches!(app.app.dialog, Some(Dialog::SearchTips)) {
        search_tips(app, cx, max_h)
    } else if matches!(app.app.dialog, Some(Dialog::About)) {
        about(app, cx, max_h)
    } else if matches!(app.app.dialog, Some(Dialog::Options)) {
        options(app, cx, max_h)
    } else if matches!(app.app.dialog, Some(Dialog::Rotate { .. })) {
        rotate(app, cx, max_h)
    } else if matches!(app.app.dialog, Some(Dialog::Unsaved { .. })) {
        unsaved(app, cx, max_h)
    } else if matches!(app.app.dialog, Some(Dialog::DetectCommon { .. })) {
        detect_common(app, cx, max_h)
    } else if matches!(app.app.dialog, Some(Dialog::ColorSelection(_))) {
        color_selection(app, cx, max_h)
    } else if matches!(app.app.dialog, Some(Dialog::Pcr(_))) {
        pcr(app, cx, max_h)
    } else if matches!(app.app.dialog, Some(Dialog::PrimerDesign(_))) {
        primer_design(app, cx, max_h)
    } else if matches!(app.app.dialog, Some(Dialog::Digest)) {
        digest(app, cx, max_h)
    } else if matches!(app.app.dialog, Some(Dialog::Alignment(_))) {
        alignment(app, cx, max_h)
    } else if matches!(app.app.dialog, Some(Dialog::Assembly(_))) {
        assembly(app, cx, max_h)
    } else if matches!(app.app.dialog, Some(Dialog::Crispr(_))) {
        crispr(app, cx, max_h)
    } else if matches!(app.app.dialog, Some(Dialog::Codon(_))) {
        codon(app, cx, max_h)
    } else if matches!(app.app.dialog, Some(Dialog::Blast(_))) {
        blast(app, cx, max_h)
    } else if matches!(app.app.dialog, Some(Dialog::Combinatorial(_))) {
        combinatorial(app, cx, max_h)
    } else if matches!(app.app.dialog, Some(Dialog::Registry(_))) {
        registry(app, cx, max_h)
    } else if matches!(app.app.dialog, Some(Dialog::RegistryEditor(_))) {
        registry_editor(app, cx, max_h)
    } else if matches!(app.app.dialog, Some(Dialog::Protein(_))) {
        protein(app, cx, max_h)
    } else if matches!(app.app.dialog, Some(Dialog::FeatureLib(_))) {
        feature_lib(app, cx, max_h)
    } else if matches!(app.app.dialog, Some(Dialog::Workspace(_))) {
        workspace(app, cx, max_h)
    } else if matches!(app.app.dialog, Some(Dialog::Notebook(_))) {
        notebook(app, cx, max_h)
    } else if matches!(app.app.dialog, Some(Dialog::EntryEditor(_))) {
        entry_editor(app, cx, max_h)
    } else if matches!(app.app.dialog, Some(Dialog::Projects(_))) {
        projects(app, cx, max_h)
    } else if matches!(app.app.dialog, Some(Dialog::Audit(_))) {
        audit(app, cx, max_h)
    } else if matches!(app.app.dialog, Some(Dialog::Home(_))) {
        home(app, cx, max_h)
    } else if matches!(app.app.dialog, Some(Dialog::EntityMap(_))) {
        entity_map(app, cx, max_h)
    } else if matches!(app.app.dialog, Some(Dialog::RegistryImport(_))) {
        registry_import(app, cx, max_h)
    } else if matches!(app.app.dialog, Some(Dialog::Inventory(_))) {
        inventory(app, cx, max_h)
    } else if matches!(app.app.dialog, Some(Dialog::Workflows(_))) {
        workflows(app, cx, max_h)
    } else if matches!(app.app.dialog, Some(Dialog::Biologics(_))) {
        biologics(app, cx, max_h)
    } else if matches!(app.app.dialog, Some(Dialog::ChemAdd(_))) {
        chem_add(app, cx, max_h)
    } else if matches!(app.app.dialog, Some(Dialog::ChemReaction(_))) {
        chem_reaction(app, cx, max_h)
    } else if matches!(app.app.dialog, Some(Dialog::ChemLot(_))) {
        chem_lot(app, cx, max_h)
    } else if matches!(app.app.dialog, Some(Dialog::Rna(_))) {
        rna(app, cx, max_h)
    } else if matches!(app.app.dialog, Some(Dialog::IvStudy(_))) {
        iv_study(app, cx, max_h)
    } else if matches!(app.app.dialog, Some(Dialog::IvAnimal(_))) {
        iv_animal(app, cx, max_h)
    } else if matches!(app.app.dialog, Some(Dialog::IvMeasure(_))) {
        iv_measure(app, cx, max_h)
    } else if matches!(app.app.dialog, Some(Dialog::BpRecipe(_))) {
        bp_recipe(app, cx, max_h)
    } else if matches!(app.app.dialog, Some(Dialog::BpReplicate(_))) {
        bp_replicate(app, cx, max_h)
    } else {
        return None;
    };

    let mut wrapper = div();
    wrapper = wrapper.child(card);
    if let Some(combo) = render_dlg_combo(app, cx) {
        wrapper = wrapper.child(combo);
    }
    Some(wrapper.into_any_element())
}

fn close_dialog(app: &mut HelixBench, cx: &mut Context<HelixBench>) {
    app.app.dialog = None;
    app.dlg_combo = None;
    cx.notify();
}

// ---------------------------------------------------------------------------
// Shared chrome
// ---------------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn modal_shell(
    app: &mut HelixBench,
    cx: &mut Context<HelixBench>,
    title: &str,
    width: f32,
    max_h: Pixels,
    dismissable: bool,
    body: AnyElement,
    footer: Option<AnyElement>,
) -> AnyElement {
    let mut card = div()
        .flex()
        .flex_col()
        .w(px(width))
        .max_h(max_h)
        .bg(theme::OVERLAY)
        .border_1()
        .border_color(theme::BORDER_STRONG)
        .rounded(px(10.0))
        .shadow(theme::shadow_dialog())
        .overflow_hidden()
        .on_mouse_down(
            MouseButton::Left,
            cx.listener(|_, _, _, cx| cx.stop_propagation()),
        );

    let mut title_bar = div()
        .flex()
        .items_center()
        .justify_between()
        .h(px(46.0))
        .flex_shrink_0()
        .px(px(16.0))
        .border_b_1()
        .border_color(theme::BORDER)
        .child(
            div()
                .text_size(px(theme::T_MD))
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(theme::TEXT)
                .child(title.to_owned()),
        );
    if dismissable {
        title_bar = title_bar.child(
            div()
                .id("dlg-close")
                .w(px(26.0))
                .h(px(26.0))
                .flex()
                .items_center()
                .justify_center()
                .rounded(px(5.0))
                .cursor_pointer()
                .text_color(theme::MUTED)
                .hover(|s| s.bg(theme::SURFACE_2).text_color(theme::TEXT))
                .child(crate::icons::icon("x", 13.0, theme::MUTED))
                .on_click(cx.listener(|t, _, _, cx| close_dialog(t, cx))),
        );
    }
    card = card.child(title_bar);
    let handle = app.dlg_scroll.clone();
    let rail_handle = handle.clone();
    card = card.child(
        div()
            .flex_1()
            .min_h(px(0.0))
            .flex()
            .flex_row()
            .overflow_hidden()
            .child(
                div()
                    .id("dlg-body")
                    .flex_1()
                    .min_w(px(0.0))
                    .overflow_y_scroll()
                    .track_scroll(&handle)
                    .p(px(16.0))
                    .child(body),
            )
            .child(
                canvas(
                    move |_b, _w, _cx| {},
                    move |b, _s, window, _cx| {
                        let offset = -rail_handle.offset().y.v();
                        let viewport = rail_handle.bounds().size.height.v();
                        let content = viewport + rail_handle.max_offset().height.v();
                        let rail_w = px(8.0);
                        let track = Bounds::new(
                            point(b.right() - rail_w - px(2.0), b.top() + px(2.0)),
                            size(rail_w, (b.size.height - px(4.0)).max(px(0.0))),
                        );
                        crate::draw::vscrollbar(window, track, offset, viewport, content);
                    },
                )
                .w(px(10.0))
                .h_full(),
            ),
    );
    if let Some(f) = footer {
        card = card.child(
            div()
                .flex_shrink_0()
                .px(px(16.0))
                .py(px(12.0))
                .border_t_1()
                .border_color(theme::BORDER)
                .child(f),
        );
    }

    // The scrim is sized to the window explicitly and anchored at the window
    // origin: `deferred` keeps layout in-flow, so a `size_full` absolute child
    // would collapse to a zero-height slot at the end of the root column.
    let scrim = div()
        .id("dlg-scrim")
        .w(app.viewport.width)
        .h(app.viewport.height)
        .bg(theme::SCRIM)
        .flex()
        .items_center()
        .justify_center()
        .on_click(cx.listener(move |t, _, _, cx| {
            if dismissable {
                close_dialog(t, cx);
            }
        }))
        .child(card);
    deferred(
        anchored()
            .anchor(Corner::TopLeft)
            .position(point(px(0.0), px(0.0)))
            .snap_to_window()
            .child(scrim),
    )
    .with_priority(50)
    .into_any_element()
}

fn field_label(text: &str) -> Div {
    div()
        .w(px(104.0))
        .flex_shrink_0()
        .text_size(px(theme::T_SM))
        .text_color(theme::MUTED)
        .child(text.to_owned())
}

fn field_row(label: &str, control: AnyElement) -> Div {
    div()
        .flex()
        .items_center()
        .gap(px(10.0))
        .mb(px(8.0))
        .child(field_label(label))
        .child(control)
}

fn input_field(inp: Entity<crate::text_input::TextInput>, w: f32) -> Div {
    div().w(px(w)).child(inp)
}

fn section_heading(text: &str) -> Div {
    div()
        .mt(px(2.0))
        .mb(px(6.0))
        .text_size(px(theme::T_XS))
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(theme::ACCENT)
        .child(text.to_owned())
}

fn error_label(e: &str) -> Div {
    div()
        .mt(px(6.0))
        .flex()
        .items_center()
        .gap(px(6.0))
        .text_size(px(theme::T_SM))
        .text_color(theme::DANGER)
        .child(crate::icons::icon("alert", 13.0, theme::DANGER))
        .child(e.to_owned())
}

fn seg(id: impl Into<gpui::ElementId>, label: &str, active: bool) -> Stateful<Div> {
    let d = div()
        .id(id)
        .px(px(10.0))
        .h(px(26.0))
        .flex()
        .items_center()
        .rounded(px(5.0))
        .border_1()
        .text_size(px(theme::T_SM))
        .cursor_pointer()
        .child(label.to_owned());
    if active {
        d.bg(theme::ACCENT.opacity(0.16))
            .border_color(theme::ACCENT)
            .text_color(theme::TEXT)
    } else {
        d.bg(theme::SURFACE)
            .border_color(theme::BORDER)
            .text_color(theme::MUTED)
            .hover(|s| s.bg(theme::SURFACE_2).text_color(theme::TEXT))
    }
}

fn combo_button(id: impl Into<gpui::ElementId>, label: &str, w: f32) -> Stateful<Div> {
    div()
        .id(id)
        .w(px(w))
        .h(px(30.0))
        .flex()
        .items_center()
        .justify_between()
        .gap(px(8.0))
        .px(px(10.0))
        .rounded(px(6.0))
        .bg(theme::SURFACE)
        .border_1()
        .border_color(theme::BORDER_STRONG)
        .cursor_pointer()
        .text_size(px(theme::T_SM))
        .text_color(theme::TEXT)
        .hover(|s| s.bg(theme::SURFACE_2))
        .child(div().truncate().child(label.to_owned()))
        .child(div().text_color(theme::FAINT).child("▾"))
}

fn check_row(id: &str, text: &str, checked: bool) -> Stateful<Div> {
    div()
        .id(SharedString::from(id.to_owned()))
        .flex()
        .items_center()
        .gap(px(8.0))
        .h(px(26.0))
        .cursor_pointer()
        .text_size(px(theme::T_SM))
        .text_color(if checked { theme::TEXT } else { theme::MUTED })
        .child(check_box(SharedString::from(format!("{id}-b")), checked))
        .child(text.to_owned())
}

fn footer_row(children: Vec<AnyElement>) -> Div {
    let mut d = div().flex().justify_end().items_center().gap(px(8.0));
    for c in children {
        d = d.child(c);
    }
    d
}

const SWATCHES: [[u8; 3]; 12] = [
    [0xf8, 0x51, 0x49],
    [0xe8, 0xb1, 0x3c],
    [0x57, 0xd9, 0x8a],
    [0x33, 0xc6, 0xb3],
    [0x5a, 0xa2, 0xf0],
    [0x79, 0xc0, 0xff],
    [0xa3, 0x71, 0xf7],
    [0xf7, 0x78, 0xb8],
    [0xff, 0xa6, 0x57],
    [0x8b, 0x97, 0xa7],
    [0x46, 0xc4, 0x6e],
    [0xe6, 0xeb, 0xf2],
];

#[derive(Clone, Copy)]
enum ColorTarget {
    Feature,
    Primer,
    ColorSel,
}

fn color_swatches(cx: &mut Context<HelixBench>, current: [u8; 3], target: ColorTarget) -> Div {
    let tag = match target {
        ColorTarget::Feature => "f",
        ColorTarget::Primer => "p",
        ColorTarget::ColorSel => "c",
    };
    let mut row = div().flex().flex_wrap().items_center().gap(px(5.0)).child(
        div()
            .w(px(22.0))
            .h(px(22.0))
            .rounded(px(5.0))
            .bg(theme::rgb_arr(current))
            .border_1()
            .border_color(theme::BORDER_STRONG)
            .mr(px(3.0)),
    );
    for (i, c) in SWATCHES.iter().enumerate() {
        let c = *c;
        let active = c == current;
        row = row.child(
            div()
                .id(SharedString::from(format!("sw-{tag}-{i}")))
                .w(px(20.0))
                .h(px(20.0))
                .rounded(px(5.0))
                .cursor_pointer()
                .bg(theme::rgb_arr(c))
                .border_2()
                .border_color(if active {
                    theme::TEXT
                } else {
                    theme::transparent()
                })
                .hover(|s| s.border_color(theme::TEXT.opacity(0.6)))
                .on_click(cx.listener(move |t, _, _, cx| {
                    match target {
                        ColorTarget::Feature => {
                            if let Some(Dialog::FeatureEditor(d)) = &mut t.app.dialog {
                                d.color = c;
                            }
                        }
                        ColorTarget::Primer => {
                            if let Some(Dialog::PrimerEditor(d)) = &mut t.app.dialog {
                                d.color = c;
                            }
                        }
                        ColorTarget::ColorSel => {
                            if let Some(Dialog::ColorSelection(d)) = &mut t.app.dialog {
                                d.color = c;
                            }
                        }
                    }
                    cx.notify();
                })),
        );
    }
    row
}

// ---------------------------------------------------------------------------
// New sequence
// ---------------------------------------------------------------------------

fn new_sequence(app: &mut HelixBench, cx: &mut Context<HelixBench>, max_h: Pixels) -> AnyElement {
    let (seed_name, seed_text, topology) = match &app.app.dialog {
        Some(Dialog::NewSequence(s)) => (s.name.clone(), s.text.clone(), s.topology),
        _ => (String::new(), String::new(), Topology::Linear),
    };
    let name_inp = app.input_entity(cx, "dlg:new:name", &seed_name, "Sequence name", false);
    let text_inp = app.input_entity(
        cx,
        "dlg:new:text",
        &seed_text,
        "ATGCCC…  paste letters only",
        true,
    );
    let name = name_inp.read(cx).text();
    let text = text_inp.read(cx).text();
    let cleaned = clean_sequence(&text);
    if let Some(Dialog::NewSequence(s)) = &mut app.app.dialog {
        s.name = name;
        s.text = text;
    }

    let mut body = div().flex().flex_col().size_full();
    body = body.child(field_row(
        "Name",
        input_field(name_inp, 380.0).into_any_element(),
    ));

    let mut topo_row = div().flex().gap(px(6.0));
    for (t, label) in [
        (Topology::Linear, "Linear"),
        (Topology::Circular, "Circular"),
    ] {
        topo_row = topo_row.child(
            seg(
                SharedString::from(format!("new-topo-{label}")),
                label,
                topology == t,
            )
            .on_click(cx.listener(move |tst, _, _, cx| {
                if let Some(Dialog::NewSequence(s)) = &mut tst.app.dialog {
                    s.topology = t;
                }
                cx.notify();
            })),
        );
    }
    body = body.child(field_row("Topology", topo_row.into_any_element()));

    body = body.child(section_heading("Sequence (paste or type — letters only)"));
    body = body.child(div().w_full().mb(px(6.0)).child(text_inp));
    let mut stats = div().flex().gap(px(14.0)).child(
        div()
            .font_family(theme::MONO_FONT)
            .text_size(px(theme::T_SM))
            .text_color(theme::MUTED)
            .child(format!("{} bp", cleaned.len())),
    );
    if !cleaned.is_empty() {
        let gc = crate::app::gc_percent(&cleaned);
        stats = stats.child(
            div()
                .font_family(theme::MONO_FONT)
                .text_size(px(theme::T_SM))
                .text_color(theme::MUTED)
                .child(format!("GC {gc:.1}%")),
        );
    }
    body = body.child(stats);
    let err = match &app.app.dialog {
        Some(Dialog::NewSequence(s)) => s.error.clone(),
        _ => None,
    };
    if let Some(e) = err {
        body = body.child(error_label(&e));
    }

    let footer = footer_row(vec![
        widgets::button("new-cancel", "Cancel", Btn::Ghost)
            .on_click(cx.listener(|t, _, _, cx| close_dialog(t, cx)))
            .into_any_element(),
        widgets::button("new-create", "Create", Btn::Primary)
            .on_click(cx.listener(|t, _, _, cx| {
                let state = match &t.app.dialog {
                    Some(Dialog::NewSequence(s)) => appcore::state::NewSeqState {
                        name: s.name.clone(),
                        text: s.text.clone(),
                        topology: s.topology,
                        error: None,
                    },
                    _ => return,
                };
                match act::create_from_dialog(&mut t.app, &state) {
                    Ok(()) => close_dialog(t, cx),
                    Err(e) => {
                        if let Some(Dialog::NewSequence(s)) = &mut t.app.dialog {
                            s.error = Some(e);
                        }
                        cx.notify();
                    }
                }
            }))
            .into_any_element(),
    ]);
    modal_shell(
        app,
        cx,
        "New Sequence",
        520.0,
        max_h,
        true,
        body.into_any_element(),
        Some(footer.into_any_element()),
    )
}

// ---------------------------------------------------------------------------
// Feature editor
// ---------------------------------------------------------------------------

fn feature_editor(app: &mut HelixBench, cx: &mut Context<HelixBench>, max_h: Pixels) -> AnyElement {
    let mut d = match &app.app.dialog {
        Some(Dialog::FeatureEditor(d)) => d.clone(),
        _ => return div().into_any_element(),
    };
    let l = app.app.doc.sequence.len();
    let circular = app.app.doc.sequence.is_circular();
    let default_code = app.app.doc.default_genetic_code;

    let name_inp = app.input_entity(
        cx,
        "dlg:feat:name",
        &d.name,
        "e.g. GFP, T7 promoter…",
        false,
    );
    let seg_inp = app.input_entity(
        cx,
        "dlg:feat:seg",
        &d.segments_text,
        "100..200, 300..400  (1-based)",
        true,
    );
    let prod_inp = app.input_entity(
        cx,
        "dlg:feat:prod",
        &d.product,
        "protein / product name",
        false,
    );
    let note_inp = app.input_entity(cx, "dlg:feat:note", &d.note, "optional note", false);
    d.name = name_inp.read(cx).text();
    d.segments_text = seg_inp.read(cx).text();
    d.product = prod_inp.read(cx).text();
    d.note = note_inp.read(cx).text();
    app.app.dialog = Some(Dialog::FeatureEditor(d.clone()));

    let mut body = div().flex().flex_col().size_full();
    body = body.child(field_row(
        "Name",
        input_field(name_inp, 372.0).into_any_element(),
    ));

    let kind_btn = combo_button("feat-kind", &d.kind, 190.0).on_mouse_down(
        MouseButton::Left,
        cx.listener(|t, ev: &MouseDownEvent, _, cx| {
            t.dlg_combo = Some(DlgComboKind::FeatureKind);
            t.dlg_combo_anchor = ev.position;
            cx.notify();
        }),
    );
    let kind_row = div()
        .flex()
        .items_center()
        .gap(px(10.0))
        .child(kind_btn)
        .child(
            div()
                .text_size(px(theme::T_SM))
                .text_color(theme::MUTED)
                .child("Color"),
        )
        .child(color_swatches(cx, d.color, ColorTarget::Feature));
    body = body.child(field_row("Kind", kind_row.into_any_element()));

    let mut dir_row = div().flex().gap(px(6.0));
    for (dir, label) in [
        (Direction::None, "— none"),
        (Direction::Forward, "→ forward"),
        (Direction::Reverse, "← reverse"),
        (Direction::Bidirectional, "↔ both"),
    ] {
        dir_row = dir_row.child(
            seg(
                SharedString::from(format!("feat-dir-{label}")),
                label,
                d.direction == dir,
            )
            .on_click(cx.listener(move |t, _, _, cx| {
                if let Some(Dialog::FeatureEditor(dd)) = &mut t.app.dialog {
                    dd.direction = dir;
                }
                cx.notify();
            })),
        );
    }
    body = body.child(field_row("Direction", dir_row.into_any_element()));
    body = body.child(field_row(
        "Segments",
        input_field(seg_inp, 372.0).into_any_element(),
    ));
    body = body.child(field_row(
        "Product",
        input_field(prod_inp, 372.0).into_any_element(),
    ));
    body = body.child(field_row(
        "Note",
        input_field(note_inp, 372.0).into_any_element(),
    ));

    body = body.child(widgets::divider());
    body = body.child(section_heading("Translation"));
    let translated = d.translated;
    body = body.child(
        check_row("feat-translated", "Translate this feature", translated).on_click(cx.listener(
            |t, _, _, cx| {
                if let Some(Dialog::FeatureEditor(dd)) = &mut t.app.dialog {
                    dd.translated = !dd.translated;
                }
                cx.notify();
            },
        )),
    );
    if translated {
        let code_label = if d.genetic_code == 0 {
            format!("Document default ({})", code_by_id(default_code).name)
        } else {
            format!("{} — {}", d.genetic_code, code_by_id(d.genetic_code).name)
        };
        let code_btn = combo_button("feat-code", &code_label, 320.0).on_mouse_down(
            MouseButton::Left,
            cx.listener(|t, ev: &MouseDownEvent, _, cx| {
                t.dlg_combo = Some(DlgComboKind::FeatureCode);
                t.dlg_combo_anchor = ev.position;
                cx.notify();
            }),
        );
        body = body.child(field_row("Genetic code", code_btn.into_any_element()));
        let met = d.first_codon_met;
        body = body.child(
            check_row("feat-met", "Translate first codon as M", met).on_click(cx.listener(
                |t, _, _, cx| {
                    if let Some(Dialog::FeatureEditor(dd)) = &mut t.app.dialog {
                        dd.first_codon_met = !dd.first_codon_met;
                    }
                    cx.notify();
                },
            )),
        );
        let mut fr = div().flex().gap(px(6.0));
        for off in 0..3usize {
            fr = fr.child(
                seg(
                    SharedString::from(format!("feat-frame-{off}")),
                    &format!("+{off}"),
                    d.frame_offset == off,
                )
                .on_click(cx.listener(move |t, _, _, cx| {
                    if let Some(Dialog::FeatureEditor(dd)) = &mut t.app.dialog {
                        dd.frame_offset = off;
                    }
                    cx.notify();
                })),
            );
        }
        body = body.child(field_row("Frame offset", fr.into_any_element()));

        if let Ok(segments) = act::parse_segments(&d.segments_text, l, circular) {
            let mut f = Feature::new(0, d.name.clone(), d.kind.clone(), segments);
            f.direction = d.direction;
            f.translated = true;
            f.genetic_code = d.genetic_code;
            f.first_codon_met = d.first_codon_met;
            f.frame_offset = d.frame_offset;
            let code = code_by_id(default_code);
            let aa = f.translation(&app.app.doc.sequence, code);
            let stops = aa.iter().filter(|&&a| a == b'*').count();
            let mut info = div().flex().gap(px(12.0)).items_center().child(
                div()
                    .font_family(theme::MONO_FONT)
                    .text_size(px(theme::T_SM))
                    .text_color(theme::MUTED)
                    .child(format!("{} aa", aa.len())),
            );
            if stops > 0 && f.has_internal_stop(&app.app.doc.sequence, code) {
                info = info.child(
                    div()
                        .flex()
                        .items_center()
                        .gap(px(6.0))
                        .child(crate::icons::icon("alert", 13.0, theme::WARN))
                        .child(widgets::label(
                            &format!("{stops} stop codon(s) inside"),
                            theme::T_SM,
                            theme::WARN,
                        )),
                );
            } else if !aa.is_empty() {
                info = info.child(
                    div()
                        .flex()
                        .items_center()
                        .gap(px(6.0))
                        .child(crate::icons::icon("check", 13.0, theme::OK))
                        .child(widgets::label("no internal stops", theme::T_SM, theme::OK)),
                );
            }
            body = body.child(info);
            let text = String::from_utf8_lossy(&aa);
            let preview: String = text.chars().take(240).collect();
            body = body.child(
                div()
                    .mt(px(4.0))
                    .w_full()
                    .font_family(theme::MONO_FONT)
                    .text_size(px(11.5))
                    .text_color(theme::MUTED)
                    .child(preview),
            );
        }
    }
    if let Some(e) = &d.error {
        body = body.child(error_label(e));
    }

    let title = if d.id.is_some() {
        "Edit Feature"
    } else {
        "Add Feature"
    };
    let footer = footer_row(vec![
        widgets::button("feat-cancel", "Cancel", Btn::Ghost)
            .on_click(cx.listener(|t, _, _, cx| close_dialog(t, cx)))
            .into_any_element(),
        widgets::button("feat-save", "Save", Btn::Primary)
            .on_click(cx.listener(|t, _, _, cx| {
                let draft = match &t.app.dialog {
                    Some(Dialog::FeatureEditor(d)) => d.clone(),
                    _ => return,
                };
                match act::save_feature_draft(&mut t.app, &draft) {
                    Ok(()) => close_dialog(t, cx),
                    Err(e) => {
                        if let Some(Dialog::FeatureEditor(d)) = &mut t.app.dialog {
                            d.error = Some(e);
                        }
                        cx.notify();
                    }
                }
            }))
            .into_any_element(),
    ]);
    modal_shell(
        app,
        cx,
        title,
        600.0,
        max_h,
        true,
        body.into_any_element(),
        Some(footer.into_any_element()),
    )
}

// ---------------------------------------------------------------------------
// Primer editor
// ---------------------------------------------------------------------------

fn primer_editor(app: &mut HelixBench, cx: &mut Context<HelixBench>, max_h: Pixels) -> AnyElement {
    let mut d = match &app.app.dialog {
        Some(Dialog::PrimerEditor(d)) => d.clone(),
        _ => return div().into_any_element(),
    };
    let name_inp = app.input_entity(cx, "dlg:prim:name", &d.name, "Primer name", false);
    let seq_inp = app.input_entity(cx, "dlg:prim:seq", &d.sequence, "ATGAAACCC…  5′→3′", true);
    let desc_inp = app.input_entity(
        cx,
        "dlg:prim:desc",
        &d.description,
        "optional description",
        false,
    );
    d.name = name_inp.read(cx).text();
    d.sequence = seq_inp.read(cx).text();
    d.description = desc_inp.read(cx).text();
    app.app.dialog = Some(Dialog::PrimerEditor(d.clone()));

    let mut body = div().flex().flex_col().size_full();
    body = body.child(field_row(
        "Name",
        input_field(name_inp, 372.0).into_any_element(),
    ));
    body = body.child(field_row(
        "Sequence 5′→3′",
        input_field(seq_inp, 372.0).into_any_element(),
    ));
    body = body.child(field_row(
        "Description",
        input_field(desc_inp, 372.0).into_any_element(),
    ));
    let phospho = d.phosphorylated;
    let color_row = div()
        .flex()
        .items_center()
        .gap(px(12.0))
        .child(color_swatches(cx, d.color, ColorTarget::Primer))
        .child(
            check_row("prim-phospho", "5′ phosphorylated", phospho).on_click(cx.listener(
                |t, _, _, cx| {
                    if let Some(Dialog::PrimerEditor(dd)) = &mut t.app.dialog {
                        dd.phosphorylated = !dd.phosphorylated;
                    }
                    cx.notify();
                },
            )),
        );
    body = body.child(field_row("Color", color_row.into_any_element()));

    body = body.child(widgets::divider());
    body = body.child(section_heading("Properties & binding"));
    let cleaned = clean_sequence(&d.sequence);
    let params = HybridizationParams::default();
    if cleaned.is_empty() {
        body = body.child(widgets::label(
            "Enter a sequence to see Tm and binding sites.",
            theme::T_SM,
            theme::FAINT,
        ));
    } else {
        let seq_str = String::from_utf8_lossy(&cleaned).into_owned();
        let p = Primer::new(0, d.name.clone(), &seq_str);
        let tm = melting_temperature(&cleaned, &params);
        let mut props = div()
            .flex()
            .gap(px(14.0))
            .items_center()
            .child(
                div()
                    .font_family(theme::MONO_FONT)
                    .text_size(px(theme::T_SM))
                    .text_color(theme::TEXT)
                    .child(format!("{} nt", cleaned.len())),
            )
            .child(
                div()
                    .font_family(theme::MONO_FONT)
                    .text_size(px(theme::T_SM))
                    .text_color(theme::TEXT)
                    .child(format!("GC {:.1}%", p.gc_percent())),
            );
        match tm {
            Some(t) => {
                let col = if t < params.min_tm {
                    theme::WARN
                } else {
                    theme::OK
                };
                props = props.child(
                    div()
                        .font_family(theme::MONO_FONT)
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_size(px(theme::T_SM))
                        .text_color(col)
                        .child(format!("Tm {t:.1} °C")),
                );
            }
            None => {
                props = props.child(
                    div()
                        .font_family(theme::MONO_FONT)
                        .text_size(px(theme::T_SM))
                        .text_color(theme::FAINT)
                        .child("Tm —"),
                );
            }
        }
        props = props.child(widgets::label(
            &format!("({} mM Na⁺, {} nM)", params.salt_mm, params.primer_nm),
            theme::T_XS,
            theme::FAINT,
        ));
        body = body.child(props);

        let ss = domain::primer_struct::analyze(&cleaned);
        let mut ss_row = div().flex().items_center().gap(px(10.0)).mt(px(4.0));
        ss_row = ss_row.child(widgets::label(
            "Secondary structure:",
            theme::T_XS,
            theme::FAINT,
        ));
        match ss.hairpin {
            Some((stem, loop_len)) => {
                let col = if ss.hairpin_warn() {
                    theme::WARN
                } else {
                    theme::MUTED
                };
                ss_row = ss_row.child(
                    div()
                        .font_family(theme::MONO_FONT)
                        .text_size(px(theme::T_SM))
                        .text_color(col)
                        .child(format!("hairpin {stem} bp stem / {loop_len} nt loop")),
                );
            }
            None => {
                ss_row = ss_row.child(widgets::label("no hairpin", theme::T_SM, theme::MUTED));
            }
        }
        let dcol = if ss.dimer_warn() {
            theme::WARN
        } else {
            theme::MUTED
        };
        ss_row = ss_row.child(
            div()
                .font_family(theme::MONO_FONT)
                .text_size(px(theme::T_SM))
                .text_color(dcol)
                .child(format!(
                    "self-dimer {} bp{}",
                    ss.dimer,
                    if ss.dimer_3prime { " (3′)" } else { "" }
                )),
        );
        body = body.child(ss_row);

        let sites = binding_sites(&p, &app.app.doc.sequence, &params);
        if sites.is_empty() {
            body = body.child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(6.0))
                    .mt(px(4.0))
                    .child(crate::icons::icon("alert", 13.0, theme::WARN))
                    .child(widgets::label(
                        &format!(
                            "No binding sites (≥{} bp 3′ match, Tm ≥ {} °C)",
                            params.min_3prime_match, params.min_tm
                        ),
                        theme::T_SM,
                        theme::WARN,
                    )),
            );
        } else {
            body = body.child(
                widgets::label(
                    &format!("{} binding site(s):", sites.len()),
                    theme::T_SM,
                    theme::MUTED,
                )
                .mt(px(4.0)),
            );
            let mut list = div()
                .flex()
                .flex_col()
                .mt(px(2.0))
                .max_h(px(120.0))
                .overflow_hidden();
            for s in sites.iter().take(12) {
                let strand = if s.strand == Strand::Top {
                    "top"
                } else {
                    "bottom"
                };
                list = list.child(
                    div()
                        .font_family(theme::MONO_FONT)
                        .text_size(px(11.5))
                        .text_color(theme::TEXT)
                        .child(format!(
                            "{}..{} · {} strand · {} bp matched · Tm {:.1} °C",
                            s.start + 1,
                            s.start + s.len,
                            strand,
                            s.matched_3prime,
                            s.tm
                        )),
                );
            }
            body = body.child(list);
        }
    }
    if let Some(e) = &d.error {
        body = body.child(error_label(e));
    }

    let title = if d.id.is_some() {
        "Edit Primer"
    } else {
        "Add Primer"
    };
    let footer = footer_row(vec![
        widgets::button("prim-cancel", "Cancel", Btn::Ghost)
            .on_click(cx.listener(|t, _, _, cx| close_dialog(t, cx)))
            .into_any_element(),
        widgets::button("prim-save", "Save", Btn::Primary)
            .on_click(cx.listener(|t, _, _, cx| {
                let draft = match &t.app.dialog {
                    Some(Dialog::PrimerEditor(d)) => d.clone(),
                    _ => return,
                };
                match act::save_primer_draft(&mut t.app, &draft) {
                    Ok(()) => close_dialog(t, cx),
                    Err(e) => {
                        if let Some(Dialog::PrimerEditor(d)) = &mut t.app.dialog {
                            d.error = Some(e);
                        }
                        cx.notify();
                    }
                }
            }))
            .into_any_element(),
    ]);
    modal_shell(
        app,
        cx,
        title,
        600.0,
        max_h,
        true,
        body.into_any_element(),
        Some(footer.into_any_element()),
    )
}

// ---------------------------------------------------------------------------
// Genetic codes
// ---------------------------------------------------------------------------

fn genetic_codes(app: &mut HelixBench, cx: &mut Context<HelixBench>, max_h: Pixels) -> AnyElement {
    let selected = match &app.app.dialog {
        Some(Dialog::GeneticCodes { selected }) => *selected,
        _ => 1,
    };
    let default_code = app.app.doc.default_genetic_code;
    let code = code_by_id(selected);

    let mut body = div().flex().flex_col().size_full();
    let combo = combo_button("gc-table", &format!("{} — {}", code.id, code.name), 360.0)
        .on_mouse_down(
            MouseButton::Left,
            cx.listener(|t, ev: &MouseDownEvent, _, cx| {
                t.dlg_combo = Some(DlgComboKind::CodesTable);
                t.dlg_combo_anchor = ev.position;
                cx.notify();
            }),
        );
    body = body.child(field_row("Genetic code", combo.into_any_element()));

    let bases = *b"TCAG";
    let mono = theme::MONO_FONT;
    // header row
    let mut header = div()
        .flex()
        .gap(px(6.0))
        .mb(px(4.0))
        .child(div().w(px(20.0)));
    for b2 in bases {
        header = header.child(
            div()
                .w(px(118.0))
                .font_family(mono)
                .font_weight(FontWeight::SEMIBOLD)
                .text_size(px(theme::T_SM))
                .text_color(theme::ACCENT)
                .child((b2 as char).to_string()),
        );
    }
    body = body.child(header);
    for (i, &b1) in bases.iter().enumerate() {
        let mut row = div().flex().gap(px(6.0)).mb(px(4.0)).child(
            div()
                .w(px(20.0))
                .font_family(mono)
                .font_weight(FontWeight::SEMIBOLD)
                .text_size(px(theme::T_SM))
                .text_color(theme::ACCENT)
                .child((b1 as char).to_string()),
        );
        for (j, &b2) in bases.iter().enumerate() {
            let mut cell = div().w(px(118.0)).flex().flex_col();
            let mut has_stop = false;
            for (k, &b3) in bases.iter().enumerate() {
                let idx = i * 16 + j * 4 + k;
                let aa = code.table.as_bytes()[idx];
                let codon = format!("{}{}{}", b1 as char, b2 as char, b3 as char);
                let is_start = code.starts.contains(&codon.as_str());
                let marker = if aa == b'*' {
                    "■"
                } else if is_start {
                    "►"
                } else {
                    " "
                };
                if aa == b'*' {
                    has_stop = true;
                }
                cell = cell.child(
                    div()
                        .font_family(mono)
                        .text_size(px(10.5))
                        .text_color(if aa == b'*' {
                            theme::MUTED
                        } else {
                            theme::TEXT
                        })
                        .child(format!("{codon} {} {marker}", aa as char)),
                );
            }
            let _ = has_stop;
            row = row.child(cell);
        }
        body = body.child(row);
    }
    body =
        body.child(widgets::label("► start codon   ■ stop", theme::T_XS, theme::FAINT).mt(px(4.0)));

    let is_default = default_code == selected;
    let mut footer_buttons = vec![widgets::button("gc-close", "Close", Btn::Ghost)
        .on_click(cx.listener(|t, _, _, cx| close_dialog(t, cx)))
        .into_any_element()];
    let mut use_btn = widgets::button("gc-use", "Use as document default", Btn::Primary);
    if !is_default {
        use_btn = use_btn.on_click(cx.listener(move |t, _, _, cx| {
            let name = code_by_id(selected).name;
            t.app
                .mutate(&format!("Set genetic code to {name}"), Vec::new(), |doc| {
                    doc.default_genetic_code = selected;
                });
            t.app.toast(format!("Default genetic code: {name}"));
            cx.notify();
        }));
    } else {
        use_btn = use_btn.opacity(0.4);
    }
    footer_buttons.push(use_btn.into_any_element());
    let footer = footer_row(footer_buttons);
    modal_shell(
        app,
        cx,
        "Genetic Codes",
        640.0,
        max_h,
        true,
        body.into_any_element(),
        Some(footer.into_any_element()),
    )
}

// ---------------------------------------------------------------------------
// Letter codes
// ---------------------------------------------------------------------------

fn letter_codes(app: &mut HelixBench, cx: &mut Context<HelixBench>, max_h: Pixels) -> AnyElement {
    let dna = [
        ("A", "Adenine"),
        ("C", "Cytosine"),
        ("G", "Guanine"),
        ("T", "Thymine"),
        ("R", "A or G (purine)"),
        ("Y", "C or T (pyrimidine)"),
        ("S", "G or C (strong)"),
        ("W", "A or T (weak)"),
        ("K", "G or T (keto)"),
        ("M", "A or C (amino)"),
        ("B", "C, G or T (not A)"),
        ("D", "A, G or T (not C)"),
        ("H", "A, C or T (not G)"),
        ("V", "A, C or G (not T)"),
        ("N", "Any base"),
    ];
    let prot = [
        ("B", "D or N (Asx)"),
        ("Z", "E or Q (Glx)"),
        ("X", "Any amino acid"),
        ("*", "Stop codon"),
    ];
    let mut body = div().flex().flex_col().size_full();
    body = body.child(section_heading("DNA letter codes"));
    let mut grid = div().flex().flex_col();
    for (code, meaning) in dna {
        grid = grid.child(letter_row(code, meaning));
    }
    body = body.child(grid);
    body = body.child(section_heading("Protein letter codes").mt(px(8.0)));
    let mut grid2 = div().flex().flex_col();
    for (code, meaning) in prot {
        grid2 = grid2.child(letter_row(code, meaning));
    }
    body = body.child(grid2);
    body = body.child(
        widgets::label(
            "The 20 standard amino-acid letters (A, C, D, E, F, G, H, I, K, L, M, N, P, Q, R, S, T, V, W, Y) match themselves.",
            theme::T_XS,
            theme::MUTED,
        ).mt(px(8.0)),
    );
    let footer = footer_row(vec![widgets::button("lc-close", "Close", Btn::Ghost)
        .on_click(cx.listener(|t, _, _, cx| close_dialog(t, cx)))
        .into_any_element()]);
    modal_shell(
        app,
        cx,
        "Letter Codes",
        480.0,
        max_h,
        true,
        body.into_any_element(),
        Some(footer.into_any_element()),
    )
}

fn letter_row(code: &str, meaning: &str) -> Div {
    div()
        .flex()
        .items_center()
        .gap(px(14.0))
        .h(px(20.0))
        .child(
            div()
                .w(px(20.0))
                .font_family(theme::MONO_FONT)
                .font_weight(FontWeight::SEMIBOLD)
                .text_size(px(theme::T_SM))
                .text_color(theme::ACCENT)
                .child(code.to_owned()),
        )
        .child(
            div()
                .text_size(px(theme::T_SM))
                .text_color(theme::TEXT)
                .child(meaning.to_owned()),
        )
}

// ---------------------------------------------------------------------------
// Search tips
// ---------------------------------------------------------------------------

fn search_tips(app: &mut HelixBench, cx: &mut Context<HelixBench>, max_h: Pixels) -> AnyElement {
    let mut body = div().flex().flex_col().size_full().overflow_hidden();
    body = body.child(section_heading("DNA searches"));
    for (q, desc) in [
        (
            "GAATTC",
            "Find an exact sequence on both strands (and across the origin of circular DNA).",
        ),
        (
            "GGTCTRCTTC",
            "R allows A or G at that position — finds GGTCTACTTC and GGTCTGCTTC.",
        ),
        (
            "GGTCT(AG)CTTC",
            "Parentheses allow an explicit set of bases at one position.",
        ),
        ("AAA!R", "! excludes bases: AAA not followed by A or G."),
        ("AAA!(AG)", "Exclude an explicit set with !(…)."),
        (
            "\"R\"",
            "Quoted letters match only bases literally represented as R.",
        ),
    ] {
        body = body.child(tip(q, desc));
    }
    body = body.child(section_heading("Protein searches").mt(px(6.0)));
    for (q, desc) in [
        ("MKWG", "Searches all six translation frames of visible sequence."),
        ("MXWG", "X allows any amino acid at that position."),
        ("!P", "Exclude an amino acid; !(ST) excludes a set."),
        ("\"X\"", "Quoted X matches only residues literally represented as X. B matches D/N, Z matches E/Q."),
    ] {
        body = body.child(tip(q, desc));
    }
    body = body.child(section_heading("Enzyme / feature / primer searches").mt(px(6.0)));
    body = body.child(widgets::label(
        "Queries are not case sensitive. Use * as a wildcard: \"*promoter\" or \"M13*\".",
        theme::T_SM,
        theme::MUTED,
    ));
    let footer = footer_row(vec![widgets::button("st-close", "Close", Btn::Ghost)
        .on_click(cx.listener(|t, _, _, cx| close_dialog(t, cx)))
        .into_any_element()]);
    modal_shell(
        app,
        cx,
        "Search Tips",
        560.0,
        max_h,
        true,
        body.into_any_element(),
        Some(footer.into_any_element()),
    )
}

fn tip(q: &str, desc: &str) -> Div {
    div()
        .flex()
        .flex_col()
        .mb(px(6.0))
        .child(
            div()
                .font_family(theme::MONO_FONT)
                .font_weight(FontWeight::SEMIBOLD)
                .text_size(px(theme::T_SM))
                .text_color(theme::ACCENT)
                .child(q.to_owned()),
        )
        .child(
            div()
                .text_size(px(theme::T_SM))
                .text_color(theme::MUTED)
                .child(desc.to_owned()),
        )
}

// ---------------------------------------------------------------------------
// About
// ---------------------------------------------------------------------------

fn about(app: &mut HelixBench, cx: &mut Context<HelixBench>, max_h: Pixels) -> AnyElement {
    let mut body = div().flex().flex_col().size_full();
    let brand = div()
        .flex()
        .items_center()
        .gap(px(12.0))
        .mb(px(10.0))
        .child(
            div()
                .w(px(34.0))
                .h(px(34.0))
                .rounded(px(8.0))
                .bg(theme::ACCENT)
                .flex()
                .items_center()
                .justify_center()
                .child(div().w(px(14.0)).h(px(14.0)).rounded(px(3.0)).bg(theme::BG)),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .child(
                    div()
                        .text_size(px(theme::T_XL))
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(theme::ACCENT)
                        .child("HelixBench"),
                )
                .child(widgets::label(
                    "Local-first DNA workbench · version 0.1.0",
                    theme::T_SM,
                    theme::MUTED,
                )),
        );
    body = body.child(brand);
    body = body.child(
        widgets::label(
            "A native Windows desktop app for plasmid and sequence analysis: restriction mapping, feature annotation, primer design, six-frame translation and history-tracked editing. All data stays on this machine.",
            theme::T_BASE,
            theme::TEXT,
        ),
    );
    body = body.child(section_heading("Keyboard shortcuts").mt(px(10.0)));
    let mut keys = div().flex().flex_col();
    for (k, v) in [
        ("Ctrl+O / Ctrl+S", "Open / Save"),
        ("Ctrl+Z / Ctrl+Y", "Undo / Redo"),
        ("Ctrl+F", "Find"),
        (
            "Ctrl+A / Ctrl+C / Ctrl+V / Ctrl+X",
            "Select all / Copy / Paste / Cut",
        ),
        ("Delete", "Delete selection"),
        ("Ctrl+1…6", "Switch views"),
        ("Esc", "Close dialog / find bar"),
    ] {
        keys = keys.child(
            div()
                .flex()
                .items_center()
                .gap(px(16.0))
                .h(px(20.0))
                .child(
                    div()
                        .w(px(220.0))
                        .font_family(theme::MONO_FONT)
                        .text_size(px(11.5))
                        .text_color(theme::ACCENT)
                        .child(k),
                )
                .child(div().text_size(px(11.5)).text_color(theme::MUTED).child(v)),
        );
    }
    body = body.child(keys);
    body = body.child(
        widgets::label(
            "Built with Rust + GPUI · feature set derived from the SnapGene User Guide.",
            theme::T_XS,
            theme::FAINT,
        )
        .mt(px(10.0)),
    );
    let footer = footer_row(vec![widgets::button("ab-close", "Close", Btn::Ghost)
        .on_click(cx.listener(|t, _, _, cx| close_dialog(t, cx)))
        .into_any_element()]);
    modal_shell(
        app,
        cx,
        "About HelixBench",
        500.0,
        max_h,
        true,
        body.into_any_element(),
        Some(footer.into_any_element()),
    )
}

// ---------------------------------------------------------------------------
// Options
// ---------------------------------------------------------------------------

fn options(app: &mut HelixBench, cx: &mut Context<HelixBench>, max_h: Pixels) -> AnyElement {
    let p = app.app.prefs.clone();
    let colors_mode = app.app.colors_mode;
    let orf_min = p.orf_min_aa;
    let default_code = app.app.doc.default_genetic_code;
    let topology = app.app.doc.sequence.topology;

    let mut body = div().flex().flex_col().size_full();
    body = body.child(section_heading("Show in map & sequence"));
    let toggles = [
        ("show_features", "Features", p.show_features),
        ("show_enzymes", "Enzymes", p.show_enzymes),
        ("show_primers", "Primers", p.show_primers),
        ("show_orfs", "ORFs", p.show_orfs),
        ("show_translations", "Translations", p.show_translations),
        ("show_complement", "Complement strand", p.show_complement),
        ("show_colors", "Base colors", p.show_colors),
        ("show_zoom_controls", "Zoom controls", p.show_zoom_controls),
    ];
    let mut grid = div().flex().flex_col();
    for chunk in toggles.chunks(2) {
        let mut row = div().flex().gap(px(20.0));
        for (key, label, on) in chunk {
            let key = *key;
            row = row.child(div().w(px(210.0)).child(
                check_row(&format!("opt-{key}"), label, *on).on_click(cx.listener(
                    move |t, _, _, cx| {
                        t.toggle_pref(key);
                        cx.notify();
                    },
                )),
            ));
        }
        grid = grid.child(row);
    }
    body = body.child(grid);

    let mut mode_row = div().flex().gap(px(6.0));
    for (mode, label) in [
        (ColorsMode::None, "No colors"),
        (ColorsMode::History, "History"),
        (ColorsMode::Custom, "Custom"),
    ] {
        mode_row = mode_row.child(
            seg(
                SharedString::from(format!("opt-mode-{label}")),
                label,
                colors_mode == mode,
            )
            .on_click(cx.listener(move |t, _, _, cx| {
                t.app.colors_mode = mode;
                cx.notify();
            })),
        );
    }
    body = body.child(field_row("Show colors", mode_row.into_any_element()).mt(px(6.0)));

    body = body.child(widgets::divider());
    body = body.child(section_heading("ORFs & translation"));
    let stepper = div()
        .flex()
        .items_center()
        .gap(px(8.0))
        .child(
            widgets::button("orf-minus", "−", Btn::Outline).on_click(cx.listener(|t, _, _, cx| {
                t.app.prefs.orf_min_aa = t.app.prefs.orf_min_aa.saturating_sub(10).max(10);
                t.app.analysis_dirty = true;
                cx.notify();
            })),
        )
        .child(
            div()
                .w(px(72.0))
                .font_family(theme::MONO_FONT)
                .text_size(px(theme::T_SM))
                .text_color(theme::TEXT)
                .child(format!("{orf_min} aa")),
        )
        .child(
            widgets::button("orf-plus", "+", Btn::Outline).on_click(cx.listener(|t, _, _, cx| {
                t.app.prefs.orf_min_aa = (t.app.prefs.orf_min_aa + 10).min(300);
                t.app.analysis_dirty = true;
                cx.notify();
            })),
        );
    body = body.child(field_row("Min ORF length", stepper.into_any_element()));

    let code = code_by_id(default_code);
    let code_btn = combo_button("opt-code", &format!("{} — {}", code.id, code.name), 320.0)
        .on_mouse_down(
            MouseButton::Left,
            cx.listener(|t, ev: &MouseDownEvent, _, cx| {
                t.dlg_combo = Some(DlgComboKind::OptionsCode);
                t.dlg_combo_anchor = ev.position;
                cx.notify();
            }),
        );
    body = body.child(field_row("Genetic code", code_btn.into_any_element()));

    body = body.child(widgets::divider());
    body = body.child(section_heading("Topology"));
    let mut topo_row = div().flex().items_center().gap(px(6.0));
    for (t, label) in [
        (Topology::Linear, "Linear"),
        (Topology::Circular, "Circular"),
    ] {
        topo_row = topo_row.child(
            seg(
                SharedString::from(format!("opt-topo-{label}")),
                label,
                topology == t,
            )
            .on_click(cx.listener(move |tst, _, _, cx| {
                act::set_topology(&mut tst.app, t);
                cx.notify();
            })),
        );
    }
    if topology == Topology::Circular {
        topo_row = topo_row.child(
            widgets::button("opt-rotate", "Rotate origin…", Btn::Subtle)
                .ml(px(8.0))
                .on_click(cx.listener(|t, _, _, cx| {
                    t.app.dialog = Some(Dialog::Rotate {
                        position: "1".into(),
                        error: None,
                    });
                    cx.notify();
                })),
        );
    }
    body = body.child(topo_row);

    let footer = footer_row(vec![widgets::button("opt-done", "Done", Btn::Primary)
        .on_click(cx.listener(|t, _, _, cx| close_dialog(t, cx)))
        .into_any_element()]);
    modal_shell(
        app,
        cx,
        "Map & Sequence Options",
        520.0,
        max_h,
        true,
        body.into_any_element(),
        Some(footer.into_any_element()),
    )
}

// ---------------------------------------------------------------------------
// Rotate
// ---------------------------------------------------------------------------

fn rotate(app: &mut HelixBench, cx: &mut Context<HelixBench>, max_h: Pixels) -> AnyElement {
    let l = app.app.doc.sequence.len();
    let (seed, error) = match &app.app.dialog {
        Some(Dialog::Rotate { position, error }) => (position.clone(), error.clone()),
        _ => ("1".into(), None),
    };
    let inp = app.input_entity(cx, "dlg:rotate:pos", &seed, "1", true);
    let position = inp.read(cx).text();
    if let Some(Dialog::Rotate { position: p, .. }) = &mut app.app.dialog {
        *p = position.clone();
    }

    let mut body = div().flex().flex_col().size_full();
    body = body.child(
        widgets::label(
            &format!("Move the origin (position 1) of this {l} bp circular sequence. All annotations rotate with it."),
            theme::T_BASE,
            theme::MUTED,
        ),
    );
    let row = div()
        .flex()
        .items_center()
        .gap(px(10.0))
        .mt(px(10.0))
        .child(field_label("New position 1"))
        .child(input_field(inp, 100.0))
        .child(widgets::label(
            &format!("(1..{l})"),
            theme::T_SM,
            theme::FAINT,
        ));
    body = body.child(row);
    if let Some(e) = error {
        body = body.child(error_label(&e));
    }

    let footer = footer_row(vec![
        widgets::button("rot-cancel", "Cancel", Btn::Ghost)
            .on_click(cx.listener(|t, _, _, cx| close_dialog(t, cx)))
            .into_any_element(),
        widgets::button("rot-go", "Rotate", Btn::Primary)
            .on_click(cx.listener(move |t, _, _, cx| {
                let pos = match &t.app.dialog {
                    Some(Dialog::Rotate { position, .. }) => position.clone(),
                    _ => return,
                };
                match pos.trim().parse::<usize>() {
                    Ok(p) if p >= 1 && p <= l => {
                        act::rotate_to(&mut t.app, p - 1);
                        close_dialog(t, cx);
                    }
                    _ => {
                        if let Some(Dialog::Rotate { error, .. }) = &mut t.app.dialog {
                            *error = Some(format!("Enter a position between 1 and {l}"));
                        }
                        cx.notify();
                    }
                }
            }))
            .into_any_element(),
    ]);
    modal_shell(
        app,
        cx,
        "Rotate Circular Sequence",
        440.0,
        max_h,
        true,
        body.into_any_element(),
        Some(footer.into_any_element()),
    )
}

// ---------------------------------------------------------------------------
// Unsaved changes
// ---------------------------------------------------------------------------

fn unsaved(app: &mut HelixBench, cx: &mut Context<HelixBench>, max_h: Pixels) -> AnyElement {
    let (name, action) = match &app.app.dialog {
        Some(Dialog::Unsaved { action }) => (app.app.doc.name.clone(), action.clone()),
        _ => (String::new(), PendingAction::CloseApp),
    };
    let mut body = div().flex().flex_col().size_full();
    body = body.child(
        div()
            .text_size(px(theme::T_MD))
            .text_color(theme::TEXT)
            .child(format!("\"{name}\" has unsaved changes.")),
    );
    body = body
        .child(widgets::label("Save before continuing?", theme::T_BASE, theme::MUTED).mt(px(4.0)));

    let act_discard = action.clone();
    let act_save = action;
    let footer = footer_row(vec![
        widgets::button("us-cancel", "Cancel", Btn::Ghost)
            .on_click(cx.listener(|t, _, _, cx| close_dialog(t, cx)))
            .into_any_element(),
        widgets::button("us-discard", "Discard", Btn::Danger)
            .on_click(cx.listener(move |t, _, _, cx| {
                t.app.dialog = None;
                act::perform(&mut t.app, act_discard.clone());
                cx.notify();
            }))
            .into_any_element(),
        widgets::button("us-save", "Save", Btn::Primary)
            .on_click(cx.listener(move |t, _, _, cx| {
                if act::save(&mut t.app) {
                    t.app.dialog = None;
                    act::perform(&mut t.app, act_save.clone());
                }
                cx.notify();
            }))
            .into_any_element(),
    ]);
    modal_shell(
        app,
        cx,
        "Unsaved Changes",
        440.0,
        max_h,
        false,
        body.into_any_element(),
        Some(footer.into_any_element()),
    )
}

// ---------------------------------------------------------------------------
// Detect common features
// ---------------------------------------------------------------------------

struct DetectRow {
    name: String,
    range: String,
    strand: String,
    dup: bool,
    checked: bool,
}

fn detect_common(app: &mut HelixBench, cx: &mut Context<HelixBench>, max_h: Pixels) -> AnyElement {
    let (detected, checked) = match &app.app.dialog {
        Some(Dialog::DetectCommon { detected, checked }) => (detected.clone(), checked.clone()),
        _ => return div().into_any_element(),
    };
    let entity: Entity<HelixBench> = cx.entity();
    let n_new = detected
        .iter()
        .zip(checked.iter())
        .filter(|(d, &c)| c && !domain::common_features::is_duplicate(&app.app.doc.features, d))
        .count();

    let rows: Vec<DetectRow> = detected
        .iter()
        .enumerate()
        .map(|(i, d)| {
            let t = &domain::common_features::COMMON_FEATURES[d.template];
            let dup = domain::common_features::is_duplicate(&app.app.doc.features, d);
            DetectRow {
                name: t.name.to_string(),
                range: format!("{}..{}", d.range.start + 1, d.range.start + d.range.len),
                strand: if dup {
                    "already present".into()
                } else if d.strand == Strand::Top {
                    "top".into()
                } else {
                    "bottom".into()
                },
                dup,
                checked: checked[i],
            }
        })
        .collect();

    let mut body = div().flex().flex_col();
    body = body.child(widgets::label(
        &format!("{} known element(s) found · {} new", detected.len(), n_new),
        theme::T_BASE,
        theme::MUTED,
    ));
    let tools = div()
        .flex()
        .gap(px(8.0))
        .mt(px(6.0))
        .mb(px(8.0))
        .child(
            widgets::button("dc-all", "Select all new", Btn::Subtle).on_click(cx.listener(
                |t, _, _, cx| {
                    if let Some(Dialog::DetectCommon { detected, checked }) = &mut t.app.dialog {
                        for (d, c) in detected.iter().zip(checked.iter_mut()) {
                            *c = !domain::common_features::is_duplicate(&t.app.doc.features, d);
                        }
                    }
                    cx.notify();
                },
            )),
        )
        .child(
            widgets::button("dc-none", "Select none", Btn::Subtle).on_click(cx.listener(
                |t, _, _, cx| {
                    if let Some(Dialog::DetectCommon { checked, .. }) = &mut t.app.dialog {
                        for c in checked.iter_mut() {
                            *c = false;
                        }
                    }
                    cx.notify();
                },
            )),
        );
    body = body.child(tools);

    let n = rows.len();
    let scroll = app.detect_scroll.clone();
    let list = uniform_list("detect-list", n, move |range, window, _cx| {
        range
            .map(|idx| {
                let r = &rows[idx];
                let e_tog = entity.clone();
                let name_col = if r.dup { theme::FAINT } else { theme::TEXT };
                let strand_col = if r.dup { theme::FAINT } else { theme::ACCENT };
                let name = crate::views::ellipsize(window, &r.name, 248.0, theme::T_BASE, false);
                div()
                    .id(("detect-row", idx))
                    .h(px(28.0))
                    .w_full()
                    .flex()
                    .items_center()
                    .bg(crate::views::row_bg(false, idx))
                    .border_b_1()
                    .border_color(theme::BORDER.opacity(0.45))
                    .child(
                        div()
                            .w(px(34.0))
                            .h_full()
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(
                                check_box(SharedString::from(format!("dc-{idx}")), r.checked)
                                    .on_click(move |_, _, cx| {
                                        cx.stop_propagation();
                                        e_tog.update(cx, |t, cx| {
                                            if let Some(Dialog::DetectCommon {
                                                detected,
                                                checked,
                                            }) = &mut t.app.dialog
                                            {
                                                if idx < checked.len()
                                                    && !domain::common_features::is_duplicate(
                                                        &t.app.doc.features,
                                                        &detected[idx],
                                                    )
                                                {
                                                    checked[idx] = !checked[idx];
                                                }
                                            }
                                            cx.notify();
                                        });
                                    }),
                            ),
                    )
                    .child(
                        div()
                            .flex_1()
                            .min_w(px(0.0))
                            .px(px(8.0))
                            .overflow_hidden()
                            .text_size(px(theme::T_BASE))
                            .text_color(name_col)
                            .child(name),
                    )
                    .child(
                        div()
                            .w(px(120.0))
                            .px(px(8.0))
                            .font_family(theme::MONO_FONT)
                            .text_size(px(11.5))
                            .text_color(theme::MUTED)
                            .child(r.range.clone()),
                    )
                    .child(
                        div()
                            .w(px(120.0))
                            .px(px(8.0))
                            .text_size(px(theme::T_XS))
                            .text_color(strand_col)
                            .child(r.strand.clone()),
                    )
                    .into_any_element()
            })
            .collect()
    })
    .track_scroll(scroll)
    .with_decoration(ScrollbarDecoration)
    .h(px(280.0));
    body = body.child(list);

    let det_clone = detected.clone();
    let chk_clone = checked.clone();
    let mut add_btn = widgets::button("dc-add", &format!("Add {n_new} Features"), Btn::Primary);
    if n_new > 0 {
        add_btn = add_btn.on_click(cx.listener(move |t, _, _, cx| {
            act::apply_detected(&mut t.app, &det_clone, &chk_clone);
            close_dialog(t, cx);
        }));
    } else {
        add_btn = add_btn.opacity(0.4);
    }
    let footer = footer_row(vec![
        widgets::button("dc-cancel", "Cancel", Btn::Ghost)
            .on_click(cx.listener(|t, _, _, cx| close_dialog(t, cx)))
            .into_any_element(),
        add_btn.into_any_element(),
    ]);
    modal_shell(
        app,
        cx,
        "Detect Common Features",
        580.0,
        max_h,
        true,
        body.into_any_element(),
        Some(footer.into_any_element()),
    )
}

// ---------------------------------------------------------------------------
// Color selection
// ---------------------------------------------------------------------------

fn color_selection(
    app: &mut HelixBench,
    cx: &mut Context<HelixBench>,
    max_h: Pixels,
) -> AnyElement {
    let d = match &app.app.dialog {
        Some(Dialog::ColorSelection(d)) => *d,
        _ => return div().into_any_element(),
    };
    let end = d.range.start + d.range.len;
    let mut body = div().flex().flex_col().size_full();
    body = body.child(widgets::label(
        &format!(
            "Color bases {}..{} ({} bp).",
            d.range.start + 1,
            end,
            d.range.len
        ),
        theme::T_BASE,
        theme::MUTED,
    ));
    body = body.child(
        field_row(
            "Color",
            color_swatches(cx, d.color, ColorTarget::ColorSel).into_any_element(),
        )
        .mt(px(10.0)),
    );
    let mut strand_row = div().flex().gap(px(6.0));
    for (mask, label) in [
        (StrandMask(StrandMask::TOP), "Top"),
        (StrandMask(StrandMask::BOTTOM), "Bottom"),
        (StrandMask(StrandMask::BOTH), "Both"),
    ] {
        strand_row = strand_row.child(
            seg(
                SharedString::from(format!("cs-{label}")),
                label,
                d.strands == mask,
            )
            .on_click(cx.listener(move |t, _, _, cx| {
                if let Some(Dialog::ColorSelection(dd)) = &mut t.app.dialog {
                    dd.strands = mask;
                }
                cx.notify();
            })),
        );
    }
    body = body.child(field_row("Strands", strand_row.into_any_element()));
    body = body.child(widgets::label(
        "Shown in the Custom DNA colors mode.",
        theme::T_XS,
        theme::FAINT,
    ));

    let footer = footer_row(vec![
        widgets::button("cs-cancel", "Cancel", Btn::Ghost)
            .on_click(cx.listener(|t, _, _, cx| close_dialog(t, cx)))
            .into_any_element(),
        widgets::button("cs-apply", "Apply", Btn::Primary)
            .on_click(cx.listener(|t, _, _, cx| {
                let draft = match &t.app.dialog {
                    Some(Dialog::ColorSelection(d)) => *d,
                    _ => return,
                };
                act::apply_color_selection(&mut t.app, draft);
                close_dialog(t, cx);
            }))
            .into_any_element(),
    ]);
    modal_shell(
        app,
        cx,
        "Color Selection",
        440.0,
        max_h,
        true,
        body.into_any_element(),
        Some(footer.into_any_element()),
    )
}

// ---------------------------------------------------------------------------
// Dropdown popups
// ---------------------------------------------------------------------------

fn combo_row(id: impl Into<gpui::ElementId>, label: &str, checked: bool) -> Stateful<Div> {
    div()
        .id(id)
        .flex()
        .items_center()
        .gap(px(8.0))
        .w_full()
        .h(px(24.0))
        .px(px(10.0))
        .rounded(px(4.0))
        .text_size(px(theme::T_SM))
        .cursor_pointer()
        .text_color(theme::TEXT)
        .hover(|s| s.bg(theme::SURFACE_2))
        .child(
            div()
                .w(px(12.0))
                .text_color(theme::ACCENT)
                .child(if checked { "✓" } else { "" }.to_owned()),
        )
        .child(div().truncate().child(label.to_owned()))
}

fn render_dlg_combo(app: &mut HelixBench, cx: &mut Context<HelixBench>) -> Option<AnyElement> {
    let kind = app.dlg_combo?;
    let mut rows: Vec<AnyElement> = Vec::new();
    match kind {
        DlgComboKind::FeatureKind => {
            let current = match &app.app.dialog {
                Some(Dialog::FeatureEditor(d)) => d.kind.clone(),
                _ => String::new(),
            };
            for k in FEATURE_KINDS {
                let active = *k == current;
                rows.push(
                    combo_row(SharedString::from(format!("dk-{k}")), k, active)
                        .on_click(cx.listener(move |t, _, _, cx| {
                            if let Some(Dialog::FeatureEditor(d)) = &mut t.app.dialog {
                                d.kind = k.to_string();
                                d.color = kind_color(k);
                                d.translated = *k == "CDS";
                            }
                            t.dlg_combo = None;
                            cx.notify();
                        }))
                        .into_any_element(),
                );
            }
        }
        DlgComboKind::FeatureCode => {
            let (cur, default_code) = match &app.app.dialog {
                Some(Dialog::FeatureEditor(d)) => {
                    (d.genetic_code, app.app.doc.default_genetic_code)
                }
                _ => (0, 1),
            };
            let def_label = format!("Document default ({})", code_by_id(default_code).name);
            rows.push(
                combo_row("dc-0", &def_label, cur == 0)
                    .on_click(cx.listener(|t, _, _, cx| {
                        if let Some(Dialog::FeatureEditor(d)) = &mut t.app.dialog {
                            d.genetic_code = 0;
                        }
                        t.dlg_combo = None;
                        cx.notify();
                    }))
                    .into_any_element(),
            );
            for c in CODES {
                let id = c.id;
                rows.push(
                    combo_row(
                        SharedString::from(format!("dc-{id}")),
                        &format!("{} — {}", c.id, c.name),
                        cur == id,
                    )
                    .on_click(cx.listener(move |t, _, _, cx| {
                        if let Some(Dialog::FeatureEditor(d)) = &mut t.app.dialog {
                            d.genetic_code = id;
                        }
                        t.dlg_combo = None;
                        cx.notify();
                    }))
                    .into_any_element(),
                );
            }
        }
        DlgComboKind::OptionsCode => {
            let cur = app.app.doc.default_genetic_code;
            for c in CODES {
                let id = c.id;
                rows.push(
                    combo_row(
                        SharedString::from(format!("oc-{id}")),
                        &format!("{} — {}", c.id, c.name),
                        cur == id,
                    )
                    .on_click(cx.listener(move |t, _, _, cx| {
                        let name = code_by_id(id).name;
                        t.app
                            .mutate(&format!("Set genetic code to {name}"), Vec::new(), |doc| {
                                doc.default_genetic_code = id;
                            });
                        t.dlg_combo = None;
                        cx.notify();
                    }))
                    .into_any_element(),
                );
            }
        }
        DlgComboKind::CodesTable => {
            let cur = match &app.app.dialog {
                Some(Dialog::GeneticCodes { selected }) => *selected,
                _ => 1,
            };
            for c in CODES {
                let id = c.id;
                rows.push(
                    combo_row(
                        SharedString::from(format!("ct-{id}")),
                        &format!("{} — {}", c.id, c.name),
                        cur == id,
                    )
                    .on_click(cx.listener(move |t, _, _, cx| {
                        if let Some(Dialog::GeneticCodes { selected }) = &mut t.app.dialog {
                            *selected = id;
                        }
                        t.dlg_combo = None;
                        cx.notify();
                    }))
                    .into_any_element(),
                );
            }
        }
    }

    let mut card = div()
        .id("dlg-combo-card")
        .flex()
        .flex_col()
        .min_w(px(240.0))
        .max_w(px(400.0))
        .max_h(px(600.0))
        .py(px(4.0))
        .bg(theme::OVERLAY)
        .border_1()
        .border_color(theme::BORDER_STRONG)
        .rounded(px(8.0))
        .shadow(theme::shadow_dialog())
        .overflow_hidden()
        .on_mouse_down(
            MouseButton::Left,
            cx.listener(|_, _, _, cx| cx.stop_propagation()),
        )
        .on_mouse_down_out(cx.listener(|t, _, _, cx| {
            t.dlg_combo = None;
            cx.notify();
        }));
    for row in rows {
        card = card.child(row);
    }
    Some(
        deferred(
            anchored()
                .anchor(Corner::TopLeft)
                .position(app.dlg_combo_anchor)
                .offset(point(px(-4.0), px(6.0)))
                .snap_to_window()
                .child(card),
        )
        .with_priority(55)
        .into_any_element(),
    )
}

// ---------------------------------------------------------------------------
// In-silico PCR
// ---------------------------------------------------------------------------

fn pcr(app: &mut HelixBench, cx: &mut Context<HelixBench>, max_h: Pixels) -> AnyElement {
    let (fseed, rseed, copy, error) = match &app.app.dialog {
        Some(Dialog::Pcr(d)) => (
            d.fwd.clone(),
            d.rev.clone(),
            d.copy_features,
            d.error.clone(),
        ),
        _ => (String::new(), String::new(), true, None),
    };
    let fi = app.input_entity(
        cx,
        "dlg:pcr:fwd",
        &fseed,
        "Forward primer: name or 5'-3'",
        true,
    );
    let ri = app.input_entity(
        cx,
        "dlg:pcr:rev",
        &rseed,
        "Reverse primer: name or 5'-3'",
        true,
    );
    let f = fi.read(cx).text();
    let r = ri.read(cx).text();
    if let Some(Dialog::Pcr(d)) = &mut app.app.dialog {
        d.fwd = f.clone();
        d.rev = r.clone();
    }
    let draft = match &app.app.dialog {
        Some(Dialog::Pcr(d)) => d.clone(),
        _ => PcrDraft::default(),
    };
    let preview = act::pcr_preview(&app.app, &draft);

    let mut body = div().flex().flex_col().size_full().gap(px(10.0));
    body = body.child(widgets::label(
        "Amplify the current template between a forward and reverse primer. Primers may be given by name or sequence.",
        theme::T_SM,
        theme::MUTED,
    ));
    body = body.child(
        div()
            .flex()
            .items_center()
            .gap(px(10.0))
            .child(field_label("Forward"))
            .child(input_field(fi, 260.0)),
    );
    body = body.child(
        div()
            .flex()
            .items_center()
            .gap(px(10.0))
            .child(field_label("Reverse"))
            .child(input_field(ri, 260.0)),
    );
    body = body.child(
        div()
            .flex()
            .items_center()
            .gap(px(8.0))
            .child(
                check_box("pcr-copy", copy).on_click(cx.listener(|t, _, _, cx| {
                    if let Some(Dialog::Pcr(d)) = &mut t.app.dialog {
                        d.copy_features = !d.copy_features;
                    }
                    cx.notify();
                })),
            )
            .child(widgets::label(
                "Copy contained features into the product",
                theme::T_SM,
                theme::MUTED,
            )),
    );
    match &preview {
        Some(p) => {
            body = body.child(
                div()
                    .p(px(10.0))
                    .rounded(px(6.0))
                    .bg(theme::SURFACE)
                    .border_1()
                    .border_color(theme::BORDER)
                    .child(widgets::label(
                        &format!(
                            "Amplicon {} bp · {}..{} · fwd Tm {:.1}°C · rev Tm {:.1}°C",
                            p.len,
                            p.start + 1,
                            p.end,
                            p.fwd_site.tm,
                            p.rev_site.tm
                        ),
                        theme::T_SM,
                        theme::TEXT,
                    )),
            )
        }
        None => {
            body = body.child(widgets::label(
                "No amplicon yet — primers must bind in opposite orientations.",
                theme::T_SM,
                theme::FAINT,
            ))
        }
    }
    if let Some(e) = error {
        body = body.child(error_label(&e));
    }

    let footer = footer_row(vec![
        widgets::button("pcr-cancel", "Cancel", Btn::Ghost)
            .on_click(cx.listener(|t, _, _, cx| close_dialog(t, cx)))
            .into_any_element(),
        widgets::button("pcr-create", "Create product", Btn::Primary)
            .on_click(cx.listener(|t, _, _, cx| {
                let d = match &t.app.dialog {
                    Some(Dialog::Pcr(d)) => d.clone(),
                    _ => return,
                };
                match act::create_pcr_product(&mut t.app, &d) {
                    Ok(()) => close_dialog(t, cx),
                    Err(e) => {
                        if let Some(Dialog::Pcr(dd)) = &mut t.app.dialog {
                            dd.error = Some(e);
                        }
                        cx.notify();
                    }
                }
            }))
            .into_any_element(),
    ]);
    modal_shell(
        app,
        cx,
        "In-silico PCR",
        480.0,
        max_h,
        true,
        body.into_any_element(),
        Some(footer.into_any_element()),
    )
}

// ---------------------------------------------------------------------------
// Primer design
// ---------------------------------------------------------------------------

fn primer_design(app: &mut HelixBench, cx: &mut Context<HelixBench>, max_h: Pixels) -> AnyElement {
    let (sseed, lseed, tseed, oseed, error) = match &app.app.dialog {
        Some(Dialog::PrimerDesign(d)) => (
            d.start.clone(),
            d.len.clone(),
            d.target_tm.clone(),
            d.overhang.clone(),
            d.error.clone(),
        ),
        _ => (
            String::new(),
            String::new(),
            "60".into(),
            String::new(),
            None,
        ),
    };
    let si = app.input_entity(cx, "dlg:pd:start", &sseed, "1", true);
    let li = app.input_entity(cx, "dlg:pd:len", &lseed, "bp", true);
    let ti = app.input_entity(cx, "dlg:pd:tm", &tseed, "60", true);
    let oi = app.input_entity(cx, "dlg:pd:ov", &oseed, "optional 5' overhang", true);
    let (s, l, tm, ov) = (
        si.read(cx).text(),
        li.read(cx).text(),
        ti.read(cx).text(),
        oi.read(cx).text(),
    );
    if let Some(Dialog::PrimerDesign(d)) = &mut app.app.dialog {
        d.start = s;
        d.len = l;
        d.target_tm = tm;
        d.overhang = ov;
    }
    let draft = match &app.app.dialog {
        Some(Dialog::PrimerDesign(d)) => d.clone(),
        _ => {
            return modal_shell(
                app,
                cx,
                "Primer Design",
                560.0,
                max_h,
                true,
                div().into_any_element(),
                None,
            )
        }
    };

    let mut body = div().flex().flex_col().size_full().gap(px(10.0));
    body = body.child(widgets::label(
        "Design forward/reverse primers flanking a target region, scored by Tm, GC and 3' GC-clamp.",
        theme::T_SM,
        theme::MUTED,
    ));
    body = body.child(
        div()
            .flex()
            .items_center()
            .gap(px(8.0))
            .child(field_label("Start"))
            .child(input_field(si, 70.0))
            .child(field_label("Length"))
            .child(input_field(li, 70.0))
            .child(field_label("Tm"))
            .child(input_field(ti, 60.0))
            .child(field_label("Overhang"))
            .child(input_field(oi, 120.0)),
    );
    body = body.child(
        widgets::button("pd-run", "Design primers", Btn::Outline).on_click(cx.listener(
            |t, _, _, cx| {
                let mut d = match &t.app.dialog {
                    Some(Dialog::PrimerDesign(d)) => d.clone(),
                    _ => return,
                };
                act::run_primer_design(&mut t.app, &mut d);
                t.app.dialog = Some(Dialog::PrimerDesign(d));
                cx.notify();
            },
        )),
    );
    if let Some(e) = &error {
        body = body.child(error_label(e));
    }
    for (label, cands, pick, is_fwd) in [
        ("Forward candidates", &draft.fwd, draft.fwd_pick, true),
        ("Reverse candidates", &draft.rev, draft.rev_pick, false),
    ] {
        body = body.child(widgets::label(label, theme::T_SM, theme::FAINT));
        let mut list = div().flex().flex_col().gap(px(4.0));
        if cands.is_empty() {
            list = list.child(widgets::label(
                "Run design to list candidates.",
                theme::T_SM,
                theme::FAINT,
            ));
        }
        for (i, c) in cands.iter().enumerate() {
            let seq = String::from_utf8_lossy(&c.sequence).into_owned();
            let selected = i == pick;
            list = list.child(
                div()
                    .id(SharedString::from(format!(
                        "pd-{}-{}",
                        if is_fwd { "f" } else { "r" },
                        i
                    )))
                    .flex()
                    .items_center()
                    .gap(px(8.0))
                    .p(px(6.0))
                    .rounded(px(5.0))
                    .cursor_pointer()
                    .bg(if selected {
                        theme::SURFACE_2
                    } else {
                        theme::SURFACE
                    })
                    .border_1()
                    .border_color(if selected {
                        theme::ACCENT
                    } else {
                        theme::BORDER
                    })
                    .child(
                        div()
                            .flex_grow()
                            .text_size(px(theme::T_SM))
                            .font_family(theme::MONO_FONT)
                            .text_color(theme::TEXT)
                            .child(seq),
                    )
                    .child(widgets::label(
                        &format!(
                            "{}bp · Tm {:.1} · GC {:.0}%{}{}",
                            c.len,
                            c.tm,
                            c.gc,
                            if c.clamp { " · clamp" } else { "" },
                            {
                                let ss = domain::primer_struct::analyze(&c.sequence);
                                let mut s = String::new();
                                if let Some((stem, _)) = ss.hairpin {
                                    s.push_str(&format!(" · hp{stem}"));
                                }
                                if ss.dimer >= 3 {
                                    s.push_str(&format!(" · dim{}", ss.dimer));
                                }
                                s
                            }
                        ),
                        theme::T_XS,
                        {
                            let ss = domain::primer_struct::analyze(&c.sequence);
                            if ss.hairpin_warn() || ss.dimer_warn() {
                                theme::WARN
                            } else {
                                theme::MUTED
                            }
                        },
                    ))
                    .on_click(cx.listener(move |t, _, _, cx| {
                        if let Some(Dialog::PrimerDesign(d)) = &mut t.app.dialog {
                            if is_fwd {
                                d.fwd_pick = i;
                            } else {
                                d.rev_pick = i;
                            }
                        }
                        cx.notify();
                    })),
            );
        }
        body = body.child(list);
    }

    let footer = footer_row(vec![
        widgets::button("pd-cancel", "Cancel", Btn::Ghost)
            .on_click(cx.listener(|t, _, _, cx| close_dialog(t, cx)))
            .into_any_element(),
        widgets::button("pd-add", "Add primer pair", Btn::Primary)
            .on_click(cx.listener(|t, _, _, cx| {
                let d = match &t.app.dialog {
                    Some(Dialog::PrimerDesign(d)) => d.clone(),
                    _ => return,
                };
                match act::add_designed_primers(&mut t.app, &d) {
                    Ok(()) => close_dialog(t, cx),
                    Err(e) => {
                        if let Some(Dialog::PrimerDesign(dd)) = &mut t.app.dialog {
                            dd.error = Some(e);
                        }
                        cx.notify();
                    }
                }
            }))
            .into_any_element(),
    ]);
    modal_shell(
        app,
        cx,
        "Primer Design",
        620.0,
        max_h,
        true,
        body.into_any_element(),
        Some(footer.into_any_element()),
    )
}

// ---------------------------------------------------------------------------
// Virtual digest + gel
// ---------------------------------------------------------------------------

fn digest(app: &mut HelixBench, cx: &mut Context<HelixBench>, max_h: Pixels) -> AnyElement {
    let frags = act::digest_fragments(&app.app);
    let circular = app.app.doc.sequence.is_circular();
    let total: usize = frags.iter().map(|f| f.size).sum();
    let max_bp = frags.iter().map(|f| f.size).max().unwrap_or(1000).max(1000) as f32;

    let mut body = div().flex().flex_col().size_full().gap(px(10.0));
    body = body.child(widgets::label(
        &format!(
            "{} fragment(s) · {} bp total · {} template · {} enzyme(s) selected",
            frags.len(),
            total,
            if circular { "circular" } else { "linear" },
            app.app.analysis.chosen.len()
        ),
        theme::T_SM,
        theme::MUTED,
    ));

    // Agarose gel simulation: ladder lane + sample lane.
    let gel_frags = frags.clone();
    let gel = div().h(px(240.0)).w_full().flex_shrink_0().child(
        canvas(
            move |_, _, _| {},
            move |bounds, _s, window, cx| {
                crate::draw::rect(window, bounds, 6.0, theme::SURFACE);
                let top = bounds.origin.y.v() + 16.0;
                let h = bounds.size.height.v() - 32.0;
                let ladder_x = bounds.origin.x.v() + 70.0;
                let lane_x = bounds.origin.x.v() + 180.0;
                let band_w = 60.0;
                for &m in &domain::digest::LADDER {
                    let y = top + domain::digest::gel_fraction(m, 100.0, max_bp) * h;
                    crate::draw::rect(
                        window,
                        Bounds::new(point(px(ladder_x), px(y)), size(px(band_w), px(3.0))),
                        1.0,
                        theme::FAINT,
                    );
                    crate::draw::text(
                        window,
                        cx,
                        point(px(ladder_x - 8.0), px(y + 1.5)),
                        &m.to_string(),
                        9.0,
                        theme::FAINT,
                        theme::mono_font(),
                        crate::draw::Align::Right,
                    );
                }
                for f in &gel_frags {
                    let y = top + domain::digest::gel_fraction(f.size, 100.0, max_bp) * h;
                    crate::draw::rect(
                        window,
                        Bounds::new(point(px(lane_x), px(y)), size(px(band_w), px(4.0))),
                        1.0,
                        theme::ACCENT,
                    );
                    crate::draw::text(
                        window,
                        cx,
                        point(px(lane_x + band_w + 8.0), px(y + 2.0)),
                        &format!("{} bp", f.size),
                        9.0,
                        theme::MUTED,
                        theme::mono_font(),
                        crate::draw::Align::Left,
                    );
                }
                crate::draw::text(
                    window,
                    cx,
                    point(px(ladder_x + band_w / 2.0), px(top - 8.0)),
                    "Ladder",
                    9.0,
                    theme::FAINT,
                    theme::ui_font(),
                    crate::draw::Align::Center,
                );
                crate::draw::text(
                    window,
                    cx,
                    point(px(lane_x + band_w / 2.0), px(top - 8.0)),
                    "Digest",
                    9.0,
                    theme::FAINT,
                    theme::ui_font(),
                    crate::draw::Align::Center,
                );
            },
        )
        .size_full(),
    );
    body = body.child(gel);

    let mut list = div().flex().flex_col().gap(px(2.0));
    for (i, f) in frags.iter().enumerate() {
        list = list.child(
            div()
                .flex()
                .items_center()
                .gap(px(10.0))
                .p(px(6.0))
                .rounded(px(4.0))
                .bg(if i % 2 == 0 {
                    theme::SURFACE
                } else {
                    theme::PANEL
                })
                .child(widgets::label(
                    &format!("#{}", i + 1),
                    theme::T_XS,
                    theme::FAINT,
                ))
                .child(
                    div()
                        .flex_grow()
                        .text_size(px(theme::T_SM))
                        .font_family(theme::MONO_FONT)
                        .text_color(theme::TEXT)
                        .child(format!("{} bp", f.size)),
                )
                .child(widgets::label(
                    &format!("{}..{}", f.start + 1, f.end),
                    theme::T_XS,
                    theme::MUTED,
                )),
        );
    }
    body = body.child(list);

    let footer = footer_row(vec![widgets::button("dg-close", "Close", Btn::Ghost)
        .on_click(cx.listener(|t, _, _, cx| close_dialog(t, cx)))
        .into_any_element()]);
    modal_shell(
        app,
        cx,
        "Virtual Digest & Gel",
        520.0,
        max_h,
        true,
        body.into_any_element(),
        Some(footer.into_any_element()),
    )
}

// ---------------------------------------------------------------------------
// Sequence alignment
// ---------------------------------------------------------------------------

fn alignment(app: &mut HelixBench, cx: &mut Context<HelixBench>, max_h: Pixels) -> AnyElement {
    let draft = match &app.app.dialog {
        Some(Dialog::Alignment(d)) => d.clone(),
        _ => {
            return modal_shell(
                app,
                cx,
                "Sequence Alignment",
                640.0,
                max_h,
                true,
                div().into_any_element(),
                None,
            )
        }
    };
    let names = if !draft.labels.is_empty() {
        draft.labels.clone()
    } else {
        let mut n = vec![app.app.doc.name.clone()];
        for od in &app.app.library {
            n.push(od.doc.name.clone());
        }
        n
    };

    let mut body = div().flex().flex_col().size_full().gap(px(10.0));
    body = body.child(widgets::label(
        "Center-star alignment of the open documents against a reference, with a consensus row. Circular sequences can be auto-reindexed.",
        theme::T_SM,
        theme::MUTED,
    ));
    body = body.child(widgets::label("Reference", theme::T_SM, theme::FAINT));
    let mut refs = div().flex().flex_col().gap(px(4.0));
    for (i, n) in names.iter().enumerate() {
        let selected = i == draft.reference;
        refs = refs.child(
            div()
                .id(SharedString::from(format!("al-ref-{i}")))
                .flex()
                .items_center()
                .gap(px(8.0))
                .p(px(6.0))
                .rounded(px(5.0))
                .cursor_pointer()
                .bg(if selected {
                    theme::SURFACE_2
                } else {
                    theme::SURFACE
                })
                .border_1()
                .border_color(if selected {
                    theme::ACCENT
                } else {
                    theme::BORDER
                })
                .child(check_box(
                    SharedString::from(format!("al-refc-{i}")),
                    selected,
                ))
                .child(widgets::label(n, theme::T_SM, theme::TEXT))
                .on_click(cx.listener(move |t, _, _, cx| {
                    if let Some(Dialog::Alignment(d)) = &mut t.app.dialog {
                        d.reference = i;
                    }
                    cx.notify();
                })),
        );
    }
    body = body.child(refs);
    body = body.child(
        div()
            .flex()
            .items_center()
            .gap(px(8.0))
            .child(
                check_box("al-reindex", draft.reindex).on_click(cx.listener(|t, _, _, cx| {
                    if let Some(Dialog::Alignment(d)) = &mut t.app.dialog {
                        d.reindex = !d.reindex;
                    }
                    cx.notify();
                })),
            )
            .child(widgets::label(
                "Auto-reindex circular sequences",
                theme::T_SM,
                theme::MUTED,
            )),
    );
    body = body.child(
        widgets::button("al-run", "Align", Btn::Outline).on_click(cx.listener(|t, _, _, cx| {
            let mut d = match &t.app.dialog {
                Some(Dialog::Alignment(d)) => d.clone(),
                _ => return,
            };
            act::run_alignment(&mut t.app, &mut d);
            t.app.dialog = Some(Dialog::Alignment(d));
            cx.notify();
        })),
    );
    if draft.result.is_some() {
        let name_inp = app.input_entity(
            cx,
            "dlg:al-save-name",
            "",
            "Alignment name (optional)",
            false,
        );
        body = body.child(
            div()
                .flex()
                .items_center()
                .gap(px(8.0))
                .child(div().w(px(240.0)).child(name_inp))
                .child(
                    widgets::button("al-save", "Save alignment", Btn::Primary).on_click(
                        cx.listener(|t, _, _, cx| {
                            let name = t
                                .input_entity(
                                    cx,
                                    "dlg:al-save-name",
                                    "",
                                    "Alignment name (optional)",
                                    false,
                                )
                                .read(cx)
                                .text();
                            act::alignment_save(&mut t.app, &name);
                            cx.notify();
                        }),
                    ),
                ),
        );
    }
    if let Some(e) = &draft.error {
        body = body.child(error_label(e));
    }
    if let Some(al) = &draft.result {
        body = body.child(widgets::label(
            &format!(
                "{} columns · identity {:.1}%",
                al.consensus.len(),
                al.identity
            ),
            theme::T_SM,
            theme::TEXT,
        ));
        let block = 60usize;
        let cols = al.consensus.len();
        let mut start = 0usize;
        let mut blocks = 0usize;
        while start < cols && blocks < 12 {
            let end = (start + block).min(cols);
            let mut grid = div()
                .flex()
                .flex_col()
                .gap(px(2.0))
                .p(px(6.0))
                .rounded(px(5.0))
                .bg(theme::SURFACE);
            for (i, row) in al.rows.iter().enumerate() {
                let label = names
                    .get(i)
                    .cloned()
                    .unwrap_or_else(|| format!("seq{}", i + 1));
                let seg = String::from_utf8_lossy(&row[start..end]).into_owned();
                grid = grid.child(
                    div()
                        .flex()
                        .gap(px(8.0))
                        .child(
                            div()
                                .w(px(90.0))
                                .text_size(px(theme::T_XS))
                                .text_color(theme::FAINT)
                                .child(label),
                        )
                        .child(
                            div()
                                .text_size(px(theme::T_XS))
                                .font_family(theme::MONO_FONT)
                                .text_color(theme::TEXT)
                                .child(seg),
                        ),
                );
            }
            let cseg = String::from_utf8_lossy(&al.consensus[start..end]).into_owned();
            grid = grid.child(
                div()
                    .flex()
                    .gap(px(8.0))
                    .child(
                        div()
                            .w(px(90.0))
                            .text_size(px(theme::T_XS))
                            .text_color(theme::ACCENT)
                            .child("Consensus"),
                    )
                    .child(
                        div()
                            .text_size(px(theme::T_XS))
                            .font_family(theme::MONO_FONT)
                            .text_color(theme::ACCENT)
                            .child(cseg),
                    ),
            );
            body = body.child(grid);
            start = end;
            blocks += 1;
        }
        if start < cols {
            body = body.child(widgets::label(
                &format!("… {} more columns", cols - start),
                theme::T_XS,
                theme::FAINT,
            ));
        }
    }

    body = body.child(widgets::label(
        "Saved alignments",
        theme::T_SM,
        theme::FAINT,
    ));
    if app.app.alignments.items.is_empty() {
        body = body.child(widgets::label(
            "No saved alignments yet. Run an alignment and save it to reuse later.",
            theme::T_XS,
            theme::FAINT,
        ));
    } else {
        let mut list = div().flex().flex_col().gap(px(4.0));
        for a in app.app.alignments.items.clone() {
            let id = a.id;
            list = list.child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(8.0))
                    .p(px(6.0))
                    .rounded(px(5.0))
                    .bg(theme::SURFACE)
                    .border_1()
                    .border_color(theme::BORDER)
                    .child(
                        div()
                            .flex_grow()
                            .flex()
                            .flex_col()
                            .gap(px(2.0))
                            .child(widgets::label(&a.name, theme::T_SM, theme::TEXT))
                            .child(widgets::label(
                                &format!(
                                    "{} columns · identity {:.1}%",
                                    a.result.consensus.len(),
                                    a.result.identity
                                ),
                                theme::T_XS,
                                theme::FAINT,
                            )),
                    )
                    .child(
                        widgets::button(
                            SharedString::from(format!("al-load-{id}")),
                            "Load",
                            Btn::Subtle,
                        )
                        .on_click(cx.listener(move |t, _, _, cx| {
                            act::alignment_load_saved(&mut t.app, id);
                            cx.notify();
                        })),
                    )
                    .child(
                        widgets::button(
                            SharedString::from(format!("al-del-{id}")),
                            "Delete",
                            Btn::Ghost,
                        )
                        .on_click(cx.listener(move |t, _, _, cx| {
                            act::alignment_delete_saved(&mut t.app, id);
                            cx.notify();
                        })),
                    ),
            );
        }
        body = body.child(list);
    }

    let footer = footer_row(vec![widgets::button("al-close", "Close", Btn::Ghost)
        .on_click(cx.listener(|t, _, _, cx| close_dialog(t, cx)))
        .into_any_element()]);
    modal_shell(
        app,
        cx,
        "Sequence Alignment",
        700.0,
        max_h,
        true,
        body.into_any_element(),
        Some(footer.into_any_element()),
    )
}

// ---------------------------------------------------------------------------
// Assembly wizard
// ---------------------------------------------------------------------------

fn assembly(app: &mut HelixBench, cx: &mut Context<HelixBench>, max_h: Pixels) -> AnyElement {
    let draft = match &app.app.dialog {
        Some(Dialog::Assembly(d)) => d.clone(),
        _ => {
            return modal_shell(
                app,
                cx,
                "Assembly Wizard",
                520.0,
                max_h,
                true,
                div().into_any_element(),
                None,
            )
        }
    };
    let mut body = div().flex().flex_col().size_full().gap(px(10.0));
    body = body.child(widgets::label(
        "Join insert documents into the current vector (linearized at the cut site). Gibson trims homologous overlaps.",
        theme::T_SM,
        theme::MUTED,
    ));
    body = body.child(widgets::label("Strategy", theme::T_SM, theme::FAINT));
    let mut strat = div().flex().gap(px(6.0));
    for (i, label) in ["Digest & Ligate", "Gibson", "Golden Gate"]
        .iter()
        .enumerate()
    {
        let sel = draft.strategy == i;
        strat = strat.child(
            widgets::button(
                SharedString::from(format!("as-strat-{i}")),
                label,
                if sel { Btn::Primary } else { Btn::Outline },
            )
            .on_click(cx.listener(move |t, _, _, cx| {
                if let Some(Dialog::Assembly(d)) = &mut t.app.dialog {
                    d.strategy = i;
                }
                cx.notify();
            })),
        );
    }
    body = body.child(strat);
    let ci = app.input_entity(cx, "dlg:as:cut", &draft.cut, "1", true);
    let cut = ci.read(cx).text();
    if let Some(Dialog::Assembly(d)) = &mut app.app.dialog {
        d.cut = cut;
    }
    body = body.child(
        div()
            .flex()
            .items_center()
            .gap(px(8.0))
            .child(field_label("Cut position"))
            .child(input_field(ci, 90.0)),
    );
    body = body.child(widgets::label(
        "Inserts (open documents)",
        theme::T_SM,
        theme::FAINT,
    ));
    if app.app.library.is_empty() {
        body = body.child(widgets::label(
            "Open a second document (e.g. PCR product) to use as an insert.",
            theme::T_SM,
            theme::FAINT,
        ));
    }
    for (i, od) in app.app.library.iter().enumerate() {
        let on = draft.inserts.get(i).copied().unwrap_or(false);
        let name = od.doc.name.clone();
        body = body.child(
            div()
                .flex()
                .items_center()
                .gap(px(8.0))
                .child(
                    check_box(SharedString::from(format!("as-ins-{i}")), on).on_click(cx.listener(
                        move |t, _, _, cx| {
                            if let Some(Dialog::Assembly(d)) = &mut t.app.dialog {
                                if let Some(b) = d.inserts.get_mut(i) {
                                    *b = !*b;
                                }
                            }
                            cx.notify();
                        },
                    )),
                )
                .child(widgets::label(&name, theme::T_SM, theme::TEXT)),
        );
    }
    if let Some(e) = &draft.error {
        body = body.child(error_label(e));
    }
    body = body.child(
        widgets::button(
            "as-extras",
            "Design Gibson primers / validate overhangs",
            Btn::Outline,
        )
        .on_click(cx.listener(|t, _, _, cx| {
            let mut d = match &t.app.dialog {
                Some(Dialog::Assembly(d)) => d.clone(),
                _ => return,
            };
            act::assembly_extras(&mut t.app, &mut d);
            t.app.dialog = Some(Dialog::Assembly(d));
            cx.notify();
        })),
    );
    if !draft.primers.is_empty() {
        body = body.child(widgets::label(
            "Gibson junction primers",
            theme::T_SM,
            theme::FAINT,
        ));
        let mut pl = div().flex().flex_col().gap(px(2.0));
        for p in &draft.primers {
            pl = pl.child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(1.0))
                    .p(px(4.0))
                    .rounded(px(4.0))
                    .bg(theme::SURFACE)
                    .child(widgets::label(&p.name, theme::T_XS, theme::TEXT))
                    .child(
                        div()
                            .text_size(px(theme::T_XS))
                            .font_family(theme::MONO_FONT)
                            .text_color(theme::MUTED)
                            .child(format!("F: {}", String::from_utf8_lossy(&p.forward))),
                    )
                    .child(
                        div()
                            .text_size(px(theme::T_XS))
                            .font_family(theme::MONO_FONT)
                            .text_color(theme::MUTED)
                            .child(format!("R: {}", String::from_utf8_lossy(&p.reverse))),
                    ),
            );
        }
        body = body.child(pl);
    }
    if !draft.gg_issues.is_empty() {
        for issue in &draft.gg_issues {
            body = body.child(error_label(issue));
        }
    } else if !draft.primers.is_empty() {
        body = body.child(widgets::label(
            "Golden Gate overhangs OK (unique, non-palindromic)",
            theme::T_XS,
            theme::OK,
        ));
    }
    let footer = footer_row(vec![
        widgets::button("as-cancel", "Cancel", Btn::Ghost)
            .on_click(cx.listener(|t, _, _, cx| close_dialog(t, cx)))
            .into_any_element(),
        widgets::button("as-go", "Assemble", Btn::Primary)
            .on_click(cx.listener(|t, _, _, cx| {
                let d = match &t.app.dialog {
                    Some(Dialog::Assembly(d)) => d.clone(),
                    _ => return,
                };
                match act::create_assembly(&mut t.app, &d) {
                    Ok(()) => close_dialog(t, cx),
                    Err(e) => {
                        if let Some(Dialog::Assembly(dd)) = &mut t.app.dialog {
                            dd.error = Some(e);
                        }
                        cx.notify();
                    }
                }
            }))
            .into_any_element(),
    ]);
    modal_shell(
        app,
        cx,
        "Assembly Wizard",
        520.0,
        max_h,
        true,
        body.into_any_element(),
        Some(footer.into_any_element()),
    )
}

// ---------------------------------------------------------------------------
// CRISPR guides
// ---------------------------------------------------------------------------

fn crispr(app: &mut HelixBench, cx: &mut Context<HelixBench>, max_h: Pixels) -> AnyElement {
    let draft = match &app.app.dialog {
        Some(Dialog::Crispr(d)) => d.clone(),
        _ => {
            return modal_shell(
                app,
                cx,
                "CRISPR Guides",
                620.0,
                max_h,
                true,
                div().into_any_element(),
                None,
            )
        }
    };
    let li = app.input_entity(cx, "dlg:cr:limit", &draft.limit, "20", true);
    let limit = li.read(cx).text();
    if let Some(Dialog::Crispr(d)) = &mut app.app.dialog {
        d.limit = limit;
    }
    let mut body = div().flex().flex_col().size_full().gap(px(10.0));
    body = body.child(widgets::label(
        "Scan both strands for NGG PAMs; guides ranked by on-target score with off-target counts.",
        theme::T_SM,
        theme::MUTED,
    ));
    body = body.child(
        div()
            .flex()
            .items_center()
            .gap(px(8.0))
            .child(field_label("Max guides"))
            .child(input_field(li, 70.0))
            .child(
                widgets::button("cr-scan", "Scan", Btn::Outline).on_click(cx.listener(
                    |t, _, _, cx| {
                        let mut d = match &t.app.dialog {
                            Some(Dialog::Crispr(d)) => d.clone(),
                            _ => return,
                        };
                        act::run_crispr(&mut t.app, &mut d);
                        t.app.dialog = Some(Dialog::Crispr(d));
                        cx.notify();
                    },
                )),
            ),
    );
    if let Some(e) = &draft.error {
        body = body.child(error_label(e));
    }
    let mut list = div().flex().flex_col().gap(px(3.0));
    for (i, g) in draft.guides.iter().enumerate() {
        let seq = String::from_utf8_lossy(&g.sequence).into_owned();
        list = list.child(
            div()
                .flex()
                .items_center()
                .gap(px(8.0))
                .p(px(6.0))
                .rounded(px(5.0))
                .bg(if i % 2 == 0 {
                    theme::SURFACE
                } else {
                    theme::PANEL
                })
                .child(
                    div()
                        .w(px(24.0))
                        .text_size(px(theme::T_XS))
                        .text_color(theme::FAINT)
                        .child(format!("{}", i + 1)),
                )
                .child(
                    div()
                        .flex_grow()
                        .text_size(px(theme::T_XS))
                        .font_family(theme::MONO_FONT)
                        .text_color(theme::TEXT)
                        .child(seq),
                )
                .child(widgets::label(&g.pam, theme::T_XS, theme::MUTED))
                .child(widgets::label(
                    if g.strand == Strand::Top { "+" } else { "-" },
                    theme::T_XS,
                    theme::MUTED,
                ))
                .child(widgets::label(
                    &format!("{:.2}", g.score),
                    theme::T_XS,
                    theme::ACCENT,
                ))
                .child(widgets::label(
                    &format!("off:{}", g.off_targets),
                    theme::T_XS,
                    theme::FAINT,
                )),
        );
    }
    body = body.child(list);
    if !draft.off.is_empty() {
        body = body.child(widgets::label(
            "CFD off-target sites (top guide)",
            theme::T_SM,
            theme::FAINT,
        ));
        let mut ol = div().flex().flex_col().gap(px(2.0));
        for o in draft.off.iter().take(8) {
            ol = ol.child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(8.0))
                    .p(px(4.0))
                    .rounded(px(4.0))
                    .bg(theme::SURFACE)
                    .child(
                        div()
                            .flex_grow()
                            .text_size(px(theme::T_XS))
                            .text_color(theme::TEXT)
                            .child(o.name.clone()),
                    )
                    .child(widgets::label(
                        &format!("{}..{}", o.position + 1, o.position + 21),
                        theme::T_XS,
                        theme::MUTED,
                    ))
                    .child(widgets::label(
                        if o.strand == Strand::Top { "+" } else { "-" },
                        theme::T_XS,
                        theme::MUTED,
                    ))
                    .child(widgets::label(
                        &format!("mm {}", o.mismatches),
                        theme::T_XS,
                        theme::FAINT,
                    ))
                    .child(widgets::label(
                        &format!("CFD {:.2}", o.cfd),
                        theme::T_XS,
                        theme::ACCENT,
                    )),
            );
        }
        body = body.child(ol);
    }
    let footer = footer_row(vec![
        widgets::button("cr-close", "Close", Btn::Ghost)
            .on_click(cx.listener(|t, _, _, cx| close_dialog(t, cx)))
            .into_any_element(),
        widgets::button("cr-add", "Add as features", Btn::Primary)
            .on_click(cx.listener(|t, _, _, cx| {
                let d = match &t.app.dialog {
                    Some(Dialog::Crispr(d)) => d.clone(),
                    _ => return,
                };
                act::add_crispr_features(&mut t.app, &d);
                close_dialog(t, cx);
            }))
            .into_any_element(),
    ]);
    modal_shell(
        app,
        cx,
        "CRISPR Guides",
        680.0,
        max_h,
        true,
        body.into_any_element(),
        Some(footer.into_any_element()),
    )
}

// ---------------------------------------------------------------------------
// Codon optimization
// ---------------------------------------------------------------------------

fn codon(app: &mut HelixBench, cx: &mut Context<HelixBench>, max_h: Pixels) -> AnyElement {
    let draft = match &app.app.dialog {
        Some(Dialog::Codon(d)) => d.clone(),
        _ => {
            return modal_shell(
                app,
                cx,
                "Codon Optimization",
                520.0,
                max_h,
                true,
                div().into_any_element(),
                None,
            )
        }
    };
    let mut body = div().flex().flex_col().size_full().gap(px(10.0));
    body = body.child(widgets::label(
        "Re-encode the current sequence with host-preferred codons, preserving the protein.",
        theme::T_SM,
        theme::MUTED,
    ));
    let mut hosts = div().flex().gap(px(6.0));
    for (i, label) in ["E. coli", "Human"].iter().enumerate() {
        let sel = draft.host == i;
        hosts = hosts.child(
            widgets::button(
                SharedString::from(format!("cd-host-{i}")),
                label,
                if sel { Btn::Primary } else { Btn::Outline },
            )
            .on_click(cx.listener(move |t, _, _, cx| {
                if let Some(Dialog::Codon(d)) = &mut t.app.dialog {
                    d.host = i;
                }
                cx.notify();
            })),
        );
    }
    body = body.child(hosts);
    let gmin = app.input_entity(cx, "dlg:cd:gmin", &draft.gc_min, "0.3", true);
    let gmax = app.input_entity(cx, "dlg:cd:gmax", &draft.gc_max, "0.7", true);
    let av = app.input_entity(
        cx,
        "dlg:cd:avoid",
        &draft.avoid,
        "sites to avoid, comma-sep",
        true,
    );
    let (gmin_s, gmax_s, av_s) = (
        gmin.read(cx).text(),
        gmax.read(cx).text(),
        av.read(cx).text(),
    );
    if let Some(Dialog::Codon(d)) = &mut app.app.dialog {
        d.gc_min = gmin_s;
        d.gc_max = gmax_s;
        d.avoid = av_s;
    }
    body = body.child(
        div()
            .flex()
            .items_center()
            .gap(px(6.0))
            .child(field_label("GC min"))
            .child(input_field(gmin, 60.0))
            .child(field_label("GC max"))
            .child(input_field(gmax, 60.0))
            .child(
                check_box("cd-harm", draft.harmonize).on_click(cx.listener(|t, _, _, cx| {
                    if let Some(Dialog::Codon(d)) = &mut t.app.dialog {
                        d.harmonize = !d.harmonize;
                    }
                    cx.notify();
                })),
            )
            .child(widgets::label("harmonize", theme::T_XS, theme::MUTED)),
    );
    body = body.child(
        div()
            .flex()
            .items_center()
            .gap(px(6.0))
            .child(field_label("Avoid"))
            .child(input_field(av, 260.0)),
    );
    body = body.child(
        widgets::button("cd-run", "Optimize", Btn::Outline).on_click(cx.listener(|t, _, _, cx| {
            let mut d = match &t.app.dialog {
                Some(Dialog::Codon(d)) => d.clone(),
                _ => return,
            };
            act::run_codon(&mut t.app, &mut d);
            t.app.dialog = Some(Dialog::Codon(d));
            cx.notify();
        })),
    );
    if let Some(e) = &draft.error {
        body = body.child(error_label(e));
    }
    if draft.result.is_some() {
        body = body.child(widgets::label(
            &format!(
                "CAI before {:.2} → after {:.2}",
                draft.cai_before, draft.cai_after
            ),
            theme::T_SM,
            theme::TEXT,
        ));
    }
    let footer = footer_row(vec![
        widgets::button("cd-close", "Close", Btn::Ghost)
            .on_click(cx.listener(|t, _, _, cx| close_dialog(t, cx)))
            .into_any_element(),
        widgets::button("cd-apply", "Create optimized product", Btn::Primary)
            .on_click(cx.listener(|t, _, _, cx| {
                let d = match &t.app.dialog {
                    Some(Dialog::Codon(d)) => d.clone(),
                    _ => return,
                };
                match act::apply_codon(&mut t.app, &d) {
                    Ok(()) => close_dialog(t, cx),
                    Err(e) => {
                        if let Some(Dialog::Codon(dd)) = &mut t.app.dialog {
                            dd.error = Some(e);
                        }
                        cx.notify();
                    }
                }
            }))
            .into_any_element(),
    ]);
    modal_shell(
        app,
        cx,
        "Codon Optimization",
        520.0,
        max_h,
        true,
        body.into_any_element(),
        Some(footer.into_any_element()),
    )
}

// ---------------------------------------------------------------------------
// Local BLAST
// ---------------------------------------------------------------------------

fn blast(app: &mut HelixBench, cx: &mut Context<HelixBench>, max_h: Pixels) -> AnyElement {
    let draft = match &app.app.dialog {
        Some(Dialog::Blast(d)) => d.clone(),
        _ => {
            return modal_shell(
                app,
                cx,
                "Local BLAST",
                640.0,
                max_h,
                true,
                div().into_any_element(),
                None,
            )
        }
    };
    let ki = app.input_entity(cx, "dlg:bl:k", &draft.k, "11", true);
    let mi = app.input_entity(cx, "dlg:bl:max", &draft.max, "20", true);
    let (k, max) = (ki.read(cx).text(), mi.read(cx).text());
    if let Some(Dialog::Blast(d)) = &mut app.app.dialog {
        d.k = k;
        d.max = max;
    }
    let mut body = div().flex().flex_col().size_full().gap(px(10.0));
    body = body.child(widgets::label(
        "Seed-and-extend search of the current sequence against a database (loaded FASTA or open documents).",
        theme::T_SM,
        theme::MUTED,
    ));
    body = body.child(
        div()
            .flex()
            .items_center()
            .gap(px(8.0))
            .child(
                widgets::button("bl-load", "Load FASTA database", Btn::Outline).on_click(
                    cx.listener(|t, _, _, cx| {
                        act::load_blast_db(&mut t.app);
                        if let Some(Dialog::Blast(d)) = &mut t.app.dialog {
                            d.db_count = t.app.blast_db.len();
                        }
                        cx.notify();
                    }),
                ),
            )
            .child(widgets::label(
                &format!("db: {} sequences", draft.db_count),
                theme::T_XS,
                theme::FAINT,
            )),
    );
    body = body.child(
        div()
            .flex()
            .items_center()
            .gap(px(8.0))
            .child(field_label("Word size"))
            .child(input_field(ki, 60.0))
            .child(field_label("Max hits"))
            .child(input_field(mi, 60.0))
            .child(
                widgets::button("bl-run", "Search", Btn::Outline).on_click(cx.listener(
                    |t, _, _, cx| {
                        let mut d = match &t.app.dialog {
                            Some(Dialog::Blast(d)) => d.clone(),
                            _ => return,
                        };
                        act::run_blast(&mut t.app, &mut d);
                        t.app.dialog = Some(Dialog::Blast(d));
                        cx.notify();
                    },
                )),
            ),
    );
    if let Some(e) = &draft.error {
        body = body.child(error_label(e));
    }
    let mut list = div().flex().flex_col().gap(px(3.0));
    for (i, h) in draft.hits.iter().enumerate() {
        list = list.child(
            div()
                .flex()
                .items_center()
                .gap(px(8.0))
                .p(px(6.0))
                .rounded(px(5.0))
                .bg(if i % 2 == 0 {
                    theme::SURFACE
                } else {
                    theme::PANEL
                })
                .child(
                    div()
                        .flex_grow()
                        .text_size(px(theme::T_SM))
                        .text_color(theme::TEXT)
                        .child(h.name.clone()),
                )
                .child(widgets::label(
                    &format!("q{}..{}", h.qstart + 1, h.qend),
                    theme::T_XS,
                    theme::MUTED,
                ))
                .child(widgets::label(
                    &format!("s{}..{}", h.sstart + 1, h.send),
                    theme::T_XS,
                    theme::MUTED,
                ))
                .child(widgets::label(
                    &format!("{}", h.score),
                    theme::T_XS,
                    theme::ACCENT,
                ))
                .child(widgets::label(
                    &format!("{:.0}%", h.identity),
                    theme::T_XS,
                    theme::MUTED,
                )),
        );
    }
    body = body.child(list);
    let footer = footer_row(vec![widgets::button("bl-close", "Close", Btn::Ghost)
        .on_click(cx.listener(|t, _, _, cx| close_dialog(t, cx)))
        .into_any_element()]);
    modal_shell(
        app,
        cx,
        "Local BLAST",
        680.0,
        max_h,
        true,
        body.into_any_element(),
        Some(footer.into_any_element()),
    )
}

// ---------------------------------------------------------------------------
// Combinatorial assembly
// ---------------------------------------------------------------------------

fn combinatorial(app: &mut HelixBench, cx: &mut Context<HelixBench>, max_h: Pixels) -> AnyElement {
    let draft = match &app.app.dialog {
        Some(Dialog::Combinatorial(d)) => d.clone(),
        _ => {
            return modal_shell(
                app,
                cx,
                "Combinatorial Assembly",
                620.0,
                max_h,
                true,
                div().into_any_element(),
                None,
            )
        }
    };
    let ci = app.input_entity(cx, "dlg:cb:cut", &draft.cut, "1", true);
    let capi = app.input_entity(cx, "dlg:cb:cap", &draft.cap, "24", true);
    let (cut, cap) = (ci.read(cx).text(), capi.read(cx).text());
    if let Some(Dialog::Combinatorial(d)) = &mut app.app.dialog {
        d.cut = cut;
        d.cap = cap;
    }
    let mut body = div().flex().flex_col().size_full().gap(px(10.0));
    body = body.child(widgets::label(
        "Generate constructs for every ordering of the open insert documents (capped).",
        theme::T_SM,
        theme::MUTED,
    ));
    let mut strat = div().flex().gap(px(6.0));
    for (i, label) in ["Digest & Ligate", "Gibson", "Golden Gate"]
        .iter()
        .enumerate()
    {
        let sel = draft.strategy == i;
        strat = strat.child(
            widgets::button(
                SharedString::from(format!("cb-strat-{i}")),
                label,
                if sel { Btn::Primary } else { Btn::Outline },
            )
            .on_click(cx.listener(move |t, _, _, cx| {
                if let Some(Dialog::Combinatorial(d)) = &mut t.app.dialog {
                    d.strategy = i;
                }
                cx.notify();
            })),
        );
    }
    body = body.child(strat);
    body = body.child(
        div()
            .flex()
            .items_center()
            .gap(px(8.0))
            .child(field_label("Cut"))
            .child(input_field(ci, 70.0))
            .child(field_label("Cap"))
            .child(input_field(capi, 70.0))
            .child(
                widgets::button("cb-run", "Generate", Btn::Outline).on_click(cx.listener(
                    |t, _, _, cx| {
                        let mut d = match &t.app.dialog {
                            Some(Dialog::Combinatorial(d)) => d.clone(),
                            _ => return,
                        };
                        act::run_combinatorial(&mut t.app, &mut d);
                        t.app.dialog = Some(Dialog::Combinatorial(d));
                        cx.notify();
                    },
                )),
            ),
    );
    if let Some(e) = &draft.error {
        body = body.child(error_label(e));
    }
    let mut list = div().flex().flex_col().gap(px(3.0));
    for (i, asm) in draft.results.iter().enumerate() {
        let label = draft.labels.get(i).cloned().unwrap_or_default();
        list = list.child(
            div()
                .flex()
                .items_center()
                .gap(px(8.0))
                .p(px(6.0))
                .rounded(px(5.0))
                .bg(if i % 2 == 0 {
                    theme::SURFACE
                } else {
                    theme::PANEL
                })
                .child(
                    div()
                        .flex_grow()
                        .text_size(px(theme::T_XS))
                        .text_color(theme::TEXT)
                        .child(label),
                )
                .child(widgets::label(
                    &format!("{} bp", asm.bases.len()),
                    theme::T_XS,
                    theme::MUTED,
                ))
                .child(
                    widgets::button(
                        SharedString::from(format!("cb-create-{i}")),
                        "Create",
                        Btn::Subtle,
                    )
                    .on_click(cx.listener(move |t, _, _, cx| {
                        let d = match &t.app.dialog {
                            Some(Dialog::Combinatorial(d)) => d.clone(),
                            _ => return,
                        };
                        if act::create_combinatorial(&mut t.app, &d, i).is_ok() {
                            close_dialog(t, cx);
                        }
                    })),
                ),
        );
    }
    body = body.child(list);
    let footer = footer_row(vec![widgets::button("cb-close", "Close", Btn::Ghost)
        .on_click(cx.listener(|t, _, _, cx| close_dialog(t, cx)))
        .into_any_element()]);
    modal_shell(
        app,
        cx,
        "Combinatorial Assembly",
        680.0,
        max_h,
        true,
        body.into_any_element(),
        Some(footer.into_any_element()),
    )
}

// ---------------------------------------------------------------------------
// Local registry (catalog)
// ---------------------------------------------------------------------------

fn registry(app: &mut HelixBench, cx: &mut Context<HelixBench>, max_h: Pixels) -> AnyElement {
    let draft = match &app.app.dialog {
        Some(Dialog::Registry(d)) => d.clone(),
        _ => {
            return modal_shell(
                app,
                cx,
                "Local Registry",
                700.0,
                max_h,
                true,
                div().into_any_element(),
                None,
            )
        }
    };
    let mut body = div().flex().flex_col().size_full().gap(px(10.0));
    body = body.child(widgets::label(
        "Offline catalog of plasmids, strains and primers. Entries can be linked to the active document.",
        theme::T_SM,
        theme::MUTED,
    ));
    let mut bar = div().flex().items_center().gap(px(6.0));
    for (i, label) in ["All", "Plasmid", "Strain", "Primer"].iter().enumerate() {
        let sel = draft.filter == i;
        bar = bar.child(
            widgets::button(
                SharedString::from(format!("rg-filter-{i}")),
                label,
                if sel { Btn::Primary } else { Btn::Outline },
            )
            .on_click(cx.listener(move |t, _, _, cx| {
                if let Some(Dialog::Registry(d)) = &mut t.app.dialog {
                    d.filter = i;
                }
                cx.notify();
            })),
        );
    }
    bar = bar.child(div().flex_grow());
    bar = bar.child(
        widgets::button("rg-add", "Add entry", Btn::Primary).on_click(cx.listener(
            |t, _, _, cx| {
                let kind = match &t.app.dialog {
                    Some(Dialog::Registry(d)) if d.filter >= 1 => d.filter - 1,
                    _ => 0,
                };
                act::open_registry_editor(&mut t.app, kind, None);
                cx.notify();
            },
        )),
    );
    body = body.child(bar);

    let entries: Vec<domain::registry::Entry> = app
        .app
        .registry
        .entries
        .iter()
        .filter(|e| draft.filter == 0 || (e.kind as usize) + 1 == draft.filter)
        .cloned()
        .collect();
    let mut list = div().flex().flex_col().gap(px(3.0));
    if entries.is_empty() {
        list = list.child(widgets::label(
            "No entries yet. Add a plasmid, strain or primer.",
            theme::T_SM,
            theme::FAINT,
        ));
    }
    for e in entries {
        let id = e.id;
        let linked = e.linked_doc.clone().unwrap_or_default();
        list = list.child(
            div()
                .flex()
                .items_center()
                .gap(px(8.0))
                .p(px(6.0))
                .rounded(px(5.0))
                .bg(theme::SURFACE)
                .child(
                    div()
                        .w(px(60.0))
                        .text_size(px(theme::T_XS))
                        .text_color(theme::FAINT)
                        .child(e.kind.label()),
                )
                .child(
                    div()
                        .flex_grow()
                        .text_size(px(theme::T_SM))
                        .text_color(theme::TEXT)
                        .child(e.name().to_string()),
                )
                .child(
                    div()
                        .w(px(120.0))
                        .text_size(px(theme::T_XS))
                        .text_color(theme::MUTED)
                        .child(linked),
                )
                .child(
                    widgets::button(
                        SharedString::from(format!("rg-edit-{id}")),
                        "Edit",
                        Btn::Subtle,
                    )
                    .on_click(cx.listener(move |t, _, _, cx| {
                        let kind = t.app.registry.get(id).map(|e| e.kind as usize).unwrap_or(0);
                        act::open_registry_editor(&mut t.app, kind, Some(id));
                        cx.notify();
                    })),
                )
                .child(
                    widgets::button(
                        SharedString::from(format!("rg-link-{id}")),
                        "Link",
                        Btn::Subtle,
                    )
                    .on_click(cx.listener(move |t, _, _, cx| {
                        act::link_registry_entry(&mut t.app, id);
                        cx.notify();
                    })),
                )
                .child(
                    widgets::button(
                        SharedString::from(format!("rg-open-{id}")),
                        "Open",
                        Btn::Subtle,
                    )
                    .on_click(cx.listener(move |t, _, _, cx| {
                        act::open_linked_entry(&mut t.app, id);
                        cx.notify();
                    })),
                )
                .child(
                    widgets::button(
                        SharedString::from(format!("rg-del-{id}")),
                        "Delete",
                        Btn::Ghost,
                    )
                    .on_click(cx.listener(move |t, _, _, cx| {
                        act::delete_registry_entry(&mut t.app, id);
                        cx.notify();
                    })),
                ),
        );
    }
    body = body.child(list);
    let footer = footer_row(vec![widgets::button("rg-close", "Close", Btn::Ghost)
        .on_click(cx.listener(|t, _, _, cx| close_dialog(t, cx)))
        .into_any_element()]);
    modal_shell(
        app,
        cx,
        "Local Registry",
        760.0,
        max_h,
        true,
        body.into_any_element(),
        Some(footer.into_any_element()),
    )
}

// ---------------------------------------------------------------------------
// Registry entry editor
// ---------------------------------------------------------------------------

fn registry_editor(
    app: &mut HelixBench,
    cx: &mut Context<HelixBench>,
    max_h: Pixels,
) -> AnyElement {
    let draft = match &app.app.dialog {
        Some(Dialog::RegistryEditor(d)) => d.clone(),
        _ => {
            return modal_shell(
                app,
                cx,
                "Registry Entry",
                520.0,
                max_h,
                true,
                div().into_any_element(),
                None,
            )
        }
    };
    let kind = domain::registry::Kind::ALL[draft.kind.min(2)];
    let fields = kind.fields();
    // Pull-model text inputs per field.
    let mut inputs = Vec::new();
    for (i, f) in fields.iter().enumerate() {
        let seed = draft.values.get(i).cloned().unwrap_or_default();
        let inp = app.input_entity(cx, &format!("dlg:re:{i}"), &seed, f, false);
        inputs.push(inp);
    }
    let values: Vec<String> = inputs.iter().map(|inp| inp.read(cx).text()).collect();
    if let Some(Dialog::RegistryEditor(d)) = &mut app.app.dialog {
        d.values = values;
    }
    let mut body = div().flex().flex_col().size_full().gap(px(8.0));
    if draft.id.is_none() {
        let mut kinds = div().flex().gap(px(6.0));
        for (i, k) in domain::registry::Kind::ALL.iter().enumerate() {
            let sel = draft.kind == i;
            kinds = kinds.child(
                widgets::button(
                    SharedString::from(format!("re-kind-{i}")),
                    k.label(),
                    if sel { Btn::Primary } else { Btn::Outline },
                )
                .on_click(cx.listener(move |t, _, _, cx| {
                    if let Some(Dialog::RegistryEditor(d)) = &mut t.app.dialog {
                        d.kind = i;
                        d.values =
                            vec![String::new(); domain::registry::Kind::ALL[i].fields().len()];
                    }
                    cx.notify();
                })),
            );
        }
        body = body.child(kinds);
    }
    for (i, f) in fields.iter().enumerate() {
        body = body.child(
            div()
                .flex()
                .items_center()
                .gap(px(8.0))
                .child(div().w(px(140.0)).child(field_label(f)))
                .child(input_field(inputs[i].clone(), 300.0)),
        );
    }
    if let Some(e) = &draft.error {
        body = body.child(error_label(e));
    }
    let footer = footer_row(vec![
        widgets::button("re-cancel", "Cancel", Btn::Ghost)
            .on_click(cx.listener(|t, _, _, cx| {
                act::open_registry(&mut t.app);
                cx.notify();
            }))
            .into_any_element(),
        widgets::button("re-save", "Save", Btn::Primary)
            .on_click(cx.listener(|t, _, _, cx| {
                let d = match &t.app.dialog {
                    Some(Dialog::RegistryEditor(d)) => d.clone(),
                    _ => return,
                };
                match act::save_registry_entry(&mut t.app, &d) {
                    Ok(()) => {
                        act::open_registry(&mut t.app);
                        cx.notify();
                    }
                    Err(e) => {
                        if let Some(Dialog::RegistryEditor(dd)) = &mut t.app.dialog {
                            dd.error = Some(e);
                        }
                        cx.notify();
                    }
                }
            }))
            .into_any_element(),
    ]);
    modal_shell(
        app,
        cx,
        if draft.id.is_some() {
            "Edit Registry Entry"
        } else {
            "New Registry Entry"
        },
        560.0,
        max_h,
        true,
        body.into_any_element(),
        Some(footer.into_any_element()),
    )
}

// ---------------------------------------------------------------------------
// Protein analysis
// ---------------------------------------------------------------------------

fn protein(app: &mut HelixBench, cx: &mut Context<HelixBench>, max_h: Pixels) -> AnyElement {
    let draft = match &app.app.dialog {
        Some(Dialog::Protein(d)) => d.clone(),
        _ => {
            return modal_shell(
                app,
                cx,
                "Protein Analysis",
                560.0,
                max_h,
                true,
                div().into_any_element(),
                None,
            )
        }
    };
    let mut body = div().flex().flex_col().size_full().gap(px(10.0));
    body = body.child(widgets::label(
        "Translate the current sequence and compute physicochemical properties; align against the first open document.",
        theme::T_SM,
        theme::MUTED,
    ));
    body = body.child(
        widgets::button("pr-run", "Analyze", Btn::Outline).on_click(cx.listener(|t, _, _, cx| {
            let mut d = match &t.app.dialog {
                Some(Dialog::Protein(d)) => d.clone(),
                _ => return,
            };
            act::run_protein(&mut t.app, &mut d);
            t.app.dialog = Some(Dialog::Protein(d));
            cx.notify();
        })),
    );
    if let Some(s) = &draft.stats {
        let mut grid = div()
            .flex()
            .flex_col()
            .gap(px(4.0))
            .p(px(8.0))
            .rounded(px(6.0))
            .bg(theme::SURFACE);
        for (k, v) in [
            ("Length", format!("{} aa", s.len)),
            ("Molecular weight", format!("{:.1} Da", s.mw)),
            ("Isoelectric point (pI)", format!("{:.2}", s.pi)),
            ("GRAVY (hydropathy)", format!("{:.2}", s.gravy)),
            ("Charge at pH 7", format!("{:.2}", s.charge_ph7)),
            ("Extinction coeff", format!("{:.0} M-1cm-1", s.ext_coeff)),
        ] {
            grid = grid.child(
                div()
                    .flex()
                    .justify_between()
                    .gap(px(8.0))
                    .child(widgets::label(k, theme::T_SM, theme::MUTED))
                    .child(widgets::label(&v, theme::T_SM, theme::TEXT)),
            );
        }
        body = body.child(grid);
        let comp: String = s
            .composition
            .iter()
            .map(|(c, n)| format!("{c}:{n}"))
            .collect::<Vec<_>>()
            .join("  ");
        body = body.child(widgets::label(
            &format!("Composition  {comp}"),
            theme::T_XS,
            theme::FAINT,
        ));
    }
    if let Some((a, b, score)) = &draft.align {
        body = body.child(widgets::label(
            &format!("BLOSUM62 alignment vs first open doc (score {score})",),
            theme::T_SM,
            theme::FAINT,
        ));
        let mut al = div()
            .flex()
            .flex_col()
            .gap(px(2.0))
            .p(px(6.0))
            .rounded(px(5.0))
            .bg(theme::SURFACE);
        for row in [a, b] {
            al = al.child(
                div()
                    .text_size(px(theme::T_XS))
                    .font_family(theme::MONO_FONT)
                    .text_color(theme::TEXT)
                    .child(String::from_utf8_lossy(row).into_owned()),
            );
        }
        body = body.child(al);
    }
    let footer = footer_row(vec![widgets::button("pr-close", "Close", Btn::Ghost)
        .on_click(cx.listener(|t, _, _, cx| close_dialog(t, cx)))
        .into_any_element()]);
    modal_shell(
        app,
        cx,
        "Protein Analysis",
        620.0,
        max_h,
        true,
        body.into_any_element(),
        Some(footer.into_any_element()),
    )
}

// ---------------------------------------------------------------------------
// Feature library + bulk auto-annotation
// ---------------------------------------------------------------------------

fn feature_lib(app: &mut HelixBench, cx: &mut Context<HelixBench>, max_h: Pixels) -> AnyElement {
    let ni = app.input_entity(cx, "dlg:fl:name", "", "Name", false);
    let ki = app.input_entity(cx, "dlg:fl:kind", "", "kind", false);
    let mi = app.input_entity(cx, "dlg:fl:motif", "", "DNA motif", true);
    let (name, kind, motif) = (ni.read(cx).text(), ki.read(cx).text(), mi.read(cx).text());
    let mut body = div().flex().flex_col().size_full().gap(px(10.0));
    body = body.child(widgets::label(
        "Reusable motif library. Bulk auto-annotate scans the active document and all open documents on both strands.",
        theme::T_SM,
        theme::MUTED,
    ));
    body = body.child(
        div()
            .flex()
            .items_center()
            .gap(px(6.0))
            .child(input_field(ni, 140.0))
            .child(input_field(ki, 90.0))
            .child(input_field(mi, 160.0))
            .child(
                widgets::button("fl-add", "Add", Btn::Outline).on_click(cx.listener(
                    move |t, _, _, cx| {
                        act::add_lib_feature(&mut t.app, &name, &kind, &motif);
                        cx.notify();
                    },
                )),
            ),
    );
    let mut list = div().flex().flex_col().gap(px(3.0));
    for (i, lf) in app.app.feature_lib.iter().enumerate() {
        let motif_s = String::from_utf8_lossy(&lf.motif).into_owned();
        list = list.child(
            div()
                .flex()
                .items_center()
                .gap(px(8.0))
                .p(px(6.0))
                .rounded(px(5.0))
                .bg(theme::SURFACE)
                .child(
                    div()
                        .flex_grow()
                        .text_size(px(theme::T_SM))
                        .text_color(theme::TEXT)
                        .child(lf.name.clone()),
                )
                .child(
                    div()
                        .w(px(70.0))
                        .text_size(px(theme::T_XS))
                        .text_color(theme::FAINT)
                        .child(lf.kind.clone()),
                )
                .child(
                    div()
                        .flex_grow()
                        .text_size(px(theme::T_XS))
                        .font_family(theme::MONO_FONT)
                        .text_color(theme::MUTED)
                        .child(motif_s),
                )
                .child(
                    widgets::button(
                        SharedString::from(format!("fl-del-{i}")),
                        "Delete",
                        Btn::Ghost,
                    )
                    .on_click(cx.listener(move |t, _, _, cx| {
                        act::delete_lib_feature(&mut t.app, i);
                        cx.notify();
                    })),
                ),
        );
    }
    body = body.child(list);
    let footer = footer_row(vec![
        widgets::button("fl-close", "Close", Btn::Ghost)
            .on_click(cx.listener(|t, _, _, cx| close_dialog(t, cx)))
            .into_any_element(),
        widgets::button("fl-bulk", "Bulk auto-annotate", Btn::Primary)
            .on_click(cx.listener(|t, _, _, cx| {
                act::bulk_annotate(&mut t.app);
                cx.notify();
            }))
            .into_any_element(),
    ]);
    modal_shell(
        app,
        cx,
        "Feature Library",
        700.0,
        max_h,
        true,
        body.into_any_element(),
        Some(footer.into_any_element()),
    )
}

// ---------------------------------------------------------------------------
// Workspace / bulk operations + tags
// ---------------------------------------------------------------------------

fn workspace(app: &mut HelixBench, cx: &mut Context<HelixBench>, max_h: Pixels) -> AnyElement {
    let draft = match &app.app.dialog {
        Some(Dialog::Workspace(d)) => d.clone(),
        _ => {
            return modal_shell(
                app,
                cx,
                "Workspace",
                620.0,
                max_h,
                true,
                div().into_any_element(),
                None,
            )
        }
    };
    let ti = app.input_entity(cx, "dlg:ws:tag", &draft.tag, "tag", false);
    let tag = ti.read(cx).text();
    if let Some(Dialog::Workspace(d)) = &mut app.app.dialog {
        d.tag = tag.clone();
    }
    let mut body = div().flex().flex_col().size_full().gap(px(10.0));
    body = body.child(widgets::label(
        "Select documents for bulk tag / export / close. Index 1 is the active document.",
        theme::T_SM,
        theme::MUTED,
    ));
    let mut names = vec![app.app.doc.name.clone()];
    for od in &app.app.library {
        names.push(od.doc.name.clone());
    }
    let mut list = div().flex().flex_col().gap(px(3.0));
    for (i, n) in names.iter().enumerate() {
        let on = draft.selected.get(i).copied().unwrap_or(false);
        let tags = if i == 0 {
            app.app.doc.tags.clone()
        } else {
            app.app.library[i - 1].doc.tags.clone()
        };
        list = list.child(
            div()
                .flex()
                .items_center()
                .gap(px(8.0))
                .p(px(6.0))
                .rounded(px(5.0))
                .bg(theme::SURFACE)
                .child(
                    check_box(SharedString::from(format!("ws-sel-{i}")), on).on_click(cx.listener(
                        move |t, _, _, cx| {
                            if let Some(Dialog::Workspace(d)) = &mut t.app.dialog {
                                if let Some(b) = d.selected.get_mut(i) {
                                    *b = !*b;
                                }
                            }
                            cx.notify();
                        },
                    )),
                )
                .child(
                    div()
                        .flex_grow()
                        .text_size(px(theme::T_SM))
                        .text_color(theme::TEXT)
                        .child(n.clone()),
                )
                .child(
                    div()
                        .text_size(px(theme::T_XS))
                        .text_color(theme::FAINT)
                        .child(tags.join(", ")),
                ),
        );
    }
    body = body.child(list);
    body = body.child(
        div()
            .flex()
            .items_center()
            .gap(px(6.0))
            .child(field_label("Tag"))
            .child(input_field(ti, 140.0))
            .child(
                widgets::button("ws-tag", "Apply tag", Btn::Outline).on_click(cx.listener(
                    |t, _, _, cx| {
                        let d = match &t.app.dialog {
                            Some(Dialog::Workspace(d)) => d.clone(),
                            _ => return,
                        };
                        act::bulk_tag(&mut t.app, &d);
                        cx.notify();
                    },
                )),
            ),
    );
    let footer = footer_row(vec![
        widgets::button("ws-close", "Close", Btn::Ghost)
            .on_click(cx.listener(|t, _, _, cx| close_dialog(t, cx)))
            .into_any_element(),
        widgets::button("ws-export", "Export FASTA", Btn::Outline)
            .on_click(cx.listener(|t, _, _, cx| {
                let d = match &t.app.dialog {
                    Some(Dialog::Workspace(d)) => d.clone(),
                    _ => return,
                };
                act::bulk_export(&mut t.app, &d);
                cx.notify();
            }))
            .into_any_element(),
        widgets::button("ws-delete", "Close selected", Btn::Ghost)
            .on_click(cx.listener(|t, _, _, cx| {
                let d = match &t.app.dialog {
                    Some(Dialog::Workspace(d)) => d.clone(),
                    _ => return,
                };
                act::bulk_delete(&mut t.app, &d);
                act::open_workspace(&mut t.app);
                cx.notify();
            }))
            .into_any_element(),
    ]);
    modal_shell(
        app,
        cx,
        "Workspace / Bulk",
        680.0,
        max_h,
        true,
        body.into_any_element(),
        Some(footer.into_any_element()),
    )
}

// ---------------------------------------------------------------------------
// Phase 1: Notebook
// ---------------------------------------------------------------------------

fn notebook(app: &mut HelixBench, cx: &mut Context<HelixBench>, max_h: Pixels) -> AnyElement {
    let draft = match &app.app.dialog {
        Some(Dialog::Notebook(d)) => d.clone(),
        _ => {
            return modal_shell(
                app,
                cx,
                "Notebook",
                700.0,
                max_h,
                true,
                div().into_any_element(),
                None,
            )
        }
    };
    let mut body = div().flex().flex_col().size_full().gap(px(10.0));
    let mut bar = div().flex().items_center().gap(px(6.0));
    for (i, label) in ["All", "Draft", "In review", "Approved", "Archived"]
        .iter()
        .enumerate()
    {
        let sel = draft.filter == i;
        bar = bar.child(
            widgets::button(
                SharedString::from(format!("nb-f-{i}")),
                label,
                if sel { Btn::Primary } else { Btn::Outline },
            )
            .on_click(cx.listener(move |t, _, _, cx| {
                if let Some(Dialog::Notebook(d)) = &mut t.app.dialog {
                    d.filter = i;
                }
                cx.notify();
            })),
        );
    }
    bar = bar.child(div().flex_grow());
    bar = bar.child(
        widgets::button("nb-new", "New entry", Btn::Primary).on_click(cx.listener(
            |t, _, _, cx| {
                act::open_entry_editor(&mut t.app, None, 0);
                cx.notify();
            },
        )),
    );
    body = body.child(bar);
    let mut list = div().flex().flex_col().gap(px(3.0));
    let entries: Vec<domain::notebook::Entry> = app
        .app
        .records
        .entries
        .iter()
        .filter(|e| match draft.filter {
            1 => e.status == domain::notebook::EntryStatus::Draft,
            2 => e.status == domain::notebook::EntryStatus::InReview,
            3 => e.status == domain::notebook::EntryStatus::Approved,
            4 => e.status == domain::notebook::EntryStatus::Archived,
            _ => true,
        })
        .cloned()
        .collect();
    if entries.is_empty() {
        list = list.child(widgets::label(
            "No entries. Create one from a template.",
            theme::T_SM,
            theme::FAINT,
        ));
    }
    for e in entries {
        let id = e.id;
        list = list.child(
            div()
                .flex()
                .items_center()
                .gap(px(8.0))
                .p(px(6.0))
                .rounded(px(5.0))
                .bg(theme::SURFACE)
                .child(
                    div()
                        .flex_grow()
                        .text_size(px(theme::T_SM))
                        .text_color(theme::TEXT)
                        .child(e.title.clone()),
                )
                .child(widgets::label(e.status.label(), theme::T_XS, theme::MUTED))
                .child(
                    widgets::button(
                        SharedString::from(format!("nb-open-{id}")),
                        "Open",
                        Btn::Subtle,
                    )
                    .on_click(cx.listener(move |t, _, _, cx| {
                        act::open_entry_editor(&mut t.app, Some(id), 0);
                        cx.notify();
                    })),
                )
                .child(
                    widgets::button(
                        SharedString::from(format!("nb-sub-{id}")),
                        "Submit",
                        Btn::Subtle,
                    )
                    .on_click(cx.listener(move |t, _, _, cx| {
                        act::entry_action(&mut t.app, id, "submit");
                        cx.notify();
                    })),
                )
                .child(
                    widgets::button(
                        SharedString::from(format!("nb-app-{id}")),
                        "Approve",
                        Btn::Subtle,
                    )
                    .on_click(cx.listener(move |t, _, _, cx| {
                        act::entry_action(&mut t.app, id, "approve");
                        cx.notify();
                    })),
                ),
        );
    }
    body = body.child(list);
    let footer = footer_row(vec![widgets::button("nb-close", "Close", Btn::Ghost)
        .on_click(cx.listener(|t, _, _, cx| close_dialog(t, cx)))
        .into_any_element()]);
    modal_shell(
        app,
        cx,
        "Notebook",
        760.0,
        max_h,
        true,
        body.into_any_element(),
        Some(footer.into_any_element()),
    )
}

fn entry_editor(app: &mut HelixBench, cx: &mut Context<HelixBench>, max_h: Pixels) -> AnyElement {
    let draft = match &app.app.dialog {
        Some(Dialog::EntryEditor(d)) => d.clone(),
        _ => {
            return modal_shell(
                app,
                cx,
                "Entry",
                640.0,
                max_h,
                true,
                div().into_any_element(),
                None,
            )
        }
    };
    let ti = app.input_entity(cx, "dlg:ee:title", &draft.title, "Title", false);
    let title = ti.read(cx).text();
    if let Some(Dialog::EntryEditor(d)) = &mut app.app.dialog {
        d.title = title;
    }
    let mut body = div().flex().flex_col().size_full().gap(px(8.0));
    body = body.child(
        div()
            .flex()
            .items_center()
            .gap(px(8.0))
            .child(field_label("Title"))
            .child(input_field(ti, 320.0)),
    );
    // sections
    for (i, (st, sb)) in draft.sections.iter().enumerate() {
        let si = app.input_entity(cx, &format!("dlg:ee:s{i}"), sb, st, false);
        let val = si.read(cx).text();
        if let Some(Dialog::EntryEditor(d)) = &mut app.app.dialog {
            if let Some(s) = d.sections.get_mut(i) {
                s.1 = val;
            }
        }
        body = body.child(
            div()
                .flex()
                .flex_col()
                .gap(px(2.0))
                .child(field_label(st))
                .child(input_field(si, 560.0)),
        );
    }
    // tables
    for (ti2, t) in draft.tables.iter().enumerate() {
        body = body.child(widgets::label(
            &format!(
                "Table: {} ({} rows){}",
                t.name,
                t.rows.len(),
                if t.submitted { " · submitted" } else { "" }
            ),
            theme::T_SM,
            theme::FAINT,
        ));
        let mut grid = div()
            .flex()
            .flex_col()
            .gap(px(2.0))
            .p(px(6.0))
            .rounded(px(5.0))
            .bg(theme::SURFACE);
        grid = grid.child(
            div().flex().gap(px(8.0)).children(
                t.columns
                    .iter()
                    .map(|c| widgets::label(c, theme::T_XS, theme::MUTED)),
            ),
        );
        for row in t.rows.iter().take(6) {
            grid = grid.child(
                div().flex().gap(px(8.0)).children(
                    row.iter()
                        .map(|c| widgets::label(c, theme::T_XS, theme::TEXT)),
                ),
            );
        }
        body = body.child(grid);
        let eid = draft.id;
        body = body.child(
            widgets::button(
                SharedString::from(format!("ee-submit-{ti2}")),
                "Submit table (write back)",
                Btn::Outline,
            )
            .on_click(cx.listener(move |t, _, _, cx| {
                if let Some(id) = eid {
                    act::submit_entry_table(&mut t.app, id, ti2);
                }
                cx.notify();
            })),
        );
    }
    if let Some(e) = &draft.error {
        body = body.child(error_label(e));
    }
    let footer = footer_row(vec![
        widgets::button("ee-cancel", "Back", Btn::Ghost)
            .on_click(cx.listener(|t, _, _, cx| {
                act::open_notebook(&mut t.app);
                cx.notify();
            }))
            .into_any_element(),
        widgets::button("ee-save", "Save entry", Btn::Primary)
            .on_click(cx.listener(|t, _, _, cx| {
                let d = match &t.app.dialog {
                    Some(Dialog::EntryEditor(d)) => d.clone(),
                    _ => return,
                };
                match act::save_entry(&mut t.app, &d) {
                    Ok(_) => {
                        act::open_notebook(&mut t.app);
                        cx.notify();
                    }
                    Err(e) => {
                        if let Some(Dialog::EntryEditor(dd)) = &mut t.app.dialog {
                            dd.error = Some(e);
                        }
                        cx.notify();
                    }
                }
            }))
            .into_any_element(),
    ]);
    modal_shell(
        app,
        cx,
        if draft.id.is_some() {
            "Edit Entry"
        } else {
            "New Entry"
        },
        680.0,
        max_h,
        true,
        body.into_any_element(),
        Some(footer.into_any_element()),
    )
}

fn projects(app: &mut HelixBench, cx: &mut Context<HelixBench>, max_h: Pixels) -> AnyElement {
    let draft = match &app.app.dialog {
        Some(Dialog::Projects(d)) => d.clone(),
        _ => {
            return modal_shell(
                app,
                cx,
                "Projects",
                560.0,
                max_h,
                true,
                div().into_any_element(),
                None,
            )
        }
    };
    let ni = app.input_entity(
        cx,
        "dlg:pj:name",
        &draft.name,
        "Project / folder name",
        false,
    );
    let name = ni.read(cx).text();
    if let Some(Dialog::Projects(d)) = &mut app.app.dialog {
        d.name = name;
    }
    let mut body = div().flex().flex_col().size_full().gap(px(8.0));
    body = body.child(
        div()
            .flex()
            .items_center()
            .gap(px(6.0))
            .child(input_field(ni, 240.0))
            .child(
                widgets::button("pj-add", "Add", Btn::Outline).on_click(cx.listener(
                    |t, _, _, cx| {
                        let d = match &t.app.dialog {
                            Some(Dialog::Projects(d)) => d.clone(),
                            _ => return,
                        };
                        act::add_project(&mut t.app, &d);
                        cx.notify();
                    },
                )),
            ),
    );
    let mut list = div().flex().flex_col().gap(px(2.0));
    fn walk(
        ws: &domain::project::Workspace,
        parent: Option<u64>,
        depth: usize,
        out: &mut Vec<(u64, String, usize)>,
    ) {
        for p in ws.children(parent) {
            out.push((p.id, p.name.clone(), depth));
            walk(ws, Some(p.id), depth + 1, out);
        }
    }
    let mut rows = Vec::new();
    walk(&app.app.records.workspace, None, 0, &mut rows);
    for (id, name, depth) in rows {
        list = list.child(
            div()
                .flex()
                .items_center()
                .gap(px(6.0))
                .p(px(4.0))
                .child(div().w(px((depth * 16) as f32)))
                .child(
                    div()
                        .flex_grow()
                        .text_size(px(theme::T_SM))
                        .text_color(theme::TEXT)
                        .child(name),
                )
                .child(
                    widgets::button(
                        SharedString::from(format!("pj-del-{id}")),
                        "Delete",
                        Btn::Ghost,
                    )
                    .on_click(cx.listener(move |t, _, _, cx| {
                        act::delete_project(&mut t.app, id);
                        cx.notify();
                    })),
                ),
        );
    }
    body = body.child(list);
    let footer = footer_row(vec![widgets::button("pj-close", "Close", Btn::Ghost)
        .on_click(cx.listener(|t, _, _, cx| close_dialog(t, cx)))
        .into_any_element()]);
    modal_shell(
        app,
        cx,
        "Projects & Folders",
        600.0,
        max_h,
        true,
        body.into_any_element(),
        Some(footer.into_any_element()),
    )
}

fn audit(app: &mut HelixBench, cx: &mut Context<HelixBench>, max_h: Pixels) -> AnyElement {
    let draft = match &app.app.dialog {
        Some(Dialog::Audit(d)) => d.clone(),
        _ => {
            return modal_shell(
                app,
                cx,
                "Audit Log",
                640.0,
                max_h,
                true,
                div().into_any_element(),
                None,
            )
        }
    };
    let fi = app.input_entity(
        cx,
        "dlg:au:filter",
        &draft.filter,
        "filter by object",
        false,
    );
    let filter = fi.read(cx).text();
    if let Some(Dialog::Audit(d)) = &mut app.app.dialog {
        d.filter = filter.clone();
    }
    let mut body = div().flex().flex_col().size_full().gap(px(8.0));
    body = body.child(
        div()
            .flex()
            .items_center()
            .gap(px(6.0))
            .child(input_field(fi, 240.0))
            .child(
                widgets::button("au-export", "Export CSV", Btn::Outline).on_click(cx.listener(
                    |t, _, _, cx| {
                        act::export_audit(&mut t.app);
                        cx.notify();
                    },
                )),
            ),
    );
    let mut list = div().flex().flex_col().gap(px(2.0));
    for e in app
        .app
        .records
        .audit
        .events
        .iter()
        .rev()
        .filter(|e| filter.is_empty() || e.object.contains(&filter))
        .take(60)
    {
        list = list.child(
            div()
                .flex()
                .items_center()
                .gap(px(8.0))
                .p(px(4.0))
                .rounded(px(4.0))
                .bg(theme::SURFACE)
                .child(
                    div()
                        .w(px(70.0))
                        .text_size(px(theme::T_XS))
                        .font_family(theme::MONO_FONT)
                        .text_color(theme::FAINT)
                        .child(e.ts.to_string()),
                )
                .child(
                    div()
                        .w(px(110.0))
                        .text_size(px(theme::T_XS))
                        .text_color(theme::MUTED)
                        .child(e.object.clone()),
                )
                .child(
                    div()
                        .flex_grow()
                        .text_size(px(theme::T_XS))
                        .text_color(theme::TEXT)
                        .child(e.action.clone()),
                )
                .child(
                    div()
                        .text_size(px(theme::T_XS))
                        .text_color(theme::FAINT)
                        .child(e.detail.clone()),
                ),
        );
    }
    body = body.child(list);
    let footer = footer_row(vec![widgets::button("au-close", "Close", Btn::Ghost)
        .on_click(cx.listener(|t, _, _, cx| close_dialog(t, cx)))
        .into_any_element()]);
    modal_shell(
        app,
        cx,
        "Audit Log",
        700.0,
        max_h,
        true,
        body.into_any_element(),
        Some(footer.into_any_element()),
    )
}

fn home(app: &mut HelixBench, cx: &mut Context<HelixBench>, max_h: Pixels) -> AnyElement {
    let mut body = div().flex().flex_col().size_full().gap(px(10.0));
    body = body.child(widgets::label("My Work", theme::T_MD, theme::TEXT));
    body = body.child(widgets::label(
        &format!(
            "Open documents: {} · Registry entries: {} · Notebook entries: {}",
            app.app.library.len() + 1,
            app.app.registry.entries.len(),
            app.app.records.entries.len()
        ),
        theme::T_SM,
        theme::MUTED,
    ));
    let mut recent = div().flex().flex_col().gap(px(2.0));
    recent = recent.child(widgets::label("Recent entries", theme::T_SM, theme::FAINT));
    for e in app.app.records.entries.iter().rev().take(6) {
        recent = recent.child(
            div()
                .flex()
                .items_center()
                .gap(px(8.0))
                .p(px(4.0))
                .rounded(px(4.0))
                .bg(theme::SURFACE)
                .child(
                    div()
                        .flex_grow()
                        .text_size(px(theme::T_SM))
                        .text_color(theme::TEXT)
                        .child(e.title.clone()),
                )
                .child(widgets::label(e.status.label(), theme::T_XS, theme::MUTED)),
        );
    }
    body = body.child(recent);
    let mut quick = div().flex().gap(px(6.0));
    for (id, label, f) in [
        ("hm-nb", "Notebook", 1),
        ("hm-pj", "Projects", 2),
        ("hm-au", "Audit", 5),
        ("hm-em", "Entity Map", 3),
    ] {
        quick = quick.child(
            widgets::button(id, label, Btn::Outline).on_click(cx.listener(move |t, _, _, cx| {
                match f {
                    1 => act::open_notebook(&mut t.app),
                    2 => act::open_projects(&mut t.app),
                    3 => act::open_entity_map(&mut t.app),
                    _ => act::open_audit(&mut t.app),
                }
                cx.notify();
            })),
        );
    }
    body = body.child(quick);
    let footer = footer_row(vec![widgets::button("hm-close", "Close", Btn::Ghost)
        .on_click(cx.listener(|t, _, _, cx| close_dialog(t, cx)))
        .into_any_element()]);
    modal_shell(
        app,
        cx,
        "Home / My Work",
        620.0,
        max_h,
        true,
        body.into_any_element(),
        Some(footer.into_any_element()),
    )
}

fn entity_map(app: &mut HelixBench, cx: &mut Context<HelixBench>, max_h: Pixels) -> AnyElement {
    let mut body = div().flex().flex_col().size_full().gap(px(8.0));
    body = body.child(widgets::label(
        "Entity Map — registry relationships (select two to link).",
        theme::T_SM,
        theme::MUTED,
    ));
    let mut list = div().flex().flex_col().gap(px(2.0));
    for e in app.app.registry.entries.iter().filter(|e| !e.archived) {
        let rel: Vec<String> = e
            .relations
            .iter()
            .filter_map(|r| app.app.registry.get(*r).map(|x| x.name().to_string()))
            .collect();
        list = list.child(
            div()
                .flex()
                .items_center()
                .gap(px(8.0))
                .p(px(4.0))
                .rounded(px(4.0))
                .bg(theme::SURFACE)
                .child(
                    div()
                        .flex_grow()
                        .text_size(px(theme::T_SM))
                        .text_color(theme::TEXT)
                        .child(e.name().to_string()),
                )
                .child(
                    div()
                        .text_size(px(theme::T_XS))
                        .text_color(theme::MUTED)
                        .child(if rel.is_empty() {
                            "—".into()
                        } else {
                            rel.join(", ")
                        }),
                ),
        );
    }
    body = body.child(list);
    body = body.child(widgets::label(
        "Link first two non-archived entries (demo):",
        theme::T_XS,
        theme::FAINT,
    ));
    body = body.child(
        widgets::button("em-link", "Link entry 1 ↔ 2", Btn::Outline).on_click(cx.listener(
            |t, _, _, cx| {
                let ids: Vec<u64> = t
                    .app
                    .registry
                    .entries
                    .iter()
                    .filter(|e| !e.archived)
                    .take(2)
                    .map(|e| e.id)
                    .collect();
                if ids.len() == 2 {
                    act::link_registry_entries(&mut t.app, ids[0], ids[1]);
                }
                cx.notify();
            },
        )),
    );
    let footer = footer_row(vec![widgets::button("em-close", "Close", Btn::Ghost)
        .on_click(cx.listener(|t, _, _, cx| close_dialog(t, cx)))
        .into_any_element()]);
    modal_shell(
        app,
        cx,
        "Entity Map",
        620.0,
        max_h,
        true,
        body.into_any_element(),
        Some(footer.into_any_element()),
    )
}

fn registry_import(
    app: &mut HelixBench,
    cx: &mut Context<HelixBench>,
    max_h: Pixels,
) -> AnyElement {
    let draft = match &app.app.dialog {
        Some(Dialog::RegistryImport(d)) => d.clone(),
        _ => {
            return modal_shell(
                app,
                cx,
                "Registry Import",
                620.0,
                max_h,
                true,
                div().into_any_element(),
                None,
            )
        }
    };
    let mut body = div().flex().flex_col().size_full().gap(px(8.0));
    let mut kinds = div().flex().gap(px(6.0));
    for (i, k) in domain::registry::Kind::ALL.iter().enumerate() {
        let sel = draft.kind == i;
        kinds = kinds.child(
            widgets::button(
                SharedString::from(format!("ri-k-{i}")),
                k.label(),
                if sel { Btn::Primary } else { Btn::Outline },
            )
            .on_click(cx.listener(move |t, _, _, cx| {
                if let Some(Dialog::RegistryImport(d)) = &mut t.app.dialog {
                    d.kind = i;
                }
                cx.notify();
            })),
        );
    }
    body = body.child(kinds);
    let data_rows = draft
        .csv
        .lines()
        .filter(|l| !l.trim().is_empty())
        .count()
        .saturating_sub(1);
    body = body.child(
        div()
            .flex()
            .items_center()
            .gap(px(8.0))
            .child(
                widgets::button("ri-load", "Load CSV…", Btn::Outline).on_click(cx.listener(
                    |t, _, _, cx| {
                        act::registry_import_load_csv(&mut t.app);
                        cx.notify();
                    },
                )),
            )
            .child(widgets::label(
                &format!("{data_rows} data row(s)"),
                theme::T_XS,
                theme::FAINT,
            )),
    );
    let mut prev = div()
        .flex()
        .flex_col()
        .gap(px(2.0))
        .p(px(6.0))
        .rounded(px(5.0))
        .bg(theme::SURFACE)
        .max_h(px(90.0))
        .overflow_hidden();
    for l in draft.csv.lines().filter(|l| !l.trim().is_empty()).take(4) {
        prev = prev.child(
            div()
                .font_family(theme::MONO_FONT)
                .text_size(px(theme::T_XS))
                .text_color(theme::MUTED)
                .child(l.to_owned()),
        );
    }
    body = body.child(prev);
    let cols = act::csv_columns(&draft.csv);
    if !cols.is_empty() {
        let mut map = div().flex().flex_wrap().items_center().gap(px(6.0));
        map = map.child(widgets::label("Name column:", theme::T_XS, theme::FAINT));
        map = map.child(
            widgets::button(
                "ri-auto",
                "Auto",
                if draft.name_col.is_none() {
                    Btn::Primary
                } else {
                    Btn::Outline
                },
            )
            .on_click(cx.listener(|t, _, _, cx| {
                if let Some(Dialog::RegistryImport(d)) = &mut t.app.dialog {
                    d.name_col = None;
                }
                cx.notify();
            })),
        );
        for (i, c) in cols.iter().enumerate().take(12) {
            let sel = draft.name_col == Some(i);
            let label = c.clone();
            map = map.child(
                widgets::button(
                    SharedString::from(format!("ri-col-{i}")),
                    &label,
                    if sel { Btn::Primary } else { Btn::Outline },
                )
                .on_click(cx.listener(move |t, _, _, cx| {
                    if let Some(Dialog::RegistryImport(d)) = &mut t.app.dialog {
                        d.name_col = Some(i);
                    }
                    cx.notify();
                })),
            );
        }
        body = body.child(map);
    }
    if let Some(e) = &draft.error {
        body = body.child(error_label(e));
    }
    if draft.imported > 0 {
        body = body.child(widgets::label(
            &format!("Imported {} entries", draft.imported),
            theme::T_SM,
            theme::OK,
        ));
    }
    if !draft.row_errors.is_empty() {
        body = body.child(widgets::label(
            &format!("{} row issue(s):", draft.row_errors.len()),
            theme::T_SM,
            theme::WARN,
        ));
        let mut list = div()
            .flex()
            .flex_col()
            .gap(px(2.0))
            .max_h(px(120.0))
            .overflow_hidden();
        for e in draft.row_errors.iter().take(20) {
            list = list.child(
                div()
                    .font_family(theme::MONO_FONT)
                    .text_size(px(theme::T_XS))
                    .text_color(theme::WARN)
                    .child(e.clone()),
            );
        }
        body = body.child(list);
    }
    let footer = footer_row(vec![
        widgets::button("ri-close", "Close", Btn::Ghost)
            .on_click(cx.listener(|t, _, _, cx| close_dialog(t, cx)))
            .into_any_element(),
        widgets::button("ri-run", "Import", Btn::Primary)
            .on_click(cx.listener(|t, _, _, cx| {
                let mut d = match &t.app.dialog {
                    Some(Dialog::RegistryImport(d)) => d.clone(),
                    _ => return,
                };
                act::run_registry_import(&mut t.app, &mut d);
                t.app.dialog = Some(Dialog::RegistryImport(d));
                cx.notify();
            }))
            .into_any_element(),
    ]);
    modal_shell(
        app,
        cx,
        "Registry Import (CSV)",
        660.0,
        max_h,
        true,
        body.into_any_element(),
        Some(footer.into_any_element()),
    )
}

// ---------------------------------------------------------------------------
// Phase 2: Inventory
// ---------------------------------------------------------------------------

fn inventory(app: &mut HelixBench, cx: &mut Context<HelixBench>, max_h: Pixels) -> AnyElement {
    let draft = match &app.app.dialog {
        Some(Dialog::Inventory(d)) => d.clone(),
        _ => {
            return modal_shell(
                app,
                cx,
                "Inventory",
                700.0,
                max_h,
                true,
                div().into_any_element(),
                None,
            )
        }
    };
    let ni = app.input_entity(
        cx,
        "dlg:inv:name",
        &draft.name,
        "Location / box / plate name",
        false,
    );
    let li = app.input_entity(
        cx,
        "dlg:inv:lookup",
        &draft.lookup,
        "find entity containers",
        false,
    );
    let (name, lookup) = (ni.read(cx).text(), li.read(cx).text());
    if let Some(Dialog::Inventory(d)) = &mut app.app.dialog {
        d.name = name;
        d.lookup = lookup.clone();
    }
    let mut body = div().flex().flex_col().size_full().gap(px(10.0));
    let mut kinds = div().flex().gap(px(6.0));
    for (i, label) in ["Location", "Box", "Plate", "Tube"].iter().enumerate() {
        let sel = draft.kind == i;
        kinds = kinds.child(
            widgets::button(
                SharedString::from(format!("inv-k-{i}")),
                label,
                if sel { Btn::Primary } else { Btn::Outline },
            )
            .on_click(cx.listener(move |t, _, _, cx| {
                if let Some(Dialog::Inventory(d)) = &mut t.app.dialog {
                    d.kind = i;
                }
                cx.notify();
            })),
        );
    }
    body = body.child(
        div()
            .flex()
            .items_center()
            .gap(px(6.0))
            .child(input_field(ni, 220.0))
            .child(kinds)
            .child(
                widgets::button("inv-add", "Add", Btn::Outline).on_click(cx.listener(
                    |t, _, _, cx| {
                        let d = match &t.app.dialog {
                            Some(Dialog::Inventory(d)) => d.clone(),
                            _ => return,
                        };
                        act::inv_add(&mut t.app, &d);
                        cx.notify();
                    },
                )),
            ),
    );
    let mut tree = div().flex().flex_col().gap(px(2.0));
    fn walk(
        inv: &domain::inventory::Inventory,
        parent: Option<u64>,
        depth: usize,
        out: &mut Vec<(u64, String, String, usize)>,
    ) {
        for c in inv.children(parent) {
            out.push((c.id, c.name.clone(), c.kind.label().to_string(), depth));
            walk(inv, Some(c.id), depth + 1, out);
        }
    }
    let mut rows = Vec::new();
    walk(&app.app.inventory, None, 0, &mut rows);
    if rows.is_empty() {
        tree = tree.child(widgets::label(
            "No storage yet. Add a location, then boxes/plates.",
            theme::T_SM,
            theme::FAINT,
        ));
    }
    for (id, name, kind, depth) in rows {
        tree = tree.child(
            div()
                .flex()
                .items_center()
                .gap(px(6.0))
                .p(px(4.0))
                .rounded(px(4.0))
                .bg(theme::SURFACE)
                .child(div().w(px((depth * 14) as f32)))
                .child(
                    div()
                        .flex_grow()
                        .text_size(px(theme::T_SM))
                        .text_color(theme::TEXT)
                        .child(name),
                )
                .child(widgets::label(&kind, theme::T_XS, theme::MUTED))
                .child(
                    widgets::button(
                        SharedString::from(format!("inv-sel-{id}")),
                        "Select",
                        Btn::Subtle,
                    )
                    .on_click(cx.listener(move |t, _, _, cx| {
                        if let Some(Dialog::Inventory(d)) = &mut t.app.dialog {
                            d.selected_plate = Some(id);
                            d.parent = Some(id);
                        }
                        cx.notify();
                    })),
                ),
        );
    }
    body = body.child(tree);
    if let Some(plate) = draft.selected_plate {
        let wells: Vec<domain::inventory::Container> = app
            .app
            .inventory
            .wells(plate)
            .iter()
            .map(|c| (*c).clone())
            .collect();
        body = body.child(widgets::label(
            &format!("Plate wells ({}):", wells.len()),
            theme::T_SM,
            theme::FAINT,
        ));
        let mut grid = div()
            .flex()
            .flex_col()
            .gap(px(2.0))
            .p(px(6.0))
            .rounded(px(5.0))
            .bg(theme::SURFACE);
        for w in wells.iter().take(12) {
            grid = grid.child(
                div()
                    .flex()
                    .gap(px(8.0))
                    .child(
                        div()
                            .w(px(40.0))
                            .text_size(px(theme::T_XS))
                            .font_family(theme::MONO_FONT)
                            .text_color(theme::TEXT)
                            .child(w.position.clone().unwrap_or_default()),
                    )
                    .child(
                        div()
                            .flex_grow()
                            .text_size(px(theme::T_XS))
                            .text_color(theme::MUTED)
                            .child(w.entity.clone().unwrap_or_else(|| "—".into())),
                    )
                    .child(widgets::label(
                        w.role.map(|r| r.label()).unwrap_or(""),
                        theme::T_XS,
                        theme::FAINT,
                    )),
            );
        }
        body = body.child(grid);
        body = body.child(
            div()
                .flex()
                .gap(px(6.0))
                .child(
                    widgets::button("inv-fill", "Fill demo wells", Btn::Outline).on_click(
                        cx.listener(move |t, _, _, cx| {
                            act::inv_fill_demo_wells(&mut t.app, plate);
                            cx.notify();
                        }),
                    ),
                )
                .child(
                    widgets::button("inv-snap", "Snapshot to notebook", Btn::Outline).on_click(
                        cx.listener(move |t, _, _, cx| {
                            act::inv_snapshot_to_entry(&mut t.app, plate);
                            cx.notify();
                        }),
                    ),
                ),
        );
    }
    if !lookup.trim().is_empty() {
        let hits: Vec<domain::inventory::Container> = app
            .app
            .inventory
            .find_by_entity(lookup.trim())
            .iter()
            .map(|c| (*c).clone())
            .collect();
        body = body.child(widgets::label(
            &format!("Containers holding \"{}\": {}", lookup, hits.len()),
            theme::T_SM,
            theme::FAINT,
        ));
        for h in hits {
            body = body.child(widgets::label(
                &format!(
                    "{} [{}] qty {} {}",
                    h.name,
                    h.position.clone().unwrap_or_default(),
                    h.quantity,
                    h.unit
                ),
                theme::T_XS,
                theme::MUTED,
            ));
        }
    }
    body = body.child(
        div()
            .flex()
            .items_center()
            .gap(px(6.0))
            .child(field_label("Find entity"))
            .child(input_field(li, 220.0)),
    );
    let footer = footer_row(vec![widgets::button("inv-close", "Close", Btn::Ghost)
        .on_click(cx.listener(|t, _, _, cx| close_dialog(t, cx)))
        .into_any_element()]);
    modal_shell(
        app,
        cx,
        "Inventory",
        760.0,
        max_h,
        true,
        body.into_any_element(),
        Some(footer.into_any_element()),
    )
}

// ---------------------------------------------------------------------------
// Phase 3: Workflows
// ---------------------------------------------------------------------------

fn workflows(app: &mut HelixBench, cx: &mut Context<HelixBench>, max_h: Pixels) -> AnyElement {
    let draft = match &app.app.dialog {
        Some(Dialog::Workflows(d)) => d.clone(),
        _ => {
            return modal_shell(
                app,
                cx,
                "Workflows",
                700.0,
                max_h,
                true,
                div().into_any_element(),
                None,
            )
        }
    };
    let ti = app.input_entity(cx, "dlg:wf:title", &draft.title, "Task title", false);
    let si = app.input_entity(cx, "dlg:wf:schema", &draft.schema, "schema", false);
    let ri = app.input_entity(cx, "dlg:wf:req", &draft.request_name, "Request name", false);
    let oki = app.input_entity(cx, "dlg:wf:okey", &draft.output_key, "output key", false);
    let ovi = app.input_entity(cx, "dlg:wf:oval", &draft.output_val, "output value", false);
    let (title, schema, req, okey, oval) = (
        ti.read(cx).text(),
        si.read(cx).text(),
        ri.read(cx).text(),
        oki.read(cx).text(),
        ovi.read(cx).text(),
    );
    if let Some(Dialog::Workflows(d)) = &mut app.app.dialog {
        d.title = title;
        d.schema = schema;
        d.request_name = req;
        d.output_key = okey;
        d.output_val = oval;
    }
    let mut body = div().flex().flex_col().size_full().gap(px(10.0));
    body = body.child(
        div()
            .flex()
            .items_center()
            .gap(px(6.0))
            .child(input_field(ti, 180.0))
            .child(input_field(si, 90.0))
            .child(
                widgets::button("wf-add", "Add task", Btn::Outline).on_click(cx.listener(
                    |t, _, _, cx| {
                        let d = match &t.app.dialog {
                            Some(Dialog::Workflows(d)) => d.clone(),
                            _ => return,
                        };
                        act::wf_add_task(&mut t.app, &d);
                        cx.notify();
                    },
                )),
            ),
    );
    let mut list = div().flex().flex_col().gap(px(2.0));
    let tasks: Vec<domain::workflow::Task> = app.app.workflows.tasks.to_vec();
    if tasks.is_empty() {
        list = list.child(widgets::label("No tasks yet.", theme::T_SM, theme::FAINT));
    }
    for t in tasks {
        let id = t.id;
        let sel = draft.selected_task == Some(id);
        list = list.child(
            div()
                .flex()
                .items_center()
                .gap(px(6.0))
                .p(px(5.0))
                .rounded(px(5.0))
                .bg(if sel {
                    theme::SURFACE_2
                } else {
                    theme::SURFACE
                })
                .child(
                    div()
                        .flex_grow()
                        .text_size(px(theme::T_SM))
                        .text_color(theme::TEXT)
                        .child(t.title.clone()),
                )
                .child(widgets::label(t.status.label(), theme::T_XS, theme::MUTED))
                .child(
                    widgets::button(
                        SharedString::from(format!("wf-sel-{id}")),
                        "Select",
                        Btn::Subtle,
                    )
                    .on_click(cx.listener(move |t, _, _, cx| {
                        if let Some(Dialog::Workflows(d)) = &mut t.app.dialog {
                            d.selected_task = Some(id);
                        }
                        cx.notify();
                    })),
                )
                .child(
                    widgets::button(
                        SharedString::from(format!("wf-start-{id}")),
                        "Start",
                        Btn::Subtle,
                    )
                    .on_click(cx.listener(move |t, _, _, cx| {
                        act::wf_set_status(
                            &mut t.app,
                            id,
                            domain::workflow::TaskStatus::InProgress,
                        );
                        cx.notify();
                    })),
                )
                .child(
                    widgets::button(
                        SharedString::from(format!("wf-fail-{id}")),
                        "Fail",
                        Btn::Ghost,
                    )
                    .on_click(cx.listener(move |t, _, _, cx| {
                        act::wf_set_status(&mut t.app, id, domain::workflow::TaskStatus::Failed);
                        cx.notify();
                    })),
                ),
        );
    }
    body = body.child(list);
    body = body.child(
        div()
            .flex()
            .items_center()
            .gap(px(6.0))
            .child(input_field(oki, 110.0))
            .child(input_field(ovi, 140.0))
            .child(
                widgets::button("wf-done", "Complete w/ output", Btn::Outline).on_click(
                    cx.listener(|t, _, _, cx| {
                        let d = match &t.app.dialog {
                            Some(Dialog::Workflows(d)) => d.clone(),
                            _ => return,
                        };
                        act::wf_submit_output(&mut t.app, &d);
                        cx.notify();
                    }),
                ),
            ),
    );
    body = body.child(
        div()
            .flex()
            .items_center()
            .gap(px(6.0))
            .child(input_field(ri, 180.0))
            .child(
                widgets::button("wf-req", "Draft request", Btn::Outline).on_click(cx.listener(
                    |t, _, _, cx| {
                        let d = match &t.app.dialog {
                            Some(Dialog::Workflows(d)) => d.clone(),
                            _ => return,
                        };
                        act::wf_add_request(&mut t.app, &d);
                        cx.notify();
                    },
                )),
            ),
    );
    let reqs: Vec<domain::workflow::Request> = app.app.workflows.requests.to_vec();
    for r in reqs {
        let id = r.id;
        let sent = r.status == domain::workflow::RequestStatus::Sent;
        body = body.child(
            div()
                .flex()
                .items_center()
                .gap(px(6.0))
                .p(px(4.0))
                .rounded(px(4.0))
                .bg(theme::SURFACE)
                .child(
                    div()
                        .flex_grow()
                        .text_size(px(theme::T_SM))
                        .text_color(theme::TEXT)
                        .child(r.name.clone()),
                )
                .child(widgets::label(
                    if sent { "Sent" } else { "Draft" },
                    theme::T_XS,
                    theme::MUTED,
                ))
                .child(
                    widgets::button(
                        SharedString::from(format!("wf-send-{id}")),
                        "Send",
                        Btn::Subtle,
                    )
                    .on_click(cx.listener(move |t, _, _, cx| {
                        act::wf_send_request(&mut t.app, id);
                        cx.notify();
                    })),
                ),
        );
    }
    body = body.child(
        div()
            .flex()
            .items_center()
            .gap(px(6.0))
            .child(widgets::label("Flowchart:", theme::T_SM, theme::FAINT))
            .child(
                widgets::button("wf-exec", "Execute next step", Btn::Outline).on_click(
                    cx.listener(|t, _, _, cx| {
                        act::wf_execute_next(&mut t.app);
                        cx.notify();
                    }),
                ),
            ),
    );
    let stages: Vec<domain::workflow::Stage> = app.app.workflows.stages.to_vec();
    for s in stages {
        let st = app
            .app
            .workflows
            .task(s.task_id)
            .map(|t| t.status.label())
            .unwrap_or("?");
        body = body.child(widgets::label(
            &format!("{} [{}]", s.name, st),
            theme::T_XS,
            theme::MUTED,
        ));
    }
    let footer = footer_row(vec![widgets::button("wf-close", "Close", Btn::Ghost)
        .on_click(cx.listener(|t, _, _, cx| close_dialog(t, cx)))
        .into_any_element()]);
    modal_shell(
        app,
        cx,
        "Workflows",
        760.0,
        max_h,
        true,
        body.into_any_element(),
        Some(footer.into_any_element()),
    )
}

// ---------------------------------------------------------------------------
// Phase 5: Biologics registration wizard
// ---------------------------------------------------------------------------

fn biologics(app: &mut HelixBench, cx: &mut Context<HelixBench>, max_h: Pixels) -> AnyElement {
    let draft = match &app.app.dialog {
        Some(Dialog::Biologics(d)) => d.clone(),
        _ => {
            return modal_shell(
                app,
                cx,
                "Register Biologic",
                660.0,
                max_h,
                true,
                div().into_any_element(),
                None,
            )
        }
    };
    let name_inp = app.input_entity(cx, "dlg:bio:name", &draft.name, "Biologic name", false);
    if let Some(Dialog::Biologics(d)) = &mut app.app.dialog {
        d.name = name_inp.read(cx).text();
    }
    let mut body = div().flex().flex_col().size_full().gap(px(8.0));
    body = body.child(field_row(
        "Name",
        input_field(name_inp, 372.0).into_any_element(),
    ));
    let mut fmts = div().flex().flex_wrap().gap(px(6.0));
    for (i, f) in domain::biologics::Format::ALL.iter().enumerate() {
        let sel = draft.format == i;
        fmts = fmts.child(
            widgets::button(
                SharedString::from(format!("bio-f-{i}")),
                f.label(),
                if sel { Btn::Primary } else { Btn::Outline },
            )
            .on_click(cx.listener(move |t, _, _, cx| {
                if let Some(Dialog::Biologics(d)) = &mut t.app.dialog {
                    d.format = i;
                    let n = domain::biologics::chain_kinds(domain::biologics::Format::ALL[i]).len();
                    d.seqs.resize(n, String::new());
                }
                cx.notify();
            })),
        );
    }
    body = body.child(field_row("Format", fmts.into_any_element()));
    body = body.child(widgets::label(
        domain::biologics::Format::ALL[draft.format.min(5)].chains_hint(),
        theme::T_XS,
        theme::FAINT,
    ));
    let mut nums = div().flex().gap(px(6.0));
    for (i, n) in domain::biologics::Numbering::ALL.iter().enumerate() {
        let sel = draft.numbering == i;
        nums = nums.child(
            widgets::button(
                SharedString::from(format!("bio-n-{i}")),
                n.label(),
                if sel { Btn::Primary } else { Btn::Outline },
            )
            .on_click(cx.listener(move |t, _, _, cx| {
                if let Some(Dialog::Biologics(d)) = &mut t.app.dialog {
                    d.numbering = i;
                }
                cx.notify();
            })),
        );
    }
    body = body.child(field_row("Numbering", nums.into_any_element()));
    let kinds = domain::biologics::chain_kinds(domain::biologics::Format::ALL[draft.format.min(5)]);
    for (i, k) in kinds.iter().enumerate() {
        let seed = draft.seqs.get(i).cloned().unwrap_or_default();
        let inp = app.input_entity(
            cx,
            &format!("dlg:bio:seq{i}"),
            &seed,
            &format!("{} sequence (one-letter AA)", k.label()),
            true,
        );
        let val = inp.read(cx).text();
        if let Some(Dialog::Biologics(d)) = &mut app.app.dialog {
            if d.seqs.len() <= i {
                d.seqs.resize(i + 1, String::new());
            }
            d.seqs[i] = val;
        }
        body = body.child(field_row(
            &format!("Chain {} ({})", i + 1, k.label()),
            input_field(inp, 480.0).into_any_element(),
        ));
    }
    if let Some(e) = &draft.error {
        body = body.child(error_label(e));
    }
    let footer = footer_row(vec![
        widgets::button("bio-cancel", "Cancel", Btn::Ghost)
            .on_click(cx.listener(|t, _, _, cx| close_dialog(t, cx)))
            .into_any_element(),
        widgets::button("bio-save", "Register", Btn::Primary)
            .on_click(cx.listener(|t, _, _, cx| {
                let d = match &t.app.dialog {
                    Some(Dialog::Biologics(d)) => d.clone(),
                    _ => return,
                };
                match act::biologics_save(&mut t.app, &d) {
                    Ok(_) => close_dialog(t, cx),
                    Err(e) => {
                        if let Some(Dialog::Biologics(dd)) = &mut t.app.dialog {
                            dd.error = Some(e);
                        }
                        cx.notify();
                    }
                }
            }))
            .into_any_element(),
    ]);
    modal_shell(
        app,
        cx,
        "Register Biologic",
        660.0,
        max_h,
        true,
        body.into_any_element(),
        Some(footer.into_any_element()),
    )
}

// ---------------------------------------------------------------------------
// Phase 5: Chemistry dialogs
// ---------------------------------------------------------------------------

fn chem_add(app: &mut HelixBench, cx: &mut Context<HelixBench>, max_h: Pixels) -> AnyElement {
    let draft = match &app.app.dialog {
        Some(Dialog::ChemAdd(d)) => d.clone(),
        _ => return div().into_any_element(),
    };
    let name_inp = app.input_entity(cx, "dlg:chem:name", &draft.name, "Compound name", false);
    let smi_inp = app.input_entity(
        cx,
        "dlg:chem:smiles",
        &draft.smiles,
        "SMILES e.g. CCO",
        true,
    );
    if let Some(Dialog::ChemAdd(d)) = &mut app.app.dialog {
        d.name = name_inp.read(cx).text();
        d.smiles = smi_inp.read(cx).text();
    }
    let mut body = div().flex().flex_col().size_full().gap(px(8.0));
    body = body.child(field_row(
        "Name",
        input_field(name_inp, 372.0).into_any_element(),
    ));
    body = body.child(field_row(
        "SMILES",
        input_field(smi_inp, 480.0).into_any_element(),
    ));
    if let Ok(m) = domain::chemistry::Molecule::parse(&draft.smiles) {
        body = body.child(widgets::label(
            &format!("{} · MW {:.2}", m.formula(), m.molecular_weight()),
            theme::T_SM,
            theme::OK,
        ));
    }
    if let Some(e) = &draft.error {
        body = body.child(error_label(e));
    }
    let footer = footer_row(vec![
        widgets::button("chem-add-cancel", "Cancel", Btn::Ghost)
            .on_click(cx.listener(|t, _, _, cx| close_dialog(t, cx)))
            .into_any_element(),
        widgets::button("chem-add-save", "Add", Btn::Primary)
            .on_click(cx.listener(|t, _, _, cx| {
                let d = match &t.app.dialog {
                    Some(Dialog::ChemAdd(d)) => d.clone(),
                    _ => return,
                };
                match act::chem_add_save(&mut t.app, &d) {
                    Ok(_) => close_dialog(t, cx),
                    Err(e) => {
                        if let Some(Dialog::ChemAdd(dd)) = &mut t.app.dialog {
                            dd.error = Some(e);
                        }
                        cx.notify();
                    }
                }
            }))
            .into_any_element(),
    ]);
    modal_shell(
        app,
        cx,
        "Add Compound",
        620.0,
        max_h,
        true,
        body.into_any_element(),
        Some(footer.into_any_element()),
    )
}

fn chem_reaction(app: &mut HelixBench, cx: &mut Context<HelixBench>, max_h: Pixels) -> AnyElement {
    let draft = match &app.app.dialog {
        Some(Dialog::ChemReaction(d)) => d.clone(),
        _ => return div().into_any_element(),
    };
    let name_inp = app.input_entity(cx, "dlg:rxn:name", &draft.name, "Reaction name", false);
    let req_inp = app.input_entity(cx, "dlg:rxn:req", &draft.req, "1.0", true);
    let peq_inp = app.input_entity(cx, "dlg:rxn:peq", &draft.peq, "1.0", true);
    if let Some(Dialog::ChemReaction(d)) = &mut app.app.dialog {
        d.name = name_inp.read(cx).text();
        d.req = req_inp.read(cx).text();
        d.peq = peq_inp.read(cx).text();
    }
    let compounds = app.app.chemistry.compounds.clone();
    let n = compounds.len();
    let rname = compounds
        .get(draft.reactant.min(n.saturating_sub(1)))
        .map(|c| c.name.clone())
        .unwrap_or_else(|| "—".into());
    let pname = compounds
        .get(draft.product.min(n.saturating_sub(1)))
        .map(|c| c.name.clone())
        .unwrap_or_else(|| "—".into());
    let mut body = div().flex().flex_col().size_full().gap(px(8.0));
    body = body.child(field_row(
        "Name",
        input_field(name_inp, 372.0).into_any_element(),
    ));
    body = body.child(field_row(
        "Reactant",
        div()
            .flex()
            .items_center()
            .gap(px(8.0))
            .child(
                widgets::button("rxn-r-cycle", &rname, Btn::Outline).on_click(cx.listener(
                    move |t, _, _, cx| {
                        if let Some(Dialog::ChemReaction(d)) = &mut t.app.dialog {
                            let n = t.app.chemistry.compounds.len().max(1);
                            d.reactant = (d.reactant + 1) % n;
                        }
                        cx.notify();
                    },
                )),
            )
            .child(div().w(px(70.0)).child(req_inp))
            .child(widgets::label("eq", theme::T_XS, theme::FAINT))
            .into_any_element(),
    ));
    body = body.child(field_row(
        "Product",
        div()
            .flex()
            .items_center()
            .gap(px(8.0))
            .child(
                widgets::button("rxn-p-cycle", &pname, Btn::Outline).on_click(cx.listener(
                    move |t, _, _, cx| {
                        if let Some(Dialog::ChemReaction(d)) = &mut t.app.dialog {
                            let n = t.app.chemistry.compounds.len().max(1);
                            d.product = (d.product + 1) % n;
                        }
                        cx.notify();
                    },
                )),
            )
            .child(div().w(px(70.0)).child(peq_inp))
            .child(widgets::label("eq", theme::T_XS, theme::FAINT))
            .into_any_element(),
    ));
    body = body.child(widgets::label(
        "Click a compound button to cycle the selection.",
        theme::T_XS,
        theme::FAINT,
    ));
    if let Some(e) = &draft.error {
        body = body.child(error_label(e));
    }
    let footer = footer_row(vec![
        widgets::button("rxn-cancel", "Cancel", Btn::Ghost)
            .on_click(cx.listener(|t, _, _, cx| close_dialog(t, cx)))
            .into_any_element(),
        widgets::button("rxn-save", "Add", Btn::Primary)
            .on_click(cx.listener(|t, _, _, cx| {
                let d = match &t.app.dialog {
                    Some(Dialog::ChemReaction(d)) => d.clone(),
                    _ => return,
                };
                match act::chem_reaction_save(&mut t.app, &d) {
                    Ok(_) => close_dialog(t, cx),
                    Err(e) => {
                        if let Some(Dialog::ChemReaction(dd)) = &mut t.app.dialog {
                            dd.error = Some(e);
                        }
                        cx.notify();
                    }
                }
            }))
            .into_any_element(),
    ]);
    modal_shell(
        app,
        cx,
        "Add Reaction",
        620.0,
        max_h,
        true,
        body.into_any_element(),
        Some(footer.into_any_element()),
    )
}

fn chem_lot(app: &mut HelixBench, cx: &mut Context<HelixBench>, max_h: Pixels) -> AnyElement {
    let draft = match &app.app.dialog {
        Some(Dialog::ChemLot(d)) => d.clone(),
        _ => return div().into_any_element(),
    };
    let qty_inp = app.input_entity(cx, "dlg:lot:qty", &draft.quantity, "100", true);
    let unit_inp = app.input_entity(cx, "dlg:lot:unit", &draft.unit, "mg", false);
    if let Some(Dialog::ChemLot(d)) = &mut app.app.dialog {
        d.quantity = qty_inp.read(cx).text();
        d.unit = unit_inp.read(cx).text();
    }
    let mut body = div().flex().flex_col().size_full().gap(px(8.0));
    let mut forms = div().flex().flex_wrap().gap(px(6.0));
    for (i, f) in domain::chemistry::salt_solvate_library().iter().enumerate() {
        let sel = draft.form == i;
        forms = forms.child(
            widgets::button(
                SharedString::from(format!("lot-f-{i}")),
                &f.name,
                if sel { Btn::Primary } else { Btn::Outline },
            )
            .on_click(cx.listener(move |t, _, _, cx| {
                if let Some(Dialog::ChemLot(d)) = &mut t.app.dialog {
                    d.form = i;
                }
                cx.notify();
            })),
        );
    }
    body = body.child(field_row("Form / salt", forms.into_any_element()));
    body = body.child(field_row(
        "Quantity",
        div()
            .flex()
            .items_center()
            .gap(px(8.0))
            .child(div().w(px(90.0)).child(qty_inp))
            .child(div().w(px(70.0)).child(unit_inp))
            .into_any_element(),
    ));
    if let Some(e) = &draft.error {
        body = body.child(error_label(e));
    }
    let footer = footer_row(vec![
        widgets::button("lot-cancel", "Cancel", Btn::Ghost)
            .on_click(cx.listener(|t, _, _, cx| close_dialog(t, cx)))
            .into_any_element(),
        widgets::button("lot-save", "Add", Btn::Primary)
            .on_click(cx.listener(|t, _, _, cx| {
                let d = match &t.app.dialog {
                    Some(Dialog::ChemLot(d)) => d.clone(),
                    _ => return,
                };
                match act::chem_lot_save(&mut t.app, &d) {
                    Ok(_) => close_dialog(t, cx),
                    Err(e) => {
                        if let Some(Dialog::ChemLot(dd)) = &mut t.app.dialog {
                            dd.error = Some(e);
                        }
                        cx.notify();
                    }
                }
            }))
            .into_any_element(),
    ]);
    modal_shell(
        app,
        cx,
        "Add Lot",
        620.0,
        max_h,
        true,
        body.into_any_element(),
        Some(footer.into_any_element()),
    )
}

// ---------------------------------------------------------------------------
// Phase 5: RNA / oligo dialog
// ---------------------------------------------------------------------------

fn rna(app: &mut HelixBench, cx: &mut Context<HelixBench>, max_h: Pixels) -> AnyElement {
    let draft = match &app.app.dialog {
        Some(Dialog::Rna(d)) => d.clone(),
        _ => return div().into_any_element(),
    };
    let name_inp = app.input_entity(cx, "dlg:rna:name", &draft.name, "Oligo name", false);
    let seq_inp = app.input_entity(cx, "dlg:rna:seq", &draft.seq, "RNA sequence A/U/G/C", true);
    let helm_inp = app.input_entity(
        cx,
        "dlg:rna:helm",
        &draft.helm,
        "optional HELM e.g. RNA1{[mA]PUP}",
        true,
    );
    if let Some(Dialog::Rna(d)) = &mut app.app.dialog {
        d.name = name_inp.read(cx).text();
        d.seq = seq_inp.read(cx).text();
        d.helm = helm_inp.read(cx).text();
    }
    let mut body = div().flex().flex_col().size_full().gap(px(8.0));
    body = body.child(field_row(
        "Name",
        input_field(name_inp, 372.0).into_any_element(),
    ));
    body = body.child(field_row(
        "Sequence",
        input_field(seq_inp, 480.0).into_any_element(),
    ));
    let mut pats = div().flex().flex_wrap().gap(px(6.0));
    for (i, pat) in domain::rna::Pattern::ALL.iter().enumerate() {
        let sel = draft.pattern == i;
        pats = pats.child(
            widgets::button(
                SharedString::from(format!("rna-dpat-{i}")),
                pat.label(),
                if sel { Btn::Primary } else { Btn::Outline },
            )
            .on_click(cx.listener(move |t, _, _, cx| {
                if let Some(Dialog::Rna(d)) = &mut t.app.dialog {
                    d.pattern = i;
                }
                cx.notify();
            })),
        );
    }
    body = body.child(field_row("Pattern", pats.into_any_element()));
    let lib = domain::rna::monomer_library();
    let mut mods = div().flex().flex_wrap().gap(px(6.0));
    for (i, m) in lib.iter().enumerate() {
        let sel = draft.mod_code == i;
        mods = mods.child(
            widgets::button(
                SharedString::from(format!("rna-dmod-{i}")),
                &m.code,
                if sel { Btn::Primary } else { Btn::Outline },
            )
            .on_click(cx.listener(move |t, _, _, cx| {
                if let Some(Dialog::Rna(d)) = &mut t.app.dialog {
                    d.mod_code = i;
                }
                cx.notify();
            })),
        );
    }
    body = body.child(field_row("Modification", mods.into_any_element()));
    body = body.child(field_row(
        "HELM (optional)",
        input_field(helm_inp, 480.0).into_any_element(),
    ));
    if let Some(e) = &draft.error {
        body = body.child(error_label(e));
    }
    let footer = footer_row(vec![
        widgets::button("rna-cancel", "Cancel", Btn::Ghost)
            .on_click(cx.listener(|t, _, _, cx| close_dialog(t, cx)))
            .into_any_element(),
        widgets::button("rna-save", "Add", Btn::Primary)
            .on_click(cx.listener(|t, _, _, cx| {
                let d = match &t.app.dialog {
                    Some(Dialog::Rna(d)) => d.clone(),
                    _ => return,
                };
                match act::rna_add_save(&mut t.app, &d) {
                    Ok(_) => close_dialog(t, cx),
                    Err(e) => {
                        if let Some(Dialog::Rna(dd)) = &mut t.app.dialog {
                            dd.error = Some(e);
                        }
                        cx.notify();
                    }
                }
            }))
            .into_any_element(),
    ]);
    modal_shell(
        app,
        cx,
        "Add Oligo",
        660.0,
        max_h,
        true,
        body.into_any_element(),
        Some(footer.into_any_element()),
    )
}

// ---------------------------------------------------------------------------
// Phase 5: In Vivo dialogs
// ---------------------------------------------------------------------------

fn iv_study(app: &mut HelixBench, cx: &mut Context<HelixBench>, max_h: Pixels) -> AnyElement {
    let draft = match &app.app.dialog {
        Some(Dialog::IvStudy(d)) => d.clone(),
        _ => return div().into_any_element(),
    };
    let name_inp = app.input_entity(cx, "dlg:iv:name", &draft.name, "Study name", false);
    let sp_inp = app.input_entity(cx, "dlg:iv:species", &draft.species, "mouse", false);
    let st_inp = app.input_entity(cx, "dlg:iv:strain", &draft.strain, "BALB/c", false);
    if let Some(Dialog::IvStudy(d)) = &mut app.app.dialog {
        d.name = name_inp.read(cx).text();
        d.species = sp_inp.read(cx).text();
        d.strain = st_inp.read(cx).text();
    }
    let mut body = div().flex().flex_col().size_full().gap(px(8.0));
    body = body.child(field_row(
        "Name",
        input_field(name_inp, 372.0).into_any_element(),
    ));
    body = body.child(field_row(
        "Species",
        input_field(sp_inp, 200.0).into_any_element(),
    ));
    body = body.child(field_row(
        "Strain",
        input_field(st_inp, 200.0).into_any_element(),
    ));
    if let Some(e) = &draft.error {
        body = body.child(error_label(e));
    }
    let footer = footer_row(vec![
        widgets::button("iv-study-cancel", "Cancel", Btn::Ghost)
            .on_click(cx.listener(|t, _, _, cx| close_dialog(t, cx)))
            .into_any_element(),
        widgets::button("iv-study-save", "Create", Btn::Primary)
            .on_click(cx.listener(|t, _, _, cx| {
                let d = match &t.app.dialog {
                    Some(Dialog::IvStudy(d)) => d.clone(),
                    _ => return,
                };
                match act::iv_study_save(&mut t.app, &d) {
                    Ok(_) => close_dialog(t, cx),
                    Err(e) => {
                        if let Some(Dialog::IvStudy(dd)) = &mut t.app.dialog {
                            dd.error = Some(e);
                        }
                        cx.notify();
                    }
                }
            }))
            .into_any_element(),
    ]);
    modal_shell(
        app,
        cx,
        "New Study",
        620.0,
        max_h,
        true,
        body.into_any_element(),
        Some(footer.into_any_element()),
    )
}

fn iv_animal(app: &mut HelixBench, cx: &mut Context<HelixBench>, max_h: Pixels) -> AnyElement {
    let draft = match &app.app.dialog {
        Some(Dialog::IvAnimal(d)) => d.clone(),
        _ => return div().into_any_element(),
    };
    let tag_inp = app.input_entity(cx, "dlg:iv:tag", &draft.tag, "M1", false);
    let w_inp = app.input_entity(cx, "dlg:iv:weight", &draft.weight, "20", true);
    if let Some(Dialog::IvAnimal(d)) = &mut app.app.dialog {
        d.tag = tag_inp.read(cx).text();
        d.weight = w_inp.read(cx).text();
    }
    let mut body = div().flex().flex_col().size_full().gap(px(8.0));
    body = body.child(field_row(
        "Tag",
        input_field(tag_inp, 200.0).into_any_element(),
    ));
    let mut sexes = div().flex().gap(px(6.0));
    for (i, label) in [(0usize, "Female"), (1, "Male")] {
        let sel = draft.sex == i;
        sexes = sexes.child(
            widgets::button(
                SharedString::from(format!("iv-sex-{i}")),
                label,
                if sel { Btn::Primary } else { Btn::Outline },
            )
            .on_click(cx.listener(move |t, _, _, cx| {
                if let Some(Dialog::IvAnimal(d)) = &mut t.app.dialog {
                    d.sex = i;
                }
                cx.notify();
            })),
        );
    }
    body = body.child(field_row("Sex", sexes.into_any_element()));
    body = body.child(field_row(
        "Weight (g)",
        input_field(w_inp, 120.0).into_any_element(),
    ));
    if let Some(e) = &draft.error {
        body = body.child(error_label(e));
    }
    let footer = footer_row(vec![
        widgets::button("iv-animal-cancel", "Cancel", Btn::Ghost)
            .on_click(cx.listener(|t, _, _, cx| close_dialog(t, cx)))
            .into_any_element(),
        widgets::button("iv-animal-save", "Add", Btn::Primary)
            .on_click(cx.listener(|t, _, _, cx| {
                let d = match &t.app.dialog {
                    Some(Dialog::IvAnimal(d)) => d.clone(),
                    _ => return,
                };
                match act::iv_animal_save(&mut t.app, &d) {
                    Ok(_) => close_dialog(t, cx),
                    Err(e) => {
                        if let Some(Dialog::IvAnimal(dd)) = &mut t.app.dialog {
                            dd.error = Some(e);
                        }
                        cx.notify();
                    }
                }
            }))
            .into_any_element(),
    ]);
    modal_shell(
        app,
        cx,
        "Add Animal",
        620.0,
        max_h,
        true,
        body.into_any_element(),
        Some(footer.into_any_element()),
    )
}

fn iv_measure(app: &mut HelixBench, cx: &mut Context<HelixBench>, max_h: Pixels) -> AnyElement {
    let draft = match &app.app.dialog {
        Some(Dialog::IvMeasure(d)) => d.clone(),
        _ => return div().into_any_element(),
    };
    let day_inp = app.input_entity(cx, "dlg:iv:day", &draft.day, "0", true);
    let w_inp = app.input_entity(cx, "dlg:iv:mweight", &draft.weight, "20", true);
    let t_inp = app.input_entity(cx, "dlg:iv:tumor", &draft.tumor, "optional mm3", true);
    if let Some(Dialog::IvMeasure(d)) = &mut app.app.dialog {
        d.day = day_inp.read(cx).text();
        d.weight = w_inp.read(cx).text();
        d.tumor = t_inp.read(cx).text();
    }
    let study = app.app.iv_sel;
    let animals: Vec<domain::invivo::Animal> = app
        .app
        .invivo
        .animals
        .iter()
        .filter(|a| {
            app.app
                .invivo
                .cohorts
                .iter()
                .any(|c| c.id == a.cohort && Some(c.study) == study)
        })
        .cloned()
        .collect();
    let mut body = div().flex().flex_col().size_full().gap(px(8.0));
    let mut picks = div().flex().flex_wrap().gap(px(6.0));
    for (i, a) in animals.iter().enumerate() {
        let sel = draft.animal == i;
        picks = picks.child(
            widgets::button(
                SharedString::from(format!("iv-manim-{i}")),
                &a.tag,
                if sel { Btn::Primary } else { Btn::Outline },
            )
            .on_click(cx.listener(move |t, _, _, cx| {
                if let Some(Dialog::IvMeasure(d)) = &mut t.app.dialog {
                    d.animal = i;
                }
                cx.notify();
            })),
        );
    }
    body = body.child(field_row("Animal", picks.into_any_element()));
    body = body.child(field_row(
        "Day",
        input_field(day_inp, 90.0).into_any_element(),
    ));
    body = body.child(field_row(
        "Weight (g)",
        input_field(w_inp, 120.0).into_any_element(),
    ));
    body = body.child(field_row(
        "Tumor (mm3)",
        input_field(t_inp, 120.0).into_any_element(),
    ));
    if let Some(e) = &draft.error {
        body = body.child(error_label(e));
    }
    let footer = footer_row(vec![
        widgets::button("iv-measure-cancel", "Cancel", Btn::Ghost)
            .on_click(cx.listener(|t, _, _, cx| close_dialog(t, cx)))
            .into_any_element(),
        widgets::button("iv-measure-save", "Record", Btn::Primary)
            .on_click(cx.listener(|t, _, _, cx| {
                let d = match &t.app.dialog {
                    Some(Dialog::IvMeasure(d)) => d.clone(),
                    _ => return,
                };
                match act::iv_measure_save(&mut t.app, &d) {
                    Ok(_) => close_dialog(t, cx),
                    Err(e) => {
                        if let Some(Dialog::IvMeasure(dd)) = &mut t.app.dialog {
                            dd.error = Some(e);
                        }
                        cx.notify();
                    }
                }
            }))
            .into_any_element(),
    ]);
    modal_shell(
        app,
        cx,
        "Record Measurement",
        620.0,
        max_h,
        true,
        body.into_any_element(),
        Some(footer.into_any_element()),
    )
}

// ---------------------------------------------------------------------------
// Phase 5: Bioprocess dialogs
// ---------------------------------------------------------------------------

fn bp_recipe(app: &mut HelixBench, cx: &mut Context<HelixBench>, max_h: Pixels) -> AnyElement {
    let draft = match &app.app.dialog {
        Some(Dialog::BpRecipe(d)) => d.clone(),
        _ => return div().into_any_element(),
    };
    let name_inp = app.input_entity(cx, "dlg:bp:name", &draft.name, "Recipe name", false);
    let prod_inp = app.input_entity(cx, "dlg:bp:product", &draft.product, "Product", false);
    if let Some(Dialog::BpRecipe(d)) = &mut app.app.dialog {
        d.name = name_inp.read(cx).text();
        d.product = prod_inp.read(cx).text();
    }
    let mut body = div().flex().flex_col().size_full().gap(px(8.0));
    body = body.child(field_row(
        "Name",
        input_field(name_inp, 372.0).into_any_element(),
    ));
    body = body.child(field_row(
        "Product",
        input_field(prod_inp, 240.0).into_any_element(),
    ));
    body = body.child(widgets::label(
        "Creates a default template: Inoculate → Feed → Induce → Harvest → Clarify → Purify.",
        theme::T_XS,
        theme::FAINT,
    ));
    if let Some(e) = &draft.error {
        body = body.child(error_label(e));
    }
    let footer = footer_row(vec![
        widgets::button("bp-recipe-cancel", "Cancel", Btn::Ghost)
            .on_click(cx.listener(|t, _, _, cx| close_dialog(t, cx)))
            .into_any_element(),
        widgets::button("bp-recipe-save", "Create", Btn::Primary)
            .on_click(cx.listener(|t, _, _, cx| {
                let d = match &t.app.dialog {
                    Some(Dialog::BpRecipe(d)) => d.clone(),
                    _ => return,
                };
                match act::bp_recipe_save(&mut t.app, &d) {
                    Ok(_) => close_dialog(t, cx),
                    Err(e) => {
                        if let Some(Dialog::BpRecipe(dd)) = &mut t.app.dialog {
                            dd.error = Some(e);
                        }
                        cx.notify();
                    }
                }
            }))
            .into_any_element(),
    ]);
    modal_shell(
        app,
        cx,
        "New Recipe",
        620.0,
        max_h,
        true,
        body.into_any_element(),
        Some(footer.into_any_element()),
    )
}

fn bp_replicate(app: &mut HelixBench, cx: &mut Context<HelixBench>, max_h: Pixels) -> AnyElement {
    let draft = match &app.app.dialog {
        Some(Dialog::BpReplicate(d)) => d.clone(),
        _ => return div().into_any_element(),
    };
    let label_inp = app.input_entity(cx, "dlg:bp:label", &draft.label, "Replicate label", false);
    let titer_inp = app.input_entity(cx, "dlg:bp:titer", &draft.titer, "mg/L", true);
    let temp_inp = app.input_entity(cx, "dlg:bp:temp", &draft.temp, "37", true);
    let ph_inp = app.input_entity(cx, "dlg:bp:ph", &draft.ph, "7.0", true);
    if let Some(Dialog::BpReplicate(d)) = &mut app.app.dialog {
        d.label = label_inp.read(cx).text();
        d.titer = titer_inp.read(cx).text();
        d.temp = temp_inp.read(cx).text();
        d.ph = ph_inp.read(cx).text();
    }
    let mut body = div().flex().flex_col().size_full().gap(px(8.0));
    body = body.child(field_row(
        "Label",
        input_field(label_inp, 200.0).into_any_element(),
    ));
    body = body.child(field_row(
        "Titer (mg/L)",
        input_field(titer_inp, 120.0).into_any_element(),
    ));
    body = body.child(field_row(
        "Temp (°C)",
        input_field(temp_inp, 90.0).into_any_element(),
    ));
    body = body.child(field_row(
        "pH",
        input_field(ph_inp, 90.0).into_any_element(),
    ));
    if let Some(e) = &draft.error {
        body = body.child(error_label(e));
    }
    let footer = footer_row(vec![
        widgets::button("bp-rep-cancel", "Cancel", Btn::Ghost)
            .on_click(cx.listener(|t, _, _, cx| close_dialog(t, cx)))
            .into_any_element(),
        widgets::button("bp-rep-save", "Add", Btn::Primary)
            .on_click(cx.listener(|t, _, _, cx| {
                let d = match &t.app.dialog {
                    Some(Dialog::BpReplicate(d)) => d.clone(),
                    _ => return,
                };
                match act::bp_replicate_save(&mut t.app, &d) {
                    Ok(_) => close_dialog(t, cx),
                    Err(e) => {
                        if let Some(Dialog::BpReplicate(dd)) = &mut t.app.dialog {
                            dd.error = Some(e);
                        }
                        cx.notify();
                    }
                }
            }))
            .into_any_element(),
    ]);
    modal_shell(
        app,
        cx,
        "Add Replicate",
        620.0,
        max_h,
        true,
        body.into_any_element(),
        Some(footer.into_any_element()),
    )
}
