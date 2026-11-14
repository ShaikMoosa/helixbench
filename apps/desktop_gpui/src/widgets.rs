//! Small styled building blocks for the chrome: buttons, tabs, toggles, menu
//! rows and labels. All sit on the 4px grid and borrow the ink/teal palette.

use gpui::{div, prelude::*, px, Div, ElementId, Hsla, Stateful};

use crate::{icons, theme};

#[derive(Clone, Copy)]
pub enum Btn {
    Primary,
    Outline,
    Ghost,
    Danger,
    Subtle,
}

fn variant_text(variant: Btn) -> Hsla {
    match variant {
        Btn::Primary => theme::BG,
        Btn::Danger => theme::DANGER,
        Btn::Subtle => theme::MUTED,
        _ => theme::TEXT,
    }
}

fn button_base(id: impl Into<ElementId>, variant: Btn) -> Stateful<Div> {
    let base = div()
        .id(id)
        .flex()
        .items_center()
        .justify_center()
        .h(px(30.0))
        .px(px(12.0))
        .rounded(px(6.0))
        .text_size(px(theme::T_BASE))
        .cursor_pointer();
    match variant {
        Btn::Primary => base
            .bg(theme::ACCENT)
            .text_color(theme::BG)
            .hover(|s| s.bg(theme::ACCENT.opacity(0.88))),
        Btn::Outline => base
            .bg(theme::SURFACE)
            .border_1()
            .border_color(theme::BORDER_STRONG)
            .text_color(theme::TEXT)
            .hover(|s| s.bg(theme::SURFACE_2)),
        Btn::Ghost => base
            .bg(theme::transparent())
            .text_color(theme::TEXT)
            .hover(|s| s.bg(theme::SURFACE_2)),
        Btn::Danger => base
            .bg(theme::transparent())
            .border_1()
            .border_color(theme::DANGER.opacity(0.6))
            .text_color(theme::DANGER)
            .hover(|s| s.bg(theme::DANGER.opacity(0.12))),
        Btn::Subtle => base
            .bg(theme::SURFACE)
            .text_color(theme::MUTED)
            .hover(|s| s.bg(theme::SURFACE_2).text_color(theme::TEXT)),
    }
}

pub fn button(id: impl Into<ElementId>, label: &str, variant: Btn) -> Stateful<Div> {
    button_base(id, variant).child(label.to_owned())
}

pub fn icon_button(
    id: impl Into<ElementId>,
    icon: &str,
    label: &str,
    variant: Btn,
) -> Stateful<Div> {
    button_base(id, variant)
        .gap(px(7.0))
        .child(icons::icon(icon, 14.0, variant_text(variant)))
        .child(label.to_owned())
}

pub fn icon_only(id: impl Into<ElementId>, icon: &str, variant: Btn) -> Stateful<Div> {
    button_base(id, variant)
        .w(px(30.0))
        .px(px(0.0))
        .child(icons::icon(icon, 14.0, variant_text(variant)))
}

pub fn menu_row(
    id: impl Into<ElementId>,
    label: &str,
    shortcut: &str,
    enabled: bool,
) -> Stateful<Div> {
    let d = div()
        .id(id)
        .flex()
        .items_center()
        .justify_between()
        .gap(px(24.0))
        .w_full()
        .h(px(28.0))
        .px(px(10.0))
        .rounded(px(4.0))
        .text_size(px(theme::T_BASE))
        .cursor_pointer()
        .child(label.to_owned())
        .child(
            div()
                .text_size(px(theme::T_XS))
                .text_color(theme::FAINT)
                .child(shortcut.to_owned()),
        );
    if enabled {
        d.text_color(theme::TEXT).hover(|s| s.bg(theme::SURFACE_2))
    } else {
        d.text_color(theme::FAINT).opacity(0.6)
    }
}

/// A menu row with a leading check mark slot (for toggles / radio choices).
pub fn menu_check(id: impl Into<ElementId>, label: &str, checked: bool) -> Stateful<Div> {
    div()
        .id(id)
        .flex()
        .items_center()
        .gap(px(8.0))
        .w_full()
        .h(px(28.0))
        .px(px(10.0))
        .rounded(px(4.0))
        .text_size(px(theme::T_BASE))
        .cursor_pointer()
        .text_color(theme::TEXT)
        .hover(|s| s.bg(theme::SURFACE_2))
        .child(
            div()
                .w(px(14.0))
                .text_color(theme::ACCENT)
                .child(if checked { "✓" } else { "" }.to_owned()),
        )
        .child(label.to_owned())
}

pub fn menu_section(text: &str) -> Div {
    div()
        .px(px(10.0))
        .pt(px(6.0))
        .pb(px(2.0))
        .text_size(px(theme::T_XS))
        .text_color(theme::FAINT)
        .child(text.to_owned())
}

pub fn divider() -> Div {
    div().h(px(1.0)).my(px(4.0)).bg(theme::BORDER)
}

pub fn label(text: &str, size: f32, color: Hsla) -> Div {
    div()
        .text_size(px(size))
        .text_color(color)
        .child(text.to_owned())
}
