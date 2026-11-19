//! Visual language for the GPUI shell: a light, modern laboratory
//! editor. White canvas and chrome, ink text, a single teal accent for
//! selection, and pastel feature fills so annotations read softly on light.
//! Spacing sits on a 4px grid; v6 uses 12px cards and 8px action controls.

use gpui::{hsla, point, px, BoxShadow, Font, Hsla};

/// Compile-time sRGB hex -> Hsla so palette entries are plain `const` values
/// that flow straight into `.bg()`, `.text_color()` and `.border_color()`.
const fn hex(v: u32) -> Hsla {
    let r = ((v >> 16) & 0xff) as f32 / 255.0;
    let g = ((v >> 8) & 0xff) as f32 / 255.0;
    let b = (v & 0xff) as f32 / 255.0;
    let max = if r > g {
        if r > b {
            r
        } else {
            b
        }
    } else if g > b {
        g
    } else {
        b
    };
    let min = if r < g {
        if r < b {
            r
        } else {
            b
        }
    } else if g < b {
        g
    } else {
        b
    };
    let l = (max + min) / 2.0;
    let d = max - min;
    if d == 0.0 {
        return Hsla {
            h: 0.0,
            s: 0.0,
            l,
            a: 1.0,
        };
    }
    let s = if l > 0.5 {
        d / (2.0 - max - min)
    } else {
        d / (max + min)
    };
    let mut h = if max == r {
        (g - b) / d + (if g < b { 6.0 } else { 0.0 })
    } else if max == g {
        (b - r) / d + 2.0
    } else {
        (r - g) / d + 4.0
    };
    h /= 6.0;
    Hsla { h, s, l, a: 1.0 }
}

// --- Light neutral ramp -----------------------------------------------------
pub const BG: Hsla = hex(0xf7f6f3); // app canvas
pub const PANEL: Hsla = hex(0xffffff); // chrome bars / cards
pub const SURFACE: Hsla = hex(0xf1f5f5); // fields / rows
pub const SURFACE_2: Hsla = hex(0xdcf4e9); // hover / selected row
pub const OVERLAY: Hsla = hex(0xffffff); // dialog surface
pub const BORDER: Hsla = hex(0xe6e4df);
pub const BORDER_STRONG: Hsla = hex(0xc6cbc8);
pub const TEXT: Hsla = hex(0x17191a);
pub const MUTED: Hsla = hex(0x666b6e);
// Darkened from #8fa0ae to reach ~4.5:1 on white for 11-12px secondary labels.
pub const FAINT: Hsla = hex(0x6d7274);
pub const SCRIM: Hsla = Hsla {
    h: 0.6,
    s: 0.2,
    l: 0.1,
    a: 0.35,
};

// --- Dark top application bar (the one dark chrome strip) -------------------
pub const TOPBAR: Hsla = hex(0xf7f6f3);
pub const TOPBAR_TEXT: Hsla = hex(0x17191a);
pub const TOPBAR_MUTED: Hsla = hex(0x666b6e);

// --- Single UI accent (instrument teal) ------------------------------------
// Existing scientific/display geometry retains its original accent.
pub const ACCENT: Hsla = hex(0x357d73);
pub const SELECT: Hsla = hex(0x357d73);

// Translucent washes for selection + search hits on light.
pub const SELECT_FILL: Hsla = Hsla {
    h: 0.52,
    s: 0.83,
    l: 0.36,
    a: 0.16,
};
pub const SEARCH_DNA: Hsla = Hsla {
    h: 0.52,
    s: 0.83,
    l: 0.36,
    a: 0.22,
};
pub const SEARCH_PROT: Hsla = Hsla {
    h: 0.58,
    s: 0.7,
    l: 0.5,
    a: 0.22,
};

// --- Base ladder (darkened for legibility on white) -------------------------
pub const BASE_A: Hsla = hex(0x2e9e5b);
pub const BASE_T: Hsla = hex(0xd94f46);
pub const BASE_C: Hsla = hex(0x2f7fd6);
pub const BASE_G: Hsla = hex(0xd99a2b);
pub const BASE_N: Hsla = hex(0x8b97a7);

// --- Semantic tones ---------------------------------------------------------
pub const OK: Hsla = hex(0x286d4e);
pub const WARN: Hsla = hex(0x7a4f00);
pub const DANGER: Hsla = hex(0xb42318);
pub const WARN_SURFACE: Hsla = hex(0xfdf0d5);
pub const NEUTRAL_STATUS: Hsla = hex(0xf1f0ec);
pub const INFO: Hsla = hex(0x2f7fd6);

