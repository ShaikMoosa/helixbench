//! Local BLAST-like similarity search: exact k-mer seeds + ungapped dropoff
//! extension against a sequence database. Pure Rust, no network.

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Hit {
    pub name: String,
    pub qstart: usize,
    pub qend: usize,
    pub sstart: usize,
    pub send: usize,
    pub score: i32,
    pub identity: f32,
    pub len: usize,
}

const MATCH: i32 = 1;
const MISMATCH: i32 = -1;
const DROPOFF: i32 = 5;

/// Search `query` against `db`, returning up to `max_hits` best hits (one per db entry).
pub fn search(query: &[u8], db: &[(String, Vec<u8>)], k: usize, max_hits: usize) -> Vec<Hit> {
    let k = k.clamp(4, 28);
    if query.len() < k {
        return Vec::new();
    }
    // Seed index: query k-mer -> list of query positions.
    let mut seeds: HashMap<&[u8], Vec<usize>> = HashMap::new();
    for i in 0..=(query.len() - k) {
        seeds.entry(&query[i..i + k]).or_default().push(i);
    }
    let mut hits: Vec<Hit> = Vec::new();
    for (name, subject) in db {
        if subject.len() < k {
            continue;
        }
        let mut best: Option<Hit> = None;
        for j in 0..=(subject.len() - k) {
            let Some(qpositions) = seeds.get(&subject[j..j + k]) else {
                continue;
            };
            for &qi in qpositions {
                // Ungapped extension with dropoff in both directions.
                let (mut qs, mut ss) = (qi, j);
                let (mut qe, mut se) = (qi + k, j + k);
                let score = k as i32 * MATCH;
                let mut best_score = score;
                let (mut bqs, mut bss, mut bqe, mut bse) = (qs, ss, qe, se);
                // extend right
                let (mut rq, mut rs, mut run) = (qe, se, score);
                while rq < query.len() && rs < subject.len() {
                    run += if query[rq] == subject[rs] {
                        MATCH
                    } else {
                        MISMATCH
                    };
                    rq += 1;
                    rs += 1;
                    if run > best_score {
                        best_score = run;
                        bqe = rq;
                        bse = rs;
                    }
                    if best_score - run > DROPOFF {
                        break;
                    }
                }
                // extend left
                let (mut lq, mut ls, mut runl) = (qs, ss, best_score);
                while lq > 0 && ls > 0 {
                    runl += if query[lq - 1] == subject[ls - 1] {
                        MATCH
                    } else {
                        MISMATCH
                    };
                    lq -= 1;
                    ls -= 1;
                    if runl > best_score {
                        best_score = runl;
                        bqs = lq;
                        bss = ls;
                    }
                    if best_score - runl > DROPOFF {
                        break;
                    }
                }
                let _ = (qs, ss, qe, se, score);
                qs = bqs;
                ss = bss;
                qe = bqe;
                se = bse;
                let len = qe - qs;
                if len < k {
                    continue;
                }
                let matches = query[qs..qe]
                    .iter()
                    .zip(subject[ss..se].iter())
                    .filter(|(a, b)| a == b)
                    .count();
                let identity = matches as f32 / len as f32 * 100.0;
                let hit = Hit {
                    name: name.clone(),
                    qstart: qs,
                    qend: qe,
                    sstart: ss,
                    send: se,
                    score: best_score,
                    identity,
                    len,
                };
                if best.as_ref().map(|b| b.score).unwrap_or(i32::MIN) < hit.score {
                    best = Some(hit);
                }
            }
        }
        if let Some(h) = best {
            hits.push(h);
        }
    }
    hits.sort_by_key(|h| std::cmp::Reverse(h.score));
    hits.truncate(max_hits);
    hits
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identical_full_length() {
        let q = b"ACGTACGTACGTACGTACGT";
        let db = vec![("self".to_string(), q.to_vec())];
        let h = search(q, &db, 8, 10);
        assert_eq!(h.len(), 1);
        assert_eq!(h[0].len, q.len());
        assert!((h[0].identity - 100.0).abs() < 1e-6);
    }

    #[test]
    fn substring_hit() {
        let q = b"TTTTACGTACGTACGTAAAA";
        let db = vec![("gene".to_string(), b"GGGACGTACGTACGTCCC".to_vec())];
        let h = search(q, &db, 8, 10);
        assert!(!h.is_empty());
        assert!(h[0].len >= 12);
    }

    #[test]
    fn no_match_empty() {
        let q = b"AAAAAAAAAAAAAAAA";
        let db = vec![("x".to_string(), b"CCCCCCCCCCCCCCCC".to_vec())];
        assert!(search(q, &db, 8, 10).is_empty());
    }
}
