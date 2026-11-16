//! Operation orchestration and truthful completion feedback shared by UI shells.
use crate::state::AppState;
use domain::operation::{OperationReceipt, OperationState};
use std::path::Path;
use storage::operations::{Journal, WritePlan};

pub fn dispatch_saved(app: &mut AppState, plan: &WritePlan, receipt: &OperationReceipt) {
    if !plan.matches_native_receipt(receipt) {
        return;
    }
    // Compare the full captured document, including version history, not just its revision.
    if app.doc == *plan.document() {
        app.path = Some(receipt.destination.clone());
        crate::navigation::mark_saved(app);
        app.dirty = false;
        app.saved_depth = Some(app.history.len());
    }
    // An inactive document stays dirty until its owning session can reconcile its baseline.
    // Never clear the active document's state because an older/different source completed.
}

pub fn save_to(app: &mut AppState, path: &Path, journal: &Journal, config_path: &Path) -> bool {
    if crate::first_use::is_sample(app) {
        app.operation_error=Some("This is a temporary sample workspace. Use Make personal copy before saving a personal document.".into());
        return false;
    }

    let result = journal
        .prepare_native(&app.doc, path)
        .and_then(|plan| journal.execute(plan.id()).map(|receipt| (plan, receipt)));
    match result {
        Ok((plan, receipt)) if receipt.state == OperationState::Completed => {
            dispatch_saved(app, &plan, &receipt);
            app.config.add_recent(&receipt.destination);
            app.config.remember_work(storage::config::RecentWork {
                target: storage::config::RecentTarget::Document {
                    document: receipt.source.document,
                    path: Some(receipt.destination.clone()),
                },
                title: plan.document().name.clone(),
            });
            app.operation_error = storage::config::save_to(&app.config, config_path).err()
                .map(|error| format!("Document saved, but settings could not be saved: {error}. Save again to retry settings."));
            app.exit_settings_failed = false;
            app.toast(format!("Saved {}", receipt.destination.display()));
            app.recovery = journal.recovery();
            true
        }
        Ok(_) => {
            app.operation_error = Some("The file was written, but its completion receipt could not be saved. Review recovery before retrying.".into());
            app.recovery = journal.recovery();
            false
        }
        Err(error) => {
            app.operation_error = Some(format!(
                "Save failed: {error}. Your document remains open; retry Save or choose Save As."
            ));
            app.recovery = journal.recovery();
            false
        }
    }
}

pub fn export_to(app: &mut AppState, path: &Path, text: String, label: &str, journal: &Journal) {
    match journal.prepare_text(&app.doc, path, text).and_then(|plan| journal.execute(plan.id())) {
        Ok(receipt) if receipt.state == OperationState::Completed => app.toast(format!("Exported {label} to {}", receipt.destination.display())),
        Ok(_) => app.operation_error = Some("Export was written, but completion could not be recorded. Review recovery before retrying.".into()),
        Err(error) => app.operation_error = Some(format!("Export failed: {error}. Retry Export or choose another destination.")),
    }
    app.recovery = journal.recovery();
}

pub fn reconcile(app: &mut AppState, id: domain::OperationId, journal: &Journal) {
    match journal.execute(id) {
        Ok(receipt) if receipt.state == OperationState::Completed => app.toast(format!(
            "Recovered output: {}",
            receipt.destination.display()
        )),
        Ok(_) => {
            app.operation_error =
                Some("Output is present, but its completion receipt still cannot be saved.".into())
        }
        Err(error) => app.operation_error = Some(format!("Recovery failed: {error}")),
    }
    app.recovery = journal.recovery();
}

/// The explicit settings-only fallback must still respect edits made after failed Quit.
pub fn can_exit_without_settings(app: &AppState) -> bool {
    app.exit_settings_failed && !crate::navigation::has_unsaved_work(app)
}
