use serde::{Deserialize, Serialize};

use crate::enzyme::Enzyme;
use crate::enzyme_db;
use crate::sequence::DnaSequence;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnzymeSet {
    pub name: String,
    pub enzymes: Vec<String>,
    pub builtin: bool,
}

impl EnzymeSet {
    pub fn custom(name: String, enzymes: Vec<String>) -> Self {
        Self {
            name,
            enzymes,
            builtin: false,
        }
    }
}

/// Predefined sets computed against the current sequence, mirroring the
/// guide's Unique Cutters / Unique & Dual / 6+ / Unique 6+ families.
pub fn predefined_sets(seq: &DnaSequence) -> Vec<EnzymeSet> {
    let all = enzyme_db::all_enzymes();
    let counts: Vec<(&Enzyme, usize)> = all.iter().map(|e| (e, e.find_sites(seq).len())).collect();

    let names = |pred: &dyn Fn(&Enzyme, usize) -> bool| -> Vec<String> {
        counts
            .iter()
            .filter(|(e, n)| pred(e, *n))
            .map(|(e, _)| e.name.clone())
            .collect()
    };

    vec![
        EnzymeSet {
            name: "All Enzymes".into(),
            enzymes: all.iter().map(|e| e.name.clone()).collect(),
            builtin: true,
        },
        EnzymeSet {
            name: "Unique Cutters".into(),
            enzymes: names(&|_, n| n == 1),
            builtin: true,
        },
        EnzymeSet {
            name: "Unique & Dual Cutters".into(),
            enzymes: names(&|_, n| n == 1 || n == 2),
            builtin: true,
        },
        EnzymeSet {
            name: "6+ Cutters".into(),
            enzymes: names(&|e, _| e.site_len() >= 6),
            builtin: true,
        },
        EnzymeSet {
            name: "Unique 6+ Cutters".into(),
            enzymes: names(&|e, n| e.site_len() >= 6 && n == 1),
            builtin: true,
        },
        EnzymeSet {
            name: "Common Cloning Enzymes".into(),
            enzymes: enzyme_db::COMMON_CLONING
                .iter()
                .map(|s| s.to_string())
                .collect(),
            builtin: true,
        },
        EnzymeSet {
            name: "Type IIS (Golden Gate)".into(),
            enzymes: [
                "BsaI", "BbsI", "BsmBI", "SapI", "BspMI", "BtsI", "BsrDI", "FokI",
            ]
            .iter()
            .map(|s| s.to_string())
            .collect(),
            builtin: true,
        },
        EnzymeSet {
            name: "Noncutters".into(),
            enzymes: names(&|_, n| n == 0),
            builtin: true,
        },
    ]
}

/// Filter criteria for the Choose Enzymes dialog.
#[derive(Debug, Clone, Default)]
pub struct EnzymeFilter {
    pub name_contains: String,
    /// (min, max) number of cut sites, inclusive.
    pub cut_count: Option<(usize, usize)>,
    pub overhang: Option<OverhangFilter>,
    pub site_len: Option<(usize, usize)>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OverhangFilter {
    FivePrime,
    ThreePrime,
    Blunt,
}

pub fn filter_enzymes(seq: &DnaSequence, filter: &EnzymeFilter) -> Vec<(Enzyme, usize)> {
    let all = enzyme_db::all_enzymes();
    let mut out = Vec::new();
    for e in all {
        if !filter.name_contains.is_empty()
            && !e
                .name
                .to_lowercase()
                .contains(&filter.name_contains.to_lowercase())
        {
            continue;
        }
        let n = e.find_sites(seq).len();
        if let Some((lo, hi)) = filter.cut_count {
            if n < lo || n > hi {
                continue;
            }
        }
        if let Some((lo, hi)) = filter.site_len {
            if e.site_len() < lo || e.site_len() > hi {
                continue;
            }
        }
        if let Some(oh) = filter.overhang {
            let ok = matches!(
                (oh, e.overhang()),
                (
                    OverhangFilter::FivePrime,
                    crate::enzyme::Overhang::FivePrime(_)
                ) | (
                    OverhangFilter::ThreePrime,
                    crate::enzyme::Overhang::ThreePrime(_)
                ) | (OverhangFilter::Blunt, crate::enzyme::Overhang::Blunt)
            );
            if !ok {
                continue;
            }
        }
        out.push((e, n));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sequence::Topology;

    #[test]
    fn unique_cutters() {
        let seq = DnaSequence::new(b"AAAGAATTCCCGGGAAAGAATTCCC".to_vec(), Topology::Linear);
        let sets = predefined_sets(&seq);
        let unique = sets.iter().find(|s| s.name == "Unique Cutters").unwrap();
        assert!(!unique.enzymes.contains(&"EcoRI".to_string()));
        let non = sets.iter().find(|s| s.name == "Noncutters").unwrap();
        assert!(non.enzymes.contains(&"NotI".to_string()));
    }

    #[test]
    fn filter_by_overhang() {
        let seq = DnaSequence::new(b"AAAGAATTCCCGGGAA".to_vec(), Topology::Linear);
        let f = EnzymeFilter {
            name_contains: "eco".into(),
            overhang: Some(OverhangFilter::FivePrime),
            ..Default::default()
        };
        let hits = filter_enzymes(&seq, &f);
        assert!(hits.iter().any(|(e, _)| e.name == "EcoRI"));
        assert!(!hits.iter().any(|(e, _)| e.name == "EcoRV"));
    }
}
