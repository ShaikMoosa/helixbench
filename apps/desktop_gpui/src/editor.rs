//! Light, modern editor: circular map on the left, an annotated
//! double-stranded sequence panel on the right, and description / properties /
//! legend cards along the bottom. Pastel feature fills on a white canvas.

use appcore::state::MapMode;
use domain::Range;
use gpui::{
    canvas, div, point, prelude::*, px, size, AnyElement, Bounds, Context, MouseButton,
    MouseDownEvent, MouseMoveEvent, MouseUpEvent, Pixels, Point, ScrollWheelEvent, Window,
};

use crate::app::HelixBench;
use crate::draw::{self, Align, PxVal};
use crate::theme;

const SEQ_W: f32 = 430.0;
const CHAR_W: f32 = 7.4;
const PAD: f32 = 14.0;
const ROW_H: f32 = 78.0;

fn bases_per_row(width: f32) -> usize {
    (((width - PAD * 2.0) / CHAR_W).floor() as usize).max(10)
}

fn complement(b: u8) -> u8 {
    match b {
        b'A' => b'T',
        b'T' => b'A',
        b'C' => b'G',
        b'G' => b'C',
        _ => b'N',
    }
}

struct SeqDraw {
    seq: Vec<u8>,
    features: Vec<(String, [u8; 3], usize, usize)>,
    selection: Option<Range>,
    scroll: f32,
}

impl SeqDraw {
    fn build(app: &appcore::state::AppState, scroll: f32) -> Self {
        let l = app.doc.sequence.len();
        let features = app
            .doc
            .features
            .iter()
            .filter(|f| f.visible && !f.segments.is_empty())
            .map(|f| {
                let r = f.span(l);
                (f.name.clone(), f.color, r.start, r.start + r.len)
            })
            .collect();
        SeqDraw {
            seq: app.doc.sequence.bases.clone(),
            features,
            selection: app.selection,
            scroll,
        }
    }
}

fn paint_seq(d: &SeqDraw, bounds: Bounds<Pixels>, window: &mut Window, cx: &mut gpui::App) {
    draw::rect(window, bounds, 0.0, theme::PANEL);
    let l = d.seq.len();
    if l == 0 {
        return;
    }
    let n = bases_per_row(bounds.size.width.v());
    let rows = l.div_ceil(n);
    let view_h = bounds.size.height.v();
    let content_h = rows as f32 * ROW_H;
    let max_scroll = (content_h - view_h).max(0.0);
    let scroll = d.scroll.clamp(0.0, max_scroll);
    let first = (scroll / ROW_H).floor() as usize;
    let y0 = bounds.origin.y.v() - (scroll - first as f32 * ROW_H);
    let x_of = |i: usize| bounds.origin.x.v() + PAD + (i % n) as f32 * CHAR_W;

    for r in first..rows {
        let y = y0 + (r - first) as f32 * ROW_H;
        if y > bounds.origin.y.v() + view_h {
            break;
        }
        if y + ROW_H < bounds.origin.y.v() {
            continue;
        }
        let start = r * n;
        let end = (start + n).min(l);
        // top strand letters
        for (i, b) in d.seq.iter().enumerate().take(end).skip(start) {
            let ch = (*b as char).to_string();
            draw::text(
                window,
                cx,
                point(px(x_of(i) + CHAR_W / 2.0), px(y + 10.0)),
                &ch,
                11.0,
                theme::MUTED,
                theme::mono_font(),
                Align::Center,
            );
        }
        // pastel feature bars (two lanes)
        for (fi, (name, color, fs, fe)) in d.features.iter().enumerate() {
            if *fe <= start || *fs >= end {
                continue;
            }
            let lane = fi % 2;
            let by = y + 20.0 + lane as f32 * 13.0;
            let x0 = x_of((*fs).max(start));
            let x1 = x_of((*fe).min(end).saturating_sub(1)) + CHAR_W;
            let bar = Bounds::new(
                point(px(x0), px(by)),
                size(px((x1 - x0).max(2.0)), px(11.0)),
            );
            draw::rect(window, bar, 3.0, theme::pastel(*color));
            if x1 - x0 > 46.0 {
                draw::text(
                    window,
                    cx,
                    point(px(x0 + 4.0), px(by + 5.5)),
                    name,
                    9.0,
                    theme::TEXT,
                    theme::ui_font(),
                    Align::Left,
                );
            }
        }
        // bottom strand letters
        for (i, b) in d.seq.iter().enumerate().take(end).skip(start) {
            let ch = (complement(*b) as char).to_string();
            draw::text(
                window,
                cx,
                point(px(x_of(i) + CHAR_W / 2.0), px(y + 50.0)),
                &ch,
                11.0,
                theme::FAINT,
                theme::mono_font(),
                Align::Center,
            );
        }
        // ruler
        let step = 50usize;
        let mut t = start.div_ceil(step) * step;
        while t < end {
            let x = x_of(t);
            draw::segment(
                window,
                point(px(x), px(y + 62.0)),
                point(px(x), px(y + 66.0)),
                1.0,
                theme::BORDER_STRONG,
            );
            draw::text(
                window,
                cx,
                point(px(x), px(y + 72.0)),
                &t.to_string(),
                9.0,
                theme::FAINT,
                theme::mono_font(),
                Align::Center,
            );
            t += step;
        }
        // selection wash + outline
        if let Some(sel) = d.selection {
            let s = sel.start.max(start);
            let e = (sel.start + sel.len).min(end);
            if e > s {
                let x0 = x_of(s);
                let x1 = x_of(e.saturating_sub(1)) + CHAR_W;
                let sb = Bounds::new(point(px(x0), px(y + 2.0)), size(px(x1 - x0), px(56.0)));
                draw::rect(window, sb, 2.0, theme::SELECT_FILL);
                draw::rect_border(window, sb, 2.0, theme::transparent(), 1.0, theme::SELECT);
            }
        }
    }
    let track = Bounds::new(
        point(
            bounds.origin.x + bounds.size.width - px(8.0),
            bounds.origin.y,
        ),
        size(px(6.0), bounds.size.height),
    );
    draw::vscrollbar(window, track, scroll, view_h, content_h);
}

