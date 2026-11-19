//! Native viewer chrome; display controls never dispatch scientific edits.
use crate::{
    app::HelixBench,
    draw::{self, PxVal},
    icons, theme,
    widgets::{self, Btn},
};
use appcore::viewer::{self, Surface};
use domain::DocumentId;
use gpui::{
    anchored, canvas, deferred, div, point, prelude::*, px, size, AnyElement, Bounds, Context,
    Corner, Div, FocusHandle, MouseButton, Pixels, SharedString, Stateful,
};
use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq)]
pub enum Panel {
    Zoom,
    Layers,
    Palette,
    Options,
}
#[derive(Clone, Copy, PartialEq)]
enum Slider {
    Zoom,
    Rotation,
}
#[derive(Clone, Copy)]
struct Drag {
    document: DocumentId,
    surface: Surface,
    kind: Slider,
    bounds: Bounds<Pixels>,
}
#[derive(Default)]
pub struct Ui {
    context: Option<(DocumentId, Surface)>,
    panel: Option<Panel>,
    surface_bounds: Option<Bounds<Pixels>>,
    zoom_bounds: Option<Bounds<Pixels>>,
    rotation_bounds: Option<Bounds<Pixels>>,
    drag: Option<Drag>,
    focus: HashMap<String, FocusHandle>,
}
impl Ui {
    pub fn sync_context(&mut self, context: Option<(DocumentId, Surface)>) {
        if self.context != context {
            self.context = context;
            self.panel = None;
            self.drag = None;
        }
    }
    fn drag_allowed(&self, pressed: Option<MouseButton>) -> bool {
        pressed == Some(MouseButton::Left)
            && self
                .drag
                .is_some_and(|d| self.context == Some((d.document, d.surface)))
    }
}
fn focus(app: &mut HelixBench, id: &str, cx: &mut Context<HelixBench>) -> FocusHandle {
    app.viewer_ui
        .focus
        .entry(id.into())
        .or_insert_with(|| cx.focus_handle().tab_stop(true))
        .clone()
}
fn control(
    app: &mut HelixBench,
    cx: &mut Context<HelixBench>,
    id: &'static str,
    label: String,
    asset: Option<&str>,
    action: impl Fn(&mut HelixBench, &mut Context<HelixBench>) + 'static,
) -> Stateful<Div> {
    let handle = focus(app, id, cx);
    let mut button = div()
        .id(id)
        .h(px(32.0))
        .min_w(px(32.0))
        .px(px(6.0))
        .flex()
        .items_center()
        .justify_center()
        .rounded(px(6.0))
        .cursor_pointer()
        .text_size(px(12.0))
        .track_focus(&handle)
        .hover(|s| s.bg(theme::FILL_HOVER))
        .focus(|s| s.bg(theme::MINT).border_1().border_color(theme::ACCENT))
        .on_click(cx.listener(move |t, _, window, cx| {
            let had = t.viewer_ui.panel.is_some();
            action(t, cx);
            if had && t.viewer_ui.panel.is_none() {
                t.focus_handle.focus(window);
            }
        }));
    if let Some(asset) = asset {
        button = button.child(icons::v6_asset(asset, 18.0, 18.0, theme::NAV_TEXT));
    } else {
        button = button.child(label.clone());
    }
    button.tooltip(move |_, cx| cx.new(|_| Tip(label.clone())).into())
}
fn task_button(
    app: &mut HelixBench,
    cx: &mut Context<HelixBench>,
    id: &'static str,
    label: &str,
    variant: Btn,
    action: impl Fn(&mut HelixBench, &mut Context<HelixBench>) + 'static,
) -> Stateful<Div> {
    let handle = focus(app, id, cx);
    widgets::button(id, label, variant)
        .track_focus(&handle)
        .focus(|s| s.bg(theme::MINT))
        .on_click(cx.listener(move |t, _, window, cx| {
            let had = t.viewer_ui.panel.is_some();
            action(t, cx);
            if had && t.viewer_ui.panel.is_none() {
                t.focus_handle.focus(window);
            }
        }))
}
struct Tip(String);
impl gpui::Render for Tip {
    fn render(&mut self, _: &mut gpui::Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .p(px(8.0))
            .rounded(px(6.0))
            .bg(theme::TEXT)
            .text_color(theme::PANEL)
            .text_size(px(12.0))
            .child(self.0.clone())
    }
}
fn toggle(app: &mut HelixBench, panel: Panel) {
    app.viewer_ui.panel = if app.viewer_ui.panel == Some(panel) {
        None
    } else {
        Some(panel)
    };
}
fn choice(
    app: &mut HelixBench,
    cx: &mut Context<HelixBench>,
    id: String,
    label: &str,
    checked: bool,
    action: impl Fn(&mut HelixBench, &mut Context<HelixBench>) + 'static,
) -> Stateful<Div> {
    let handle = focus(app, &id, cx);
    widgets::menu_check(SharedString::from(id), label, checked)
        .h(px(32.0))
        .track_focus(&handle)
        .focus(|s| s.bg(theme::MINT))
        .on_click(cx.listener(move |t, _, window, cx| {
            let had = t.viewer_ui.panel.is_some();
            action(t, cx);
            if had && t.viewer_ui.panel.is_none() {
                t.focus_handle.focus(window);
            }
        }))
}
pub fn options_button(app: &mut HelixBench, cx: &mut Context<HelixBench>) -> AnyElement {
    task_button(
        app,
        cx,
        "viewer-options",
        "View options",
        Btn::Outline,
        |t, cx| {
            toggle(t, Panel::Options);
            cx.notify();
        },
    )
    .into_any_element()
}
fn toolbar(app: &mut HelixBench, surface: Surface, cx: &mut Context<HelixBench>) -> AnyElement {
    let zoom = viewer::settings(&app.app, surface).zoom;
    let mut bar = div()
        .occlude()
        .flex()
        .items_center()
        .gap(px(2.0))
        .p(px(4.0))
        .rounded(px(10.0))
        .bg(theme::PANEL)
        .border_1()
        .border_color(theme::BORDER)
        .shadow(theme::shadow_card());
    bar = bar
        .child(control(
            app,
            cx,
            "viewer-out",
            "Zoom out (-)".into(),
            Some("zoom-out"),
            move |t, cx| {
                let z = viewer::settings(&t.app, surface).zoom;
                viewer::set_zoom(&mut t.app, surface, z / 1.15);
                cx.notify();
            },
        ))
        .child(control(
            app,
            cx,
            "viewer-value",
            format!("{:.0}%", zoom * 100.0),
            None,
            |t, cx| {
                toggle(t, Panel::Zoom);
                cx.notify();
            },
        ))
        .child(control(
            app,
            cx,
            "viewer-in",
            "Zoom in (+)".into(),
            Some("zoom-in"),
            move |t, cx| {
                let z = viewer::settings(&t.app, surface).zoom;
                viewer::set_zoom(&mut t.app, surface, z * 1.15);
                cx.notify();
            },
        ))
        .child(control(
            app,
            cx,
            "viewer-fit",
            "Fit to view (0)".into(),
            Some("fit"),
            move |t, cx| {
                viewer::fit(&mut t.app, surface);
                cx.notify();
            },
        ));
    if surface == Surface::CircularMap {
        bar = bar
            .child(control(
                app,
                cx,
                "viewer-left",
                "Rotate left 15 degrees".into(),
                Some("rotate-left"),
                move |t, cx| {
                    viewer::rotate(&mut t.app, surface, -15.0);
                    cx.notify();
                },
            ))
            .child(control(
                app,
                cx,
                "viewer-right",
                "Rotate right 15 degrees (R)".into(),
                Some("rotate-right"),
                move |t, cx| {
                    viewer::rotate(&mut t.app, surface, 15.0);
                    cx.notify();
                },
            ));
    }
    bar.child(control(
        app,
        cx,
        "viewer-layers",
        "Visible layers".into(),
        Some("layers"),
        |t, cx| {
            toggle(t, Panel::Layers);
            cx.notify();
        },
    ))
    .child(control(
        app,
        cx,
        "viewer-palette",
        "Color scheme".into(),
        Some("palette"),
        |t, cx| {
            toggle(t, Panel::Palette);
            cx.notify();
        },
    ))
    .into_any_element()
}
fn apply_drag(app: &mut HelixBench, x: Pixels) {
    let Some(drag) = app.viewer_ui.drag else {
        return;
    };
    if drag.document != app.app.doc.id || viewer::active(&app.app) != Some(drag.surface) {
        app.viewer_ui.drag = None;
        return;
    }
    let fraction =
        ((x - drag.bounds.left()).v() / drag.bounds.size.width.v().max(1.0)).clamp(0.0, 1.0);
    if drag.kind == Slider::Zoom {
        let (min, max) = drag.surface.limits(app.app.doc.sequence.len());
        viewer::set_zoom(&mut app.app, drag.surface, min + fraction * (max - min));
    } else {
        let current = viewer::settings(&app.app, drag.surface).rotation;
        viewer::rotate(&mut app.app, drag.surface, fraction * 359.0 - current);
    }
}
fn slider(
    app: &mut HelixBench,
    surface: Surface,
    kind: Slider,
    cx: &mut Context<HelixBench>,
) -> AnyElement {
    let s = viewer::settings(&app.app, surface);
    let (min, max) = surface.limits(app.app.doc.sequence.len());
    let fraction = if kind == Slider::Zoom {
        (s.zoom - min) / (max - min).max(0.001)
    } else {
        s.rotation / 359.0
    };
    let e = cx.entity();
    let id = if kind == Slider::Zoom {
        "viewer-zoom-slider"
    } else {
        "viewer-rotation-slider"
    };
    let handle = focus(app, id, cx);
    div()
        .id(id)
        .h(px(32.0))
        .w_full()
        .cursor_pointer()
        .track_focus(&handle)
        .focus(|s| s.bg(theme::MINT).rounded(px(6.0)))
        .child(
            canvas(
                move |bounds, _, cx| {
                    e.update(cx, |t, _| {
                        if kind == Slider::Zoom {
                            t.viewer_ui.zoom_bounds = Some(bounds)
                        } else {
                            t.viewer_ui.rotation_bounds = Some(bounds)
                        }
                    });
                },
                move |bounds, _, window, _| {
                    let y = bounds.center().y;
                    let w = bounds.size.width.v();
                    draw::rect(
                        window,
                        Bounds::new(point(bounds.left(), y - px(2.0)), size(px(w), px(4.0))),
                        2.0,
                        theme::BORDER_STRONG,
                    );
                    draw::rect(
                        window,
                        Bounds::new(
                            point(bounds.left(), y - px(2.0)),
                            size(px(w * fraction.clamp(0.0, 1.0)), px(4.0)),
                        ),
                        2.0,
                        theme::ACCENT,
                    );
                    draw::disc(
                        window,
                        point(bounds.left() + px(w * fraction.clamp(0.0, 1.0)), y),
                        6.0,
                        theme::TEXT,
                    );
                },
            )
            .size_full(),
        )
        .on_mouse_down(
            MouseButton::Left,
            cx.listener(move |t, ev: &gpui::MouseDownEvent, window, cx| {
                let bounds = if kind == Slider::Zoom {
                    t.viewer_ui.zoom_bounds
                } else {
                    t.viewer_ui.rotation_bounds
                };
                if let Some(bounds) = bounds {
                    t.viewer_ui.drag = Some(Drag {
                        document: t.app.doc.id,
                        surface,
                        kind,
                        bounds,
                    });
                    handle.focus(window);
                    apply_drag(t, ev.position.x);
                    cx.notify();
                }
                cx.stop_propagation();
            }),
        )
        .on_mouse_move(cx.listener(|t, ev: &gpui::MouseMoveEvent, _, cx| {
            if t.viewer_ui.drag_allowed(ev.pressed_button) {
                apply_drag(t, ev.position.x);
                cx.notify();
            }
        }))
        .on_mouse_up(
            MouseButton::Left,
            cx.listener(|t, _, _, cx| {
                t.viewer_ui.drag = None;
                cx.notify();
            }),
        )
        .on_mouse_up_out(
            MouseButton::Left,
            cx.listener(|t, _, _, cx| {
                t.viewer_ui.drag = None;
                cx.notify();
            }),
        )
        .on_key_down(cx.listener(move |t, ev: &gpui::KeyDownEvent, _, cx| {
            let s = viewer::settings(&t.app, surface);
            let key = ev.keystroke.key.as_str();
            if !matches!(key, "left" | "right" | "home" | "end") {
                return;
            }
            if kind == Slider::Zoom {
                let (lo, hi) = surface.limits(t.app.doc.sequence.len());
                let v = match key {
                    "home" => lo,
                    "end" => hi,
                    "left" => s.zoom - 0.01,
                    _ => s.zoom + 0.01,
                };
                if matches!(key, "left" | "right") {
                    viewer::step_zoom(&mut t.app, surface, key == "right");
                } else {
                    viewer::set_zoom(&mut t.app, surface, v);
                }
            } else {
                let target = match key {
                    "home" => 0.0,
                    "end" => 359.0,
                    "left" => s.rotation - 1.0,
                    _ => s.rotation + 1.0,
                };
                viewer::rotate(&mut t.app, surface, target - s.rotation);
            }
            cx.stop_propagation();
            cx.notify();
        }))
        .into_any_element()
}
fn panel(
    app: &mut HelixBench,
    surface: Surface,
    kind: Panel,
    cx: &mut Context<HelixBench>,
) -> AnyElement {
    let s = viewer::settings(&app.app, surface);
    let mut p = div().flex().flex_col().gap(px(10.0)).flex_shrink_0();
    let title = match kind {
        Panel::Zoom => "Zoom and orientation",
        Panel::Layers => "Visible layers",
        Panel::Palette => "Color scheme",
        Panel::Options => "View options",
    };
    match kind {
        Panel::Zoom => {
            p = p
                .child(widgets::label(
                    &format!("Zoom: {:.0}%", s.zoom * 100.0),
                    theme::T_BASE,
                    theme::TEXT,
                ))
                .child(slider(app, surface, Slider::Zoom, cx));
            if surface == Surface::CircularMap {
                p = p
                    .child(widgets::label(
                        &format!("Rotation: {:.0} degrees", s.rotation),
                        theme::T_BASE,
                        theme::TEXT,
                    ))
                    .child(slider(app, surface, Slider::Rotation, cx));
            }
            p = p.child(widgets::label(
                "Drag continuously. Arrow keys adjust; Home/End reach the limits.",
                theme::T_SM,
                theme::MUTED,
            ));
        }
        Panel::Layers => {
            for (key, label, visible) in [
                ("features", "Features", s.prefs.show_features),
                ("enzymes", "Restriction sites", s.prefs.show_enzymes),
                ("primers", "Primers", s.prefs.show_primers),
                ("orfs", "Open reading frames", s.prefs.show_orfs),
                ("complement", "Complement strand", s.prefs.show_complement),
                ("translations", "Translations", s.prefs.show_translations),
            ] {
                if !viewer::supports_layer(surface, key) {
                    continue;
                }
                p = p.child(choice(
                    app,
                    cx,
                    format!("viewer-layer-{key}"),
                    label,
                    visible,
                    move |t, cx| {
                        viewer::toggle_layer(&mut t.app, surface, key);
                        cx.notify();
                    },
                ));
            }
        }
        Panel::Palette => {
            for (label, mono) in [
                (
                    if surface == Surface::Sequence {
                        "Base colors"
                    } else {
                        "Annotation colors"
                    },
                    false,
                ),
                (
                    if surface == Surface::Sequence {
                        "Plain bases"
                    } else {
                        "Single teal"
                    },
                    true,
                ),
            ] {
                p = p.child(choice(
                    app,
                    cx,
                    format!("viewer-color-{mono}"),
                    label,
                    s.monochrome == mono,
                    move |t, cx| {
                        viewer::set_palette(&mut t.app, surface, mono);
                        cx.notify();
                    },
                ));
            }
            p = p.child(widgets::label(
                "Changes display only; saved annotation colors remain unchanged.",
                theme::T_SM,
                theme::MUTED,
            ));
        }
        Panel::Options => {
            if surface != Surface::Sequence {
                for (mode, label) in [
                    (appcore::state::MapMode::Auto, "Auto representation"),
                    (appcore::state::MapMode::Plasmid, "Circular representation"),
                    (appcore::state::MapMode::Linear, "Linear representation"),
                    (appcore::state::MapMode::Metadata, "Metadata"),
                ] {
                    p = p.child(choice(
                        app,
                        cx,
                        format!("viewer-representation-{label}"),
                        label,
                        app.app.map_mode == mode,
                        move |t, cx| {
                            appcore::shell::set_representation(&mut t.app, mode);
                            t.viewer_ui.panel = None;
                            cx.notify();
                        },
                    ));
                }
                p = p.child(choice(
                    app,
                    cx,
                    "viewer-preview".into(),
                    "Sequence preview",
                    app.app.shell.sequence_preview,
                    |t, cx| {
                        t.app.shell.sequence_preview = !t.app.shell.sequence_preview;
                        t.viewer_ui.panel = None;
                        cx.notify();
                    },
                ));
            }
        }
    }
    let height = (app.viewport.height.v() - 120.0).clamp(160.0, 380.0);
    let footer = div()
        .flex()
        .items_center()
        .justify_between()
        .gap(px(8.0))
        .flex_shrink_0()
        .child(task_button(
            app,
            cx,
            "viewer-reset",
            "Reset view",
            Btn::Outline,
            move |t, cx| {
                viewer::reset(&mut t.app, surface);
                if surface == Surface::Sequence {
                    t.seq_scroll = 0.0;
                }
                cx.notify();
            },
        ))
        .child(task_button(
            app,
            cx,
            "viewer-close-panel",
            "Close",
            Btn::Ghost,
            |t, cx| {
                t.viewer_ui.panel = None;
                t.viewer_ui.drag = None;
                cx.notify();
            },
        ));
    div()
        .id("viewer-panel")
        .occlude()
        .flex()
        .flex_col()
        .gap(px(10.0))
        .w(px(300.0))
        .h(px(height))
        .p(px(16.0))
        .bg(theme::PANEL)
        .border_1()
        .border_color(theme::BORDER)
        .rounded(px(12.0))
        .shadow(theme::shadow_card())
        .child(widgets::section_title(title).flex_shrink_0())
        .child(
            div()
                .id("viewer-panel-scroll")
                .flex()
                .flex_col()
                .w_full()
                .flex_1()
                .min_h(px(0.0))
                .overflow_y_scroll()
                .child(p.w_full()),
        )
        .child(footer)
        .into_any_element()
}

