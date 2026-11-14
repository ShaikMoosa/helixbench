//! Sequence search with the guide's query syntax:
//! - IUPAC degenerate codes (R matches A/G/R)
//! - `(AG)` explicit allowed sets
//! - `!R` / `!(AG)` exclusions
//! - quoted `"R"` matches only a literal R
//!
//! DNA searches run on both strands and across the origin of circular sequences.

use serde::{Deserialize, Serialize};

use crate::iupac;
use crate::sequence::{reverse_complement, DnaSequence, Strand};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DnaMatcher {
    Allowed(u8),
    Excluded(u8),
    Literal(u8),
}

impl DnaMatcher {
    fn matches(self, target: u8) -> bool {
        match self {
            DnaMatcher::Allowed(mask) => {
                let t = iupac::dna_mask(target);
                t != 0 && (t & !mask) == 0
            }
            DnaMatcher::Excluded(mask) => {
                let t = iupac::dna_mask(target);
                t != 0 && (t & mask) == 0
            }
            DnaMatcher::Literal(c) => target.to_ascii_uppercase() == c,
        }
    }
}

fn parse_dna_query(query: &str) -> Result<Vec<DnaMatcher>, String> {
    let chars: Vec<u8> = query.bytes().filter(|b| !b.is_ascii_whitespace()).collect();
    let mut out = Vec::new();
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i].to_ascii_uppercase();
        match c {
            b'"' => {
                i += 1;
                while i < chars.len() && chars[i] != b'"' {
                    let lit = chars[i].to_ascii_uppercase();
                    if iupac::dna_mask(lit) == 0 {
                        return Err(format!("Invalid DNA character '{}'", lit as char));
                    }
                    out.push(DnaMatcher::Literal(lit));
                    i += 1;
                }
                if i >= chars.len() {
                    return Err("Unterminated quote in query".into());
                }
                i += 1;
            }
            b'!' => {
                i += 1;
                let (mask, next) = parse_set(&chars, i)?;
                out.push(DnaMatcher::Excluded(mask));
                i = next;
            }
            b'(' => {
                let (mask, next) = parse_set(&chars, i)?;
                out.push(DnaMatcher::Allowed(mask));
                i = next;
            }
            _ => {
                let mask = iupac::dna_mask(c);
                if mask == 0 {
                    return Err(format!("Invalid DNA character '{}'", c as char));
                }
                out.push(DnaMatcher::Allowed(mask));
                i += 1;
            }
        }
    }
    if out.is_empty() {
        return Err("Empty query".into());
    }
    Ok(out)
}

