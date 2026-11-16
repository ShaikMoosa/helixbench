//! Reviewed, all-or-nothing in-memory import. Saving remains an explicit action.
use crate::{
    navigation::{self, ReturnLocation},
    state::{AppState, Dialog, OpenDoc, View},
};
use domain::{Document, DocumentId, OperationId, Range, SourceRef};
use std::collections::HashMap;
pub use storage::imports::{preview_import, ImportOptions, ImportPreview, ImportSource, Molecule};

#[derive(Debug, Clone)]
pub enum ImportDestination {
    NewDocuments,
    Insert {
        source: SourceRef,
        selection: Option<Range>,
        replace_selection: bool,
    },
}
pub struct ImportReviewDraft {
    pub preview: ImportPreview,
    pub options: HashMap<usize, ImportOptions>,
    pub selected: Vec<bool>,
    pub names: Vec<String>,
    pub active: usize,
    pub acknowledge_changes: bool,
    pub acknowledge_skipped: bool,
    pub error: Option<String>,
    pub destination: ImportDestination,
    pub revision: u64,
    operation: OperationId,
    origin: ReturnLocation,
    previous_dialog: Option<Box<Dialog>>,
}
impl ImportReviewDraft {
    pub fn invalidate(&mut self) {
        self.revision = self.revision.wrapping_add(1);
        self.error = None;
    }
    pub fn refresh(&mut self) -> Result<(), String> {
        self.preview = preview_import(self.preview.source.clone(), &self.options)
            .map_err(|e| e.to_string())?;
        self.acknowledge_changes = false;
        self.invalidate();
        Ok(())
    }
    pub fn operation(&self) -> OperationId {
        self.operation
    }
}
#[derive(Clone)]
pub struct ReviewedImportPlan {
    operation: OperationId,
    review_revision: u64,
    documents: Vec<Document>,
    destination: ImportDestination,
}

pub fn begin_review(
    app: &mut AppState,
    source: ImportSource,
    destination: ImportDestination,
) -> Result<(), String> {
    if app.navigation.pending.is_some() {
        return Err("Answer the unsaved-work decision before importing".into());
    }
    if app.dialog.is_some() {
        return Err("Apply or cancel the current form before starting another import".into());
    }
    begin_review_inner(app, source, destination)
}
fn begin_review_inner(
    app: &mut AppState,
    source: ImportSource,
    destination: ImportDestination,
) -> Result<(), String> {
    let preview = preview_import(source, &HashMap::new()).map_err(|e| e.to_string())?;
    let selected = preview
        .records
        .iter()
        .map(|row| row.document.is_some() && row.errors.is_empty())
        .collect();
    let names = preview.records.iter().map(|row| row.name.clone()).collect();
    let origin = ReturnLocation::capture(app);
    let previous_dialog = app.dialog.take().map(Box::new);
    app.dialog = Some(Dialog::ImportReview(Box::new(ImportReviewDraft {
        preview,
        options: HashMap::new(),
        selected,
        names,
        active: 0,
        acknowledge_changes: false,
        acknowledge_skipped: false,
        error: None,
        destination,
        revision: 0,
        operation: OperationId::new(),
        origin,
        previous_dialog,
    })));
    Ok(())
}
pub fn review_new_sequence(app: &mut AppState) -> Result<(), String> {
    let Some(Dialog::NewSequence(form)) = &app.dialog else {
        return Err("No new-sequence form is open".into());
    };
    let source = ImportSource::text(
        format!(
            "{}.txt",
            if form.name.trim().is_empty() {
                "Untitled sequence"
            } else {
                form.name.trim()
            }
        ),
        form.text.clone(),
    );
    let topology = form.topology;
    begin_review_inner(app, source, ImportDestination::NewDocuments)?;
    if let Some(Dialog::ImportReview(draft)) = &mut app.dialog {
        if draft.preview.format == storage::imports::ImportFormat::Raw {
            draft.options.insert(
                1,
                ImportOptions {
                    topology: Some(topology),
                    ..Default::default()
                },
            );
            draft.refresh()?;
        }
    }
    Ok(())
}
pub fn review_paste(app: &mut AppState, text: &str) -> Result<(), String> {
    if !matches!(app.view, View::Sequence | View::Map) {
        return Err("Open Map or Sequence to paste into a molecule".into());
    }
    let destination = ImportDestination::Insert {
        source: app.doc.source_ref(),
        selection: app.selection,
        replace_selection: true,
    };
    begin_review(app, ImportSource::text("Clipboard.txt", text), destination)
}

pub fn reviewed_plan(draft: &ImportReviewDraft) -> Result<ReviewedImportPlan, String> {
    if draft.selected.len() != draft.preview.records.len()
        || draft.names.len() != draft.preview.records.len()
    {
        return Err("Import review no longer matches the source; reopen it".into());
    }
    let mut documents = vec![];
    for (i, row) in draft.preview.records.iter().enumerate() {
        if !draft.selected[i] {
            continue;
        }
        if !row.errors.is_empty() {
            return Err(format!(
                "Record {} is invalid: {}",
                row.index,
                row.errors.join("; ")
            ));
        }
        if row.requires_acknowledgment && !draft.acknowledge_changes {
            return Err(
                "Review and acknowledge the listed conversions/metadata changes before importing"
                    .into(),
            );
        }
        let mut doc = row
            .document
            .clone()
            .ok_or_else(|| format!("Record {} has no supported DNA interpretation", row.index))?;
        let name = draft.names[i].trim();
        if name.is_empty() {
            return Err(format!("Give record {} a name", row.index));
        }
        doc.name = name.into();
        if name != row.name {
            if let Some(source) = &mut doc.import_source {
                source
                    .transformations
                    .push(format!("Renamed record to {name}"));
            }
        }
        documents.push(doc);
    }
    if documents.is_empty() {
        return Err("Select at least one valid DNA record".into());
    }
    if documents.len() != draft.preview.records.len() && !draft.acknowledge_skipped {
        return Err(
            "Acknowledge the unselected records before importing only the selected subset".into(),
        );
    }
    if matches!(draft.destination, ImportDestination::Insert { .. }) {
        if documents.len() != 1 {
            return Err("Select exactly one record for sequence insertion".into());
        }
        if !documents[0].features.is_empty() || !documents[0].primers.is_empty() {
            return Err(
                "Annotated input must open as a new document to preserve its annotations".into(),
            );
        }
    }
    Ok(ReviewedImportPlan {
        operation: draft.operation,
        review_revision: draft.revision,
        documents,
        destination: draft.destination.clone(),
    })
}