fn wrap(
    app: &mut HelixBench,
    surface: Surface,
    body: AnyElement,
    cx: &mut Context<HelixBench>,
) -> AnyElement {
    let entity = cx.entity();
    let card = div()
        .relative()
        .size_full()
        .bg(theme::PANEL)
        .rounded(px(12.0))
        .overflow_hidden()
        .shadow(theme::shadow_card())
        .on_mouse_move(cx.listener(|t, ev: &gpui::MouseMoveEvent, _, cx| {
            if t.viewer_ui.drag_allowed(ev.pressed_button) {
                apply_drag(t, ev.position.x);
                cx.notify();
            }
        }))
        .on_mouse_up(
            MouseButton::Left,
            cx.listener(|t, _, _, cx| {
                t.viewer_ui.drag = None;
                cx.notify();
            }),
        )
        .on_mouse_up_out(
            MouseButton::Left,
            cx.listener(|t, _, _, cx| {
                t.viewer_ui.drag = None;
                cx.notify();
            }),
        )
        .child(
            canvas(
                move |bounds, _, cx| {
                    entity.update(cx, |t, _| t.viewer_ui.surface_bounds = Some(bounds));
                },
                |_, _, _, _| {},
            )
            .absolute()
            .size_full(),
        )
        .child(body)
        .child(
            div()
                .absolute()
                .bottom(px(12.0))
                .right(px(12.0))
                .child(toolbar(app, surface, cx)),
        );
    let mut root = div().size_full().px(px(28.0)).py(px(12.0)).child(card);
    if let (Some(kind), Some(bounds)) = (app.viewer_ui.panel, app.viewer_ui.surface_bounds) {
        root = root.child(
            deferred(
                anchored()
                    .anchor(Corner::BottomRight)
                    .position(point(bounds.right() - px(12.0), bounds.bottom() - px(60.0)))
                    .snap_to_window()
                    .child(panel(app, surface, kind, cx)),
            )
            .with_priority(48),
        );
    }
    root.into_any_element()
}