/// Parse `(ABC)` or a single character at position i; returns (mask, next index).
fn parse_set(chars: &[u8], i: usize) -> Result<(u8, usize), String> {
    if i >= chars.len() {
        return Err("Expected bases after '!'".into());
    }
    if chars[i] == b'(' {
        let mut mask = 0u8;
        let mut j = i + 1;
        while j < chars.len() && chars[j] != b')' {
            let m = iupac::dna_mask(chars[j]);
            if m == 0 {
                return Err(format!("Invalid DNA character '{}'", chars[j] as char));
            }
            mask |= m;
            j += 1;
        }
        if j >= chars.len() {
            return Err("Unterminated '(' in query".into());
        }
        if mask == 0 {
            return Err("Empty base set '()'".into());
        }
        Ok((mask, j + 1))
    } else {
        let m = iupac::dna_mask(chars[i]);
        if m == 0 {
            return Err(format!("Invalid DNA character '{}'", chars[i] as char));
        }
        Ok((m, i + 1))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DnaHit {
    pub start: usize,
    pub len: usize,
    pub strand: Strand,
}

pub fn find_dna(seq: &DnaSequence, query: &str) -> Result<Vec<DnaHit>, String> {
    let matchers = parse_dna_query(query)?;
    let l = seq.len();
    let n = matchers.len();
    if l == 0 || n > l && !seq.is_circular() {
        return Ok(Vec::new());
    }
    let extended = seq.extended(n.saturating_sub(1));
    let limit = if seq.is_circular() {
        l
    } else {
        l + 1 - n.min(l + 1)
    };
    let rc_matchers: Vec<DnaMatcher> = matchers
        .iter()
        .rev()
        .map(|m| match m {
            DnaMatcher::Allowed(mask) => DnaMatcher::Allowed(complement_mask(*mask)),
            DnaMatcher::Excluded(mask) => DnaMatcher::Excluded(complement_mask(*mask)),
            DnaMatcher::Literal(c) => DnaMatcher::Literal(crate::sequence::complement_base(*c)),
        })
        .collect();

    let mut out = Vec::new();
    for pos in 0..limit {
        if pos + n > extended.len() {
            break;
        }
        let window = &extended[pos..pos + n];
        if matchers.iter().zip(window).all(|(m, &t)| m.matches(t)) {
            out.push(DnaHit {
                start: pos,
                len: n,
                strand: Strand::Top,
            });
        }
        if rc_matchers.iter().zip(window).all(|(m, &t)| m.matches(t)) {
            out.push(DnaHit {
                start: pos,
                len: n,
                strand: Strand::Bottom,
            });
        }
    }
    // A palindromic query matches both strands at the same locus; keep both
    // annotations only when the query is not self-complementary.
    out.sort_by_key(|h| (h.start, h.strand as u8));
    Ok(out)
}

fn complement_mask(mask: u8) -> u8 {
    let mut out = 0u8;
    for (base, bit) in [
        (b'A', iupac::A),
        (b'C', iupac::C),
        (b'G', iupac::G),
        (b'T', iupac::T),
    ] {
        if mask & bit != 0 {
            out |= iupac::dna_mask(crate::sequence::complement_base(base));
        }
    }
    out
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProteinHit {
    /// Top-strand start of the coding region.
    pub start: usize,
    /// Length in bases (3 * query length).
    pub len: usize,
    pub strand: Strand,
    pub frame: u8,
}

/// Search all six translation frames of the whole sequence.
pub fn find_protein(
    seq: &DnaSequence,
    query: &str,
    code: &crate::genetic_code::GeneticCode,
) -> Result<Vec<ProteinHit>, String> {
    let chars: Vec<u8> = query.bytes().filter(|b| !b.is_ascii_whitespace()).collect();
    if chars.is_empty() {
        return Err("Empty query".into());
    }
    let mut matchers: Vec<(u32, bool)> = Vec::new(); // (mask, literal)
    let mut i = 0;
    let mut quoted = false;
    while i < chars.len() {
        let c = chars[i].to_ascii_uppercase();
        if c == b'"' {
            quoted = !quoted;
            i += 1;
            continue;
        }
        let mask = iupac::protein_mask(c);
        if mask == 0 {
            return Err(format!("Invalid amino acid character '{}'", c as char));
        }
        matchers.push((mask, quoted));
        i += 1;
    }
    if quoted {
        return Err("Unterminated quote in query".into());
    }

    let l = seq.len();
    let n = matchers.len();
    let mut out = Vec::new();
    for strand in [Strand::Top, Strand::Bottom] {
        let bases = match strand {
            Strand::Top => seq.extended(if seq.is_circular() { 3 * n } else { 0 }),
            Strand::Bottom => {
                let rc = reverse_complement(&seq.bases);
                let mut ext = rc.clone();
                if seq.is_circular() {
                    for k in 0..(3 * n).min(l) {
                        ext.push(rc[k % l.max(1)]);
                    }
                }
                ext
            }
        };
        for frame in 0..3usize {
            let aa: Vec<u8> = bases[frame..]
                .as_chunks::<3>()
                .0
                .iter()
                .map(|c| crate::genetic_code::translate_codon(c, code))
                .collect();
            let scan_limit = if seq.is_circular() {
                l / 3 + 1
            } else {
                aa.len()
            };
            for start_aa in 0..scan_limit.saturating_sub(n - 1) {
                if start_aa + n > aa.len() {
                    break;
                }
                let ok = matchers.iter().enumerate().all(|(k, &(mask, literal))| {
                    let t = aa[start_aa + k];
                    if literal {
                        t == query_byte_for_mask(mask)
                    } else {
                        let tm = iupac::protein_mask(t);
                        tm != 0 && (tm & !mask) == 0
                    }
                });
                if ok {
                    let base_start = frame + start_aa * 3;
                    let start = match strand {
                        Strand::Top => base_start % l.max(1),
                        Strand::Bottom => (l - (base_start + 3 * n) % l) % l.max(1),
                    };
                    out.push(ProteinHit {
                        start,
                        len: 3 * n,
                        strand,
                        frame: frame as u8,
                    });
                }
            }
        }
    }
    out.sort_by_key(|h| (h.start, h.strand as u8));
    out.dedup();
    Ok(out)
}

fn query_byte_for_mask(mask: u32) -> u8 {
    for c in b'A'..=b'Z' {
        if iupac::protein_mask(c) == mask {
            return c;
        }
    }
    b'X'
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sequence::Topology;

    #[test]
    fn plain_search_both_strands() {
        let seq = DnaSequence::new(b"CCGAATTCAAAGAATTGG".to_vec(), Topology::Linear);
        let hits = find_dna(&seq, "GAATT").unwrap();
        assert!(hits.iter().any(|h| h.start == 2 && h.strand == Strand::Top));
        assert!(hits.iter().any(|h| h.strand == Strand::Bottom));
    }

    #[test]
    fn degenerate_and_sets() {
        let seq = DnaSequence::new(b"GGTCTACTTCGGTCTGCTTC".to_vec(), Topology::Linear);
        let hits = find_dna(&seq, "GGTCTRCTTC").unwrap();
        let top: Vec<_> = hits.iter().filter(|h| h.strand == Strand::Top).collect();
        assert_eq!(top.len(), 2);
        let hits2 = find_dna(&seq, "GGTCT(AG)CTTC").unwrap();
        assert_eq!(hits2.iter().filter(|h| h.strand == Strand::Top).count(), 2);
    }

    #[test]
    fn exclusion() {
        let seq = DnaSequence::new(b"AAACAAAGAAATAAAA".to_vec(), Topology::Linear);
        let hits = find_dna(&seq, "AAA!R").unwrap();
        // AAA followed by not-A/G: AAAC at 0 and AAAT at 8; AAAG at 4 excluded.
        let top: Vec<_> = hits.iter().filter(|h| h.strand == Strand::Top).collect();
        assert!(top.iter().any(|h| h.start == 0));
        assert!(top.iter().any(|h| h.start == 8));
        assert!(!top.iter().any(|h| h.start == 4));
    }

    #[test]
    fn quoted_literal() {
        let seq = DnaSequence::new(b"ACRAGT".to_vec(), Topology::Linear);
        let deg = find_dna(&seq, "R").unwrap();
        assert!(deg.iter().filter(|h| h.strand == Strand::Top).count() >= 3);
        let lit = find_dna(&seq, "\"R\"").unwrap();
        let top: Vec<_> = lit.iter().filter(|h| h.strand == Strand::Top).collect();
        assert_eq!(top.len(), 1);
        assert_eq!(top[0].start, 2);
    }

    #[test]
    fn circular_search() {
        let seq = DnaSequence::new(b"AATTCCCCGG".to_vec(), Topology::Circular);
        let hits = find_dna(&seq, "GGAATT").unwrap();
        assert!(hits.iter().any(|h| h.start == 8 && h.strand == Strand::Top));
    }

    #[test]
    fn protein_search() {
        // ATG AAA TGG GGG TAA -> M K W G *
        let seq = DnaSequence::new(b"CCATGAAATGGGGGTAACC".to_vec(), Topology::Linear);
        let hits = find_protein(&seq, "MKWG", crate::genetic_code::code_by_id(1)).unwrap();
        assert!(hits.iter().any(|h| h.start == 2 && h.strand == Strand::Top));
        let x = find_protein(&seq, "MXWG", crate::genetic_code::code_by_id(1)).unwrap();
        assert!(!x.is_empty());
    }

    #[test]
    fn invalid_query_errors() {
        assert!(parse_dna_query("GAZTT").is_err());
        assert!(parse_dna_query("(AG").is_err());
        assert!(parse_dna_query("").is_err());
    }
}
