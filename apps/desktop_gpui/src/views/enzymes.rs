//! Enzymes view: choose the working enzyme set with filters and cut counts.
//! Dropdowns are anchored, deferred popups drawn in the instrument palette.

use gpui::{
    anchored, deferred, div, point, prelude::*, px, uniform_list, AnyElement, Context, Corner, Div,
    ElementId, Entity, MouseButton, SharedString, Stateful, Window,
};

use appcore::state::{
    AppState, SortDir, CUT_COUNT_OPTIONS, OVERHANG_OPTIONS, REGION_OPTIONS, SITE_LEN_OPTIONS,
};
use domain::enzyme::Overhang;
use domain::{enzyme_db, Enzyme, EnzymeSet};

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
    (1, "Enzyme"),
    (2, "Recognition"),
    (3, "Cuts"),
    (4, "Overhang"),
    (5, "Cut positions"),
];

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ComboKind {
    Set,
    CutCount,
    Overhang,
    SiteLen,
    Region,
}

struct Row {
    name: String,
    site: String,
    n: usize,
    overhang: String,
    positions: String,
    in_set: bool,
    first_site: Option<usize>,
}

fn combo_btn(id: impl Into<ElementId>, label: &str, open: bool) -> Stateful<Div> {
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
        .hover(|s| s.bg(theme::FILL_HOVER))
        .child(div().truncate().child(label.to_owned()))
        .child(div().text_color(theme::FAINT).text_size(px(9.0)).child("▾"))
}

