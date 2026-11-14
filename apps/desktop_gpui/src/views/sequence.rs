//! Sequence view: annotated double-stranded rows with zoom, selection, search
//! highlights and a palette-matched scrollbar. Geometry is ported from the egui
//! shell; rendering runs on a GPUI canvas from an owned per-frame snapshot.

use gpui::{
    canvas, div, point, prelude::*, px, size, AnyElement, App, Bounds, Context, Div, Hsla,
    MouseButton, MouseDownEvent, MouseMoveEvent, MouseUpEvent, Pixels, Point, ScrollWheelEvent,
    Window,
};

use appcore::state::{AppState, ColorsMode, SearchMode};
use domain::genetic_code::{code_by_id, GeneticCode};
use domain::sequence::complement_base;
use domain::{Direction, Feature, Orf, Range, Strand};

use crate::app::HelixBench;
use crate::draw::{self, Align, PxVal};
use crate::theme;
use crate::widgets::{self, Btn};

const BASES_PER_ROW: usize = 60;
const GROUP: usize = 10;
const GROUP_GAP: f32 = 7.0;
const GUTTER: f32 = 62.0;
const CW: f32 = 9.2;
const BASE_H: f32 = 17.0;
const COMP_H: f32 = 15.0;
const FEAT_LANE: f32 = 19.0;
const ORF_LANE: f32 = 15.0;
const ENZ_ROW: f32 = 15.0;
const TRANS_ROW: f32 = 15.0;
const PRIMER_LANE: f32 = 17.0;
const ROW_GAP: f32 = 13.0;

fn base_x(i: usize) -> f32 {
    GUTTER + i as f32 * CW + (i / GROUP) as f32 * GROUP_GAP
}

fn row_width() -> f32 {
    base_x(BASES_PER_ROW) + 12.0
}

struct LaneAssign {
    feat_lanes: Vec<(u64, usize)>,
    orf_top: Vec<(usize, usize)>,
    orf_bot: Vec<(usize, usize)>,
    primer_lanes: Vec<(u64, usize)>,
    translated: Vec<u64>,
}

fn intervals(start: usize, len: usize, l: usize, circular: bool) -> Vec<(usize, usize)> {
    if len == 0 {
        return Vec::new();
    }
    if circular && start + len > l {
        vec![(start, l), (0, start + len - l)]
    } else {
        vec![(start, start + len)]
    }
}

fn assign_lanes(app: &AppState) -> LaneAssign {
    let l = app.doc.sequence.len();
    let circular = app.doc.sequence.is_circular();
    let mut assign = LaneAssign {
        feat_lanes: Vec::new(),
        orf_top: Vec::new(),
        orf_bot: Vec::new(),
        primer_lanes: Vec::new(),
        translated: Vec::new(),
    };
    let place = |ivs: Vec<(usize, usize)>, ends: &mut Vec<usize>| -> usize {
        let mut lane = 0;
        'outer: loop {
            while ends.len() <= lane {
                ends.push(0);
            }
            for &(a, _) in &ivs {
                if ends[lane] > a {
                    lane += 1;
                    continue 'outer;
                }
            }
            for &(_, b) in &ivs {
                ends[lane] = ends[lane].max(b + 2);
            }
            return lane;
        }
    };
    if app.prefs.show_features {
        let mut ends = Vec::new();
        let mut feats: Vec<&Feature> = app
            .doc
            .features
            .iter()
            .filter(|f| f.visible && !f.segments.is_empty())
            .collect();
        feats.sort_by_key(|f| f.span(l).start);
        for f in feats {
            let span = f.span(l);
            let lane = place(intervals(span.start, span.len, l, circular), &mut ends);
            assign.feat_lanes.push((f.id, lane));
            if f.translated {
                assign.translated.push(f.id);
            }
        }
    }
    if app.prefs.show_orfs {
        let mut ends_t = Vec::new();
        let mut ends_b = Vec::new();
        for (i, orf) in app.analysis.orfs.iter().enumerate() {
            let ivs = intervals(orf.start, orf.len, l, circular);
            if orf.strand == Strand::Top {
                let lane = place(ivs, &mut ends_t);
                assign.orf_top.push((i, lane));
            } else {
                let lane = place(ivs, &mut ends_b);
                assign.orf_bot.push((i, lane));
            }
        }
    }
    if app.prefs.show_primers {
        let mut ends = Vec::new();
        for p in &app.doc.primers {
            if !p.visible {
                continue;
            }
            let Some(sites) = app.analysis.primer_sites.get(&p.id) else {
                continue;
            };
            let mut ivs = Vec::new();
            for s in sites {
                ivs.extend(intervals(s.start, s.len.max(1), l, circular));
            }
            if ivs.is_empty() {
                continue;
            }
            let lane = place(ivs, &mut ends);
            assign.primer_lanes.push((p.id, lane));
        }
    }
    if !app.prefs.show_translations {
        assign.translated.clear();
    }
    assign
}

/// Center top-strand position of each codon of a feature, in translation order.
fn codon_positions(f: &Feature, l: usize, circular: bool) -> Vec<usize> {
    let mut out = Vec::new();
    let mut skip = f.frame_offset.min(2);
    if f.direction == Direction::Reverse {
        for seg in f.segments.iter().rev() {
            let ivs = intervals(seg.start, seg.len, l, circular);
            for (a, b) in ivs.into_iter().rev() {
                let mut p = b as i64 - 1 - skip as i64;
                skip = 0;
                while p >= a as i64 + 2 {
                    out.push((p - 1) as usize);
                    p -= 3;
                }
            }
        }
    } else {
        for seg in &f.segments {
            let ivs = intervals(seg.start, seg.len, l, circular);
            for (a, b) in ivs {
                let mut p = a + skip;
                skip = 0;
                while p + 3 <= b {
                    out.push(p + 1);
                    p += 3;
                }
            }
        }
    }
    out
}

// --- Owned snapshot used by the paint closure -------------------------------

