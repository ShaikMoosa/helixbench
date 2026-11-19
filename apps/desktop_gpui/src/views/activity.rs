//! Truthful local recovery surface; no invented background-job progress.
use crate::{
    app::HelixBench,
    theme,
    widgets::{self, Btn},
};
use gpui::{div, prelude::*, px, AnyElement, Context};
pub fn render(app: &mut HelixBench, cx: &mut Context<HelixBench>) -> AnyElement {
    let mut body = div()
        .id("activity-page")
        .size_full()
        .overflow_y_scroll()
        .flex()
        .flex_col()
        .gap(px(16.0))
        .p(px(theme::PAGE_PAD));
    body = body.child(
        widgets::button("activity-refresh", "Refresh local recovery", Btn::Outline).on_click(
            cx.listener(|t, _, _, cx| {
                t.app.recovery = t.app.operation_journal().recovery();
                cx.notify();
            }),
        ),
    );
    if app.app.recovery.items.is_empty()
        && app.app.recovery.errors.is_empty()
        && app.app.operation_error.is_none()
    {
        body = body.child(widgets::empty_state(
            "check",
            "No unresolved local operations",
            "Save and export failures will remain discoverable here.",
        ));
    } else {
        body = body.child(widgets::section_title("Needs attention"));
        for item in &app.app.recovery.items {
            body = body.child(widgets::row_card().child(widgets::label(
                &item.destination.display().to_string(),
                theme::T_BASE,
                theme::TEXT,
            )));
        }
        for error in &app.app.recovery.errors {
            body = body.child(widgets::label(error, theme::T_BASE, theme::DANGER));
        }
        body=body.child(widgets::label("Use the recovery controls above to inspect and reconcile the retained output. Completed writes are not claimed to be rolled back.",theme::T_BASE,theme::MUTED));
    }
    body.child(widgets::label(
        "This view shows journaled local recovery. Live background scheduling is not available.",
        theme::T_SM,
        theme::MUTED,
    ))
    .into_any_element()
}