struct MiniDraw {
    l: usize,
    features: Vec<(usize, usize, [u8; 3])>,
    zoom_start: usize,
    zoom_width: usize,
    selection: Option<Range>,
}

fn arc(c: Point<Pixels>, r: f32, a0: f32, a1: f32) -> Vec<Point<Pixels>> {
    let n = 24;
    (0..=n)
        .map(|i| {
            let a = a0 + (a1 - a0) * i as f32 / n as f32;
            point(c.x + px(r * a.cos()), c.y + px(r * a.sin()))
        })
        .collect()
}

fn paint_mini(d: &MiniDraw, bounds: Bounds<Pixels>, window: &mut Window, _cx: &mut gpui::App) {
    let c = bounds.center();
    let r = bounds.size.width.v() / 2.0 - 6.0;
    draw::disc(window, c, r + 4.0, theme::PANEL);
    draw::ring(window, c, r + 4.0, 1.0, theme::BORDER);
    if d.l == 0 {
        return;
    }
    let tau = std::f32::consts::TAU;
    for &(s, e, color) in &d.features {
        let a0 = tau * s as f32 / d.l as f32;
        let a1 = tau * e as f32 / d.l as f32;
        draw::polyline(window, &arc(c, r - 3.0, a0, a1), 4.0, theme::pastel(color));
    }
    draw::ring(window, c, r - 9.0, 1.0, theme::BORDER_STRONG);
    if d.zoom_width < d.l {
        let a0 = tau * d.zoom_start as f32 / d.l as f32;
        let a1 = a0 + tau * d.zoom_width as f32 / d.l as f32;
        draw::polyline(window, &arc(c, r - 9.0, a0, a1), 3.0, theme::ACCENT);
    }
    if let Some(sel) = d.selection {
        let a0 = tau * sel.start as f32 / d.l as f32;
        let a1 = a0 + tau * sel.len as f32 / d.l as f32;
        draw::polyline(window, &arc(c, r + 1.0, a0, a1), 2.0, theme::SELECT);
    }
}