struct PrimerSeqDraw {
    id: u64,
    name: String,
    color: Hsla,
    seq_len: usize,
    sites: Vec<(usize, usize, bool)>, // start, len, top strand
}

/// Per-row track layout: rows only reserve the lanes they actually use, so
/// empty stretches of sequence stay compact instead of inheriting the tallest
/// row's annotation stack.
#[derive(Clone)]
struct RowLayout {
    y: f32,
    h: f32,
    top_h: f32,
    mid_h: f32,
    feat: Vec<(u64, usize)>,
    orf_top: Vec<(usize, usize)>,
    orf_bot: Vec<(usize, usize)>,
    primers: Vec<(u64, usize)>,
    trans: Vec<usize>,
    enz: bool,
}

fn overlaps(a: &[(usize, usize)], b: &[(usize, usize)]) -> bool {
    a.iter()
        .any(|&(a0, a1)| b.iter().any(|&(b0, b1)| a0 < b1 && b0 < a1))
}

/// Remap the global lanes used inside one row to a compact local stack.
fn compact<T: Copy>(entries: &[(T, usize)], keep: impl Fn(T) -> bool) -> Vec<(T, usize)> {
    let mut used: Vec<usize> = entries
        .iter()
        .filter(|(id, _)| keep(*id))
        .map(|(_, l)| *l)
        .collect();
    used.sort_unstable();
    used.dedup();
    entries
        .iter()
        .filter(|(id, _)| keep(*id))
        .map(|(id, l)| (*id, used.iter().position(|u| u == l).unwrap_or(0)))
        .collect()
}

pub struct SeqDraw {
    l: usize,
    circular: bool,
    seq: domain::DnaSequence,
    positions: Vec<usize>,
    disp_len: usize,
    n_rows: usize,
    features: Vec<Feature>,
    orfs: Vec<Orf>,
    primers: Vec<PrimerSeqDraw>,
    translations: Vec<(u64, Vec<u8>, Vec<usize>)>,
    regions: Vec<(Range, Hsla)>,
    dna_hits: Vec<Range>,
    prot_hits: Vec<Range>,
    hit_current: usize,
    sel: Option<Range>,
    selected_feature: Option<u64>,
    enz_marks: Vec<(usize, usize, String, usize)>, // pos, len, name, stagger
    show_complement: bool,
    show_features: bool,
    show_orfs: bool,
    show_primers: bool,
    colorize: bool,
    rows: Vec<RowLayout>,
    total_h: f32,
}