pub fn commit_import(
    app: &mut AppState,
    plan: &ReviewedImportPlan,
) -> Result<Vec<DocumentId>, String> {
    if let Some(ids) = app.imported_operations.get(&plan.operation) {
        return Ok(ids.clone());
    }
    if app.navigation.pending.is_some() {
        return Err("Answer the unsaved-work decision first".into());
    }
    if !matches!(&app.dialog, Some(Dialog::ImportReview(draft)) if draft.operation == plan.operation && draft.revision == plan.review_revision)
    {
        return Err(
            "This review changed or is no longer active; review the current choices again".into(),
        );
    }
    for doc in &plan.documents {
        if doc.id.is_nil()
            || doc.revision_id.is_nil()
            || app.doc.id == doc.id
            || app.library.iter().any(|other| other.doc.id == doc.id)
        {
            return Err(
                "An imported document identity already exists; reopen the import as a new copy"
                    .into(),
            );
        }
    }
    let ids = match &plan.destination {
        ImportDestination::NewDocuments => {
            app.dialog = None;
            navigation::suspend_session(app);
            let mut docs = plan.documents.clone().into_iter();
            let first = docs.next().ok_or("No reviewed records")?;
            let previous = OpenDoc {
                doc: std::mem::replace(&mut app.doc, first),
                path: app.path.take(),
                dirty: app.dirty,
            };
            app.library.push(previous);
            app.library.extend(docs.map(|doc| OpenDoc {
                doc,
                path: None,
                dirty: true,
            }));
            navigation::mark_imported(app);
            app.navigation.context_epoch = app.navigation.context_epoch.wrapping_add(1);
            app.history.clear();
            app.future.clear();
            app.history_highlight = None;
            app.selection = None;
            app.selected_feature = None;
            app.selected_primer = None;
            app.search = Default::default();
            app.zoom_start = 0;
            app.zoom_width = app.doc.sequence.len().max(1);
            app.feature_filter.clear();
            app.primer_filter.clear();
            app.feature_sort = Default::default();
            app.primer_sort = Default::default();
            app.enzyme_sort = Default::default();
            app.scroll_to = None;
            app.map_drag_anchor = None;
            app.seq_drag_anchor = None;
            app.analysis_dirty = true;
            app.view = View::Map;
            plan.documents.iter().map(|doc| doc.id).collect::<Vec<_>>()
        }
        ImportDestination::Insert {
            source,
            selection,
            replace_selection,
        } => {
            if app.doc.source_ref() != *source {
                return Err(
                    "The paste target changed; cancel and paste into the intended revision again"
                        .into(),
                );
            }
            let imported = &plan.documents[0];
            let bases = &imported.sequence.bases;
            let position = selection
                .map(|range| range.start)
                .unwrap_or(app.doc.sequence.len());
            if let Some(range) = selection {
                if range.start > app.doc.sequence.len()
                    || range.len > app.doc.sequence.len()
                    || (!app.doc.sequence.is_circular()
                        && range
                            .start
                            .checked_add(range.len)
                            .is_none_or(|end| end > app.doc.sequence.len()))
                {
                    return Err("The captured paste range is invalid".into());
                }
            }
            app.mutate("Paste reviewed sequence", vec![], |doc| {
                if let Some(range) = selection.filter(|range| *replace_selection && range.len > 0) {
                    doc.replace_range(range, bases);
                } else {
                    doc.insert_bases(position, bases);
                }
                if let Some(provenance) = &imported.import_source {
                    let mut provenance = provenance.clone();
                    provenance.target = Some(domain::document::PasteTarget {
                        source: *source,
                        selection: *selection,
                        replace_selection: *replace_selection,
                        insertion_position: position,
                    });
                    doc.import_history.push(provenance);
                }
            });
            app.select(Some(Range::new(position, bases.len())));
            app.scroll_to = Some(position);
            app.dialog = None;
            vec![app.doc.id]
        }
    };
    app.imported_operations.insert(plan.operation, ids.clone());
    app.toast(format!(
        "Imported {} reviewed record(s) — unsaved; save to keep your work",
        plan.documents.len()
    ));
    Ok(ids)
}

pub fn confirm_import(app: &mut AppState) -> Result<Vec<DocumentId>, String> {
    let Some(Dialog::ImportReview(draft)) = &app.dialog else {
        return Err("No import is being reviewed".into());
    };
    let plan = reviewed_plan(draft)?;
    commit_import(app, &plan)
}
pub fn cancel_import(app: &mut AppState) -> Result<(), String> {
    if !matches!(app.dialog, Some(Dialog::ImportReview(_))) {
        return Err("No import is being reviewed".into());
    }
    let Some(Dialog::ImportReview(draft)) = app.dialog.take() else {
        return Err("No import is being reviewed".into());
    };
    app.dialog = draft.previous_dialog.map(|dialog| *dialog);
    navigation::restore_return_location(app, draft.origin)
}
