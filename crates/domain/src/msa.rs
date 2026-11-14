//! Progressive multiple sequence alignment (Feng–Doolittle): pairwise
//! Needleman–Wunsch distances → UPGMA guide tree → profile–profile alignment.
//! A genuine MSA (not center-star), suitable for consensus and bulk alignment.

use crate::align::needleman_wunsch;

const MATCH: i32 = 2;
const MISMATCH: i32 = -1;
const GAP: i32 = -2;

fn sub(a: u8, b: u8) -> i32 {
    if a == b'-' || b == b'-' {
        if a == b'-' && b == b'-' {
            0
        } else {
            GAP
        }
    } else if a == b {
        MATCH
    } else {
        MISMATCH
    }
}

/// Fraction of identical non-gap columns between two aligned rows.
fn identity(a: &[u8], b: &[u8]) -> f32 {
    let mut same = 0;
    let mut n = 0;
    for (x, y) in a.iter().zip(b.iter()) {
        if *x == b'-' || *y == b'-' {
            continue;
        }
        n += 1;
        if x == y {
            same += 1;
        }
    }
    if n == 0 {
        0.0
    } else {
        same as f32 / n as f32
    }
}

/// Align two profiles (each a set of equal-length rows) by sum-of-pairs NW.
fn profile_nw(a: &[Vec<u8>], b: &[Vec<u8>]) -> (Vec<Vec<u8>>, Vec<Vec<u8>>) {
    let n = a[0].len();
    let m = b[0].len();
    let w = m + 1;
    // Column-column score = mean over row pairs.
    let mut col = vec![0i32; n * m];
    for i in 0..n {
        for j in 0..m {
            let mut s = 0i32;
            for ra in a {
                for rb in b {
                    s += sub(ra[i], rb[j]);
                }
            }
            col[i * m + j] = s / (a.len() * b.len()) as i32;
        }
    }
    let mut dp = vec![0i32; (n + 1) * w];
    for i in 0..=n {
        dp[i * w] = GAP * i as i32;
    }
    for (j, item) in dp.iter_mut().enumerate().take(m + 1) {
        *item = GAP * j as i32;
    }
    for i in 1..=n {
        for j in 1..=m {
            let diag = dp[(i - 1) * w + (j - 1)] + col[(i - 1) * m + (j - 1)];
            let up = dp[(i - 1) * w + j] + GAP;
            let left = dp[i * w + (j - 1)] + GAP;
            dp[i * w + j] = diag.max(up).max(left);
        }
    }
    // Traceback, building gap-inserted profiles.
    let mut ia = Vec::new();
    let mut ib = Vec::new();
    let (mut i, mut j) = (n, m);
    while i > 0 || j > 0 {
        if i > 0 && j > 0 && dp[i * w + j] == dp[(i - 1) * w + (j - 1)] + col[(i - 1) * m + (j - 1)]
        {
            ia.push(i - 1);
            ib.push(j - 1);
            i -= 1;
            j -= 1;
        } else if i > 0 && dp[i * w + j] == dp[(i - 1) * w + j] + GAP {
            ia.push(i - 1);
            ib.push(usize::MAX); // gap in b
            i -= 1;
        } else {
            ia.push(usize::MAX);
            ib.push(j - 1);
            j -= 1;
        }
    }
    ia.reverse();
    ib.reverse();
    let mut out_a = vec![Vec::new(); a.len()];
    let mut out_b = vec![Vec::new(); b.len()];
    for (ca, cb) in ia.iter().zip(ib.iter()) {
        for (r, row) in a.iter().enumerate() {
            out_a[r].push(if *ca == usize::MAX { b'-' } else { row[*ca] });
        }
        for (r, row) in b.iter().enumerate() {
            out_b[r].push(if *cb == usize::MAX { b'-' } else { row[*cb] });
        }
    }
    (out_a, out_b)
}