// --- Alpha-based interaction fills (sit naturally on any surface) ----------
// Mirrors the quiet-md approach: hover/pressed/selected are black-at-alpha
// rather than opaque variants, so they read correctly over panels and rows.
pub const FILL_HOVER: Hsla = Hsla {
    h: 0.0,
    s: 0.0,
    l: 0.0,
    a: 0.045,
};
pub const FILL_HOVER_STRONG: Hsla = Hsla {
    h: 0.0,
    s: 0.0,
    l: 0.0,
    a: 0.06,
};
pub const FILL_PRESSED: Hsla = Hsla {
    h: 0.0,
    s: 0.0,
    l: 0.0,
    a: 0.09,
};
pub const ROW_SELECTED: Hsla = Hsla {
    h: 0.0,
    s: 0.0,
    l: 0.0,
    a: 0.072,
};
pub const HAIRLINE: Hsla = Hsla {
    h: 0.0,
    s: 0.0,
    l: 0.0,
    a: 0.08,
};

// --- Layered elevation (soft two-layer shadows, quiet-md family) -----------
fn shadow(a: f32, y: f32, blur: f32, spread: f32) -> BoxShadow {
    BoxShadow {
        color: hsla(0.0, 0.0, 0.0, a),
        offset: point(px(0.0), px(y)),
        blur_radius: px(blur),
        spread_radius: px(spread),
    }
}

/// Menus / dropdowns / popovers.
pub fn shadow_menu() -> Vec<BoxShadow> {
    vec![shadow(0.05, 1.0, 3.0, 0.0), shadow(0.10, 8.0, 24.0, 0.0)]
}

/// Quiet v6 content-card elevation; transient surfaces retain stronger shadows.
pub fn shadow_card() -> Vec<BoxShadow> {
    vec![shadow(0.03, 1.0, 2.0, 0.0), shadow(0.05, 2.0, 8.0, 0.0)]
}

/// Second-level panels, search popups, toasts.
pub fn shadow_panel() -> Vec<BoxShadow> {
    vec![shadow(0.05, 1.0, 4.0, 0.0), shadow(0.12, 12.0, 32.0, 0.0)]
}

/// Modal dialogs.
pub fn shadow_dialog() -> Vec<BoxShadow> {
    vec![shadow(0.06, 2.0, 8.0, 0.0), shadow(0.16, 24.0, 64.0, 0.0)]
}

// --- ORF strands ------------------------------------------------------------
pub const ORF_TOP: Hsla = hex(0xd97b3f);
pub const ORF_BOTTOM: Hsla = hex(0x2e9e5b);

pub fn base_color(b: u8) -> Hsla {
    match b {
        b'A' => BASE_A,
        b'T' => BASE_T,
        b'C' => BASE_C,
        b'G' => BASE_G,
        _ => BASE_N,
    }
}

pub fn transparent() -> Hsla {
    hsla(0.0, 0.0, 0.0, 0.0)
}

/// Convert a domain `[u8;3]` color into an opaque Hsla.
pub fn rgb_arr(c: [u8; 3]) -> Hsla {
    let v = ((c[0] as u32) << 16) | ((c[1] as u32) << 8) | c[2] as u32;
    hex(v)
}

/// Domain color with an explicit alpha (0..=255).
pub fn rgb_arr_a(c: [u8; 3], alpha: u8) -> Hsla {
    rgb_arr(c).opacity(alpha as f32 / 255.0)
}

/// Pastel variant of a domain color for feature fills on the light canvas:
/// blend toward white so arrows / bars read softly like the reference editor.
pub fn pastel(c: [u8; 3]) -> Hsla {
    let mix = |v: u8| (v as f32 + (255.0 - v as f32) * 0.45) as u8;
    rgb_arr([mix(c[0]), mix(c[1]), mix(c[2])])
}

pub const UI_FONT: &str = "DM Sans";
pub const MONO_FONT: &str = "Consolas";

pub fn ui_font() -> Font {
    gpui::font(ui_family())
}

pub fn mono_font() -> Font {
    gpui::font(mono_family())
}

// Type scale (px). Quiet chrome, one step up for the map title.
pub const T_XS: f32 = 11.0;
pub const T_SM: f32 = 12.0;
pub const T_BASE: f32 = 13.0;
pub const T_MD: f32 = 14.0;
pub const T_LG: f32 = 16.0;
pub const T_XL: f32 = 20.0;