pub fn map_page(app: &mut HelixBench, cx: &mut Context<HelixBench>) -> AnyElement {
    if app.app.map_mode == appcore::state::MapMode::Metadata {
        return crate::editor::render_editor(app, cx);
    }
    let surface = viewer::map_surface(&app.app);
    let body = if app.app.shell.sequence_preview {
        crate::editor::render_editor(app, cx)
    } else {
        app.render_map(cx)
    };
    wrap(app, surface, body, cx)
}
pub fn sequence_page(app: &mut HelixBench, cx: &mut Context<HelixBench>) -> AnyElement {
    let body = app.render_sequence(cx);
    wrap(app, Surface::Sequence, body, cx)
}
pub fn key(app: &mut HelixBench, key: &gpui::Keystroke) -> bool {
    let Some(surface) = viewer::active(&app.app) else {
        return false;
    };
    if key.modifiers.control || key.modifiers.alt || key.modifiers.platform {
        return false;
    }
    let name = key.key_char.as_deref().unwrap_or(&key.key);
    match name {
        "+" | "=" => {
            let z = viewer::settings(&app.app, surface).zoom;
            viewer::set_zoom(&mut app.app, surface, z * 1.15);
        }
        "-" => {
            let z = viewer::settings(&app.app, surface).zoom;
            viewer::set_zoom(&mut app.app, surface, z / 1.15);
        }
        "0" => viewer::fit(&mut app.app, surface),
        "r" | "R" if surface == Surface::CircularMap => viewer::rotate(&mut app.app, surface, 15.0),
        _ => return false,
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn leaving_a_viewer_clears_a_drag_before_returning_to_the_same_document() {
        let doc = DocumentId::new();
        let surface = Surface::CircularMap;
        let mut ui = Ui::default();
        ui.sync_context(Some((doc, surface)));
        ui.drag = Some(Drag {
            document: doc,
            surface,
            kind: Slider::Zoom,
            bounds: Bounds::new(point(px(0.0), px(0.0)), size(px(200.0), px(32.0))),
        });
        assert!(ui.drag_allowed(Some(MouseButton::Left)));
        assert!(!ui.drag_allowed(None));
        ui.sync_context(None);
        ui.sync_context(Some((doc, surface)));
        assert!(ui.drag.is_none());
    }
}
