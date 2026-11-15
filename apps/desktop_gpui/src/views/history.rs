//! History view: the operation log with history colors and revert. Newest first,
//! a "current state" marker on top; click to highlight, double-click to revert.

use gpui::{
    div, prelude::*, px, uniform_list, AnyElement, Context, Entity, MouseButton, SharedString,
    Window,
};

use appcore::actions as act;
use appcore::state::ColorsMode;

use crate::app::HelixBench;
use crate::draw::PxVal;
use crate::theme;
use crate::views::{ellipsize, ScrollbarDecoration};
use crate::widgets::{self, Btn};

struct HRow {
    orig: Option<usize>,
    number: usize,
    label: String,
    regions: String,
    selected: bool,
}

impl HelixBench {
    pub fn render_history(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        let entity: Entity<Self> = cx.entity();
        let can_undo = !self.app.history.is_empty();
        let can_redo = !self.app.future.is_empty();
        let has_sel = self.app.history_highlight.is_some();

        let mut toolbar = div()
            .flex()
            .items_center()
            .gap(px(8.0))
            .px(px(12.0))
            .py(px(9.0))
            .bg(theme::SURFACE)
            .border_b_1()
            .border_color(theme::BORDER);
        let mut undo = widgets::icon_button("hist-undo", "undo", "Undo", Btn::Outline)
            .opacity(if can_undo { 1.0 } else { 0.4 });
        if can_undo {
            undo = undo.on_click(cx.listener(|t, _, _, cx| {
                if t.app.undo() {
                    t.app.toast("Undo");
                }
                cx.notify();
            }));
        }
        toolbar = toolbar.child(undo);
        let mut redo = widgets::icon_button("hist-redo", "redo", "Redo", Btn::Outline)
            .opacity(if can_redo { 1.0 } else { 0.4 });
        if can_redo {
            redo = redo.on_click(cx.listener(|t, _, _, cx| {
                if t.app.redo() {
                    t.app.toast("Redo");
                }
                cx.notify();
            }));
        }
        toolbar = toolbar.child(redo);
        let mut revert = widgets::icon_button(
            "hist-revert",
            "history",
            "Revert to Before Selected",
            Btn::Subtle,
        )
        .opacity(if has_sel { 1.0 } else { 0.4 });
        if has_sel {
            revert = revert.on_click(cx.listener(|t, _, _, cx| {
                if let Some(i) = t.app.history_highlight {
                    t.app.revert_to(i);
                    t.app.toast("Reverted to earlier state");
                }
                cx.notify();
            }));
        }
        toolbar = toolbar.child(revert);
        toolbar = toolbar.child(div().flex_grow().flex().justify_end().child(widgets::label(
            "Click an operation to highlight its regions · double-click to revert",
            theme::T_SM,
            theme::FAINT,
        )));

        // Saved version snapshots (persistent, restorable).
        let mut vpanel = div()
            .flex()
            .flex_col()
            .gap(px(4.0))
            .px(px(12.0))
            .py(px(8.0))
            .bg(theme::PANEL)
            .border_b_1()
            .border_color(theme::BORDER);
        vpanel = vpanel.child(
            div()
                .flex()
                .items_center()
                .gap(px(8.0))
                .child(widgets::label("Versions", theme::T_SM, theme::FAINT))
                .child(
                    widgets::button("hist-savever", "Save version", Btn::Outline).on_click(
                        cx.listener(|t, _, _, cx| {
                            act::save_version(&mut t.app, "");
                            cx.notify();
                        }),
                    ),
                ),
        );
        let mut vlist = div()
            .id("hist-versions")
            .flex()
            .flex_col()
            .gap(px(3.0))
            .max_h(px(140.0))
            .overflow_y_scroll();
        if self.app.doc.versions.is_empty() {
            vlist = vlist.child(widgets::label(
                "No saved versions yet. Save a snapshot to restore it later.",
                theme::T_XS,
                theme::FAINT,
            ));
        }
        for (i, v) in self.app.doc.versions.iter().enumerate().rev() {
            let label = v.label.clone();
            vlist = vlist.child(
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
                            .child(label),
                    )
                    .child(
                        widgets::button(
                            SharedString::from(format!("hist-restore-{i}")),
                            "Restore",
                            Btn::Subtle,
                        )
                        .on_click(cx.listener(move |t, _, _, cx| {
                            act::restore_version(&mut t.app, i);
                            cx.notify();
                        })),
                    ),
            );
        }
        vpanel = vpanel.child(vlist);

        if self.app.history.is_empty() {
            let empty = div()
                .size_full()
                .flex()
                .flex_col()
                .items_center()
                .justify_center()
                .gap(px(8.0))
                .child(
                    div()
                        .text_size(px(40.0))
                        .text_color(theme::FAINT)
                        .child("↺"),
                )
                .child(
                    div()
                        .text_size(px(theme::T_LG))
                        .text_color(theme::MUTED)
                        .child("No operations yet"),
                )
                .child(
                    div()
                        .text_size(px(theme::T_SM))
                        .text_color(theme::FAINT)
                        .child("Edits, annotations and enzyme changes are recorded here."),
                )
                .into_any_element();
            return div()
                .size_full()
                .flex()
                .flex_col()
                .bg(theme::BG)
                .child(toolbar)
                .child(vpanel)
                .child(div().flex_1().min_h(px(0.0)).child(empty))
                .into_any_element();
        }

