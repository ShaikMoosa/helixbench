//! Sequence alignment: Needleman–Wunsch (global), Smith–Waterman (local),
//! center-star multiple alignment with a consensus row, and circular auto-reindex.

use crate::sequence::DnaSequence;

const MATCH: i32 = 2;
const MISMATCH: i32 = -1;
const GAP: i32 = -2;

fn score(a: u8, b: u8) -> i32 {
    if a == b {
        MATCH
    } else {
        MISMATCH
    }
}

/// Global alignment. Returns (aligned_a, aligned_b, score) with `-` gaps.
pub fn needleman_wunsch(a: &[u8], b: &[u8]) -> (Vec<u8>, Vec<u8>, i32) {
    let n = a.len();
    let m = b.len();
    let w = m + 1;
    let mut dp = vec![0i32; (n + 1) * w];
    for i in 0..=n {
        dp[i * w] = GAP * i as i32;
    }
    for (j, item) in dp.iter_mut().enumerate().take(m + 1) {
        *item = GAP * j as i32;
    }
    for i in 1..=n {
        for j in 1..=m {
            let diag = dp[(i - 1) * w + (j - 1)] + score(a[i - 1], b[j - 1]);
            let up = dp[(i - 1) * w + j] + GAP;
            let left = dp[i * w + (j - 1)] + GAP;
            dp[i * w + j] = diag.max(up).max(left);
        }
    }
    // Traceback.
    let mut ra = Vec::new();
    let mut rb = Vec::new();
    let (mut i, mut j) = (n, m);
    while i > 0 || j > 0 {
        if i > 0 && j > 0 && dp[i * w + j] == dp[(i - 1) * w + (j - 1)] + score(a[i - 1], b[j - 1])
        {
            ra.push(a[i - 1]);
            rb.push(b[j - 1]);
            i -= 1;
            j -= 1;
        } else if i > 0 && dp[i * w + j] == dp[(i - 1) * w + j] + GAP {
            ra.push(a[i - 1]);
            rb.push(b'-');
            i -= 1;
        } else {
            ra.push(b'-');
            rb.push(b[j - 1]);
            j -= 1;
        }
    }
    ra.reverse();
    rb.reverse();
    (ra, rb, dp[n * w + m])
}

/// Local alignment (Smith–Waterman). Returns the best-scoring aligned segment.
pub fn smith_waterman(a: &[u8], b: &[u8]) -> (Vec<u8>, Vec<u8>, i32) {
    let n = a.len();
    let m = b.len();
    let w = m + 1;
    let mut dp = vec![0i32; (n + 1) * w];
    let mut best = 0i32;
    let (mut bi, mut bj) = (0, 0);
    for i in 1..=n {
        for j in 1..=m {
            let diag = dp[(i - 1) * w + (j - 1)] + score(a[i - 1], b[j - 1]);
            let up = dp[(i - 1) * w + j] + GAP;
            let left = dp[i * w + (j - 1)] + GAP;
            let v = diag.max(up).max(left).max(0);
            dp[i * w + j] = v;
            if v > best {
                best = v;
                bi = i;
                bj = j;
            }
        }
    }
    let mut ra = Vec::new();
    let mut rb = Vec::new();
    let (mut i, mut j) = (bi, bj);
    while i > 0 && j > 0 && dp[i * w + j] > 0 {
        if dp[i * w + j] == dp[(i - 1) * w + (j - 1)] + score(a[i - 1], b[j - 1]) {
            ra.push(a[i - 1]);
            rb.push(b[j - 1]);
            i -= 1;
            j -= 1;
        } else if dp[i * w + j] == dp[(i - 1) * w + j] + GAP {
            ra.push(a[i - 1]);
            rb.push(b'-');
            i -= 1;
        } else {
            ra.push(b'-');
            rb.push(b[j - 1]);
            j -= 1;
        }
    }
    ra.reverse();
    rb.reverse();
    (ra, rb, best)
}