/// UPGMA guide tree; returns merges as (cluster_a, cluster_b, new_cluster).
fn upgma(dist: &[Vec<f32>]) -> Vec<(usize, usize, usize)> {
    let n = dist.len();
    let cap = 2 * n;
    let mut d = vec![vec![f32::MAX; cap]; cap];
    for i in 0..n {
        for j in 0..n {
            d[i][j] = dist[i][j];
        }
    }
    let mut size = vec![1usize; cap];
    let mut active: Vec<usize> = (0..n).collect();
    let mut merges = Vec::new();
    let mut next = n;
    while active.len() > 1 {
        let (mut bi, mut bj, mut best) = (active[0], active[1], f32::MAX);
        for &i in &active {
            for &j in &active {
                if i < j && d[i][j] < best {
                    best = d[i][j];
                    bi = i;
                    bj = j;
                }
            }
        }
        let na = size[bi];
        let nb = size[bj];
        for &k in &active {
            if k == bi || k == bj {
                continue;
            }
            let v = (d[bi][k] * na as f32 + d[bj][k] * nb as f32) / (na + nb) as f32;
            d[next][k] = v;
            d[k][next] = v;
        }
        size[next] = na + nb;
        merges.push((bi, bj, next));
        active.retain(|&x| x != bi && x != bj);
        active.push(next);
        next += 1;
    }
    merges
}

/// Progressive MSA of `seqs`; returns aligned rows (all equal length).
pub fn progressive(seqs: &[Vec<u8>]) -> Vec<Vec<u8>> {
    let n = seqs.len();
    if n == 0 {
        return Vec::new();
    }
    if n == 1 {
        return vec![seqs[0].clone()];
    }
    let mut dist = vec![vec![0f32; n]; n];
    for i in 0..n {
        for j in (i + 1)..n {
            let (a, b, _) = needleman_wunsch(&seqs[i], &seqs[j]);
            let d = 1.0 - identity(&a, &b);
            dist[i][j] = d;
            dist[j][i] = d;
        }
    }
    let merges = upgma(&dist);
    let mut profiles: Vec<Option<Vec<Vec<u8>>>> = Vec::with_capacity(2 * n);
    for s in seqs {
        profiles.push(Some(vec![s.clone()]));
    }
    for (a, b, new) in merges {
        let pa = profiles[a].take().unwrap();
        let pb = profiles[b].take().unwrap();
        let (ma, mb) = profile_nw(&pa, &pb);
        let mut merged = ma;
        merged.extend(mb);
        while profiles.len() <= new {
            profiles.push(None);
        }
        profiles[new] = Some(merged);
    }
    profiles.pop().flatten().unwrap_or_default()
}

/// Consensus (majority, gaps excluded) over aligned rows.
pub fn consensus(rows: &[Vec<u8>]) -> Vec<u8> {
    crate::align::consensus(rows)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn progressive_equal_lengths() {
        let seqs = vec![
            b"ACGTACGT".to_vec(),
            b"ACGTTCGT".to_vec(),
            b"ACGTACGA".to_vec(),
        ];
        let al = progressive(&seqs);
        assert_eq!(al.len(), 3);
        let l = al[0].len();
        assert!(al.iter().all(|r| r.len() == l));
        assert!(l >= 8);
    }

    #[test]
    fn progressive_handles_indels() {
        let seqs = vec![
            b"AAAACCCCGGGG".to_vec(),
            b"AAAAGGGG".to_vec(),
            b"AAAACCCCGGGGTTTT".to_vec(),
        ];
        let al = progressive(&seqs);
        let l = al[0].len();
        assert!(al.iter().all(|r| r.len() == l));
        // consensus should retain the shared AAAA...GGGG core
        let c = consensus(&al);
        assert!(c.starts_with(b"AAAA"));
    }

    #[test]
    fn single_and_empty() {
        assert!(progressive(&[]).is_empty());
        assert_eq!(progressive(&[b"ACGT".to_vec()]), vec![b"ACGT".to_vec()]);
    }
}
