//! Shared column-config + saved-views popup for the list tables (Features,
//! Primers, Enzymes). Columns toggle visibility; saved views capture column
//! config + sort + filter and can be reloaded or deleted.

use gpui::{
    anchored, deferred, div, point, prelude::*, px, AnyElement, Context, Corner, Div, ElementId,
    MouseButton, SharedString, Stateful,
};

use appcore::actions as act;

use crate::app::HelixBench;
use crate::theme;
use crate::widgets::{self, Btn};

pub fn opts_button(id: impl Into<ElementId>, open: bool) -> Stateful<Div> {
    div()
        .id(id)
        .flex()
        .items_center()
        .gap(px(6.0))
        .h(px(28.0))
        .px(px(10.0))
        .rounded(px(6.0))
        .border_1()
        .border_color(if open {
            theme::ACCENT
        } else {
            theme::BORDER_STRONG
        })
        .bg(theme::SURFACE)
        .text_size(px(theme::T_SM))
        .text_color(theme::TEXT)
        .cursor_pointer()
        .hover(|s| s.bg(theme::SURFACE_2))
        .child(crate::icons::icon("sliders", 14.0, theme::MUTED))
        .child("Columns / Views")
}

/// Toggle the options popup for a table (anchor at the click position).
pub fn toggle_opts(
    t: &mut HelixBench,
    table: &str,
    ev: &gpui::MouseDownEvent,
    cx: &mut Context<HelixBench>,
) {
    if t.opts.as_deref() == Some(table) {
        t.opts = None;
    } else {
        t.opts = Some(table.to_string());
        t.opts_anchor = ev.position;
    }
    cx.notify();
}

pub fn render_table_opts(
    app: &mut HelixBench,
    cx: &mut Context<HelixBench>,
    table: &'static str,
    columns: &[(usize, &'static str)],
) -> Option<AnyElement> {
    if app.opts.as_deref() != Some(table) {
        return None;
    }
    let hidden = app.app.hidden_cols(table);
    let mut card = div()
        .id("table-opts")
        .flex()
        .flex_col()
        .min_w(px(230.0))
        .max_w(px(320.0))
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
            t.opts = None;
            cx.notify();
        }));
    card = card.child(widgets::menu_section("Columns"));
    for (idx, label) in columns {
        let idx = *idx;
        let shown = !hidden.contains(&idx);
        card = card.child(
            widgets::menu_check(
                SharedString::from(format!("opts-col-{table}-{idx}")),
                label,
                shown,
            )
            .on_click(cx.listener(move |t, _, _, cx| {
                act::table_toggle_column(&mut t.app, table, idx);
                cx.notify();
            })),
        );
    }
    card = card.child(widgets::divider());
    card = card.child(widgets::menu_section("Saved views"));
    let name_key = format!("opts:{table}:name");
    let name_inp = app.input_entity(cx, &name_key, "", "View name", false);
    let save_key = name_key.clone();
    card = card.child(
        div()
            .flex()
            .items_center()
            .gap(px(6.0))
            .px(px(10.0))
            .child(div().flex_1().child(name_inp))
            .child(
                widgets::button(
                    SharedString::from(format!("opts-save-{table}")),
                    "Save",
                    Btn::Subtle,
                )
                .on_click(cx.listener(move |t, _, _, cx| {
                    let name = t
                        .input_entity(cx, &save_key, "", "View name", false)
                        .read(cx)
                        .text();
                    act::view_save(&mut t.app, table, &name);
                    cx.notify();
                })),
            ),
    );
    let views: Vec<storage::views::SavedView> = app
        .app
        .views
        .for_table(table)
        .into_iter()
        .cloned()
        .collect();
    if views.is_empty() {
        card = card.child(
            div()
                .px(px(10.0))
                .py(px(4.0))
                .text_size(px(theme::T_XS))
                .text_color(theme::FAINT)
                .child("No saved views yet."),
        );
    }
    for v in views {
        let id = v.id;
        card = card.child(
            div()
                .flex()
                .items_center()
                .gap(px(6.0))
                .px(px(10.0))
                .py(px(3.0))
                .child(
                    div()
                        .flex_1()
                        .text_size(px(theme::T_SM))
                        .text_color(theme::TEXT)
                        .child(v.name.clone()),
                )
                .child(
                    widgets::button(
                        SharedString::from(format!("opts-load-{id}")),
                        "Load",
                        Btn::Subtle,
                    )
                    .on_click(cx.listener(move |t, _, _, cx| {
                        act::view_load(&mut t.app, id);
                        t.opts = None;
                        cx.notify();
                    })),
                )
                .child(
                    widgets::button(
                        SharedString::from(format!("opts-del-{id}")),
                        "Delete",
                        Btn::Ghost,
                    )
                    .on_click(cx.listener(move |t, _, _, cx| {
                        act::view_delete(&mut t.app, id);
                        cx.notify();
                    })),
                ),
        );
    }
    Some(
        deferred(
            anchored()
                .anchor(Corner::TopLeft)
                .position(app.opts_anchor)
                .offset(point(px(-4.0), px(8.0)))
                .snap_to_window()
                .child(card),
        )
        .with_priority(45)
        .into_any_element(),
    )
}