impl SeqDraw {
    pub fn build(app: &AppState) -> Self {
        let l = app.doc.sequence.len();
        let circular = app.doc.sequence.is_circular();
        let zoom = app.prefs.show_zoom_controls && app.zoom_width < l;
        let (disp_start, disp_len) = if zoom {
            (app.zoom_start % l, app.zoom_width.min(l))
        } else {
            (0, l)
        };
        let positions: Vec<usize> = (0..disp_len).map(|i| (disp_start + i) % l).collect();
        let n_rows = disp_len.div_ceil(BASES_PER_ROW);
        let lanes = assign_lanes(app);
        let any_enzymes = app.prefs.show_enzymes && !app.analysis.chosen.is_empty();

        let regions = app
            .history_regions()
            .iter()
            .map(|r| {
                (
                    r.range,
                    theme::rgb_arr_a(app.region_rgb(r), app.region_alpha()),
                )
            })
            .collect();
        let dna_hits: Vec<Range> = if app.search.open && app.search.mode == SearchMode::Dna {
            app.search
                .dna_hits
                .iter()
                .map(|h| Range::new(h.start, h.len))
                .collect()
        } else {
            Vec::new()
        };
        let prot_hits: Vec<Range> = if app.search.open && app.search.mode == SearchMode::Protein {
            app.search
                .protein_hits
                .iter()
                .map(|h| Range::new(h.start, h.len))
                .collect()
        } else {
            Vec::new()
        };

        let mut enz_marks: Vec<(usize, usize, String, usize)> = Vec::new();
        if any_enzymes {
            let mut sorted: Vec<(usize, usize, String)> = Vec::new();
            for (e, sites) in &app.analysis.chosen {
                for s in sites {
                    sorted.push((s.position, e.site_len(), e.name.clone()));
                }
            }
            sorted.sort_by_key(|(p, _, _)| *p);
            let mut last_end = [usize::MAX / 2, usize::MAX / 2];
            for (p, n, name) in sorted {
                let stagger = if p >= last_end[0] {
                    0
                } else if p >= last_end[1] {
                    1
                } else {
                    0
                };
                last_end[stagger] = p + name.len() * 6 / CW as usize + 2;
                enz_marks.push((p, n, name, stagger));
            }
        }

        let features: Vec<Feature> = app
            .doc
            .features
            .iter()
            .filter(|f| f.visible && !f.segments.is_empty())
            .cloned()
            .collect();
        let code: &'static GeneticCode = code_by_id(app.doc.default_genetic_code);
        let translations: Vec<(u64, Vec<u8>, Vec<usize>)> = lanes
            .translated
            .iter()
            .filter_map(|fid| {
                let f = app.doc.feature(*fid)?;
                Some((
                    *fid,
                    f.translation(&app.doc.sequence, code),
                    codon_positions(f, l, circular),
                ))
            })
            .collect();
        let primers: Vec<PrimerSeqDraw> = app
            .doc
            .primers
            .iter()
            .filter(|p| p.visible)
            .filter_map(|p| {
                let sites = app.analysis.primer_sites.get(&p.id)?;
                Some(PrimerSeqDraw {
                    id: p.id,
                    name: p.name.clone(),
                    color: theme::rgb_arr(p.color),
                    seq_len: p.sequence.len(),
                    sites: sites
                        .iter()
                        .map(|s| (s.start, s.len, s.strand == Strand::Top))
                        .collect(),
                })
            })
            .collect();

        let mut rows: Vec<RowLayout> = Vec::with_capacity(n_rows);
        let mut y = 0.0f32;
        let comp_h = if app.prefs.show_complement {
            COMP_H
        } else {
            0.0
        };
        for row in 0..n_rows {
            let i0 = row * BASES_PER_ROW;
            let i1 = (i0 + BASES_PER_ROW).min(disp_len);
            let rivs = intervals(positions[i0], i1 - i0, l, circular);
            let feat_ivs = |fid: u64| -> Vec<(usize, usize)> {
                features
                    .iter()
                    .find(|f| f.id == fid)
                    .map(|f| {
                        let span = f.span(l);
                        intervals(span.start, span.len, l, circular)
                    })
                    .unwrap_or_default()
            };
            let feat = compact(&lanes.feat_lanes, |fid| overlaps(&feat_ivs(fid), &rivs));
            let orf_top = compact(&lanes.orf_top, |oi| {
                let o = &app.analysis.orfs[oi];
                overlaps(&intervals(o.start, o.len, l, circular), &rivs)
            });
            let orf_bot = compact(&lanes.orf_bot, |oi| {
                let o = &app.analysis.orfs[oi];
                overlaps(&intervals(o.start, o.len, l, circular), &rivs)
            });
            let primers = compact(&lanes.primer_lanes, |pid| {
                primers
                    .iter()
                    .find(|p| p.id == pid)
                    .map(|p| {
                        p.sites.iter().any(|&(s, len, _)| {
                            overlaps(&intervals(s, len.max(1), l, circular), &rivs)
                        })
                    })
                    .unwrap_or(false)
            });
            let trans: Vec<usize> = translations
                .iter()
                .enumerate()
                .filter(|(_, (_, _, cps))| {
                    cps.iter()
                        .any(|&cp| rivs.iter().any(|&(a, b)| (a..b).contains(&cp)))
                })
                .map(|(i, _)| i)
                .collect();
            let enz = any_enzymes
                && enz_marks
                    .iter()
                    .any(|(p, _, _, _)| rivs.iter().any(|&(a, b)| (a..b).contains(p)));
            let n_feat = feat.iter().map(|(_, l)| l + 1).max().unwrap_or(0);
            let n_ot = orf_top.iter().map(|(_, l)| l + 1).max().unwrap_or(0);
            let n_ob = orf_bot.iter().map(|(_, l)| l + 1).max().unwrap_or(0);
            let n_pr = primers.iter().map(|(_, l)| l + 1).max().unwrap_or(0);
            let top_h = (n_ot + n_ob) as f32 * ORF_LANE
                + n_feat as f32 * FEAT_LANE
                + if enz { ENZ_ROW } else { 0.0 };
            let mid_h = BASE_H + comp_h + trans.len() as f32 * TRANS_ROW;
            let bot_h = n_pr as f32 * PRIMER_LANE;
            let h = top_h + mid_h + bot_h + ROW_GAP;
            rows.push(RowLayout {
                y,
                h,
                top_h,
                mid_h,
                feat,
                orf_top,
                orf_bot,
                primers,
                trans,
                enz,
            });
            y += h;
        }
        let total_h = y + 16.0;

        Self {
            l,
            circular,
            seq: app.doc.sequence.clone(),
            positions,
            disp_len,
            n_rows,
            features,
            orfs: app.analysis.orfs.clone(),
            primers,
            translations,
            regions,
            dna_hits,
            prot_hits,
            hit_current: app.search.current,
            sel: app.selection,
            selected_feature: app.selected_feature,
            enz_marks,
            show_complement: app.prefs.show_complement,
            show_features: app.prefs.show_features,
            show_orfs: app.prefs.show_orfs,
            show_primers: app.prefs.show_primers,
            colorize: app.colors_mode != ColorsMode::None || app.prefs.show_colors,
            rows,
            total_h,
        }
    }

    fn feature(&self, id: u64) -> Option<&Feature> {
        self.features.iter().find(|f| f.id == id)
    }
}

// --- Layout snapshot for hit testing ----------------------------------------

pub struct SeqLayout {
    pub bounds: Bounds<Pixels>,
    pub scroll: f32,
    rows: Vec<RowLayout>,
    disp_len: usize,
    positions: Vec<usize>,
}

impl SeqLayout {
    pub fn hit_base(&self, pos: Point<Pixels>) -> Option<usize> {
        let lx = (pos.x - self.bounds.left()).v();
        let ly = (pos.y - self.bounds.top()).v() + self.scroll - 8.0;
        if lx < GUTTER - 4.0 || ly < 0.0 {
            return None;
        }
        let row = self.rows.partition_point(|r| r.y + r.h <= ly);
        let r = self.rows.get(row)?;
        let in_row = ly - r.y;
        if !(r.top_h..r.top_h + r.mid_h).contains(&in_row) {
            return None;
        }
        let mut i = ((lx - GUTTER) / (CW + GROUP_GAP / GROUP as f32)) as usize;
        while i > 0 && base_x(i) > lx {
            i -= 1;
        }
        while i + 1 < BASES_PER_ROW && base_x(i + 1) <= lx {
            i += 1;
        }
        let idx = row * BASES_PER_ROW + i;
        if idx < self.disp_len {
            Some(idx)
        } else {
            None
        }
    }

    pub fn position(&self, idx: usize) -> Option<usize> {
        self.positions.get(idx).copied()
    }
}

// --- Painting ---------------------------------------------------------------

