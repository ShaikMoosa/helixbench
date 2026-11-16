//! Shell interaction state. View presentation never mutates scientific topology.
use crate::{
    state::{AppState, MapMode, View},
    workspace_search::{self, SearchTarget},
};
use domain::{DocumentId, Range, RecordRef, ResultId, SourceRef};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PanelMode {
    #[default]
    Auto,
    Open,
    Closed,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InspectorPlacement {
    Hidden,
    Docked,
    Drawer,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FocusReturn {
    #[default]
    InspectorToggle,
    PageTools,
    Editor,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PanelTab {
    #[default]
    Details,
    Tools,
    Help,
}
#[derive(Default)]
pub struct ShellState {
    pub mode: PanelMode,
    pub tab: PanelTab,
    pub all_modules: bool,
    pub sequence_preview: bool,
    opener: FocusReturn,
}
impl ShellState {
    pub fn open(&mut self, opener: FocusReturn) {
        self.mode = PanelMode::Open;
        self.opener = opener;
    }
    pub fn close(&mut self) -> FocusReturn {
        self.mode = PanelMode::Closed;
        self.opener
    }
    pub fn layout(&self, width: f32) -> ShellLayout {
        ShellLayout::new(width, self.mode)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct ShellLayout {
    pub navigation_width: f32,
    pub workspace_width: f32,
    pub content_width: f32,
    pub inspector_width: f32,
    pub inspector: InspectorPlacement,
    pub header_actions_width: f32,
    pub title_min_width: f32,
    pub gap: f32,
    pub page_padding: f32,
}
impl ShellLayout {
    pub fn new(width: f32, mode: PanelMode) -> Self {
        let width = if width.is_finite() {
            width.max(640.0)
        } else {
            1280.0
        };
        let inspector = match (mode, width >= 1480.0) {
            (PanelMode::Closed, _) | (PanelMode::Auto, false) => InspectorPlacement::Hidden,
            (_, true) => InspectorPlacement::Docked,
            _ => InspectorPlacement::Drawer,
        };
        let navigation_width = 200.0;
        let inspector_width = 320.0;
        let workspace_width = width
            - navigation_width
            - if inspector == InspectorPlacement::Docked {
                inspector_width
            } else {
                0.0
            };
        let page_padding = if width >= 1280.0 { 32.0 } else { 20.0 };
        Self {
            navigation_width,
            workspace_width,
            content_width: (workspace_width - page_padding * 2.0).max(0.0),
            inspector_width,
            inspector,
            header_actions_width: 360.0,
            title_min_width: 220.0,
            gap: 12.0,
            page_padding,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PanelContext {
    Page(View),
    WorkspaceSearch,
    Document(SourceRef),
    Feature { source: SourceRef, feature: u64 },
    Primer { source: SourceRef, primer: u64 },
    Range { source: SourceRef, range: Range },
    Record(RecordRef),
    Result(ResultId),
}
pub fn context(app: &AppState) -> PanelContext {
    if app.workspace_search.open {
        return PanelContext::WorkspaceSearch;
    }
    if app.view == View::Home {
        if let Some(id) = app.first_use.selected_result {
            return PanelContext::Result(id);
        }
    }
    if let Some(target) = app.workspace_search.inspected {
        if let SearchTarget::Record(record) = target {
            if workspace_search::lookup(app, target).is_some_and(|hit| hit.view == app.view) {
                return PanelContext::Record(record);
            }
        }
    }
    if !View::ALL.contains(&app.view) {
        return PanelContext::Page(app.view);
    }
    let source = app.doc.source_ref();
    if let Some(id) = app.selected_feature.filter(|id| {
        matches!(app.view, View::Map | View::Sequence | View::Features)
            && app.doc.feature(*id).is_some()
    }) {
        return PanelContext::Feature {
            source,
            feature: id,
        };
    }
    if let Some(id) = app.selected_primer.filter(|id| {
        matches!(app.view, View::Map | View::Sequence | View::Primers)
            && app.doc.primer(*id).is_some()
    }) {
        return PanelContext::Primer { source, primer: id };
    }
    if let Some(range) = app.selection.filter(|r| valid_range(app, *r)) {
        return PanelContext::Range { source, range };
    }
    PanelContext::Document(source)
}
pub fn validate_context(app: &AppState, expected: &PanelContext) -> Result<(), String> {
    if context(app) != *expected {
        return Err("The selected item changed. Review the current details before acting".into());
    }
    if app.dialog.is_some() || app.navigation.pending.is_some() {
        return Err("Apply or cancel the open form or decision first".into());
    }
    Ok(())
}
pub fn set_representation(app: &mut AppState, mode: MapMode) {
    app.map_mode = mode;
}
pub fn context_document(context: PanelContext) -> Option<DocumentId> {
    match context {
        PanelContext::Document(s)
        | PanelContext::Feature { source: s, .. }
        | PanelContext::Primer { source: s, .. }
        | PanelContext::Range { source: s, .. } => Some(s.document),
        _ => None,
    }
}

pub fn valid_range(app: &AppState, range: Range) -> bool {
    let len = app.doc.sequence.len();
    range.len > 0
        && range.start < len
        && range.len <= len
        && (app.doc.sequence.is_circular()
            || range
                .start
                .checked_add(range.len)
                .is_some_and(|end| end <= len))
}

/// Primer creation always requires an explicit, current, nonempty range.
pub fn primer_range(app: &AppState, expected: &PanelContext) -> Result<Range, String> {
    validate_context(app, expected)?;
    if !matches!(
        expected,
        PanelContext::Range { .. } | PanelContext::Feature { .. }
    ) {
        return Err("Select a sequence range before creating a primer".into());
    }
    app.selection
        .filter(|range| valid_range(app, *range))
        .ok_or_else(|| "Select a sequence range before creating a primer".into())
}
