//! Explicit first use, isolated sample workspaces and identity-qualified resume.
use crate::{
    navigation::{self, NavigationIntent, Route},
    state::{AppState, Dialog, OpenDoc, View},
    workspace_search::{self, SearchTarget},
};
use domain::{Document, ResultId, Topology};
use std::path::PathBuf;
use storage::config::{Config, RecentTarget, RecentWork};

#[derive(Default)]
pub struct FirstUseState {
    personal: Option<Box<AppState>>,
    pub exit_after_sample: bool,
    pub results: storage::results::ResultCatalog,
    pub results_error: Option<String>,
    pub selected_result: Option<ResultId>,
    pub error: Option<String>,
}
impl FirstUseState {
    pub fn for_profile(root: &std::path::Path) -> Self {
        let mut state = Self::default();
        match storage::results::load_from(&root.join("results.json")) {
            Ok(results) => state.results = results,
            Err(error) => {
                state.results_error = Some(format!(
                    "Saved results could not be read: {error}. The catalog is preserved."
                ))
            }
        }
        state
    }
}
#[derive(Clone)]
pub struct SampleCopyDraft {
    pub document: Document,
    pub name: String,
}
pub fn startup(config: Config) -> AppState {
    startup_in(config, storage::config::config_dir())
}
pub fn startup_in(config: Config, root: PathBuf) -> AppState {
    let mut app = AppState::new_in(
        Document::new("Untitled".into(), vec![], Topology::Linear),
        config,
        root,
    );
    app.view = View::Home;
    app
}
pub fn is_sample(app: &AppState) -> bool {
    app.first_use.personal.is_some()
}
fn ready(app: &AppState) -> Result<(), String> {
    if app.dialog.is_some() || app.navigation.pending.is_some() {
        Err("Apply or cancel the open form or decision first".into())
    } else {
        Ok(())
    }
}
pub fn try_sample(app: &mut AppState) -> Result<(), String> {
    ready(app)?;
    if is_sample(app) {
        return Err("You are already exploring a sample".into());
    }
    let root = std::env::temp_dir()
        .join("HelixBench-samples")
        .join(domain::OperationId::new().to_string());
    let config = Config {
        ui: app.prefs.clone(),
        first_use_dismissed: true,
        ..Default::default()
    };
    let mut sample = AppState::new_in(domain::demo::build_demo(), config, root);
    sample.view = View::Map;
    let personal = std::mem::replace(app, sample);
    app.first_use.personal = Some(Box::new(personal));
    app.toast("Sample workspace — temporary data, separate from your personal work");
    Ok(())
}
pub fn request_personal_copy(app: &mut AppState) -> Result<(), String> {
    if !is_sample(app) {
        return Err("This is already your personal workspace".into());
    }
    if !matches!(app.dialog, None | Some(Dialog::SampleExit)) {
        return Err("Apply or cancel the open form before copying".into());
    }
    app.dialog = Some(Dialog::SampleCopy(Box::new(SampleCopyDraft {
        document: app.doc.clone(),
        name: format!("{} (personal copy)", app.doc.name),
    })));
    Ok(())
}
pub fn cancel_sample_dialog(app: &mut AppState) {
    app.dialog = None;
    app.first_use.exit_after_sample = false;
}
fn restore_personal(app: &mut AppState) -> Result<bool, String> {
    let exit = app.first_use.exit_after_sample;
    let personal = app
        .first_use
        .personal
        .take()
        .ok_or("No sample workspace is active")?;
    *app = *personal;
    Ok(exit)
}
pub fn leave_sample(app: &mut AppState, discard: bool) -> Result<bool, String> {
    if !is_sample(app) {
        return Err("No sample workspace is active".into());
    }
    if !matches!(app.dialog, None | Some(Dialog::SampleExit)) {
        return Err("Apply or cancel the open form before leaving the sample".into());
    }
    if !discard {
        app.dialog = Some(Dialog::SampleExit);
        return Ok(false);
    }
    let exit = restore_personal(app)?;
    if exit {
        navigation::request_navigation(app, NavigationIntent::ExitApplication)?;
    }
    Ok(true)
}
pub fn confirm_personal_copy(app: &mut AppState, name: String) -> Result<(), String> {
    let Some(Dialog::SampleCopy(draft)) = &app.dialog else {
        return Err("Review the personal copy first".into());
    };
    if name.trim().is_empty() {
        return Err("Give your personal copy a name".into());
    }
    if app.doc != draft.document {
        return Err("The sample changed; cancel and review a fresh copy".into());
    }
    let copy = draft.document.independent_copy(name.trim().into());
    let exit = restore_personal(app)?;
    navigation::suspend_session(app);
    let previous = OpenDoc {
        doc: std::mem::replace(&mut app.doc, copy.clone()),
        path: app.path.take(),
        dirty: app.dirty,
    };
    app.library.push(previous);
    app.set_document(copy, None);
    navigation::mark_imported(app);
    app.view = View::Map;
    app.toast("Personal copy created with a new identity — save it to keep your work");
    if exit {
        navigation::request_navigation(app, NavigationIntent::ExitApplication)?;
    }
    Ok(())
}
pub fn dismiss_guidance(app: &mut AppState) -> Result<(), String> {
    let previous = app.config.first_use_dismissed;
    app.config.first_use_dismissed = true;
    if let Err(error) = storage::config::save_to(
        &app.config,
        &app.profile_path(storage::config::config_path()),
    ) {
        app.config.first_use_dismissed = previous;
        return Err(format!("Could not save Home preference: {error}"));
    }
    Ok(())
}
pub fn remember(app: &mut AppState, target: RecentTarget, title: String) {
    app.config.remember_work(RecentWork { target, title });
    if let Err(error) = storage::config::save_to(
        &app.config,
        &app.profile_path(storage::config::config_path()),
    ) {
        app.operation_error = Some(format!(
            "Opened work, but recent-work settings could not be saved: {error}"
        ));
    }
}
pub fn resume(app: &mut AppState, target: RecentTarget) -> Result<(), String> {
    ready(app)?;
    let title = match &target {
        RecentTarget::Document { document, path } => {
            if app.doc.id == *document || app.library.iter().any(|d| d.doc.id == *document) {
                navigation::request_navigation(
                    app,
                    NavigationIntent::Navigate(Route {
                        view: View::Sequence,
                        document: *document,
                        record: None,
                    }),
                )?;
                app.doc.name.clone()
            } else {
                let path=path.clone().ok_or("This unsaved document is no longer open. Unsaved drafts cannot be restored after exit")?;
                navigation::request_navigation(
                    app,
                    NavigationIntent::OpenKnownFile {
                        path,
                        document: *document,
                    },
                )?;
                // The actual install after any dirty-work decision remembers its identity.
                return Ok(());
            }
        }
        RecentTarget::File(path) => {
            navigation::request_navigation(app, NavigationIntent::OpenFile(path.clone()))?;
            return Ok(());
        }
        RecentTarget::Record(record) => {
            let hit = workspace_search::lookup(app, SearchTarget::Record(*record)).ok_or(
                "This record is no longer available. Refresh recent work or forget this link",
            )?;
            navigation::request_navigation(
                app,
                NavigationIntent::Navigate(Route {
                    view: hit.view,
                    document: app.doc.id,
                    record: None,
                }),
            )?;
            app.workspace_search.inspected = Some(hit.target);
            hit.title
        }
        RecentTarget::Result(id) => {
            crate::results::open(app, *id)?;
            "Saved analysis result".into()
        }
    };
    app.first_use.error = None;
    remember(app, target, title);
    Ok(())
}
pub fn refresh_results(app: &mut AppState) {
    match storage::results::load_from(&app.profile_path(storage::results::path())) {
        Ok(results) => {
            app.first_use.results = results;
            app.first_use.results_error = None;
        }
        Err(error) => {
            app.first_use.results_error = Some(format!(
                "Could not read saved results: {error}. The original catalog is preserved"
            ));
        }
    }
}
pub fn forget(app: &mut AppState, target: &RecentTarget) -> Result<(), String> {
    let previous = app.config.clone();
    app.config
        .recent_work
        .retain(|work| !work.target.same_identity(target));
    if let RecentTarget::File(path) = target {
        app.config.remove_recent(path);
    }
    if let Err(error) = storage::config::save_to(
        &app.config,
        &app.profile_path(storage::config::config_path()),
    ) {
        app.config = previous;
        return Err(format!("Could not forget recent link: {error}"));
    }
    Ok(())
}

/// UI-owned drafts/scroll handles follow the same suspend/restore boundary as AppState.
pub struct ProfileUiSession<T> {
    personal: Option<T>,
    sample: bool,
}
impl<T> Default for ProfileUiSession<T> {
    fn default() -> Self {
        Self {
            personal: None,
            sample: false,
        }
    }
}
impl<T: Default> ProfileUiSession<T> {
    pub fn sample_active(&self) -> bool {
        self.sample
    }
    pub fn sync(&mut self, sample: bool, current: &mut T) {
        if self.sample == sample {
            return;
        }
        if sample {
            self.personal = Some(std::mem::take(current));
        } else {
            *current = self.personal.take().unwrap_or_default();
        }
        self.sample = sample;
    }
}
