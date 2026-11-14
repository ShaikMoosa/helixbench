//! Map view: circular plasmid map or linear map with interactive annotations.

use egui::{Align2, Color32, FontId, Pos2, Rect, Sense, Shape, Stroke, Vec2};

use domain::{Direction, Range, Strand, Topology};

use crate::state::AppState;
use crate::theme::{self, RegionColorExt};

const TAU: f32 = std::f32::consts::TAU;

fn polar(center: Pos2, r: f32, angle: f32) -> Pos2 {
    Pos2::new(center.x + r * angle.cos(), center.y + r * angle.sin())
}

fn pos_angle(p: usize, l: usize) -> f32 {
    -std::f32::consts::FRAC_PI_2 + TAU * (p as f32 / l.max(1) as f32)
}

fn angle_pos(angle: f32, l: usize) -> usize {
    let a = (angle + std::f32::consts::FRAC_PI_2).rem_euclid(TAU);
    ((a / TAU) * l as f32).round() as usize % l.max(1)
}

fn arc_points(center: Pos2, r: f32, a0: f32, a1: f32) -> Vec<Pos2> {
    let span = (a1 - a0).abs();
    let steps = ((span / TAU) * 180.0).ceil().max(2.0) as usize;
    (0..=steps)
        .map(|i| polar(center, r, a0 + (a1 - a0) * i as f32 / steps as f32))
        .collect()
}

#[allow(clippy::too_many_arguments)]
fn band_shape(
    center: Pos2,
    r_in: f32,
    r_out: f32,
    a0: f32,
    a1: f32,
    dir: Direction,
    fill: Color32,
    stroke: Stroke,
) -> Shape {
    let r_mid = (r_in + r_out) / 2.0;
    let ha = (11.0 / r_mid).clamp(0.015, 0.14);
    let span = a1 - a0;
    let head = ha.min(span / 2.5);
    let mut pts: Vec<Pos2> = Vec::new();
    let fwd_head = matches!(dir, Direction::Forward | Direction::Bidirectional);
    let rev_head = matches!(dir, Direction::Reverse | Direction::Bidirectional);
    let b0 = if rev_head { a0 + head } else { a0 };
    let b1 = if fwd_head { a1 - head } else { a1 };
    pts.extend(arc_points(center, r_out, b0, b1));
    if fwd_head {
        pts.push(polar(center, r_mid, a1));
    }
    pts.extend(arc_points(center, r_in, b1, b0));
    if rev_head {
        pts.push(polar(center, r_mid, a0));
    }
    Shape::Path(egui::epaint::PathShape {
        points: pts,
        closed: true,
        fill,
        stroke: stroke.into(),
    })
}

enum MapHit {
    Feature(u64),
    Primer(u64),
    Enzyme(usize),
}

pub fn show(app: &mut AppState, ui: &mut egui::Ui) {
    app.ensure_analysis();
    let avail = ui.available_size();
    let (rect, resp) = ui.allocate_exact_size(avail, Sense::click_and_drag());
    let painter = ui.painter_at(rect);
    painter.rect_filled(rect, 0.0, theme::BG);

    let l = app.doc.sequence.len();
    if l == 0 {
        painter.text(
            rect.center(),
            Align2::CENTER_CENTER,
            "Empty sequence — use File ▸ New Sequence to begin.",
            FontId::proportional(14.0),
            theme::MUTED,
        );
        return;
    }

    if app.doc.sequence.topology == Topology::Circular {
        show_circular(app, &painter, rect, &resp);
    } else {
        show_linear(app, &painter, rect, &resp);
    }
}

