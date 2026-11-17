//! Map view: circular plasmid map or linear backbone, drawn on a GPUI canvas.
//! Geometry is ported from the egui shell; chrome uses the ink/graphite theme.

use gpui::{
    canvas, div, point, prelude::*, px, AnyElement, App, Bounds, Context, Entity, Hsla,
    MouseButton, MouseDownEvent, MouseMoveEvent, MouseUpEvent, Pixels, Point, Window,
};

use appcore::state::AppState;
use domain::{Direction, Range, Strand};

use crate::app::HelixBench;
use crate::draw::{self, Align, PxVal};
use crate::theme;

const TAU: f32 = std::f32::consts::TAU;
const HALF_PI: f32 = std::f32::consts::FRAC_PI_2;

#[derive(Clone, Copy)]
pub struct Geom {
    pub center: Point<Pixels>,
    pub r: f32,
    pub bounds: Bounds<Pixels>,
    pub circular: bool,
}

pub fn geom(bounds: Bounds<Pixels>, circular: bool) -> Geom {
    let center = bounds.center();
    let r = (bounds.size.width.min(bounds.size.height).v() / 2.0 - 96.0).max(60.0);
    Geom {
        center,
        r,
        bounds,
        circular,
    }
}

fn polar(center: Point<Pixels>, r: f32, angle: f32) -> Point<Pixels> {
    point(
        center.x + px(r * angle.cos()),
        center.y + px(r * angle.sin()),
    )
}

fn pos_angle(p: usize, l: usize) -> f32 {
    -HALF_PI + TAU * (p as f32 / l.max(1) as f32)
}

pub fn angle_pos(angle: f32, l: usize) -> usize {
    let a = (angle + HALF_PI).rem_euclid(TAU);
    ((a / TAU) * l as f32).round() as usize % l.max(1)
}

fn angle_of(pos: Point<Pixels>, center: Point<Pixels>) -> f32 {
    let dx = (pos.x - center.x).v();
    let dy = (pos.y - center.y).v();
    dy.atan2(dx)
}

fn dist(pos: Point<Pixels>, center: Point<Pixels>) -> f32 {
    let dx = (pos.x - center.x).v();
    let dy = (pos.y - center.y).v();
    (dx * dx + dy * dy).sqrt()
}

fn arc_points(center: Point<Pixels>, r: f32, a0: f32, a1: f32) -> Vec<Point<Pixels>> {
    let span = (a1 - a0).abs();
    let steps = ((span / TAU) * 180.0).ceil().max(2.0) as usize;
    (0..=steps)
        .map(|i| polar(center, r, a0 + (a1 - a0) * i as f32 / steps as f32))
        .collect()
}

fn band_points(
    center: Point<Pixels>,
    r_in: f32,
    r_out: f32,
    a0: f32,
    a1: f32,
    dir: Direction,
) -> Vec<Point<Pixels>> {
    let r_mid = (r_in + r_out) / 2.0;
    let ha = (11.0 / r_mid).clamp(0.015, 0.14);
    let span = a1 - a0;
    let head = ha.min(span / 2.5);
    let fwd = matches!(dir, Direction::Forward | Direction::Bidirectional);
    let rev = matches!(dir, Direction::Reverse | Direction::Bidirectional);
    let b0 = if rev { a0 + head } else { a0 };
    let b1 = if fwd { a1 - head } else { a1 };
    let mut pts = arc_points(center, r_out, b0, b1);
    if fwd {
        pts.push(polar(center, r_mid, a1));
    }
    pts.extend(arc_points(center, r_in, b1, b0));
    if rev {
        pts.push(polar(center, r_mid, a0));
    }
    pts
}

fn angle_in(a: f32, a0: f32, a1: f32) -> bool {
    let span = (a1 - a0).rem_euclid(TAU);
    let rel = (a - a0).rem_euclid(TAU);
    rel <= span.max(0.0001)
}

fn rect_hits(occupied: &[(f32, f32, f32, f32)], r: &(f32, f32, f32, f32)) -> bool {
    occupied
        .iter()
        .any(|o| r.0 < o.2 && o.0 < r.2 && r.1 < o.3 && o.1 < r.3)
}

/// Cartographic halo so labels stay readable where they cross thin ORF arcs.
fn label_halo(window: &mut Window, r: &(f32, f32, f32, f32)) {
    draw::rect(
        window,
        Bounds::new(
            point(px(r.0 + 1.0), px(r.1 + 1.0)),
            gpui::size(px(r.2 - r.0 - 2.0), px(r.3 - r.1 - 2.0)),
        ),
        2.0,
        theme::BG.opacity(0.85),
    );
}

// --- Owned snapshot used by the paint closure -------------------------------

struct FeatDraw {
    name: String,
    color: Hsla,
    dir: Direction,
    span: Range,
    multi: bool,
    selected: bool,
}

struct PrimerDraw {
    name: String,
    color: Hsla,
    seq_len: usize,
    sites: Vec<(usize, usize, bool)>, // start, len, top
}

