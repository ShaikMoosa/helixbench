//! Edit history and persistent checkpoints have separate, explicit actions.
use crate::{
    app::HelixBench,
    theme,
    widgets::{self, Btn},
};
use appcore::{
    history_workflow as flow,
    state::{ColorsMode, Dialog},
};
use gpui::{div, prelude::*, px, AnyElement, Context, SharedString, Window};
impl HelixBench {
    pub fn render_history(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        let source = self.app.doc.source_ref();
        let mut toolbar = div().flex().flex_wrap().items_center().gap(px(8.0));
        let undo_label = self
            .app
            .history
            .last()
            .map(|e| format!("Undo: {}", e.label))
            .unwrap_or_else(|| "Undo — no earlier edit".into());
        let redo_label = self
            .app
            .future
            .last()
            .map(|e| format!("Redo: {}", e.label))
            .unwrap_or_else(|| "Redo — no later edit".into());
        let undo = widgets::button("hist-undo", &undo_label, Btn::Outline)
            .opacity(if self.app.history.is_empty() {
                0.4
            } else {
                1.0
            })
            .on_click(cx.listener(move |t, _, _, cx| {
                if flow_source(&mut t.app, source) {
                    t.app.undo();
                }
                cx.notify();
            }));
        let redo = widgets::button("hist-redo", &redo_label, Btn::Outline)
            .opacity(if self.app.future.is_empty() { 0.4 } else { 1.0 })
            .on_click(cx.listener(move |t, _, _, cx| {
                if flow_source(&mut t.app, source) {
                    t.app.redo();
                }
                cx.notify();
            }));
        toolbar = toolbar.child(undo).child(redo).child(
            widgets::button("hist-savever", "Name checkpoint", Btn::Primary).on_click(cx.listener(
                move |t, _, _, cx| {
                    if flow_source(&mut t.app, source) {
                        t.app.dialog = Some(Dialog::Checkpoint(flow::CheckpointDraft {
                            source,
                            label: String::new(),
                            error: None,
                        }));
                    }
                    cx.notify();
                },
            )),
        );
        let mut page=div().id("history-page").size_full().overflow_y_scroll().flex().flex_col().gap(px(16.0)).p(px(12.0)).bg(theme::BG).child(toolbar)
 .child(widgets::label("Named checkpoints",theme::T_LG,theme::TEXT))
 .child(widgets::label("Checkpoints keep a complete snapshot. Preview compares it with the current document; restore creates a new revision with Undo available. Save the document to retain checkpoints on disk.",theme::T_SM,theme::MUTED));
        if self.app.doc.versions.is_empty() {
            page = page.child(widgets::label(
                "No named checkpoints yet. Name the current state before making a change.",
                theme::T_SM,
                theme::MUTED,
            ));
        }
        for (i, v) in self.app.doc.versions.iter().enumerate().rev() {
            let checkpoint = v.doc.source_ref();
            let captured = v.clone();
            let persisted = self.app.navigation.checkpoint_saved(v);
            let label = format!(
                "{} · {} bp · {} features · {} primers",
                v.label,
                v.doc.sequence.len(),
                v.doc.features.len(),
                v.doc.primers.len()
            );
            let row = div()
                .flex()
                .flex_wrap()
                .items_center()
                .gap(px(8.0))
                .p(px(12.0))
                .bg(theme::PANEL)
                .rounded(px(12.0))
                .child(
                    div()
                        .flex_1()
                        .min_w(px(240.0))
                        .flex()
                        .flex_col()
                        .gap(px(4.0))
                        .child(widgets::label(&label, theme::T_BASE, theme::TEXT))
                        .child(widgets::label(
                            &format!("Created {}", crate::util::timestamp_utc(v.created)),
                            theme::T_SM,
                            theme::MUTED,
                        ))
                        .child(widgets::label(
                            &format!(
                                "Revision {} · {}",
                                checkpoint.revision,
                                if persisted {
                                    "Saved in document"
                                } else {
                                    "Local checkpoint — save document"
                                }
                            ),
                            theme::T_XS,
                            theme::MUTED,
                        )),
                )
                .child(
                    widgets::button(
                        SharedString::from(format!("hist-compare-{i}")),
                        "Compare / restore",
                        Btn::Outline,
                    )
                    .on_click(cx.listener(move |t, _, _, cx| {
                        match flow::preview_checkpoint(&t.app, source, &captured) {
                            Ok(r) => t.app.dialog = Some(Dialog::HistoryReview(Box::new(r))),
                            Err(e) => t.app.toast(e),
                        }
                        cx.notify();
                    })),
                );
            page = page.child(row);
        }
        page=page.child(widgets::label("Edit Undo / Redo",theme::T_LG,theme::TEXT)).child(widgets::label("This session’s edits are temporary history. Highlighting an operation changes no data. Review the state before an operation before restoring it.",theme::T_SM,theme::MUTED));
        if self.app.history.is_empty() {
            page=page.child(widgets::label("No edits in this session. Named checkpoints above remain available after reopening.",theme::T_SM,theme::MUTED));
        }
        for (i, e) in self.app.history.iter().enumerate().rev() {
            let revision = e.snapshot.source_ref();
            let label = e.label.clone();
            let ranges = e
                .regions
                .iter()
                .map(|r| format!("{}..{}", r.range.start + 1, r.range.end()))
                .collect::<Vec<_>>()
                .join(", ");
            page = page.child(
                div()
                    .flex()
                    .flex_wrap()
                    .items_center()
                    .gap(px(8.0))
                    .p(px(10.0))
                    .bg(if self.app.history_highlight == Some(i) {
                        theme::SURFACE_2
                    } else {
                        theme::PANEL
                    })
                    .rounded(px(12.0))
                    .child(
                        div()
                            .flex_1()
                            .min_w(px(190.0))
                            .child(widgets::label(&label, theme::T_BASE, theme::TEXT))
                            .child(widgets::label(
                                if ranges.is_empty() {
                                    "Whole document"
                                } else {
                                    &ranges
                                },
                                theme::T_XS,
                                theme::MUTED,
                            )),
                    )
                    .child(
                        widgets::button(
                            SharedString::from(format!("hist-highlight-{i}")),
                            "Highlight regions",
                            Btn::Ghost,
                        )
                        .on_click(cx.listener(move |t, _, _, cx| {
                            if flow_source(&mut t.app, source)
                                && t.app
                                    .history
                                    .get(i)
                                    .is_some_and(|e| e.snapshot.source_ref() == revision)
                            {
                                t.app.history_highlight = Some(i);
                                t.app.colors_mode = ColorsMode::History;
                            }
                            cx.notify();
                        })),
                    )
                    .child(
                        widgets::button(
                            SharedString::from(format!("hist-before-{i}")),
                            "Review before edit",
                            Btn::Outline,
                        )
                        .on_click(cx.listener(move |t, _, _, cx| {
                            match flow::preview_edit(&t.app, source, i, revision) {
                                Ok(r) => t.app.dialog = Some(Dialog::HistoryReview(Box::new(r))),
                                Err(e) => t.app.toast(e),
                            }
                            cx.notify();
                        })),
                    ),
            );
        }
        page = page.child(
            widgets::button("hist-current", "Show current state", Btn::Ghost).on_click(
                cx.listener(move |t, _, _, cx| {
                    if flow_source(&mut t.app, source) {
                        t.app.history_highlight = None;
                    }
                    cx.notify();
                }),
            ),
        );
        page.into_any_element()
    }
}
fn flow_source(app: &mut appcore::state::AppState, source: domain::SourceRef) -> bool {
    match appcore::enzyme_workflow::validate_source(app, source) {
        Ok(()) => true,
        Err(e) => {
            app.toast(e);
            false
        }
    }
}