fn show_circular(app: &mut AppState, painter: &egui::Painter, rect: Rect, resp: &egui::Response) {
    let l = app.doc.sequence.len();
    let center = rect.center();
    let r = (rect.width().min(rect.height()) / 2.0 - 96.0).max(60.0);

    // Depth: faint halo discs behind the map.
    painter.circle_filled(center, r + 58.0, theme::PANEL);
    painter.circle_stroke(center, r + 58.0, Stroke::new(1.0, theme::BORDER));
    painter.circle_filled(center, r + 26.0, theme::SURFACE);

    let r_feat_out = r;
    let r_feat_in = r - 13.0;
    let r_enz = r - 26.0;
    let r_primer = r - 52.0;
    let r_orf_top = r - 76.0;
    let r_orf_bot = r - 88.0;

    // Backbone circle.
    painter.circle_stroke(center, r_enz, Stroke::new(1.5, theme::FAINT));

    let pointer = resp.hover_pos().or(resp.interact_pointer_pos());
    let mut hovered: Option<MapHit> = None;
    let mut hover_tip: Option<String> = None;

    // --- ORF arcs -------------------------------------------------------
    if app.prefs.show_orfs {
        for orf in &app.analysis.orfs {
            let a0 = pos_angle(orf.start, l);
            let a1 = a0 + TAU * orf.len as f32 / l as f32;
            let (rr, col) = if orf.strand == Strand::Top {
                (r_orf_top, theme::ORF_TOP)
            } else {
                (r_orf_bot, theme::ORF_BOTTOM)
            };
            let pts = arc_points(center, rr, a0, a1);
            painter.add(Shape::line(pts, Stroke::new(3.0, col.gamma_multiply(0.85))));
        }
    }

    // --- Enzyme ticks + labels -------------------------------------------
    if app.prefs.show_enzymes {
        let total_sites: usize = app.analysis.chosen.iter().map(|(_, s)| s.len()).sum();
        let label_sites = total_sites <= 42;
        let mut enz_labels: Vec<(f32, String, usize)> = Vec::new();
        for (e, sites) in &app.analysis.chosen {
            for s in sites {
                let a = pos_angle(s.position, l);
                let p0 = polar(center, r_enz - 5.0, a);
                let p1 = polar(center, r_enz + 5.0, a);
                painter.line_segment([p0, p1], Stroke::new(1.5, theme::TEAL));
                if label_sites {
                    enz_labels.push((a, e.name.clone(), s.position));
                }
                if let Some(pp) = pointer {
                    if (pp - polar(center, r_enz, a)).length() < 8.0 {
                        hovered = Some(MapHit::Enzyme(s.position));
                        hover_tip = Some(format!(
                            "{} — cuts at {} ({})",
                            e.name,
                            s.cut_top + 1,
                            e.overhang().label()
                        ));
                    }
                }
            }
        }
        enz_labels.sort_by(|a, b| a.0.total_cmp(&b.0));
        let mut levels: [f32; 3] = [0.0; 3];
        for (a, name, _) in enz_labels {
            let width = name.len() as f32 * 5.6 + 6.0;
            let mut placed = false;
            for (i, last_end) in levels.iter_mut().enumerate() {
                let rr = r_enz - 14.0 - i as f32 * 12.0;
                let half_ang = width / rr / 2.0;
                if a - half_ang > *last_end {
                    let pos = polar(center, rr, a);
                    let align = if a.cos() >= 0.0 {
                        Align2::LEFT_CENTER
                    } else {
                        Align2::RIGHT_CENTER
                    };
                    painter.text(pos, align, &name, FontId::proportional(10.5), theme::MUTED);
                    *last_end = a + half_ang;
                    placed = true;
                    break;
                }
            }
            if !placed {
                // innermost fallback, accept overlap rather than hide
                let rr = r_enz - 38.0;
                let pos = polar(center, rr, a);
                let align = if a.cos() >= 0.0 {
                    Align2::LEFT_CENTER
                } else {
                    Align2::RIGHT_CENTER
                };
                painter.text(pos, align, &name, FontId::proportional(10.0), theme::FAINT);
            }
        }
    }

    // --- Primer arrows ----------------------------------------------------
    if app.prefs.show_primers {
        for p in &app.doc.primers {
            if !p.visible {
                continue;
            }
            let Some(sites) = app.analysis.primer_sites.get(&p.id) else {
                continue;
            };
            for s in sites {
                let a0 = pos_angle(s.start, l);
                let a1 = a0 + TAU * s.len as f32 / l as f32;
                let dir = if s.strand == Strand::Top {
                    Direction::Forward
                } else {
                    Direction::Reverse
                };
                let col = theme::rgb(p.color);
                let shape = band_shape(
                    center,
                    r_primer - 4.0,
                    r_primer + 4.0,
                    a0,
                    a1,
                    dir,
                    col.gamma_multiply(0.9),
                    Stroke::NONE,
                );
                painter.add(shape);
                // 5' extension tail
                let extra = p.sequence.len().saturating_sub(s.len);
                if extra > 0 {
                    let (t0, t1) = if s.strand == Strand::Top {
                        (a0 - TAU * extra as f32 / l as f32, a0)
                    } else {
                        (a1, a1 + TAU * extra as f32 / l as f32)
                    };
                    painter.add(Shape::line(
                        arc_points(center, r_primer, t0, t1),
                        Stroke::new(1.5, col.gamma_multiply(0.6)),
                    ));
                }
                let mid = (a0 + a1) / 2.0;
                let lp = polar(center, r_primer - 15.0, mid);
                let align = if mid.cos() >= 0.0 {
                    Align2::LEFT_CENTER
                } else {
                    Align2::RIGHT_CENTER
                };
                painter.text(lp, align, &p.name, FontId::proportional(10.5), col);
                if let Some(pp) = pointer {
                    let d = (pp - center).length();
                    if (d - r_primer).abs() < 9.0 {
                        let pa = (pp - center).angle();
                        let within = angle_in(pa, a0, a1);
                        if within {
                            hovered = Some(MapHit::Primer(p.id));
                            hover_tip = Some(format!(
                                "{} — {} nt, binds {}..{} ({})",
                                p.name,
                                p.sequence.len(),
                                s.start + 1,
                                s.start + s.len,
                                if s.strand == Strand::Top {
                                    "top"
                                } else {
                                    "bottom"
                                }
                            ));
                        }
                    }
                }
            }
        }
    }

    // --- Feature arrows + labels ------------------------------------------
    let mut feat_labels: Vec<(f32, f32, String, u64, Color32)> = Vec::new();
    for f in &app.doc.features {
        if !f.visible || f.segments.is_empty() {
            continue;
        }
        let span = f.span(l);
        let a0 = pos_angle(span.start, l);
        let a1 = a0 + TAU * span.len as f32 / l as f32;
        let col = theme::rgb(f.color);
        let fill = if app.selected_feature == Some(f.id) {
            col
        } else {
            col.gamma_multiply(0.82)
        };
        painter.add(band_shape(
            center,
            r_feat_in,
            r_feat_out,
            a0,
            a1,
            f.direction,
            fill,
            Stroke::new(0.5, theme::BG),
        ));
        // intron gaps: thin connector line across gaps between segments
        if f.segments.len() > 1 {
            painter.add(Shape::line(
                arc_points(center, (r_feat_in + r_feat_out) / 2.0, a0, a1),
                Stroke::new(1.0, col.gamma_multiply(0.5)),
            ));
        }
        let mid = (a0 + a1) / 2.0;
        feat_labels.push((mid, a1 - a0, f.name.clone(), f.id, col));
        if let Some(pp) = pointer {
            let d = (pp - center).length();
            if (r_feat_in - 2.0..=r_feat_out + 2.0).contains(&d) {
                let pa = (pp - center).angle();
                if angle_in(pa, a0, a1) {
                    hovered = Some(MapHit::Feature(f.id));
                    hover_tip = Some(format!(
                        "{} ({})\n{}..{} · {} bp · {}",
                        f.name,
                        f.kind,
                        span.start + 1,
                        span.start + span.len,
                        span.len,
                        f.direction.label()
                    ));
                }
            }
        }
    }
    feat_labels.sort_by(|a, b| a.0.total_cmp(&b.0));
    let mut levels: [f32; 3] = [-10.0; 3];
    for (mid, _span, name, _id, col) in feat_labels {
        let width = name.len() as f32 * 6.4 + 10.0;
        let mut placed = false;
        for (i, last_end) in levels.iter_mut().enumerate() {
            let rr = r + 16.0 + i as f32 * 17.0;
            let half_ang = width / rr / 2.0;
            if mid - half_ang > *last_end {
                let anchor = polar(center, r + 3.0, mid);
                let elbow = polar(center, rr - 6.0, mid);
                let pos = polar(center, rr, mid);
                painter.line_segment([anchor, elbow], Stroke::new(0.8, col.gamma_multiply(0.55)));
                let align = if mid.cos() >= 0.0 {
                    Align2::LEFT_CENTER
                } else {
                    Align2::RIGHT_CENTER
                };
                painter.text(pos, align, &name, FontId::proportional(11.5), col);
                *last_end = mid + half_ang;
                placed = true;
                break;
            }
        }
        if !placed {
            let rr = r + 50.0;
            let pos = polar(center, rr, mid);
            let align = if mid.cos() >= 0.0 {
                Align2::LEFT_CENTER
            } else {
                Align2::RIGHT_CENTER
            };
            painter.text(
                pos,
                align,
                &name,
                FontId::proportional(10.5),
                col.gamma_multiply(0.7),
            );
        }
    }

    // --- Color regions (history / custom) ----------------------------------
    for region in app.history_regions() {
        let a0 = pos_angle(region.range.start, l);
        let a1 = a0 + TAU * region.range.len as f32 / l as f32;
        let col = app.region_color(&region);
        painter.add(Shape::line(
            arc_points(center, r + 6.0, a0, a1),
            Stroke::new(5.0, col),
        ));
    }

    // --- Search hits (blue on the map) -------------------------------------
    let hit_ranges = search_hit_ranges(app);
    for (i, hr) in hit_ranges.iter().enumerate() {
        let a0 = pos_angle(hr.start, l);
        let a1 = a0 + TAU * hr.len as f32 / l as f32;
        let current = i == app.search.current && app.search.hit_count() > 0;
        painter.add(Shape::line(
            arc_points(center, r + 6.0, a0, a1),
            Stroke::new(
                if current { 6.0 } else { 4.0 },
                theme::ACCENT.gamma_multiply(if current { 1.0 } else { 0.65 }),
            ),
        ));
    }

    // --- Selection arc ------------------------------------------------------
    if let Some(sel) = app.selection {
        let a0 = pos_angle(sel.start, l);
        let a1 = a0 + TAU * sel.len as f32 / l as f32;
        painter.add(Shape::line(
            arc_points(center, r + 12.0, a0, a1),
            Stroke::new(4.0, theme::SELECT),
        ));
        for a in [a0, a1] {
            painter.line_segment(
                [polar(center, r + 7.0, a), polar(center, r + 17.0, a)],
                Stroke::new(2.0, theme::SELECT),
            );
        }
    }

    // --- Zoom window (white rectangle on the overview) -----------------------
    if app.prefs.show_zoom_controls && app.zoom_width < l {
        let a0 = pos_angle(app.zoom_start, l);
        let a1 = a0 + TAU * app.zoom_width as f32 / l as f32;
        painter.add(Shape::line(
            arc_points(center, r + 21.0, a0, a1),
            Stroke::new(2.5, Color32::WHITE.gamma_multiply(0.85)),
        ));
    }

    // Origin tick at 12 o'clock.
    painter.line_segment(
        [
            polar(center, r + 24.0, -std::f32::consts::FRAC_PI_2),
            polar(center, r + 30.0, -std::f32::consts::FRAC_PI_2),
        ],
        Stroke::new(1.5, theme::FAINT),
    );

    // --- Center info ----------------------------------------------------------
    let gc = app.doc.sequence.gc_content();
    painter.text(
        center + Vec2::new(0.0, -34.0),
        Align2::CENTER_CENTER,
        &app.doc.name,
        FontId::proportional(16.0),
        theme::TEXT,
    );
    painter.text(
        center + Vec2::new(0.0, -12.0),
        Align2::CENTER_CENTER,
        format!("{} bp · circular", l),
        FontId::proportional(12.5),
        theme::MUTED,
    );
    painter.text(
        center + Vec2::new(0.0, 6.0),
        Align2::CENTER_CENTER,
        format!("GC {:.1}%", gc),
        FontId::proportional(12.0),
        theme::FAINT,
    );
    if let Some(sel) = app.selection {
        painter.text(
            center + Vec2::new(0.0, 28.0),
            Align2::CENTER_CENTER,
            format!(
                "{}..{} · {} bp",
                sel.start + 1,
                sel.start + sel.len,
                sel.len
            ),
            FontId::proportional(12.0),
            theme::ACCENT,
        );
    } else {
        painter.text(
            center + Vec2::new(0.0, 28.0),
            Align2::CENTER_CENTER,
            "drag to select",
            FontId::proportional(11.0),
            theme::FAINT,
        );
    }

    handle_interaction(app, resp, center, Some(r), hovered, hover_tip, rect);
}

