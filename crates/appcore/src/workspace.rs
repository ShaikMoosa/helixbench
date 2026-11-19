//! Object-centered workspace destinations; no scientific mutation or persistence schema.
use crate::{
    navigation::{self, NavigationIntent, Route},
    state::{AppState, View},
};
use domain::DocumentId;
use std::path::PathBuf;

pub const WORKSPACE: [View; 4] = [View::Home, View::Sequences, View::Results, View::Notebook];
pub const RECORDS: [View; 2] = [View::Registry, View::Inventory];
pub const OPERATIONS: [View; 3] = [View::Workflows, View::Insights, View::Automation];
pub const SPECIALISTS: [View; 5] = [
    View::Biologics,
    View::Chemistry,
    View::Rna,
    View::InVivo,
    View::Bioprocess,
];

pub fn owner(view: View) -> View {
    if View::ALL.contains(&view) || view == View::Split {
        View::Sequences
    } else {
        view
    }
}
pub fn enter(app: &mut AppState, view: View) -> Result<(), String> {
    if app.dialog.is_some() || app.navigation.pending.is_some() {
        return Err("Finish or cancel the open form first".into());
    }
    if view == View::Results {
        if app.view == View::Results
            && app.results.list
            && app.results.pending.is_none()
            && app.first_use.selected_result.is_none()
        {
            return Ok(());
        }
        return crate::results::browse(app);
    }
    navigation::request_navigation(
        app,
        NavigationIntent::Navigate(Route {
            view,
            document: app.doc.id,
            record: None,
        }),
    )?;
    Ok(())
}
#[derive(Clone)]
pub struct SequenceSummary {
    pub id: DocumentId,
    pub name: String,
    pub length: usize,
    pub circular: bool,
    pub dirty: bool,
    pub path: Option<PathBuf>,
    pub active: bool,
}
impl SequenceSummary {
    pub fn persistence(&self) -> &'static str {
        if self.path.is_some() && !self.dirty {
            "Saved locally"
        } else {
            "Unsaved document"
        }
    }
}
pub fn sequences(app: &AppState, query: &str) -> Vec<SequenceSummary> {
    let query = query.trim().to_lowercase();
    std::iter::once((&app.doc, &app.path, app.dirty, true))
        .chain(
            app.library
                .iter()
                .map(|d| (&d.doc, &d.path, d.dirty, false)),
        )
        .filter(|(doc, path, dirty, _)| {
            !(doc.name == "Untitled" && doc.sequence.is_empty() && path.is_none() && !*dirty)
        })
        .filter(|(doc, path, _, _)| {
            format!(
                "{} {} DNA",
                doc.name,
                path.as_ref()
                    .map(|p| p.display().to_string())
                    .unwrap_or_default()
            )
            .to_lowercase()
            .contains(&query)
        })
        .map(|(doc, path, dirty, active)| SequenceSummary {
            id: doc.id,
            name: doc.name.clone(),
            length: doc.sequence.len(),
            circular: doc.sequence.is_circular(),
            dirty,
            path: path.clone(),
            active,
        })
        .collect()
}
pub fn open_sequence(app: &mut AppState, id: DocumentId) -> Result<(), String> {
    if app.dialog.is_some() || app.navigation.pending.is_some() {
        return Err("Finish or cancel the open form first".into());
    }
    navigation::request_navigation(
        app,
        NavigationIntent::Navigate(Route {
            view: View::Map,
            document: id,
            record: None,
        }),
    )?;
    Ok(())
}
