use serde::{Deserialize, Serialize};

use crate::genetic_code::{self, GeneticCode};
use crate::sequence::{reverse_complement, DnaSequence, Range, Strand};

pub const FEATURE_KINDS: &[&str] = &[
    "CDS",
    "gene",
    "promoter",
    "terminator",
    "RBS",
    "rep_origin",
    "operator",
    "enhancer",
    "polyA_signal",
    "signal_peptide",
    "tag",
    "primer_bind",
    "protein_bind",
    "misc_binding",
    "misc_feature",
    "misc_difference",
    "regulatory",
    "mutation",
    "source",
    "STS",
    "intron",
    "exon",
    "MCS",
    "unsure",
];

/// Default annotation color for a GenBank feature key.
pub fn kind_color(kind: &str) -> [u8; 3] {
    match kind {
        "CDS" | "exon" => [0xe8, 0x83, 0x3a],
        "gene" => [0xd2, 0x9a, 0x3a],
        "promoter" | "regulatory" => [0x3f, 0xb9, 0x50],
        "terminator" | "mutation" => [0xf8, 0x51, 0x49],
        "RBS" | "MCS" => [0x58, 0xa6, 0xff],
        "rep_origin" => [0xbc, 0x8c, 0xff],
        "operator" | "enhancer" => [0x2d, 0xd4, 0xbf],
        "polyA_signal" => [0xf7, 0x78, 0xba],
        "signal_peptide" => [0x7e, 0xe7, 0x87],
        "tag" => [0xff, 0xd8, 0x66],
        "primer_bind" => [0x79, 0xc0, 0xff],
        "protein_bind" | "misc_binding" => [0x56, 0xd3, 0x64],
        "misc_difference" => [0xff, 0xa6, 0x57],
        "source" | "STS" | "unsure" => [0x8b, 0x94, 0x9e],
        "intron" => [0x6e, 0x76, 0x81],
        _ => [0x8a, 0xb4, 0xf8],
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Direction {
    #[default]
    None,
    Forward,
    Reverse,
    Bidirectional,
}

impl Direction {
    pub fn label(self) -> &'static str {
        match self {
            Direction::None => "—",
            Direction::Forward => "→",
            Direction::Reverse => "←",
            Direction::Bidirectional => "↔",
        }
    }

    pub fn strand(self) -> Option<Strand> {
        match self {
            Direction::Forward => Some(Strand::Top),
            Direction::Reverse => Some(Strand::Bottom),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Feature {
    pub id: u64,
    pub name: String,
    pub kind: String,
    /// Segments in ascending genomic order; gaps between them are introns.
    pub segments: Vec<Range>,
    pub direction: Direction,
    pub color: [u8; 3],
    pub note: String,
    pub product: String,
    pub translated: bool,
    /// 0 means "use the document default genetic code".
    pub genetic_code: u8,
    pub first_codon_met: bool,
    pub frame_offset: usize,
    pub visible: bool,
    pub cleavage_arrows: Vec<usize>,
}

impl Feature {
    pub fn new(id: u64, name: String, kind: String, segments: Vec<Range>) -> Self {
        let translated = kind == "CDS";
        let color = kind_color(&kind);
        Self {
            id,
            name,
            kind,
            segments,
            direction: Direction::None,
            color,
            note: String::new(),
            product: String::new(),
            translated,
            genetic_code: 0,
            first_codon_met: false,
            frame_offset: 0,
            visible: true,
            cleavage_arrows: Vec::new(),
        }
    }

    /// Overall span from the first segment start to the last segment end.
    pub fn span(&self, seq_len: usize) -> Range {
        if self.segments.is_empty() {
            return Range::new(0, 0);
        }
        let first = self.segments[0];
        let last = *self.segments.last().unwrap();
        let mut end = last.end();
        if end < first.start {
            end += seq_len;
        }
        Range::new(first.start, end - first.start)
    }

    pub fn total_len(&self) -> usize {
        self.segments.iter().map(|s| s.len).sum()
    }

    /// Spliced coding sequence in transcription orientation.
    pub fn spliced(&self, seq: &DnaSequence) -> Vec<u8> {
        let mut parts: Vec<u8> = Vec::new();
        if self.direction == Direction::Reverse {
            for seg in self.segments.iter().rev() {
                parts.extend(reverse_complement(&seq.region(*seg)));
            }
        } else {
            for seg in &self.segments {
                parts.extend(seq.region(*seg));
            }
        }
        parts
    }

    pub fn effective_code<'a>(&self, default: &'a GeneticCode) -> &'a GeneticCode {
        if self.genetic_code == 0 {
            default
        } else {
            genetic_code::code_by_id(self.genetic_code)
        }
    }

    pub fn translation(&self, seq: &DnaSequence, default_code: &GeneticCode) -> Vec<u8> {
        if !self.translated {
            return Vec::new();
        }
        let spliced = self.spliced(seq);
        let offset = self.frame_offset.min(2);
        if offset >= spliced.len() {
            return Vec::new();
        }
        let mut aa =
            crate::translate::translate(&spliced[offset..], self.effective_code(default_code));
        if self.first_codon_met && !aa.is_empty() {
            aa[0] = b'M';
        }
        aa
    }

    pub fn has_internal_stop(&self, seq: &DnaSequence, default_code: &GeneticCode) -> bool {
        let aa = self.translation(seq, default_code);
        aa.iter()
            .take(aa.len().saturating_sub(1))
            .any(|&a| a == b'*')
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sequence::Topology;

    #[test]
    fn reverse_feature_translation() {
        // Top: TTATTT CAT... rc of CATATG... let's build: CDS on bottom = ATG AAA TAA
        // top strand = rc(ATGAAATAA) = TTATTTCAT
        let seq = DnaSequence::new(b"GCTTATTTCATGG".to_vec(), Topology::Linear);
        let mut f = Feature::new(1, "rev".into(), "CDS".into(), vec![Range::new(2, 9)]);
        f.direction = Direction::Reverse;
        let aa = f.translation(&seq, genetic_code::code_by_id(1));
        assert_eq!(aa, b"MK*".to_vec());
    }

    #[test]
    fn spliced_intron() {
        // exon1 ATGAAA (6) + intron GGGGG + exon2 GGGTAA (6) -> MKG*
        let seq = DnaSequence::new(b"ATGAAAGGGGGGGGTAA".to_vec(), Topology::Linear);
        let mut f = Feature::new(
            1,
            "spliced".into(),
            "CDS".into(),
            vec![Range::new(0, 6), Range::new(11, 6)],
        );
        f.direction = Direction::Forward;
        assert_eq!(f.spliced(&seq), b"ATGAAAGGGTAA".to_vec());
        assert_eq!(
            f.translation(&seq, genetic_code::code_by_id(1)),
            b"MKG*".to_vec()
        );
        assert_eq!(f.span(17), Range::new(0, 17));
    }

    #[test]
    fn first_codon_met() {
        let seq = DnaSequence::new(b"GTGAAATAA".to_vec(), Topology::Linear);
        let mut f = Feature::new(1, "gtg".into(), "CDS".into(), vec![Range::new(0, 9)]);
        f.direction = Direction::Forward;
        f.first_codon_met = true;
        assert_eq!(
            f.translation(&seq, genetic_code::code_by_id(1)),
            b"MK*".to_vec()
        );
    }
}
