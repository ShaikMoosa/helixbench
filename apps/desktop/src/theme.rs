//! HelixBench visual language: a deep-space lab palette derived from fluorescent
//! base colors (A green, T red, C blue, G amber) on blue-black surfaces.

use egui::{Color32, CornerRadius, FontFamily, FontId, Stroke, Style, Visuals};

pub const BG: Color32 = Color32::from_rgb(0x0b, 0x0f, 0x16);
pub const PANEL: Color32 = Color32::from_rgb(0x10, 0x16, 0x21);
pub const SURFACE: Color32 = Color32::from_rgb(0x16, 0x1e, 0x2c);
pub const SURFACE_2: Color32 = Color32::from_rgb(0x1d, 0x27, 0x38);
pub const BORDER: Color32 = Color32::from_rgb(0x2a, 0x35, 0x48);
pub const TEXT: Color32 = Color32::from_rgb(0xe8, 0xee, 0xf7);
pub const MUTED: Color32 = Color32::from_rgb(0x93, 0xa1, 0xb8);
pub const FAINT: Color32 = Color32::from_rgb(0x5c, 0x6a, 0x82);

pub const ACCENT: Color32 = Color32::from_rgb(0x4d, 0xa3, 0xff);
pub const TEAL: Color32 = Color32::from_rgb(0x2d, 0xd4, 0xbf);

pub const BASE_A: Color32 = Color32::from_rgb(0x4a, 0xde, 0x80);
pub const BASE_T: Color32 = Color32::from_rgb(0xf8, 0x71, 0x71);
pub const BASE_C: Color32 = Color32::from_rgb(0x60, 0xa5, 0xfa);
pub const BASE_G: Color32 = Color32::from_rgb(0xfb, 0xbf, 0x24);

pub const SELECT: Color32 = Color32::from_rgb(0x2f, 0x81, 0xf7);
pub const SELECT_FILL: Color32 = Color32::from_rgba_premultiplied(0x14, 0x33, 0x61, 0xb4);
pub const SEARCH_DNA: Color32 = Color32::from_rgba_premultiplied(0x14, 0x53, 0x2d, 0xc8);
pub const SEARCH_PROT: Color32 = Color32::from_rgba_premultiplied(0x17, 0x3a, 0x6b, 0xc8);

pub const ORF_TOP: Color32 = Color32::from_rgb(0xfb, 0x92, 0x3c);
pub const ORF_BOTTOM: Color32 = Color32::from_rgb(0x4a, 0xde, 0x80);

pub const OK: Color32 = Color32::from_rgb(0x3f, 0xb9, 0x50);
pub const WARN: Color32 = Color32::from_rgb(0xd2, 0x99, 0x22);
pub const DANGER: Color32 = Color32::from_rgb(0xf8, 0x51, 0x49);

pub fn base_color(b: u8) -> Color32 {
    match b {
        b'A' => BASE_A,
        b'T' => BASE_T,
        b'C' => BASE_C,
        b'G' => BASE_G,
        _ => MUTED,
    }
}

pub fn rgb(c: [u8; 3]) -> Color32 {
    Color32::from_rgb(c[0], c[1], c[2])
}

/// egui paint color for a history/custom color region, from the shared core's RGB+alpha.
pub trait RegionColorExt {
    fn region_color(&self, r: &domain::ColorRegion) -> Color32;
}

impl RegionColorExt for appcore::state::AppState {
    fn region_color(&self, r: &domain::ColorRegion) -> Color32 {
        let c = self.region_rgb(r);
        let a = self.region_alpha();
        if a == 0xff {
            rgb(c)
        } else {
            Color32::from_rgba_premultiplied(c[0], c[1], c[2], a)
        }
    }
}

