use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Topology {
    Linear,
    #[default]
    Circular,
}

impl Topology {
    pub fn label(self) -> &'static str {
        match self {
            Topology::Linear => "linear",
            Topology::Circular => "circular",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum Strand {
    Top,
    Bottom,
}

impl Strand {
    pub fn other(self) -> Strand {
        match self {
            Strand::Top => Strand::Bottom,
            Strand::Bottom => Strand::Top,
        }
    }
}

/// A half-open region on the top-strand coordinate system.
/// For circular sequences `start + len` may exceed the sequence length (wrap).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct Range {
    pub start: usize,
    pub len: usize,
}

impl Range {
    pub fn new(start: usize, len: usize) -> Self {
        Self { start, len }
    }

    pub fn end(self) -> usize {
        self.start + self.len
    }

    pub fn is_empty(self) -> bool {
        self.len == 0
    }

    /// Normalize start into [0, seq_len) for circular sequences.
    pub fn normalized(self, seq_len: usize) -> Self {
        if seq_len == 0 {
            return self;
        }
        Self {
            start: self.start % seq_len,
            len: self.len.min(seq_len),
        }
    }

    pub fn contains(self, pos: usize, seq_len: usize, circular: bool) -> bool {
        if self.len == 0 {
            return false;
        }
        if circular && seq_len > 0 {
            let rel = (pos + seq_len - self.start % seq_len) % seq_len;
            rel < self.len
        } else {
            pos >= self.start && pos < self.end()
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DnaSequence {
    pub bases: Vec<u8>,
    pub topology: Topology,
}

impl DnaSequence {
    pub fn new(bases: Vec<u8>, topology: Topology) -> Self {
        Self { bases, topology }
    }

    pub fn len(&self) -> usize {
        self.bases.len()
    }

    pub fn is_empty(&self) -> bool {
        self.bases.is_empty()
    }

    pub fn is_circular(&self) -> bool {
        self.topology == Topology::Circular
    }

    pub fn base_at(&self, pos: usize) -> u8 {
        let l = self.bases.len();
        if l == 0 {
            return b'N';
        }
        if pos < l {
            self.bases[pos]
        } else if self.is_circular() {
            self.bases[pos % l]
        } else {
            b'N'
        }
    }

    /// Extract a region, wrapping around the origin for circular sequences.
    pub fn region(&self, r: Range) -> Vec<u8> {
        let l = self.bases.len();
        if l == 0 || r.len == 0 {
            return Vec::new();
        }
        let len = if self.is_circular() {
            r.len.min(l)
        } else {
            r.len.min(l.saturating_sub(r.start))
        };
        let mut out = Vec::with_capacity(len);
        for i in 0..len {
            out.push(self.base_at(r.start + i));
        }
        out
    }

    pub fn subseq(&self, start: usize, len: usize) -> Vec<u8> {
        self.region(Range::new(start, len))
    }

    /// Bases extended past the origin so circular motifs can be matched linearly.
    pub fn extended(&self, extra: usize) -> Vec<u8> {
        let mut out = self.bases.clone();
        if self.is_circular() {
            let l = self.bases.len();
            for i in 0..extra.min(l) {
                out.push(self.bases[i % l.max(1)]);
            }
        }
        out
    }

    pub fn gc_content(&self) -> f32 {
        if self.bases.is_empty() {
            return 0.0;
        }
        let gc = self
            .bases
            .iter()
            .filter(|&&b| b == b'G' || b == b'C' || b == b'S')
            .count();
        gc as f32 / self.bases.len() as f32 * 100.0
    }

    pub fn rotate(&mut self, offset: usize) {
        let l = self.bases.len();
        if l == 0 {
            return;
        }
        let off = offset % l;
        self.bases.rotate_left(off);
    }
}

/// Uppercase, map U to T, keep only letters (IUPAC ambiguity codes allowed).
pub fn clean_sequence(input: &str) -> Vec<u8> {
    input
        .bytes()
        .filter_map(|b| {
            let c = b.to_ascii_uppercase();
            if c.is_ascii_alphabetic() {
                Some(if c == b'U' { b'T' } else { c })
            } else {
                None
            }
        })
        .collect()
}

pub fn complement_base(b: u8) -> u8 {
    match b {
        b'A' => b'T',
        b'T' => b'A',
        b'C' => b'G',
        b'G' => b'C',
        b'R' => b'Y',
        b'Y' => b'R',
        b'S' => b'S',
        b'W' => b'W',
        b'K' => b'M',
        b'M' => b'K',
        b'B' => b'V',
        b'V' => b'B',
        b'D' => b'H',
        b'H' => b'D',
        b'N' => b'N',
        b'U' => b'A',
        _ => b'N',
    }
}

pub fn complement(seq: &[u8]) -> Vec<u8> {
    seq.iter().map(|&b| complement_base(b)).collect()
}

pub fn reverse_complement(seq: &[u8]) -> Vec<u8> {
    seq.iter().rev().map(|&b| complement_base(b)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn circular_region_wraps() {
        let s = DnaSequence::new(b"AAACCCGGGTTT".to_vec(), Topology::Circular);
        assert_eq!(s.region(Range::new(10, 4)), b"TTAA".to_vec());
        assert_eq!(s.base_at(12), b'A');
    }

    #[test]
    fn linear_region_clamps() {
        let s = DnaSequence::new(b"ACGT".to_vec(), Topology::Linear);
        assert_eq!(s.region(Range::new(2, 10)), b"GT".to_vec());
        assert_eq!(s.base_at(9), b'N');
    }

    #[test]
    fn revcomp_iupac() {
        assert_eq!(reverse_complement(b"ARYN"), b"NRYT".to_vec());
        assert_eq!(clean_sequence("acgu n-x!"), b"ACGTNX".to_vec());
    }

    #[test]
    fn range_contains_circular() {
        let r = Range::new(10, 4);
        assert!(r.contains(11, 12, true));
        assert!(r.contains(0, 12, true));
        assert!(!r.contains(5, 12, true));
    }
}
