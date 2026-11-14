//! Sequence view: annotated double-stranded rows with zoom, selection, search.

use egui::{Align2, Color32, FontId, Pos2, Rect, Sense, Shape, Stroke, Vec2};

use domain::{Direction, Range, Strand};

use crate::state::AppState;
use crate::theme::{self, RegionColorExt};

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
    n_feat: usize,
    orf_top: Vec<(usize, usize)>,
    orf_bot: Vec<(usize, usize)>,
    n_orf_top: usize,
    n_orf_bot: usize,
    primer_lanes: Vec<(u64, usize)>,
    n_primer: usize,
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
        n_feat: 0,
        orf_top: Vec::new(),
        orf_bot: Vec::new(),
        n_orf_top: 0,
        n_orf_bot: 0,
        primer_lanes: Vec::new(),
        n_primer: 0,
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
            for &(a, b) in &ivs {
                let _ = a;
                ends[lane] = ends[lane].max(b + 2);
            }
            return lane;
        }
    };
    if app.prefs.show_features {
        let mut ends = Vec::new();
        let mut feats: Vec<&domain::Feature> = app
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
        assign.n_feat = ends.len();
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
        assign.n_orf_top = ends_t.len();
        assign.n_orf_bot = ends_b.len();
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
        assign.n_primer = ends.len();
    }
    if !app.prefs.show_translations {
        assign.translated.clear();
    }
    assign
}

