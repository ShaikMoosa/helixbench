//! RNA / modified oligos: RNA sequences, a modified-monomer library (sugar /
//! base / phosphate), bulk modification patterns, and a linear HELM subset.
//! Pure Rust, offline.

use serde::{Deserialize, Serialize};

/// A monomer (nucleotide) with optional sugar / base / phosphate modification.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Monomer {
    pub code: String,
    pub name: String,
    pub base: char,
    pub sugar: String,
    pub phosphate: String,
    /// Residue molecular weight (monophosphate, minus water for polymer).
    pub mw: f32,
}

/// Built-in monomer / modification library.
pub fn monomer_library() -> Vec<Monomer> {
    let std = [
        ("A", "Adenosine", 'A', "ribose", "PO", 329.21),
        ("U", "Uridine", 'U', "ribose", "PO", 306.17),
        ("G", "Guanosine", 'G', "ribose", "PO", 345.21),
        ("C", "Cytidine", 'C', "ribose", "PO", 305.18),
    ];
    let mods = [
        ("mA", "2'-O-Methyl A", 'A', "2'-OMe", "PO", 343.24),
        ("mU", "2'-O-Methyl U", 'U', "2'-OMe", "PO", 320.20),
        ("mG", "2'-O-Methyl G", 'G', "2'-OMe", "PO", 359.24),
        ("mC", "2'-O-Methyl C", 'C', "2'-OMe", "PO", 319.21),
        ("fA", "2'-F A", 'A', "2'-F", "PO", 347.20),
        ("fU", "2'-F U", 'U', "2'-F", "PO", 324.16),
        ("fC", "2'-F C", 'C', "2'-F", "PO", 323.17),
        ("lA", "LNA A", 'A', "LNA", "PO", 341.22),
        ("lC", "LNA C", 'C', "LNA", "PO", 317.19),
        ("sA", "Phosphorothioate A", 'A', "ribose", "PS", 345.28),
        ("sU", "Phosphorothioate U", 'U', "ribose", "PS", 322.24),
        ("sG", "Phosphorothioate G", 'G', "ribose", "PS", 361.28),
        ("sC", "Phosphorothioate C", 'C', "ribose", "PS", 321.25),
        ("5mC", "5-Methyl C", 'C', "ribose", "PO", 319.21),
        ("Psi", "Pseudouridine", 'U', "ribose", "PO", 306.17),
    ];
    let mut out: Vec<Monomer> = std
        .iter()
        .map(|(c, n, b, s, p, mw)| Monomer {
            code: (*c).into(),
            name: (*n).into(),
            base: *b,
            sugar: (*s).into(),
            phosphate: (*p).into(),
            mw: *mw,
        })
        .collect();
    out.extend(mods.iter().map(|(c, n, b, s, p, mw)| Monomer {
        code: (*c).into(),
        name: (*n).into(),
        base: *b,
        sugar: (*s).into(),
        phosphate: (*p).into(),
        mw: *mw,
    }));
    out
}

/// A modification applied at a 0-based position.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Mod {
    pub pos: usize,
    pub code: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Oligo {
    pub id: u64,
    pub name: String,
    /// RNA sequence (A/U/G/C).
    pub seq: String,
    pub mods: Vec<Mod>,
}

/// Bulk modification pattern.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pattern {
    All,
    Alternating,
    EveryThird,
    Ends,
}

impl Pattern {
    pub const ALL: [Pattern; 4] = [
        Pattern::All,
        Pattern::Alternating,
        Pattern::EveryThird,
        Pattern::Ends,
    ];
    pub fn label(self) -> &'static str {
        match self {
            Pattern::All => "Every position",
            Pattern::Alternating => "Alternating",
            Pattern::EveryThird => "Every 3rd",
            Pattern::Ends => "Ends (3 each)",
        }
    }
    pub fn positions(self, len: usize) -> Vec<usize> {
        match self {
            Pattern::All => (0..len).collect(),
            Pattern::Alternating => (0..len).step_by(2).collect(),
            Pattern::EveryThird => (0..len).step_by(3).collect(),
            Pattern::Ends => {
                let mut v: Vec<usize> = (0..len.min(3)).collect();
                if len > 3 {
                    v.extend((len - 3)..len);
                }
                v.sort_unstable();
                v.dedup();
                v
            }
        }
    }
}

pub fn transcribe(dna: &str) -> String {
    dna.chars()
        .map(|c| match c.to_ascii_uppercase() {
            'T' => 'U',
            'A' => 'A',
            'G' => 'G',
            'C' => 'C',
            _ => 'N',
        })
        .collect()
}

pub fn rev_comp_rna(seq: &str) -> String {
    seq.chars()
        .rev()
        .map(|c| match c {
            'A' => 'U',
            'U' => 'A',
            'G' => 'C',
            'C' => 'G',
            _ => 'N',
        })
        .collect()
}

