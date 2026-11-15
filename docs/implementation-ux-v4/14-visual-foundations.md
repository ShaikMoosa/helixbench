# Soft Gallery visual implementation contract

Measured from the current UX v4 Home frame, not the earlier dark or teal theme. These are native layout units at 100% scale; accommodate OS scaling and text metrics. Full raw evidence, including earlier collections retained in the file, is in reference/figma-foundations.json. Use the Soft Gallery collection (VariableID:38:*); do not select a same-named token from an old collection.

## Palette

- `color/ink`: `#17191A`; Figma `VariableID:38:887`.
- `color/muted`: `#666B6E`; Figma `VariableID:38:888`.
- `color/faint`: `#797E80`; Figma `VariableID:38:889`.
- `color/line`: `#E8E9E7`; Figma `VariableID:38:890`.
- `color/white`: `#FFFFFF`; Figma `VariableID:38:891`.
- `color/warm`: `#F7F6F3`; Figma `VariableID:38:892`.
- `color/cool`: `#F1F5F5`; Figma `VariableID:38:893`.
- `color/mint`: `#DCF4E9`; Figma `VariableID:38:894`.
- `color/teal`: `#357D73`; Figma `VariableID:38:895`.
- `color/dark`: `#252728`; Figma `VariableID:38:896`.
- `color/dark2`: `#323536`; Figma `VariableID:38:897`.
- `color/darktext`: `#F3F4F3`; Figma `VariableID:38:898`.
- `color/darkmuted`: `#A9ADAD`; Figma `VariableID:38:899`.
- `color/lime`: `#D8EF74`; Figma `VariableID:38:900`.
- `color/pink`: `#EDCADF`; Figma `VariableID:38:901`.
- `color/blue`: `#C9E8EF`; Figma `VariableID:38:902`.
- `color/lavender`: `#DCD6EE`; Figma `VariableID:38:903`.

## Typography

Use DM Sans for interface text and Cascadia Mono for sequence/code content where used in the established design. Verify font distribution rights and availability before bundling; provide deterministic fallback metrics and validate Windows scaling. Current Home uses DM Sans Regular and Medium: 12/17, 13/19, 14/20, 15/21, 19/27, 24/34 and 27/38 (size/line-height). Heading and body roles must be named in the native theme rather than copied as scattered constants. Some labels have automatic line height; use measured frame layouts to check wrapping. Readability and accessible scaling take priority over truncating essential scientific content.

## Measured shell

- Reference frame: 1600 × 1080. Navigation: 200 wide, 16 padding, 7 item gap; rows 168 × 36 with 12 radius and 8 horizontal inset.
- Center workspace: 1080 wide with 32 padding and 16 section gap. Header: 1016 × 62 with 12 gap. Task workspace: 1016 × 900 with 16 gap.
- Contextual right panel: 320 wide with 24 padding and 14 section gap. Keep selection identity, relevant tools and prerequisites visible.
- These measurements describe the reference size. At 1280 wide, follow the documented responsive states: preserve the scientific viewport and make the inspector collapsible/reopenable; do not squeeze or remove action labels without an accessible alternative.
- Use layout containers and intrinsic text sizing. Scientific canvases may pan/zoom independently; the full application must not require horizontal scrolling to find Save or Run.

## Components and states

- Buttons: primary, secondary, quiet and destructive roles; default, hover, pressed, focus, disabled-with-reason and busy states. Primary actions use the established ink/mint/teal hierarchy. Never use a pastel color alone to communicate status.
- Inputs: persistent label, unit or format hint, empty/value, focused, invalid, read-only and disabled states. Validation associates an exact message with the field and preserves the user value.
- Tables: headers, sort, filter, no data, no matches, loading, failure, selection, keyboard focus and unavailable-record states. Selection and focus must remain visually distinguishable.
- Panels: close/reopen affordance, selected-record title, relevant properties/tools, scrollable body and safe action footer. Preserve context when switching tabs or resizing.
- Dialogs: concise title, affected object, consequences, retained draft values, explicit primary/cancel actions, focus containment and return focus. Consequential operations use the functional state machines; routine success does not block work.
- Scientific annotations retain their semantic feature/strand/base distinctions and a legend. Decorative palette changes must not merge distinct scientific categories.
- Semantic success/warning/error colors must be contrast-checked against their actual surface. Do not reuse an old theme token simply because its name matches. Validate text, focus and non-text control contrast before release.

## Cave-art illustrations and product feedback

Retain the requested cave-painting art direction for empty, success, warning, alert and deletion illustrations: restrained charcoal/ochre figures, hand-rendered marks and light warm texture. Keep artwork secondary to the state title, explanation and next action; avoid texture behind body text or scientific data. Illustrations are decorative unless they carry information absent from the text. Loading, zero matches, failed operation, partial outcome, permissions/read-only context and true empty data need distinct copy/actions even if they share an illustration family. Use existing approved Figma artwork assets; do not substitute emoji or introduce new mascot content during implementation. Destructive confirmation names the exact target and consequence; completed deletion is a separate outcome. Respect reduced motion; essential feedback remains available without animation.

## Native implementation ownership

P07 introduces reusable theme/component roles in desktop_gpui; P19 verifies keyboard, scaling, contrast, responsive behavior and large-data performance. Preserve the user-supplied HelixBench logo from Figma node 18:3 and its current v4 usage; export the approved asset and verify rendering rather than redrawing it. Document any accessibility-driven correction to a measured token with before/after evidence.