fn paint(d: &SeqDraw, scroll: f32, bounds: Bounds<Pixels>, window: &mut Window, cx: &mut App) {
    let mono = theme::mono_font();
    let prop = theme::ui_font();
    let width = row_width().max(bounds.size.width.v());

    window.with_content_mask(Some(gpui::ContentMask { bounds }), |window| {
        let ox = bounds.left().v();
        let oy = bounds.top().v() + 8.0 - scroll;
        let pt = |x: f32, y: f32| point(px(x), px(y));

        for row in 0..d.n_rows {
            let rl = &d.rows[row];
            let y = oy + rl.y;
            if y + rl.h < bounds.top().v() || y > bounds.bottom().v() {
                continue;
            }
            let i0 = row * BASES_PER_ROW;
            let i1 = (i0 + BASES_PER_ROW).min(d.disp_len);
            let row_pos = &d.positions[i0..i1];
            let first = row_pos[0];
            let top_h = rl.top_h;
            let base_y = y + top_h;
            let comp_y = base_y + BASE_H;

            // Position gutter
            draw::text(
                window,
                cx,
                pt(ox + GUTTER - 10.0, y + top_h + BASE_H / 2.0),
                &format!("{}", first + 1),
                11.5,
                theme::FAINT,
                mono.clone(),
                Align::Right,
            );

            // Per-base backgrounds: regions, search hits, selection
            for (i, &p) in row_pos.iter().enumerate() {
                let x = ox + base_x(i);
                let cell = Bounds::new(
                    pt(x, base_y),
                    size(
                        px(CW - 0.6),
                        px(BASE_H + if d.show_complement { COMP_H } else { 0.0 }),
                    ),
                );
                let mut fill: Option<Hsla> = None;
                for (r, col) in &d.regions {
                    if r.contains(p, d.l, d.circular) {
                        fill = Some(*col);
                    }
                }
                for (k, h) in d.dna_hits.iter().enumerate() {
                    if h.contains(p, d.l, d.circular) {
                        fill = Some(if k == d.hit_current {
                            theme::SEARCH_DNA.opacity(0.62)
                        } else {
                            theme::SEARCH_DNA
                        });
                    }
                }
                for (k, h) in d.prot_hits.iter().enumerate() {
                    if h.contains(p, d.l, d.circular) {
                        fill = Some(if k == d.hit_current {
                            theme::SEARCH_PROT.opacity(0.62)
                        } else {
                            theme::SEARCH_PROT
                        });
                    }
                }
                let selected = d
                    .sel
                    .map(|s| s.contains(p, d.l, d.circular))
                    .unwrap_or(false);
                if selected {
                    fill = Some(theme::SELECT_FILL);
                }
                if let Some(c) = fill {
                    draw::rect(window, cell, 1.0, c);
                }
                if selected {
                    draw::rect_border(
                        window,
                        cell,
                        1.0,
                        theme::transparent(),
                        0.5,
                        theme::SELECT.opacity(0.5),
                    );
                }
            }

            // Enzyme recognition-site boxes + labels
            if rl.enz {
                for (p, n, name, stagger) in &d.enz_marks {
                    let Some(i) = row_pos.iter().position(|&x| x == *p) else {
                        continue;
                    };
                    let x = ox + base_x(i);
                    let w = (*n as f32 * CW).min(ox + base_x(i1 - i0) - x);
                    let b = Bounds::new(pt(x, base_y - 1.0), size(px(w), px(BASE_H + 2.0)));
                    draw::rect(window, b, 2.0, theme::ACCENT.opacity(0.10));
                    draw::rect_border(
                        window,
                        b,
                        2.0,
                        theme::transparent(),
                        0.75,
                        theme::ACCENT.opacity(0.45),
                    );
                    let ly = y + top_h - ENZ_ROW + 2.0;
                    let lx = if *stagger == 1 { x + w + 3.0 } else { x + 1.0 };
                    draw::text(
                        window,
                        cx,
                        pt(lx, ly + 6.0),
                        name,
                        10.5,
                        theme::ACCENT.opacity(0.9),
                        prop.clone(),
                        Align::Left,
                    );
                }
            }

            // ORF arrows
            if d.show_orfs {
                let n_ot = rl.orf_top.iter().map(|(_, l)| l + 1).max().unwrap_or(0);
                for (oi, lane) in &rl.orf_top {
                    draw_orf_arrow(
                        d,
                        window,
                        cx,
                        ox,
                        y + *lane as f32 * ORF_LANE,
                        *oi,
                        row_pos,
                        theme::ORF_TOP,
                        Strand::Top,
                        &prop,
                    );
                }
                for (oi, lane) in &rl.orf_bot {
                    let ly = y + (n_ot + lane) as f32 * ORF_LANE;
                    draw_orf_arrow(
                        d,
                        window,
                        cx,
                        ox,
                        ly,
                        *oi,
                        row_pos,
                        theme::ORF_BOTTOM,
                        Strand::Bottom,
                        &prop,
                    );
                }
            }

            // Feature arrows
            let n_ot = rl.orf_top.iter().map(|(_, l)| l + 1).max().unwrap_or(0);
            let n_ob = rl.orf_bot.iter().map(|(_, l)| l + 1).max().unwrap_or(0);
            let feat_y0 = y + (n_ot + n_ob) as f32 * ORF_LANE;
            if d.show_features {
                for (fid, lane) in &rl.feat {
                    let Some(f) = d.feature(*fid) else { continue };
                    let fy = feat_y0 + *lane as f32 * FEAT_LANE + FEAT_LANE / 2.0;
                    let col = theme::rgb_arr(f.color);
                    let fill = if d.selected_feature == Some(f.id) {
                        col
                    } else {
                        col.opacity(0.85)
                    };
                    let mut first_seg_x: Option<f32> = None;
                    let mut last_seg_x: Option<f32> = None;
                    for (si, seg) in f.segments.iter().enumerate() {
                        let ivs = intervals(seg.start, seg.len, d.l, d.circular);
                        for (a, b) in ivs {
                            let ia = row_pos.iter().position(|&p| p == a);
                            let ib = row_pos.iter().position(|&p| p == b.saturating_sub(1));
                            let (Some(ia), Some(ib)) = (ia, ib) else {
                                continue;
                            };
                            let x0 = ox + base_x(ia);
                            let x1 = ox + base_x(ib) + CW - 0.6;
                            if x1 - x0 < 2.0 {
                                continue;
                            }
                            first_seg_x.get_or_insert(x0);
                            last_seg_x = Some(x1);
                            let h = 5.5;
                            let head = 7.0_f32.min((x1 - x0) / 2.2);
                            let is_first = si == 0;
                            let is_last = si == f.segments.len() - 1;
                            let head_right = matches!(
                                f.direction,
                                Direction::Forward | Direction::Bidirectional
                            ) && is_last;
                            let head_left = matches!(
                                f.direction,
                                Direction::Reverse | Direction::Bidirectional
                            ) && is_first;
                            let xl = if head_left { x0 + head } else { x0 };
                            let xr = if head_right { x1 - head } else { x1 };
                            let mut pts = vec![pt(xl, fy - h), pt(xr, fy - h)];
                            if head_right {
                                pts.push(pt(x1, fy));
                            }
                            pts.push(pt(xr, fy + h));
                            pts.push(pt(xl, fy + h));
                            if head_left {
                                pts.push(pt(x0, fy));
                            }
                            draw::polygon(window, &pts, fill);
                        }
                    }
                    if let (Some(xa), Some(xb)) = (first_seg_x, last_seg_x) {
                        if f.segments.len() > 1 {
                            draw::segment(window, pt(xa, fy), pt(xb, fy), 1.0, col.opacity(0.5));
                        }
                        let name_w = draw::measure(window, &f.name, 10.5, prop.clone()).v();
                        if xb - xa > name_w + 8.0 {
                            draw::text(
                                window,
                                cx,
                                pt((xa + xb) / 2.0, fy - 8.5),
                                &f.name,
                                10.5,
                                col,
                                prop.clone(),
                                Align::Center,
                            );
                        }
                    }
                }
            }

            // Bases + complement
            for (i, &p) in row_pos.iter().enumerate() {
                let x = ox + base_x(i) + CW / 2.0 - 0.3;
                let b = d.seq.base_at(p);
                let col = if d.colorize {
                    theme::base_color(b)
                } else {
                    theme::TEXT
                };
                draw::text(
                    window,
                    cx,
                    pt(x, base_y + BASE_H / 2.0),
                    &(b as char).to_string(),
                    13.0,
                    col,
                    mono.clone(),
                    Align::Center,
                );
                if d.show_complement {
                    let cb = complement_base(b);
                    let ccol = if d.colorize {
                        theme::base_color(cb).opacity(0.55)
                    } else {
                        theme::MUTED.opacity(0.7)
                    };
                    draw::text(
                        window,
                        cx,
                        pt(x, comp_y + COMP_H / 2.0),
                        &(cb as char).to_string(),
                        11.5,
                        ccol,
                        mono.clone(),
                        Align::Center,
                    );
                }
                if (i + 1) % GROUP == 0 && i + 1 < i1 - i0 {
                    let gx = ox + base_x(i + 1) - GROUP_GAP / 2.0;
                    draw::segment(
                        window,
                        pt(gx, base_y + 2.0),
                        pt(gx, base_y + BASE_H - 2.0),
                        0.5,
                        theme::BORDER,
                    );
                }
            }

            // Translation rows
            let mut ty = comp_y + if d.show_complement { COMP_H } else { 0.0 };
            for &ti in &rl.trans {
                let (fid, aa, cps) = &d.translations[ti];
                let Some(f) = d.feature(*fid) else { continue };
                let col = theme::rgb_arr(f.color).opacity(0.95);
                for (k, &cp) in cps.iter().enumerate() {
                    let Some(i) = row_pos.iter().position(|&p| p == cp) else {
                        continue;
                    };
                    let Some(&a) = aa.get(k) else { continue };
                    let x = ox + base_x(i) + CW / 2.0;
                    let c = if a == b'*' { theme::DANGER } else { col };
                    draw::text(
                        window,
                        cx,
                        pt(x, ty + TRANS_ROW / 2.0),
                        &(a as char).to_string(),
                        11.5,
                        c,
                        mono.clone(),
                        Align::Center,
                    );
                }
                draw::text(
                    window,
                    cx,
                    pt(ox + GUTTER - 10.0, ty + TRANS_ROW / 2.0),
                    &f.name,
                    9.5,
                    theme::rgb_arr(f.color).opacity(0.7),
                    prop.clone(),
                    Align::Right,
                );
                ty += TRANS_ROW;
            }

            // Primer arrows
            let py0 = ty;
            if d.show_primers {
                for (pid, lane) in &rl.primers {
                    let Some(p) = d.primers.iter().find(|p| p.id == *pid) else {
                        continue;
                    };
                    let py = py0 + *lane as f32 * PRIMER_LANE + PRIMER_LANE / 2.0;
                    let col = p.color;
                    for &(start, len, top) in &p.sites {
                        let ia = row_pos.iter().position(|&x| x == start);
                        let end = (start + len).saturating_sub(1) % d.l.max(1);
                        let ib = row_pos.iter().position(|&x| x == end);
                        let (Some(ia), Some(ib)) = (ia, ib) else {
                            continue;
                        };
                        let x0 = ox + base_x(ia);
                        let x1 = ox + base_x(ib) + CW - 0.6;
                        let h = 4.5;
                        let head = 6.0_f32.min((x1 - x0) / 2.0);
                        let pts = if top {
                            vec![
                                pt(x0, py - h),
                                pt(x1 - head, py - h),
                                pt(x1, py),
                                pt(x1 - head, py + h),
                                pt(x0, py + h),
                            ]
                        } else {
                            vec![
                                pt(x0, py),
                                pt(x0 + head, py - h),
                                pt(x1, py - h),
                                pt(x1, py + h),
                                pt(x0 + head, py + h),
                            ]
                        };
                        draw::polygon(window, &pts, col.opacity(0.9));
                        let extra = p.seq_len.saturating_sub(len);
                        if extra > 0 {
                            let tw = extra as f32 * CW;
                            let (tx0, tx1) = if top { (x0 - tw, x0) } else { (x1, x1 + tw) };
                            draw::segment(window, pt(tx0, py), pt(tx1, py), 1.5, col.opacity(0.5));
                        }
                        let name_w = draw::measure(window, &p.name, 9.5, prop.clone()).v();
                        if x1 - x0 > name_w + 6.0 {
                            draw::text(
                                window,
                                cx,
                                pt((x0 + x1) / 2.0, py + 7.5),
                                &p.name,
                                9.5,
                                col,
                                prop.clone(),
                                Align::Center,
                            );
                        }
                    }
                }
            }

            // Row separator
            draw::segment(
                window,
                pt(ox + GUTTER, y + rl.h - ROW_GAP / 2.0),
                pt(ox + width - 12.0, y + rl.h - ROW_GAP / 2.0),
                0.5,
                theme::SURFACE_2,
            );
        }
    });

    // Palette-matched vertical scrollbar
    let rail_w = px(8.0);
    let track = Bounds::new(
        point(bounds.right() - rail_w - px(2.0), bounds.top() + px(2.0)),
        size(rail_w, (bounds.size.height - px(4.0)).max(px(0.0))),
    );
    draw::vscrollbar(window, track, scroll, bounds.size.height.v(), d.total_h);
}