impl Oligo {
    pub fn len(&self) -> usize {
        self.seq.len()
    }

    pub fn is_empty(&self) -> bool {
        self.seq.is_empty()
    }

    pub fn gc_percent(&self) -> f32 {
        if self.seq.is_empty() {
            return 0.0;
        }
        let gc = self.seq.chars().filter(|c| *c == 'G' || *c == 'C').count();
        gc as f32 * 100.0 / self.seq.len() as f32
    }

    /// Molecular weight: sum of residue weights (modified where present) minus
    /// water for each phosphodiester linkage, plus a 5'-phosphate estimate.
    pub fn molecular_weight(&self) -> f32 {
        let lib = monomer_library();
        let mut mw = 0.0;
        for (i, base) in self.seq.chars().enumerate() {
            let code = self
                .mods
                .iter()
                .find(|m| m.pos == i)
                .map(|m| m.code.clone())
                .unwrap_or_else(|| base.to_string());
            let w = lib
                .iter()
                .find(|m| m.code == code)
                .map(|m| m.mw)
                .unwrap_or(320.0);
            mw += w;
        }
        if self.seq.len() > 1 {
            mw -= 18.02 * (self.seq.len() - 1) as f32;
        }
        mw
    }

    /// Linear HELM subset: RNA1{<tokens>} where a token is a base letter or a
    /// bracketed monomer code, separated by P (phosphate) except the last.
    pub fn to_helm(&self) -> String {
        let mut tokens = Vec::new();
        for (i, base) in self.seq.chars().enumerate() {
            match self.mods.iter().find(|m| m.pos == i) {
                Some(m) => tokens.push(format!("[{}]", m.code)),
                None => tokens.push(base.to_string()),
            }
        }
        let body = tokens
            .iter()
            .enumerate()
            .map(|(i, t)| {
                if i + 1 < tokens.len() {
                    format!("{t}P")
                } else {
                    t.clone()
                }
            })
            .collect::<Vec<_>>()
            .join("");
        format!("RNA1{{{body}}}")
    }

    /// Parse a linear HELM subset produced by `to_helm`.
    pub fn from_helm(name: &str, helm: &str) -> Result<Oligo, String> {
        let inner = helm
            .trim()
            .strip_prefix("RNA1{")
            .and_then(|s| s.strip_suffix('}'))
            .ok_or("expected RNA1{...}")?;
        let lib = monomer_library();
        let mut seq = String::new();
        let mut mods = Vec::new();
        let mut chars = inner.chars().peekable();
        let mut pos = 0usize;
        while let Some(c) = chars.next() {
            let code = if c == '[' {
                let mut code = String::new();
                for n in chars.by_ref() {
                    if n == ']' {
                        break;
                    }
                    code.push(n);
                }
                code
            } else {
                c.to_string()
            };
            if chars.peek() == Some(&'P') {
                chars.next();
            }
            let mon = lib
                .iter()
                .find(|m| m.code == code)
                .ok_or_else(|| format!("unknown monomer '{code}'"))?;
            seq.push(mon.base);
            if mon.code != mon.base.to_string() {
                mods.push(Mod {
                    pos,
                    code: mon.code.clone(),
                });
            }
            pos += 1;
        }
        if seq.is_empty() {
            return Err("empty HELM".into());
        }
        Ok(Oligo {
            id: 0,
            name: name.into(),
            seq,
            mods,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transcribe_and_revcomp() {
        assert_eq!(transcribe("ATGC"), "AUGC");
        assert_eq!(rev_comp_rna("AUGC"), "GCAU");
    }

    #[test]
    fn patterns() {
        assert_eq!(Pattern::All.positions(4), vec![0, 1, 2, 3]);
        assert_eq!(Pattern::Alternating.positions(6), vec![0, 2, 4]);
        assert_eq!(Pattern::EveryThird.positions(7), vec![0, 3, 6]);
        assert_eq!(Pattern::Ends.positions(8), vec![0, 1, 2, 5, 6, 7]);
    }

    #[test]
    fn helm_roundtrip_with_mods() {
        let o = Oligo {
            id: 1,
            name: "gapmer".into(),
            seq: "AUGCA".into(),
            mods: vec![
                Mod {
                    pos: 0,
                    code: "mA".into(),
                },
                Mod {
                    pos: 4,
                    code: "sA".into(),
                },
            ],
        };
        let helm = o.to_helm();
        assert_eq!(helm, "RNA1{[mA]PUPGPCP[sA]}");
        let back = Oligo::from_helm("gapmer", &helm).unwrap();
        assert_eq!(back.seq, o.seq);
        assert_eq!(back.mods, o.mods);
    }

    #[test]
    fn mw_and_gc() {
        let o = Oligo {
            id: 1,
            name: "x".into(),
            seq: "GGCC".into(),
            mods: vec![],
        };
        assert_eq!(o.gc_percent(), 100.0);
        assert!(o.molecular_weight() > 1000.0);
    }
}
