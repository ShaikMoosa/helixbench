# HelixBench — Soft Gallery design contract

UX v4 is the visual authority. Preserve live scientific data, base/feature colors,
coordinates, topology and established algorithm behavior when applying this shell.
Reference Figma file: U1kWw1K5NZBgEYkuxMz3Qb; Home 107:29237, Map 107:29429,
context inspector 107:29634, empty Home 107:37024.

## Foundations

Use central roles in `apps/desktop_gpui/src/theme.rs`: warm canvas #F7F6F3,
white panels, ink #17191A, muted #666B6E, borders #E8E9E7, mint #DCF4E9,
teal #357D73. Secondary text #6D7274 and selected navigation text #2E7168
are darker than the design exports for readable contrast. Semantic feedback
colors remain distinct; scientific A/T/C/G colors retain their existing meanings.

DM Sans is embedded with its OFL license. Registration failure surfaces an error
and uses Segoe UI. Scientific text uses installed Cascadia Mono, falling back to
Consolas. UI roles are 13/19 body, 15/21 description, 19/27 section, 24/34 task
card, 27/38 page heading. Use medium weight and whitespace for hierarchy.

## Layout

Reference viewport is 1600×1080: 200px white navigation, warm center with 32px
padding, 320px white inspector with 24px padding. The inspector docks by default
at widths >=1480px. Below that it starts hidden and opens as a right drawer;
manual Close remains closed across resizing. Header actions reserve 360px.
Sequence view tabs and document tabs are independently accessible. All modules
retains specialist destinations; the native application menu retains existing tools.

Buttons are 36px high, radius18, horizontal padding16; primary actions use ink
with white text. Cards use radius16; rows radius14. Resting surfaces use subtle
borders/spacing. Reserve elevation for transient menus, dialogs and drawers.
Scrollable inspector content must retain natural control heights and wrap within
272px; do not shrink buttons to fit the viewport.

## Interaction

Page tools opens the contextual command catalog in one click. A second click
opens the chosen tool; unavailable tools explain their prerequisites. Search is
workspace metadata/tool search; Ctrl+F remains sequence Find. Ctrl+Shift+I toggles
the inspector. Close restores focus to the actual opener (Page tools or header).

Context is derived from stable document/revision, feature, primer, record or result
IDs. Revalidate before acting. Search must not expose hidden DNA actions. Primer
creation requires a current nonempty valid range, never the whole construct as a
fallback. View representation changes do not alter DNA topology or dirty state.
Related records are actual linked records, never authored reference fixtures.

## Assets and scope

Use embedded approved Figma SVGs: full-color logos via image rendering; monochrome
tool icons at their native16/18/15px sizes. Provenance and checksums are in
`apps/desktop_gpui/assets/soft-gallery/README.md`. No network asset is required at
runtime. The latest empty Home has an import card without a cave-art slot.

P07 migrates shared chrome, Home task card, tokens and contextual tools. Detailed
scientific screens remain P08; other module redesigns remain P11–P17. Native menus,
live canvas, existing scroll/zoom controls and algorithm entry points remain.
Application-wide accessibility, responsive hardening and large-data performance
remain P19. Motion should be brief and functional; no decorative loops.