impl HelixBench {
    pub fn render_enzymes(&mut self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        self.app.ensure_analysis();
        let entity: Entity<Self> = cx.entity();

        let seed = self.app.enzyme_filter.name.clone();
        let filter_input = self.input_entity(cx, "filter:enzymes", &seed, "Name contains…", false);
        self.app.enzyme_filter.name = filter_input.read(cx).text();

        let sets = self.app.analysis.sets.clone();
        let set_name = current_set_name(&self.app, &sets);
        let shown = self.app.analysis.chosen.len();

        // --- Toolbar (two rows) ---
        let row1 = div()
            .flex()
            .items_center()
            .gap(px(8.0))
            .child(widgets::label("Enzyme set", theme::T_SM, theme::MUTED))
            .child(
                combo_btn("enz-set", &set_name, self.combo == Some(ComboKind::Set)).on_mouse_down(
                    MouseButton::Left,
                    cx.listener(|t, ev: &gpui::MouseDownEvent, _, cx| {
                        if t.combo == Some(ComboKind::Set) {
                            t.combo = None;
                        } else {
                            t.combo = Some(ComboKind::Set);
                            t.combo_anchor = ev.position;
                        }
                        cx.notify();
                    }),
                ),
            )
            .child(div().w(px(1.0)).h(px(18.0)).bg(theme::BORDER))
            .child(widgets::label(
                &format!("{shown} enzymes shown on map"),
                theme::T_SM,
                theme::FAINT,
            ));

        let cc = self.app.enzyme_filter.cut_count;
        let oh = self.app.enzyme_filter.overhang;
        let sl = self.app.enzyme_filter.site_len;
        let row2 =
            div()
                .flex()
                .items_center()
                .gap(px(8.0))
                .child(div().w(px(150.0)).child(filter_input.clone()))
                .child(
                    combo_btn(
                        "enz-cc",
                        CUT_COUNT_OPTIONS[cc],
                        self.combo == Some(ComboKind::CutCount),
                    )
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|t, ev: &gpui::MouseDownEvent, _, cx| {
                            if t.combo == Some(ComboKind::CutCount) {
                                t.combo = None;
                            } else {
                                t.combo = Some(ComboKind::CutCount);
                                t.combo_anchor = ev.position;
                            }
                            cx.notify();
                        }),
                    ),
                )
                .child(
                    combo_btn(
                        "enz-oh",
                        OVERHANG_OPTIONS[oh],
                        self.combo == Some(ComboKind::Overhang),
                    )
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|t, ev: &gpui::MouseDownEvent, _, cx| {
                            if t.combo == Some(ComboKind::Overhang) {
                                t.combo = None;
                            } else {
                                t.combo = Some(ComboKind::Overhang);
                                t.combo_anchor = ev.position;
                            }
                            cx.notify();
                        }),
                    ),
                )
                .child(
                    combo_btn(
                        "enz-sl",
                        SITE_LEN_OPTIONS[sl],
                        self.combo == Some(ComboKind::SiteLen),
                    )
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|t, ev: &gpui::MouseDownEvent, _, cx| {
                            if t.combo == Some(ComboKind::SiteLen) {
                                t.combo = None;
                            } else {
                                t.combo = Some(ComboKind::SiteLen);
                                t.combo_anchor = ev.position;
                            }
                            cx.notify();
                        }),
                    ),
                )
                .child(
                    widgets::icon_button("enz-check", "check", "Check shown", Btn::Subtle)
                        .on_click(cx.listener(|t, _, _, cx| {
                            let names = filtered_names(&t.app);
                            t.app
                                .mutate("Add filtered enzymes to set", Vec::new(), |doc| {
                                    for n in names {
                                        if !doc.chosen_enzymes.contains(&n) {
                                            doc.chosen_enzymes.push(n);
                                        }
                                    }
                                });
                            cx.notify();
                        })),
                )
                .child(
                    widgets::icon_button("enz-uncheck", "x", "Uncheck shown", Btn::Subtle)
                        .on_click(cx.listener(|t, _, _, cx| {
                            let names = filtered_names(&t.app);
                            t.app
                                .mutate("Remove filtered enzymes from set", Vec::new(), |doc| {
                                    doc.chosen_enzymes.retain(|n| !names.contains(n));
                                });
                            cx.notify();
                        })),
                )
                .child(
                    widgets::button("enz-clear", "Clear set", Btn::Ghost).on_click(cx.listener(
                        |t, _, _, cx| {
                            t.app.mutate("Clear enzyme set", Vec::new(), |doc| {
                                doc.chosen_enzymes.clear()
                            });
                            cx.notify();
                        },
                    )),
                );

        let region = self.app.enzyme_filter.region;
        let rs_seed = self.app.enzyme_filter.region_start.clone();
        let re_seed = self.app.enzyme_filter.region_end.clone();
        let rs_inp = self.input_entity(cx, "filter:enz-rstart", &rs_seed, "start", false);
        let re_inp = self.input_entity(cx, "filter:enz-rend", &re_seed, "end", false);
        self.app.enzyme_filter.region_start = rs_inp.read(cx).text();
        self.app.enzyme_filter.region_end = re_inp.read(cx).text();
        let mut row3 = div()
            .flex()
            .items_center()
            .gap(px(8.0))
            .child(widgets::label("Region", theme::T_SM, theme::MUTED))
            .child(
                combo_btn(
                    "enz-region",
                    REGION_OPTIONS[region],
                    self.combo == Some(ComboKind::Region),
                )
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(|t, ev: &gpui::MouseDownEvent, _, cx| {
                        if t.combo == Some(ComboKind::Region) {
                            t.combo = None;
                        } else {
                            t.combo = Some(ComboKind::Region);
                            t.combo_anchor = ev.position;
                        }
                        cx.notify();
                    }),
                ),
            );
        if region == 2 {
            row3 = row3
                .child(div().w(px(80.0)).child(rs_inp))
                .child(widgets::label("..", theme::T_SM, theme::FAINT))
                .child(div().w(px(80.0)).child(re_inp));
        } else if region == 1 {
            let summary = match self.app.selection {
                Some(sel) => format!("selection {}..{}", sel.start + 1, sel.start + sel.len),
                None => "no selection".into(),
            };
            row3 = row3.child(widgets::label(&summary, theme::T_SM, theme::FAINT));
        }
        if let Some(r) = self.app.enzyme_region() {
            row3 = row3.child(widgets::label(
                &format!("cuts in {}..{}", r.start + 1, r.start + r.len),
                theme::T_XS,
                theme::ACCENT,
            ));
        }

        let row2 = row2.child(
            crate::views::table_opts::opts_button(
                "enz-opts",
                self.opts.as_deref() == Some("enzymes"),
            )
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|t, ev: &gpui::MouseDownEvent, _, cx| {
                    crate::views::table_opts::toggle_opts(t, "enzymes", ev, cx)
                }),
            ),
        );

        let toolbar = div()
            .flex()
            .flex_col()
            .gap(px(8.0))
            .px(px(12.0))
            .py(px(9.0))
            .bg(theme::SURFACE)
            .border_b_1()
            .border_color(theme::BORDER)
            .child(row1)
            .child(row2)
            .child(row3);

        // --- Filter + sort ---
        let mut items = filtered_enzymes(&self.app);
        let sort = self.app.enzyme_sort;
        if let Some(dir) = sort.dir {
            items.sort_by(|a, b| {
                let ord = match sort.column {
                    1 => a.0.name.to_lowercase().cmp(&b.0.name.to_lowercase()),
                    2 => a.0.site.len().cmp(&b.0.site.len()),
                    3 => a.1.cmp(&b.1),
                    _ => std::cmp::Ordering::Equal,
                };
                if dir == SortDir::Desc {
                    ord.reverse()
                } else {
                    ord
                }
            });
        } else {
            items.sort_by_key(|a| a.0.name.to_lowercase());
        }

        let rows: Vec<Row> = items
            .into_iter()
            .map(|(e, n)| {
                let outside = e.cut_top < 0 || e.cut_top > e.site_len() as i32;
                let oh = e.overhang();
                let overhang = if outside {
                    format!("{} · outside site", oh.label())
                } else {
                    oh.label()
                };
                let empty = Vec::new();
                let sites = self.app.analysis.all_sites.get(&e.name).unwrap_or(&empty);
                let mut pos: Vec<String> = sites
                    .iter()
                    .take(8)
                    .map(|s| format!("{}", s.cut_top + 1))
                    .collect();
                if sites.len() > 8 {
                    pos.push(format!("+{} more", sites.len() - 8));
                }
                Row {
                    in_set: self.app.doc.chosen_enzymes.contains(&e.name),
                    first_site: sites.first().map(|s| s.position),
                    name: e.name,
                    site: e.site,
                    n,
                    overhang,
                    positions: if n == 0 { "—".into() } else { pos.join(", ") },
                }
            })
            .collect();

        // --- Header ---
        let hid = self.app.hidden_cols("enzymes");
        let content_w = self.viewport.width.v() - 184.0;
        let pos_w = (content_w - 508.0).max(140.0);
        let mut header = div()
            .flex()
            .items_center()
            .w_full()
            .child(header_label(window, "✓", 34.0));
        let mut hcells: Vec<AnyElement> = Vec::new();
        for (col, label, w) in [
            (1, "Enzyme", 120.0),
            (2, "Recognition", 140.0),
            (3, "Cuts", 64.0),
        ] {
            if hid.contains(&col) {
                continue;
            }
            hcells.push(
                sort_header(
                    window,
                    SharedString::from(format!("eh-{col}")),
                    label,
                    w,
                    col,
                    sort,
                )
                .on_click(cx.listener(move |t, _, _, cx| {
                    cycle_sort(&mut t.app.enzyme_sort, col);
                    cx.notify();
                }))
                .into_any_element(),
            );
        }
        if !hid.contains(&4) {
            hcells.push(header_label(window, "Overhang", 150.0).into_any_element());
        }
        if !hid.contains(&5) {
            hcells.push(header_label(window, "Cut positions", pos_w).into_any_element());
        }
        header = header.children(hcells);

        // --- Body ---
        let n = rows.len();
        let scroll = self.enzyme_scroll.clone();
        let body = if n == 0 {
            empty_state(
                "No enzymes match",
                "Adjust the name, cut-count, overhang, length or region filters.",
            )
        } else {
            uniform_list("enzymes-list", n, move |range, window, _cx| {
                range
                    .map(|idx| {
                        let r = &rows[idx];
                        let name = r.name.clone();
                        let name_tog = name.clone();
                        let name_fitted = ellipsize(window, &r.name, 100.0, theme::T_SM, false);
                        let n_cuts = r.n;
                        let in_set = r.in_set;
                        let first_site = r.first_site;
                        let e_tog = entity.clone();
                        let e_click = entity.clone();
                        let name_col = if n_cuts == 0 {
                            theme::FAINT
                        } else if n_cuts == 1 {
                            theme::TEXT
                        } else {
                            theme::MUTED
                        };
                        let cuts_col = match n_cuts {
                            0 => theme::FAINT,
                            1 => theme::OK,
                            2 => theme::WARN,
                            _ => theme::DANGER,
                        };
                        let pos_col = if n_cuts == 0 {
                            theme::FAINT
                        } else {
                            theme::MUTED
                        };
                        let row = div()
                            .id(("enz-row", idx))
                            .h(px(26.0))
                            .w_full()
                            .flex()
                            .items_center()
                            .bg(row_bg(in_set && n_cuts > 0, idx))
                            .border_b_1()
                            .border_color(theme::HAIRLINE)
                            .cursor_pointer()
                            .hover(|s| s.bg(theme::FILL_HOVER_STRONG))
                            .on_click(move |_, _, cx| {
                                if let Some(pos) = first_site {
                                    let nm = name.clone();
                                    e_click.update(cx, |t, cx| {
                                        t.app.select(Some(domain::Range::new(pos, 1)));
                                        t.app.scroll_to = Some(pos);
                                        t.app.toast(format!("{nm} cut at {}", pos + 1));
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
                                        check_box(SharedString::from(format!("ev-{idx}")), in_set)
                                            .on_click(move |_, _, cx| {
                                                cx.stop_propagation();
                                                let nm = name_tog.clone();
                                                e_tog.update(cx, |t, cx| {
                                                    t.app.mutate(
                                                        "Change enzyme set",
                                                        Vec::new(),
                                                        |doc| {
                                                            if let Some(p) = doc
                                                                .chosen_enzymes
                                                                .iter()
                                                                .position(|n| *n == nm)
                                                            {
                                                                doc.chosen_enzymes.remove(p);
                                                            } else {
                                                                doc.chosen_enzymes.push(nm.clone());
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
                                    .w(px(120.0))
                                    .flex_shrink_0()
                                    .flex()
                                    .items_center()
                                    .px(px(8.0))
                                    .overflow_hidden()
                                    .child(
                                        div()
                                            .font_weight(gpui::FontWeight::SEMIBOLD)
                                            .text_size(px(theme::T_SM))
                                            .text_color(name_col)
                                            .child(name_fitted),
                                    )
                                    .into_any_element(),
                            );
                        }
                        if !hid.contains(&2) {
                            cells.push(
                                cell(window, &r.site, 140.0, theme::ACCENT, true)
                                    .into_any_element(),
                            );
                        }
                        if !hid.contains(&3) {
                            cells.push(
                                div()
                                    .h_full()
                                    .w(px(64.0))
                                    .flex_shrink_0()
                                    .flex()
                                    .items_center()
                                    .px(px(8.0))
                                    .child(
                                        div()
                                            .font_family(theme::MONO_FONT)
                                            .font_weight(gpui::FontWeight::SEMIBOLD)
                                            .text_size(px(theme::T_SM))
                                            .text_color(cuts_col)
                                            .child(n_cuts.to_string()),
                                    )
                                    .into_any_element(),
                            );
                        }
                        if !hid.contains(&4) {
                            cells.push(
                                cell(window, &r.overhang, 150.0, theme::MUTED, false)
                                    .into_any_element(),
                            );
                        }
                        if !hid.contains(&5) {
                            cells.push(
                                cell(window, &r.positions, pos_w, pos_col, true).into_any_element(),
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
        if let Some(popup) = self.render_enzyme_combo(cx) {
            root = root.child(popup);
        }
        if let Some(p) = crate::views::table_opts::render_table_opts(self, cx, "enzymes", COLS) {
            root = root.child(p);
        }
        root.into_any_element()
    }

    fn render_enzyme_combo(&mut self, cx: &mut Context<Self>) -> Option<AnyElement> {
        let kind = self.combo?;
        let mut rows: Vec<AnyElement> = Vec::new();
        match kind {
            ComboKind::Set => {
                let sets = self.app.analysis.sets.clone();
                let current = current_set_name(&self.app, &sets);
                for (i, s) in sets.into_iter().enumerate() {
                    let n_cutting = s
                        .enzymes
                        .iter()
                        .filter(|n| self.app.analysis.counts.get(*n).copied().unwrap_or(0) > 0)
                        .count();
                    let label = format!("{}  ({})", s.name, n_cutting);
                    let names = s.enzymes.clone();
                    let nm = s.name.clone();
                    let sel = current == s.name;
                    rows.push(
                        widgets::menu_check(
                            SharedString::from(format!("combo-set-{i}")),
                            &label,
                            sel,
                        )
                        .on_click(cx.listener(move |t, _, _, cx| {
                            t.combo = None;
                            t.app
                                .mutate(&format!("Use enzyme set \"{nm}\""), Vec::new(), |doc| {
                                    doc.chosen_enzymes = names.clone()
                                });
                            t.app.toast(format!("Enzyme set: {nm}"));
                            cx.notify();
                        }))
                        .into_any_element(),
                    );
                }
            }
            ComboKind::CutCount => {
                for (i, opt) in CUT_COUNT_OPTIONS.iter().enumerate() {
                    let sel = self.app.enzyme_filter.cut_count == i;
                    rows.push(
                        widgets::menu_check(SharedString::from(format!("combo-cc-{i}")), opt, sel)
                            .on_click(cx.listener(move |t, _, _, cx| {
                                t.combo = None;
                                t.app.enzyme_filter.cut_count = i;
                                cx.notify();
                            }))
                            .into_any_element(),
                    );
                }
            }
            ComboKind::Overhang => {
                for (i, opt) in OVERHANG_OPTIONS.iter().enumerate() {
                    let sel = self.app.enzyme_filter.overhang == i;
                    rows.push(
                        widgets::menu_check(SharedString::from(format!("combo-oh-{i}")), opt, sel)
                            .on_click(cx.listener(move |t, _, _, cx| {
                                t.combo = None;
                                t.app.enzyme_filter.overhang = i;
                                cx.notify();
                            }))
                            .into_any_element(),
                    );
                }
            }
            ComboKind::SiteLen => {
                for (i, opt) in SITE_LEN_OPTIONS.iter().enumerate() {
                    let sel = self.app.enzyme_filter.site_len == i;
                    rows.push(
                        widgets::menu_check(SharedString::from(format!("combo-sl-{i}")), opt, sel)
                            .on_click(cx.listener(move |t, _, _, cx| {
                                t.combo = None;
                                t.app.enzyme_filter.site_len = i;
                                cx.notify();
                            }))
                            .into_any_element(),
                    );
                }
            }
            ComboKind::Region => {
                for (i, opt) in REGION_OPTIONS.iter().enumerate() {
                    let sel = self.app.enzyme_filter.region == i;
                    rows.push(
                        widgets::menu_check(SharedString::from(format!("combo-rg-{i}")), opt, sel)
                            .on_click(cx.listener(move |t, _, _, cx| {
                                t.combo = None;
                                t.app.enzyme_filter.region = i;
                                cx.notify();
                            }))
                            .into_any_element(),
                    );
                }
            }
        }

        let mut card = div()
            .id("enzyme-combo")
            .flex()
            .flex_col()
            .min_w(px(210.0))
            .max_w(px(320.0))
            .py(px(4.0))
            .bg(theme::OVERLAY)
            .border_1()
            .border_color(theme::BORDER_STRONG)
            .rounded(px(8.0))
            .shadow(theme::shadow_menu())
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|_, _, _, cx| cx.stop_propagation()),
            )
            .on_mouse_down_out(cx.listener(|t, _, _, cx| {
                t.combo = None;
                cx.notify();
            }));
        for r in rows {
            card = card.child(r);
        }
        Some(
            deferred(
                anchored()
                    .anchor(Corner::TopLeft)
                    .position(self.combo_anchor)
                    .offset(point(px(-4.0), px(8.0)))
                    .snap_to_window()
                    .child(card),
            )
            .with_priority(45)
            .into_any_element(),
        )
    }
}

fn current_set_name(app: &AppState, sets: &[EnzymeSet]) -> String {
    let mut chosen = app.doc.chosen_enzymes.clone();
    chosen.sort();
    for s in sets {
        let mut names = s.enzymes.clone();
        names.sort();
        if names == chosen {
            return s.name.clone();
        }
    }
    "Custom".to_string()
}

fn passes_filter(app: &AppState, e: &Enzyme, n: usize) -> bool {
    let f = &app.enzyme_filter;
    if !f.name.is_empty()
        && !e
            .name
            .to_ascii_lowercase()
            .contains(&f.name.to_ascii_lowercase())
    {
        return false;
    }
    let ok = match f.cut_count {
        0 => true,
        1 => n == 0,
        2 => n == 1,
        3 => n == 2,
        4 => n <= 2,
        _ => n >= 3,
    };
    if !ok {
        return false;
    }
    let outside = e.cut_top < 0 || e.cut_top > e.site_len() as i32;
    let ok = match f.overhang {
        0 => true,
        1 => matches!(e.overhang(), Overhang::FivePrime(_)) && !outside,
        2 => matches!(e.overhang(), Overhang::ThreePrime(_)) && !outside,
        3 => matches!(e.overhang(), Overhang::Blunt) && !outside,
        _ => outside,
    };
    if !ok {
        return false;
    }
    let sl = e.site_len();
    let len_ok = match f.site_len {
        0 => true,
        1 => sl == 4,
        2 => sl == 5,
        3 => sl == 6,
        _ => sl >= 7,
    };
    if !len_ok {
        return false;
    }
    if let Some(r) = app.enzyme_region() {
        let empty = Vec::new();
        let sites = app.analysis.all_sites.get(&e.name).unwrap_or(&empty);
        let end = r.start + r.len;
        let in_region = sites.iter().any(|s| {
            (s.cut_top >= r.start && s.cut_top < end) || (s.position >= r.start && s.position < end)
        });
        if !in_region {
            return false;
        }
    }
    true
}

fn filtered_enzymes(app: &AppState) -> Vec<(Enzyme, usize)> {
    enzyme_db::all_enzymes()
        .into_iter()
        .filter_map(|e| {
            let n = app.analysis.counts.get(&e.name).copied().unwrap_or(0);
            if passes_filter(app, &e, n) {
                Some((e, n))
            } else {
                None
            }
        })
        .collect()
}

fn filtered_names(app: &AppState) -> Vec<String> {
    filtered_enzymes(app)
        .into_iter()
        .map(|(e, _)| e.name)
        .collect()
}
