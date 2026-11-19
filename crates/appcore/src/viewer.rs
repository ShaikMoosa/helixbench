//! Ephemeral display settings, scoped to exact document and viewer identities.
use crate::state::{AppState, View};
use domain::DocumentId;
use std::collections::HashMap;
use storage::config::UiPrefs;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Surface {
    CircularMap,
    LinearMap,
    Sequence,
}
impl Surface {
    pub fn limits(self, len: usize) -> (f32, f32) {
        match self {
            Self::CircularMap => (0.5, 2.5),
            Self::LinearMap => (1.0, 10.0),
            Self::Sequence => (1.0, (len as f32 / 20.min(len).max(1) as f32).max(1.0)),
        }
    }
}
pub fn active(app: &AppState) -> Option<Surface> {
    match app.view {
        View::Map if app.map_mode != crate::state::MapMode::Metadata => Some(map_surface(app)),
        View::Sequence => Some(Surface::Sequence),
        _ => None,
    }
}
pub fn map_surface(app: &AppState) -> Surface {
    if app.map_circular() {
        Surface::CircularMap
    } else {
        Surface::LinearMap
    }
}
pub fn supports_layer(surface: Surface, layer: &str) -> bool {
    match layer {
        "features" | "enzymes" | "primers" => true,
        "orfs" => surface != Surface::LinearMap,
        "complement" | "translations" => surface == Surface::Sequence,
        _ => false,
    }
}
pub fn step_zoom(app: &mut AppState, surface: Surface, inward: bool) {
    let width = app.zoom_width;
    let current = settings(app, surface).zoom;
    set_zoom(app, surface, current + if inward { 0.01 } else { -0.01 });
    if surface == Surface::Sequence && width == app.zoom_width {
        let len = app.doc.sequence.len();
        let min = 20.min(len).max(1);
        app.zoom_width = if inward {
            width.saturating_sub(1)
        } else {
            width.saturating_add(1)
        }
        .clamp(min, len.max(1));
        app.center_zoom_on_selection();
        if !app.doc.sequence.is_circular() {
            app.zoom_start = app
                .zoom_start
                .min(app.doc.sequence.len().saturating_sub(app.zoom_width));
        }
    }
}
#[derive(Clone)]
pub struct Settings {
    pub zoom: f32,
    pub rotation: f32,
    pub prefs: UiPrefs,
    pub monochrome: bool,
}
#[derive(Default)]
pub struct State {
    views: HashMap<(DocumentId, Surface), Settings>,
    layers: HashMap<(DocumentId, Surface), HashMap<String, bool>>,
    palettes: HashMap<(DocumentId, Surface), bool>,
}
pub fn settings(app: &AppState, surface: Surface) -> Settings {
    let mut s = app
        .viewer
        .views
        .get(&(app.doc.id, surface))
        .cloned()
        .unwrap_or(Settings {
            zoom: 1.0,
            rotation: 0.0,
            prefs: app.prefs.clone(),
            monochrome: surface == Surface::Sequence
                && !app.prefs.show_colors
                && app.colors_mode == crate::state::ColorsMode::None,
        });
    s.prefs = app.prefs.clone();
    if let Some(overrides) = app.viewer.layers.get(&(app.doc.id, surface)) {
        for (key, value) in overrides {
            match key.as_str() {
                "features" => s.prefs.show_features = *value,
                "enzymes" => s.prefs.show_enzymes = *value,
                "primers" => s.prefs.show_primers = *value,
                "orfs" => s.prefs.show_orfs = *value,
                "complement" => s.prefs.show_complement = *value,
                "translations" => s.prefs.show_translations = *value,
                _ => {}
            }
        }
    }
    if let Some(mono) = app.viewer.palettes.get(&(app.doc.id, surface)) {
        s.monochrome = *mono;
        if surface == Surface::Sequence {
            s.prefs.show_colors = !*mono;
        }
    } else {
        s.monochrome = surface == Surface::Sequence
            && !app.prefs.show_colors
            && app.colors_mode == crate::state::ColorsMode::None;
    }
    if surface == Surface::Sequence {
        s.zoom = (app.doc.sequence.len().max(1) as f32 / app.zoom_width.max(1) as f32).max(1.0);
        s.prefs.show_zoom_controls |= app.zoom_width < app.doc.sequence.len();
    }
    s
}
fn edit(app: &mut AppState, surface: Surface) -> &mut Settings {
    let current = settings(app, surface);
    app.viewer
        .views
        .entry((app.doc.id, surface))
        .or_insert(current)
}
pub fn prefs(app: &AppState, surface: Surface) -> UiPrefs {
    settings(app, surface).prefs
}
pub fn set_zoom(app: &mut AppState, surface: Surface, value: f32) {
    if !value.is_finite() {
        return;
    }
    let len = app.doc.sequence.len();
    let (min, max) = surface.limits(len);
    let value = value.clamp(min, max);
    edit(app, surface).zoom = value;
    if surface == Surface::Sequence {
        app.zoom_width =
            ((len as f32 / value).round() as usize).clamp(20.min(len).max(1), len.max(1));
        app.center_zoom_on_selection();
        if !app.doc.sequence.is_circular() {
            app.zoom_start = app
                .zoom_start
                .min(app.doc.sequence.len().saturating_sub(app.zoom_width));
        }
    }
}
pub fn rotate(app: &mut AppState, surface: Surface, degrees: f32) {
    if surface != Surface::CircularMap || !degrees.is_finite() {
        return;
    }
    let s = edit(app, surface);
    s.rotation = (s.rotation + degrees).rem_euclid(360.0);
}
pub fn fit(app: &mut AppState, surface: Surface) {
    set_zoom(app, surface, 1.0);
    if surface == Surface::Sequence {
        crate::selection::fit(app);
        app.scroll_to = Some(0);
    }
}
pub fn reset(app: &mut AppState, surface: Surface) {
    app.viewer.views.remove(&(app.doc.id, surface));
    app.viewer.layers.remove(&(app.doc.id, surface));
    app.viewer.palettes.remove(&(app.doc.id, surface));
    fit(app, surface);
    edit(app, surface).rotation = 0.0;
}
pub fn effective_prefs(app: &AppState) -> UiPrefs {
    active(app)
        .map(|s| prefs(app, s))
        .unwrap_or_else(|| app.prefs.clone())
}
pub fn layer_key(legacy: &str) -> Option<&'static str> {
    match legacy {
        "show_features" => Some("features"),
        "show_enzymes" => Some("enzymes"),
        "show_primers" => Some("primers"),
        "show_orfs" => Some("orfs"),
        "show_translations" => Some("translations"),
        "show_complement" => Some("complement"),
        _ => None,
    }
}
pub fn toggle_layer(app: &mut AppState, surface: Surface, layer: &str) {
    if !supports_layer(surface, layer) {
        return;
    }
    let p = prefs(app, surface);
    let current = match layer {
        "features" => p.show_features,
        "enzymes" => p.show_enzymes,
        "primers" => p.show_primers,
        "orfs" => p.show_orfs,
        "complement" => p.show_complement,
        "translations" => p.show_translations,
        _ => return,
    };
    app.viewer
        .layers
        .entry((app.doc.id, surface))
        .or_default()
        .insert(layer.into(), !current);
}
pub fn set_palette(app: &mut AppState, surface: Surface, monochrome: bool) {
    app.viewer
        .palettes
        .insert((app.doc.id, surface), monochrome);
}
#[derive(Clone, Copy, Debug)]
pub struct Transform {
    pub zoom: f32,
    pub rotation: f32,
}
impl Transform {
    pub fn forward(self, p: [f32; 2]) -> [f32; 2] {
        let (sin, cos) = self.rotation.to_radians().sin_cos();
        [
            self.zoom * (p[0] * cos - p[1] * sin),
            self.zoom * (p[0] * sin + p[1] * cos),
        ]
    }
    pub fn inverse(self, p: [f32; 2]) -> [f32; 2] {
        let (sin, cos) = self.rotation.to_radians().sin_cos();
        [
            (p[0] * cos + p[1] * sin) / self.zoom,
            (-p[0] * sin + p[1] * cos) / self.zoom,
        ]
    }
}
