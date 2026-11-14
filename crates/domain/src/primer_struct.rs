//! Primer secondary-structure heuristics: hairpin (inverted repeat) and
//! self-dimer (homodimer) previews. Pure and offline; scores are contiguous
//! complementary-run lengths, not full thermodynamics.

/// Best hairpin as `(stem, loop)`: the longest 5′ prefix that reverse-complements
/// the 3′ suffix, separated by a loop of 3..=10 nt.
pub fn hairpin(seq: &[u8]) -> Option<(usize, usize)> {
    let n = seq.len();
    let max_stem = (n.saturating_sub(3)) / 2;
    for stem in (3..=max_stem.min(12)).rev() {
        let loop_len = n - 2 * stem;
        if loop_len > 10 {
            continue;
        }
        if (0..stem).all(|i| comp(seq[i]) == seq[n - 1 - i]) {
            return Some((stem, loop_len));
        }
    }
    None
}

/// Longest contiguous self-complementary run `(length, involves_3prime)` when the
/// primer is paired antiparallel with a copy of itself at any offset.
pub fn self_dimer(seq: &[u8]) -> (usize, bool) {
    let n = seq.len();
    if n == 0 {
        return (0, false);
    }
    let mut best = 0usize;
    let mut best_3p = false;
    for k in 0..(2 * n - 1) {
        let lo = k.saturating_sub(n - 1);
        let hi = k.min(n - 1);
        let mut run = 0usize;
        let mut run_3p = false;
        for i in lo..=hi {
            if comp(seq[i]) == seq[k - i] {
                run += 1;
                if i == n - 1 || k - i == n - 1 {
                    run_3p = true;
                }
                if run > best || (run == best && run_3p && !best_3p) {
                    best = run;
                    best_3p = run_3p;
                }
            } else {
                run = 0;
                run_3p = false;
            }
        }
    }
    (best, best_3p)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Secondary {
    pub hairpin: Option<(usize, usize)>,
    pub dimer: usize,
    pub dimer_3prime: bool,
}

pub fn analyze(seq: &[u8]) -> Secondary {
    let (dimer, dimer_3prime) = self_dimer(seq);
    Secondary {
        hairpin: hairpin(seq),
        dimer,
        dimer_3prime,
    }
}

impl Secondary {
    /// A stem of >=4 bp is likely to nucleate a stable hairpin.
    pub fn hairpin_warn(&self) -> bool {
        self.hairpin.is_some_and(|(stem, _)| stem >= 4)
    }

    /// Long dimers, or shorter ones involving a 3′ end, risk primer-dimer.
    pub fn dimer_warn(&self) -> bool {
        self.dimer >= 5 || (self.dimer >= 3 && self.dimer_3prime)
    }
}

fn comp(b: u8) -> u8 {
    match b {
        b'A' => b'T',
        b'T' => b'A',
        b'G' => b'C',
        b'C' => b'G',
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hairpin_stem_and_loop() {
        // prefix ATGC + loop TTT + revcomp(prefix) GCAT
        let seq = b"ATGCTTTGCAT";
        assert_eq!(hairpin(seq), Some((4, 3)));
        assert!(analyze(seq).hairpin_warn());
    }

    #[test]
    fn no_hairpin_for_homopolymer() {
        assert_eq!(hairpin(b"AAAAAA"), None);
        assert_eq!(self_dimer(b"AAAAAA"), (0, false));
    }

    #[test]
    fn self_dimer_palindrome() {
        let (run, three) = self_dimer(b"ATATAT");
        assert!(run >= 4);
        assert!(three);
        assert!(analyze(b"ATATAT").dimer_warn());
    }

    #[test]
    fn clean_primer_has_no_warnings() {
        let s = analyze(b"ATGCTTAGCCATGATCCTGA");
        assert!(!s.hairpin_warn());
        assert!(!s.dimer_warn());
    }
}
