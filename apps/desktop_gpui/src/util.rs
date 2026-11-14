//! Small shared helpers for the GPUI shell.

use appcore::state::{AppState, SearchMode};
use domain::Range;

/// Ranges highlighted by the active DNA/protein search (empty for name search).
pub fn search_hit_ranges(app: &AppState) -> Vec<Range> {
    if !app.search.open {
        return Vec::new();
    }
    match app.search.mode {
        SearchMode::Dna => app
            .search
            .dna_hits
            .iter()
            .map(|h| Range::new(h.start, h.len))
            .collect(),
        SearchMode::Protein => app
            .search
            .protein_hits
            .iter()
            .map(|h| Range::new(h.start, h.len))
            .collect(),
        SearchMode::Name => Vec::new(),
    }
}

/// Open the most recent existing document, else the seeded demo plasmid.
pub fn initial_document(config: &storage::config::Config) -> domain::Document {
    for path in &config.recent_files {
        if path.exists() {
            if let Ok(doc) = storage::load_any(path) {
                return doc;
            }
        }
    }
    domain::demo::build_demo()
}
