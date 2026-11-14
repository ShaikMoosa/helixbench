use serde::{Deserialize, Serialize};

use crate::genetic_code::{self, GeneticCode};
use crate::sequence::{reverse_complement, DnaSequence, Strand};

/// Translate complete codons; ambiguous codons become X.
pub fn translate(dna: &[u8], code: &GeneticCode) -> Vec<u8> {
    dna.as_chunks::<3>()
        .0
        .iter()
        .map(|c| genetic_code::translate_codon(c, code))
        .collect()
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Orf {
    /// Top-strand coordinate of the first base (start codon for Top, last base for Bottom).
    pub start: usize,
    /// Length in bases, including the stop codon.
    pub len: usize,
    pub strand: Strand,
    pub frame: u8,
    pub aa_len: usize,
}

/// Find start-to-stop ORFs in all six frames.
/// Circular sequences are scanned across the origin; ORFs longer than the
/// sequence itself are discarded and duplicates removed.
pub fn find_orfs(
    seq: &DnaSequence,
    code: &GeneticCode,
    min_aa: usize,
    allow_alternative_starts: bool,
) -> Vec<Orf> {
    let l = seq.len();
    if l < 3 {
        return Vec::new();
    }
    let mut out = Vec::new();
    let scan_len = if seq.is_circular() { l * 2 } else { l };
    let top = seq.extended(if seq.is_circular() { l } else { 0 });
    let bottom = reverse_complement(&top);

    for strand in [Strand::Top, Strand::Bottom] {
        let bases = if strand == Strand::Top { &top } else { &bottom };
        for frame in 0..3usize {
            let mut orf_start: Option<usize> = None;
            let mut i = frame;
            while i + 3 <= scan_len {
                let codon = &bases[i..i + 3];
                let aa = genetic_code::translate_codon(codon, code);
                if orf_start.is_none() {
                    if genetic_code::is_start_codon(codon, code, allow_alternative_starts) {
                        orf_start = Some(i);
                    }
                } else if aa == b'*' {
                    let s = orf_start.take().unwrap();
                    let len = i + 3 - s;
                    let aa_len = len / 3 - 1;
                    if aa_len >= min_aa && len <= l {
                        let (start, mapped_len) = if strand == Strand::Top {
                            (s % l, len)
                        } else {
                            ((l - (s + len) % l) % l, len)
                        };
                        out.push(Orf {
                            start,
                            len: mapped_len,
                            strand,
                            frame: frame as u8,
                            aa_len,
                        });
                    }
                }
                i += 3;
            }
        }
    }

    out.sort_by_key(|o| (o.start, o.strand as u8, o.len));
    out.dedup_by(|a, b| a.start == b.start && a.len == b.len && a.strand == b.strand);
    out
}

/// Translate one frame of a sequence for display (partial trailing codon ignored).
pub fn translate_frame(
    seq: &DnaSequence,
    strand: Strand,
    frame: usize,
    code: &GeneticCode,
) -> Vec<u8> {
    let bases = match strand {
        Strand::Top => seq.bases.clone(),
        Strand::Bottom => reverse_complement(&seq.bases),
    };
    if frame >= bases.len() {
        return Vec::new();
    }
    translate(&bases[frame..], code)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sequence::Topology;

    #[test]
    fn simple_orf() {
        // ATG AAA GGG TAA -> M K G *
        let s = DnaSequence::new(b"CCATGAAAGGGTAACC".to_vec(), Topology::Linear);
        let orfs = find_orfs(&s, genetic_code::code_by_id(1), 1, false);
        assert_eq!(orfs.len(), 1);
        assert_eq!(orfs[0].start, 2);
        assert_eq!(orfs[0].len, 12);
        assert_eq!(orfs[0].aa_len, 3);
        assert_eq!(orfs[0].strand, Strand::Top);
    }

    #[test]
    fn reverse_strand_orf() {
        // rc(ATG AAA TAA) = TTA TTT CAT embedded
        let s = DnaSequence::new(b"GCTTATTTCATGG".to_vec(), Topology::Linear);
        let orfs = find_orfs(&s, genetic_code::code_by_id(1), 1, false);
        let rev: Vec<_> = orfs.iter().filter(|o| o.strand == Strand::Bottom).collect();
        assert_eq!(rev.len(), 1);
        assert_eq!(rev[0].aa_len, 2);
    }

    #[test]
    fn circular_orf_wraps_origin() {
        // ATG split across origin: ...AT | GAAATAA...
        let s = DnaSequence::new(b"GAAATAACCAT".to_vec(), Topology::Circular);
        let orfs = find_orfs(&s, genetic_code::code_by_id(1), 1, false);
        assert!(orfs.iter().any(|o| o.start == 9 && o.len == 9));
    }

    #[test]
    fn min_length_filters() {
        let s = DnaSequence::new(b"ATGAAATAA".to_vec(), Topology::Linear);
        assert!(find_orfs(&s, genetic_code::code_by_id(1), 5, false).is_empty());
        assert_eq!(
            find_orfs(&s, genetic_code::code_by_id(1), 2, false).len(),
            1
        );
    }

    #[test]
    fn alternative_starts() {
        let s = DnaSequence::new(b"GTGAAATAA".to_vec(), Topology::Linear);
        assert!(find_orfs(&s, genetic_code::code_by_id(1), 1, false).is_empty());
        assert_eq!(find_orfs(&s, genetic_code::code_by_id(1), 1, true).len(), 1);
    }
}
