//! Primers view: oligo table with Tm, GC and binding-site counts.

use gpui::{
    div, prelude::*, px, uniform_list, AnyElement, Context, Entity, Hsla, MouseButton,
    SharedString, Window,
};

use appcore::actions as act;
use appcore::state::SortDir;
use domain::primer::{melting_temperature, HybridizationParams};

use crate::app::HelixBench;
use crate::draw::PxVal;
use crate::theme;
use crate::views::features::empty_state;
use crate::views::{
    cell, check_box, cycle_sort, ellipsize, header_label, row_bg, sort_header, table_scaffold,
    ScrollbarDecoration,
};
use crate::widgets::{self, Btn};

const COLS: &[(usize, &str)] = &[
    (1, "Name"),
    (2, "Sequence 5′→3′"),
    (3, "Length"),
    (4, "GC%"),
    (5, "Tm (°C)"),
    (6, "Sites"),
    (7, "Description"),
];

struct Row {
    id: u64,
    name: String,
    seq: String,
    len: usize,
    gc: f32,
    tm: Option<f32>,
    sites: usize,
    description: String,
    color: Hsla,
    visible: bool,
    phospho: bool,
    selected: bool,
}

impl HelixBench {
    pub fn render_primers(&mut self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        self.app.ensure_analysis();
        let entity: Entity<Self> = cx.entity();
        let params = HybridizationParams::default();

        let seed = self.app.primer_filter.clone();
        let filter_input = self.input_entity(cx, "filter:primers", &seed, "Filter primers…", false);
        self.app.primer_filter = filter_input.read(cx).text();
        let filter = self.app.primer_filter.to_ascii_lowercase();

        let selected = self.app.selected_primer;
        let mut items: Vec<u64> = self
            .app
            .doc
            .primers
            .iter()
            .filter(|p| {
                filter.is_empty()
                    || p.name.to_ascii_lowercase().contains(&filter)
                    || p.description.to_ascii_lowercase().contains(&filter)
            })
            .map(|p| p.id)
            .collect();

        let sort = self.app.primer_sort;
        if let Some(dir) = sort.dir {
            let doc = &self.app.doc;
            let analysis = &self.app.analysis;
            items.sort_by(|a, b| {
                let pa = doc.primer(*a).unwrap();
                let pb = doc.primer(*b).unwrap();
                let ord = match sort.column {
                    1 => pa.name.to_lowercase().cmp(&pb.name.to_lowercase()),
                    3 => pa.len().cmp(&pb.len()),
                    4 => pa.gc_percent().total_cmp(&pb.gc_percent()),
                    5 => {
                        let ta = melting_temperature(&pa.sequence, &params).unwrap_or(-1.0);
                        let tb = melting_temperature(&pb.sequence, &params).unwrap_or(-1.0);
                        ta.total_cmp(&tb)
                    }
                    6 => {
                        let na = analysis.primer_sites.get(a).map(|s| s.len()).unwrap_or(0);
                        let nb = analysis.primer_sites.get(b).map(|s| s.len()).unwrap_or(0);
                        na.cmp(&nb)
                    }
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
                let p = self.app.doc.primer(*id).unwrap();
                let seq = String::from_utf8_lossy(&p.sequence);
                let shown: String = seq.chars().take(36).collect();
                let ell = if seq.len() > 36 { "…" } else { "" };
                Row {
                    id: p.id,
                    name: p.name.clone(),
                    seq: format!("{shown}{ell}"),
                    len: p.len(),
                    gc: p.gc_percent(),
                    tm: melting_temperature(&p.sequence, &params),
                    sites: self
                        .app
                        .analysis
                        .primer_sites
                        .get(id)
                        .map(|s| s.len())
                        .unwrap_or(0),
                    description: p.description.clone(),
                    color: theme::rgb_arr(p.color),
                    visible: p.visible,
                    phospho: p.phosphorylated,
                    selected: selected == Some(p.id),
                }
            })
            .collect();

        let sel = self.app.selected_primer;
        let count = self.app.doc.primers.len();
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
                widgets::icon_button("pr-add", "plus", "Add Primer", Btn::Primary).on_click(
                    cx.listener(|t, _, _, cx| {
                        crate::dialogs::open_primer_editor(&mut t.app, None);
                        cx.notify();
                    }),
                ),
            )
            .child(
                widgets::icon_button("pr-fromsel", "plus", "From Selection", Btn::Subtle).on_click(
                    cx.listener(|t, _, _, cx| {
                        act::primer_from_selection(&mut t.app);
                        cx.notify();
                    }),
                ),
            );
        let mut edit = widgets::icon_button("pr-edit", "pencil", "Edit", Btn::Subtle)
            .opacity(if sel.is_some() { 1.0 } else { 0.4 });
        if sel.is_some() {
            edit = edit.on_click(cx.listener(move |t, _, _, cx| {
                crate::dialogs::open_primer_editor(&mut t.app, sel);
                cx.notify();
            }));
        }
        toolbar = toolbar.child(edit);
        let mut del = widgets::icon_button("pr-del", "trash", "Delete", Btn::Danger)
            .opacity(if sel.is_some() { 1.0 } else { 0.4 });
        if sel.is_some() {
            del = del.on_click(cx.listener(move |t, _, _, cx| {
                if let Some(id) = sel {
                    act::delete_primer(&mut t.app, id);
                }
                cx.notify();
            }));
        }
        toolbar = toolbar.child(del);
        toolbar = toolbar.child(
            crate::views::table_opts::opts_button(
                "pr-opts",
                self.opts.as_deref() == Some("primers"),
            )
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|t, ev: &gpui::MouseDownEvent, _, cx| {
                    crate::views::table_opts::toggle_opts(t, "primers", ev, cx)
                }),
            ),
        );
        toolbar = toolbar.child(div().flex_grow().flex().justify_end().child(widgets::label(
            &format!(
                "{count} primers · Tm at {} mM Na⁺, {} nM oligo",
                params.salt_mm, params.primer_nm
            ),
            theme::T_SM,
            theme::MUTED,
        )));

