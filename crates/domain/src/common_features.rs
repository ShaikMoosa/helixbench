//! A library of well-known short sequence elements for auto-annotation
//! ("Detect Common Features" in the guide). Detection is exact IUPAC-aware
//! matching on both strands, including across the origin of circular DNA.

use serde::{Deserialize, Serialize};

use crate::feature::{Direction, Feature};
use crate::sequence::{DnaSequence, Range, Strand};

#[derive(Debug, Clone)]
pub struct CommonFeature {
    pub name: &'static str,
    pub kind: &'static str,
    pub sequence: &'static str,
    pub color: [u8; 3],
    pub note: &'static str,
}

pub const COMMON_FEATURES: &[CommonFeature] = &[
    CommonFeature {
        name: "T7 promoter",
        kind: "promoter",
        sequence: "TAATACGACTCACTATAGGG",
        color: [0x6f, 0xd0, 0x8c],
        note: "Bacteriophage T7 RNA polymerase promoter",
    },
    CommonFeature {
        name: "T3 promoter",
        kind: "promoter",
        sequence: "AATTAACCCTCACTAAAGGG",
        color: [0x6f, 0xd0, 0x8c],
        note: "Bacteriophage T3 RNA polymerase promoter",
    },
    CommonFeature {
        name: "SP6 promoter",
        kind: "promoter",
        sequence: "ATTTAGGTGACACTATAG",
        color: [0x6f, 0xd0, 0x8c],
        note: "Bacteriophage SP6 RNA polymerase promoter",
    },
    CommonFeature {
        name: "lac operator",
        kind: "operator",
        sequence: "GGAATTGTGAGCGGATAACAATT",
        color: [0x9e, 0xd0, 0x72],
        note: "lac repressor binding site (lacO)",
    },
    CommonFeature {
        name: "M13 fwd priming site",
        kind: "primer_bind",
        sequence: "GTAAAACGACGGCCAGT",
        color: [0xf2, 0x8b, 0x82],
        note: "M13 forward (-20) priming site",
    },
    CommonFeature {
        name: "M13 rev priming site",
        kind: "primer_bind",
        sequence: "CAGGAAACAGCTATGAC",
        color: [0xf2, 0x8b, 0x82],
        note: "M13 reverse priming site",
    },
    CommonFeature {
        name: "pUC19 MCS",
        kind: "MCS",
        sequence: "GAATTCGAGCTCGGTACCCGGGGATCCTCTAGAGTCGACCTGCAGGCATGCAAGCTT",
        color: [0x8a, 0xb4, 0xf8],
        note: "pUC19 lacZ-alpha multiple cloning site",
    },
    CommonFeature {
        name: "His6 tag",
        kind: "tag",
        sequence: "CATCATCATCATCATCAT",
        color: [0xc5, 0x8a, 0xf2],
        note: "Hexahistidine affinity tag",
    },
    CommonFeature {
        name: "FLAG tag",
        kind: "tag",
        sequence: "GACTACAAGGACGACGATGACAAG",
        color: [0xc5, 0x8a, 0xf2],
        note: "DYKDDDDK epitope tag",
    },
    CommonFeature {
        name: "HA tag",
        kind: "tag",
        sequence: "TACCCATACGATGTTCCAGATTACGCT",
        color: [0xc5, 0x8a, 0xf2],
        note: "YPYDVPDYA influenza hemagglutinin epitope tag",
    },
    CommonFeature {
        name: "c-Myc tag",
        kind: "tag",
        sequence: "GAACAAAAACTCATCTCAGAAGAGGATCTG",
        color: [0xc5, 0x8a, 0xf2],
        note: "EQKLISEEDL epitope tag",
    },
    CommonFeature {
        name: "V5 tag",
        kind: "tag",
        sequence: "GGTAAGCCTATCCCTAACCCTCTCCTCGGTCTCGATTCTACG",
        color: [0xc5, 0x8a, 0xf2],
        note: "GKPIPNPLLGLDST epitope tag",
    },
    CommonFeature {
        name: "Strep-tag II",
        kind: "tag",
        sequence: "TGGAGCCACCCGCAGTTCGAAAAA",
        color: [0xc5, 0x8a, 0xf2],
        note: "WSHPQFEK streptavidin-binding tag",
    },
    CommonFeature {
        name: "T7 tag",
        kind: "tag",
        sequence: "ATGGCTAGCATGACTGGTGGACAGCAAATGGGT",
        color: [0xc5, 0x8a, 0xf2],
        note: "MASMTGGQQMG leader peptide",
    },
    CommonFeature {
        name: "SV40 NLS",
        kind: "tag",
        sequence: "CCAAAAAAGAAGAGAAAGGTA",
        color: [0xc5, 0x8a, 0xf2],
        note: "PKKKRKV simian virus 40 nuclear localization signal",
    },
    CommonFeature {
        name: "TEV protease site",
        kind: "misc_feature",
        sequence: "GAAAACCTGTATTTTCAGGGC",
        color: [0xf2, 0xc9, 0x4c],
        note: "ENLYFQG cleavage site for TEV protease",
    },
    CommonFeature {
        name: "Thrombin site",
        kind: "misc_feature",
        sequence: "CTGGTTCCGCGTGGTAGC",
        color: [0xf2, 0xc9, 0x4c],
        note: "LVPRGS cleavage site for thrombin",
    },
    CommonFeature {
        name: "Factor Xa site",
        kind: "misc_feature",
        sequence: "ATCGAAGGTCGT",
        color: [0xf2, 0xc9, 0x4c],
        note: "IEGR cleavage site for factor Xa",
    },
    CommonFeature {
        name: "Enterokinase site",
        kind: "misc_feature",
        sequence: "GACGACGACGACAAG",
        color: [0xf2, 0xc9, 0x4c],
        note: "DDDDK cleavage site for enterokinase",
    },
    CommonFeature {
        name: "loxP",
        kind: "misc_binding",
        sequence: "ATAACTTCGTATAATGTATGCTATACGAAGTTAT",
        color: [0xf2, 0x9e, 0x6b],
        note: "Cre recombinase recognition site",
    },
    CommonFeature {
        name: "FRT",
        kind: "misc_binding",
        sequence: "GAAGTTCCTATTCTCTAGAAAGTATAGGAACTTC",
        color: [0xf2, 0x9e, 0x6b],
        note: "FLP recombinase recognition site",
    },
    CommonFeature {
        name: "Kozak sequence",
        kind: "regulatory",
        sequence: "GCCACC",
        color: [0x9e, 0xd0, 0x72],
        note: "Eukaryotic translation initiation consensus",
    },
    CommonFeature {
        name: "polyA signal",
        kind: "polyA_signal",
        sequence: "AATAAA",
        color: [0x9e, 0xd0, 0x72],
        note: "Canonical polyadenylation signal",
    },
];

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DetectedFeature {
    pub template: usize,
    pub range: Range,
    pub strand: Strand,
}

