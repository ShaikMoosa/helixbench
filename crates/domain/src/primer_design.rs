//! Parameter-driven primer design flanking a target region.

use crate::primer::{melting_temperature, HybridizationParams};
use crate::sequence::{clean_sequence, reverse_complement, DnaSequence, Strand};

#[derive(Debug, Clone)]
pub struct DesignParams {
    pub min_len: usize,
    pub max_len: usize,
    pub target_tm: f32,
    pub tm_tol: f32,
    pub min_gc: f32,
    pub max_gc: f32,
    /// Optional 5' overhang (e.g. restriction site) prepended to the annealing region.
    pub overhang: String,
}

impl Default for DesignParams {
    fn default() -> Self {
        Self {
            min_len: 18,
            max_len: 25,
            target_tm: 60.0,
            tm_tol: 6.0,
            min_gc: 40.0,
            max_gc: 60.0,
            overhang: String::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Candidate {
    /// Full primer including any 5' overhang.
    pub sequence: Vec<u8>,
    /// Annealing-region start on the top strand.
    pub start: usize,
    pub strand: Strand,
    pub tm: f32,
    pub gc: f32,
    pub len: usize,
    pub clamp: bool,
    pub score: f32,
}

fn gc_percent(seq: &[u8]) -> f32 {
    if seq.is_empty() {
        return 0.0;
    }
    let gc = seq.iter().filter(|b| matches!(b, b'G' | b'C')).count();
    gc as f32 / seq.len() as f32 * 100.0
}

fn gc_clamp(seq: &[u8]) -> bool {
    let tail = &seq[seq.len().saturating_sub(5)..];
    tail.iter().filter(|b| matches!(b, b'G' | b'C')).count() >= 2
}

fn score_candidate(tm: f32, gc: f32, clamp: bool, p: &DesignParams) -> f32 {
    let mut s = -(tm - p.target_tm).abs();
    if gc < p.min_gc {
        s -= (p.min_gc - gc) * 0.5;
    }
    if gc > p.max_gc {
        s -= (gc - p.max_gc) * 0.5;
    }
    if clamp {
        s += 2.0;
    }
    s
}

fn push_candidate(
    out: &mut Vec<Candidate>,
    anneal: Vec<u8>,
    start: usize,
    strand: Strand,
    p: &DesignParams,
    hyb: &HybridizationParams,
) {
    let Some(tm) = melting_temperature(&anneal, hyb) else {
        return;
    };
    let gc = gc_percent(&anneal);
    if (tm - p.target_tm).abs() > p.tm_tol || gc < p.min_gc || gc > p.max_gc {
        return;
    }
    let clamp = gc_clamp(&anneal);
    let score = score_candidate(tm, gc, clamp, p);
    let mut sequence = clean_sequence(&p.overhang);
    sequence.extend_from_slice(&anneal);
    out.push(Candidate {
        sequence,
        start,
        strand,
        tm,
        gc,
        len: anneal.len(),
        clamp,
        score,
    });
}

/// Design forward and reverse primer candidates flanking `[target_start, target_start+target_len)`.
/// Returns (forward, reverse) sorted best-first.
pub fn design(
    seq: &DnaSequence,
    target_start: usize,
    target_len: usize,
    p: &DesignParams,
) -> (Vec<Candidate>, Vec<Candidate>) {
    let l = seq.len();
    let mut fwd = Vec::new();
    let mut rev = Vec::new();
    if l == 0 || target_len == 0 {
        return (fwd, rev);
    }
    let hyb = HybridizationParams::default();
    let target_end = target_start + target_len;
    for len in p.min_len..=p.max_len {
        for off in 0..=3usize {
            // Forward: top strand starting at/just before target start.
            let fs = target_start.saturating_sub(off);
            if !seq.is_circular() && fs + len > l {
                continue;
            }
            let anneal = seq.subseq(fs, len);
            push_candidate(&mut fwd, anneal, fs, Strand::Top, p, &hyb);
            // Reverse: bottom strand ending at/just after target end.
            let re = target_end + off;
            if !seq.is_circular() && re > l {
                continue;
            }
            let rs = re.saturating_sub(len);
            let window = seq.subseq(rs, len);
            let anneal = reverse_complement(&window);
            push_candidate(&mut rev, anneal, rs, Strand::Bottom, p, &hyb);
        }
    }
    fwd.sort_by(|a, b| b.score.total_cmp(&a.score));
    rev.sort_by(|a, b| b.score.total_cmp(&a.score));
    fwd.truncate(6);
    rev.truncate(6);
    (fwd, rev)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sequence::Topology;

    #[test]
    fn designs_flanking_primers() {
        let body = "ATGCGTAAAGGCTTAGCTAGCTAGGATCCGAATTCGGTACCTAGCTAGCATGCGTAAAGGCTTAGCTA";
        let seq = DnaSequence::new(body.bytes().collect(), Topology::Linear);
        let (f, r) = design(&seq, 10, 30, &DesignParams::default());
        assert!(!f.is_empty());
        assert!(!r.is_empty());
        assert!(f[0].strand == Strand::Top);
        assert!(r[0].strand == Strand::Bottom);
        assert!(f[0].tm > 0.0);
        // forward anneals at/before target start; reverse ends at/after target end
        assert!(f[0].start <= 10);
        assert!(r[0].start + r[0].len >= 40);
    }

    #[test]
    fn overhang_prepended() {
        let body = "ATGCGTAAAGGCTTAGCTAGCTAGGATCCGAATTCGGTACCTAGCTAGCATGCGTAAAGGCTTAGCTA";
        let seq = DnaSequence::new(body.bytes().collect(), Topology::Linear);
        let p = DesignParams {
            overhang: "GAATTC".into(),
            ..Default::default()
        };
        let (f, _) = design(&seq, 10, 30, &p);
        assert!(!f.is_empty());
        assert!(f[0].sequence.starts_with(b"GAATTC"));
    }
}