#[allow(clippy::too_many_arguments)]
fn draw_orf_arrow(
    d: &SeqDraw,
    window: &mut Window,
    cx: &mut App,
    ox: f32,
    y: f32,
    oi: usize,
    row_pos: &[usize],
    col: Hsla,
    strand: Strand,
    prop: &gpui::Font,
) {
    let Some(orf) = d.orfs.get(oi) else { return };
    let end = (orf.start + orf.len).saturating_sub(1) % d.l.max(1);
    let ia = row_pos.iter().position(|&p| p == orf.start);
    let ib = row_pos.iter().position(|&p| p == end);
    let (Some(ia), Some(ib)) = (ia, ib) else {
        return;
    };
    let x0 = ox + base_x(ia);
    let x1 = ox + base_x(ib) + CW - 0.6;
    if x1 - x0 < 3.0 {
        return;
    }
    let pt = |x: f32, yy: f32| point(px(x), px(yy));
    let fy = y + ORF_LANE / 2.0;
    let h = 4.0;
    let head = 6.0_f32.min((x1 - x0) / 2.0);
    let pts = if strand == Strand::Top {
        vec![
            pt(x0, fy - h),
            pt(x1 - head, fy - h),
            pt(x1, fy),
            pt(x1 - head, fy + h),
            pt(x0, fy + h),
        ]
    } else {
        vec![
            pt(x0, fy),
            pt(x0 + head, fy - h),
            pt(x1, fy - h),
            pt(x1, fy + h),
            pt(x0 + head, fy + h),
        ]
    };
    draw::polygon(window, &pts, col.opacity(0.75));
    let label = format!("ORF · {} aa", orf.aa_len);
    let label_w = draw::measure(window, &label, 9.0, prop.clone()).v();
    if x1 - x0 > label_w + 6.0 {
        draw::text(
            window,
            cx,
            pt((x0 + x1) / 2.0, fy - 6.5),
            &label,
            9.0,
            col.opacity(0.9),
            prop.clone(),
            Align::Center,
        );
    }
}

