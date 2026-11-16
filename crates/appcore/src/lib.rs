//! Framework-agnostic application core: document state, undo/redo, search,
//! enzyme analysis and typed actions. Shared by every UI shell.

pub mod actions;
pub mod analysis;
pub mod annotation_review;
pub mod commands;
pub mod first_use;
pub mod fs_dialogs;
pub mod import;
pub mod navigation;
pub mod operations;
pub mod selection;
pub mod shell;
pub mod state;
pub mod workspace_search;