fn angle_in(a: f32, a0: f32, a1: f32) -> bool {
    let span = (a1 - a0).rem_euclid(TAU);
    let rel = (a - a0).rem_euclid(TAU);
    rel <= span.max(0.0001)
}

fn show_linear(app: &mut AppState, painter: &egui::Painter, rect: Rect, resp: &egui::Response) {
    let l = app.doc.sequence.len();
    let margin = 48.0;
    let x0 = rect.left() + margin;
    let x1 = rect.right() - margin;
    let w = (x1 - x0).max(50.0);
    let y = rect.center().y;
    let px = |p: usize| x0 + w * (p as f32 / l as f32);

    painter.line_segment(
        [Pos2::new(x0, y), Pos2::new(x1, y)],
        Stroke::new(2.0, theme::FAINT),
    );

    // Ruler
    let step = nice_step(l);
    let mut p = 0;
    while p <= l {
        let x = px(p);
        painter.line_segment(
            [Pos2::new(x, y + 40.0), Pos2::new(x, y + 46.0)],
            Stroke::new(1.0, theme::FAINT),
        );
        painter.text(
            Pos2::new(x, y + 58.0),
            Align2::CENTER_CENTER,
            format!("{}", p),
            FontId::proportional(10.0),
            theme::FAINT,
        );
        p += step;
    }

    let pointer = resp.hover_pos().or(resp.interact_pointer_pos());
    let mut hovered: Option<MapHit> = None;
    let mut hover_tip: Option<String> = None;

    // Features above the line, stacked in lanes.
    let mut lane_ends: Vec<f32> = Vec::new();
    for f in &app.doc.features {
        if !f.visible || f.segments.is_empty() {
            continue;
        }
        let span = f.span(l);
        let fx0 = px(span.start);
        let fx1 = px((span.start + span.len).min(l));
        let mut lane = 0;
        while lane < lane_ends.len() && lane_ends[lane] > fx0 - 4.0 {
            lane += 1;
        }
        if lane == lane_ends.len() {
            lane_ends.push(0.0);
        }
        lane_ends[lane] = fx1 + 8.0;
        let fy = y - 26.0 - lane as f32 * 24.0;
        let col = theme::rgb(f.color);
        let h = 9.0;
        let head = 8.0_f32.min((fx1 - fx0) / 2.5);
        let mut pts = Vec::new();
        match f.direction {
            Direction::Reverse => {
                pts.push(Pos2::new(fx0, fy));
                pts.push(Pos2::new(fx0 + head, fy - h));
                pts.push(Pos2::new(fx1, fy - h));
                pts.push(Pos2::new(fx1, fy + h));
                pts.push(Pos2::new(fx0 + head, fy + h));
            }
            Direction::None => {
                pts.push(Pos2::new(fx0, fy - h));
                pts.push(Pos2::new(fx1, fy - h));
                pts.push(Pos2::new(fx1, fy + h));
                pts.push(Pos2::new(fx0, fy + h));
            }
            _ => {
                pts.push(Pos2::new(fx0, fy - h));
                pts.push(Pos2::new(fx1 - head, fy - h));
                pts.push(Pos2::new(fx1, fy));
                pts.push(Pos2::new(fx1 - head, fy + h));
                pts.push(Pos2::new(fx0, fy + h));
            }
        }
        painter.add(Shape::convex_polygon(
            pts,
            col.gamma_multiply(0.85),
            Stroke::NONE,
        ));
        painter.text(
            Pos2::new((fx0 + fx1) / 2.0, fy - h - 8.0),
            Align2::CENTER_CENTER,
            &f.name,
            FontId::proportional(11.0),
            col,
        );
        if let Some(pp) = pointer {
            if pp.x >= fx0 && pp.x <= fx1 && (pp.y - fy).abs() < h + 6.0 {
                hovered = Some(MapHit::Feature(f.id));
                hover_tip = Some(format!(
                    "{} ({})\n{}..{} · {} bp",
                    f.name,
                    f.kind,
                    span.start + 1,
                    span.start + span.len,
                    span.len
                ));
            }
        }
    }

    // Enzymes below the line.
    if app.prefs.show_enzymes {
        for (e, sites) in &app.analysis.chosen {
            for s in sites {
                let x = px(s.position);
                painter.line_segment(
                    [Pos2::new(x, y - 6.0), Pos2::new(x, y + 6.0)],
                    Stroke::new(1.5, theme::TEAL),
                );
                painter.text(
                    Pos2::new(x, y + 16.0),
                    Align2::CENTER_CENTER,
                    &e.name,
                    FontId::proportional(10.0),
                    theme::MUTED,
                );
                if let Some(pp) = pointer {
                    if (pp.x - x).abs() < 6.0 && (pp.y - y).abs() < 14.0 {
                        hovered = Some(MapHit::Enzyme(s.position));
                        hover_tip = Some(format!("{} — cuts at {}", e.name, s.cut_top + 1));
                    }
                }
            }
        }
    }

    // Primers below enzymes.
    if app.prefs.show_primers {
        for p in &app.doc.primers {
            if !p.visible {
                continue;
            }
            let Some(sites) = app.analysis.primer_sites.get(&p.id) else {
                continue;
            };
            for s in sites {
                let sx0 = px(s.start);
                let sx1 = px((s.start + s.len).min(l));
                let py = y + 34.0;
                let col = theme::rgb(p.color);
                let head = 6.0_f32.min((sx1 - sx0) / 2.0);
                let pts = if s.strand == Strand::Top {
                    vec![
                        Pos2::new(sx0, py - 3.0),
                        Pos2::new(sx1 - head, py - 3.0),
                        Pos2::new(sx1, py),
                        Pos2::new(sx1 - head, py + 3.0),
                        Pos2::new(sx0, py + 3.0),
                    ]
                } else {
                    vec![
                        Pos2::new(sx0, py),
                        Pos2::new(sx0 + head, py - 3.0),
                        Pos2::new(sx1, py - 3.0),
                        Pos2::new(sx1, py + 3.0),
                        Pos2::new(sx0 + head, py + 3.0),
                    ]
                };
                painter.add(Shape::convex_polygon(pts, col, Stroke::NONE));
                painter.text(
                    Pos2::new((sx0 + sx1) / 2.0, py + 12.0),
                    Align2::CENTER_CENTER,
                    &p.name,
                    FontId::proportional(10.0),
                    col,
                );
            }
        }
    }

    // Selection + search + colors on the backbone.
    for region in app.history_regions() {
        let rx0 = px(region.range.start);
        let rx1 = px((region.range.start + region.range.len).min(l));
        painter.rect_filled(
            Rect::from_min_max(Pos2::new(rx0, y - 4.0), Pos2::new(rx1, y + 4.0)),
            2.0,
            app.region_color(&region),
        );
    }
    for hr in search_hit_ranges(app) {
        let rx0 = px(hr.start);
        let rx1 = px((hr.start + hr.len).min(l));
        painter.rect_filled(
            Rect::from_min_max(Pos2::new(rx0, y - 4.0), Pos2::new(rx1, y + 4.0)),
            2.0,
            theme::ACCENT.gamma_multiply(0.7),
        );
    }
    if let Some(sel) = app.selection {
        let sx0 = px(sel.start);
        let sx1 = px((sel.start + sel.len).min(l));
        painter.rect_filled(
            Rect::from_min_max(Pos2::new(sx0, y - 5.0), Pos2::new(sx1, y + 5.0)),
            2.0,
            theme::SELECT_FILL,
        );
        painter.rect_stroke(
            Rect::from_min_max(Pos2::new(sx0, y - 5.0), Pos2::new(sx1, y + 5.0)),
            2.0,
            Stroke::new(1.5, theme::SELECT),
            egui::StrokeKind::Inside,
        );
    }

    painter.text(
        Pos2::new(x0, rect.top() + 18.0),
        Align2::LEFT_CENTER,
        format!(
            "{} · {} bp · linear · GC {:.1}%",
            app.doc.name,
            l,
            app.doc.sequence.gc_content()
        ),
        FontId::proportional(13.0),
        theme::TEXT,
    );

    // Linear drag selection.
    if resp.drag_started() {
        if let Some(pp) = resp.interact_pointer_pos() {
            let p = (((pp.x - x0) / w) * l as f32).clamp(0.0, (l - 1) as f32) as usize;
            app.map_drag_anchor = Some(p);
        }
    }
    if resp.dragged() {
        if let (Some(anchor), Some(pp)) = (app.map_drag_anchor, resp.interact_pointer_pos()) {
            let p = (((pp.x - x0) / w) * l as f32).clamp(0.0, (l - 1) as f32) as usize;
            let (a, b) = (anchor.min(p), anchor.max(p));
            app.select(Some(Range::new(a, b - a + 1)));
        }
    }
    if resp.drag_stopped() {
        app.map_drag_anchor = None;
    }
    if resp.clicked() {
        match hovered {
            Some(MapHit::Feature(id)) => app.select_feature(id),
            Some(MapHit::Enzyme(pos)) => app.select(Some(Range::new(pos, 1))),
            _ => {
                app.select(None);
                app.selected_feature = None;
                app.history_highlight = None;
            }
        }
    }
    if resp.double_clicked() {
        if let Some(MapHit::Feature(id)) = hovered {
            crate::dialogs::open_feature_editor(app, Some(id));
        }
    }
    if let Some(tip) = hover_tip {
        if resp.hovered() {
            egui::Tooltip::for_widget(resp).at_pointer().show(|ui| {
                ui.set_max_width(280.0);
                ui.label(tip);
            });
        }
    }
}