        let hid = self.app.hidden_cols("primers");
        let content_w = self.viewport.width.v() - 184.0;
        let seq_w = (content_w - 596.0).max(140.0);
        let mut header = div()
            .flex()
            .items_center()
            .w_full()
            .child(header_label(window, "", 34.0));
        let mut hcells: Vec<AnyElement> = Vec::new();
        if !hid.contains(&1) {
            hcells.push(
                sort_header(window, "ph-1", "Name", 150.0, 1, sort)
                    .on_click(cx.listener(|t, _, _, cx| {
                        cycle_sort(&mut t.app.primer_sort, 1);
                        cx.notify();
                    }))
                    .into_any_element(),
            );
        }
        if !hid.contains(&2) {
            hcells.push(header_label(window, "Sequence 5′→3′", seq_w).into_any_element());
        }
        for (col, label, w) in [
            (3, "Length", 64.0),
            (4, "GC%", 60.0),
            (5, "Tm (°C)", 72.0),
            (6, "Sites", 56.0),
        ] {
            if hid.contains(&col) {
                continue;
            }
            hcells.push(
                sort_header(
                    window,
                    SharedString::from(format!("ph-{col}")),
                    label,
                    w,
                    col,
                    sort,
                )
                .on_click(cx.listener(move |t, _, _, cx| {
                    cycle_sort(&mut t.app.primer_sort, col);
                    cx.notify();
                }))
                .into_any_element(),
            );
        }
        if !hid.contains(&7) {
            hcells.push(header_label(window, "Description", 160.0).into_any_element());
        }
        header = header.children(hcells);

