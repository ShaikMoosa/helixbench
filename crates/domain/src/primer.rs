use serde::{Deserialize, Serialize};

use crate::iupac;
use crate::sequence::{clean_sequence, reverse_complement, DnaSequence, Strand};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Primer {
    pub id: u64,
    pub name: String,
    pub sequence: Vec<u8>,
    pub description: String,
    pub color: [u8; 3],
    pub phosphorylated: bool,
    pub visible: bool,
}

impl Primer {
    pub fn new(id: u64, name: String, sequence: &str) -> Self {
        Self {
            id,
            name,
            sequence: clean_sequence(sequence),
            description: String::new(),
            color: [0xf2, 0x8b, 0x82],
            phosphorylated: false,
            visible: true,
        }
    }

    pub fn len(&self) -> usize {
        self.sequence.len()
    }

    pub fn is_empty(&self) -> bool {
        self.sequence.is_empty()
    }

    pub fn gc_percent(&self) -> f32 {
        if self.sequence.is_empty() {
            return 0.0;
        }
        let gc = self
            .sequence
            .iter()
            .filter(|&&b| b == b'G' || b == b'C')
            .count();
        gc as f32 / self.sequence.len() as f32 * 100.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct HybridizationParams {
    /// Consecutive matching bases required at the primer 3' end.
    pub min_3prime_match: usize,
    /// Minimum melting temperature of the hybridized region (°C).
    pub min_tm: f32,
    /// Monovalent salt concentration (mM).
    pub salt_mm: f32,
    /// Primer concentration (nM).
    pub primer_nm: f32,
}

impl Default for HybridizationParams {
    fn default() -> Self {
        Self {
            min_3prime_match: 12,
            min_tm: 45.0,
            salt_mm: 50.0,
            primer_nm: 250.0,
        }
    }
}

/// SantaLucia (1998) unified nearest-neighbor parameters: (dH kcal/mol, dS cal/mol·K).
fn nn_params(pair: [u8; 2]) -> Option<(f64, f64)> {
    Some(match pair {
        [b'A', b'A'] | [b'T', b'T'] => (-7.9, -22.2),
        [b'A', b'T'] => (-7.2, -20.4),
        [b'T', b'A'] => (-7.2, -21.3),
        [b'C', b'A'] | [b'T', b'G'] => (-8.5, -22.7),
        [b'G', b'T'] | [b'A', b'C'] => (-8.4, -22.4),
        [b'C', b'T'] | [b'A', b'G'] => (-7.8, -21.0),
        [b'G', b'A'] | [b'T', b'C'] => (-8.2, -22.2),
        [b'C', b'G'] => (-10.6, -27.2),
        [b'G', b'C'] => (-9.8, -24.4),
        [b'G', b'G'] | [b'C', b'C'] => (-8.0, -19.9),
        _ => return None,
    })
}

/// Nearest-neighbor melting temperature with salt correction.
/// Returns None for sequences shorter than 2 concrete bases.
pub fn melting_temperature(seq: &[u8], params: &HybridizationParams) -> Option<f32> {
    if seq.len() < 2 {
        return None;
    }
    let mut dh = 0.0f64;
    let mut ds = 0.0f64;
    for pair in seq.windows(2) {
        let (h, s) = nn_params([pair[0], pair[1]])?;
        dh += h;
        ds += s;
    }
    // Initiation terms per terminal base.
    for &end in [seq.first().unwrap(), seq.last().unwrap()] {
        if end == b'G' || end == b'C' {
            dh += 0.1;
            ds += -2.8;
        } else {
            dh += 2.3;
            ds += 4.1;
        }
    }
    // SantaLucia 1998 monovalent salt correction.
    let na = (params.salt_mm.max(0.1) as f64) / 1000.0;
    ds += 0.368 * (seq.len() as f64 - 1.0) * na.ln();

    let ct = (params.primer_nm.max(0.1) as f64) * 1e-9;
    let r = 1.987f64;
    let tm = dh * 1000.0 / (ds + r * (ct / 4.0).ln()) - 273.15;
    Some(tm as f32)
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BindingSite {
    /// Top-strand start of the hybridized region.
    pub start: usize,
    pub len: usize,
    /// Top: primer sequence equals the top strand, extends rightward.
    /// Bottom: primer equals the bottom strand, extends leftward.
    pub strand: Strand,
    pub tm: f32,
    pub matched_3prime: usize,
}

/// Find hybridization sites: at least `min_3prime_match` consecutive matches
/// at the 3' end and a hybridized-region Tm of at least `min_tm`.
/// The 5' tail may mismatch (extensions / mutations).
pub fn binding_sites(
    primer: &Primer,
    seq: &DnaSequence,
    params: &HybridizationParams,
) -> Vec<BindingSite> {
    let l = seq.len();
    let n = primer.sequence.len();
    if l == 0 || n == 0 {
        return Vec::new();
    }
    let mut out = Vec::new();
    let rc = reverse_complement(&primer.sequence);
    let min_match = params.min_3prime_match.min(n);

    // Top strand: primer sequence equals the top strand; its 3' end is the
    // rightmost aligned base and extension proceeds rightward.
    for p in 0..l {
        let mut matched = 0usize;
        for i in (0..n).rev() {
            let back = n - 1 - i;
            if !seq.is_circular() && p < back {
                break;
            }
            let top_pos = if p >= back { p - back } else { p + l - back };
            if iupac::dna_query_matches(primer.sequence[i], seq.base_at(top_pos)) {
                matched += 1;
            } else {
                break;
            }
        }
        if matched < min_match {
            continue;
        }
        let Some(tm) = melting_temperature(&primer.sequence[n - matched..], params) else {
            continue;
        };
        if tm < params.min_tm {
            continue;
        }
        let start = if p >= matched - 1 {
            p - (matched - 1)
        } else {
            p + l - (matched - 1)
        };
        out.push(BindingSite {
            start,
            len: matched,
            strand: Strand::Top,
            tm,
            matched_3prime: matched,
        });
    }

    // Bottom strand: rc(primer) equals the top strand; the primer's 3' end is
    // the LEFTmost aligned base (query[0]) and extension proceeds leftward.
    for p in 0..l {
        let mut matched = 0usize;
        for (i, &rc_base) in rc.iter().enumerate() {
            let top_pos = p + i;
            if !seq.is_circular() && top_pos >= l {
                break;
            }
            if iupac::dna_query_matches(rc_base, seq.base_at(top_pos)) {
                matched += 1;
            } else {
                break;
            }
        }
        if matched < min_match {
            continue;
        }
        let Some(tm) = melting_temperature(&primer.sequence[n - matched..], params) else {
            continue;
        };
        if tm < params.min_tm {
            continue;
        }
        out.push(BindingSite {
            start: p,
            len: matched,
            strand: Strand::Bottom,
            tm,
            matched_3prime: matched,
        });
    }

    out.sort_by_key(|s| (s.start, s.strand as u8));
    out.dedup_by(|a, b| a.start == b.start && a.len == b.len && a.strand == b.strand);
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sequence::Topology;

    #[test]
    fn tm_reasonable_range() {
        let p = Primer::new(1, "test".into(), "GTAAAACGACGGCCAGT");
        let tm = melting_temperature(&p.sequence, &HybridizationParams::default()).unwrap();
        assert!(tm > 45.0 && tm < 70.0, "tm was {tm}");
    }

    #[test]
    fn exact_binding_both_strands() {
        let seq = DnaSequence::new(b"CCCCGTAAAACGACGGCCAGTTTTT".to_vec(), Topology::Linear);
        let p = Primer::new(1, "M13F".into(), "GTAAAACGACGGCCAGT");
        let sites = binding_sites(&p, &seq, &HybridizationParams::default());
        assert_eq!(sites.len(), 1);
        assert_eq!(sites[0].start, 4);
        assert_eq!(sites[0].strand, Strand::Top);
        assert_eq!(sites[0].len, 17);
    }

    #[test]
    fn reverse_primer_binds_bottom() {
        let seq = DnaSequence::new(b"CCCCGTAAAACGACGGCCAGTTTTT".to_vec(), Topology::Linear);
        let p = Primer::new(1, "rev".into(), "ACTGGCCGTCGTTTTAC");
        let sites = binding_sites(&p, &seq, &HybridizationParams::default());
        assert_eq!(sites.len(), 1);
        assert_eq!(sites[0].strand, Strand::Bottom);
        assert_eq!(sites[0].start, 4);
    }

    #[test]
    fn five_prime_extension_tolerated() {
        // Non-T flank so the poly-A extension cannot hybridize accidentally.
        let seq = DnaSequence::new(b"CCCCGTAAAACGACGGCCAGGCGCG".to_vec(), Topology::Linear);
        // 8-base non-matching 5' tail + 16-base match
        let p = Primer::new(1, "ext".into(), "AAAAAAAACTGGCCGTCGTTTTAC");
        let params = HybridizationParams::default();
        let sites = binding_sites(&p, &seq, &params);
        assert_eq!(sites.len(), 1);
        assert_eq!(sites[0].matched_3prime, 16);
        assert_eq!(sites[0].len, 16);
    }

    #[test]
    fn circular_binding_wraps() {
        let seq = DnaSequence::new(b"CGACGGCCAGTTTTTCCCCGTAAAA".to_vec(), Topology::Circular);
        let p = Primer::new(1, "M13F".into(), "GTAAAACGACGGCCAGT");
        let sites = binding_sites(&p, &seq, &HybridizationParams::default());
        assert_eq!(sites.len(), 1);
        assert_eq!(sites[0].start, 19);
        assert_eq!(sites[0].len, 17);
    }
}