struct EnzDraw {
    name: String,
    position: usize,
}

pub struct MapDraw {
    l: usize,
    circular: bool,
    topology: &'static str,
    name: String,
    gc: f32,
    selection: Option<Range>,
    show_orfs: bool,
    show_enzymes: bool,
    show_primers: bool,
    show_zoom: bool,
    zoom_start: usize,
    zoom_width: usize,
    orfs: Vec<(usize, usize, bool)>,
    enzymes: Vec<EnzDraw>,
    primers: Vec<PrimerDraw>,
    features: Vec<FeatDraw>,
    regions: Vec<(Range, Hsla)>,
    hits: Vec<Range>,
    hit_current: usize,
    has_hits: bool,
}

impl MapDraw {
    pub fn build(app: &AppState) -> Self {
        let l = app.doc.sequence.len();
        let mut features = Vec::new();
        for f in &app.doc.features {
            if !app.prefs.show_features || !f.visible || f.segments.is_empty() {
                continue;
            }
            features.push(FeatDraw {
                name: f.name.clone(),
                color: theme::pastel(f.color),
                dir: f.direction,
                span: f.span(l),
                multi: f.segments.len() > 1,
                selected: app.selected_feature == Some(f.id),
            });
        }
        let mut primers = Vec::new();
        for p in &app.doc.primers {
            if !p.visible {
                continue;
            }
            let sites = app
                .analysis
                .primer_sites
                .get(&p.id)
                .map(|s| {
                    s.iter()
                        .map(|b| (b.start, b.len, b.strand == Strand::Top))
                        .collect()
                })
                .unwrap_or_default();
            primers.push(PrimerDraw {
                name: p.name.clone(),
                color: theme::rgb_arr(p.color),
                seq_len: p.sequence.len(),
                sites,
            });
        }
        let mut enzymes = Vec::new();
        for (e, sites) in &app.analysis.chosen {
            for s in sites {
                enzymes.push(EnzDraw {
                    name: e.name.clone(),
                    position: s.position,
                });
            }
        }
        let regions = app
            .history_regions()
            .into_iter()
            .map(|r| {
                let c = theme::rgb_arr_a(app.region_rgb(&r), app.region_alpha());
                (r.range, c)
            })
            .collect();
        let hits = crate::util::search_hit_ranges(app);
        MapDraw {
            l,
            circular: app.map_circular(),
            topology: app.doc.sequence.topology.label(),
            name: app.doc.name.clone(),
            gc: app.doc.sequence.gc_content(),
            selection: app.selection,
            show_orfs: app.prefs.show_orfs,
            show_enzymes: app.prefs.show_enzymes,
            show_primers: app.prefs.show_primers,
            show_zoom: app.prefs.show_zoom_controls,
            zoom_start: app.zoom_start,
            zoom_width: app.zoom_width,
            orfs: app
                .analysis
                .orfs
                .iter()
                .map(|o| (o.start, o.len, o.strand == Strand::Top))
                .collect(),
            enzymes,
            primers,
            features,
            regions,
            has_hits: app.search.hit_count() > 0,
            hit_current: app.search.current,
            hits,
        }
    }

    /// Read-only snapshot of a document for the Split Workspace compare panes.
    pub fn build_compare(doc: &domain::Document, prefs: &storage::config::UiPrefs) -> Self {
        let analysis = appcore::analysis::compute(doc, prefs);
        let l = doc.sequence.len();
        let features = doc
            .features
            .iter()
            .filter(|f| f.visible && !f.segments.is_empty())
            .map(|f| FeatDraw {
                name: f.name.clone(),
                color: theme::pastel(f.color),
                dir: f.direction,
                span: f.span(l),
                multi: f.segments.len() > 1,
                selected: false,
            })
            .collect();
        let primers = doc
            .primers
            .iter()
            .filter(|p| p.visible)
            .map(|p| PrimerDraw {
                name: p.name.clone(),
                color: theme::rgb_arr(p.color),
                seq_len: p.sequence.len(),
                sites: analysis
                    .primer_sites
                    .get(&p.id)
                    .map(|s| {
                        s.iter()
                            .map(|b| (b.start, b.len, b.strand == Strand::Top))
                            .collect()
                    })
                    .unwrap_or_default(),
            })
            .collect();
        let mut enzymes = Vec::new();
        for (e, sites) in &analysis.chosen {
            for s in sites {
                enzymes.push(EnzDraw {
                    name: e.name.clone(),
                    position: s.position,
                });
            }
        }
        MapDraw {
            l,
            circular: doc.sequence.topology == domain::Topology::Circular,
            topology: doc.sequence.topology.label(),
            name: doc.name.clone(),
            gc: doc.sequence.gc_content(),
            selection: None,
            show_orfs: prefs.show_orfs,
            show_enzymes: prefs.show_enzymes,
            show_primers: prefs.show_primers,
            show_zoom: false,
            zoom_start: 0,
            zoom_width: l.max(1),
            orfs: analysis
                .orfs
                .iter()
                .map(|o| (o.start, o.len, o.strand == Strand::Top))
                .collect(),
            enzymes,
            primers,
            features,
            regions: Vec::new(),
            hits: Vec::new(),
            hit_current: 0,
            has_hits: false,
        }
    }
}