// --- Minimap ----------------------------------------------------------------

struct MiniDraw {
    l: usize,
    circular: bool,
    feats: Vec<(usize, usize, Hsla)>,
    enzymes: Vec<usize>,
    sel: Option<Range>,
    zoom_start: usize,
    zoom_width: usize,
}

impl MiniDraw {
    fn build(app: &AppState) -> Self {
        let l = app.doc.sequence.len();
        Self {
            l,
            circular: app.doc.sequence.is_circular(),
            feats: app
                .doc
                .features
                .iter()
                .filter(|f| f.visible)
                .map(|f| {
                    let span = f.span(l);
                    (span.start, span.len, theme::rgb_arr(f.color).opacity(0.8))
                })
                .collect(),
            enzymes: app
                .analysis
                .chosen
                .iter()
                .flat_map(|(_, sites)| sites.iter().map(|s| s.position))
                .collect(),
            sel: app.selection,
            zoom_start: app.zoom_start,
            zoom_width: app.zoom_width,
        }
    }
}

fn paint_mini(d: &MiniDraw, b: Bounds<Pixels>, window: &mut Window) {
    draw::rect_border(window, b, 3.0, theme::BG, 1.0, theme::BORDER);
    if d.l == 0 {
        return;
    }
    let x = |p: usize| b.left().v() + b.size.width.v() * (p as f32 / d.l as f32);
    for (start, len, col) in &d.feats {
        let a = x(*start);
        let bb = x((start + len).min(d.l));
        draw::rect(
            window,
            Bounds::new(
                point(px(a), b.top() + px(3.0)),
                size(px((bb - a).max(1.5)), px(5.0)),
            ),
            1.0,
            *col,
        );
        if d.circular && start + len > d.l {
            let w2 = x(start + len - d.l) - b.left().v();
            draw::rect(
                window,
                Bounds::new(
                    point(b.left(), b.top() + px(3.0)),
                    size(px(w2.max(1.0)), px(5.0)),
                ),
                1.0,
                *col,
            );
        }
    }
    for p in &d.enzymes {
        let ex = x(*p);
        draw::segment(
            window,
            point(px(ex), b.top() + px(10.0)),
            point(px(ex), b.bottom() - px(3.0)),
            0.75,
            theme::ACCENT.opacity(0.5),
        );
    }
    if let Some(sel) = d.sel {
        let a = x(sel.start);
        let bb = x((sel.start + sel.len).min(d.l));
        draw::rect(
            window,
            Bounds::new(
                point(px(a), b.top() + px(10.0)),
                size(px((bb - a).max(1.0)), b.size.height - px(13.0)),
            ),
            1.0,
            theme::SELECT.opacity(0.8),
        );
    }
    if d.zoom_width < d.l {
        let a = x(d.zoom_start);
        let w = b.size.width.v() * (d.zoom_width as f32 / d.l as f32);
        let win = Bounds::new(
            point(px(a), b.top() + px(1.0)),
            size(px(w.min(b.right().v() - a)), b.size.height - px(2.0)),
        );
        draw::rect_border(
            window,
            win,
            2.0,
            theme::TEXT.opacity(0.08),
            1.5,
            theme::TEXT.opacity(0.9),
        );
    }
}