/// Find every occurrence of every library element on both strands.
pub fn detect(seq: &DnaSequence) -> Vec<DetectedFeature> {
    let mut out = Vec::new();
    for (idx, cf) in COMMON_FEATURES.iter().enumerate() {
        let enzyme = crate::enzyme::Enzyme::new(cf.name, cf.sequence, 0, 0);
        for site in enzyme.find_sites(seq) {
            out.push(DetectedFeature {
                template: idx,
                range: Range::new(site.position, cf.sequence.len()),
                strand: site.strand,
            });
        }
    }
    out.sort_by_key(|d| (d.range.start, d.template));
    out
}

impl DetectedFeature {
    pub fn to_feature(&self, id: u64) -> Feature {
        let cf = &COMMON_FEATURES[self.template];
        let mut f = Feature::new(
            id,
            cf.name.to_string(),
            cf.kind.to_string(),
            vec![self.range],
        );
        f.color = cf.color;
        f.note = cf.note.to_string();
        f.direction = match self.strand {
            Strand::Top => Direction::Forward,
            Strand::Bottom => Direction::Reverse,
        };
        f
    }
}

/// Skip duplicates: same name, same span, already present.
pub fn is_duplicate(existing: &[Feature], candidate: &DetectedFeature) -> bool {
    let cf = &COMMON_FEATURES[candidate.template];
    existing
        .iter()
        .any(|f| f.name == cf.name && f.segments.len() == 1 && f.segments[0] == candidate.range)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sequence::Topology;

    #[test]
    fn detects_t7_and_loxp() {
        let seq = DnaSequence::new(
            b"CCCTAATACGACTCACTATAGGGCCCATAACTTCGTATAATGTATGCTATACGAAGTTATGGG".to_vec(),
            Topology::Linear,
        );
        let hits = detect(&seq);
        assert!(hits
            .iter()
            .any(|h| COMMON_FEATURES[h.template].name == "T7 promoter" && h.range.start == 3));
        assert!(hits
            .iter()
            .any(|h| COMMON_FEATURES[h.template].name == "loxP"));
    }

    #[test]
    fn reverse_strand_detection() {
        let seq = DnaSequence::new(
            crate::sequence::reverse_complement(b"TAATACGACTCACTATAGGG"),
            Topology::Linear,
        );
        let hits = detect(&seq);
        let t7: Vec<_> = hits
            .iter()
            .filter(|h| COMMON_FEATURES[h.template].name == "T7 promoter")
            .collect();
        assert_eq!(t7.len(), 1);
        assert_eq!(t7[0].strand, Strand::Bottom);
    }

    #[test]
    fn tags_translate_correctly() {
        // FLAG tag must encode DYKDDDDK
        let flag = COMMON_FEATURES
            .iter()
            .find(|c| c.name == "FLAG tag")
            .unwrap();
        let aa = crate::translate::translate(
            flag.sequence.as_bytes(),
            crate::genetic_code::code_by_id(1),
        );
        assert_eq!(aa, b"DYKDDDDK".to_vec());
        let ha = COMMON_FEATURES.iter().find(|c| c.name == "HA tag").unwrap();
        assert_eq!(
            crate::translate::translate(ha.sequence.as_bytes(), crate::genetic_code::code_by_id(1)),
            b"YPYDVPDYA".to_vec()
        );
        let myc = COMMON_FEATURES
            .iter()
            .find(|c| c.name == "c-Myc tag")
            .unwrap();
        assert_eq!(
            crate::translate::translate(
                myc.sequence.as_bytes(),
                crate::genetic_code::code_by_id(1)
            ),
            b"EQKLISEEDL".to_vec()
        );
    }
}
