//! Protein / amino-acid analysis: BLOSUM62 alignment and physicochemical
//! properties (MW, pI, hydropathy, charge, extinction coefficient, composition).

const AA_ORDER: &[u8] = b"ARNDCQEGHILKMFPSTWYV*";

fn idx(a: u8) -> usize {
    AA_ORDER
        .iter()
        .position(|&x| x == a)
        .unwrap_or(AA_ORDER.len() - 1)
}

/// BLOSUM62 substitution matrix (rows/cols in AA_ORDER).
#[rustfmt::skip]
const BLOSUM62: [[i32; 21]; 21] = [
//  A  R  N  D  C  Q  E  G  H  I  L  K  M  F  P  S  T  W  Y  V  *
 [ 4,-1,-2,-2, 0,-1,-1, 0,-2,-1,-1,-1,-1,-2,-1, 1, 0,-3,-2, 0,-4], // A
 [-1, 5, 0,-2,-3, 1, 0,-2, 0,-3,-2, 2,-1,-3,-2,-1,-1,-3,-2,-3,-4], // R
 [-2, 0, 6, 1,-3, 0, 0, 0, 1,-3,-3, 0,-2,-3,-2, 1, 0,-4,-2,-3,-4], // N
 [-2,-2, 1, 6,-3, 0, 2,-1,-1,-3,-4,-1,-3,-3,-1, 0,-1,-4,-3,-3,-4], // D
 [ 0,-3,-3,-3, 9,-3,-4,-3,-3,-1,-1,-3,-1,-2,-3,-1,-1,-2,-2,-1,-4], // C
 [-1, 1, 0, 0,-3, 5, 2,-2, 0,-3,-2, 1, 0,-3,-1, 0,-1,-2,-1,-2,-4], // Q
 [-1, 0, 0, 2,-4, 2, 5,-2, 0,-3,-3, 1,-2,-3,-1, 0,-1,-3,-2,-2,-4], // E
 [ 0,-2, 0,-1,-3,-2,-2, 6,-2,-4,-4,-2,-3,-3,-2, 0,-2,-2,-3,-3,-4], // G
 [-2, 0, 1,-1,-3, 0, 0,-2, 8,-3,-3,-1,-2,-1,-2,-1,-2,-2, 2,-3,-4], // H
 [-1,-3,-3,-3,-1,-3,-3,-4,-3, 4, 2,-3, 1, 0,-3,-2,-1,-3,-1, 3,-4], // I
 [-1,-2,-3,-4,-1,-2,-3,-4,-3, 2, 4,-2, 2, 0,-3,-2,-1,-2,-1, 1,-4], // L
 [-1, 2, 0,-1,-3, 1, 1,-2,-1,-3,-2, 5,-1,-3,-1, 0,-1,-3,-2,-2,-4], // K
 [-1,-1,-2,-3,-1, 0,-2,-3,-2, 1, 2,-1, 5, 0,-2,-1,-1,-1,-1, 1,-4], // M
 [-2,-3,-3,-3,-2,-3,-3,-3,-1, 0, 0,-3, 0, 6,-4,-2,-2, 1, 3,-1,-4], // F
 [-1,-2,-2,-1,-3,-1,-1,-2,-2,-3,-3,-1,-2,-4, 7,-1,-1,-4,-3,-2,-4], // P
 [ 1,-1, 1, 0,-1, 0, 0, 0,-1,-2,-2, 0,-1,-2,-1, 4, 1,-3,-2,-2,-4], // S
 [ 0,-1, 0,-1,-1,-1,-1,-2,-2,-1,-1,-1,-1,-2,-1, 1, 5,-2,-2, 0,-4], // T
 [-3,-3,-4,-4,-2,-2,-3,-2,-2,-3,-2,-3,-1, 1,-4,-3,-2,11, 2,-3,-4], // W
 [-2,-2,-2,-3,-2,-1,-2,-3, 2,-1,-1,-2,-1, 3,-3,-2,-2, 2, 7,-1,-4], // Y
 [ 0,-3,-3,-3,-1,-2,-2,-3,-3, 3, 1,-2, 1,-1,-2,-2, 0,-3,-1, 4,-4], // V
 [-4,-4,-4,-4,-4,-4,-4,-4,-4,-4,-4,-4,-4,-4,-4,-4,-4,-4,-4,-4, 1], // *
];

pub fn blosum(a: u8, b: u8) -> i32 {
    BLOSUM62[idx(a)][idx(b)]
}

