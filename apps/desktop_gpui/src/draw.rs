//! Low-level canvas drawing helpers shared by the Map and Sequence views:
//! shaped text, stroked/filled paths, discs and a palette-matched scrollbar.

use gpui::{
    point, px, quad, size, App, Bounds, Hsla, PathBuilder, Pixels, Point, SharedString, TextRun,
    Window,
};

use crate::theme;

/// gpui 0.2 keeps the inner value of `Pixels` private; this reads it as `f32`.
pub trait PxVal {
    fn v(self) -> f32;
}

impl PxVal for gpui::Pixels {
    fn v(self) -> f32 {
        f32::from(self)
    }
}

#[derive(Clone, Copy)]
pub enum Align {
    Left,
    Center,
    Right,
}

/// Shape a single line and paint it vertically centered on `center.y`.
/// Returns the measured width so callers can lay out neighbouring labels.
#[allow(clippy::too_many_arguments)]
pub fn text(
    window: &mut Window,
    cx: &mut App,
    center: Point<Pixels>,
    s: &str,
    size_px: f32,
    color: Hsla,
    font: gpui::Font,
    align: Align,
) -> Pixels {
    if s.is_empty() {
        return px(0.0);
    }
    let run = TextRun {
        len: s.len(),
        font,
        color,
        background_color: None,
        underline: None,
        strikethrough: None,
    };
    let shaped = window.text_system().shape_line(
        SharedString::from(s.to_owned()),
        px(size_px),
        &[run],
        None,
    );
    let w = shaped.width;
    let lh = px(size_px * 1.32);
    let x = match align {
        Align::Left => center.x,
        Align::Center => center.x - w / 2.0,
        Align::Right => center.x - w,
    };
    let _ = shaped.paint(point(x, center.y - lh / 2.0), lh, window, cx);
    w
}

pub fn measure(window: &mut Window, s: &str, size_px: f32, font: gpui::Font) -> Pixels {
    if s.is_empty() {
        return px(0.0);
    }
    let run = TextRun {
        len: s.len(),
        font,
        color: theme::TEXT,
        background_color: None,
        underline: None,
        strikethrough: None,
    };
    window
        .text_system()
        .shape_line(SharedString::from(s.to_owned()), px(size_px), &[run], None)
        .width
}

pub fn polyline(window: &mut Window, pts: &[Point<Pixels>], width: f32, color: Hsla) {
    if pts.len() < 2 {
        return;
    }
    let mut b = PathBuilder::stroke(px(width));
    b.move_to(pts[0]);
    for p in &pts[1..] {
        b.line_to(*p);
    }
    if let Ok(path) = b.build() {
        window.paint_path(path, color);
    }
}

pub fn polygon(window: &mut Window, pts: &[Point<Pixels>], fill: Hsla) {
    if pts.len() < 3 {
        return;
    }
    let mut b = PathBuilder::fill();
    b.add_polygon(pts, true);
    if let Ok(path) = b.build() {
        window.paint_path(path, fill);
    }
}

pub fn segment(window: &mut Window, a: Point<Pixels>, b: Point<Pixels>, width: f32, color: Hsla) {
    polyline(window, &[a, b], width, color);
}

fn circle_points(center: Point<Pixels>, r: f32) -> Vec<Point<Pixels>> {
    let n = 128;
    (0..=n)
        .map(|i| {
            let a = std::f32::consts::TAU * i as f32 / n as f32;
            point(center.x + px(r * a.cos()), center.y + px(r * a.sin()))
        })
        .collect()
}

pub fn disc(window: &mut Window, center: Point<Pixels>, r: f32, fill: Hsla) {
    polygon(window, &circle_points(center, r), fill);
}

pub fn ring(window: &mut Window, center: Point<Pixels>, r: f32, width: f32, color: Hsla) {
    polyline(window, &circle_points(center, r), width, color);
}

pub fn rect(window: &mut Window, bounds: Bounds<Pixels>, radius: f32, fill: Hsla) {
    window.paint_quad(quad(
        bounds,
        px(radius),
        fill,
        px(0.0),
        theme::BORDER,
        Default::default(),
    ));
}

pub fn rect_border(
    window: &mut Window,
    bounds: Bounds<Pixels>,
    radius: f32,
    fill: Hsla,
    border_w: f32,
    border: Hsla,
) {
    window.paint_quad(quad(
        bounds,
        px(radius),
        fill,
        px(border_w),
        border,
        Default::default(),
    ));
}

/// A vertical scrollbar drawn in the artifact's own visual language.
/// `track` is the full-height rail; thumb size/position derive from the ratio
/// of viewport to content height.
pub fn vscrollbar(
    window: &mut Window,
    track: Bounds<Pixels>,
    offset: f32,
    viewport: f32,
    content: f32,
) {
    if content <= viewport + 0.5 {
        return;
    }
    let rail_w = track.size.width;
    rect(window, track, rail_w.v() / 2.0, theme::PANEL);
    let ratio = (viewport / content).clamp(0.04, 1.0);
    let thumb_h = (track.size.height * ratio).max(px(28.0));
    let travel = track.size.height - thumb_h;
    let max_off = (content - viewport).max(1.0);
    let t = (offset / max_off).clamp(0.0, 1.0);
    let thumb = Bounds::new(
        point(track.origin.x, track.origin.y + travel * t),
        size(rail_w, thumb_h),
    );
    rect(window, thumb, rail_w.v() / 2.0, theme::BORDER_STRONG);
}
