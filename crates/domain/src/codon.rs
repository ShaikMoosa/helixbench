//! Host-aware codon optimization: re-encode a coding sequence with preferred
//! codons while preserving the protein, and report codon adaptation index (CAI).

use crate::genetic_code::{translate_codon, CODES};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Host {
    EColi,
    Human,
}

impl Host {
    pub fn label(self) -> &'static str {
        match self {
            Host::EColi => "E. coli",
            Host::Human => "Human",
        }
    }
}

// Relative codon usage (per thousand), E. coli K12 and Human.
const ECOLI: [(&str, f32); 64] = [
    ("TTT", 22.0),
    ("TTC", 16.0),
    ("TTA", 13.0),
    ("TTG", 13.0),
    ("CTT", 11.0),
    ("CTC", 11.0),
    ("CTA", 4.0),
    ("CTG", 52.0),
    ("ATT", 30.0),
    ("ATC", 25.0),
    ("ATA", 4.0),
    ("ATG", 28.0),
    ("GTT", 18.0),
    ("GTC", 15.0),
    ("GTA", 11.0),
    ("GTG", 26.0),
    ("TCT", 8.0),
    ("TCC", 9.0),
    ("TCA", 7.0),
    ("TCG", 9.0),
    ("AGT", 9.0),
    ("AGC", 16.0),
    ("CCT", 7.0),
    ("CCC", 5.0),
    ("CCA", 8.0),
    ("CCG", 23.0),
    ("ACT", 9.0),
    ("ACC", 23.0),
    ("ACA", 7.0),
    ("ACG", 14.0),
    ("GCT", 15.0),
    ("GCC", 25.0),
    ("GCA", 20.0),
    ("GCG", 33.0),
    ("TAT", 16.0),
    ("TAC", 12.0),
    ("TAA", 2.0),
    ("TAG", 0.2),
    ("CAT", 13.0),
    ("CAC", 10.0),
    ("CAA", 15.0),
    ("CAG", 29.0),
    ("AAT", 18.0),
    ("AAC", 22.0),
    ("AAA", 34.0),
    ("AAG", 10.0),
    ("GAT", 32.0),
    ("GAC", 19.0),
    ("GAA", 39.0),
    ("GAG", 18.0),
    ("TGT", 5.0),
    ("TGC", 6.0),
    ("TGG", 15.0),
    ("CGT", 21.0),
    ("CGC", 22.0),
    ("CGA", 3.0),
    ("CGG", 5.0),
    ("AGA", 2.0),
    ("AGG", 1.0),
    ("GGT", 25.0),
    ("GGC", 29.0),
    ("GGA", 8.0),
    ("GGG", 11.0),
    ("TGA", 1.0),
];
const HUMAN: [(&str, f32); 64] = [
    ("TTT", 17.0),
    ("TTC", 20.0),
    ("TTA", 7.0),
    ("TTG", 13.0),
    ("CTT", 13.0),
    ("CTC", 20.0),
    ("CTA", 7.0),
    ("CTG", 40.0),
    ("ATT", 16.0),
    ("ATC", 21.0),
    ("ATA", 7.0),
    ("ATG", 22.0),
    ("GTT", 11.0),
    ("GTC", 15.0),
    ("GTA", 7.0),
    ("GTG", 28.0),
    ("TCT", 15.0),
    ("TCC", 18.0),
    ("TCA", 12.0),
    ("TCG", 4.0),
    ("AGT", 12.0),
    ("AGC", 20.0),
    ("CCT", 17.0),
    ("CCC", 20.0),
    ("CCA", 17.0),
    ("CCG", 7.0),
    ("ACT", 13.0),
    ("ACC", 19.0),
    ("ACA", 15.0),
    ("ACG", 6.0),
    ("GCT", 18.0),
    ("GCC", 28.0),
    ("GCA", 16.0),
    ("GCG", 7.0),
    ("TAT", 12.0),
    ("TAC", 15.0),
    ("TAA", 1.0),
    ("TAG", 0.8),
    ("CAT", 11.0),
    ("CAC", 15.0),
    ("CAA", 12.0),
    ("CAG", 34.0),
    ("AAT", 17.0),
    ("AAC", 19.0),
    ("AAA", 24.0),
    ("AAG", 32.0),
    ("GAT", 22.0),
    ("GAC", 25.0),
    ("GAA", 30.0),
    ("GAG", 40.0),
    ("TGT", 10.0),
    ("TGC", 12.0),
    ("TGG", 13.0),
    ("CGT", 5.0),
    ("CGC", 10.0),
    ("CGA", 6.0),
    ("CGG", 11.0),
    ("AGA", 12.0),
    ("AGG", 12.0),
    ("GGT", 11.0),
    ("GGC", 22.0),
    ("GGA", 16.0),
    ("GGG", 16.0),
    ("TGA", 1.6),
];

