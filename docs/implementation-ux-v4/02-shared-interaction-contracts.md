# Shared interaction contracts

## Persistent shell

Maintain labeled navigation, document identity, draft/saved state, document tabs, current scientific view, task content and the contextual right panel. Use the existing GPUI hierarchy; do not render product functions as disconnected pages because a Figma example was an isolated frame.

## Command discovery

Create a single command registry with ID, visible label, owning page, category, input requirements, availability reason, shortcut and typed action. The owning page exposes its common tools directly and the complete catalog in one click; choosing the tool is the second click. Avoid an additional Advanced submenu that adds a third discovery click.

## Input/selection context

Capture document/revision, selected range/strand, active split pane, selected feature/primer IDs, typed entity reference and plate/well set in the command context. A tool opened from another module explicitly asks which input to use. If that input disappears, show a prerequisite/recovery state; never substitute another record.

## Navigation and dirty guards

Keep view switches and read-only tools inside the same draft without a guard. Guard only an intent that would discard/replace data or cannot preserve the draft: document/app close, destructive replacement and relevant file/document transitions. Save performs the exact pending intent only after successful persistence. Discard restores the saved source before continuing. Stay/Escape returns to the invoking view with input retained.

## Back and return

Store a typed ReturnLocation with route, source IDs, scope/query, filters/sort, selection and scroll anchor. Back is not a hardcoded Home button. An explicit Back to search returns to the same search scope/query; a source link opens the historical revision when that is what the result references.

## Forms

Separate draft values from committed objects. Define required fields, valid formats/ranges, inline errors and submit availability. Repeated submits are idempotent at the application operation boundary. Preserve user input on validation/write failure. Cancel has no mutation. When an input changes, clear or mark stale any computed preview.

## Feedback

Use inline field errors for correctable input, inline page status/toast for routine success and modal decisions for consequential choices. Every message names the affected object, outcome and constructive next action. Do not implement every operation-outcome frame as a blocking confirmation. Status is announced without moving focus unnecessarily.

## Empty-state taxonomy

Distinguish no records yet, no matching query, no rows after filters, no scientific matches, processing, failed load, inaccessible source and restricted/locked action. Give a relevant creation/reset/retry/locate remedy. Cave-art illustration remains decorative and must not carry the only explanation or obscure the primary action.

## Tables and grids

Rows and cells act on stable identities; sort/filter never retargets a mutation. A colored row is a real selected/hover state, not a misleading decorative default. After deletion, clear the inspector and row selection unless an explicit next-record selection is made. Virtualize large tables and provide keyboard activation.

## Units and coordinates

Show quantity dimension and unit with every numeric value. Validate finite numbers before calculation. UI positions are 1-based inclusive; internal ranges are zero-based start plus length. Circular origin wrapping is an explicit representation, not an invalid negative-length interval.

## Modal versus panel

Specify each surface as non-modal inspector, modal dialog, popover, toast or full task view. A modal has initial focus, contained tab order, inert background, Escape/cancel and logical focus return. A non-modal panel permits keyboard movement back to the workspace and has an explicit Close control.

## Responsive behavior

Use fluid GPUI layout. At 1280px, preserve usable main content and open the right panel on demand; do not scale all text and controls down. At wider widths, panel pinning is available. Wrap header actions, use accessible horizontal table scrolling and keep focused controls visible at increased Windows DPI/text size. The captured 1280 cohort is evidence for named first-use paths, not every possible layout.

## Keyboard contract

Preserve Ctrl+N/O/S, Ctrl+Shift+S, Ctrl+Z/Y, Ctrl+F, Escape, Ctrl+A/C/X/V, Delete/Backspace and Ctrl+1–6 as inspected. Text inputs consume editing keys before global sequence shortcuts. Add global workspace Search as a separate command (proposed Ctrl+K); Ctrl+F continues to search the active sequence. Focus ring and error examples appear in the interaction-contract Figma frame.

## Sample mode

Sample records have an explicit origin and namespace throughout setup/results. Exiting keeps them separate from personal records. Saving a personal copy allocates a new document ID/revision and retains source attribution; sample result IDs are never relabeled as personal analysis outputs.

## Non-negotiable invariants

### INV-01 · Typed identity

Every command carries stable source/target IDs and relevant revision IDs. Never bind a mutation to a display name, row index or currently active record at completion time.

### INV-02 · Coordinate contract

UI coordinates are 1-based inclusive. Domain Range is zero-based start plus length, with documented circular wrap. Conversions occur at a typed boundary and are tested once.

### INV-03 · Preserve input

Keep original import/paste text and produce a transformation report before meaningful changes. Invalid input cannot silently disappear.

### INV-04 · No silent substitution

A protein tool cannot choose a chain because DNA is active; a well cannot become A1 because H12 was clicked; no selection yields a prerequisite state.

### INV-05 · Draft safety

View switches retain the same draft. Destructive replacement, document close, file open and app close guard dirty data. Save failure/cancel never performs the pending intent.

### INV-06 · Saved truth

The UI reports saved/submitted/deleted only after the required persistence boundary succeeds. Partial/uncertain outcomes are not called success.

### INV-07 · Historical truth

Result, submitted-entry and plate-snapshot provenance is immutable. New source edits do not rewrite old outputs.

### INV-08 · Selection truth

Deleting a selection clears its inspector and actions. Filters/sorts retain the selected ID or explicitly clear it; they never select an unrelated row silently.

### INV-09 · Async truth

Each job has an operation ID and captured inputs. Closing a view does not retarget its completion; stale results remain tied to their captured source.

### INV-10 · Local truth

The local application does not claim remote delivery, authenticated review or cloud synchronization without an implemented service.

### INV-11 · Recoverable mutation

Multi-record and cross-store actions validate a complete plan, commit predictably, and reconcile on restart without repeating successful writes.

### INV-12 · Accessible operation

Visible labels, focus, keyboard navigation, readable scientific alternatives and announced status are part of the interaction, not optional decoration.

