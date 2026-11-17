# HelixBench UX v6 — UI Improvement Changelog

Nov 17, 2026 · @shaik moosa

## Summary

All 88 HelixBench screens now have icons, a clearer visual hierarchy, raised cards, WCAG AA text contrast and viewer controls (zoom, fit, rotate, layers, colour, sliders). The work is on a new Figma page, **Soft Gallery · UX v6**, a full copy of UX v5. The v5 page and the shared variables were not changed, so the two versions can be compared side by side. [Open the Figma file](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=174-5102)

| Change | Count in v6 |
| --- | --- |
| Screens updated | 88 |
| Navigation items with icons | 756 |
| Action buttons with leading icons | 74 primary/secondary + 18 quiet |
| Text layers fixed for contrast | 67 (all 30 WCAG failures resolved) |
| Cards given elevation + border | 121 |
| Heading/description pairs regrouped | 37 |
| Table state cells converted to status badges | 20 |
| Scientific canvases given viewer toolbars | 26 |
| New components | 34 icons, Viewer toolbar, Slider (6 positions), Colour scheme picker |

## UX principles applied

Every change maps to a widely used UX principle or standard. Each one was applied to all screens, not just a few.

| Principle | Source | How it shows up in v6 |
| --- | --- | --- |
| Recognition rather than recall | Nielsen heuristic #6 | Icons beside every nav label and key action, so people spot destinations instead of reading every label |
| Visibility of system status | Nielsen heuristic #1 | State columns are now coloured badges with a dot; zoom level is always shown in the viewer toolbar |
| Consistency and standards | Nielsen heuristic #4 | One toolbar, one slider and one badge component are reused on every screen |
| User control and freedom | Nielsen heuristic #3 | "Reset view" in View options; zoom, fit and rotate are always one click away |
| Proximity | Gestalt | Section headings sit 4px from their description (was 24px), so each pair reads as one unit |
| Common region | Gestalt | Cards get a 1px border, 12px radius and soft shadow to show clear groups |
| Visual hierarchy | Typographic scale | Page title 26 → 28px; sidebar group labels 11px uppercase; current breadcrumb item in medium weight |
| Contrast (minimum) | WCAG 2.2 SC 1.4.3 | All text ≥ 4.5:1 (≥ 3:1 for large text) |
| Non-text contrast | WCAG 2.2 SC 1.4.11 | Chart bars and occupied plate wells raised to ≥ 3:1 against white |
| Use of colour | WCAG 2.2 SC 1.4.1 | Status uses colour + dot + text label; colour scheme picker names each palette |
| Target size | Fitts's law, WCAG 2.2 SC 2.5.8 | Toolbar buttons are 32×32px; nav rows 40px tall |
| 8-point spacing | Common spacing system | New parts use 4/8/12/16/24px spacing only |

## Component changes

All components live in **00 Start and interaction system › Reusable interaction components** on the v6 page. Every screen instance picked up these changes automatically.

| Component | Before (v5) | After (v6) |
| --- | --- | --- |
| V6 / Icons (new) | Only 4 icons (search, clock, settings, chevron) | 34 outline icons, 24px grid, 2px stroke: home, sequences, results, experiments, registry, inventory, workflows, insights, automation, zoom in/out, rotate left/right, fit, layers, palette, sliders, move, flip, download, upload, save, play, plus, check, alert, error, info, chevron, arrow, file, search, history, grid |
| V5 / Navigation item | Text only; selected state was a green fill with 4.2:1 label contrast | 18px icon (INSTANCE\_SWAP "Icon" property), 10px gap, 3px left accent bar on selected, label #2A6A60 (5.4:1) |
| V5 / Action button | Label only; disabled looked identical to default | Optional leading icon ("Show icon" + "Icon" properties), auto width, darker hover, disabled at 45% opacity, danger red #B42318 (6.5:1 with white) |
| V5 / Status badge | Plain 14px text; Warning and Error had no tint | Pill, 12px medium text, status dot, tinted fill per state: Neutral, Success, Warning, Error |
| V5 / View tab | Selected label #357D73 | Selected label #2A6A60, rounded indicator; rest label #4A4F52 |
| V6 / Viewer toolbar (new) | — | Floating pill: zoom out, zoom %, zoom in, fit, rotate left/right, layers, colour scheme. Properties: Zoom (text), Show rotate, Show display |
| V6 / Slider (new) | — | Label + live value + track + thumb; Position variants 0, 12, 25, 50, 75, 100 |
| V6 / Colour scheme picker (new) | — | "Colour by" segmented control (Feature type, Strand, GC %) + palette swatches labelled "Colorblind-safe palette" |

## Changes on every screen