/// Center top-strand position of each codon of a feature, in translation order.
fn codon_positions(f: &domain::Feature, l: usize, circular: bool) -> Vec<usize> {
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

pub fn show(app: &mut AppState, ui: &mut egui::Ui) {
    app.ensure_analysis();
    let l = app.doc.sequence.len();
    if l == 0 {
        ui.centered_and_justified(|ui| {
            ui.label("Empty sequence — use File ▸ New Sequence to begin.");
        });
        return;
    }

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

    let top_h = (lanes.n_orf_top + lanes.n_orf_bot) as f32 * ORF_LANE
        + lanes.n_feat as f32 * FEAT_LANE
        + if any_enzymes { ENZ_ROW } else { 0.0 };
    let mid_h = BASE_H
        + if app.prefs.show_complement {
            COMP_H
        } else {
            0.0
        }
        + lanes.translated.len() as f32 * TRANS_ROW;
    let bot_h = lanes.n_primer as f32 * PRIMER_LANE;
    let row_h = top_h + mid_h + bot_h + ROW_GAP;
    let total_h = n_rows as f32 * row_h + 16.0;
    let width = row_width().max(ui.available_width());

    let scroll = egui::ScrollArea::both()
        .id_salt("seq_scroll")
        .auto_shrink([false, false]);
    let out = scroll.show(ui, |ui| {
        let (rect, resp) =
            ui.allocate_exact_size(Vec2::new(width, total_h), Sense::click_and_drag());
        let painter = ui.painter_at(rect);
        let o = rect.min;
        let clip = ui.clip_rect();
        let mono = FontId::monospace(13.0);
        let mono_small = FontId::monospace(11.5);
        let prop_small = FontId::proportional(10.5);

        let regions = app.history_regions();
        let dna_hits: Vec<Range> =
            if app.search.open && app.search.mode == crate::state::SearchMode::Dna {
                app.search
                    .dna_hits
                    .iter()
                    .map(|h| Range::new(h.start, h.len))
                    .collect()
            } else {
                Vec::new()
            };
        let prot_hits: Vec<Range> =
            if app.search.open && app.search.mode == crate::state::SearchMode::Protein {
                app.search
                    .protein_hits
                    .iter()
                    .map(|h| Range::new(h.start, h.len))
                    .collect()
            } else {
                Vec::new()
            };
        let sel = app.selection;
        let code = domain::genetic_code::code_by_id(app.doc.default_genetic_code);

        // Precompute enzyme site rectangles per position for highlight + labels.
        let mut enz_marks: Vec<(usize, usize, String, usize)> = Vec::new(); // (pos, len, name, stagger)
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

        for row in 0..n_rows {
            let y = o.y + 8.0 + row as f32 * row_h;
            if y + row_h < clip.top() || y > clip.bottom() {
                continue;
            }
            let i0 = row * BASES_PER_ROW;
            let i1 = (i0 + BASES_PER_ROW).min(disp_len);
            let row_pos = &positions[i0..i1];
            let first = row_pos[0];

            // Position gutter
            painter.text(
                Pos2::new(o.x + GUTTER - 10.0, y + top_h + BASE_H / 2.0),
                Align2::RIGHT_CENTER,
                format!("{}", first + 1),
                mono_small.clone(),
                theme::FAINT,
            );

            let base_y = y + top_h;
            let comp_y = base_y + BASE_H;

            // --- per-base backgrounds: selection, regions, search, enzyme sites ---
            for (i, &p) in row_pos.iter().enumerate() {
                let x = o.x + base_x(i);
                let cell = Rect::from_min_size(
                    Pos2::new(x, base_y),
                    Vec2::new(
                        CW - 0.6,
                        BASE_H
                            + if app.prefs.show_complement {
                                COMP_H
                            } else {
                                0.0
                            },
                    ),
                );
                let mut fill: Option<Color32> = None;
                for r in &regions {
                    if r.range.contains(p, l, app.doc.sequence.is_circular()) {
                        fill = Some(app.region_color(r));
                    }
                }
                for (k, h) in dna_hits.iter().enumerate() {
                    if h.contains(p, l, app.doc.sequence.is_circular()) {
                        fill = Some(if k == app.search.current {
                            theme::SEARCH_DNA.gamma_multiply(1.4)
                        } else {
                            theme::SEARCH_DNA
                        });
                    }
                }
                for (k, h) in prot_hits.iter().enumerate() {
                    if h.contains(p, l, app.doc.sequence.is_circular()) {
                        fill = Some(if k == app.search.current {
                            theme::SEARCH_PROT.gamma_multiply(1.4)
                        } else {
                            theme::SEARCH_PROT
                        });
                    }
                }
                if let Some(s) = sel {
                    if s.contains(p, l, app.doc.sequence.is_circular()) {
                        fill = Some(theme::SELECT_FILL);
                    }
                }
                if let Some(c) = fill {
                    painter.rect_filled(cell, 1.0, c);
                }
                if let Some(s) = sel {
                    if s.contains(p, l, app.doc.sequence.is_circular()) {
                        painter.rect_stroke(
                            cell,
                            1.0,
                            Stroke::new(0.5, theme::SELECT.gamma_multiply(0.5)),
                            egui::StrokeKind::Inside,
                        );
                    }
                }
            }

            // Enzyme recognition-site boxes + labels
            if any_enzymes {
                for (p, n, name, stagger) in &enz_marks {
                    let Some(i) = row_pos.iter().position(|&x| x == *p) else {
                        continue;
                    };
                    let x = o.x + base_x(i);
                    let w = (*n as f32 * CW).min(o.x + base_x(i1 - i0) - x);
                    painter.rect_filled(
                        Rect::from_min_size(Pos2::new(x, base_y - 1.0), Vec2::new(w, BASE_H + 2.0)),
                        2.0,
                        theme::TEAL.gamma_multiply(0.10),
                    );
                    painter.rect_stroke(
                        Rect::from_min_size(Pos2::new(x, base_y - 1.0), Vec2::new(w, BASE_H + 2.0)),
                        2.0,
                        Stroke::new(0.75, theme::TEAL.gamma_multiply(0.45)),
                        egui::StrokeKind::Inside,
                    );
                    let ly = y + top_h - ENZ_ROW + 2.0;
                    let lx = if *stagger == 1 { x + w + 3.0 } else { x + 1.0 };
                    painter.text(
                        Pos2::new(lx, ly),
                        Align2::LEFT_TOP,
                        name,
                        prop_small.clone(),
                        theme::TEAL.gamma_multiply(0.9),
                    );
                }
            }

            // --- ORF arrows ---
            if app.prefs.show_orfs {
                for (oi, lane) in &lanes.orf_top {
                    draw_orf_arrow(
                        app,
                        &painter,
                        o,
                        y + *lane as f32 * ORF_LANE,
                        *oi,
                        row_pos,
                        i0,
                        l,
                        theme::ORF_TOP,
                        Strand::Top,
                    );
                }
                for (oi, lane) in &lanes.orf_bot {
                    let ly = y + (lanes.n_orf_top + lane) as f32 * ORF_LANE;
                    draw_orf_arrow(
                        app,
                        &painter,
                        o,
                        ly,
                        *oi,
                        row_pos,
                        i0,
                        l,
                        theme::ORF_BOTTOM,
                        Strand::Bottom,
                    );
                }
            }

            // --- Feature arrows ---
            let feat_y0 = y + (lanes.n_orf_top + lanes.n_orf_bot) as f32 * ORF_LANE;
            if app.prefs.show_features {
                for (fid, lane) in &lanes.feat_lanes {
                    let Some(f) = app.doc.feature(*fid) else {
                        continue;
                    };
                    let fy = feat_y0 + *lane as f32 * FEAT_LANE + FEAT_LANE / 2.0;
                    let col = theme::rgb(f.color);
                    let selected = app.selected_feature == Some(f.id);
                    let fill = if selected {
                        col
                    } else {
                        col.gamma_multiply(0.8)
                    };
                    let mut drew_any = false;
                    let mut first_seg_x: Option<f32> = None;
                    let mut last_seg_x: Option<f32> = None;
                    for seg in &f.segments {
                        let ivs = intervals(seg.start, seg.len, l, app.doc.sequence.is_circular());
                        for (a, b) in ivs {
                            let ia = row_pos.iter().position(|&p| p == a);
                            let ib = row_pos.iter().position(|&p| p == b.saturating_sub(1));
                            let (Some(ia), Some(ib)) = (ia, ib) else {
                                continue;
                            };
                            let x0 = o.x + base_x(ia);
                            let x1 = o.x + base_x(ib) + CW - 0.6;
                            if x1 - x0 < 2.0 {
                                continue;
                            }
                            drew_any = true;
                            first_seg_x.get_or_insert(x0);
                            last_seg_x = Some(x1);
                            let h = 5.5;
                            let head = 7.0_f32.min((x1 - x0) / 2.2);
                            let is_first = seg == f.segments.first().unwrap();
                            let is_last = seg == f.segments.last().unwrap();
                            let head_right = matches!(
                                f.direction,
                                Direction::Forward | Direction::Bidirectional
                            ) && is_last;
                            let head_left = matches!(
                                f.direction,
                                Direction::Reverse | Direction::Bidirectional
                            ) && is_first;
                            let mut pts = Vec::new();
                            let xl = if head_left { x0 + head } else { x0 };
                            let xr = if head_right { x1 - head } else { x1 };
                            pts.push(Pos2::new(xl, fy - h));
                            pts.push(Pos2::new(xr, fy - h));
                            if head_right {
                                pts.push(Pos2::new(x1, fy));
                            }
                            pts.push(Pos2::new(xr, fy + h));
                            pts.push(Pos2::new(xl, fy + h));
                            if head_left {
                                pts.push(Pos2::new(x0, fy));
                            }
                            painter.add(Shape::convex_polygon(pts, fill, Stroke::NONE));
                        }
                    }
                    if let (Some(xa), Some(xb)) = (first_seg_x, last_seg_x) {
                        if f.segments.len() > 1 {
                            painter.line_segment(
                                [Pos2::new(xa, fy), Pos2::new(xb, fy)],
                                Stroke::new(1.0, col.gamma_multiply(0.5)),
                            );
                        }
                        let name_w = f.name.len() as f32 * 6.0;
                        if xb - xa > name_w + 8.0 {
                            painter.text(
                                Pos2::new((xa + xb) / 2.0, fy - 8.5),
                                Align2::CENTER_CENTER,
                                &f.name,
                                prop_small.clone(),
                                col,
                            );
                        }
                        let _ = drew_any;
                    }
                }
            }

            // --- Bases ---
            for (i, &p) in row_pos.iter().enumerate() {
                let x = o.x + base_x(i) + CW / 2.0 - 0.3;
                let b = app.doc.sequence.base_at(p);
                let col = if app.colors_mode == crate::state::ColorsMode::None
                    && !app.prefs.show_colors
                {
                    theme::TEXT
                } else {
                    theme::base_color(b)
                };
                painter.text(
                    Pos2::new(x, base_y + BASE_H / 2.0),
                    Align2::CENTER_CENTER,
                    (b as char).to_string(),
                    mono.clone(),
                    col,
                );
                if app.prefs.show_complement {
                    let cb = domain::sequence::complement_base(b);
                    painter.text(
                        Pos2::new(x, comp_y + COMP_H / 2.0),
                        Align2::CENTER_CENTER,
                        (cb as char).to_string(),
                        mono_small.clone(),
                        theme::base_color(cb).gamma_multiply(0.55),
                    );
                }
                // group separator tick
                if (i + 1) % GROUP == 0 && i + 1 < i1 - i0 {
                    let gx = o.x + base_x(i + 1) - GROUP_GAP / 2.0;
                    painter.line_segment(
                        [
                            Pos2::new(gx, base_y + 2.0),
                            Pos2::new(gx, base_y + BASE_H - 2.0),
                        ],
                        Stroke::new(0.5, theme::BORDER),
                    );
                }
            }

            // --- Translation rows ---
            let mut ty = comp_y
                + if app.prefs.show_complement {
                    COMP_H
                } else {
                    0.0
                };
            for fid in &lanes.translated {
                let Some(f) = app.doc.feature(*fid) else {
                    continue;
                };
                let aa = f.translation(&app.doc.sequence, code);
                let cps = codon_positions(f, l, app.doc.sequence.is_circular());
                let col = theme::rgb(f.color).gamma_multiply(0.95);
                for (k, &cp) in cps.iter().enumerate() {
                    let Some(i) = row_pos.iter().position(|&p| p == cp) else {
                        continue;
                    };
                    let Some(&a) = aa.get(k) else { continue };
                    let x = o.x + base_x(i) + CW / 2.0;
                    let c = if a == b'*' { theme::DANGER } else { col };
                    painter.text(
                        Pos2::new(x, ty + TRANS_ROW / 2.0),
                        Align2::CENTER_CENTER,
                        (a as char).to_string(),
                        mono_small.clone(),
                        c,
                    );
                }
                // row label in gutter
                painter.text(
                    Pos2::new(o.x + GUTTER - 10.0, ty + TRANS_ROW / 2.0),
                    Align2::RIGHT_CENTER,
                    &f.name,
                    FontId::proportional(9.5),
                    theme::rgb(f.color).gamma_multiply(0.7),
                );
                ty += TRANS_ROW;
            }

            // --- Primer arrows ---
            let py0 = ty;
            if app.prefs.show_primers {
                for (pid, lane) in &lanes.primer_lanes {
                    let Some(p) = app.doc.primer(*pid) else {
                        continue;
                    };
                    let Some(sites) = app.analysis.primer_sites.get(pid) else {
                        continue;
                    };
                    let py = py0 + *lane as f32 * PRIMER_LANE + PRIMER_LANE / 2.0;
                    let col = theme::rgb(p.color);
                    for s in sites {
                        let ia = row_pos.iter().position(|&x| x == s.start);
                        let end = (s.start + s.len).saturating_sub(1) % l.max(1);
                        let ib = row_pos.iter().position(|&x| x == end);
                        let (Some(ia), Some(ib)) = (ia, ib) else {
                            continue;
                        };
                        let x0 = o.x + base_x(ia);
                        let x1 = o.x + base_x(ib) + CW - 0.6;
                        let h = 4.5;
                        let head = 6.0_f32.min((x1 - x0) / 2.0);
                        let pts = if s.strand == Strand::Top {
                            vec![
                                Pos2::new(x0, py - h),
                                Pos2::new(x1 - head, py - h),
                                Pos2::new(x1, py),
                                Pos2::new(x1 - head, py + h),
                                Pos2::new(x0, py + h),
                            ]
                        } else {
                            vec![
                                Pos2::new(x0, py),
                                Pos2::new(x0 + head, py - h),
                                Pos2::new(x1, py - h),
                                Pos2::new(x1, py + h),
                                Pos2::new(x0 + head, py + h),
                            ]
                        };
                        painter.add(Shape::convex_polygon(
                            pts,
                            col.gamma_multiply(0.9),
                            Stroke::NONE,
                        ));
                        let extra = p.sequence.len().saturating_sub(s.len);
                        if extra > 0 {
                            let tw = extra as f32 * CW;
                            let (tx0, tx1) = if s.strand == Strand::Top {
                                (x0 - tw, x0)
                            } else {
                                (x1, x1 + tw)
                            };
                            painter.line_segment(
                                [Pos2::new(tx0, py), Pos2::new(tx1, py)],
                                Stroke::new(1.5, col.gamma_multiply(0.5)),
                            );
                        }
                        let name_w = p.name.len() as f32 * 5.8;
                        if x1 - x0 > name_w + 6.0 {
                            painter.text(
                                Pos2::new((x0 + x1) / 2.0, py + 7.5),
                                Align2::CENTER_CENTER,
                                &p.name,
                                FontId::proportional(9.5),
                                col,
                            );
                        }
                    }
                }
            }

            // row separator
            painter.line_segment(
                [
                    Pos2::new(o.x + GUTTER, y + row_h - ROW_GAP / 2.0),
                    Pos2::new(o.x + width - 12.0, y + row_h - ROW_GAP / 2.0),
                ],
                Stroke::new(0.5, theme::SURFACE_2),
            );
        }

        // --- Interaction: drag selection over bases ---
        let show_complement = app.prefs.show_complement;
        let hit_base = |pos: Pos2| -> Option<usize> {
            let lx = pos.x - o.x;
            let ly = pos.y - o.y - 8.0;
            if lx < GUTTER - 4.0 || ly < 0.0 {
                return None;
            }
            let row = (ly / row_h) as usize;
            if row >= n_rows {
                return None;
            }
            let in_row = ly - row as f32 * row_h;
            if !(top_h..top_h + BASE_H + if show_complement { COMP_H } else { 0.0 })
                .contains(&in_row)
            {
                return None;
            }
            // invert base_x: i*cw + (i/10)*gap
            let mut i = ((lx - GUTTER) / (CW + GROUP_GAP / GROUP as f32)) as usize;
            while i > 0 && base_x(i) > lx {
                i -= 1;
            }
            while i + 1 < BASES_PER_ROW && base_x(i + 1) <= lx {
                i += 1;
            }
            let idx = row * BASES_PER_ROW + i;
            if idx < disp_len {
                Some(idx)
            } else {
                None
            }
        };

        if resp.drag_started() {
            if let Some(pp) = resp.interact_pointer_pos() {
                app.seq_drag_anchor = hit_base(pp);
            }
        }
        if resp.dragged() {
            if let (Some(anchor), Some(pp)) = (app.seq_drag_anchor, resp.interact_pointer_pos()) {
                if let Some(cur) = hit_base(pp) {
                    let (a, b) = (anchor.min(cur), anchor.max(cur));
                    let start = positions[a];
                    app.select(Some(Range::new(start, b - a + 1)));
                }
            }
        }
        if resp.drag_stopped() {
            app.seq_drag_anchor = None;
        }
        if resp.clicked() {
            if let Some(pp) = resp.interact_pointer_pos() {
                if let Some(idx) = hit_base(pp) {
                    let p = positions[idx];
                    if app
                        .selection
                        .map(|s| s.len == 1 && s.start == p)
                        .unwrap_or(false)
                    {
                        app.select(None);
                    } else {
                        app.select(Some(Range::new(p, 1)));
                    }
                    app.history_highlight = None;
                }
            }
        }

        // --- Scroll-to request (search navigation) ---
        if let Some(target) = app.scroll_to.take() {
            if let Some(idx) = positions.iter().position(|&p| p == target % l) {
                let row = idx / BASES_PER_ROW;
                let y = o.y + 8.0 + row as f32 * row_h;
                ui.scroll_to_rect(
                    Rect::from_min_size(Pos2::new(o.x, y - 20.0), Vec2::new(width, row_h + 40.0)),
                    Some(egui::Align::Center),
                );
            }
        }
    });
    let _ = out;

    if app.prefs.show_zoom_controls {
        zoom_controls(app, ui, l);
    }
}

#[allow(clippy::too_many_arguments)]
fn draw_orf_arrow(
    app: &AppState,
    painter: &egui::Painter,
    o: Pos2,
    y: f32,
    oi: usize,
    row_pos: &[usize],
    _i0: usize,
    l: usize,
    col: Color32,
    strand: Strand,
) {
    let Some(orf) = app.analysis.orfs.get(oi) else {
        return;
    };
    let end = (orf.start + orf.len).saturating_sub(1) % l.max(1);
    let ia = row_pos.iter().position(|&p| p == orf.start);
    let ib = row_pos.iter().position(|&p| p == end);
    let (Some(ia), Some(ib)) = (ia, ib) else {
        return;
    };
    let x0 = o.x + base_x(ia);
    let x1 = o.x + base_x(ib) + CW - 0.6;
    if x1 - x0 < 3.0 {
        return;
    }
    let fy = y + ORF_LANE / 2.0;
    let h = 4.0;
    let head = 6.0_f32.min((x1 - x0) / 2.0);
    let pts = if strand == Strand::Top {
        vec![
            Pos2::new(x0, fy - h),
            Pos2::new(x1 - head, fy - h),
            Pos2::new(x1, fy),
            Pos2::new(x1 - head, fy + h),
            Pos2::new(x0, fy + h),
        ]
    } else {
        vec![
            Pos2::new(x0, fy),
            Pos2::new(x0 + head, fy - h),
            Pos2::new(x1, fy - h),
            Pos2::new(x1, fy + h),
            Pos2::new(x0 + head, fy + h),
        ]
    };
    painter.add(Shape::convex_polygon(
        pts,
        col.gamma_multiply(0.75),
        Stroke::NONE,
    ));
    let label = format!("ORF · {} aa", orf.aa_len);
    if x1 - x0 > label.len() as f32 * 5.5 + 6.0 {
        painter.text(
            Pos2::new((x0 + x1) / 2.0, fy - 6.5),
            Align2::CENTER_CENTER,
            label,
            FontId::proportional(9.0),
            col.gamma_multiply(0.9),
        );
    }
}

fn zoom_controls(app: &mut AppState, ui: &mut egui::Ui, l: usize) {
    egui::Frame::new()
        .fill(theme::PANEL)
        .stroke(Stroke::new(1.0, theme::BORDER))
        .inner_margin(egui::Margin::symmetric(10, 7))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("🔍").size(13.0));
                if ui
                    .button("−")
                    .on_hover_text("Zoom in (narrower window)")
                    .clicked()
                {
                    app.zoom_width = (app.zoom_width / 2).max(20);
                    app.center_zoom_on_selection();
                }
                if ui
                    .button("+")
                    .on_hover_text("Zoom out (wider window)")
                    .clicked()
                {
                    app.zoom_width = (app.zoom_width * 2).min(l);
                    app.center_zoom_on_selection();
                }
                let mut w = app.zoom_width as f32;
                let slider = egui::Slider::new(&mut w, 20.0..=l as f32)
                    .logarithmic(true)
                    .text("window");
                if ui.add(slider).changed() {
                    app.zoom_width = (w as usize).clamp(20, l);
                }
                ui.separator();
                ui.label("width");
                let mut width = app.zoom_width;
                if ui
                    .add(egui::DragValue::new(&mut width).range(20..=l).speed(5))
                    .changed()
                {
                    app.zoom_width = width.clamp(20, l);
                }
                ui.label("range");
                let mut start = app.zoom_start + 1;
                let mut end = app.zoom_start + app.zoom_width;
                let r1 = ui.add(egui::DragValue::new(&mut start).range(1..=l).speed(2));
                ui.label("..");
                let r2 = ui.add(egui::DragValue::new(&mut end).range(1..=l).speed(2));
                if r1.changed() {
                    app.zoom_start = (start - 1) % l;
                }
                if r2.changed() {
                    let new_w = end.saturating_sub(app.zoom_start).clamp(20, l);
                    app.zoom_width = new_w;
                }
                if ui
                    .button("Fit")
                    .on_hover_text("Show the whole sequence")
                    .clicked()
                {
                    app.zoom_width = l;
                    app.zoom_start = 0;
                }
                if ui.button("Center selection").clicked() {
                    app.center_zoom_on_selection();
                }
            });
            ui.add_space(4.0);
            minimap(app, ui, l);
        });
}

