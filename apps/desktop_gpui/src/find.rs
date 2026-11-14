//! Find bar: DNA / protein / name search with live results, recents and
//! Enter / Shift+Enter navigation. Rendered under the tab strip.

use gpui::{
    actions, anchored, deferred, div, point, prelude::*, px, AnyElement, App, Context, Corner, Div,
    ElementId, Focusable, KeyBinding, MouseButton, SharedString, Stateful, Window,
};

use appcore::state::{Dialog, SearchMode};

use crate::app::HelixBench;
use crate::theme;
use crate::widgets::{self, Btn};

actions!(findbar, [Next, Prev]);

pub fn bind_keys(cx: &mut App) {
    cx.bind_keys([
        KeyBinding::new("enter", Next, Some("FindBar")),
        KeyBinding::new("shift-enter", Prev, Some("FindBar")),
    ]);
}

fn mode_btn(id: impl Into<ElementId>, label: &str, active: bool) -> Stateful<Div> {
    let d = div()
        .id(id)
        .h(px(26.0))
        .px(px(11.0))
        .flex()
        .items_center()
        .rounded(px(5.0))
        .text_size(px(theme::T_SM))
        .cursor_pointer();
    if active {
        d.bg(theme::SURFACE_2)
            .border_1()
            .border_color(theme::ACCENT.opacity(0.7))
            .text_color(theme::ACCENT)
    } else {
        d.text_color(theme::MUTED)
            .hover(|s| s.bg(theme::SURFACE).text_color(theme::TEXT))
    }
    .child(label.to_owned())
}

pub fn show(app: &mut HelixBench, window: &mut Window, cx: &mut Context<HelixBench>) -> AnyElement {
    // Pull the query text and re-run search live as it changes.
    let seed = app.app.search.query.clone();
    let query = app.input_entity(cx, "find:query", &seed, "Search sequence or name…", true);
    let text = query.read(cx).text();
    if text != app.app.search.query {
        app.app.search.query = text;
        app.app.run_search();
        if app.app.search.hit_count() > 0 {
            app.app.goto_current_hit();
        }
    }
    if app.app.search.focus_request {
        let fh = query.read(cx).focus_handle(cx);
        fh.focus(window);
        app.app.search.focus_request = false;
    }

    let mode = app.app.search.mode;
    let n = app.app.search.hit_count();

    let mut bar = div()
        .key_context("FindBar")
        .flex()
        .items_center()
        .gap(px(8.0))
        .px(px(12.0))
        .py(px(8.0))
        .bg(theme::PANEL)
        .border_b_1()
        .border_color(theme::BORDER)
        .on_action(cx.listener(|t, _: &Next, _, cx| {
            t.app.search_next(true);
            cx.notify();
        }))
        .on_action(cx.listener(|t, _: &Prev, _, cx| {
            t.app.search_next(false);
            cx.notify();
        }))
        .child(widgets::label("Find", theme::T_SM, theme::FAINT));

    // Mode segmented control.
    let mut seg = div()
        .flex()
        .items_center()
        .gap(px(2.0))
        .p(px(2.0))
        .rounded(px(7.0))
        .bg(theme::BG)
        .border_1()
        .border_color(theme::BORDER);
    for (m, label) in [
        (SearchMode::Dna, "DNA"),
        (SearchMode::Protein, "Protein"),
        (SearchMode::Name, "Name"),
    ] {
        seg = seg.child(
            mode_btn(
                SharedString::from(format!("find-mode-{}", m.label())),
                label,
                mode == m,
            )
            .on_click(cx.listener(move |t, _, _, cx| {
                if t.app.search.mode != m {
                    t.app.search.mode = m;
                    t.app.run_search();
                    if t.app.search.hit_count() > 0 {
                        t.app.goto_current_hit();
                    }
                }
                cx.notify();
            })),
        );
    }
    bar = bar.child(seg);
    bar = bar.child(div().w(px(280.0)).child(query.clone()));

    // Recents dropdown.
    bar = bar.child(
        widgets::button("find-recents", "Recent ▾", Btn::Subtle).on_mouse_down(
            MouseButton::Left,
            cx.listener(|t, ev: &gpui::MouseDownEvent, _, cx| {
                t.find_recents_open = !t.find_recents_open;
                t.find_recents_anchor = ev.position;
                cx.notify();
            }),
        ),
    );

    // Prev / next.
    let mut prev = widgets::icon_only("find-prev", "chevron-left", Btn::Outline)
        .opacity(if n > 0 { 1.0 } else { 0.4 });
    if n > 0 {
        prev = prev.on_click(cx.listener(|t, _, _, cx| {
            t.app.search_next(false);
            cx.notify();
        }));
    }
    bar = bar.child(prev);
    let mut next = widgets::icon_only("find-next", "chevron-right", Btn::Outline)
        .opacity(if n > 0 { 1.0 } else { 0.4 });
    if n > 0 {
        next = next.on_click(cx.listener(|t, _, _, cx| {
            t.app.search_next(true);
            cx.notify();
        }));
    }
    bar = bar.child(next);

    bar = bar.child(status_label(&app.app, n));
    bar = bar.child(div().flex_grow());
    bar = bar.child(
        widgets::button("find-tips", "?", Btn::Ghost).on_click(cx.listener(|t, _, _, cx| {
            t.app.dialog = Some(Dialog::SearchTips);
            cx.notify();
        })),
    );
    bar = bar.child(
        widgets::icon_only("find-close", "x", Btn::Ghost).on_click(cx.listener(|t, _, _, cx| {
            t.app.search.open = false;
            t.find_recents_open = false;
            cx.notify();
        })),
    );

    if let Some(popup) = recents_popup(app, cx) {
        bar = bar.child(popup);
    }
    bar.into_any_element()
}

