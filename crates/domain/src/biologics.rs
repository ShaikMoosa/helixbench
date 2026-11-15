//! Biologics: format-aware protein entities (IgG / bispecific / nanobody / TCR /
//! Fab), chains + domains, and residue numbering schemes. Local-first, pure Rust.
//!
//! Numbering note: Kabat / IMGT here are *simplified display* schemes — residues
//! are walked across canonical framework/CDR regions with insertion letters for
//! over-length CDRs. They are approximations for visualization, not a substitute
//! for a full structural alignment.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Format {
    IgG,
    Bispecific,
    Nanobody,
    Tcr,
    Fab,
    Other,
}

impl Format {
    pub const ALL: [Format; 6] = [
        Format::IgG,
        Format::Bispecific,
        Format::Nanobody,
        Format::Tcr,
        Format::Fab,
        Format::Other,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Format::IgG => "IgG",
            Format::Bispecific => "Bispecific",
            Format::Nanobody => "Nanobody",
            Format::Tcr => "TCR",
            Format::Fab => "Fab",
            Format::Other => "Other",
        }
    }

    /// Expected chain composition, for the registration wizard hint.
    pub fn chains_hint(self) -> &'static str {
        match self {
            Format::IgG => "2 heavy + 2 light",
            Format::Bispecific => "2 distinct heavy + 2 light",
            Format::Nanobody => "1 single-domain (VHH)",
            Format::Tcr => "1 alpha + 1 beta",
            Format::Fab => "1 heavy (Fd) + 1 light",
            Format::Other => "custom chains",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChainKind {
    Heavy,
    Light,
    Alpha,
    Beta,
    Single,
    Other,
}

impl ChainKind {
    pub const ALL: [ChainKind; 6] = [
        ChainKind::Heavy,
        ChainKind::Light,
        ChainKind::Alpha,
        ChainKind::Beta,
        ChainKind::Single,
        ChainKind::Other,
    ];

    pub fn label(self) -> &'static str {
        match self {
            ChainKind::Heavy => "Heavy",
            ChainKind::Light => "Light",
            ChainKind::Alpha => "Alpha",
            ChainKind::Beta => "Beta",
            ChainKind::Single => "VHH",
            ChainKind::Other => "Other",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Chain {
    pub name: String,
    pub kind: ChainKind,
    /// Amino-acid sequence (one-letter).
    pub sequence: String,
}

/// A domain/region on a chain, 1-based inclusive residue bounds.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Domain {
    pub chain: usize,
    pub name: String,
    pub kind: String,
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Numbering {
    Sequential,
    Kabat,
    Imgt,
}

impl Numbering {
    pub const ALL: [Numbering; 3] = [Numbering::Sequential, Numbering::Kabat, Numbering::Imgt];

    pub fn label(self) -> &'static str {
        match self {
            Numbering::Sequential => "Sequential",
            Numbering::Kabat => "Kabat",
            Numbering::Imgt => "IMGT",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Biologic {
    pub id: u64,
    pub name: String,
    pub format: Format,
    pub chains: Vec<Chain>,
    pub domains: Vec<Domain>,
    pub numbering: Numbering,
    pub created: u64,
    pub updated: u64,
}

impl Biologic {
    pub fn total_residues(&self) -> usize {
        self.chains.iter().map(|c| c.sequence.len()).sum()
    }
}

/// Expected chain kinds per format, in registration order.
pub fn chain_kinds(f: Format) -> &'static [ChainKind] {
    match f {
        Format::IgG => &[ChainKind::Heavy, ChainKind::Light],
        Format::Bispecific => &[ChainKind::Heavy, ChainKind::Heavy, ChainKind::Light],
        Format::Nanobody => &[ChainKind::Single],
        Format::Tcr => &[ChainKind::Alpha, ChainKind::Beta],
        Format::Fab => &[ChainKind::Heavy, ChainKind::Light],
        Format::Other => &[ChainKind::Other],
    }
}

/// Derive coarse domain bounds (V/C regions) from chain length — a local
/// approximation for display, not a structural annotation.
pub fn auto_domains(chains: &[Chain]) -> Vec<Domain> {
    let mut out = Vec::new();
    for (i, c) in chains.iter().enumerate() {
        let len = c.sequence.len();
        if len == 0 {
            continue;
        }
        let (v_name, v_len) = match c.kind {
            ChainKind::Heavy | ChainKind::Single => ("VH", 120usize),
            ChainKind::Light => ("VL", 110usize),
            ChainKind::Alpha => ("Vα", 110usize),
            ChainKind::Beta => ("Vβ", 110usize),
            ChainKind::Other => ("V", 110usize),
        };
        let v_end = v_len.min(len);
        out.push(Domain {
            chain: i,
            name: v_name.into(),
            kind: "variable".into(),
            start: 1,
            end: v_end,
        });
        if len > v_end {
            let rest = len - v_end;
            let c_names: &[&str] = match c.kind {
                ChainKind::Heavy => &["CH1", "CH2", "CH3"],
                ChainKind::Light => &["CL"],
                ChainKind::Alpha => &["Cα"],
                ChainKind::Beta => &["Cβ"],
                ChainKind::Single | ChainKind::Other => &["C"],
            };
            let n = c_names.len();
            let per = rest / n;
            let mut start = v_end + 1;
            for (k, cn) in c_names.iter().enumerate() {
                let end = if k == n - 1 { len } else { start + per - 1 };
                if end >= start {
                    out.push(Domain {
                        chain: i,
                        name: (*cn).into(),
                        kind: "constant".into(),
                        start,
                        end,
                    });
                }
                start = end + 1;
            }
        }
    }
    out
}

/// A numbered residue: position label, region, amino acid.
#[derive(Debug, Clone, PartialEq)]
pub struct NumberedResidue {
    pub label: String,
    pub region: &'static str,
    pub aa: char,
}

// Canonical (region, length, start-position) for a VH domain.
const KABAT_VH: &[(&str, usize, usize)] = &[
    ("FR1", 30, 1),
    ("CDR1", 5, 31),
    ("FR2", 14, 36),
    ("CDR2", 16, 50),
    ("FR3", 29, 66),
    ("CDR3", 8, 95),
    ("FR4", 11, 103),
];
const IMGT_VH: &[(&str, usize, usize)] = &[
    ("FR1", 26, 1),
    ("CDR1", 12, 27),
    ("FR2", 17, 39),
    ("CDR2", 10, 56),
    ("FR3", 39, 66),
    ("CDR3", 13, 105),
    ("FR4", 11, 118),
];

/// Number a chain under a scheme. Sequential is exact; Kabat/IMGT walk canonical
/// regions and add insertion letters (a, b, …) for over-length CDR residues.
pub fn number_chain(sequence: &str, scheme: Numbering) -> Vec<NumberedResidue> {
    let aas: Vec<char> = sequence
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .collect();
    if scheme == Numbering::Sequential {
        return aas
            .iter()
            .enumerate()
            .map(|(i, a)| NumberedResidue {
                label: (i + 1).to_string(),
                region: "",
                aa: *a,
            })
            .collect();
    }
    let regions = if scheme == Numbering::Kabat {
        KABAT_VH
    } else {
        IMGT_VH
    };
    let mut out = Vec::new();
    let mut idx = 0usize;
    for &(region, canon, start) in regions {
        let mut pos = start;
        let mut taken = 0usize;
        while idx < aas.len() && taken < canon {
            out.push(NumberedResidue {
                label: pos.to_string(),
                region,
                aa: aas[idx],
            });
            idx += 1;
            taken += 1;
            pos += 1;
        }
        if idx >= aas.len() {
            break;
        }
    }
    // Any remainder (longer than the canonical domain) trails FR4 with letters.
    let mut letter = b'a';
    while idx < aas.len() {
        out.push(NumberedResidue {
            label: format!("128{}", letter as char),
            region: "FR4",
            aa: aas[idx],
        });
        idx += 1;
        letter += 1;
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sequential_is_exact() {
        let n = number_chain("ACDEFG", Numbering::Sequential);
        assert_eq!(n.len(), 6);
        assert_eq!(n[0].label, "1");
        assert_eq!(n[5].label, "6");
    }

    #[test]
    fn kabat_cdr1_starts_at_31() {
        let seq = "A".repeat(120);
        let n = number_chain(&seq, Numbering::Kabat);
        let cdr1: Vec<&NumberedResidue> = n.iter().filter(|r| r.region == "CDR1").collect();
        assert_eq!(cdr1[0].label, "31");
        assert_eq!(cdr1.len(), 5);
    }

    #[test]
    fn imgt_cdr1_starts_at_27() {
        let seq = "A".repeat(130);
        let n = number_chain(&seq, Numbering::Imgt);
        let cdr1: Vec<&NumberedResidue> = n.iter().filter(|r| r.region == "CDR1").collect();
        assert_eq!(cdr1[0].label, "27");
    }

    #[test]
    fn format_and_chain_labels() {
        assert_eq!(Format::Nanobody.chains_hint(), "1 single-domain (VHH)");
        assert_eq!(ChainKind::Heavy.label(), "Heavy");
        let b = Biologic {
            id: 1,
            name: "mAb1".into(),
            format: Format::IgG,
            chains: vec![
                Chain {
                    name: "HC".into(),
                    kind: ChainKind::Heavy,
                    sequence: "ACD".into(),
                },
                Chain {
                    name: "LC".into(),
                    kind: ChainKind::Light,
                    sequence: "EF".into(),
                },
            ],
            domains: vec![],
            numbering: Numbering::Kabat,
            created: 0,
            updated: 0,
        };
        assert_eq!(b.total_residues(), 5);
    }
}