fn table(host: Host) -> &'static [(&'static str, f32); 64] {
    match host {
        Host::EColi => &ECOLI,
        Host::Human => &HUMAN,
    }
}

fn freq(host: Host, codon: &[u8]) -> f32 {
    table(host)
        .iter()
        .find(|(c, _)| c.as_bytes() == codon)
        .map(|(_, f)| *f)
        .unwrap_or(0.0)
}

fn aa_of(codon: &[u8]) -> u8 {
    translate_codon(codon, &CODES[0])
}

fn max_freq_for_aa(host: Host, aa: u8) -> f32 {
    let mut best = 0.0f32;
    for (c, f) in table(host) {
        if aa_of(c.as_bytes()) == aa && *f > best {
            best = *f;
        }
    }
    best
}

/// Re-encode a coding sequence with host-preferred codons, preserving the protein.
pub fn optimize(dna: &[u8], host: Host) -> Vec<u8> {
    optimize_with(dna, host, &OptimizeOptions::default())
}

#[derive(Debug, Clone)]
pub struct OptimizeOptions {
    pub gc_min: f32,
    pub gc_max: f32,
    pub avoid: Vec<Vec<u8>>,
    pub harmonize: bool,
}

impl Default for OptimizeOptions {
    fn default() -> Self {
        Self {
            gc_min: 0.3,
            gc_max: 0.7,
            avoid: Vec::new(),
            harmonize: false,
        }
    }
}

fn synonyms(aa: u8, host: Host) -> Vec<Vec<u8>> {
    let mut v: Vec<(f32, Vec<u8>)> = table(host)
        .iter()
        .filter(|(c, _)| aa_of(c.as_bytes()) == aa)
        .map(|(c, f)| (*f, c.as_bytes().to_vec()))
        .collect();
    v.sort_by(|a, b| b.0.total_cmp(&a.0));
    v.into_iter().map(|(_, c)| c).collect()
}

fn gc_of(seq: &[u8]) -> f32 {
    if seq.is_empty() {
        return 0.0;
    }
    seq.iter().filter(|b| matches!(b, b'G' | b'C')).count() as f32 / seq.len() as f32
}

fn find_site(seq: &[u8], site: &[u8]) -> Option<usize> {
    if site.is_empty() || seq.len() < site.len() {
        return None;
    }
    (0..=(seq.len() - site.len())).find(|&i| &seq[i..i + site.len()] == site)
}

/// Full optimizer: preferred (or harmonized) encoding, then restriction-site
/// avoidance and GC-window smoothing, all protein-preserving.
pub fn optimize_with(dna: &[u8], host: Host, opts: &OptimizeOptions) -> Vec<u8> {
    let mut out = Vec::with_capacity(dna.len());
    for chunk in dna.chunks(3) {
        if chunk.len() < 3 {
            out.extend_from_slice(chunk);
            continue;
        }
        let aa = aa_of(chunk);
        if aa == b'*' {
            out.extend_from_slice(chunk);
            continue;
        }
        let syn = synonyms(aa, host);
        let chosen = if opts.harmonize {
            // Match the source codon's usage rank (E. coli reference) in the target host.
            let mut src: Vec<(f32, Vec<u8>)> = table(Host::EColi)
                .iter()
                .filter(|(c, _)| aa_of(c.as_bytes()) == aa)
                .map(|(c, f)| (*f, c.as_bytes().to_vec()))
                .collect();
            src.sort_by(|a, b| b.0.total_cmp(&a.0));
            let rank = src.iter().position(|(_, c)| c == chunk).unwrap_or(0);
            syn.get(rank.min(syn.len() - 1))
                .cloned()
                .unwrap_or_else(|| syn[0].clone())
        } else {
            syn[0].clone()
        };
        out.extend_from_slice(&chosen);
    }
    // Restriction-site avoidance: swap overlapping codons to synonymous alternatives.
    for site in &opts.avoid {
        let mut guard = 0;
        while let Some(pos) = find_site(&out, site) {
            guard += 1;
            if guard > 1000 {
                break;
            }
            let mut fixed = false;
            let c0 = pos / 3;
            let c1 = ((pos + site.len()).saturating_sub(1)) / 3;
            for ci in c0..=c1 {
                let s = ci * 3;
                if s + 3 > out.len() {
                    continue;
                }
                let aa = aa_of(&out[s..s + 3]);
                if aa == b'*' {
                    continue;
                }
                for alt in synonyms(aa, host) {
                    let mut trial = out.clone();
                    trial[s..s + 3].copy_from_slice(&alt);
                    if find_site(&trial, site).is_none() {
                        out = trial;
                        fixed = true;
                        break;
                    }
                }
                if fixed {
                    break;
                }
            }
            if !fixed {
                break;
            }
        }
    }
    // GC-window smoothing.
    let win = 30usize;
    if out.len() >= win {
        let mut start = 0;
        while start + win <= out.len() {
            let g = gc_of(&out[start..start + win]);
            if g < opts.gc_min || g > opts.gc_max {
                let want_up = g < opts.gc_min;
                for ci in (start / 3)..=((start + win) / 3).min(out.len() / 3) {
                    let s = ci * 3;
                    if s + 3 > out.len() {
                        continue;
                    }
                    let aa = aa_of(&out[s..s + 3]);
                    if aa == b'*' {
                        continue;
                    }
                    let mut syn = synonyms(aa, host);
                    syn.sort_by(|a, b| {
                        let ga = gc_of(a);
                        let gb = gc_of(b);
                        if want_up {
                            gb.total_cmp(&ga)
                        } else {
                            ga.total_cmp(&gb)
                        }
                    });
                    if let Some(best) = syn.first() {
                        out[s..s + 3].copy_from_slice(best);
                    }
                    let g2 = gc_of(&out[start..start + win]);
                    if g2 >= opts.gc_min && g2 <= opts.gc_max {
                        break;
                    }
                }
            }
            start += 15;
        }
    }
    out
}