        // Snapshot: index 0 = current-state marker, then entries newest-first.
        let l = self.app.doc.sequence.len();
        let nf = self.app.doc.features.len();
        let np = self.app.doc.primers.len();
        let highlight = self.app.history_highlight;
        let mut rows: Vec<HRow> = Vec::with_capacity(self.app.history.len() + 1);
        rows.push(HRow {
            orig: None,
            number: 0,
            label: format!("{l} bp · {nf} features · {np} primers"),
            regions: String::new(),
            selected: false,
        });
        for (i, entry) in self.app.history.iter().enumerate().rev() {
            let regions = if entry.regions.is_empty() {
                "whole document".to_string()
            } else {
                let ranges: Vec<String> = entry
                    .regions
                    .iter()
                    .take(3)
                    .map(|r| format!("{}..{}", r.range.start + 1, r.range.start + r.range.len))
                    .collect();
                let more = if entry.regions.len() > 3 {
                    format!(" +{}", entry.regions.len() - 3)
                } else {
                    String::new()
                };
                format!("{}{}", ranges.join(", "), more)
            };
            rows.push(HRow {
                orig: Some(i),
                number: i + 1,
                label: entry.label.clone(),
                regions,
                selected: highlight == Some(i),
            });
        }

        let n = rows.len();
        let scroll = self.history_scroll.clone();
        let label_w = (self.viewport.width.v() - 184.0 - 250.0).max(160.0);
        let body = uniform_list("history-list", n, move |range, window, _cx| {
            range
                .map(|idx| {
                    let r = &rows[idx];
                    let e_click = entity.clone();
                    let e_dbl = entity.clone();
                    if r.orig.is_none() {
                        let card = div()
                            .id("hist-current")
                            .mx(px(10.0))
                            .my(px(3.0))
                            .h(px(38.0))
                            .flex()
                            .items_center()
                            .gap(px(8.0))
                            .px(px(12.0))
                            .rounded(px(6.0))
                            .bg(theme::SURFACE)
                            .border_1()
                            .border_color(theme::BORDER)
                            .cursor_pointer()
                            .hover(|s| s.border_color(theme::ACCENT.opacity(0.6)))
                            .on_click(move |_, _, cx| {
                                e_click.update(cx, |t, cx| {
                                    t.app.history_highlight = None;
                                    cx.notify();
                                });
                            })
                            .child(
                                div()
                                    .text_color(theme::OK)
                                    .text_size(px(theme::T_SM))
                                    .child("●"),
                            )
                            .child(
                                div()
                                    .font_weight(gpui::FontWeight::SEMIBOLD)
                                    .text_size(px(theme::T_BASE))
                                    .text_color(theme::TEXT)
                                    .child("Current state"),
                            )
                            .child(
                                div()
                                    .text_size(px(theme::T_SM))
                                    .text_color(theme::MUTED)
                                    .child(r.label.clone()),
                            );
                        return div().h(px(44.0)).w_full().child(card).into_any_element();
                    }
                    let orig = r.orig.unwrap();
                    let number = r.number;
                    let label = ellipsize(window, &r.label, label_w, theme::T_BASE, false);
                    let label_dbl = r.label.clone();
                    let regions = r.regions.clone();
                    let selected = r.selected;
                    let whole = regions == "whole document";
                    let card = div()
                        .id(("hist-entry", orig))
                        .mx(px(10.0))
                        .my(px(3.0))
                        .h(px(38.0))
                        .flex()
                        .items_center()
                        .gap(px(10.0))
                        .px(px(12.0))
                        .rounded(px(6.0))
                        .bg(if selected {
                            theme::SURFACE_2
                        } else {
                            theme::PANEL
                        })
                        .border_1()
                        .border_color(if selected {
                            theme::ACCENT
                        } else {
                            theme::BORDER
                        })
                        .cursor_pointer()
                        .hover(|s| s.bg(theme::FILL_HOVER))
                        .on_click(move |_, _, cx| {
                            e_click.update(cx, |t, cx| {
                                t.app.history_highlight = Some(orig);
                                t.app.colors_mode = ColorsMode::History;
                                cx.notify();
                            });
                        })
                        .on_mouse_down(MouseButton::Left, move |ev, _, cx| {
                            if ev.click_count >= 2 {
                                let lbl = label_dbl.clone();
                                e_dbl.update(cx, |t, cx| {
                                    t.app.revert_to(orig);
                                    t.app.toast(format!("Reverted to before \"{lbl}\""));
                                    cx.notify();
                                });
                            }
                        })
                        .child(
                            div()
                                .font_family(theme::MONO_FONT)
                                .text_size(px(theme::T_SM))
                                .text_color(theme::FAINT)
                                .child(number.to_string()),
                        )
                        .child(
                            div()
                                .flex_1()
                                .min_w(px(0.0))
                                .overflow_hidden()
                                .flex()
                                .items_center()
                                .child(
                                    div()
                                        .text_size(px(theme::T_BASE))
                                        .text_color(theme::TEXT)
                                        .child(label),
                                ),
                        )
                        .child(
                            div()
                                .flex_shrink_0()
                                .font_family(if whole {
                                    theme::UI_FONT
                                } else {
                                    theme::MONO_FONT
                                })
                                .text_size(px(theme::T_XS))
                                .text_color(if whole { theme::FAINT } else { theme::MUTED })
                                .child(regions),
                        );
                    div().h(px(44.0)).w_full().child(card).into_any_element()
                })
                .collect()
        })
        .track_scroll(scroll)
        .with_decoration(ScrollbarDecoration)
        .size_full();

        div()
            .size_full()
            .flex()
            .flex_col()
            .bg(theme::BG)
            .child(toolbar)
            .child(vpanel)
            .child(div().flex_1().min_h(px(0.0)).overflow_hidden().child(body))
            .into_any_element()
    }
}