fn paint_slider(bounds: Bounds<Pixels>, frac: f32, window: &mut Window, _cx: &mut gpui::App) {
    let track = Bounds::new(
        point(
            bounds.origin.x + bounds.size.width / 2.0 - px(2.0),
            bounds.origin.y,
        ),
        size(px(4.0), bounds.size.height),
    );
    draw::rect(window, track, 2.0, theme::SURFACE_2);
    let thumb_h = px(14.0);
    let travel = bounds.size.height - thumb_h;
    let ty = bounds.origin.y + travel * (1.0 - frac.clamp(0.0, 1.0));
    let thumb = Bounds::new(
        point(bounds.origin.x + bounds.size.width / 2.0 - px(5.0), ty),
        size(px(10.0), thumb_h),
    );
    draw::rect(window, thumb, 3.0, theme::BORDER_STRONG);
}

pub fn render_editor(this: &mut HelixBench, cx: &mut Context<HelixBench>) -> AnyElement {
    this.app.ensure_analysis();
    let l = this.app.doc.sequence.len();

    // Left: map + minimap + zoom slider overlays.
    let map = this.render_map(cx);
    let mini = MiniDraw {
        l,
        features: this
            .app
            .doc
            .features
            .iter()
            .filter(|f| f.visible && !f.segments.is_empty())
            .map(|f| {
                let r = f.span(l);
                (r.start, r.start + r.len, f.color)
            })
            .collect(),
        zoom_start: this.app.zoom_start,
        zoom_width: this.app.zoom_width,
        selection: this.app.selection,
    };
    let minimap = div()
        .absolute()
        .top(px(10.0))
        .right(px(10.0))
        .w(px(96.0))
        .h(px(96.0))
        .child(
            canvas(
                move |_, _, _| {},
                move |bounds, _s, window, cx| paint_mini(&mini, bounds, window, cx),
            )
            .size_full(),
        );
    let frac = if l > 0 {
        this.app.zoom_width as f32 / l as f32
    } else {
        1.0
    };
    let e2 = cx.entity();
    let slider = div()
        .id("editor-zoom-slider")
        .absolute()
        .top(px(120.0))
        .right(px(14.0))
        .w(px(14.0))
        .h(px(150.0))
        .cursor_pointer()
        .child(
            canvas(
                move |bounds, _w, cx| {
                    e2.update(cx, |t, _| t.editor_slider_bounds = Some(bounds));
                },
                move |bounds, _s, window, cx| paint_slider(bounds, frac, window, cx),
            )
            .size_full(),
        )
        .on_mouse_down(
            MouseButton::Left,
            cx.listener(HelixBench::editor_slider_down),
        )
        .on_mouse_move(cx.listener(HelixBench::editor_slider_move));
    let mut map_panel = div()
        .relative()
        .flex_grow()
        .h_full()
        .overflow_hidden()
        .child(map)
        .child(slider);
    if this.app.map_circular() {
        map_panel = map_panel.child(minimap);
    }

    // Right: annotated sequence panel.
    let sd = SeqDraw::build(&this.app, this.editor_scroll);
    let e3 = cx.entity();
    let seq = div()
        .id("editor-seq")
        .w(px(SEQ_W))
        .h_full()
        .flex_shrink_0()
        .border_l_1()
        .border_color(theme::BORDER)
        .child(
            canvas(
                move |bounds, _w, cx| {
                    e3.update(cx, |t, _| t.editor_seq_bounds = Some(bounds));
                },
                move |bounds, _s, window, cx| paint_seq(&sd, bounds, window, cx),
            )
            .size_full(),
        )
        .on_mouse_down(MouseButton::Left, cx.listener(HelixBench::editor_seq_down))
        .on_mouse_move(cx.listener(HelixBench::editor_seq_move))
        .on_mouse_up(MouseButton::Left, cx.listener(HelixBench::editor_seq_up))
        .on_mouse_up_out(MouseButton::Left, cx.listener(HelixBench::editor_seq_up))
        .on_scroll_wheel(cx.listener(HelixBench::editor_seq_wheel));

    let cards = bottom_cards(this, cx);

    let left_body = if this.app.map_mode == MapMode::Metadata {
        metadata_panel(this)
    } else {
        map_panel.into_any_element()
    };
    let left = div()
        .flex()
        .flex_col()
        .flex_grow()
        .h_full()
        .overflow_hidden()
        .child(map_tabs(this, cx))
        .child(left_body);

    div()
        .flex()
        .flex_col()
        .size_full()
        .bg(theme::BG)
        .child(
            div()
                .flex()
                .flex_row()
                .flex_grow()
                .overflow_hidden()
                .child(left)
                .when(
                    this.app.shell.sequence_preview
                        && this.app.shell.layout(this.viewport.width.v()).content_width >= 900.0,
                    |d| d.child(seq),
                ),
        )
        .when(this.app.map_mode == MapMode::Metadata, |d| d.child(cards))
        .into_any_element()
}

