//! Visual language for the GPUI shell: a light, Benchling-style laboratory
//! editor. White canvas and chrome, ink text, a single teal accent for
//! selection, and pastel feature fills so annotations read softly on light.
//! Spacing sits on a 4px grid; radii nest (8 outer / 6 inner / 4).

use gpui::{hsla, Font, Hsla};

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
pub const BG: Hsla = hex(0xf7f9fa); // app canvas
pub const PANEL: Hsla = hex(0xffffff); // chrome bars / cards
pub const SURFACE: Hsla = hex(0xf1f4f6); // fields / rows
pub const SURFACE_2: Hsla = hex(0xe4e9ee); // hover / selected row
pub const OVERLAY: Hsla = hex(0xffffff); // dialog surface
pub const BORDER: Hsla = hex(0xdde3e8);
pub const BORDER_STRONG: Hsla = hex(0xb9c4cd);
pub const TEXT: Hsla = hex(0x1a2430);
pub const MUTED: Hsla = hex(0x5a6b7c);
pub const FAINT: Hsla = hex(0x8fa0ae);
pub const SCRIM: Hsla = Hsla {
    h: 0.6,
    s: 0.2,
    l: 0.1,
    a: 0.35,
};

// --- Dark top application bar (the one dark chrome strip) -------------------
pub const TOPBAR: Hsla = hex(0x0d1b2a);
pub const TOPBAR_TEXT: Hsla = hex(0xe8eef4);
pub const TOPBAR_MUTED: Hsla = hex(0x9db0c0);

// --- Single UI accent (instrument teal) ------------------------------------
pub const ACCENT: Hsla = hex(0x0e9aa8);
pub const SELECT: Hsla = hex(0x0e9aa8);

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
pub const OK: Hsla = hex(0x2f9e57);
pub const WARN: Hsla = hex(0xc08a20);
pub const DANGER: Hsla = hex(0xd64545);
pub const INFO: Hsla = hex(0x2f7fd6);

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

pub const UI_FONT: &str = ".SystemUIFont";
pub const MONO_FONT: &str = "Consolas";

pub fn ui_font() -> Font {
    gpui::font(UI_FONT)
}

pub fn mono_font() -> Font {
    gpui::font(MONO_FONT)
}

// Type scale (px). Quiet chrome, one step up for the map title.
pub const T_XS: f32 = 11.0;
pub const T_SM: f32 = 12.0;
pub const T_BASE: f32 = 13.0;
pub const T_MD: f32 = 14.0;
pub const T_LG: f32 = 16.0;
pub const T_XL: f32 = 20.0;