/// Majority consensus over equal-length aligned rows (`-` = gap).
pub fn consensus(rows: &[Vec<u8>]) -> Vec<u8> {
    if rows.is_empty() {
        return Vec::new();
    }
    let len = rows[0].len();
    let mut out = Vec::with_capacity(len);
    for c in 0..len {
        let mut counts = [0usize; 5]; // A C G T -
        for r in rows {
            let idx = match r.get(c).copied().unwrap_or(b'-') {
                b'A' => 0,
                b'C' => 1,
                b'G' => 2,
                b'T' => 3,
                _ => 4,
            };
            counts[idx] += 1;
        }
        let mut best = 4;
        for k in 0..4 {
            if counts[k] > counts[best] {
                best = k;
            }
        }
        out.push(match best {
            0 => b'A',
            1 => b'C',
            2 => b'G',
            3 => b'T',
            _ => b'-',
        });
    }
    out
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Alignment {
    pub rows: Vec<Vec<u8>>,
    pub consensus: Vec<u8>,
    pub identity: f32,
}

/// Fraction of columns where all non-gap rows agree.
pub fn identity(rows: &[Vec<u8>]) -> f32 {
    if rows.is_empty() {
        return 0.0;
    }
    let len = rows[0].len();
    if len == 0 {
        return 0.0;
    }
    let mut agree = 0usize;
    for c in 0..len {
        let mut first: Option<u8> = None;
        let mut ok = true;
        for r in rows {
            let b = r.get(c).copied().unwrap_or(b'-');
            if b == b'-' {
                continue;
            }
            match first {
                None => first = Some(b),
                Some(f) if f != b => {
                    ok = false;
                    break;
                }
                _ => {}
            }
        }
        if ok && first.is_some() {
            agree += 1;
        }
    }
    agree as f32 / len as f32 * 100.0
}

/// Best rotation offset for a circular sequence against a reference (auto-index).
pub fn best_rotation(seq: &DnaSequence, reference: &[u8]) -> usize {
    if !seq.is_circular() || seq.is_empty() || reference.is_empty() {
        return 0;
    }
    let n = seq.len();
    let step = (n / 512).max(1);
    let mut best_off = 0;
    let mut best_score = i32::MIN;
    let mut off = 0;
    while off < n {
        let mut s = 0i32;
        let m = reference.len().min(n);
        for (k, &rb) in reference.iter().enumerate().take(m) {
            let b = seq.base_at((off + k) % n);
            s += if b == rb { 1 } else { -1 };
        }
        if s > best_score {
            best_score = s;
            best_off = off;
        }
        off += step;
    }
    best_off
}

/// Center-star multiple alignment against `rows[ref_idx]`, with optional circular
/// auto-reindex of each input. Returns aligned rows + consensus + identity.
pub fn align_many(seqs: &[DnaSequence], ref_idx: usize, reindex: bool) -> Alignment {
    if seqs.is_empty() {
        return Alignment {
            rows: Vec::new(),
            consensus: Vec::new(),
            identity: 0.0,
        };
    }
    let ref_idx = ref_idx.min(seqs.len() - 1);
    let mut oriented: Vec<Vec<u8>> = Vec::new();
    for (i, s) in seqs.iter().enumerate() {
        let mut bases = s.bases.clone();
        if reindex && i != ref_idx {
            let off = best_rotation(s, &seqs[ref_idx].bases);
            if off > 0 {
                bases.rotate_left(off);
            }
        }
        oriented.push(bases);
    }
    let reference = &oriented[ref_idx];
    // Per-sequence mapping: ref position -> base or gap, plus insertions after each ref pos.
    struct Map {
        cols: Vec<u8>,
        ins: Vec<Vec<u8>>,
    }
    let mut maps: Vec<Map> = Vec::new();
    for q in &oriented {
        let (ra, rb, _) = needleman_wunsch(q, reference);
        let mut cols = vec![b'-'; reference.len()];
        let mut ins = vec![Vec::new(); reference.len() + 1];
        let mut rp = 0usize; // reference position consumed
        for (a, b) in ra.iter().zip(rb.iter()) {
            match (*a, *b) {
                (_, b'-') => ins[rp].push(*a),
                (b'-', _) => {
                    // gap in query at ref position rp
                    cols[rp] = b'-';
                    rp += 1;
                }
                (x, _) => {
                    cols[rp] = x;
                    rp += 1;
                }
            }
        }
        maps.push(Map { cols, ins });
    }
    // Build MSA columns.
    let mut rows: Vec<Vec<u8>> = vec![Vec::new(); seqs.len()];
    for p in 0..=reference.len() {
        if p < reference.len() {
            for (i, m) in maps.iter().enumerate() {
                rows[i].push(m.cols[p]);
            }
        }
        let max_ins = maps.iter().map(|m| m.ins[p].len()).max().unwrap_or(0);
        for k in 0..max_ins {
            for (i, m) in maps.iter().enumerate() {
                rows[i].push(m.ins[p].get(k).copied().unwrap_or(b'-'));
            }
        }
    }
    let cons = consensus(&rows);
    let id = identity(&rows);
    Alignment {
        rows,
        consensus: cons,
        identity: id,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sequence::Topology;

    #[test]
    fn nw_identical() {
        let (a, b, s) = needleman_wunsch(b"ACGT", b"ACGT");
        assert_eq!(a, b"ACGT");
        assert_eq!(b, b"ACGT");
        assert_eq!(s, 8);
    }

    #[test]
    fn nw_gap() {
        let (a, b, _) = needleman_wunsch(b"ACGT", b"AGT");
        assert_eq!(a.len(), b.len());
        assert_eq!(a, b"ACGT");
        assert_eq!(b, b"A-GT");
    }

    #[test]
    fn sw_local() {
        let (a, b, s) = smith_waterman(b"TTACGTTT", b"GGACGTGG");
        assert!(s > 0);
        assert_eq!(a, b"ACGT");
        assert_eq!(b, b"ACGT");
    }

    #[test]
    fn consensus_majority() {
        let rows = vec![b"ACGT".to_vec(), b"ACGT".to_vec(), b"ATGT".to_vec()];
        assert_eq!(consensus(&rows), b"ACGT");
    }

    #[test]
    fn msa_equal_lengths_and_consensus() {
        let s1 = DnaSequence::new(b"ACGTACGT".to_vec(), Topology::Linear);
        let s2 = DnaSequence::new(b"ACGTTCGT".to_vec(), Topology::Linear);
        let al = align_many(&[s1, s2], 0, false);
        assert_eq!(al.rows[0].len(), al.rows[1].len());
        assert_eq!(al.rows[0].len(), al.consensus.len());
        assert!(al.identity > 50.0);
    }

    #[test]
    fn reindex_finds_offset() {
        let reference = b"ACGTACGTAC";
        let rotated = DnaSequence::new(b"ACGTACACGT".to_vec(), Topology::Circular);
        // rotation of reference by some offset should score best at that offset
        let off = best_rotation(&rotated, reference);
        let mut bases = rotated.bases.clone();
        bases.rotate_left(off);
        let matches = bases
            .iter()
            .zip(reference.iter())
            .filter(|(a, b)| a == b)
            .count();
        assert!(matches >= 6);
    }
}
