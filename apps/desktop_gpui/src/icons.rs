//! Monochrome stroke icons (lucide-style, 24px grid) served from memory and
//! tinted by the caller, so chrome never depends on emoji font fallback.

use std::borrow::Cow;

use gpui::{px, svg, AssetSource, Hsla, SharedString, Styled, Svg};

use appcore::state::View;

const HEAD: &str = "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 24 24\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2\" stroke-linecap=\"round\" stroke-linejoin=\"round\">";
const TAIL: &str = "</svg>";

macro_rules! icons {
    ($($name:literal => $body:literal),* $(,)?) => {
        const TABLE: &[(&str, &str)] = &[$(($name, concat!($body, ""))),*];
        fn body(name: &str) -> Option<&'static str> {
            TABLE.iter().find(|(n, _)| *n == name).map(|(_, b)| *b)
        }
        pub fn names() -> Vec<SharedString> {
            TABLE.iter().map(|(n, _)| SharedString::from(format!("icons/{n}.svg"))).collect()
        }
    };
}

icons! {
    "map" => "<circle cx=\"12\" cy=\"12\" r=\"9\"/><circle cx=\"12\" cy=\"12\" r=\"2.5\" fill=\"currentColor\" stroke=\"none\"/>",
    "sequence" => "<line x1=\"21\" y1=\"6\" x2=\"3\" y2=\"6\"/><line x1=\"15\" y1=\"12\" x2=\"3\" y2=\"12\"/><line x1=\"17\" y1=\"18\" x2=\"3\" y2=\"18\"/>",
    "features" => "<path d=\"M4 15s1-1 4-1 5 2 8 2 4-1 4-1V3s-1 1-4 1-5-2-8-2-4 1-4 1z\"/><line x1=\"4\" y1=\"22\" x2=\"4\" y2=\"15\"/>",
    "primers" => "<line x1=\"5\" y1=\"12\" x2=\"19\" y2=\"12\"/><polyline points=\"12 5 19 12 12 19\"/>",
    "enzymes" => "<circle cx=\"6\" cy=\"6\" r=\"3\"/><circle cx=\"6\" cy=\"18\" r=\"3\"/><line x1=\"20\" y1=\"4\" x2=\"8.12\" y2=\"15.88\"/><line x1=\"14.47\" y1=\"14.48\" x2=\"20\" y2=\"20\"/><line x1=\"8.12\" y1=\"8.12\" x2=\"12\" y2=\"12\"/>",
    "history" => "<polyline points=\"1 4 1 10 7 10\"/><path d=\"M3.51 15a9 9 0 1 0 2.13-9.36L1 10\"/>",
    "plus" => "<line x1=\"12\" y1=\"5\" x2=\"12\" y2=\"19\"/><line x1=\"5\" y1=\"12\" x2=\"19\" y2=\"12\"/>",
    "minus" => "<line x1=\"5\" y1=\"12\" x2=\"19\" y2=\"12\"/>",
    "pencil" => "<path d=\"M17 3a2.85 2.83 0 1 1 4 4L7.5 20.5 2 22l1.5-5.5Z\"/>",
    "trash" => "<polyline points=\"3 6 5 6 21 6\"/><path d=\"M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2\"/><line x1=\"10\" y1=\"11\" x2=\"10\" y2=\"17\"/><line x1=\"14\" y1=\"11\" x2=\"14\" y2=\"17\"/>",
    "scan" => "<path d=\"M3 7V5a2 2 0 0 1 2-2h2\"/><path d=\"M17 3h2a2 2 0 0 1 2 2v2\"/><path d=\"M21 17v2a2 2 0 0 1-2 2h-2\"/><path d=\"M7 21H5a2 2 0 0 1-2-2v-2\"/><circle cx=\"11\" cy=\"11\" r=\"3\"/><line x1=\"16\" y1=\"16\" x2=\"13.2\" y2=\"13.2\"/>",
    "search" => "<circle cx=\"11\" cy=\"11\" r=\"8\"/><line x1=\"21\" y1=\"21\" x2=\"16.65\" y2=\"16.65\"/>",
    "sliders" => "<line x1=\"21\" y1=\"4\" x2=\"14\" y2=\"4\"/><line x1=\"10\" y1=\"4\" x2=\"3\" y2=\"4\"/><line x1=\"21\" y1=\"12\" x2=\"12\" y2=\"12\"/><line x1=\"8\" y1=\"12\" x2=\"3\" y2=\"12\"/><line x1=\"21\" y1=\"20\" x2=\"16\" y2=\"20\"/><line x1=\"12\" y1=\"20\" x2=\"3\" y2=\"20\"/><line x1=\"14\" y1=\"2\" x2=\"14\" y2=\"6\"/><line x1=\"8\" y1=\"10\" x2=\"8\" y2=\"14\"/><line x1=\"16\" y1=\"18\" x2=\"16\" y2=\"22\"/>",
    "x" => "<line x1=\"18\" y1=\"6\" x2=\"6\" y2=\"18\"/><line x1=\"6\" y1=\"6\" x2=\"18\" y2=\"18\"/>",
    "chevron-left" => "<polyline points=\"15 18 9 12 15 6\"/>",
    "chevron-right" => "<polyline points=\"9 18 15 12 9 6\"/>",
    "alert" => "<path d=\"m21.73 18-8-14a2 2 0 0 0-3.48 0l-8 14A2 2 0 0 0 4 21h16a2 2 0 0 0 1.73-3Z\"/><line x1=\"12\" y1=\"9\" x2=\"12\" y2=\"13\"/><line x1=\"12\" y1=\"17\" x2=\"12.01\" y2=\"17\"/>",
    "check" => "<polyline points=\"20 6 9 17 4 12\"/>",
    "zoom-in" => "<circle cx=\"11\" cy=\"11\" r=\"8\"/><line x1=\"21\" y1=\"21\" x2=\"16.65\" y2=\"16.65\"/><line x1=\"11\" y1=\"8\" x2=\"11\" y2=\"14\"/><line x1=\"8\" y1=\"11\" x2=\"14\" y2=\"11\"/>",
    "zoom-out" => "<circle cx=\"11\" cy=\"11\" r=\"8\"/><line x1=\"21\" y1=\"21\" x2=\"16.65\" y2=\"16.65\"/><line x1=\"8\" y1=\"11\" x2=\"14\" y2=\"11\"/>",
    "fit" => "<path d=\"M8 3H5a2 2 0 0 0-2 2v3\"/><path d=\"M21 8V5a2 2 0 0 0-2-2h-3\"/><path d=\"M3 16v3a2 2 0 0 0 2 2h3\"/><path d=\"M16 21h3a2 2 0 0 0 2-2v-3\"/>",
    "crosshair" => "<circle cx=\"12\" cy=\"12\" r=\"9\"/><line x1=\"21\" y1=\"12\" x2=\"17\" y2=\"12\"/><line x1=\"7\" y1=\"12\" x2=\"3\" y2=\"12\"/><line x1=\"12\" y1=\"7\" x2=\"12\" y2=\"3\"/><line x1=\"12\" y1=\"21\" x2=\"12\" y2=\"17\"/>",
    "undo" => "<path d=\"M9 14 4 9l5-5\"/><path d=\"M4 9h10.5a5.5 5.5 0 0 1 5.5 5.5 5.5 5.5 0 0 1-5.5 5.5H11\"/>",
    "redo" => "<path d=\"m15 14 5-5-5-5\"/><path d=\"M20 9H9.5A5.5 5.5 0 0 0 4 14.5 5.5 5.5 0 0 0 9.5 20H13\"/>",
    "orfs" => "<polygon points=\"12 2 2 7 12 12 22 7 12 2\"/><polyline points=\"2 17 12 22 22 17\"/><polyline points=\"2 12 12 17 22 12\"/>",
    "translation" => "<polyline points=\"4 7 4 4 20 4 20 7\"/><line x1=\"9\" y1=\"20\" x2=\"15\" y2=\"20\"/><line x1=\"12\" y1=\"4\" x2=\"12\" y2=\"20\"/>",
    "colors" => "<path d=\"M12 22a10 10 0 1 1 10-10c0 1.66-1.34 3-3 3h-2.6a2.4 2.4 0 0 0-1.8 4 2.4 2.4 0 0 1-1.8 4Z\"/><circle cx=\"13.5\" cy=\"6.5\" r=\"1\" fill=\"currentColor\" stroke=\"none\"/><circle cx=\"17.5\" cy=\"10.5\" r=\"1\" fill=\"currentColor\" stroke=\"none\"/><circle cx=\"8.5\" cy=\"7.5\" r=\"1\" fill=\"currentColor\" stroke=\"none\"/><circle cx=\"6.5\" cy=\"12.5\" r=\"1\" fill=\"currentColor\" stroke=\"none\"/>",
    "circle" => "<circle cx=\"12\" cy=\"12\" r=\"9\"/>",
    "circle-dot" => "<circle cx=\"12\" cy=\"12\" r=\"9\"/><circle cx=\"12\" cy=\"12\" r=\"2.5\" fill=\"currentColor\" stroke=\"none\"/>",
    "split" => "<rect x=\"3\" y=\"4\" width=\"7\" height=\"16\" rx=\"1\"/><rect x=\"14\" y=\"4\" width=\"7\" height=\"16\" rx=\"1\"/>",
    "biologic" => "<path d=\"M12 21v-8\"/><path d=\"M12 13 6 5\"/><path d=\"M12 13l6-8\"/><circle cx=\"6\" cy=\"4\" r=\"1.6\"/><circle cx=\"18\" cy=\"4\" r=\"1.6\"/>",
    "chem" => "<path d=\"M9 3h6\"/><path d=\"M10 3v6l-5 9a2 2 0 0 0 1.8 3h10.4a2 2 0 0 0 1.8-3l-5-9V3\"/><line x1=\"7\" y1=\"15\" x2=\"17\" y2=\"15\"/>",
    "rna" => "<path d=\"M4 4c4 4 12 4 16 8\"/><path d=\"M4 12c4 4 12 4 16 8\"/><line x1=\"7\" y1=\"6\" x2=\"7\" y2=\"10\"/><line x1=\"12\" y1=\"8\" x2=\"12\" y2=\"12\"/><line x1=\"17\" y1=\"10\" x2=\"17\" y2=\"14\"/>",
    "invivo" => "<rect x=\"3\" y=\"7\" width=\"18\" height=\"13\" rx=\"2\"/><path d=\"M8 7V5a4 4 0 0 1 8 0v2\"/><line x1=\"3\" y1=\"12\" x2=\"21\" y2=\"12\"/>",
}

