//! View modules plus shared table chrome: a palette-matched scrollbar drawn as
//! a `uniform_list` decoration, sortable headers, body cells and sort cycling.

pub mod enzymes;
pub mod features;
pub mod history;
pub mod map;
pub mod primers;
pub mod results;
pub mod sequence;
pub mod table_opts;

use std::ops::Range;

use gpui::{
    canvas, div, point, prelude::*, px, size, AnyElement, App, Bounds, Div, ElementId, Hsla,
    Pixels, Stateful, UniformListDecoration, Window,
};

use appcore::state::{SortDir, TableSort};

use crate::draw::{self, PxVal};
use crate::theme;

/// A vertical scrollbar painted in the artifact's own visual language, sized
/// from the list's viewport / content ratio. Drawn on top of the rows.
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

/// gpui 0.2.2 caches the first text measurement, so `.truncate()` never
/// ellipsizes text laid out inside flex rows. Fit the string ourselves.
pub fn ellipsize(window: &mut Window, text: &str, max_w: f32, size: f32, mono: bool) -> String {
    let font = gpui::font(if mono {
        theme::mono_family()
    } else {
        theme::ui_family()
    });
    if crate::draw::measure(window, text, size, font.clone()).v() <= max_w {
        return text.to_owned();
    }
    let ell = crate::draw::measure(window, "…", size, font.clone()).v();
    let chars: Vec<char> = text.chars().collect();
    let (mut lo, mut hi) = (0usize, chars.len());
    while lo < hi {
        let mid = (lo + hi).div_ceil(2);
        let prefix: String = chars[..mid].iter().collect();
        if crate::draw::measure(window, &prefix, size, font.clone()).v() + ell <= max_w {
            lo = mid;
        } else {
            hi = mid - 1;
        }
    }
    let prefix: String = chars[..lo].iter().collect();
    format!("{prefix}…")
}

/// A body cell with a definite width and ellipsized text.
pub fn cell(window: &mut Window, text: &str, w: f32, color: Hsla, mono: bool) -> Div {
    let fitted = ellipsize(window, text, w - 16.0, theme::T_SM, mono);
    let mut d = div()
        .h_full()
        .w(px(w))
        .flex_shrink_0()
        .flex()
        .items_center()
        .px(px(8.0))
        .text_size(px(theme::T_SM))
        .text_color(color)
        .overflow_hidden()
        .child(fitted);
    if mono {
        d = d.font_family(theme::mono_family());
    }
    d
}

/// A non-sortable column header label.
pub fn header_label(window: &mut Window, text: &str, w: f32) -> Div {
    let fitted = ellipsize(window, text, w - 16.0, theme::T_XS, false);
    div()
        .h_full()
        .w(px(w))
        .flex_shrink_0()
        .flex()
        .items_center()
        .px(px(8.0))
        .text_size(px(theme::T_XS))
        .font_weight(gpui::FontWeight::SEMIBOLD)
        .text_color(theme::FAINT)
        .overflow_hidden()
        .child(fitted)
}

/// A sortable column header. The caller attaches the `on_click` that cycles the
/// appropriate [`TableSort`]; this only renders label + arrow + active state.
pub fn sort_header(
    window: &mut Window,
    id: impl Into<ElementId>,
    label: &str,
    w: f32,
    col: usize,
    sort: TableSort,
) -> Stateful<Div> {
    let active = sort.column == col && sort.dir.is_some();
    let arrow = if active {
        if sort.dir == Some(SortDir::Asc) {
            "  ↑"
        } else {
            "  ↓"
        }
    } else {
        ""
    };
    let fitted = ellipsize(
        window,
        &format!("{label}{arrow}"),
        w - 16.0,
        theme::T_XS,
        false,
    );
    div()
        .id(id)
        .h_full()
        .w(px(w))
        .flex_shrink_0()
        .flex()
        .items_center()
        .px(px(8.0))
        .text_size(px(theme::T_XS))
        .font_weight(gpui::FontWeight::SEMIBOLD)
        .cursor_pointer()
        .overflow_hidden()
        .text_color(if active {
            theme::ACCENT_TEXT
        } else {
            theme::MUTED
        })
        .hover(|s| s.text_color(theme::TEXT))
        .child(fitted)
}

/// The leading ✓ / empty box used for visibility and set-membership toggles.
/// The caller wraps it in a fixed-width cell and attaches `on_click`.
pub fn check_box(id: impl Into<ElementId>, checked: bool) -> Stateful<Div> {
    div()
        .id(id)
        .w(px(16.0))
        .h(px(16.0))
        .flex()
        .items_center()
        .justify_center()
        .rounded(px(4.0))
        .cursor_pointer()
        .text_size(px(theme::T_XS))
        .border_1()
        .border_color(if checked {
            theme::ACCENT
        } else {
            theme::BORDER_STRONG
        })
        .bg(if checked {
            theme::ACCENT.opacity(0.18)
        } else {
            theme::transparent()
        })
        .text_color(theme::ACCENT_TEXT)
        .hover(|s| s.border_color(theme::ACCENT.opacity(0.7)))
        .child(if checked { "✓" } else { "" }.to_owned())
}

/// Striped row background with a teal-tinted selected state.
pub fn row_bg(selected: bool, idx: usize) -> Hsla {
    if selected {
        theme::ROW_SELECTED
    } else if idx % 2 == 1 {
        theme::SURFACE.opacity(0.5)
    } else {
        theme::transparent()
    }
}

/// Cycle a column's sort direction: Asc → Desc → unsorted.
pub fn cycle_sort(sort: &mut TableSort, col: usize) {
    if sort.column == col {
        sort.dir = match sort.dir {
            Some(SortDir::Asc) => Some(SortDir::Desc),
            Some(SortDir::Desc) => None,
            None => Some(SortDir::Asc),
        };
    } else {
        sort.column = col;
        sort.dir = Some(SortDir::Asc);
    }
}

/// The shared table scaffold: toolbar slot, header row and a scrollable body.
pub fn table_scaffold(toolbar: AnyElement, header: AnyElement, body: AnyElement) -> Div {
    div()
        .size_full()
        .flex()
        .flex_col()
        .bg(theme::BG)
        .child(toolbar)
        .child(
            div()
                .h(px(30.0))
                .flex()
                .items_center()
                .bg(theme::PANEL)
                .border_b_1()
                .border_color(theme::BORDER)
                .child(header),
        )
        .child(div().flex_1().min_h(px(0.0)).overflow_hidden().child(body))
}