fn map_tabs(this: &mut HelixBench, cx: &mut Context<HelixBench>) -> gpui::Div {
    let eff = if this.app.map_mode == MapMode::Metadata {
        MapMode::Metadata
    } else if this.app.map_circular() {
        MapMode::Plasmid
    } else {
        MapMode::Linear
    };
    let mut row = div()
        .flex()
        .flex_row()
        .gap(px(4.0))
        .px(px(10.0))
        .py(px(6.0))
        .bg(theme::PANEL)
        .border_b_1()
        .border_color(theme::BORDER);
    for (mode, label) in [
        (MapMode::Auto, "Auto view"),
        (MapMode::Plasmid, "Circular view"),
        (MapMode::Linear, "Linear view"),
        (MapMode::Metadata, "Metadata"),
    ] {
        let active = eff == mode;
        row = row.child(
            div()
                .id(gpui::SharedString::from(format!("map-tab-{label}")))
                .h(px(26.0))
                .px(px(12.0))
                .flex()
                .items_center()
                .rounded(px(6.0))
                .cursor_pointer()
                .text_size(px(theme::T_SM))
                .bg(if active {
                    theme::SURFACE_2
                } else {
                    theme::transparent()
                })
                .text_color(if active { theme::TEXT } else { theme::MUTED })
                .hover(|s| s.bg(theme::SURFACE))
                .child(label)
                .on_click(cx.listener(move |t, _, _, cx| {
                    appcore::shell::set_representation(&mut t.app, mode);
                    cx.notify();
                })),
        );
    }
    row = row.child(
        crate::widgets::button(
            "map-sequence-preview",
            if this.app.shell.sequence_preview {
                "Hide sequence preview"
            } else {
                "Sequence preview"
            },
            crate::widgets::Btn::Ghost,
        )
        .on_click(cx.listener(|t, _, _, cx| {
            if t.app.shell.layout(t.viewport.width.v()).content_width < 900.0 {
                t.app
                    .toast("Use the Sequence tab for a readable sequence at this width");
            } else {
                t.app.shell.sequence_preview = !t.app.shell.sequence_preview;
            }
            cx.notify();
        })),
    );
    row
}

fn mcard(title: &str) -> gpui::Div {
    div()
        .flex()
        .flex_col()
        .gap(px(6.0))
        .p(px(12.0))
        .bg(theme::PANEL)
        .border_1()
        .border_color(theme::BORDER)
        .rounded(px(8.0))
        .child(
            div()
                .text_size(px(theme::T_SM))
                .font_weight(gpui::FontWeight::SEMIBOLD)
                .text_color(theme::TEXT)
                .child(title.to_owned()),
        )
}

