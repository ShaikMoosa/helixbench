# HelixBench — Soft Gallery design contract

UX v6 is the target visual authority, adopted by the user on 17 November 2026.
Figma page `199:1316`, Map `199:2197`, supersedes v4 presentation in migrated slices.
See `docs/implementation-ux-v6/README.md` for delivery order and boundaries.
Unmigrated layouts remain v4 until their own route/viewer milestone; do not claim
the shared foundation update completes the entire redesign.

Preserve live scientific data, base/feature colors,
coordinates, topology and established algorithm behavior when applying this shell.
Reference Figma file: U1kWw1K5NZBgEYkuxMz3Qb; Home 107:29237, Map 107:29429,
context inspector 107:29634, empty Home 107:37024.

## Foundations

Use central roles in `apps/desktop_gpui/src/theme.rs`: warm canvas #F7F6F3,
white panels, ink #17191A, muted #666B6E, borders #E6E4DF, mint #DCF4E9,
display teal #357D73 and readable accent text #2A6A60. Navigation rests at #2E3235;
warning text is #7A4F00 and error/danger text #B42318. Secondary text remains #6D7274.
Keep display accents separate from text roles. Semantic feedback
colors remain distinct; scientific A/T/C/G colors retain their existing meanings.

DM Sans is embedded with its OFL license. Registration failure surfaces an error
and uses Segoe UI. Scientific text uses installed Cascadia Mono, falling back to
Consolas. UI roles are 13/19 body, 15/21 description, 19/27 section, 24/34 task
card, 28/40 page heading. Group section headings and descriptions with a 4px gap;
page title/subtitle gap is 6px. Use medium weight and whitespace for hierarchy.

## Layout

Reference viewport is 1600×1080: 224px white navigation, warm center with 32px
padding, 320px white inspector with 24px padding. The inspector docks by default
at widths >=1480px. Below that it starts hidden and opens as a right drawer;
manual Close remains closed across resizing. Header actions reserve 360px.
Sequence view tabs and document tabs are independently accessible. Workspace navigation
groups Home, Sequences, Results and Experiments; Specialist tools expands the remaining
specialist destinations. Split is a viewer layout action. The native application menu
retains existing tools. Catalogs hide the object inspector; their own search and row actions
remain visible. Experiments creates an experiment entry, never a DNA document.

Buttons are 40px high, radius8, horizontal padding16; primary actions use ink
with white text. Cards/rows use radius12, a #E6E4DF border and subtle two-layer
elevation. Popovers, dialogs and drawers use stronger elevation. Navigation rows
are 40px high, with an 8px radius and a persistent 3px selected accent.
Scrollable inspector content must retain natural control heights and wrap within
272px; do not shrink buttons to fit the viewport.

## Interaction

Scientific canvases use the v6 floating viewer toolbar: 32px focusable targets, 18px
icon instances, a 10px white toolbar and window-anchored panels. Panels can extend
above a short canvas, keep their heading and Reset/Close actions visible, and scroll
their content. Both floating surfaces occlude the canvas to prevent click-through.
Zoom/rotation are continuous; rotation is offered only for circular maps. Sequence
zoom uses a real genomic window and clamps at linear ends. Map defaults to the larger
canvas; View options retains sequence preview and metadata. Existing scientific
engines, coordinates, annotation colors and captured inputs remain authoritative.

Display state is keyed by document ID and surface, with per-property layer overrides
over existing defaults. Menu/inspector controls read the same effective settings.
Reset does not edit DNA, selection, topology or another view's scroll position.
Tab reaches controls; native keyboard click activation runs once. +/- and 0 work with
viewer focus, and R rotates only circular Map so Sequence typing retains IUPAC R.

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
The migrated v6 navigation, logo and action SVGs are documented with checksums in
`apps/desktop_gpui/assets/ux-v6/README.md`. Results is a first-class catalog; unsaved
completed runs remain resumable in a separate session section, with an explicit reminder
that they do not survive app restarts. Saved rows use captured source revision, run ID and
UTC time to distinguish repeated runs.

P07 migrates shared chrome, Home task card, tokens and contextual tools. Detailed
scientific screens remain P08; other module redesigns remain P11–P17. Native menus,
live canvas, existing scroll/zoom controls and algorithm entry points remain.
Application-wide accessibility, responsive hardening and large-data performance
remain P19. Motion should be brief and functional; no decorative loops.