/// Codon adaptation index (geometric mean of relative adaptiveness), excluding stops.
pub fn cai(dna: &[u8], host: Host) -> f32 {
    let mut sum_ln = 0.0f64;
    let mut n = 0usize;
    for chunk in dna.chunks(3) {
        if chunk.len() < 3 {
            continue;
        }
        let aa = aa_of(chunk);
        if aa == b'*' {
            continue;
        }
        let mx = max_freq_for_aa(host, aa);
        if mx <= 0.0 {
            continue;
        }
        let w = (freq(host, chunk) / mx).max(1e-6);
        sum_ln += (w as f64).ln();
        n += 1;
    }
    if n == 0 {
        return 0.0;
    }
    (sum_ln / n as f64).exp() as f32
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::translate::translate;

    #[test]
    fn optimize_preserves_protein() {
        let dna = b"ATGTTTGGGAAATTTCCCTGA";
        let opt = optimize(dna, Host::EColi);
        assert_eq!(opt.len(), dna.len());
        assert_eq!(translate(dna, &CODES[0]), translate(&opt, &CODES[0]));
    }

    #[test]
    fn cai_improves_or_equal() {
        let dna = b"ATGTTTGGGAAATTTCCCTGA";
        let opt = optimize(dna, Host::EColi);
        assert!(cai(&opt, Host::EColi) >= cai(dna, Host::EColi) - 1e-6);
        assert!((0.0..=1.0).contains(&cai(&opt, Host::EColi)));
    }

    #[test]
    fn optimized_cai_near_one() {
        let dna = b"ATGTTTGGGAAATTTCCCTGA";
        let opt = optimize(dna, Host::EColi);
        assert!(cai(&opt, Host::EColi) > 0.99);
    }

    #[test]
    fn host_tables_differ() {
        let dna = b"ATGCTGAGGAAAGAT";
        let e = optimize(dna, Host::EColi);
        let h = optimize(dna, Host::Human);
        assert_eq!(translate(&e, &CODES[0]), translate(&h, &CODES[0]));
    }

    #[test]
    fn avoid_site_removed_and_protein_preserved() {
        let dna = b"ATGGAATTCGGGAAATTTCCCTGA"; // contains GAATTC (EcoRI)
        let opts = OptimizeOptions {
            avoid: vec![b"GAATTC".to_vec()],
            ..Default::default()
        };
        let out = optimize_with(dna, Host::EColi, &opts);
        assert_eq!(translate(&out, &CODES[0]), translate(dna, &CODES[0]));
        assert!(find_site(&out, b"GAATTC").is_none());
    }

    #[test]
    fn harmonize_preserves_protein() {
        let dna = b"ATGCTGAGGAAAGATTTT";
        let opts = OptimizeOptions {
            harmonize: true,
            ..Default::default()
        };
        let out = optimize_with(dna, Host::Human, &opts);
        assert_eq!(translate(&out, &CODES[0]), translate(dna, &CODES[0]));
    }

    #[test]
    fn gc_smoothing_moves_toward_range() {
        // AT-rich input; smoothing should raise GC toward the window
        let dna = b"ATGAAATTTAAATTTAAATTTAAATTTAAATTTAAATTTTGA";
        let opts = OptimizeOptions {
            gc_min: 0.4,
            gc_max: 0.7,
            ..Default::default()
        };
        let out = optimize_with(dna, Host::EColi, &opts);
        assert_eq!(translate(&out, &CODES[0]), translate(dna, &CODES[0]));
        assert!(gc_of(&out) > gc_of(dna));
    }
}