// --- View + interaction -----------------------------------------------------

impl HelixBench {
    pub fn render_sequence(&mut self, cx: &mut Context<Self>) -> AnyElement {
        self.app.ensure_analysis();
        let l = self.app.doc.sequence.len();
        if l == 0 {
            return super::features::empty_state(
                "Empty sequence",
                "Use File ▸ New sequence to begin.",
            );
        }
        let draw = SeqDraw::build(&self.app);

        // Scroll-to request (search navigation): center the target row.
        if let Some(target) = self.app.scroll_to.take() {
            if let Some(idx) = draw.positions.iter().position(|&p| p == target % l) {
                let rl = &draw.rows[idx / BASES_PER_ROW];
                let y = 8.0 + rl.y + rl.h / 2.0;
                let view = self.seq_view_h.max(200.0);
                self.seq_scroll = (y - view / 2.0).clamp(0.0, (draw.total_h - view).max(0.0));
            }
        }
        self.seq_scroll = self
            .seq_scroll
            .clamp(0.0, (draw.total_h - self.seq_view_h).max(0.0));

        let scroll = self.seq_scroll;
        let total_h = draw.total_h;
        let disp_len = draw.disp_len;
        let positions = draw.positions.clone();
        let rows = draw.rows.clone();
        let entity = cx.entity();

        let canvas_el = canvas(
            move |bounds, _window, cx| {
                entity.update(cx, |this, _| {
                    this.seq_view_h = bounds.size.height.v();
                    this.seq_total_h = total_h;
                    this.seq_layout = Some(SeqLayout {
                        bounds,
                        scroll,
                        rows: rows.clone(),
                        disp_len,
                        positions: positions.clone(),
                    });
                });
            },
            move |bounds, _state, window, cx| paint(&draw, scroll, bounds, window, cx),
        )
        .size_full();

        let mut root = div()
            .id("seq-view")
            .size_full()
            .flex()
            .flex_col()
            .bg(theme::BG)
            .child(
                div()
                    .flex_1()
                    .min_h(px(0.0))
                    .overflow_hidden()
                    .child(canvas_el)
                    .on_mouse_down(MouseButton::Left, cx.listener(Self::seq_mouse_down))
                    .on_mouse_move(cx.listener(Self::seq_mouse_move))
                    .on_mouse_up(MouseButton::Left, cx.listener(Self::seq_mouse_up))
                    .on_mouse_up_out(MouseButton::Left, cx.listener(Self::seq_mouse_up))
                    .on_scroll_wheel(cx.listener(Self::seq_wheel)),
            );
        if self.app.prefs.show_zoom_controls {
            root = root.child(self.zoom_bar(cx, l));
        }
        root.into_any_element()
    }

    fn seq_wheel(&mut self, ev: &ScrollWheelEvent, _w: &mut Window, cx: &mut Context<Self>) {
        let delta = ev.delta.pixel_delta(px(BASE_H));
        let max = (self.seq_total_h - self.seq_view_h).max(0.0);
        self.seq_scroll = (self.seq_scroll - delta.y.v()).clamp(0.0, max);
        cx.notify();
    }

    fn seq_mouse_down(&mut self, ev: &MouseDownEvent, _w: &mut Window, cx: &mut Context<Self>) {
        self.seq_press = Some(ev.position);
        self.seq_dragged = false;
        self.app.seq_drag_anchor = self
            .seq_layout
            .as_ref()
            .and_then(|lay| lay.hit_base(ev.position));
        cx.notify();
    }

    fn seq_mouse_move(&mut self, ev: &MouseMoveEvent, _w: &mut Window, cx: &mut Context<Self>) {
        let Some(press) = self.seq_press else { return };
        if (ev.position.x - press.x).v().abs() + (ev.position.y - press.y).v().abs() > 3.0 {
            self.seq_dragged = true;
        }
        if !self.seq_dragged {
            return;
        }
        let Some(lay) = self.seq_layout.as_ref() else {
            return;
        };
        if let (Some(anchor), Some(cur)) = (self.app.seq_drag_anchor, lay.hit_base(ev.position)) {
            let (a, b) = (anchor.min(cur), anchor.max(cur));
            if let Some(start) = lay.position(a) {
                self.app.select(Some(Range::new(start, b - a + 1)));
                cx.notify();
            }
        }
    }

    fn seq_mouse_up(&mut self, ev: &MouseUpEvent, _w: &mut Window, cx: &mut Context<Self>) {
        if self.seq_press.take().is_none() {
            return;
        }
        let was_drag = self.seq_dragged;
        self.seq_dragged = false;
        self.app.seq_drag_anchor = None;
        if !was_drag {
            if let Some(lay) = self.seq_layout.as_ref() {
                if let Some(idx) = lay.hit_base(ev.position) {
                    if let Some(p) = lay.position(idx) {
                        let toggle_off = self
                            .app
                            .selection
                            .map(|s| s.len == 1 && s.start == p)
                            .unwrap_or(false);
                        self.app.select(if toggle_off {
                            None
                        } else {
                            Some(Range::new(p, 1))
                        });
                        self.app.history_highlight = None;
                    }
                }
            }
        }
        cx.notify();
    }

    // --- Zoom bar + minimap --------------------------------------------------

