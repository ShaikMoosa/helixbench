//! Open-document catalog, always bound to stable document identities.
use crate::{
    app::HelixBench,
    draw::PxVal,
    theme,
    widgets::{self, Btn},
};
use gpui::{div, prelude::*, px, AnyElement, Context, SharedString};

pub fn render(app: &mut HelixBench, cx: &mut Context<HelixBench>) -> AnyElement {
    let width = (app
        .app
        .shell
        .layout_for(app.app.view, app.viewport.width.v())
        .content_width
        - 2.0)
        .max(500.0);
    let input = app.input_entity(
        cx,
        "sequences:query",
        &app.app.shell.sequence_query.clone(),
        "Search open sequences by name or path",
        false,
    );
    app.app.shell.sequence_query = input.read(cx).text();
    let rows = appcore::workspace::sequences(&app.app, &app.app.shell.sequence_query);
    let mut controls = div()
        .flex()
        .items_center()
        .gap(px(12.0))
        .child(div().flex_1().child(input));
    if !app.app.shell.sequence_query.is_empty() {
        controls = controls.child(
            widgets::button("sequence-query-clear", "Clear search", Btn::Outline).on_click(
                cx.listener(|t, _, _, cx| {
                    t.app.shell.sequence_query.clear();
                    t.inputs.remove("sequences:query");
                    cx.notify();
                }),
            ),
        );
    }
    let mut body = div()
        .id("sequence-catalog")
        .size_full()
        .overflow_y_scroll()
        .flex()
        .flex_col()
        .gap(px(20.0))
        .px(px(theme::PAGE_PAD))
        .py(px(12.0))
        .child(controls);
    let widths = [width * 0.34, width * 0.17, width * 0.24, width * 0.25];
    let mut table = div()
        .flex()
        .flex_col()
        .bg(theme::PANEL)
        .rounded(px(12.0))
        .border_1()
        .border_color(theme::BORDER)
        .shadow(theme::shadow_card())
        .overflow_hidden();
    let mut head = div().flex().items_center().h(px(44.0)).bg(theme::BG);
    for (label, w) in ["Sequence", "Length / topology", "Location", "State"]
        .iter()
        .zip(widths)
    {
        head = head.child(widgets::catalog_cell(label, w, true));
    }
    table = table.child(head);
    for item in &rows {
        let id = item.id;
        let location = item
            .path
            .as_ref()
            .map(|p| p.display().to_string())
            .unwrap_or_else(|| "Open document · no saved path".into());
        let row = div()
            .id(SharedString::from(format!("sequence-row-{id}")))
            .flex()
            .items_center()
            .h(px(64.0))
            .border_t_1()
            .border_color(theme::BORDER)
            .cursor_pointer()
            .hover(|s| s.bg(theme::MINT))
            .child(widgets::catalog_cell(
                &format!(
                    "{}{}",
                    item.name,
                    if item.active { " · active" } else { "" }
                ),
                widths[0],
                false,
            ))
            .child(widgets::catalog_cell(
                &format!(
                    "{} bp · {}",
                    item.length,
                    if item.circular { "Circular" } else { "Linear" }
                ),
                widths[1],
                true,
            ))
            .child(widgets::catalog_cell(&location, widths[2], true))
            .child(
                div()
                    .w(px(widths[3]))
                    .px(px(12.0))
                    .child(widgets::status_badge(
                        item.persistence(),
                        if item.persistence() == "Saved locally" {
                            theme::OK
                        } else {
                            theme::WARN
                        },
                        if item.persistence() == "Saved locally" {
                            theme::MINT
                        } else {
                            theme::WARN_SURFACE
                        },
                    )),
            )
            .on_click(cx.listener(move |t, _, _, cx| {
                if let Err(e) = appcore::workspace::open_sequence(&mut t.app, id) {
                    t.app.toast(e);
                }
                cx.notify();
            }));
        table = table.child(row);
    }
    body = body.child(table);
    if rows.is_empty() {
        body = body.child(widgets::empty_state(
            "sequence",
            if app.app.shell.sequence_query.is_empty() {
                "No open sequences yet"
            } else {
                "No sequences match this search"
            },
            if app.app.shell.sequence_query.is_empty() {
                "Import a file or create a sequence to start."
            } else {
                "Clear the search to see your open documents."
            },
        ));
    }
    body.child(widgets::label("Each row opens the exact document and preserves its selection, edits and undo history. Import opens local files after review.",theme::T_SM,theme::MUTED)).into_any_element()
}