fn metadata_panel(this: &HelixBench) -> AnyElement {
    let d = &this.app.doc;
    let l = d.sequence.len();
    let overview = mcard("Overview")
        .child(kv("Name", d.name.clone()))
        .child(kv("Topology", d.sequence.topology.label().to_string()))
        .child(kv("Length", format!("{l} bp")))
        .child(kv("GC content", format!("{:.1}%", d.sequence.gc_content())))
        .child(kv("Genetic code", d.default_genetic_code.to_string()))
        .child(kv(
            "Description",
            if d.description.is_empty() {
                "—".into()
            } else {
                d.description.clone()
            },
        ));
    let contents = mcard("Contents")
        .child(kv("Features", d.features.len().to_string()))
        .child(kv("Primers", d.primers.len().to_string()))
        .child(kv("Enzymes (chosen)", d.chosen_enzymes.len().to_string()))
        .child(kv("ORFs", this.app.analysis.orfs.len().to_string()))
        .child(kv("Versions", d.versions.len().to_string()))
        .child(kv(
            "Tags",
            if d.tags.is_empty() {
                "—".into()
            } else {
                d.tags.join(", ")
            },
        ));
    div()
        .id("map-metadata")
        .flex_1()
        .min_h(px(0.0))
        .overflow_y_scroll()
        .p(px(12.0))
        .flex()
        .flex_col()
        .gap(px(10.0))
        .child(overview)
        .child(contents)
        .into_any_element()
}

fn card(title: &str) -> gpui::Div {
    div()
        .flex()
        .flex_col()
        .gap(px(6.0))
        .flex_grow()
        .h_full()
        .p(px(12.0))
        .bg(theme::PANEL)
        .border_1()
        .border_color(theme::BORDER)
        .rounded(px(8.0))
        .overflow_hidden()
        .child(
            div()
                .text_size(px(theme::T_SM))
                .text_color(theme::TEXT)
                .child(title.to_owned()),
        )
}

fn kv(k: &str, v: String) -> gpui::Div {
    div()
        .flex()
        .flex_row()
        .justify_between()
        .gap(px(8.0))
        .child(
            div()
                .text_size(px(theme::T_XS))
                .text_color(theme::FAINT)
                .child(k.to_owned()),
        )
        .child(
            div()
                .text_size(px(theme::T_XS))
                .text_color(theme::MUTED)
                .child(v),
        )
}

fn bottom_cards(this: &mut HelixBench, _cx: &mut Context<HelixBench>) -> AnyElement {
    let l = this.app.doc.sequence.len();
    let feat = this
        .app
        .selected_feature
        .and_then(|id| this.app.doc.features.iter().find(|f| f.id == id))
        .or_else(|| this.app.doc.features.first());

    let mut desc = card("Feature");
    if let Some(f) = feat {
        let body = if !f.note.is_empty() {
            f.note.clone()
        } else if !f.product.is_empty() {
            f.product.clone()
        } else {
            format!("{} annotation on {}.", f.kind, this.app.doc.name)
        };
        desc = desc
            .child(
                div()
                    .text_size(px(theme::T_SM))
                    .text_color(theme::ACCENT)
                    .child(f.name.clone()),
            )
            .child(
                div()
                    .flex_grow()
                    .overflow_hidden()
                    .text_size(px(theme::T_XS))
                    .text_color(theme::MUTED)
                    .child(body),
            );
    } else {
        desc = desc.child(
            div()
                .text_size(px(theme::T_XS))
                .text_color(theme::FAINT)
                .child("No features yet."),
        );
    }

    let mut props = card("Properties");
    if let Some(f) = feat {
        let r = f.span(l);
        let s = r.start;
        let e = r.start + r.len;
        let len = e.saturating_sub(s);
        let gc = if len > 0 {
            let slice = &this.app.doc.sequence.bases[s.min(l)..e.min(l)];
            let gc = slice.iter().filter(|b| matches!(b, b'G' | b'C')).count();
            gc as f32 / len as f32 * 100.0
        } else {
            0.0
        };
        let strand = if matches!(f.direction, domain::Direction::Forward) {
            "+1"
        } else {
            "-1"
        };
        props = props
            .child(kv("Type", f.kind.clone()))
            .child(kv("Range", format!("{}..{}", s + 1, e)))
            .child(kv("Length", format!("{len} bp")))
            .child(kv("Strand", strand.to_owned()))
            .child(kv("GC", format!("{gc:.1}%")));
    }

    let mut legend = card("Legend");
    let mut seen = std::collections::HashSet::new();
    let mut row = div()
        .flex()
        .flex_col()
        .gap(px(4.0))
        .flex_grow()
        .overflow_hidden();
    for f in &this.app.doc.features {
        if !f.visible || !seen.insert(f.name.clone()) {
            continue;
        }
        row = row.child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .gap(px(6.0))
                .child(
                    div()
                        .w(px(10.0))
                        .h(px(10.0))
                        .rounded(px(2.0))
                        .bg(theme::pastel(f.color)),
                )
                .child(
                    div()
                        .text_size(px(theme::T_XS))
                        .text_color(theme::MUTED)
                        .child(f.name.clone()),
                ),
        );
    }
    legend = legend.child(row);

    div()
        .flex()
        .flex_row()
        .gap(px(10.0))
        .h(px(150.0))
        .flex_shrink_0()
        .p(px(10.0))
        .bg(theme::BG)
        .child(desc)
        .child(props)
        .child(legend)
        .into_any_element()
}