fn handle_interaction(
    app: &mut AppState,
    resp: &egui::Response,
    center: Pos2,
    r: Option<f32>,
    hovered: Option<MapHit>,
    hover_tip: Option<String>,
    _rect: Rect,
) {
    let l = app.doc.sequence.len();
    let Some(r) = r else { return };
    if resp.drag_started() {
        if let Some(pp) = resp.interact_pointer_pos() {
            let d = (pp - center).length();
            if d > r - 90.0 {
                app.map_drag_anchor = Some(angle_pos((pp - center).angle(), l));
            }
        }
    }
    if resp.dragged() {
        if let (Some(anchor), Some(pp)) = (app.map_drag_anchor, resp.interact_pointer_pos()) {
            let cur = angle_pos((pp - center).angle(), l);
            let len = (cur + l - anchor) % l + 1;
            app.select(Some(Range::new(anchor, len)));
        }
    }
    if resp.drag_stopped() {
        app.map_drag_anchor = None;
    }
    if resp.clicked() {
        match hovered {
            Some(MapHit::Feature(id)) => app.select_feature(id),
            Some(MapHit::Primer(id)) => {
                app.selected_primer = Some(id);
                app.selected_feature = None;
                if let Some(sites) = app.analysis.primer_sites.get(&id) {
                    if let Some(s) = sites.first() {
                        app.select(Some(Range::new(s.start, s.len)));
                    }
                }
            }
            Some(MapHit::Enzyme(pos)) => app.select(Some(Range::new(pos, 1))),
            _ => {
                app.select(None);
                app.selected_feature = None;
                app.history_highlight = None;
            }
        }
    }
    if resp.double_clicked() {
        if let Some(MapHit::Feature(id)) = hovered {
            crate::dialogs::open_feature_editor(app, Some(id));
        }
    }
    if let Some(tip) = hover_tip {
        if resp.hovered() {
            egui::Tooltip::for_widget(resp).at_pointer().show(|ui| {
                ui.set_max_width(300.0);
                ui.label(tip);
            });
        }
    }
}

pub fn search_hit_ranges(app: &AppState) -> Vec<Range> {
    if !app.search.open {
        return Vec::new();
    }
    match app.search.mode {
        crate::state::SearchMode::Dna => app
            .search
            .dna_hits
            .iter()
            .map(|h| Range::new(h.start, h.len))
            .collect(),
        crate::state::SearchMode::Protein => app
            .search
            .protein_hits
            .iter()
            .map(|h| Range::new(h.start, h.len))
            .collect(),
        crate::state::SearchMode::Name => Vec::new(),
    }
}

fn nice_step(l: usize) -> usize {
    let target = l / 10;
    let mut step = 1;
    while step < target {
        step *= 10;
    }
    for m in [1, 2, 5] {
        if step / 10 * m >= target {
            return (step / 10 * m).max(1);
        }
    }
    step.max(1)
}