// Adopted UX v6 Soft Gallery roles (reference 1600×1080).
pub const MINT: Hsla = hex(0xdcf4e9);
pub const ACCENT_TEXT: Hsla = hex(0x2a6a60);
pub const NAV_TEXT: Hsla = hex(0x2e3235);
pub const NAV_GROUP_TEXT: Hsla = hex(0x5f6467);
pub const NAV_WIDTH: f32 = 224.0;
pub const INSPECTOR_WIDTH: f32 = 320.0;
pub const PAGE_PAD: f32 = 32.0;
pub const PANEL_PAD: f32 = 24.0;
pub const CARD_RADIUS: f32 = 12.0;
pub const ROW_RADIUS: f32 = 12.0;
pub const T_SECTION: f32 = 19.0;
pub const T_TITLE: f32 = 28.0;
static UI_FAMILY: std::sync::OnceLock<&'static str> = std::sync::OnceLock::new();
static MONO_FAMILY: std::sync::OnceLock<&'static str> = std::sync::OnceLock::new();
pub fn ui_family() -> &'static str {
    UI_FAMILY.get().copied().unwrap_or(UI_FONT)
}
pub fn mono_family() -> &'static str {
    MONO_FAMILY.get().copied().unwrap_or(MONO_FONT)
}
pub fn load_fonts(cx: &gpui::App) -> Result<(), String> {
    let mono = if cx
        .text_system()
        .all_font_names()
        .iter()
        .any(|name| name == "Cascadia Mono")
    {
        "Cascadia Mono"
    } else {
        MONO_FONT
    };
    let _ = MONO_FAMILY.set(mono);
    match cx
        .text_system()
        .add_fonts(vec![std::borrow::Cow::Borrowed(include_bytes!(
            "../assets/fonts/DMSans.ttf"
        ))]) {
        Ok(()) => {
            let _ = UI_FAMILY.set(UI_FONT);
            Ok(())
        }
        Err(error) => {
            let _ = UI_FAMILY.set("Segoe UI");
            Err(format!(
                "DM Sans could not be loaded; using Segoe UI: {error}"
            ))
        }
    }
}

#[cfg(test)]
mod ux_v6_contrast_tests {
    use super::*;

    fn luminance(color: Hsla) -> f64 {
        let color: gpui::Rgba = color.into();
        let linear = |channel: f32| {
            let channel = f64::from(channel);
            if channel <= 0.04045 {
                channel / 12.92
            } else {
                ((channel + 0.055) / 1.055).powf(2.4)
            }
        };
        0.2126 * linear(color.r) + 0.7152 * linear(color.g) + 0.0722 * linear(color.b)
    }

    fn ratio(a: Hsla, b: Hsla) -> f64 {
        let a = luminance(a);
        let b = luminance(b);
        (a.max(b) + 0.05) / (a.min(b) + 0.05)
    }

    #[test]
    fn accent_text_is_readable_on_canvas_white_and_selected_mint() {
        for background in [BG, PANEL, MINT] {
            let actual = ratio(ACCENT_TEXT, background);
            assert!(actual >= 4.5, "Accent text contrast is {actual:.3}:1");
        }
    }

    #[test]
    fn readable_text_does_not_recolor_existing_display_geometry() {
        assert_ne!(
            ACCENT, ACCENT_TEXT,
            "Text and display roles must be independent"
        );
        assert_eq!(ACCENT, SELECT, "Keep the established display accent");
    }

    #[test]
    fn semantic_text_and_action_labels_meet_normal_text_contrast() {
        for foreground in [TEXT, MUTED, FAINT, WARN, DANGER] {
            for background in [BG, PANEL] {
                assert!(ratio(foreground, background) >= 4.5);
            }
        }
        assert!(ratio(PANEL, DANGER) >= 4.5);
        assert!(ratio(PANEL, TEXT) >= 4.5);
        assert!(ratio(WARN, WARN_SURFACE) >= 4.5);
        assert!(ratio(MUTED, NEUTRAL_STATUS) >= 4.5);
        assert!(ratio(NAV_TEXT, PANEL) >= 4.5);
        assert!(ratio(NAV_GROUP_TEXT, PANEL) >= 4.5);
        assert!(ratio(SELECT, MINT) >= 3.0);
    }
}