pub fn apply(ctx: &egui::Context) {
    let mut style = Style {
        visuals: Visuals::dark(),
        ..Default::default()
    };
    let v = &mut style.visuals;
    v.dark_mode = true;
    v.override_text_color = Some(TEXT);
    v.window_fill = PANEL;
    v.panel_fill = PANEL;
    v.faint_bg_color = SURFACE;
    v.extreme_bg_color = BG;
    v.code_bg_color = SURFACE_2;
    v.window_stroke = Stroke::new(1.0, BORDER);
    v.window_corner_radius = CornerRadius::same(10);
    v.menu_corner_radius = CornerRadius::same(8);
    v.window_shadow = egui::epaint::Shadow {
        offset: [0, 8],
        blur: 24,
        spread: 0,
        color: Color32::from_black_alpha(140),
    };
    v.popup_shadow = egui::epaint::Shadow {
        offset: [0, 4],
        blur: 12,
        spread: 0,
        color: Color32::from_black_alpha(120),
    };
    v.selection.bg_fill = SELECT_FILL;
    v.selection.stroke = Stroke::new(1.0, SELECT);
    v.hyperlink_color = ACCENT;
    v.warn_fg_color = WARN;
    v.error_fg_color = DANGER;
    v.widgets.noninteractive.bg_fill = SURFACE;
    v.widgets.noninteractive.weak_bg_fill = SURFACE;
    v.widgets.noninteractive.bg_stroke = Stroke::new(1.0, BORDER);
    v.widgets.noninteractive.fg_stroke = Stroke::new(1.0, MUTED);
    v.widgets.inactive.bg_fill = SURFACE_2;
    v.widgets.inactive.weak_bg_fill = SURFACE_2;
    v.widgets.inactive.bg_stroke = Stroke::new(1.0, BORDER);
    v.widgets.inactive.fg_stroke = Stroke::new(1.0, TEXT);
    v.widgets.hovered.bg_fill = Color32::from_rgb(0x24, 0x33, 0x4a);
    v.widgets.hovered.weak_bg_fill = Color32::from_rgb(0x24, 0x33, 0x4a);
    v.widgets.hovered.bg_stroke = Stroke::new(1.0, ACCENT);
    v.widgets.hovered.fg_stroke = Stroke::new(1.5, TEXT);
    v.widgets.active.bg_fill = Color32::from_rgb(0x1c, 0x3a, 0x5e);
    v.widgets.active.weak_bg_fill = Color32::from_rgb(0x1c, 0x3a, 0x5e);
    v.widgets.active.bg_stroke = Stroke::new(1.0, ACCENT);
    v.widgets.active.fg_stroke = Stroke::new(1.5, TEXT);
    v.widgets.open.bg_fill = SURFACE_2;
    v.widgets.open.weak_bg_fill = SURFACE_2;
    v.widgets.open.bg_stroke = Stroke::new(1.0, BORDER);
    v.widgets.open.fg_stroke = Stroke::new(1.0, TEXT);
    v.button_frame = true;
    v.indent_has_left_vline = false;
    v.striped = true;
    v.slider_trailing_fill = true;

    style.spacing.item_spacing = egui::vec2(8.0, 6.0);
    style.spacing.button_padding = egui::vec2(10.0, 5.0);
    style.spacing.menu_margin = egui::Margin::same(6);
    style.spacing.window_margin = egui::Margin::same(12);
    style.spacing.interact_size = egui::vec2(40.0, 22.0);
    style.spacing.scroll.floating = true;
    style.spacing.scroll.bar_width = 9.0;
    style.spacing.scroll.floating_allocated_width = 9.0;
    style.spacing.scroll.bar_inner_margin = 2.0;
    // Scrollbars in the artifact's own language: deep track (extreme_bg_color),
    // surface thumb, blue-tinted hover/active states from the widget palette.
    style.spacing.scroll.foreground_color = false;

    style.text_styles.insert(
        egui::TextStyle::Body,
        FontId::new(13.5, FontFamily::Proportional),
    );
    style.text_styles.insert(
        egui::TextStyle::Button,
        FontId::new(13.0, FontFamily::Proportional),
    );
    style.text_styles.insert(
        egui::TextStyle::Small,
        FontId::new(11.5, FontFamily::Proportional),
    );
    style.text_styles.insert(
        egui::TextStyle::Heading,
        FontId::new(19.0, FontFamily::Proportional),
    );
    style.text_styles.insert(
        egui::TextStyle::Monospace,
        FontId::new(13.0, FontFamily::Monospace),
    );

    ctx.set_style_of(egui::Theme::Dark, style.clone());
    ctx.set_style_of(egui::Theme::Light, style);
}

/// Add the Windows symbol font as a fallback so map/tab glyphs (◉ ◧  ➤ ↺
/// ′ ⓟ ⚠) render instead of tofu. Local file only; skipped if absent.
pub fn install_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    if let Ok(bytes) = std::fs::read("C:\\Windows\\Fonts\\seguisym.ttf") {
        fonts.font_data.insert(
            "SegoeSymbols".into(),
            egui::FontData::from_owned(bytes).into(),
        );
        for family in [egui::FontFamily::Proportional, egui::FontFamily::Monospace] {
            fonts
                .families
                .entry(family)
                .or_default()
                .push("SegoeSymbols".into());
        }
        ctx.set_fonts(fonts);
    }
}

/// Procedural 32x32 double-helix window icon.
pub fn icon() -> egui::IconData {
    let n = 32usize;
    let mut rgba = vec![0u8; n * n * 4];
    for y in 0..n {
        let t = y as f32 / (n as f32 - 1.0);
        let phase = t * std::f32::consts::PI * 2.2;
        let x1 = 16.0 + 9.0 * phase.sin();
        let x2 = 16.0 - 9.0 * phase.sin();
        for x in 0..n {
            let xf = x as f32 + 0.5;
            let yf = y as f32 + 0.5;
            let mut put = |cx: f32, col: [u8; 3]| {
                let d = ((xf - cx).powi(2) + (yf - (y as f32 + 0.5)).powi(2)).sqrt();
                if d < 1.9 {
                    let i = (y * n + x) * 4;
                    let a = (1.0 - d / 1.9).min(1.0);
                    rgba[i] = col[0];
                    rgba[i + 1] = col[1];
                    rgba[i + 2] = col[2];
                    rgba[i + 3] = (a * 255.0) as u8;
                }
            };
            put(x1, [0x4d, 0xa3, 0xff]);
            put(x2, [0x2d, 0xd4, 0xbf]);
            if y % 4 == 1 && xf > x1.min(x2) && xf < x1.max(x2) {
                let i = (y * n + x) * 4;
                rgba[i] = 0x93;
                rgba[i + 1] = 0xa1;
                rgba[i + 2] = 0xb8;
                rgba[i + 3] = 200;
            }
        }
    }
    egui::IconData {
        rgba,
        width: n as u32,
        height: n as u32,
    }
}