- **Sidebar hierarchy:** group labels (Workspace, Lab records, Lab operations) are now 11px semibold uppercase with 6% tracking. There is 16px above each group and 6px below. Rest labels are #2E3235.
- **Breadcrumbs:** "/" is now "›", and the current page is medium weight in ink colour, so people can see where they are.
- **Page titles:** 26px → 28px with −1.5% tracking; title-to-subtitle gap 4 → 6px.
- **Section headers:** 37 heading + description pairs that were 24px apart now share a 4px "Section header" group.
- **Cards:** 118 content cards get a 1px #E6E4DF border, 12px minimum radius and a two-layer soft shadow. 3 popovers get a deeper shadow so they sit above the page.
- **Tables:** State and Commit state cells use the status badge instead of plain grey text.
- **Icons on actions:** Import → upload, Run/Rerun/Design → play, Save/Retain → save, Export → download, Create/Add/New → plus, Compare → layers, Submit → check, View options → sliders, Find → search, Annotation list → file.

### Contrast fixes (WCAG 2.2 AA)

An automated check of all 4,542 visible text layers found 30 failures. All 30 are fixed. Shared variables were not edited, so v5 is unchanged.

| Use | Old colour → new | Contrast before → after | Layers |
| --- | --- | --- | --- |
| Accent text and links on canvas (#F7F6F3) | #357D73 → #2A6A60 | 4.49 → 6.1:1 | 13 failing (all accent text changed for consistency) |
| Accent text on selected mint (#DCF4E9) | #357D73 → #2A6A60 | 4.19 → 5.4:1 | 1 + nav and tab components |
| "Unsaved document" on amber (#FDF0D5) | #C08A20 → #7A4F00 | 2.70 → 7.0:1 | 11 |
| Error text on white | #D64545 → #B42318 | 4.38 → 6.5:1 | 4 |
| White on danger button | #D64545 → #B42318 fill | 4.38 → 6.5:1 | 1 + component |
| Insights chart bars (non-text) | #DCF4E9 → #357D73 | 1.2 → 5.0:1 | 3 |
| Occupied plate wells (non-text) | #DCF4E9 → #B9E0CF + #4F9E85 border | 1.2 → 3.2:1 border | 64 |

## Page-by-page changes

The global changes above apply to all 88 screens. This table lists the extra changes made to specific screens.

| Screen(s) | Change | Principle |
| --- | --- | --- |
| Start here · overview | Renamed to UX v6; "What is different" now lists the v6 changes; screen count corrected to 88 | Visibility of status |
| home, home-first | Recent sequences get a 40px DNA icon tile; State column shows Saved locally (success) and Draft (warning) badges; Import button has an upload icon | Recognition, status |
| map, map-saved, feature-edit, dirty-decision, map-1280, map-1280-closed | Viewer toolbar at the bottom-right of the plasmid map: zoom −/100%/+, fit, rotate left/right, layers, colour scheme | User control, Fitts's law |
| map-options | View options popover split into "Show on map", "Adjust view" (Rotation 45° and Label density sliders) and "Colour by" (segmented control + colour-blind-safe swatches); footer with Reset view and Done; Layers button shown as active in the toolbar | User control and freedom, use of colour |
| sequence | Toolbar (zoom, fit, layers, colour) at the top-right of the sequence card; Find sequence has a search icon | Consistency |
| chemistry | Full toolbar including rotate, to turn the compound structure | User control |
| biologics, alignment | Toolbar with zoom, fit, layers, colour | Consistency |
| digest-result, saved-result, export, source-unavailable, save-failed (gel) | Compact zoom/fit toolbar on the gel preview | Consistency |
| pcr-target, pcr-primers, primer-design, pcr-products, pcr-create, pcr-no-product, pcr-saved-result | Compact zoom/fit toolbar on the primer binding preview | Consistency |
| assembly | Compact zoom/fit toolbar on the junction schematic | Consistency |
| inventory, transfer-complete | Toolbar with colour scheme on the 96-well plate; occupied wells now meet 3:1 non-text contrast | Non-text contrast |
| insights | Bars bottom-aligned (they were top-aligned, which made the comparison misleading), solid accent fill, rounded tops, bold values, baseline rule | Data integrity, non-text contrast |
| filtered-empty | Empty state centred with a search icon, 22px heading, and a primary "Clear filters" plus secondary "Edit filters" | Help users recover from errors |
| delete-review | Danger button now #B42318 | Contrast |
| missing-prerequisite, chemistry-error | Error text now #B42318 | Contrast |

## Known limitations and next steps

- **Prototype links:** the v6 page was duplicated from v5. Check in Present mode that its prototype links go to v6 frames, not v5.
- **Variables:** colour fixes were applied directly to layers so v5 stays unchanged. If v6 is adopted, add the new values (#2A6A60, #7A4F00, #B42318) to the "HelixBench UX v5" variables and rebind them.
- **Slider values:** the thumb position is set with the Position variant (0–100% in steps). Positions in between need a new variant.
- **Icon colour on selected nav:** the accent colour comes from the default icon. A different icon swapped in shows the neutral colour until it is recoloured.
- **Not yet tested with users:** test the toolbar and View options with 3–5 researchers before handoff, especially rotate and label density on dense maps.
- **Dark mode:** not covered. The "HB Tokens / Night" collection exists but was not mapped to v6.