pub struct Assets;

impl AssetSource for Assets {
    fn load(&self, path: &str) -> gpui::Result<Option<Cow<'static, [u8]>>> {
        if path == "logo.png" {
            return Ok(Some(Cow::Borrowed(include_bytes!(
                "../assets/helixbench.png"
            ))));
        }
        if path == "logo.svg" {
            return Ok(Some(Cow::Borrowed(include_bytes!(
                "../assets/helixbench.svg"
            ))));
        }
        let name = path
            .strip_prefix("icons/")
            .and_then(|p| p.strip_suffix(".svg"));
        if name == Some("logo") {
            return Ok(Some(Cow::Borrowed(include_bytes!(
                "../assets/helixbench.svg"
            ))));
        }
        Ok(name.and_then(body).map(|b| {
            let mut s = String::with_capacity(HEAD.len() + b.len() + TAIL.len());
            s.push_str(HEAD);
            s.push_str(b);
            s.push_str(TAIL);
            Cow::Owned(s.into_bytes())
        }))
    }

    fn list(&self, path: &str) -> gpui::Result<Vec<SharedString>> {
        if path == "icons" {
            Ok(names())
        } else {
            Ok(vec![])
        }
    }
}

pub fn icon(name: &str, size: f32, color: Hsla) -> Svg {
    svg()
        .path(SharedString::from(format!("icons/{name}.svg")))
        .w(px(size))
        .h(px(size))
        .text_color(color)
}

pub fn view_icon(v: View) -> &'static str {
    match v {
        View::Map => "map",
        View::Sequence => "sequence",
        View::Features => "features",
        View::Primers => "primers",
        View::Enzymes => "enzymes",
        View::History => "history",
        View::Home => "fit",
        View::Notebook => "pencil",
        View::Registry => "features",
        View::Inventory => "orfs",
        View::Workflows => "redo",
        View::Insights => "translation",
        View::Automation => "sliders",
        View::Split => "split",
        View::Biologics => "biologic",
        View::Chemistry => "chem",
        View::Rna => "rna",
        View::InVivo => "invivo",
    }
}
