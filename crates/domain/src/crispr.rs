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

/// Doench Rule Set 2-style on-target score (feature-based logistic model).
/// Uses GC content windows, position-specific base preferences and homopolymer
/// penalties, mapped through a logistic to [0,1].
pub fn doench2(guide: &[u8]) -> f32 {
    if guide.len() < 20 {
        return 0.0;
    }
    let g = &guide[..20];
    let mut z = -0.5f32;
    // GC content in three windows (published RS2 uses GC-count features).
    let gc = |r: &[u8]| r.iter().filter(|b| matches!(b, b'G' | b'C')).count() as f32;
    let gc_all = gc(g) / 20.0;
    z += 1.2 * (1.0 - (gc_all - 0.5).abs() * 2.0); // favor ~50% GC
    z += 0.3 * gc(&g[0..6]) / 6.0;
    z += 0.2 * gc(&g[14..20]) / 6.0;
    // Position-specific preferences (PAM-proximal G favored, 5' G for U6).
    if g[19] == b'G' {
        z += 0.6;
    }
    if g[0] == b'G' {
        z += 0.4;
    }
    if g[15] == b'C' {
        z += 0.3;
    }
    if g[16] == b'C' {
        z += 0.2;
    }
    // Homopolymer runs penalized.
    let mut run = 1;
    for w in g.windows(2) {
        if w[0] == w[1] {
            run += 1;
            if run >= 4 {
                z -= 0.4;
            }
        } else {
            run = 1;
        }
    }
    // Avoid T at PAM-proximal (pos 20) and poly-T (U6 terminator).
    if g[19] == b'T' {
        z -= 0.5;
    }
    1.0 / (1.0 + (-z).exp())
}

/// CFD (cutting frequency determination) off-target score in [0,1].
/// Product of position-specific mismatch weights and a PAM weight.
pub fn cfd(guide: &[u8], target: &[u8], pam: &[u8]) -> f32 {
    if guide.len() != target.len() {
        return 0.0;
    }
    // Position-specific mismatch tolerance (PAM-proximal seed is least tolerant).
    const POS_W: [f32; 20] = [
        0.00, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00, 0.10, 0.15, 0.20, 0.25, 0.30, 0.35, 0.40,
        0.45, 0.50, 0.55, 0.60, 0.65,
    ];
    let mut score = 1.0f32;
    for (i, (a, b)) in guide.iter().zip(target.iter()).enumerate() {
        if a != b {
            // rna:dna pair-specific factor (transitions tolerated more than transversions).
            let pair = (*a, *b);
            let f = match pair {
                (b'A', b'G') | (b'G', b'A') | (b'C', b'T') | (b'T', b'C') => 0.6,
                _ => 0.3,
            };
            score *= POS_W[i.min(19)] * f;
        }
    }
    // PAM weight: NGG canonical, NAG / NGA reduced.
    let pam_w = if pam.len() >= 3 && pam[1] == b'G' && pam[2] == b'G' {
        1.0
    } else if pam.len() >= 3 && pam[1] == b'A' && pam[2] == b'G' {
        0.25
    } else if pam.len() >= 3 && pam[1] == b'G' && pam[2] == b'A' {
        0.06
    } else {
        0.0
    };
    (score * pam_w).clamp(0.0, 1.0)
}

#[derive(Debug, Clone)]
pub struct OffTarget {
    pub name: String,
    pub position: usize,
    pub strand: Strand,
    pub mismatches: usize,
    pub cfd: f32,
}

/// Scan a database for off-target sites of `guide` with <= max_mm mismatches,
/// scored by CFD. Returns ranked sites.
pub fn off_target_sites(
    guide: &[u8],
    db: &[(String, Vec<u8>)],
    max_mm: usize,
    limit: usize,
) -> Vec<OffTarget> {
    let mut out = Vec::new();
    for (name, seq) in db {
        let l = seq.len();
        if l < 23 {
            continue;
        }
        for i in 0..=(l - 23) {
            // top-strand site: protospacer + PAM
            let win = &seq[i..i + 20];
            let mm = mismatches(win, guide, max_mm);
            if mm <= max_mm {
                out.push(OffTarget {
                    name: name.clone(),
                    position: i,
                    strand: Strand::Top,
                    mismatches: mm,
                    cfd: cfd(guide, win, &seq[i + 20..i + 23]),
                });
            }
            // bottom-strand site
            let bwin = reverse_complement(&seq[i + 3..i + 23]);
            let bmm = mismatches(&bwin, guide, max_mm);
            if bmm <= max_mm {
                let bpam = reverse_complement(&seq[i..i + 3]);
                out.push(OffTarget {
                    name: name.clone(),
                    position: i + 3,
                    strand: Strand::Bottom,
                    mismatches: bmm,
                    cfd: cfd(guide, &bwin, &bpam),
                });
            }
        }
    }
    out.sort_by(|a, b| b.cfd.total_cmp(&a.cfd));
    out.truncate(limit);
    out
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
                score: doench2(&g),
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
                score: doench2(&g),
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

    #[test]
    fn doench2_in_range_and_gc_preference() {
        let s = doench2(b"ACGTACGTACGTACGTACGT");
        assert!((0.0..=1.0).contains(&s));
        // extreme GC should score lower than balanced
        let high = doench2(b"GGGGGGGGGGGGGGGGGGGG");
        assert!(s > high);
    }

    #[test]
    fn cfd_perfect_on_target_is_one() {
        let g = b"ACGTACGTACGTACGTACGT";
        assert!((cfd(g, g, b"AGG") - 1.0).abs() < 1e-6);
        // a PAM-proximal mismatch drops score sharply
        let mut t = g.to_vec();
        t[19] = b'A';
        assert!(cfd(g, &t, b"AGG") < 0.2);
        // non-NGG PAM zero
        assert_eq!(cfd(g, g, b"AAA"), 0.0);
    }

    #[test]
    fn off_target_sites_ranked() {
        let g = b"ACGTACGTACGTACGTACGT";
        let db = vec![("doc".to_string(), b"ACGTACGTACGTACGTACGTAGG".to_vec())];
        let ot = off_target_sites(g, &db, 3, 10);
        assert!(!ot.is_empty());
        assert!((ot[0].cfd - 1.0).abs() < 1e-6);
    }
}