fn status_label(app: &appcore::state::AppState, n: usize) -> Div {
    if let Some(e) = &app.search.error {
        return div()
            .flex()
            .items_center()
            .gap(px(6.0))
            .child(crate::icons::icon("alert", 13.0, theme::DANGER))
            .child(widgets::label(e, theme::T_SM, theme::DANGER));
    }
    if app.search.query.trim().is_empty() {
        return widgets::label("Type to search both strands", theme::T_SM, theme::FAINT);
    }
    if n == 0 {
        return widgets::label("No matches", theme::T_SM, theme::WARN);
    }
    let mut row = div().flex().items_center().gap(px(8.0)).child(
        div()
            .font_family(theme::MONO_FONT)
            .font_weight(gpui::FontWeight::SEMIBOLD)
            .text_size(px(theme::T_SM))
            .text_color(theme::ACCENT)
            .child(format!("{} of {}", app.search.current + 1, n)),
    );
    if let Some((start, len, strand)) = current_hit_locus(app) {
        let txt = if len == 1 {
            format!("at {}", start + 1)
        } else {
            format!("at {}..{}", start + 1, start + len)
        };
        row = row.child(
            div()
                .font_family(theme::MONO_FONT)
                .text_size(px(theme::T_XS))
                .text_color(theme::MUTED)
                .child(format!("{txt} · {strand}")),
        );
    }
    row
}

fn current_hit_locus(app: &appcore::state::AppState) -> Option<(usize, usize, &'static str)> {
    let strand = |s: domain::Strand| {
        if s == domain::Strand::Top {
            "top strand"
        } else {
            "bottom strand"
        }
    };
    match app.search.mode {
        SearchMode::Dna => app
            .search
            .dna_hits
            .get(app.search.current)
            .map(|h| (h.start, h.len, strand(h.strand))),
        SearchMode::Protein => app
            .search
            .protein_hits
            .get(app.search.current)
            .map(|h| (h.start, h.len, strand(h.strand))),
        SearchMode::Name => None,
    }
}

fn recents_popup(app: &mut HelixBench, cx: &mut Context<HelixBench>) -> Option<AnyElement> {
    if !app.find_recents_open {
        return None;
    }
    let mut card = div()
        .id("find-recents-card")
        .flex()
        .flex_col()
        .min_w(px(260.0))
        .max_w(px(360.0))
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
        .on_mouse_down_out(cx.listener(|t, _, _, cx| {
            t.find_recents_open = false;
            cx.notify();
        }));
    if app.app.search.recents.is_empty() {
        card = card.child(
            widgets::label("No recent searches", theme::T_SM, theme::FAINT)
                .px(px(10.0))
                .py(px(4.0)),
        );
    } else {
        let recents = app.app.search.recents.clone();
        for (i, (m, q)) in recents.into_iter().enumerate() {
            let tag = match m {
                SearchMode::Dna => "DNA",
                SearchMode::Protein => "AA",
                SearchMode::Name => "Name",
            };
            let label = format!("[{tag}]  {q}");
            card = card.child(
                widgets::menu_row(SharedString::from(format!("recent-{i}")), &label, "", true)
                    .on_click(cx.listener(move |t, _, _, cx| {
                        t.find_recents_open = false;
                        t.app.search.mode = m;
                        t.app.search.query = q.clone();
                        t.set_input(cx, "find:query", q.clone());
                        t.app.run_search();
                        if t.app.search.hit_count() > 0 {
                            t.app.goto_current_hit();
                        }
                        cx.notify();
                    })),
            );
        }
    }
    Some(
        deferred(
            anchored()
                .anchor(Corner::TopLeft)
                .position(app.find_recents_anchor)
                .offset(point(px(-4.0), px(8.0)))
                .snap_to_window()
                .child(card),
        )
        .with_priority(46)
        .into_any_element(),
    )
}