/// Global protein alignment with BLOSUM62 + linear gap penalty.
pub fn align_protein(a: &[u8], b: &[u8]) -> (Vec<u8>, Vec<u8>, i32) {
    let gap = -8;
    let n = a.len();
    let m = b.len();
    let w = m + 1;
    let mut dp = vec![0i32; (n + 1) * w];
    for i in 0..=n {
        dp[i * w] = gap * i as i32;
    }
    for (j, item) in dp.iter_mut().enumerate().take(m + 1) {
        *item = gap * j as i32;
    }
    for i in 1..=n {
        for j in 1..=m {
            let diag = dp[(i - 1) * w + (j - 1)] + blosum(a[i - 1], b[j - 1]);
            let up = dp[(i - 1) * w + j] + gap;
            let left = dp[i * w + (j - 1)] + gap;
            dp[i * w + j] = diag.max(up).max(left);
        }
    }
    let (mut ra, mut rb) = (Vec::new(), Vec::new());
    let (mut i, mut j) = (n, m);
    while i > 0 || j > 0 {
        if i > 0 && j > 0 && dp[i * w + j] == dp[(i - 1) * w + (j - 1)] + blosum(a[i - 1], b[j - 1])
        {
            ra.push(a[i - 1]);
            rb.push(b[j - 1]);
            i -= 1;
            j -= 1;
        } else if i > 0 && dp[i * w + j] == dp[(i - 1) * w + j] + gap {
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

fn residue_mass(a: u8) -> f32 {
    match a {
        b'A' => 71.08,
        b'R' => 156.19,
        b'N' => 114.10,
        b'D' => 115.09,
        b'C' => 103.14,
        b'E' => 129.12,
        b'Q' => 128.13,
        b'G' => 57.05,
        b'H' => 137.14,
        b'I' => 113.16,
        b'L' => 113.16,
        b'K' => 128.17,
        b'M' => 131.19,
        b'F' => 147.18,
        b'P' => 97.12,
        b'S' => 87.08,
        b'T' => 101.10,
        b'W' => 186.21,
        b'Y' => 163.18,
        b'V' => 99.13,
        _ => 0.0,
    }
}

fn hydropathy(a: u8) -> f32 {
    match a {
        b'A' => 1.8,
        b'R' => -4.5,
        b'N' => -3.5,
        b'D' => -3.5,
        b'C' => 2.5,
        b'E' => -3.5,
        b'Q' => -3.5,
        b'G' => -0.4,
        b'H' => -3.2,
        b'I' => 4.5,
        b'L' => 3.8,
        b'K' => -3.9,
        b'M' => 1.9,
        b'F' => 2.8,
        b'P' => -1.6,
        b'S' => -0.8,
        b'T' => -0.7,
        b'W' => -0.9,
        b'Y' => -1.3,
        b'V' => 4.2,
        _ => 0.0,
    }
}

/// Net charge at a given pH (Henderson–Hasselbalch over ionizable groups).
pub fn charge_at(protein: &[u8], ph: f32) -> f32 {
    let mut q = 0.0f32;
    // positive: N-term, K, R, H
    let pos = |pka: f32| 1.0 / (1.0 + 10f32.powf(ph - pka));
    let neg = |pka: f32| -1.0 / (1.0 + 10f32.powf(pka - ph));
    if !protein.is_empty() {
        q += pos(9.69); // N-term
        q += neg(2.34); // C-term
    }
    for &a in protein {
        q += match a {
            b'K' => pos(10.5),
            b'R' => pos(12.4),
            b'H' => pos(6.0),
            b'D' => neg(3.86),
            b'E' => neg(4.25),
            b'C' => neg(8.33),
            b'Y' => neg(10.07),
            _ => 0.0,
        };
    }
    q
}

#[derive(Debug, Clone)]
pub struct ProteinStats {
    pub len: usize,
    pub mw: f32,
    pub pi: f32,
    pub gravy: f32,
    pub charge_ph7: f32,
    pub ext_coeff: f32,
    pub composition: Vec<(char, usize)>,
}

pub fn analyze(protein: &[u8]) -> ProteinStats {
    let clean: Vec<u8> = protein
        .iter()
        .copied()
        .filter(|a| *a != b'-' && *a != b'*')
        .collect();
    let len = clean.len();
    let mw: f32 =
        clean.iter().map(|&a| residue_mass(a)).sum::<f32>() + if len > 0 { 18.02 } else { 0.0 };
    let gravy = if len > 0 {
        clean.iter().map(|&a| hydropathy(a)).sum::<f32>() / len as f32
    } else {
        0.0
    };
    // pI by bisection on net charge.
    let (mut lo, mut hi) = (0.0f32, 14.0f32);
    for _ in 0..60 {
        let mid = (lo + hi) / 2.0;
        if charge_at(&clean, mid) > 0.0 {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    let pi = (lo + hi) / 2.0;
    let w = clean.iter().filter(|&&a| a == b'W').count() as f32;
    let y = clean.iter().filter(|&&a| a == b'Y').count() as f32;
    let c = clean.iter().filter(|&&a| a == b'C').count() as f32;
    let ext_coeff = w * 5500.0 + y * 1490.0 + (c / 2.0).floor() * 125.0;
    let mut composition = Vec::new();
    for &a in AA_ORDER {
        if a == b'*' {
            continue;
        }
        let n = clean.iter().filter(|&&x| x == a).count();
        if n > 0 {
            composition.push((a as char, n));
        }
    }
    ProteinStats {
        len,
        mw,
        pi,
        gravy,
        charge_ph7: charge_at(&clean, 7.0),
        ext_coeff,
        composition,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mw_and_composition() {
        let s = analyze(b"AA");
        assert!((s.mw - (71.08 * 2.0 + 18.02)).abs() < 0.1);
        assert_eq!(s.len, 2);
    }

    #[test]
    fn pi_acidic_vs_basic() {
        let acidic = analyze(b"DDDDDEEEEE");
        let basic = analyze(b"KKKKKRRRRR");
        assert!(acidic.pi < 5.0);
        assert!(basic.pi > 9.0);
    }

    #[test]
    fn charge_sign() {
        assert!(charge_at(b"KKKKK", 7.0) > 0.0);
        assert!(charge_at(b"DDDDD", 7.0) < 0.0);
    }

    #[test]
    fn protein_alignment_identical() {
        let (a, b, score) = align_protein(b"MKWVTF", b"MKWVTF");
        assert_eq!(a, b"MKWVTF");
        assert_eq!(b, b"MKWVTF");
        assert!(score > 0);
    }

    #[test]
    fn blosum_symmetric() {
        assert_eq!(blosum(b'W', b'W'), 11);
        assert_eq!(blosum(b'A', b'R'), blosum(b'R', b'A'));
    }
}
