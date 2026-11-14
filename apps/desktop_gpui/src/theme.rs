//! Visual language for the GPUI shell: a quiet ink/graphite instrument chrome
//! with a single teal accent, letting the four fluorescence base colors carry
//! the content. Spacing sits on a 4px grid; radii nest (8 outer / 6 inner / 4).

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

// --- Ink / graphite neutral ramp (cool, low-chroma) -------------------------
pub const BG: Hsla = hex(0x0e1116); // app background / deepest field
pub const PANEL: Hsla = hex(0x151a21); // chrome bars
pub const SURFACE: Hsla = hex(0x1b2129); // cards, rows
pub const SURFACE_2: Hsla = hex(0x232b35); // hover / selected row
pub const OVERLAY: Hsla = hex(0x1a2028); // dialog surface
pub const BORDER: Hsla = hex(0x2c3540);
pub const BORDER_STRONG: Hsla = hex(0x3a4552);
pub const TEXT: Hsla = hex(0xe6ebf2);
pub const MUTED: Hsla = hex(0x9aa7b8);
pub const FAINT: Hsla = hex(0x626e7e);
pub const SCRIM: Hsla = Hsla {
    h: 0.6,
    s: 0.18,
    l: 0.03,
    a: 0.62,
};

// --- Single UI accent (instrument teal) ------------------------------------
pub const ACCENT: Hsla = hex(0x33c6b3);
pub const SELECT: Hsla = hex(0x33c6b3);

// Translucent washes for the sequence ladder (selection + search hits).
pub const SELECT_FILL: Hsla = Hsla {
    h: 0.47,
    s: 0.58,
    l: 0.49,
    a: 0.24,
};
pub const SEARCH_DNA: Hsla = Hsla {
    h: 0.40,
    s: 0.62,
    l: 0.50,
    a: 0.34,
};
pub const SEARCH_PROT: Hsla = Hsla {
    h: 0.58,
    s: 0.70,
    l: 0.55,
    a: 0.34,
};

// --- Fluorescence base ladder (the content moment) --------------------------
pub const BASE_A: Hsla = hex(0x57d98a); // green
pub const BASE_T: Hsla = hex(0xf2685f); // red / coral
pub const BASE_C: Hsla = hex(0x5aa2f0); // blue
pub const BASE_G: Hsla = hex(0xe8b13c); // amber
pub const BASE_N: Hsla = hex(0x8b97a7); // ambiguous / neutral

// --- Semantic tones ---------------------------------------------------------
pub const OK: Hsla = hex(0x46c46e);
pub const WARN: Hsla = hex(0xd9a13a);
pub const DANGER: Hsla = hex(0xef5350);
pub const INFO: Hsla = hex(0x5aa2f0);

// --- ORF strands ------------------------------------------------------------
pub const ORF_TOP: Hsla = hex(0xe8935a);
pub const ORF_BOTTOM: Hsla = hex(0x57d98a);

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