fn minimap(app: &mut AppState, ui: &mut egui::Ui, l: usize) {
    let w = ui.available_width().max(200.0);
    let h = 26.0;
    let (rect, resp) = ui.allocate_exact_size(Vec2::new(w, h), Sense::click_and_drag());
    let painter = ui.painter_at(rect);
    painter.rect_filled(rect, 3.0, theme::BG);
    painter.rect_stroke(
        rect,
        3.0,
        Stroke::new(1.0, theme::BORDER),
        egui::StrokeKind::Inside,
    );
    let x = |p: usize| rect.left() + rect.width() * (p as f32 / l as f32);

    // feature ticks
    for f in &app.doc.features {
        if !f.visible {
            continue;
        }
        let span = f.span(l);
        let col = theme::rgb(f.color).gamma_multiply(0.8);
        let a = x(span.start);
        let b = x((span.start + span.len).min(l));
        painter.rect_filled(
            Rect::from_min_max(
                Pos2::new(a, rect.top() + 3.0),
                Pos2::new(b.max(a + 1.5), rect.top() + 8.0),
            ),
            1.0,
            col,
        );
        if app.doc.sequence.is_circular() && span.start + span.len > l {
            painter.rect_filled(
                Rect::from_min_max(
                    Pos2::new(rect.left(), rect.top() + 3.0),
                    Pos2::new(x(span.start + span.len - l), rect.top() + 8.0),
                ),
                1.0,
                col,
            );
        }
    }
    // enzyme ticks
    for (_, sites) in &app.analysis.chosen {
        for s in sites {
            let ex = x(s.position);
            painter.line_segment(
                [
                    Pos2::new(ex, rect.top() + 10.0),
                    Pos2::new(ex, rect.bottom() - 3.0),
                ],
                Stroke::new(0.75, theme::TEAL.gamma_multiply(0.5)),
            );
        }
    }
    // selection
    if let Some(sel) = app.selection {
        let a = x(sel.start);
        let b = x((sel.start + sel.len).min(l));
        painter.rect_filled(
            Rect::from_min_max(
                Pos2::new(a, rect.top() + 10.0),
                Pos2::new(b.max(a + 1.0), rect.bottom() - 3.0),
            ),
            1.0,
            theme::SELECT.gamma_multiply(0.8),
        );
    }
    // zoom window (white rectangle)
    if app.zoom_width < l {
        let a = x(app.zoom_start);
        let b = a + rect.width() * (app.zoom_width as f32 / l as f32);
        let win = Rect::from_min_max(
            Pos2::new(a, rect.top() + 1.0),
            Pos2::new(b.min(rect.right()), rect.bottom() - 1.0),
        );
        painter.rect_filled(win, 2.0, Color32::WHITE.gamma_multiply(0.08));
        painter.rect_stroke(
            win,
            2.0,
            Stroke::new(1.5, Color32::WHITE.gamma_multiply(0.9)),
            egui::StrokeKind::Inside,
        );
    }

    // drag to pan / resize edges
    let edge = 6.0;
    if resp.drag_started() || resp.clicked() {
        if let Some(pp) = resp.interact_pointer_pos() {
            let a = x(app.zoom_start);
            let b = a + rect.width() * (app.zoom_width as f32 / l as f32);
            let near_left = (pp.x - a).abs() < edge && app.zoom_width < l;
            let near_right = (pp.x - b).abs() < edge && app.zoom_width < l;
            if near_left || near_right {
                app.map_drag_anchor = Some(if near_left {
                    usize::MAX
                } else {
                    usize::MAX - 1
                });
            } else {
                app.map_drag_anchor = None;
                let p = (((pp.x - rect.left()) / rect.width()) * l as f32)
                    .clamp(0.0, (l - 1) as f32) as usize;
                app.zoom_start = (p + l - app.zoom_width / 2) % l;
            }
        }
    }
    if resp.dragged() {
        if let Some(pp) = resp.interact_pointer_pos() {
            let p =
                (((pp.x - rect.left()) / rect.width()) * l as f32).clamp(0.0, l as f32) as usize;
            match app.map_drag_anchor {
                Some(usize::MAX) => {
                    let end = app.zoom_start + app.zoom_width;
                    let new_start = p.min(end.saturating_sub(20));
                    app.zoom_width = end - new_start;
                    app.zoom_start = new_start % l;
                }
                Some(x) if x == usize::MAX - 1 => {
                    app.zoom_width = (p.saturating_sub(app.zoom_start)).clamp(20, l);
                }
                _ => {
                    app.zoom_start = (p + l - app.zoom_width / 2) % l;
                }
            }
        }
    }
    if resp.drag_stopped() {
        app.map_drag_anchor = None;
    }
    if resp.hovered() {
        let a = x(app.zoom_start);
        let b = a + rect.width() * (app.zoom_width as f32 / l as f32);
        if let Some(pp) = resp.hover_pos() {
            if (pp.x - a).abs() < edge || (pp.x - b).abs() < edge {
                resp.ctx.set_cursor_icon(egui::CursorIcon::ResizeHorizontal);
            } else {
                resp.ctx.set_cursor_icon(egui::CursorIcon::Grab);
            }
        }
    }
}
