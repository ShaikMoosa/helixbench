//! Framework-agnostic application core: document state, undo/redo, search,
//! enzyme analysis and typed actions. Shared by every UI shell.

pub mod actions;
pub mod analysis;
pub mod fs_dialogs;
pub mod import;
pub mod navigation;
pub mod operations;
pub mod state;
