//! Construct assembly simulation: Digest & Ligate, Gibson, and Golden Gate.
//! Linearizes a vector at a cut site and joins inserts in order, reporting junctions.

use crate::sequence::DnaSequence;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Strategy {
    DigestLigate,
    Gibson,
    GoldenGate,
}

impl Strategy {
    pub fn label(self) -> &'static str {
        match self {
            Strategy::DigestLigate => "Digest & Ligate",
            Strategy::Gibson => "Gibson",
            Strategy::GoldenGate => "Golden Gate",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Assembly {
    pub bases: Vec<u8>,
    /// Top-strand positions where inserts join the backbone / each other.
    pub junctions: Vec<usize>,
    pub strategy: Strategy,
}

/// Longest suffix of `a` equal to a prefix of `b` (Gibson homology), capped at `max`.
pub fn gibson_overlap(a: &[u8], b: &[u8], max: usize) -> usize {
    let m = max.min(a.len()).min(b.len());
    for k in (1..=m).rev() {
        if a[a.len() - k..] == b[..k] {
            return k;
        }
    }
    0
}

/// Assemble `inserts` into `vector` linearized at `cut`.
/// Gibson trims homologous overlaps between adjacent fragments; the other
/// strategies blunt-join (overhangs assumed compatible).
pub fn assemble(
    vector: &DnaSequence,
    inserts: &[Vec<u8>],
    cut: usize,
    strategy: Strategy,
) -> Assembly {
    let l = vector.len();
    let cut = cut.min(l);
    // Rotate the circular vector so the backbone starts at the cut site.
    let mut bases = vector.subseq(cut, l);
    let mut junctions = Vec::new();
    let mut last = bases.clone();
    for ins in inserts {
        if ins.is_empty() {
            continue;
        }
        match strategy {
            Strategy::Gibson => {
                let ov = gibson_overlap(&last, ins, 40);
                junctions.push(bases.len() - ov);
                bases.extend_from_slice(&ins[ov..]);
            }
            _ => {
                junctions.push(bases.len());
                bases.extend_from_slice(ins);
            }
        }
        last = ins.clone();
    }
    Assembly {
        bases,
        junctions,
        strategy,
    }
}

/// Gibson junction primers for each insert: forward = prev-fragment tail +
/// insert head; reverse = rc(insert tail + next-fragment head), using `overlap`
/// homology bases on each side.
#[derive(Debug, Clone)]
pub struct JunctionPrimers {
    pub name: String,
    pub forward: Vec<u8>,
    pub reverse: Vec<u8>,
}

pub fn gibson_primers(
    backbone: &[u8],
    inserts: &[Vec<u8>],
    overlap: usize,
) -> Vec<JunctionPrimers> {
    let ov = overlap.max(1);
    let mut frags: Vec<Vec<u8>> = vec![backbone.to_vec()];
    frags.extend(inserts.iter().cloned());
    let n = frags.len();
    let mut out = Vec::new();
    for i in 1..n {
        let ins = &frags[i];
        let prev = &frags[i - 1];
        let next = &frags[(i + 1) % n];
        let mut forward = prev[prev.len().saturating_sub(ov)..].to_vec();
        forward.extend_from_slice(&ins[..ov.min(ins.len())]);
        let mut rev_src = ins[ins.len().saturating_sub(ov)..].to_vec();
        rev_src.extend_from_slice(&next[..ov.min(next.len())]);
        let reverse = crate::sequence::reverse_complement(&rev_src);
        out.push(JunctionPrimers {
            name: format!("Insert {}", i),
            forward,
            reverse,
        });
    }
    out
}

/// Validate Golden Gate (Type IIS) overhangs: equal length, unique, no
/// reverse-complement duplicates, no palindromes. Returns a list of issues.
pub fn golden_gate_validate(overhangs: &[Vec<u8>]) -> Vec<String> {
    let mut issues = Vec::new();
    if overhangs.is_empty() {
        return issues;
    }
    let len = overhangs[0].len();
    for (i, o) in overhangs.iter().enumerate() {
        if o.len() != len {
            issues.push(format!("Overhang {} length differs", i + 1));
        }
        let rc = crate::sequence::reverse_complement(o);
        if &rc == o {
            issues.push(format!("Overhang {} is palindromic", i + 1));
        }
        for (j, p) in overhangs.iter().enumerate() {
            if j <= i {
                continue;
            }
            if p == o {
                issues.push(format!("Overhangs {} and {} are identical", i + 1, j + 1));
            } else if p == &rc {
                issues.push(format!(
                    "Overhangs {} and {} are reverse complements",
                    i + 1,
                    j + 1
                ));
            }
        }
    }
    issues
}

/// Combinatorial assembly: cartesian product of part sets, one construct per
/// combination (capped at `cap`). Each combination is assembled in order.
pub fn combinatorial(
    vector: &DnaSequence,
    part_sets: &[Vec<Vec<u8>>],
    cut: usize,
    strategy: Strategy,
    cap: usize,
) -> Vec<Assembly> {
    let mut combos: Vec<Vec<u8>> = vec![Vec::new()];
    for set in part_sets {
        let mut next = Vec::new();
        for prefix in &combos {
            for part in set {
                if next.len() >= cap {
                    break;
                }
                let mut c = prefix.clone();
                c.extend_from_slice(part);
                next.push(c);
            }
        }
        combos = next;
        if combos.is_empty() {
            break;
        }
    }
    combos
        .into_iter()
        .take(cap)
        .map(|insert| assemble(vector, &[insert], cut, strategy))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sequence::Topology;

    fn vec_seq() -> DnaSequence {
        DnaSequence::new(b"AAAACCCCGGGGTTTT".to_vec(), Topology::Circular)
    }

    #[test]
    fn digest_ligate_concatenates() {
        let v = vec_seq();
        let a = assemble(
            &v,
            &[b"AAAA".to_vec(), b"CCCC".to_vec()],
            0,
            Strategy::DigestLigate,
        );
        assert_eq!(a.bases.len(), v.len() + 8);
        assert_eq!(a.junctions, vec![v.len(), v.len() + 4]);
    }

    #[test]
    fn gibson_trims_overlap() {
        assert_eq!(gibson_overlap(b"ACGTACGT", b"ACGTAAAA", 40), 4);
        let v = DnaSequence::new(b"TTTT".to_vec(), Topology::Circular);
        let a = assemble(
            &v,
            &[b"ACGTACGT".to_vec(), b"ACGTAAAA".to_vec()],
            0,
            Strategy::Gibson,
        );
        // backbone TTTT + ACGTACGT + AAAA (overlap ACGT trimmed)
        assert_eq!(a.bases, b"TTTTACGTACGTAAAA".to_vec());
        assert_eq!(a.junctions.len(), 2);
    }

    #[test]
    fn linearization_rotates() {
        let v = vec_seq();
        let a = assemble(&v, &[], 4, Strategy::DigestLigate);
        assert_eq!(a.bases, b"CCCCGGGGTTTTAAAA".to_vec());
    }

    #[test]
    fn combinatorial_cartesian() {
        let v = vec_seq();
        let sets = vec![
            vec![b"AA".to_vec(), b"CC".to_vec()],
            vec![b"GG".to_vec(), b"TT".to_vec()],
        ];
        let out = combinatorial(&v, &sets, 0, Strategy::DigestLigate, 10);
        assert_eq!(out.len(), 4);
        assert!(out.iter().all(|a| a.bases.len() == v.len() + 4));
    }

    #[test]
    fn combinatorial_cap() {
        let v = vec_seq();
        let sets = vec![
            vec![b"AA".to_vec(), b"CC".to_vec()],
            vec![b"GG".to_vec(), b"TT".to_vec()],
        ];
        let out = combinatorial(&v, &sets, 0, Strategy::Gibson, 2);
        assert_eq!(out.len(), 2);
    }

    #[test]
    fn gibson_primers_have_overlaps() {
        let backbone = b"AAAACCCCGGGGTTTT";
        let inserts = vec![b"ACGTACGTACGT".to_vec()];
        let p = gibson_primers(backbone, &inserts, 4);
        assert_eq!(p.len(), 1);
        assert_eq!(p[0].forward.len(), 8);
        assert_eq!(p[0].reverse.len(), 8);
        assert!(p[0].forward.starts_with(b"TTTT"));
    }

    #[test]
    fn golden_gate_detects_duplicates_and_palindromes() {
        let dup = vec![b"AGTC".to_vec(), b"AGTC".to_vec()];
        assert!(!golden_gate_validate(&dup).is_empty());
        let pal = vec![b"AGCT".to_vec()]; // rc(AGCT)=AGCT
        assert!(!golden_gate_validate(&pal).is_empty());
        let ok = vec![b"AGTC".to_vec(), b"TTGA".to_vec()];
        assert!(golden_gate_validate(&ok).is_empty());
    }
}
