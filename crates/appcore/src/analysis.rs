//! Cached sequence analysis: enzyme sites, ORFs, primer binding.

use domain::primer::{binding_sites, HybridizationParams};
use domain::Document;
use storage::config::UiPrefs;

use crate::state::Analysis;

pub fn compute(doc: &Document, prefs: &UiPrefs) -> Analysis {
    let mut a = Analysis::default();
    let seq = &doc.sequence;
    if seq.is_empty() {
        return a;
    }

    for e in domain::enzyme_db::all_enzymes() {
        let sites = e.find_sites(seq);
        a.counts.insert(e.name.clone(), sites.len());
        a.all_sites.insert(e.name.clone(), sites);
    }
    a.sets = domain::enzyme_set::predefined_sets(seq);
    for name in &doc.chosen_enzymes {
        if let Some(e) = domain::enzyme_db::enzyme_by_name(name) {
            let sites = e.find_sites(seq);
            if !sites.is_empty() {
                a.chosen.push((e, sites));
            }
        }
    }
    a.chosen.sort_by(|x, y| x.0.name.cmp(&y.0.name));

    let code = domain::genetic_code::code_by_id(doc.default_genetic_code);
    a.orfs = domain::translate::find_orfs(seq, code, prefs.orf_min_aa, true);

    let params = HybridizationParams::default();
    for p in &doc.primers {
        let sites = binding_sites(p, seq, &params);
        a.primer_sites.insert(p.id, sites);
    }
    a
}
