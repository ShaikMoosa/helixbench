//! CRISPR guide RNA design: PAM scanning, on-target heuristic scoring, and
//! off-target mismatch counting against the loaded sequence.

use crate::sequence::{reverse_complement, DnaSequence, Strand};

#[derive(Debug, Clone)]
pub struct Guide {
    /// Top-strand start of the 20 nt protospacer.
    pub position: usize,
    pub strand: Strand,
    pub sequence: Vec<u8>,
    pub pam: String,
    /// Simplified Doench-like on-target score in [0,1].
    pub score: f32,
    /// Number of other sites with <= 3 mismatches.
    pub off_targets: usize,
}

fn gc_frac(seq: &[u8]) -> f32 {
    if seq.is_empty() {
        return 0.0;
    }
    seq.iter().filter(|b| matches!(b, b'G' | b'C')).count() as f32 / seq.len() as f32
}

/// Simplified on-target heuristic (GC content + PAM-proximal / 5' preferences).
pub fn on_target_score(guide: &[u8]) -> f32 {
    let mut s = 0.3 + 0.4 * gc_frac(guide);
    if guide.last() == Some(&b'G') {
        s += 0.15;
    }
    if guide.first() == Some(&b'G') {
        s += 0.1;
    }
    s.clamp(0.0, 1.0)
}

fn mismatches(a: &[u8], b: &[u8], cap: usize) -> usize {
    let mut m = 0;
    for (x, y) in a.iter().zip(b.iter()) {
        if x != y {
            m += 1;
            if m > cap {
                return m;
            }
        }
    }
    m
}

/// Scan both strands for `NGG` PAMs and return up to `limit` guides ranked by score.
pub fn scan(seq: &DnaSequence, limit: usize) -> Vec<Guide> {
    let l = seq.len();
    if l < 23 {
        return Vec::new();
    }
    let mut cands: Vec<Guide> = Vec::new();
    for i in 0..=(l - 23) {
        // Top strand: protospacer then NGG.
        let pam = seq.subseq(i + 20, 3);
        if pam[1] == b'G' && pam[2] == b'G' {
            let g = seq.subseq(i, 20);
            cands.push(Guide {
                position: i,
                strand: Strand::Top,
                score: on_target_score(&g),
                sequence: g,
                pam: String::from_utf8_lossy(&pam).into_owned(),
                off_targets: 0,
            });
        }
        // Bottom strand: CCN on top => NGG on bottom; protospacer = rc(top[i+3..i+23]).
        if pam[0] == b'C' && pam[1] == b'C' {
            let g = reverse_complement(&seq.subseq(i + 3, 20));
            cands.push(Guide {
                position: i + 3,
                strand: Strand::Bottom,
                score: on_target_score(&g),
                sequence: g,
                pam: String::from_utf8_lossy(&reverse_complement(&pam)).into_owned(),
                off_targets: 0,
            });
        }
    }
    cands.sort_by(|a, b| b.score.total_cmp(&a.score));
    cands.truncate(limit);
    // Off-target count for the ranked guides (<=3 mismatches, both strands).
    for g in cands.iter_mut() {
        let mut off = 0usize;
        for i in 0..=(l - 20) {
            let win = seq.subseq(i, 20);
            if mismatches(&win, &g.sequence, 3) <= 3
                && !(i == g.position && g.strand == Strand::Top)
            {
                off += 1;
            }
            let rc = reverse_complement(&win);
            if mismatches(&rc, &g.sequence, 3) <= 3
                && !(i == g.position && g.strand == Strand::Bottom)
            {
                off += 1;
            }
        }
        g.off_targets = off.saturating_sub(1);
    }
    cands
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sequence::Topology;

    #[test]
    fn finds_ngg_guides() {
        // 20nt protospacer + AGG
        let s = DnaSequence::new(b"ACGTACGTACGTACGTACGTAGG".to_vec(), Topology::Linear);
        let g = scan(&s, 10);
        assert!(!g.is_empty());
        assert_eq!(g[0].position, 0);
        assert_eq!(g[0].pam, "AGG");
        assert_eq!(g[0].sequence.len(), 20);
    }

    #[test]
    fn score_in_range() {
        let s = on_target_score(b"ACGTACGTACGTACGTACGT");
        assert!((0.0..=1.0).contains(&s));
    }

    #[test]
    fn off_targets_counts_similar() {
        // two near-identical protospacers => off-target >=1
        let body = b"ACGTACGTACGTACGTACGTAGGTTTTACGTACGTACGTACGTACGTAGG";
        let s = DnaSequence::new(body.to_vec(), Topology::Linear);
        let g = scan(&s, 10);
        assert!(g.iter().any(|x| x.off_targets >= 1));
    }
}