impl HelixBench {
    fn seq_index_at(&self, pos: Point<Pixels>) -> Option<usize> {
        let b = self.editor_seq_bounds?;
        let l = self.app.doc.sequence.len();
        if l == 0 {
            return None;
        }
        let n = bases_per_row(b.size.width.v());
        let rows = l.div_ceil(n);
        let content_h = rows as f32 * ROW_H;
        let max_scroll = (content_h - b.size.height.v()).max(0.0);
        let scroll = self.editor_scroll.clamp(0.0, max_scroll);
        let row = ((pos.y.v() - b.origin.y.v() + scroll) / ROW_H).floor() as isize;
        let col = ((pos.x.v() - b.origin.x.v() - PAD) / CHAR_W).floor() as isize;
        if row < 0 || col < 0 || col >= n as isize {
            return None;
        }
        let idx = row as usize * n + col as usize;
        if idx < l {
            Some(idx)
        } else {
            None
        }
    }

    pub fn editor_seq_down(
        &mut self,
        ev: &MouseDownEvent,
        _w: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(i) = self.seq_index_at(ev.position) {
            self.editor_seq_anchor = Some(i);
            let source = self.app.doc.source_ref();
            let _ = appcore::selection::apply(
                &mut self.app,
                source,
                Some(Range::new(i, 1)),
                domain::Strand::Top,
            );
            cx.notify();
        }
    }

    pub fn editor_seq_move(
        &mut self,
        ev: &MouseMoveEvent,
        _w: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(anchor) = self.editor_seq_anchor else {
            return;
        };
        if let Some(cur) = self.seq_index_at(ev.position) {
            let (a, b) = (anchor.min(cur), anchor.max(cur));
            let source = self.app.doc.source_ref();
            let _ = appcore::selection::apply(
                &mut self.app,
                source,
                Some(Range::new(a, b - a + 1)),
                domain::Strand::Top,
            );
            cx.notify();
        }
    }

    pub fn editor_seq_up(&mut self, _ev: &MouseUpEvent, _w: &mut Window, _cx: &mut Context<Self>) {
        self.editor_seq_anchor = None;
    }

    pub fn editor_seq_wheel(
        &mut self,
        ev: &ScrollWheelEvent,
        _w: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let delta = ev.delta.pixel_delta(px(ROW_H));
        self.editor_scroll = (self.editor_scroll - delta.y.v()).max(0.0);
        cx.notify();
    }

    fn slider_set(&mut self, pos: Point<Pixels>, cx: &mut Context<Self>) {
        let Some(b) = self.editor_slider_bounds else {
            return;
        };
        let l = self.app.doc.sequence.len().max(1);
        let frac = ((pos.y.v() - b.origin.y.v()) / b.size.height.v()).clamp(0.0, 1.0);
        self.app.zoom_width = ((1.0 - frac) * l as f32).clamp(20.min(l) as f32, l as f32) as usize;
        self.app.center_zoom_on_selection();
        cx.notify();
    }

    pub fn editor_slider_down(
        &mut self,
        ev: &MouseDownEvent,
        _w: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.slider_set(ev.position, cx);
    }

    pub fn editor_slider_move(
        &mut self,
        ev: &MouseMoveEvent,
        _w: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if ev.pressed_button == Some(MouseButton::Left) {
            self.slider_set(ev.position, cx);
        }
    }
}
