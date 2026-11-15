# HelixBench — Design Contract

The visual language of the GPUI shell: a light, quiet, content-first laboratory
editor. White canvas and chrome, ink text, a single instrument-teal accent,
pastel feature fills. This document is the source of truth for tokens, type,
spacing, iconography, elevation, states and motion. It is informed by a design
review of the GPUI reference app `quiet-md` (OKLab dark adaptation, alpha state
fills, layered shadows, measured neutrals).

## Principles
- Content first: chrome is quiet; the sequence / document is the focus.
- One accent (teal `#0e9aa8`) for selection, primary actions, caret, links.
- States are alpha-based fills, not opaque variants, so they sit on any surface.
- Elevation is soft, layered, low-alpha; cards lift by shadow, not heavy borders.
- Motion is restrained and functional (120–300ms); no decorative animation.
- Every interactive element has hover / pressed / focus / disabled states.

## Color tokens (`src/theme.rs`)
Neutral ramp: `BG #f7f9fa`, `PANEL #ffffff`, `SURFACE #f1f4f6`, `SURFACE_2 #e4e9ee`,
`BORDER #dde3e8`, `BORDER_STRONG #b9c4cd`, `TEXT #1a2430`, `MUTED #5a6b7c`,
`FAINT #66788a` (≥4.5:1 on white for 11–12px secondary text).
Accent `ACCENT/SELECT #0e9aa8`; semantic `OK/WARN/DANGER/INFO`.
Alpha interaction fills: `FILL_HOVER` black 4.5%, `FILL_HOVER_STRONG` 6%,
`FILL_PRESSED` 9%, `ROW_SELECTED` 7.2%, `HAIRLINE` 8%.
Selection wash `SELECT_FILL` (teal 16%); search hits `SEARCH_DNA/PROT`.
Base ladder A/T/C/G + pastel() for feature fills on light.

## Typography
UI: `.SystemUIFont`; mono `Consolas`. Type scale (px): `T_XS 11`, `T_SM 12`,
`T_BASE 13`, `T_MD 14`, `T_LG 16`, `T_XL 20`. Hierarchy via size + weight +
whitespace; section titles `T_MD` semibold; page titles `T_LG` semibold.

## Layout & spacing
4px grid; radii nest 8 (outer) / 6 (inner) / 4. Page padding 20/14; body gap 8;
row card padding 10/8, gap 8. Nav sidebar 180px, rows 30px with a 3px accent bar
on the active item. Tables: 26–27px rows, hairline separators, striped + selected
fills, virtualized (`uniform_list`).

## Iconography
Monochrome stroke icons (Lucide-style), `currentColor`, tinted by theme;
14–18px in chrome. No emoji in chrome. `icons::view_icon` maps views to glyphs.

## Elevation (layered `BoxShadow`, quiet-md family)
`shadow_menu` (1/3 + 8/24) for menus/popovers; `shadow_panel` (1/4 + 12/32) for
find bar/toasts; `shadow_dialog` (2/8 + 24/64) for modals; `shadow_card`
(1/2 + 4/14) for resting cards. Black alpha; increase weight in dark mode.

## States
Hover `FILL_HOVER`, pressed `FILL_PRESSED` (`.active`), selected `ROW_SELECTED`
or `SELECT_FILL`, disabled opacity 0.4–0.6, focus ring on inputs (teal).
Empty states use `widgets::empty_state` (icon + title + helper).

## Motion
Functional only: hover/pressed transitions, dialog/popover appear, drag feedback.
Target 120–300ms with ease-out; no looping/decorative animation.

## Roadmap (design)
- **CSS/token-file theming:** user-extensible appearance via theme files.
- **Icon optical weight:** verified — 2-stroke @24-grid rendered at 14px ≈ 1.17px,
  matching the reference's 1.5-stroke @18-grid at 14px; no change needed.
- Dark mode: **not planned** (product decision — light look only).