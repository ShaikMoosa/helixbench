//! Typed destinations, per-document editing sessions and loss-aware navigation.
use crate::{
    actions,
    state::{
        AppState, ColorsMode, Dialog, HistoryEntry, MapMode, OpenDoc, PendingAction, SearchState,
        TableSort, View,
    },
};
use domain::{Document, DocumentId, Range, RecordRef, SourceRef, Strand};
use std::{collections::HashMap, path::PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Route {
    pub view: View,
    pub document: DocumentId,
    pub record: Option<RecordRef>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PaneId {
    #[default]
    Primary,
    Secondary,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WellRef {
    pub plate_id: u64,
    pub row: u8,
    pub column: u8,
}

#[derive(Debug, Clone)]
pub struct SelectionContext {
    pub source: SourceRef,
    pub range: Option<Range>,
    pub strand: Strand,
    pub active_pane: PaneId,
    pub feature: Option<u64>,
    pub primer: Option<u64>,
    pub records: Vec<RecordRef>,
    pub wells: Vec<WellRef>,
}

#[derive(Clone)]
pub struct ViewState {
    pub zoom_start: usize,
    pub zoom_width: usize,
    pub scroll_to: Option<usize>,
    pub feature_filter: String,
    pub primer_filter: String,
    pub feature_sort: TableSort,
    pub primer_sort: TableSort,
    pub enzyme_sort: TableSort,
}

#[derive(Clone)]
pub struct ReturnLocation {
    pub route: Route,
    pub selection: SelectionContext,
    pub query: SearchState,
    pub view_state: ViewState,
}
impl ReturnLocation {
    pub fn capture(app: &AppState) -> Self {
        Self {
            route: Route {
                view: app.view,
                document: app.doc.id,
                record: None,
            },
            selection: SelectionContext {
                source: app.doc.source_ref(),
                range: app.selection,
                strand: app.selection_strand,
                active_pane: PaneId::Primary,
                feature: app.selected_feature,
                primer: app.selected_primer,
                records: vec![],
                wells: vec![],
            },
            query: app.search.clone(),
            view_state: ViewState {
                zoom_start: app.zoom_start,
                zoom_width: app.zoom_width,
                scroll_to: app.scroll_to,
                feature_filter: app.feature_filter.clone(),
                primer_filter: app.primer_filter.clone(),
                feature_sort: app.feature_sort,
                primer_sort: app.primer_sort,
                enzyme_sort: app.enzyme_sort,
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NavigationIntent {
    Navigate(Route),
    SwitchDocument(DocumentId),
    OpenFile(PathBuf),
    OpenKnownFile { path: PathBuf, document: DocumentId },
    OpenFilePicker,
    CloseDocument(DocumentId),
    ExitApplication,
    NewEmpty,
    NewDemo,
}
impl From<PendingAction> for NavigationIntent {
    fn from(action: PendingAction) -> Self {
        match action {
            PendingAction::CloseApp => Self::ExitApplication,
            PendingAction::NewDemo => Self::NewDemo,
            PendingAction::NewEmpty => Self::NewEmpty,
            PendingAction::Open => Self::OpenFilePicker,
            PendingAction::OpenRecent(path) => Self::OpenFile(path),
            PendingAction::Navigate(intent) => intent,
        }
    }
}
fn legacy_action(intent: &NavigationIntent) -> PendingAction {
    match intent {
        NavigationIntent::ExitApplication => PendingAction::CloseApp,
        NavigationIntent::NewDemo => PendingAction::NewDemo,
        NavigationIntent::NewEmpty => PendingAction::NewEmpty,
        NavigationIntent::OpenFile(path) => PendingAction::OpenRecent(path.clone()),
        other => PendingAction::Navigate(other.clone()),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavigationOutcome {
    Completed,
    AwaitingDecision,
    ReviewingImport,
    Canceled,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GuardDecision {
    Save,
    Discard,
    Stay,
}

pub struct PendingNavigation {
    pub intent: NavigationIntent,
    pub source: SourceRef,
    pub origin: ReturnLocation,
    suspended_dialog: Option<Dialog>,
    prepared_document: Option<Document>,
}
pub struct DocumentSession {
    location: ReturnLocation,
    history: Vec<HistoryEntry>,
    future: Vec<HistoryEntry>,
    saved_depth: Option<usize>,
    saved_document: Option<Document>,
    dialog: Option<Dialog>,
    colors_mode: ColorsMode,
    map_mode: MapMode,
}
#[derive(Default)]
pub struct NavigationState {
    /// Changes when core state must reseed retained native input entities.
    pub context_epoch: u64,
    pub pending: Option<PendingNavigation>,
    pub back_stack: Vec<ReturnLocation>,
    pub(crate) sessions: HashMap<DocumentId, DocumentSession>,
    pub(crate) saved_document: Option<Document>,
}
impl NavigationState {
    pub fn checkpoint_saved(&self, version: &domain::document::Version) -> bool {
        self.saved_document
            .as_ref()
            .is_some_and(|d| d.versions.contains(version))
    }
    pub fn new(doc: &Document) -> Self {
        Self {
            saved_document: Some(doc.clone()),
            ..Self::default()
        }
    }
}

pub fn mark_saved(app: &mut AppState) {
    app.navigation.saved_document = Some(app.doc.clone());
}
pub fn mark_imported(app: &mut AppState) {
    app.navigation.saved_document = None;
    app.dirty = true;
    app.saved_depth = None;
}

/// Move owned editing state rather than clearing it when another document is shown.
pub(crate) fn suspend_session(app: &mut AppState) {
    let session = DocumentSession {
        location: ReturnLocation::capture(app),
        history: std::mem::take(&mut app.history),
        future: std::mem::take(&mut app.future),
        saved_depth: app.saved_depth,
        saved_document: app.navigation.saved_document.take(),
        dialog: app.dialog.take(),
        colors_mode: app.colors_mode,
        map_mode: app.map_mode,
    };
    app.navigation.sessions.insert(app.doc.id, session);
}

pub(crate) fn switch_inner(app: &mut AppState, id: DocumentId) -> Result<(), String> {
    if app.doc.id == id {
        return Ok(());
    }
    let index = app
        .library
        .iter()
        .position(|other| other.doc.id == id)
        .ok_or("The requested document is no longer open")?;
    suspend_session(app);
    let other = app.library.remove(index);
    let previous = OpenDoc {
        doc: std::mem::replace(&mut app.doc, other.doc),
        path: app.path.take(),
        dirty: app.dirty,
    };
    app.library.push(previous);
    app.path = other.path;
    app.dirty = other.dirty;
    app.history_highlight = None;
    app.map_drag_anchor = None;
    app.seq_drag_anchor = None;
    app.analysis_dirty = true;
    if let Some(session) = app.navigation.sessions.remove(&id) {
        let unchanged = session.location.selection.source == app.doc.source_ref();
        app.history = if unchanged { session.history } else { vec![] };
        app.future = if unchanged { session.future } else { vec![] };
        app.saved_depth = if unchanged { session.saved_depth } else { None };
        app.navigation.saved_document = session.saved_document;
        app.dialog = session.dialog;
        app.colors_mode = session.colors_mode;
        app.map_mode = session.map_mode;
        apply_location(app, session.location);
    } else {
        app.navigation.context_epoch = app.navigation.context_epoch.wrapping_add(1);
        app.saved_depth = if other.dirty { None } else { Some(0) };
        app.navigation.saved_document = (!other.dirty).then(|| app.doc.clone());
        app.selection = None;
        app.selection_strand = Strand::Top;
        app.selected_feature = None;
        app.selected_primer = None;
        app.zoom_start = 0;
        app.zoom_width = app.doc.sequence.len().max(1);
        app.search = SearchState::default();
        app.view = View::Map;
        app.feature_filter.clear();
        app.primer_filter.clear();
        app.feature_sort = TableSort::default();
        app.primer_sort = TableSort::default();
        app.enzyme_sort = TableSort::default();
    }
    Ok(())
}

fn apply_location(app: &mut AppState, location: ReturnLocation) {
    app.navigation.context_epoch = app.navigation.context_epoch.wrapping_add(1);
    app.view = location.route.view;
    let same_revision = app.doc.source_ref() == location.selection.source;
    app.selection = if same_revision {
        location.selection.range
    } else {
        None
    };
    app.selection_strand = if same_revision {
        location.selection.strand
    } else {
        Strand::Top
    };
    app.selected_feature = location
        .selection
        .feature
        .filter(|id| same_revision && app.doc.feature(*id).is_some());
    app.selected_primer = location
        .selection
        .primer
        .filter(|id| same_revision && app.doc.primer(*id).is_some());
    app.search = location.query;
    if !same_revision && app.search.open && !app.search.query.is_empty() {
        let previous_hit = app.search.current;
        app.run_search();
        app.search.current = previous_hit.min(app.search.hit_count().saturating_sub(1));
    }
    let state = location.view_state;
    let len = app.doc.sequence.len().max(1);
    app.zoom_start = state.zoom_start.min(len - 1);
    app.zoom_width = state.zoom_width.clamp(1, len);
    app.scroll_to = state.scroll_to.filter(|position| *position < len);
    app.feature_filter = state.feature_filter;
    app.primer_filter = state.primer_filter;
    app.feature_sort = state.feature_sort;
    app.primer_sort = state.primer_sort;
    app.enzyme_sort = state.enzyme_sort;
}

pub fn restore_return_location(app: &mut AppState, location: ReturnLocation) -> Result<(), String> {
    if app.navigation.pending.is_some() {
        return Err("Answer the unsaved-work decision first".into());
    }
    switch_inner(app, location.route.document)?;
    apply_location(app, location);
    Ok(())
}
pub fn go_back(app: &mut AppState) -> Result<NavigationOutcome, String> {
    let location = app
        .navigation
        .back_stack
        .last()
        .cloned()
        .ok_or("No previous location")?;
    restore_return_location(app, location)?;
    crate::workspace_search::depart(app);
    app.navigation.back_stack.pop();
    Ok(NavigationOutcome::Completed)
}

fn remember(app: &mut AppState, origin: ReturnLocation) {
    app.navigation.back_stack.push(origin);
    if app.navigation.back_stack.len() > 100 {
        app.navigation.back_stack.remove(0);
    }
}

pub fn request_navigation(
    app: &mut AppState,
    mut intent: NavigationIntent,
) -> Result<NavigationOutcome, String> {
    if app.navigation.pending.is_some() {
        return Err("Answer the unsaved-work decision first".into());
    }
    if matches!(intent, NavigationIntent::ExitApplication) && crate::first_use::is_sample(app) {
        if app.dialog.is_some() {
            return Err("Apply or cancel the sample form before exiting".into());
        }
        app.first_use.exit_after_sample = true;
        crate::first_use::leave_sample(app, false)?;
        return Ok(NavigationOutcome::AwaitingDecision);
    }
    let expected = if let NavigationIntent::OpenKnownFile { document, .. } = &intent {
        Some(*document)
    } else {
        None
    };
    if let NavigationIntent::OpenKnownFile { path, .. } = intent {
        intent = NavigationIntent::OpenFile(path);
    }
    if matches!(intent, NavigationIntent::OpenFilePicker) {
        let Some(path) = crate::fs_dialogs::pick_open(app.config.last_directory.as_ref()) else {
            return Ok(NavigationOutcome::Canceled);
        };
        intent = NavigationIntent::OpenFile(path);
    }
    let origin = ReturnLocation::capture(app);
    match &intent {
        NavigationIntent::Navigate(route) => {
            // Typed record routing belongs to each module; never silently ignore a target.
            if route.record.is_some() {
                return Err("This record route needs its module's typed record handler".into());
            }
            switch_inner(app, route.document)?;
            app.view = route.view;
            app.first_use.selected_result = None;
            crate::workspace_search::depart(app);
            if origin.route != *route {
                remember(app, origin);
            }
            return Ok(NavigationOutcome::Completed);
        }
        NavigationIntent::SwitchDocument(id) => {
            switch_inner(app, *id)?;
            crate::workspace_search::depart(app);
            if origin.route.document != *id {
                remember(app, origin);
            }
            return Ok(NavigationOutcome::Completed);
        }
        _ => {}
    }
    if matches!(intent, NavigationIntent::CloseDocument(id) if id != app.doc.id && !app.library.iter().any(|d| d.doc.id == id))
    {
        return Err("The document to close is no longer open".into());
    }
    let prepared = match &intent {
        NavigationIntent::OpenFile(path) => {
            let bytes = std::fs::read(path)
                .map_err(|e| format!("Could not open {}: {e}", path.display()))?;
            if expected.is_some() && !storage::imports::is_native_input(path, &bytes) {
                return Err("Choose the original native HelixBench document; this file has a different format".into());
            }
            if !storage::imports::is_native_input(path, &bytes) {
                crate::import::begin_review(
                    app,
                    crate::import::ImportSource::captured_file(path, bytes),
                    crate::import::ImportDestination::NewDocuments,
                )?;
                return Ok(NavigationOutcome::ReviewingImport);
            }
            Some(
                storage::load_captured(path, &bytes)
                    .map_err(|e| format!("Could not open {}: {e}", path.display()))?,
            )
        }
        _ => None,
    };
    if let Some(doc) = &prepared {
        if expected.is_some_and(|id| id != doc.id) {
            return Err("This file belongs to a different document. Locate the original file or open this file separately".into());
        }
        if doc.id == app.doc.id || app.library.iter().any(|other| other.doc.id == doc.id) {
            let id = doc.id;
            switch_inner(app, id)?;
            if origin.route.document != id {
                remember(app, origin);
            }
            crate::workspace_search::depart(app);
            app.toast("This document is already open — showing its existing draft");
            return Ok(NavigationOutcome::Completed);
        }
    }
    // Unapplied forms cannot be saved by saving the underlying document.
    if let Some(id) = blocking_form(app, &intent) {
        switch_inner(app, id)?;
        return Err(format!(
            "Apply or cancel the open form for \"{}\" before closing or replacing this document",
            app.doc.name
        ));
    }
    continue_intent(app, intent, origin, prepared)
}

fn is_edit_form(dialog: &Dialog) -> bool {
    !matches!(
        dialog,
        Dialog::About
            | Dialog::LetterCodes
            | Dialog::SearchTips
            | Dialog::Options
            | Dialog::Digest(_)
            | Dialog::EnzymeSites(_)
            | Dialog::HistoryReview(_)
            | Dialog::Audit(_)
    )
}

fn has_form(app: &AppState, id: DocumentId) -> bool {
    if app.doc.id == id {
        app.dialog.as_ref().is_some_and(is_edit_form)
    } else {
        app.navigation
            .sessions
            .get(&id)
            .and_then(|session| session.dialog.as_ref())
            .is_some_and(is_edit_form)
    }
}

fn blocking_form(app: &AppState, intent: &NavigationIntent) -> Option<DocumentId> {
    match intent {
        NavigationIntent::ExitApplication => std::iter::once(app.doc.id)
            .chain(app.library.iter().map(|doc| doc.doc.id))
            .find(|id| has_form(app, *id)),
        NavigationIntent::CloseDocument(id) => has_form(app, *id).then_some(*id),
        _ => has_form(app, app.doc.id).then_some(app.doc.id),
    }
}

fn dirty_target(app: &AppState, intent: &NavigationIntent) -> Option<DocumentId> {
    match intent {
        NavigationIntent::ExitApplication => {
            if app.dirty {
                Some(app.doc.id)
            } else {
                app.library
                    .iter()
                    .find(|doc| doc.dirty)
                    .map(|doc| doc.doc.id)
            }
        }
        NavigationIntent::CloseDocument(id) => {
            if *id == app.doc.id {
                app.dirty.then_some(*id)
            } else {
                app.library
                    .iter()
                    .find(|doc| doc.doc.id == *id && doc.dirty)
                    .map(|doc| doc.doc.id)
            }
        }
        _ => app.dirty.then_some(app.doc.id),
    }
}

fn continue_intent(
    app: &mut AppState,
    intent: NavigationIntent,
    origin: ReturnLocation,
    prepared: Option<Document>,
) -> Result<NavigationOutcome, String> {
    if let Some(target) = dirty_target(app, &intent) {
        switch_inner(app, target)?;
        let suspended_dialog = app.dialog.take();
        let source = app.doc.source_ref();
        app.dialog = Some(Dialog::Unsaved {
            action: legacy_action(&intent),
        });
        app.navigation.pending = Some(PendingNavigation {
            intent,
            source,
            origin,
            suspended_dialog,
            prepared_document: prepared,
        });
        return Ok(NavigationOutcome::AwaitingDecision);
    }
    execute(app, intent, prepared)?;
    Ok(NavigationOutcome::Completed)
}

pub fn resolve_guard(
    app: &mut AppState,
    decision: GuardDecision,
) -> Result<NavigationOutcome, String> {
    let mut pending = app
        .navigation
        .pending
        .take()
        .ok_or("No unsaved-work decision is pending")?;
    if decision == GuardDecision::Stay {
        app.dialog = pending.suspended_dialog.take();
        restore_return_location(app, pending.origin)?;
        return Ok(NavigationOutcome::Canceled);
    }
    if pending.source != app.doc.source_ref() {
        app.navigation.pending = Some(pending);
        return Err(
            "The source changed while awaiting a decision; choose Stay and try again".into(),
        );
    }
    if decision == GuardDecision::Save && !actions::save(app) {
        app.navigation.pending = Some(pending);
        return Ok(NavigationOutcome::AwaitingDecision);
    }
    if decision == GuardDecision::Discard {
        if let Some(saved) = app.navigation.saved_document.clone() {
            app.doc = saved;
        } else {
            // A never-saved draft is discarded in full. Keep no fixture data.
            app.doc = Document::new("Untitled".into(), vec![], domain::Topology::Linear);
            app.doc.id = pending.source.document;
            app.path = None;
        }
        app.history.clear();
        app.future.clear();
        app.saved_depth = Some(0);
        app.dirty = false;
        app.selection = None;
        app.selection_strand = Strand::Top;
        app.selected_feature = None;
        app.selected_primer = None;
        app.analysis_dirty = true;
    }
    app.dialog = pending.suspended_dialog.take();
    continue_intent(
        app,
        pending.intent,
        pending.origin,
        pending.prepared_document,
    )
}

pub fn has_unsaved_work(app: &AppState) -> bool {
    crate::first_use::is_sample(app)
        || app.dirty
        || app.library.iter().any(|doc| doc.dirty)
        || app.navigation.pending.is_some()
        || app.dialog.as_ref().is_some_and(is_edit_form)
        || app.library.iter().any(|doc| has_form(app, doc.doc.id))
}

pub fn pending_destination(app: &AppState) -> String {
    match app
        .navigation
        .pending
        .as_ref()
        .map(|pending| &pending.intent)
    {
        Some(NavigationIntent::OpenFile(path)) => format!("Before opening {}", path.display()),
        Some(NavigationIntent::CloseDocument(_)) => "Before closing this document".into(),
        Some(NavigationIntent::ExitApplication) => {
            "Before closing HelixBench; each unsaved document is reviewed separately".into()
        }
        Some(NavigationIntent::NewEmpty) => "Before creating a new sequence".into(),
        Some(NavigationIntent::NewDemo) => "Before opening the demo sequence".into(),
        _ => "Before continuing".into(),
    }
}

fn execute(
    app: &mut AppState,
    intent: NavigationIntent,
    prepared: Option<Document>,
) -> Result<(), String> {
    match intent {
        NavigationIntent::OpenFile(path) => actions::install_opened_document(
            app,
            prepared.ok_or("Missing reviewed file input")?,
            &path,
        ),
        NavigationIntent::NewEmpty => {
            app.dialog = Some(Dialog::NewSequence(crate::state::NewSeqState::default()))
        }
        NavigationIntent::NewDemo => actions::new_demo(app),
        NavigationIntent::ExitApplication => {
            // The native boundary persists settings and handles failure before exiting.
            app.exit_requested = true;
        }
        NavigationIntent::CloseDocument(id) => {
            if app.doc.id == id {
                if let Some(next) = app.library.first().map(|doc| doc.doc.id) {
                    switch_inner(app, next)?;
                    app.library.retain(|doc| doc.doc.id != id);
                } else {
                    app.set_document(
                        Document::new("Untitled".into(), vec![], domain::Topology::Linear),
                        None,
                    );
                    app.view = View::Home;
                }
            } else {
                app.library.retain(|doc| doc.doc.id != id);
            }
            app.navigation.sessions.remove(&id);
        }
        _ => return Err("Navigation destination was not prepared".into()),
    }
    crate::workspace_search::depart(app);
    Ok(())
}
