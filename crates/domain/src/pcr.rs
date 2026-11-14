//! In-silico PCR: amplify a template between a forward and reverse primer.

use crate::primer::{binding_sites, BindingSite, HybridizationParams, Primer};
use crate::sequence::{DnaSequence, Strand};

#[derive(Debug, Clone)]
pub struct PcrProduct {
    /// Top-strand start of the amplicon (inclusive).
    pub start: usize,
    /// Top-strand end of the amplicon (exclusive, mod len).
    pub end: usize,
    pub len: usize,
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
    let l = seq.len();
    if l == 0 {
        return None;
    }
    let f = binding_sites(fwd, seq, params)
        .into_iter()
        .find(|s| s.strand == Strand::Top)?;
    let r = binding_sites(rev, seq, params)
        .into_iter()
        .find(|s| s.strand == Strand::Bottom)?;
    let start = f.start;
    let end = (r.start + r.len) % l;
    let len = if seq.is_circular() {
        (end + l - start) % l
    } else {
        if end <= start {
            return None;
        }
        end - start
    };
    if len == 0 || len > l {
        return None;
    }
    Some(PcrProduct {
        start,
        end,
        len,
        bases: seq.subseq(start, len),
        fwd_site: f,
        rev_site: r,
    })
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