    fn zoom_bar(&mut self, cx: &mut Context<Self>, l: usize) -> Div {
        let zoom_in = {
            let e = cx.entity();
            widgets::icon_only("seq-zoom-in", "zoom-in", Btn::Subtle).on_click(move |_, _, cx| {
                e.update(cx, |t, cx| {
                    t.app.zoom_width = (t.app.zoom_width / 2).max(20);
                    t.app.center_zoom_on_selection();
                    cx.notify();
                });
            })
        };
        let zoom_out = {
            let e = cx.entity();
            widgets::icon_only("seq-zoom-out", "zoom-out", Btn::Subtle).on_click(move |_, _, cx| {
                e.update(cx, |t, cx| {
                    t.app.zoom_width = (t.app.zoom_width * 2).min(l);
                    t.app.center_zoom_on_selection();
                    cx.notify();
                });
            })
        };
        let fit = {
            let e = cx.entity();
            widgets::icon_button("seq-zoom-fit", "fit", "Fit", Btn::Outline).on_click(
                move |_, _, cx| {
                    e.update(cx, |t, cx| {
                        t.app.zoom_width = l;
                        t.app.zoom_start = 0;
                        cx.notify();
                    });
                },
            )
        };
        let center = {
            let e = cx.entity();
            widgets::icon_button(
                "seq-zoom-center",
                "crosshair",
                "Center selection",
                Btn::Outline,
            )
            .on_click(move |_, _, cx| {
                e.update(cx, |t, cx| {
                    t.app.center_zoom_on_selection();
                    cx.notify();
                });
            })
        };

        let mini = MiniDraw::build(&self.app);
        let e = cx.entity();
        let minimap = div()
            .id("seq-mini")
            .h(px(26.0))
            .w_full()
            .cursor_pointer()
            .child(
                canvas(
                    move |bounds, _window, cx| {
                        e.update(cx, |this, _| this.seq_mini_bounds = Some(bounds));
                    },
                    move |bounds, _state, window, _cx| paint_mini(&mini, bounds, window),
                )
                .size_full(),
            )
            .on_mouse_down(MouseButton::Left, cx.listener(Self::mini_down))
            .on_mouse_move(cx.listener(Self::mini_move))
            .on_mouse_up(MouseButton::Left, cx.listener(Self::mini_up))
            .on_mouse_up_out(MouseButton::Left, cx.listener(Self::mini_up));

        div()
            .flex()
            .flex_col()
            .gap(px(6.0))
            .bg(theme::PANEL)
            .border_t_1()
            .border_color(theme::BORDER)
            .px(px(12.0))
            .py(px(8.0))
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(8.0))
                    .child(widgets::label("Zoom", theme::T_SM, theme::MUTED))
                    .child(zoom_in)
                    .child(zoom_out)
                    .child(widgets::divider())
                    .child(widgets::label(
                        &format!("window {} bp", self.app.zoom_width),
                        theme::T_SM,
                        theme::MUTED,
                    ))
                    .child(widgets::label(
                        &format!(
                            "range {}..{}",
                            self.app.zoom_start + 1,
                            self.app.zoom_start + self.app.zoom_width
                        ),
                        theme::T_SM,
                        theme::FAINT,
                    ))
                    .child(div().flex_1())
                    .child(fit)
                    .child(center),
            )
            .child(minimap)
    }

    fn mini_pos(&self, x: Pixels) -> Option<usize> {
        let b = self.seq_mini_bounds?;
        let l = self.app.doc.sequence.len();
        if l == 0 || b.size.width.v() <= 0.0 {
            return None;
        }
        Some((((x - b.left()).v() / b.size.width.v()) * l as f32).clamp(0.0, l as f32) as usize)
    }

    fn mini_down(&mut self, ev: &MouseDownEvent, _w: &mut Window, cx: &mut Context<Self>) {
        let Some(b) = self.seq_mini_bounds else {
            return;
        };
        let l = self.app.doc.sequence.len();
        if l == 0 {
            return;
        }
        self.seq_press = Some(ev.position);
        self.seq_dragged = true;
        let a = b.left().v() + b.size.width.v() * (self.app.zoom_start as f32 / l as f32);
        let bb = a + b.size.width.v() * (self.app.zoom_width as f32 / l as f32);
        let near_left = (ev.position.x.v() - a).abs() < 6.0 && self.app.zoom_width < l;
        let near_right = (ev.position.x.v() - bb).abs() < 6.0 && self.app.zoom_width < l;
        if near_left || near_right {
            self.app.map_drag_anchor = Some(if near_left {
                usize::MAX
            } else {
                usize::MAX - 1
            });
        } else {
            self.app.map_drag_anchor = None;
            if let Some(p) = self.mini_pos(ev.position.x) {
                let p = p.min(l - 1);
                self.app.zoom_start = (p + l - self.app.zoom_width / 2) % l;
            }
        }
        cx.notify();
    }

    fn mini_move(&mut self, ev: &MouseMoveEvent, _w: &mut Window, cx: &mut Context<Self>) {
        if !self.seq_dragged || self.seq_press.is_none() {
            return;
        }
        let l = self.app.doc.sequence.len();
        if l == 0 {
            return;
        }
        let Some(p) = self.mini_pos(ev.position.x) else {
            return;
        };
        match self.app.map_drag_anchor {
            Some(usize::MAX) => {
                let end = self.app.zoom_start + self.app.zoom_width;
                let new_start = p.min(end.saturating_sub(20));
                self.app.zoom_width = end - new_start;
                self.app.zoom_start = new_start % l;
            }
            Some(x) if x == usize::MAX - 1 => {
                self.app.zoom_width = p.saturating_sub(self.app.zoom_start).clamp(20, l);
            }
            _ => {
                self.app.zoom_start = (p + l - self.app.zoom_width / 2) % l;
            }
        }
        cx.notify();
    }

    fn mini_up(&mut self, _ev: &MouseUpEvent, _w: &mut Window, cx: &mut Context<Self>) {
        self.seq_press = None;
        self.seq_dragged = false;
        self.app.map_drag_anchor = None;
        cx.notify();
    }
}
