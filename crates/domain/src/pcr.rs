//! In-silico PCR: amplify a template between a forward and reverse primer.

use crate::primer::{binding_sites, BindingSite, HybridizationParams, Primer};
use crate::sequence::{DnaSequence, Strand};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PcrProduct {
    /// Top-strand start of the amplicon (inclusive).
    pub start: usize,
    /// Top-strand end of the amplicon (exclusive, mod len).
    pub end: usize,
    pub len: usize,
    /// Template-derived body length, excluding 5-prime primer tails.
    pub template_len: usize,
    pub bases: Vec<u8>,
    pub fwd_site: BindingSite,
    pub rev_site: BindingSite,
}

/// Simulate PCR with a forward (top-strand) and reverse (bottom-strand) primer.
/// Returns the amplicon between the forward hybridization start and the reverse
/// hybridization end, handling circular templates.
pub fn simulate(
    seq: &DnaSequence,
    fwd: &Primer,
    rev: &Primer,
    params: &HybridizationParams,
) -> Option<PcrProduct> {
    products_limited(seq, fwd, rev, params, 1)
        .into_iter()
        .next()
}

/// Enumerate explicit compatible binding-site pairs; callers choose rather than silently taking first.
pub fn products(
    seq: &DnaSequence,
    fwd: &Primer,
    rev: &Primer,
    params: &HybridizationParams,
) -> Vec<PcrProduct> {
    products_limited(seq, fwd, rev, params, usize::MAX)
}
fn products_limited(
    seq: &DnaSequence,
    fwd: &Primer,
    rev: &Primer,
    params: &HybridizationParams,
    limit: usize,
) -> Vec<PcrProduct> {
    let l = seq.len();
    if l == 0 {
        return Vec::new();
    }
    let forwards: Vec<_> = binding_sites(fwd, seq, params)
        .into_iter()
        .filter(|s| s.strand == Strand::Top)
        .collect();
    let reverses: Vec<_> = binding_sites(rev, seq, params)
        .into_iter()
        .filter(|s| s.strand == Strand::Bottom)
        .collect();
    let mut out = Vec::new();
    for f in &forwards {
        for r in &reverses {
            let end_raw = r.start + r.len;
            let body = if seq.is_circular() {
                let n = (end_raw % l + l - f.start) % l;
                if n == 0 {
                    l
                } else {
                    n
                }
            } else {
                if end_raw <= f.start {
                    continue;
                }
                end_raw - f.start
            };
            if body > l || body < f.len.max(r.len) {
                continue;
            }
            let f_tail = fwd.sequence.len().saturating_sub(f.len);
            let r_tail = rev.sequence.len().saturating_sub(r.len);
            let mut bases = fwd.sequence[..f_tail].to_vec();
            bases.extend_from_slice(&seq.subseq(f.start, body));
            bases.extend_from_slice(&crate::sequence::reverse_complement(
                &rev.sequence[..r_tail],
            ));
            out.push(PcrProduct {
                start: f.start,
                end: if seq.is_circular() {
                    end_raw % l
                } else {
                    end_raw
                },
                len: bases.len(),
                template_len: body,
                bases,
                fwd_site: f.clone(),
                rev_site: r.clone(),
            });
            if out.len() >= limit {
                return out;
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sequence::Topology;

    fn seq(s: &str, circular: bool) -> DnaSequence {
        DnaSequence::new(
            s.bytes().collect(),
            if circular {
                Topology::Circular
            } else {
                Topology::Linear
            },
        )
    }

    fn params() -> HybridizationParams {
        HybridizationParams {
            min_3prime_match: 4,
            min_tm: 0.0,
            ..Default::default()
        }
    }

    #[test]
    fn linear_amplicon() {
        // template: 5' flank + fwd site + insert + rev site + flank 3'
        let template = seq("AAAAAGGGCCCTTTAAACCCGGGTTT", false);
        let fwd = Primer::new(1, "F".into(), "GGGCCC");
        let rev = Primer::new(2, "R".into(), "CCCGGG"); // rc = CCCGGG
        let p = simulate(&template, &fwd, &rev, &params()).unwrap();
        // fwd GGGCCC at 5..11; rev CCCGGG rc=CCCGGG at 17..23? template idx:
        // A A A A A G G G C C C T T T A A A C C C G G G T T T
        // 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25
        // fwd GGGCCC = idx5..11 ; rev primer CCCGGG rc = CCCGGG matches idx17..23
        assert_eq!(p.start, 5);
        assert_eq!(p.end, 23);
        assert_eq!(p.len, 18);
        assert_eq!(p.bases, template.subseq(5, 18));
    }

    #[test]
    fn no_product_when_rev_upstream_of_fwd_linear() {
        let template = seq("CCCGGGAAAAAGGGCCC", false);
        let fwd = Primer::new(1, "F".into(), "GGGCCC");
        let rev = Primer::new(2, "R".into(), "CCCGGG");
        assert!(simulate(&template, &fwd, &rev, &params()).is_none());
    }

    #[test]
    fn circular_wrap() {
        // rev site near start, fwd site near end -> product wraps origin
        let template = seq("CCCGGGTTTTTTTTTTGGGCCC", true);
        let fwd = Primer::new(1, "F".into(), "GGGCCC");
        let rev = Primer::new(2, "R".into(), "CCCGGG");
        let p = simulate(&template, &fwd, &rev, &params()).unwrap();
        assert_eq!(p.start, 16);
        assert_eq!(p.len, 12); // 16..22 (6) + 0..6 (6)
    }
}