        let n = rows.len();
        let scroll = self.primer_scroll.clone();
        let min_tm = params.min_tm;
        let body = if n == 0 {
            empty_state(
                "No primers match",
                "Add a primer or create one from the current selection.",
            )
        } else {
            uniform_list("primers-list", n, move |range, window, _cx| {
                range
                    .map(|idx| {
                        let r = &rows[idx];
                        let id = r.id;
                        let e_sel = entity.clone();
                        let e_dbl = entity.clone();
                        let e_tog = entity.clone();
                        let color = r.color;
                        let visible = r.visible;
                        let phospho = r.phospho;
                        let name_max = if phospho { 101.0 } else { 118.0 };
                        let name = ellipsize(window, &r.name, name_max, theme::T_BASE, false);
                        let (tm_txt, tm_col) = match r.tm {
                            Some(tm) => (
                                format!("{tm:.1}"),
                                if tm < min_tm { theme::WARN } else { theme::OK },
                            ),
                            None => ("—".into(), theme::FAINT),
                        };
                        let sites_col = if r.sites == 0 {
                            theme::WARN
                        } else {
                            theme::TEXT
                        };
                        let row = div()
                            .id(("pr-row", id))
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
                                    t.app.selected_primer = Some(id);
                                    t.app.selected_feature = None;
                                    if let Some(s) =
                                        t.app.analysis.primer_sites.get(&id).and_then(|s| s.first())
                                    {
                                        t.app.select(Some(domain::Range::new(s.start, s.len)));
                                    }
                                    cx.notify();
                                });
                            })
                            .on_mouse_down(MouseButton::Left, move |ev, _, cx| {
                                if ev.click_count >= 2 {
                                    e_dbl.update(cx, |t, cx| {
                                        crate::dialogs::open_primer_editor(&mut t.app, Some(id));
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
                                        check_box(SharedString::from(format!("pv-{id}")), visible)
                                            .on_click(move |_, _, cx| {
                                                cx.stop_propagation();
                                                e_tog.update(cx, |t, cx| {
                                                    let name = t
                                                        .app
                                                        .doc
                                                        .primer(id)
                                                        .map(|p| p.name.clone())
                                                        .unwrap_or_default();
                                                    t.app.mutate(
                                                        &format!("Toggle primer \"{name}\""),
                                                        Vec::new(),
                                                        |doc| {
                                                            if let Some(p) = doc.primer_mut(id) {
                                                                p.visible = !p.visible;
                                                            }
                                                        },
                                                    );
                                                    cx.notify();
                                                });
                                            }),
                                    ),
                            );
                        let mut cells: Vec<AnyElement> = Vec::new();
                        if !hid.contains(&1) {
                            cells.push(
                                div()
                                    .h_full()
                                    .w(px(150.0))
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
                                    )
                                    .child(if phospho {
                                        div()
                                            .text_size(px(theme::T_XS))
                                            .text_color(theme::WARN)
                                            .child("ⓟ")
                                    } else {
                                        div()
                                    })
                                    .into_any_element(),
                            );
                        }
                        if !hid.contains(&2) {
                            cells.push(
                                cell(window, &r.seq, seq_w, theme::MUTED, true).into_any_element(),
                            );
                        }
                        if !hid.contains(&3) {
                            cells.push(
                                cell(window, &r.len.to_string(), 64.0, theme::TEXT, true)
                                    .into_any_element(),
                            );
                        }
                        if !hid.contains(&4) {
                            cells.push(
                                cell(window, &format!("{:.0}", r.gc), 60.0, theme::TEXT, true)
                                    .into_any_element(),
                            );
                        }
                        if !hid.contains(&5) {
                            cells
                                .push(cell(window, &tm_txt, 72.0, tm_col, true).into_any_element());
                        }
                        if !hid.contains(&6) {
                            cells.push(
                                cell(window, &r.sites.to_string(), 56.0, sites_col, true)
                                    .into_any_element(),
                            );
                        }
                        if !hid.contains(&7) {
                            cells.push(
                                cell(window, &r.description, 160.0, theme::MUTED, false)
                                    .into_any_element(),
                            );
                        }
                        row.children(cells).into_any_element()
                    })
                    .collect()
            })
            .track_scroll(scroll)
            .with_decoration(ScrollbarDecoration)
            .size_full()
            .into_any_element()
        };

        let mut root = table_scaffold(toolbar.into_any_element(), header.into_any_element(), body);
        if let Some(p) = crate::views::table_opts::render_table_opts(self, cx, "primers", COLS) {
            root = root.child(p);
        }
        root.into_any_element()
    }
}
