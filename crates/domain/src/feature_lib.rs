//! Reusable feature library + bulk auto-annotation. A library of named sequence
//! motifs (promoters, operators, RBS, terminators, tags…) can be scanned against
//! any document, on both strands, to add annotations in bulk.

use serde::{Deserialize, Serialize};

use crate::feature::{Direction, Feature};
use crate::sequence::{reverse_complement, DnaSequence, Range};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LibFeature {
    pub name: String,
    pub kind: String,
    pub motif: Vec<u8>,
    pub color: [u8; 3],
}

/// A sensible default library of common cloning / expression motifs.
pub fn default_library() -> Vec<LibFeature> {
    vec![
        LibFeature {
            name: "T7 promoter".into(),
            kind: "promoter".into(),
            motif: b"TAATACGACTCACTATAG".to_vec(),
            color: [0x57, 0xd9, 0x8a],
        },
        LibFeature {
            name: "lac operator".into(),
            kind: "regulatory".into(),
            motif: b"GGAATTGTGAGCGGATAACAATT".to_vec(),
            color: [0x57, 0xd9, 0x8a],
        },
        LibFeature {
            name: "RBS (Shine-Dalgarno)".into(),
            kind: "RBS".into(),
            motif: b"AGGAGG".to_vec(),
            color: [0x57, 0xd9, 0x8a],
        },
        LibFeature {
            name: "T7 terminator".into(),
            kind: "terminator".into(),
            motif: b"CTGCTGGAGCCGGATGAG".to_vec(),
            color: [0xf2, 0x68, 0x5f],
        },
        LibFeature {
            name: "His-tag (6x)".into(),
            kind: "tag".into(),
            motif: b"CACCACCACCACCACCAC".to_vec(),
            color: [0xe8, 0xb1, 0x3c],
        },
        LibFeature {
            name: "FLAG tag".into(),
            kind: "tag".into(),
            motif: b"GACTACAAGGACGACGATGACAAG".to_vec(),
            color: [0xe8, 0xb1, 0x3c],
        },
        LibFeature {
            name: "loxP".into(),
            kind: "recombination".into(),
            motif: b"ATAACTTCGTATAATGTATGCTATACGAAGTTAT".to_vec(),
            color: [0xf2, 0x68, 0x5f],
        },
        LibFeature {
            name: "FRT".into(),
            kind: "recombination".into(),
            motif: b"GAAGTTCCTATTCTCTAGAAAGTATAGGAACTTC".to_vec(),
            color: [0xf2, 0x68, 0x5f],
        },
    ]
}

fn find_all(seq: &[u8], motif: &[u8]) -> Vec<usize> {
    if motif.is_empty() || seq.len() < motif.len() {
        return Vec::new();
    }
    (0..=(seq.len() - motif.len()))
        .filter(|&i| &seq[i..i + motif.len()] == motif)
        .collect()
}

/// Scan `seq` (both strands, circular-aware) for every library motif and return
/// new features. Existing feature names are skipped to avoid duplicates.
pub fn auto_annotate(seq: &DnaSequence, lib: &[LibFeature], existing: &[Feature]) -> Vec<Feature> {
    let l = seq.len();
    let mut out = Vec::new();
    // For circular scanning, use an extended linear copy.
    let ext = if seq.is_circular() {
        seq.extended(l)
    } else {
        seq.bases.clone()
    };
    for lf in lib {
        if lf.motif.is_empty() {
            continue;
        }
        let rc = reverse_complement(&lf.motif);
        for (motif, dir) in [(&lf.motif, Direction::Forward), (&rc, Direction::Reverse)] {
            for pos in find_all(&ext, motif) {
                if seq.is_circular() && pos >= l {
                    continue;
                }
                let name = lf.name.clone();
                if existing.iter().any(|f| f.name == name)
                    || out.iter().any(|f: &Feature| f.name == name)
                {
                    continue;
                }
                let mut f = Feature::new(
                    0,
                    name,
                    lf.kind.clone(),
                    vec![Range::new(pos % l, motif.len())],
                );
                f.direction = dir;
                f.color = lf.color;
                out.push(f);
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sequence::Topology;

    #[test]
    fn detects_t7_and_rbs() {
        let seq = DnaSequence::new(
            b"GGGTAATACGACTCACTATAGGGGAGGAGGTTT".to_vec(),
            Topology::Linear,
        );
        let lib = default_library();
        let feats = auto_annotate(&seq, &lib, &[]);
        assert!(feats.iter().any(|f| f.name == "T7 promoter"));
        assert!(feats.iter().any(|f| f.name.contains("RBS")));
    }

    #[test]
    fn reverse_strand_detected() {
        let motif = b"AGGAGG";
        let rc = reverse_complement(motif);
        let mut body = b"AAAA".to_vec();
        body.extend_from_slice(&rc);
        body.extend_from_slice(b"TTTT");
        let seq = DnaSequence::new(body, Topology::Linear);
        let lib = vec![LibFeature {
            name: "RBS".into(),
            kind: "RBS".into(),
            motif: motif.to_vec(),
            color: [0, 0, 0],
        }];
        let feats = auto_annotate(&seq, &lib, &[]);
        assert_eq!(feats.len(), 1);
        assert_eq!(feats[0].direction, Direction::Reverse);
    }

    #[test]
    fn skips_existing_names() {
        let seq = DnaSequence::new(b"AGGAGG".to_vec(), Topology::Linear);
        let lib = vec![LibFeature {
            name: "RBS".into(),
            kind: "RBS".into(),
            motif: b"AGGAGG".to_vec(),
            color: [0, 0, 0],
        }];
        let existing = vec![Feature::new(
            1,
            "RBS".into(),
            "RBS".into(),
            vec![Range::new(0, 6)],
        )];
        assert!(auto_annotate(&seq, &lib, &existing).is_empty());
    }
}
