//! Virtual restriction digest: fragment computation + agarose-gel migration model.

#[derive(Debug, Clone, PartialEq)]
pub struct Fragment {
    pub start: usize,
    pub end: usize,
    pub size: usize,
}

/// Compute digest fragments from cut positions.
/// Linear: fragments between consecutive cuts plus the two terminal fragments.
/// Circular: fragments between consecutive cuts wrapping the origin.
pub fn fragments(cuts: &[usize], len: usize, circular: bool) -> Vec<Fragment> {
    if len == 0 {
        return Vec::new();
    }
    let mut c: Vec<usize> = cuts.iter().copied().filter(|&p| p < len).collect();
    c.sort_unstable();
    c.dedup();
    if c.is_empty() {
        return vec![Fragment {
            start: 0,
            end: len,
            size: len,
        }];
    }
    let mut out = Vec::new();
    if circular {
        for i in 0..c.len() {
            let s = c[i];
            let e = c[(i + 1) % c.len()];
            let size = (e + len - s) % len;
            out.push(Fragment {
                start: s,
                end: e,
                size,
            });
        }
    } else {
        let mut prev = 0usize;
        for &pos in &c {
            out.push(Fragment {
                start: prev,
                end: pos,
                size: pos - prev,
            });
            prev = pos;
        }
        out.push(Fragment {
            start: prev,
            end: len,
            size: len - prev,
        });
    }
    out
}

/// Standard 1 kb-plus ladder marker sizes (bp), large to small.
pub const LADDER: [usize; 13] = [
    10000, 8000, 6000, 5000, 4000, 3000, 2000, 1500, 1000, 750, 500, 250, 100,
];

/// Relative migration distance (0 = well / top, 1 = bottom) for a fragment on a
/// log-scale agarose gel between `min_bp` and `max_bp`.
pub fn gel_fraction(size: usize, min_bp: f32, max_bp: f32) -> f32 {
    let s = (size as f32).clamp(min_bp, max_bp);
    let lo = min_bp.ln();
    let hi = max_bp.ln();
    if hi <= lo {
        return 0.0;
    }
    ((hi - s.ln()) / (hi - lo)).clamp(0.0, 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linear_fragments_sum_to_len() {
        let f = fragments(&[100, 300, 700], 1000, false);
        assert_eq!(f.len(), 4);
        assert_eq!(f.iter().map(|x| x.size).sum::<usize>(), 1000);
        assert_eq!(f[0].size, 100);
        assert_eq!(f[3].size, 300);
    }

    #[test]
    fn circular_fragments_wrap() {
        let f = fragments(&[100, 300], 1000, true);
        assert_eq!(f.len(), 2);
        assert_eq!(f.iter().map(|x| x.size).sum::<usize>(), 1000);
        assert_eq!(f[1].size, 800); // 300 -> 100 wrap
    }

    #[test]
    fn uncut_returns_whole() {
        assert_eq!(fragments(&[], 500, false)[0].size, 500);
        assert_eq!(fragments(&[], 500, true)[0].size, 500);
    }

    #[test]
    fn gel_migration_monotonic() {
        let big = gel_fraction(8000, 100.0, 10000.0);
        let small = gel_fraction(200, 100.0, 10000.0);
        assert!(small > big);
        assert!((0.0..=1.0).contains(&big));
        assert!((0.0..=1.0).contains(&small));
    }
}
