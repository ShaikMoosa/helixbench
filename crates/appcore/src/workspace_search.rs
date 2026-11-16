//! Scoped workspace metadata search, separate from active-sequence Find.
use crate::{
    commands::{self, CommandId},
    navigation::{self, NavigationIntent, ReturnLocation, Route},
    state::{AppState, View},
};
use domain::{DocumentId, RecordRef, StoreKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SearchScope {
    #[default]
    All,
    Documents,
    Notebook,
    Registry,
    Inventory,
    Workflows,
    Projects,
    Tools,
}
impl SearchScope {
    pub const ALL: [Self; 8] = [
        Self::All,
        Self::Documents,
        Self::Notebook,
        Self::Registry,
        Self::Inventory,
        Self::Workflows,
        Self::Projects,
        Self::Tools,
    ];
    pub fn label(self) -> &'static str {
        match self {
            Self::All => "All",
            Self::Documents => "Open documents",
            Self::Notebook => "Notebook",
            Self::Registry => "Registry",
            Self::Inventory => "Inventory",
            Self::Workflows => "Workflows",
            Self::Projects => "Projects",
            Self::Tools => "Tools & pages",
        }
    }
}
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SearchQuery {
    pub text: String,
    pub scope: SearchScope,
    pub include_archived: bool,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SearchTarget {
    Document(DocumentId),
    Record(RecordRef),
    Project(u64),
    Command(CommandId),
}
#[derive(Debug, Clone)]
pub struct SearchHit {
    pub target: SearchTarget,
    pub title: String,
    pub kind: &'static str,
    pub detail: String,
    pub view: View,
    pub archived: bool,
}
#[derive(Clone)]
pub struct SearchRequest {
    generation: u64,
    query: SearchQuery,
}
#[derive(Default)]
pub struct WorkspaceSearch {
    pub open: bool,
    pub focus_request: bool,
    pub query: SearchQuery,
    pub results: Vec<SearchHit>,
    pub selected: Option<SearchTarget>,
    pub inspected: Option<SearchTarget>,
    pub error: Option<String>,
    pub visible_limit: usize,
    pub can_return: bool,
    generation: u64,
    origin: Option<ReturnLocation>,
}
impl WorkspaceSearch {
    pub fn begin(&mut self, query: SearchQuery) -> SearchRequest {
        self.generation = self.generation.wrapping_add(1);
        if self.query != query {
            self.selected = None;
            self.visible_limit = 50;
        }
        self.query = query.clone();
        self.error = None;
        SearchRequest {
            generation: self.generation,
            query,
        }
    }
    pub fn accept(&mut self, request: &SearchRequest, hits: Vec<SearchHit>) -> bool {
        if !self.open || request.generation != self.generation || request.query != self.query {
            return false;
        }
        self.results = hits;
        if self
            .selected
            .is_none_or(|target| !self.results.iter().any(|h| h.target == target))
        {
            self.selected = self.results.first().map(|h| h.target);
        }
        true
    }
}
fn record(store: StoreKind, id: u64) -> SearchTarget {
    SearchTarget::Record(RecordRef {
        store,
        local_id: id,
    })
}
fn index(app: &AppState) -> Vec<SearchHit> {
    let mut hits = vec![];
    for doc in std::iter::once(&app.doc).chain(app.library.iter().map(|d| &d.doc)) {
        hits.push(SearchHit {
            target: SearchTarget::Document(doc.id),
            title: doc.name.clone(),
            kind: "Open document",
            detail: format!(
                "{} bp · {} · {}",
                doc.sequence.len(),
                doc.id,
                doc.tags.join(", ")
            ),
            view: View::Sequence,
            archived: false,
        });
    }
    for entry in &app.records.entries {
        hits.push(SearchHit {
            target: record(StoreKind::Notebook, entry.id),
            title: entry.title.clone(),
            kind: "Notebook entry",
            detail: format!(
                "{} · ID {}\n{}",
                entry.status.label(),
                entry.id,
                entry
                    .sections
                    .iter()
                    .map(|s| format!("{}: {}", s.title, s.body))
                    .collect::<Vec<_>>()
                    .join("\n")
            ),
            view: View::Notebook,
            archived: entry.status == domain::notebook::EntryStatus::Archived,
        });
    }
    for entry in &app.registry.entries {
        hits.push(SearchHit {
            target: record(StoreKind::Registry, entry.id),
            title: entry.name().into(),
            kind: "Registry entity",
            detail: format!(
                "{} · ID {}\n{}",
                entry.kind.label(),
                entry.id,
                entry
                    .values
                    .iter()
                    .map(|(k, v)| format!("{k}: {v}"))
                    .collect::<Vec<_>>()
                    .join("\n")
            ),
            view: View::Registry,
            archived: entry.archived,
        });
    }
    for entry in &app.inventory.containers {
        hits.push(SearchHit {
            target: record(StoreKind::Inventory, entry.id),
            title: entry.name.clone(),
            kind: "Inventory container",
            detail: format!(
                "{} · ID {} · {} {}\nPosition: {} · Linked entity: {}",
                entry.kind.label(),
                entry.id,
                entry.quantity,
                entry.unit,
                entry.position.as_deref().unwrap_or("Unassigned"),
                entry.entity.as_deref().unwrap_or("None")
            ),
            view: View::Inventory,
            archived: false,
        });
    }
    for entry in &app.workflows.tasks {
        hits.push(SearchHit {
            target: record(StoreKind::Workflow, entry.id),
            title: entry.title.clone(),
            kind: "Workflow task",
            detail: format!(
                "{} · ID {} · Assignee: {}\nSchema: {}",
                entry.status.label(),
                entry.id,
                entry.assignee,
                entry.schema
            ),
            view: View::Workflows,
            archived: false,
        });
    }
    for entry in &app.records.workspace.projects {
        hits.push(SearchHit {
            target: SearchTarget::Project(entry.id),
            title: entry.name.clone(),
            kind: "Project",
            detail: format!("Project ID {} · Parent: {:?}", entry.id, entry.parent),
            view: View::Home,
            archived: false,
        });
    }
    for descriptor in commands::catalog() {
        let availability = commands::availability(app, descriptor.id);
        hits.push(SearchHit {
            target: SearchTarget::Command(descriptor.id),
            title: descriptor.label.into(),
            kind: "Tool / page",
            detail: format!(
                "{} · {} {}",
                descriptor.owner.label(),
                descriptor.keywords,
                availability.reason()
            ),
            view: descriptor.owner,
            archived: false,
        });
    }
    hits
}
fn in_scope(target: SearchTarget, scope: SearchScope) -> bool {
    match (scope, target) {
        (SearchScope::All, _)
        | (SearchScope::Documents, SearchTarget::Document(_))
        | (SearchScope::Tools, SearchTarget::Command(_))
        | (SearchScope::Projects, SearchTarget::Project(_)) => true,
        (SearchScope::Notebook, SearchTarget::Record(r)) => r.store == StoreKind::Notebook,
        (SearchScope::Registry, SearchTarget::Record(r)) => r.store == StoreKind::Registry,
        (SearchScope::Inventory, SearchTarget::Record(r)) => r.store == StoreKind::Inventory,
        (SearchScope::Workflows, SearchTarget::Record(r)) => r.store == StoreKind::Workflow,
        _ => false,
    }
}
pub fn lookup(app: &AppState, target: SearchTarget) -> Option<SearchHit> {
    index(app).into_iter().find(|h| h.target == target)
}
pub fn run(app: &mut AppState, query: SearchQuery) {
    let request = app.workspace_search.begin(query.clone());
    let tokens: Vec<_> = query
        .text
        .split_whitespace()
        .map(str::to_lowercase)
        .collect();
    let mut hits: Vec<_> = index(app)
        .into_iter()
        .filter(|hit| {
            in_scope(hit.target, query.scope) && (query.include_archived || !hit.archived)
        })
        .filter(|hit| {
            let text = format!("{} {} {}", hit.title, hit.kind, hit.detail).to_lowercase();
            tokens.iter().all(|token| text.contains(token))
        })
        .collect();
    hits.sort_by(|a, b| {
        a.title
            .to_lowercase()
            .cmp(&b.title.to_lowercase())
            .then_with(|| format!("{:?}", a.target).cmp(&format!("{:?}", b.target)))
    });
    app.workspace_search.accept(&request, hits);
}
fn ready(app: &AppState) -> Result<(), String> {
    if app.dialog.is_some() || app.navigation.pending.is_some() {
        Err("Apply or cancel the open form or decision first".into())
    } else {
        Ok(())
    }
}
pub fn open(app: &mut AppState) -> Result<(), String> {
    ready(app)?;
    if app.workspace_search.origin.is_none() {
        app.workspace_search.origin = Some(ReturnLocation::capture(app));
    }
    app.workspace_search.open = true;
    app.workspace_search.focus_request = true;
    app.workspace_search.visible_limit = app.workspace_search.visible_limit.max(50);
    run(app, app.workspace_search.query.clone());
    Ok(())
}
pub fn close(app: &mut AppState) -> Result<(), String> {
    ready(app)?;
    if let Some(origin) = app.workspace_search.origin.clone() {
        if app.doc.id == origin.route.document
            || app
                .library
                .iter()
                .any(|doc| doc.doc.id == origin.route.document)
        {
            navigation::restore_return_location(app, origin)?;
        } else {
            app.toast("The original document is no longer open; keeping the current page");
        }
    }
    app.workspace_search.open = false;
    app.workspace_search.can_return = false;
    app.workspace_search.inspected = None;
    app.workspace_search.origin = None;
    app.workspace_search.generation = app.workspace_search.generation.wrapping_add(1);
    Ok(())
}
pub fn open_target(app: &mut AppState, target: SearchTarget) -> Result<(), String> {
    ready(app)?;
    if !app.workspace_search.open {
        return Err("Search is closed; open workspace search again".into());
    }
    let hit = lookup(app, target)
        .ok_or("This item is no longer available. Refresh search or choose another result")?;
    match target {
        SearchTarget::Command(id) => commands::execute(app, id)?,
        SearchTarget::Document(id) => {
            navigation::request_navigation(
                app,
                NavigationIntent::Navigate(Route {
                    view: View::Sequence,
                    document: id,
                    record: None,
                }),
            )?;
        }
        _ => {
            navigation::request_navigation(
                app,
                NavigationIntent::Navigate(Route {
                    view: hit.view,
                    document: app.doc.id,
                    record: None,
                }),
            )?;
        }
    }
    app.workspace_search.selected = Some(target);
    app.workspace_search.inspected = Some(target);
    app.workspace_search.open = false;
    app.workspace_search.can_return = true;
    app.workspace_search.generation = app.workspace_search.generation.wrapping_add(1);
    Ok(())
}
pub fn back_to_search(app: &mut AppState) -> Result<(), String> {
    ready(app)?;
    open(app)
}
pub fn edit_inspected(app: &mut AppState) -> Result<(), String> {
    ready(app)?;
    let target = app.workspace_search.inspected.ok_or("No item selected")?;
    lookup(app, target).ok_or("This item was deleted. Return to search and refresh")?;
    if let Some(reason) = edit_unavailable(app, target) {
        return Err(reason.into());
    }
    match target {
        SearchTarget::Record(RecordRef {
            store: StoreKind::Notebook,
            local_id,
        }) => crate::actions::open_entry_editor(app, Some(local_id), 0),
        SearchTarget::Record(RecordRef {
            store: StoreKind::Registry,
            local_id,
        }) => {
            let entry = app
                .registry
                .get(local_id)
                .ok_or("Registry entity no longer exists")?;
            let kind = domain::registry::Kind::ALL
                .iter()
                .position(|k| *k == entry.kind)
                .unwrap();
            crate::actions::open_registry_editor(app, kind, Some(local_id));
        }
        _ => return Err("Use the owning page's controls for this item".into()),
    }
    Ok(())
}

/// A successful ordinary navigation leaves search but keeps an explicit return path.
pub(crate) fn depart(app: &mut AppState) {
    if app.workspace_search.open {
        app.workspace_search.open = false;
        app.workspace_search.can_return = true;
        app.workspace_search.inspected = None;
        app.workspace_search.generation = app.workspace_search.generation.wrapping_add(1);
    }
}

pub fn edit_unavailable(app: &AppState, target: SearchTarget) -> Option<&'static str> {
    match target {
        SearchTarget::Record(RecordRef {
            store: StoreKind::Notebook,
            local_id,
        }) => match app.records.entries.iter().find(|e| e.id == local_id) {
            Some(entry) if entry.locked() => {
                Some("This notebook entry is locked; its contents are read-only")
            }
            Some(_) => None,
            None => Some("This notebook entry is no longer available"),
        },
        SearchTarget::Record(RecordRef {
            store: StoreKind::Registry,
            local_id,
        }) => match app.registry.get(local_id) {
            Some(entry) if entry.archived => {
                Some("This entity is archived; restore it from Registry before editing")
            }
            Some(_) => None,
            None => Some("This registry entity is no longer available"),
        },
        _ => Some("Use the owning page's controls for this item"),
    }
}