pub fn paint(d: &MapDraw, bounds: Bounds<Pixels>, window: &mut Window, cx: &mut App) {
    draw::rect(window, bounds, 0.0, theme::BG);
    if d.l == 0 {
        draw::text(
            window,
            cx,
            bounds.center(),
            "Empty sequence — use File ▸ New Sequence to begin.",
            theme::T_MD,
            theme::MUTED,
            theme::ui_font(),
            Align::Center,
        );
        return;
    }
    if d.circular {
        paint_circular(d, bounds, window, cx);
    } else {
        paint_linear(d, bounds, window, cx);
    }
}

fn paint_circular(d: &MapDraw, bounds: Bounds<Pixels>, window: &mut Window, cx: &mut App) {
    let g = geom(bounds, true);
    let c = g.center;
    let r = g.r;
    let l = d.l;

    draw::disc(window, c, r + 58.0, theme::PANEL);
    draw::ring(window, c, r + 58.0, 1.0, theme::BORDER);
    draw::disc(window, c, r + 26.0, theme::SURFACE);

    let r_feat_out = r;
    let r_feat_in = r - 13.0;
    let r_enz = r - 26.0;
    let r_primer = r - 52.0;
    let r_orf_top = r - 76.0;
    let r_orf_bot = r - 88.0;

    draw::ring(window, c, r_enz, 1.5, theme::FAINT);

    // ORF arcs
    if d.show_orfs {
        for &(start, len, top) in &d.orfs {
            let a0 = pos_angle(start, l);
            let a1 = a0 + TAU * len as f32 / l as f32;
            let (rr, col) = if top {
                (r_orf_top, theme::ORF_TOP)
            } else {
                (r_orf_bot, theme::ORF_BOTTOM)
            };
            draw::polyline(window, &arc_points(c, rr, a0, a1), 3.0, col.opacity(0.85));
        }
    }

    // Shared label-occupancy grid. Enzyme and primer labels extend toward the
    // map interior so they never cross the tick / arrow rings; dense clusters
    // fan out vertically with leader lines instead of stacking into a blob.
    let mut occupied: Vec<(f32, f32, f32, f32)> = Vec::new();
    if d.show_primers {
        for p in &d.primers {
            for &(start, len, _) in &p.sites {
                let a0 = pos_angle(start, l);
                let mid = a0 + TAU * len as f32 / l as f32 / 2.0;
                let ap = polar(c, r_primer, mid);
                occupied.push((
                    ap.x.v() - 7.0,
                    ap.y.v() - 7.0,
                    ap.x.v() + 7.0,
                    ap.y.v() + 7.0,
                ));
            }
        }
    }

    // Enzyme ticks + labels
    if d.show_enzymes {
        let label_sites = d.enzymes.len() <= 42;
        let mut labels: Vec<(f32, String)> = Vec::new();
        for e in &d.enzymes {
            let a = pos_angle(e.position, l);
            draw::segment(
                window,
                polar(c, r_enz - 5.0, a),
                polar(c, r_enz + 5.0, a),
                1.5,
                theme::ACCENT,
            );
            if label_sites {
                labels.push((a, e.name.clone()));
            }
        }
        if label_sites {
            labels.sort_by(|a, b| a.0.total_cmp(&b.0));
            for (a, name) in labels {
                let w = draw::measure(window, &name, 10.5, theme::ui_font()).v();
                let right = a.cos() >= 0.0;
                let align = if right { Align::Right } else { Align::Left };
                let rect_at = |pos: Point<Pixels>| -> (f32, f32, f32, f32) {
                    let (x0, x1) = if right {
                        (pos.x.v() - w, pos.x.v())
                    } else {
                        (pos.x.v(), pos.x.v() + w)
                    };
                    (x0 - 2.0, pos.y.v() - 7.0, x1 + 2.0, pos.y.v() + 7.0)
                };
                let mut placed = false;
                for i in 0..3 {
                    let pos = polar(c, r_enz - 14.0 - i as f32 * 12.0, a);
                    let rect = rect_at(pos);
                    if !rect_hits(&occupied, &rect) {
                        label_halo(window, &rect);
                        draw::text(
                            window,
                            cx,
                            pos,
                            &name,
                            10.5,
                            theme::MUTED,
                            theme::ui_font(),
                            align,
                        );
                        occupied.push(rect);
                        placed = true;
                        break;
                    }
                }
                if !placed {
                    let base = polar(c, (r_enz - 38.0).max(24.0), a);
                    for dy in [0.0, -13.0, 13.0, -26.0, 26.0, -39.0, 39.0, -52.0, 52.0] {
                        let pos = base + point(px(0.0), px(dy));
                        let rect = rect_at(pos);
                        if !rect_hits(&occupied, &rect) {
                            let from = polar(c, r_enz - 6.0, a);
                            let edge_x = if right { rect.2 - 2.0 } else { rect.0 + 2.0 };
                            draw::segment(
                                window,
                                from,
                                point(px(edge_x), pos.y),
                                0.75,
                                theme::FAINT.opacity(0.6),
                            );
                            label_halo(window, &rect);
                            draw::text(
                                window,
                                cx,
                                pos,
                                &name,
                                10.0,
                                theme::FAINT,
                                theme::ui_font(),
                                align,
                            );
                            occupied.push(rect);
                            break;
                        }
                    }
                }
                // If every candidate collides the tick alone marks the site.
            }
        }
    }

    // Primer arrows
    if d.show_primers {
        for p in &d.primers {
            for &(start, len, top) in &p.sites {
                let a0 = pos_angle(start, l);
                let a1 = a0 + TAU * len as f32 / l as f32;
                let dir = if top {
                    Direction::Forward
                } else {
                    Direction::Reverse
                };
                let pts = band_points(c, r_primer - 4.0, r_primer + 4.0, a0, a1, dir);
                draw::polygon(window, &pts, p.color.opacity(0.9));
                let extra = p.seq_len.saturating_sub(len);
                if extra > 0 {
                    let (t0, t1) = if top {
                        (a0 - TAU * extra as f32 / l as f32, a0)
                    } else {
                        (a1, a1 + TAU * extra as f32 / l as f32)
                    };
                    draw::polyline(
                        window,
                        &arc_points(c, r_primer, t0, t1),
                        1.5,
                        p.color.opacity(0.6),
                    );
                }
            }
        }
    }

    // Feature arrows + labels
    let mut feat_labels: Vec<(f32, String, Hsla)> = Vec::new();
    for f in &d.features {
        let a0 = pos_angle(f.span.start, l);
        let a1 = a0 + TAU * f.span.len as f32 / l as f32;
        let fill = if f.selected {
            f.color
        } else {
            f.color.opacity(0.82)
        };
        let pts = band_points(c, r_feat_in, r_feat_out, a0, a1, f.dir);
        draw::polygon(window, &pts, fill);
        if f.multi {
            draw::polyline(
                window,
                &arc_points(c, (r_feat_in + r_feat_out) / 2.0, a0, a1),
                1.0,
                f.color.opacity(0.5),
            );
        }
        let mid = (a0 + a1) / 2.0;
        feat_labels.push((mid, f.name.clone(), f.color));
    }
    feat_labels.sort_by(|a, b| a.0.total_cmp(&b.0));
    let mut levels = [-10.0f32; 3];
    for (mid, name, col) in feat_labels {
        let width = name.len() as f32 * 6.4 + 10.0;
        let mut placed = false;
        for (i, last_end) in levels.iter_mut().enumerate() {
            let rr = r + 16.0 + i as f32 * 17.0;
            let half = width / rr / 2.0;
            if mid - half > *last_end {
                let anchor = polar(c, r + 3.0, mid);
                let elbow = polar(c, rr - 6.0, mid);
                let pos = polar(c, rr, mid);
                draw::segment(window, anchor, elbow, 0.8, col.opacity(0.55));
                let align = if mid.cos() >= 0.0 {
                    Align::Left
                } else {
                    Align::Right
                };
                draw::text(
                    window,
                    cx,
                    pos,
                    &name,
                    11.5,
                    theme::TEXT,
                    theme::ui_font(),
                    align,
                );
                let w = draw::measure(window, &name, 11.5, theme::ui_font()).v();
                let (x0, x1) = if mid.cos() >= 0.0 {
                    (pos.x.v(), pos.x.v() + w)
                } else {
                    (pos.x.v() - w, pos.x.v())
                };
                occupied.push((x0 - 2.0, pos.y.v() - 8.0, x1 + 2.0, pos.y.v() + 8.0));
                *last_end = mid + half;
                placed = true;
                break;
            }
        }
        if !placed {
            let pos = polar(c, r + 50.0, mid);
            let align = if mid.cos() >= 0.0 {
                Align::Left
            } else {
                Align::Right
            };
            draw::text(
                window,
                cx,
                pos,
                &name,
                10.5,
                theme::TEXT,
                theme::ui_font(),
                align,
            );
            let w = draw::measure(window, &name, 10.5, theme::ui_font()).v();
            let (x0, x1) = if mid.cos() >= 0.0 {
                (pos.x.v(), pos.x.v() + w)
            } else {
                (pos.x.v() - w, pos.x.v())
            };
            occupied.push((x0 - 2.0, pos.y.v() - 8.0, x1 + 2.0, pos.y.v() + 8.0));
        }
    }

    // Primer names join the outer label column, led out from their arrows so
    // they never sit on top of the enzyme / feature rings.
    if d.show_primers {
        for p in &d.primers {
            for &(start, len, _) in &p.sites {
                let a0 = pos_angle(start, l);
                let mid = a0 + TAU * len as f32 / l as f32 / 2.0;
                let w = draw::measure(window, &p.name, 10.5, theme::ui_font()).v();
                let left = mid.cos() >= 0.0;
                let align = if left { Align::Left } else { Align::Right };
                let mut placed = false;
                for rr in [r + 16.0, r + 33.0, r + 50.0] {
                    for dy in [0.0, -13.0, 13.0, -26.0, 26.0] {
                        let pos = polar(c, rr, mid) + point(px(0.0), px(dy));
                        let (x0, x1) = if left {
                            (pos.x.v(), pos.x.v() + w)
                        } else {
                            (pos.x.v() - w, pos.x.v())
                        };
                        let rect = (x0 - 2.0, pos.y.v() - 7.0, x1 + 2.0, pos.y.v() + 7.0);
                        if rect_hits(&occupied, &rect) {
                            continue;
                        }
                        let from = polar(c, r_primer + 5.0, mid);
                        let edge_x = if left { rect.0 + 2.0 } else { rect.2 - 2.0 };
                        draw::segment(
                            window,
                            from,
                            point(px(edge_x), pos.y),
                            0.75,
                            p.color.opacity(0.45),
                        );
                        label_halo(window, &rect);
                        draw::text(
                            window,
                            cx,
                            pos,
                            &p.name,
                            10.5,
                            p.color,
                            theme::ui_font(),
                            align,
                        );
                        occupied.push(rect);
                        placed = true;
                        break;
                    }
                    if placed {
                        break;
                    }
                }
            }
        }
    }

    // Color regions
    for (range, col) in &d.regions {
        let a0 = pos_angle(range.start, l);
        let a1 = a0 + TAU * range.len as f32 / l as f32;
        draw::polyline(window, &arc_points(c, r + 6.0, a0, a1), 5.0, *col);
    }

    // Search hits
    for (i, hr) in d.hits.iter().enumerate() {
        let a0 = pos_angle(hr.start, l);
        let a1 = a0 + TAU * hr.len as f32 / l as f32;
        let current = i == d.hit_current && d.has_hits;
        draw::polyline(
            window,
            &arc_points(c, r + 6.0, a0, a1),
            if current { 6.0 } else { 4.0 },
            theme::INFO.opacity(if current { 1.0 } else { 0.65 }),
        );
    }

    // Selection arc
    if let Some(sel) = d.selection {
        let a0 = pos_angle(sel.start, l);
        let a1 = a0 + TAU * sel.len as f32 / l as f32;
        draw::polyline(window, &arc_points(c, r + 12.0, a0, a1), 4.0, theme::SELECT);
        for a in [a0, a1] {
            draw::segment(
                window,
                polar(c, r + 7.0, a),
                polar(c, r + 17.0, a),
                2.0,
                theme::SELECT,
            );
        }
    }

    // Zoom window
    if d.show_zoom && d.zoom_width < l {
        let a0 = pos_angle(d.zoom_start, l);
        let a1 = a0 + TAU * d.zoom_width as f32 / l as f32;
        draw::polyline(
            window,
            &arc_points(c, r + 21.0, a0, a1),
            2.5,
            theme::TEXT.opacity(0.85),
        );
    }

    // Origin tick at 12 o'clock
    draw::segment(
        window,
        polar(c, r + 24.0, -HALF_PI),
        polar(c, r + 30.0, -HALF_PI),
        1.5,
        theme::FAINT,
    );

    // Center info
    draw::text(
        window,
        cx,
        c + point(px(0.0), px(-34.0)),
        &d.name,
        theme::T_LG,
        theme::TEXT,
        theme::ui_font(),
        Align::Center,
    );
    draw::text(
        window,
        cx,
        c + point(px(0.0), px(-12.0)),
        &format!("{} bp · {} DNA", l, d.topology),
        theme::T_SM,
        theme::MUTED,
        theme::ui_font(),
        Align::Center,
    );
    draw::text(
        window,
        cx,
        c + point(px(0.0), px(6.0)),
        &format!("GC {:.1}%", d.gc),
        theme::T_SM,
        theme::FAINT,
        theme::ui_font(),
        Align::Center,
    );
    if let Some(sel) = d.selection {
        draw::text(
            window,
            cx,
            c + point(px(0.0), px(28.0)),
            &format!(
                "{}..{} · {} bp",
                sel.start + 1,
                sel.start + sel.len,
                sel.len
            ),
            theme::T_SM,
            theme::ACCENT_TEXT,
            theme::ui_font(),
            Align::Center,
        );
    } else {
        draw::text(
            window,
            cx,
            c + point(px(0.0), px(28.0)),
            "drag to select",
            theme::T_XS,
            theme::FAINT,
            theme::ui_font(),
            Align::Center,
        );
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

fn linear_x(bounds: Bounds<Pixels>) -> (f32, f32, f32) {
    let margin = 48.0;
    let x0 = bounds.left().v() + margin;
    let x1 = bounds.right().v() - margin;
    let y = bounds.center().y.v();
    (x0, (x1 - x0).max(50.0), y)
}

fn paint_linear(d: &MapDraw, bounds: Bounds<Pixels>, window: &mut Window, cx: &mut App) {
    let l = d.l;
    let (x0, w, y) = linear_x(bounds);
    let px_at = |p: usize| x0 + w * (p as f32 / l as f32);
    let pt = |x: f32, yy: f32| point(px(x), px(yy));

    draw::segment(window, pt(x0, y), pt(x0 + w, y), 2.0, theme::FAINT);

    // Ruler
    let step = nice_step(l);
    let mut p = 0;
    while p <= l {
        let x = px_at(p);
        draw::segment(window, pt(x, y + 40.0), pt(x, y + 46.0), 1.0, theme::FAINT);
        draw::text(
            window,
            cx,
            pt(x, y + 58.0),
            &format!("{p}"),
            10.0,
            theme::FAINT,
            theme::ui_font(),
            Align::Center,
        );
        p += step;
    }

    // Features above the line in lanes
    let mut lane_ends: Vec<f32> = Vec::new();
    for f in &d.features {
        let fx0 = px_at(f.span.start);
        let fx1 = px_at((f.span.start + f.span.len).min(l));
        let mut lane = 0;
        while lane < lane_ends.len() && lane_ends[lane] > fx0 - 4.0 {
            lane += 1;
        }
        if lane == lane_ends.len() {
            lane_ends.push(0.0);
        }
        lane_ends[lane] = fx1 + 8.0;
        let fy = y - 26.0 - lane as f32 * 24.0;
        let h = 9.0;
        let head = 8.0f32.min((fx1 - fx0) / 2.5);
        let pts = match f.dir {
            Direction::Reverse => vec![
                pt(fx0, fy),
                pt(fx0 + head, fy - h),
                pt(fx1, fy - h),
                pt(fx1, fy + h),
                pt(fx0 + head, fy + h),
            ],
            Direction::None => vec![
                pt(fx0, fy - h),
                pt(fx1, fy - h),
                pt(fx1, fy + h),
                pt(fx0, fy + h),
            ],
            _ => vec![
                pt(fx0, fy - h),
                pt(fx1 - head, fy - h),
                pt(fx1, fy),
                pt(fx1 - head, fy + h),
                pt(fx0, fy + h),
            ],
        };
        let fill = if f.selected {
            f.color
        } else {
            f.color.opacity(0.85)
        };
        draw::polygon(window, &pts, fill);
        draw::text(
            window,
            cx,
            pt((fx0 + fx1) / 2.0, fy - h - 8.0),
            &f.name,
            11.0,
            f.color,
            theme::ui_font(),
            Align::Center,
        );
    }

    // Enzymes below the line
    if d.show_enzymes {
        for e in &d.enzymes {
            let x = px_at(e.position);
            draw::segment(window, pt(x, y - 6.0), pt(x, y + 6.0), 1.5, theme::ACCENT);
            draw::text(
                window,
                cx,
                pt(x, y + 16.0),
                &e.name,
                10.0,
                theme::MUTED,
                theme::ui_font(),
                Align::Center,
            );
        }
    }

    // Primers below enzymes
    if d.show_primers {
        for p in &d.primers {
            for &(start, len, top) in &p.sites {
                let sx0 = px_at(start);
                let sx1 = px_at((start + len).min(l));
                let py = y + 34.0;
                let head = 6.0f32.min((sx1 - sx0) / 2.0);
                let pts = if top {
                    vec![
                        pt(sx0, py - 3.0),
                        pt(sx1 - head, py - 3.0),
                        pt(sx1, py),
                        pt(sx1 - head, py + 3.0),
                        pt(sx0, py + 3.0),
                    ]
                } else {
                    vec![
                        pt(sx0, py),
                        pt(sx0 + head, py - 3.0),
                        pt(sx1, py - 3.0),
                        pt(sx1, py + 3.0),
                        pt(sx0 + head, py + 3.0),
                    ]
                };
                draw::polygon(window, &pts, p.color);
                draw::text(
                    window,
                    cx,
                    pt((sx0 + sx1) / 2.0, py + 12.0),
                    &p.name,
                    10.0,
                    p.color,
                    theme::ui_font(),
                    Align::Center,
                );
            }
        }
    }

    // Regions / hits / selection on the backbone
    for (range, col) in &d.regions {
        let rx0 = px_at(range.start);
        let rx1 = px_at((range.start + range.len).min(l));
        draw::rect(
            window,
            Bounds::new(pt(rx0, y - 4.0), gpui::size(px(rx1 - rx0), px(8.0))),
            2.0,
            *col,
        );
    }
    for hr in &d.hits {
        let rx0 = px_at(hr.start);
        let rx1 = px_at((hr.start + hr.len).min(l));
        draw::rect(
            window,
            Bounds::new(pt(rx0, y - 4.0), gpui::size(px(rx1 - rx0), px(8.0))),
            2.0,
            theme::INFO.opacity(0.7),
        );
    }
    if let Some(sel) = d.selection {
        let first = sel.len.min(l - sel.start);
        for (start, len) in [(sel.start, first), (0, sel.len - first)] {
            if len == 0 {
                continue;
            }
            let b = Bounds::new(
                pt(px_at(start), y - 5.0),
                gpui::size(px(px_at(start + len) - px_at(start)), px(10.0)),
            );
            draw::rect_border(
                window,
                b,
                2.0,
                theme::SELECT.opacity(0.25),
                1.5,
                theme::SELECT,
            );
        }
    }

    draw::text(
        window,
        cx,
        pt(x0, bounds.top().v() + 18.0),
        &format!(
            "{} · {} bp · linear view · {} DNA · GC {:.1}%",
            d.name, l, d.topology, d.gc
        ),
        theme::T_BASE,
        theme::TEXT,
        theme::ui_font(),
        Align::Left,
    );
}

// --- Hit testing ------------------------------------------------------------

#[derive(Clone)]
pub enum Hit {
    Feature(u64),
    Primer(u64, Range, Strand),
    Enzyme(usize),
}

pub fn hit_test(app: &AppState, g: Geom, pos: Point<Pixels>) -> Option<Hit> {
    let l = app.doc.sequence.len();
    if l == 0 {
        return None;
    }
    if g.circular {
        let d = dist(pos, g.center);
        let pa = angle_of(pos, g.center);
        let r = g.r;
        // features (highest priority)
        for f in &app.doc.features {
            if !app.prefs.show_features || !f.visible || f.segments.is_empty() {
                continue;
            }
            let span = f.span(l);
            let a0 = pos_angle(span.start, l);
            let a1 = a0 + TAU * span.len as f32 / l as f32;
            if (r - 13.0 - 2.0..=r + 2.0).contains(&d) && angle_in(pa, a0, a1) {
                return Some(Hit::Feature(f.id));
            }
        }
        if app.prefs.show_primers {
            for p in &app.doc.primers {
                if !p.visible {
                    continue;
                }
                if let Some(sites) = app.analysis.primer_sites.get(&p.id) {
                    for s in sites {
                        let a0 = pos_angle(s.start, l);
                        let a1 = a0 + TAU * s.len as f32 / l as f32;
                        if (d - (r - 52.0)).abs() < 9.0 && angle_in(pa, a0, a1) {
                            return Some(Hit::Primer(p.id, Range::new(s.start, s.len), s.strand));
                        }
                    }
                }
            }
        }
        if app.prefs.show_enzymes {
            for (_e, sites) in &app.analysis.chosen {
                for s in sites {
                    let a = pos_angle(s.position, l);
                    if dist(pos, polar(g.center, r - 26.0, a)) < 8.0 {
                        return Some(Hit::Enzyme(s.position));
                    }
                }
            }
        }
    } else {
        let (x0, w, y) = linear_x(g.bounds);
        let px_at = |p: usize| x0 + w * (p as f32 / l as f32);
        let mut lane_ends: Vec<f32> = Vec::new();
        for f in &app.doc.features {
            if !app.prefs.show_features || !f.visible || f.segments.is_empty() {
                continue;
            }
            let span = f.span(l);
            let fx0 = px_at(span.start);
            let fx1 = px_at((span.start + span.len).min(l));
            let mut lane = 0;
            while lane < lane_ends.len() && lane_ends[lane] > fx0 - 4.0 {
                lane += 1;
            }
            if lane == lane_ends.len() {
                lane_ends.push(0.0);
            }
            lane_ends[lane] = fx1 + 8.0;
            let fy = y - 26.0 - lane as f32 * 24.0;
            if pos.x.v() >= fx0 && pos.x.v() <= fx1 && (pos.y.v() - fy).abs() < 15.0 {
                return Some(Hit::Feature(f.id));
            }
        }
        if app.prefs.show_enzymes {
            for (_e, sites) in &app.analysis.chosen {
                for s in sites {
                    let x = px_at(s.position);
                    if (pos.x.v() - x).abs() < 6.0 && (pos.y.v() - y).abs() < 14.0 {
                        return Some(Hit::Enzyme(s.position));
                    }
                }
            }
        }
    }
    None
}

// --- View + interaction -----------------------------------------------------

impl HelixBench {
    pub fn render_map(&mut self, cx: &mut Context<Self>) -> AnyElement {
        self.app.ensure_analysis();
        let draw = MapDraw::build(&self.app);
        let entity: Entity<Self> = cx.entity();
        div()
            .id("map-view")
            .size_full()
            .child(
                canvas(
                    move |bounds, _window, cx| {
                        entity.update(cx, |this, _| this.map_bounds = Some(bounds));
                    },
                    move |bounds, _state, window, cx| paint(&draw, bounds, window, cx),
                )
                .size_full(),
            )
            .on_mouse_down(MouseButton::Left, cx.listener(Self::map_mouse_down))
            .on_mouse_move(cx.listener(Self::map_mouse_move))
            .on_mouse_up(MouseButton::Left, cx.listener(Self::map_mouse_up))
            .on_mouse_up_out(MouseButton::Left, cx.listener(Self::map_mouse_up))
            .into_any_element()
    }

    fn map_geom(&self) -> Option<Geom> {
        let b = self.map_bounds?;
        Some(geom(b, self.app.map_circular()))
    }

    fn map_mouse_down(&mut self, ev: &MouseDownEvent, _w: &mut Window, cx: &mut Context<Self>) {
        let Some(g) = self.map_geom() else { return };
        let l = self.app.doc.sequence.len();
        if l == 0 {
            return;
        }
        self.map_press = Some(ev.position);
        self.map_dragged = false;
        if ev.click_count >= 2 {
            if let Some(Hit::Feature(id)) = hit_test(&self.app, g, ev.position) {
                crate::dialogs::open_feature_editor(&mut self.app, Some(id));
                cx.notify();
            }
            return;
        }
        if g.circular {
            if dist(ev.position, g.center) > g.r - 90.0 {
                self.app.map_drag_anchor = Some(angle_pos(angle_of(ev.position, g.center), l));
            }
        } else {
            let (x0, w, _) = linear_x(g.bounds);
            let p = (((ev.position.x.v() - x0) / w) * l as f32).clamp(0.0, (l - 1) as f32) as usize;
            self.app.map_drag_anchor = Some(p);
        }
        cx.notify();
    }

    fn map_mouse_move(&mut self, ev: &MouseMoveEvent, _w: &mut Window, cx: &mut Context<Self>) {
        let Some(press) = self.map_press else { return };
        let Some(g) = self.map_geom() else { return };
        let l = self.app.doc.sequence.len();
        if (ev.position.x - press.x).v().abs() + (ev.position.y - press.y).v().abs() > 3.0 {
            self.map_dragged = true;
        }
        if !self.map_dragged {
            return;
        }
        if let Some(anchor) = self.app.map_drag_anchor {
            if g.circular {
                let cur = angle_pos(angle_of(ev.position, g.center), l);
                let len = (cur + l - anchor) % l + 1;
                let source = self.app.doc.source_ref();
                let _ = appcore::selection::apply(
                    &mut self.app,
                    source,
                    Some(Range::new(anchor, len)),
                    Strand::Top,
                );
            } else {
                let (x0, w, _) = linear_x(g.bounds);
                let p =
                    (((ev.position.x.v() - x0) / w) * l as f32).clamp(0.0, (l - 1) as f32) as usize;
                let (a, b) = (anchor.min(p), anchor.max(p));
                let source = self.app.doc.source_ref();
                let _ = appcore::selection::apply(
                    &mut self.app,
                    source,
                    Some(Range::new(a, b - a + 1)),
                    Strand::Top,
                );
            }
            cx.notify();
        }
    }

    fn map_mouse_up(&mut self, ev: &MouseUpEvent, _w: &mut Window, cx: &mut Context<Self>) {
        if self.map_press.take().is_none() {
            return;
        }
        let was_drag = self.map_dragged;
        self.map_dragged = false;
        self.app.map_drag_anchor = None;
        if was_drag {
            cx.notify();
            return;
        }
        let Some(g) = self.map_geom() else { return };
        match hit_test(&self.app, g, ev.position) {
            Some(Hit::Feature(id)) => self.app.select_feature(id),
            Some(Hit::Primer(id, range, strand)) => {
                let source = self.app.doc.source_ref();
                if let Err(error) =
                    appcore::selection::primer_site(&mut self.app, source, id, range, strand)
                {
                    self.app.toast(error);
                }
            }
            Some(Hit::Enzyme(pos)) => {
                let source = self.app.doc.source_ref();
                let _ = appcore::selection::apply(
                    &mut self.app,
                    source,
                    Some(Range::new(pos, 1)),
                    Strand::Top,
                );
            }
            None => {
                let source = self.app.doc.source_ref();
                let _ = appcore::selection::apply(&mut self.app, source, None, Strand::Top);
                self.app.history_highlight = None;
            }
        }
        cx.notify();
    }
}

#[cfg(test)]
mod p08a_tests {
    use super::*;
    #[test]
    fn hidden_map_features_are_neither_drawn_nor_clickable() {
        let mut app = AppState::new(
            domain::Document::new(
                "test".into(),
                b"ATGCGTAC".to_vec(),
                domain::Topology::Circular,
            ),
            Default::default(),
        );
        app.doc.add_feature(domain::Feature::new(
            0,
            "gene".into(),
            "gene".into(),
            vec![Range::new(0, 4)],
        ));
        assert_eq!(MapDraw::build(&app).features.len(), 1);
        app.prefs.show_features = false;
        assert!(MapDraw::build(&app).features.is_empty());
        let g = geom(
            Bounds::new(point(px(0.0), px(0.0)), gpui::size(px(800.0), px(800.0))),
            true,
        );
        assert!(hit_test(&app, g, polar(g.center, g.r - 5.0, -HALF_PI + 0.2)).is_none());
    }
}
