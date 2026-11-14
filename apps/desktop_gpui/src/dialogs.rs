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
use appcore::state::{AppState, ColorsMode, Dialog, FeatureDraft, PendingAction, PrimerDraft};
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
        .shadow_lg()
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
        .shadow_lg()
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
