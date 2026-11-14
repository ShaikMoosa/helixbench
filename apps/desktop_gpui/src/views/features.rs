//! Features view: sortable annotation table with visibility toggles, filtering
//! and inline selection. Ported from the egui shell onto a GPUI uniform_list.

use gpui::{
    div, prelude::*, px, uniform_list, AnyElement, Context, Entity, MouseButton, SharedString,
    Window,
};

use appcore::actions as act;
use appcore::state::SortDir;
use domain::genetic_code::code_by_id;

use crate::app::HelixBench;
use crate::draw::PxVal;
use crate::theme;
use crate::views::{
    cell, check_box, cycle_sort, ellipsize, header_label, row_bg, sort_header, table_scaffold,
    ScrollbarDecoration,
};
use crate::widgets::{self, Btn};

struct Row {
    id: u64,
    name: String,
    kind: String,
    dir: &'static str,
    total: usize,
    loc: String,
    trans: u8, // 0 = no, 1 = yes, 2 = yes with internal stop
    product: String,
    color: gpui::Hsla,
    visible: bool,
    selected: bool,
}

impl HelixBench {
    pub fn render_features(&mut self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        self.app.ensure_analysis();
        let l = self.app.doc.sequence.len();
        let entity: Entity<Self> = cx.entity();

        let seed = self.app.feature_filter.clone();
        let filter_input =
            self.input_entity(cx, "filter:features", &seed, "Filter features…", false);
        self.app.feature_filter = filter_input.read(cx).text();
        let filter = self.app.feature_filter.to_ascii_lowercase();

        let code = code_by_id(self.app.doc.default_genetic_code);
        let selected = self.app.selected_feature;
        let mut items: Vec<u64> = self
            .app
            .doc
            .features
            .iter()
            .filter(|f| {
                filter.is_empty()
                    || f.name.to_ascii_lowercase().contains(&filter)
                    || f.kind.to_ascii_lowercase().contains(&filter)
                    || f.product.to_ascii_lowercase().contains(&filter)
            })
            .map(|f| f.id)
            .collect();

        let sort = self.app.feature_sort;
        if let Some(dir) = sort.dir {
            let doc = &self.app.doc;
            items.sort_by(|a, b| {
                let fa = doc.feature(*a).unwrap();
                let fb = doc.feature(*b).unwrap();
                let ord = match sort.column {
                    1 => fa.name.to_lowercase().cmp(&fb.name.to_lowercase()),
                    2 => fa.kind.cmp(&fb.kind),
                    3 => fa.direction.label().cmp(fb.direction.label()),
                    4 => fa.total_len().cmp(&fb.total_len()),
                    5 => fa.span(l).start.cmp(&fb.span(l).start),
                    _ => std::cmp::Ordering::Equal,
                };
                if dir == SortDir::Desc {
                    ord.reverse()
                } else {
                    ord
                }
            });
        }

        let rows: Vec<Row> = items
            .iter()
            .map(|id| {
                let f = self.app.doc.feature(*id).unwrap();
                let span = f.span(l);
                let loc = if f.segments.len() > 1 {
                    format!("join({}..{})", span.start + 1, span.start + span.len)
                } else {
                    format!("{}..{}", span.start + 1, span.start + span.len)
                };
                let trans = if f.translated {
                    if f.has_internal_stop(&self.app.doc.sequence, code) {
                        2
                    } else {
                        1
                    }
                } else {
                    0
                };
                Row {
                    id: f.id,
                    name: f.name.clone(),
                    kind: f.kind.clone(),
                    dir: f.direction.label(),
                    total: f.total_len(),
                    loc,
                    trans,
                    product: if !f.product.is_empty() {
                        f.product.clone()
                    } else {
                        f.note.clone()
                    },
                    color: theme::rgb_arr(f.color),
                    visible: f.visible,
                    selected: selected == Some(f.id),
                }
            })
            .collect();

        // --- Toolbar ---
        let sel = self.app.selected_feature;
        let count = self.app.doc.features.len();
        let mut toolbar = div()
            .flex()
            .items_center()
            .gap(px(8.0))
            .px(px(12.0))
            .py(px(9.0))
            .bg(theme::SURFACE)
            .border_b_1()
            .border_color(theme::BORDER)
            .child(div().w(px(200.0)).child(filter_input.clone()))
            .child(
                widgets::icon_button("feat-add", "plus", "Add Feature", Btn::Primary).on_click(
                    cx.listener(|t, _, _, cx| {
                        crate::dialogs::open_feature_editor(&mut t.app, None);
                        cx.notify();
                    }),
                ),
            );
        let mut edit = widgets::icon_button("feat-edit", "pencil", "Edit", Btn::Subtle)
            .opacity(if sel.is_some() { 1.0 } else { 0.4 });
        if sel.is_some() {
            edit = edit.on_click(cx.listener(move |t, _, _, cx| {
                crate::dialogs::open_feature_editor(&mut t.app, sel);
                cx.notify();
            }));
        }
        toolbar = toolbar.child(edit);
        let mut del = widgets::icon_button("feat-del", "trash", "Delete", Btn::Danger)
            .opacity(if sel.is_some() { 1.0 } else { 0.4 });
        if sel.is_some() {
            del = del.on_click(cx.listener(move |t, _, _, cx| {
                if let Some(id) = sel {
                    act::delete_feature(&mut t.app, id);
                }
                cx.notify();
            }));
        }
        toolbar = toolbar.child(del);
        toolbar = toolbar.child(
            widgets::icon_button("feat-detect", "scan", "Detect Common", Btn::Subtle).on_click(
                cx.listener(|t, _, _, cx| {
                    crate::dialogs::open_detect_common(&mut t.app);
                    cx.notify();
                }),
            ),
        );
        toolbar = toolbar.child(div().flex_grow().flex().justify_end().child(widgets::label(
            &format!("{count} features"),
            theme::T_SM,
            theme::MUTED,
        )));

        // --- Header ---
        let content_w = self.viewport.width.v() - 184.0;
        let flexw = (content_w - 458.0).max(240.0);
        let name_w = (flexw * 0.38).clamp(150.0, 340.0);
        let prod_w = (flexw - name_w).max(120.0);
        let mut header = div()
            .flex()
            .items_center()
            .w_full()
            .child(header_label(window, "", 34.0));
        for (col, label, w) in [
            (1, "Name", name_w),
            (2, "Kind", 96.0),
            (3, "Dir", 44.0),
            (4, "Length", 72.0),
            (5, "Location", 128.0),
            (6, "Translated", 84.0),
        ] {
            let h = sort_header(
                window,
                SharedString::from(format!("fh-{col}")),
                label,
                w,
                col,
                sort,
            );
            header = header.child(h.on_click(cx.listener(move |t, _, _, cx| {
                cycle_sort(&mut t.app.feature_sort, col);
                cx.notify();
            })));
        }
        header = header.child(header_label(window, "Product / Note", prod_w));

        // --- Body ---
        let n = rows.len();
        let scroll = self.feat_scroll.clone();
        let body = if n == 0 {
            empty_state(
                "No features match",
                "Add a feature or clear the filter to see annotations.",
            )
        } else {
            uniform_list("features-list", n, move |range, window, _cx| {
                range
                    .map(|idx| {
                        let r = &rows[idx];
                        let id = r.id;
                        let e_sel = entity.clone();
                        let e_dbl = entity.clone();
                        let e_tog = entity.clone();
                        let name = ellipsize(window, &r.name, name_w - 32.0, theme::T_BASE, false);
                        let color = r.color;
                        let visible = r.visible;
                        let row = div()
                            .id(("feat-row", id))
                            .h(px(27.0))
                            .w_full()
                            .flex()
                            .items_center()
                            .bg(row_bg(r.selected, idx))
                            .border_b_1()
                            .border_color(theme::BORDER.opacity(0.45))
                            .cursor_pointer()
                            .hover(|s| s.bg(theme::SURFACE_2.opacity(0.6)))
                            .on_click(move |_, _, cx| {
                                e_sel.update(cx, |t, cx| {
                                    t.app.select_feature(id);
                                    cx.notify();
                                });
                            })
                            .on_mouse_down(MouseButton::Left, move |ev, _, cx| {
                                if ev.click_count >= 2 {
                                    e_dbl.update(cx, |t, cx| {
                                        crate::dialogs::open_feature_editor(&mut t.app, Some(id));
                                        cx.notify();
                                    });
                                }
                            })
                            .child(
                                div()
                                    .w(px(34.0))
                                    .h_full()
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .child(
                                        check_box(SharedString::from(format!("fv-{id}")), visible)
                                            .on_click(move |_, _, cx| {
                                                cx.stop_propagation();
                                                e_tog.update(cx, |t, cx| {
                                                    let name = t
                                                        .app
                                                        .doc
                                                        .feature(id)
                                                        .map(|f| f.name.clone())
                                                        .unwrap_or_default();
                                                    t.app.mutate(
                                                        &format!("Toggle feature \"{name}\""),
                                                        Vec::new(),
                                                        |doc| {
                                                            if let Some(f) = doc.feature_mut(id) {
                                                                f.visible = !f.visible;
                                                            }
                                                        },
                                                    );
                                                    cx.notify();
                                                });
                                            }),
                                    ),
                            )
                            .child(
                                div()
                                    .h_full()
                                    .w(px(name_w))
                                    .flex_shrink_0()
                                    .flex()
                                    .items_center()
                                    .gap(px(7.0))
                                    .px(px(8.0))
                                    .overflow_hidden()
                                    .child(
                                        div()
                                            .w(px(9.0))
                                            .h(px(9.0))
                                            .rounded(px(2.0))
                                            .bg(color)
                                            .flex_shrink_0(),
                                    )
                                    .child(
                                        div()
                                            .text_size(px(theme::T_BASE))
                                            .text_color(theme::TEXT)
                                            .child(name),
                                    ),
                            )
                            .child(cell(window, &r.kind, 96.0, theme::MUTED, false))
                            .child(cell(window, r.dir, 44.0, theme::ACCENT, false))
                            .child(cell(window, &r.total.to_string(), 72.0, theme::TEXT, true))
                            .child(cell(window, &r.loc, 128.0, theme::MUTED, true));
                        let (txt, col) = match r.trans {
                            1 => ("yes", theme::OK),
                            2 => ("yes (stops)", theme::WARN),
                            _ => ("—", theme::FAINT),
                        };
                        row.child(cell(window, txt, 84.0, col, false))
                            .child(cell(window, &r.product, prod_w, theme::MUTED, false))
                            .into_any_element()
                    })
                    .collect()
            })
            .track_scroll(scroll)
            .with_decoration(ScrollbarDecoration)
            .size_full()
            .into_any_element()
        };

        table_scaffold(toolbar.into_any_element(), header.into_any_element(), body)
            .into_any_element()
    }
}

pub fn empty_state(title: &str, sub: &str) -> AnyElement {
    div()
        .size_full()
        .flex()
        .flex_col()
        .items_center()
        .justify_center()
        .gap(px(8.0))
        .child(
            div()
                .text_size(px(34.0))
                .text_color(theme::FAINT)
                .child("◌"),
        )
        .child(
            div()
                .text_size(px(theme::T_MD))
                .text_color(theme::MUTED)
                .child(title.to_owned()),
        )
        .child(
            div()
                .text_size(px(theme::T_SM))
                .text_color(theme::FAINT)
                .child(sub.to_owned()),
        )
        .into_any_element()
}
