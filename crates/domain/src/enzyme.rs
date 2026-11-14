use serde::{Deserialize, Serialize};

use crate::iupac;
use crate::sequence::{reverse_complement, DnaSequence, Strand};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Enzyme {
    pub name: String,
    /// Recognition sequence, 5'->3', IUPAC codes allowed.
    pub site: String,
    /// Top-strand cut offset relative to the site start (may lie outside the site).
    pub cut_top: i32,
    /// Bottom-strand cut offset in top-strand coordinates.
    pub cut_bottom: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Overhang {
    FivePrime(usize),
    ThreePrime(usize),
    Blunt,
}

impl Overhang {
    pub fn label(self) -> String {
        match self {
            Overhang::FivePrime(n) => format!("5' overhang ({n} nt)"),
            Overhang::ThreePrime(n) => format!("3' overhang ({n} nt)"),
            Overhang::Blunt => "blunt".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CutSite {
    pub enzyme: String,
    /// Site start on top-strand coordinates.
    pub position: usize,
    pub strand: Strand,
    pub cut_top: usize,
    pub cut_bottom: usize,
}

impl Enzyme {
    pub fn new(name: &str, site: &str, cut_top: i32, cut_bottom: i32) -> Self {
        Self {
            name: name.to_string(),
            site: site.to_string(),
            cut_top,
            cut_bottom,
        }
    }

    pub fn site_len(&self) -> usize {
        self.site.len()
    }

    pub fn overhang(&self) -> Overhang {
        let d = self.cut_bottom - self.cut_top;
        if d > 0 {
            Overhang::FivePrime(d as usize)
        } else if d < 0 {
            Overhang::ThreePrime((-d) as usize)
        } else {
            Overhang::Blunt
        }
    }

    pub fn is_palindromic(&self) -> bool {
        let rc = reverse_complement(self.site.as_bytes());
        rc == self.site.as_bytes()
    }

    /// All recognition sites on both strands (palindromic sites reported once).
    pub fn find_sites(&self, seq: &DnaSequence) -> Vec<CutSite> {
        let l = seq.len();
        let n = self.site_len();
        if l == 0 || n == 0 || n > l && !seq.is_circular() {
            return Vec::new();
        }
        let extended = seq.extended(n.saturating_sub(1));
        let limit = if seq.is_circular() {
            l
        } else {
            l + 1 - n.min(l + 1)
        };
        let mut out = Vec::new();
        let site = self.site.as_bytes();
        let rc_site = reverse_complement(site);

        let scan = |pattern: &[u8], strand: Strand, out: &mut Vec<CutSite>| {
            for pos in 0..limit {
                if pos + n > extended.len() {
                    break;
                }
                let window = &extended[pos..pos + n];
                if pattern
                    .iter()
                    .zip(window)
                    .all(|(&q, &t)| iupac::dna_query_matches(q, t))
                {
                    let (ct, cb) = if strand == Strand::Top {
                        (self.cut_top, self.cut_bottom)
                    } else {
                        // mirrored offsets for a bottom-strand recognition site
                        (n as i32 - self.cut_bottom, n as i32 - self.cut_top)
                    };
                    let norm = |v: i32| -> usize {
                        let x = pos as i64 + v as i64;
                        if seq.is_circular() {
                            x.rem_euclid(l as i64) as usize
                        } else {
                            x.max(0) as usize
                        }
                    };
                    out.push(CutSite {
                        enzyme: self.name.clone(),
                        position: pos,
                        strand,
                        cut_top: norm(ct),
                        cut_bottom: norm(cb),
                    });
                }
            }
        };

        scan(site, Strand::Top, &mut out);
        if !self.is_palindromic() {
            scan(&rc_site, Strand::Bottom, &mut out);
        }
        out.sort_by_key(|s| (s.position, s.strand as u8));
        out
    }
}

/// Cut sites for a set of enzymes, sorted by top-strand cut position.
pub fn digest(seq: &DnaSequence, enzymes: &[Enzyme]) -> Vec<CutSite> {
    let mut all = Vec::new();
    for e in enzymes {
        all.extend(e.find_sites(seq));
    }
    all.sort_by_key(|s| (s.cut_top, s.enzyme.clone()));
    all
}

/// Fragment sizes produced by a digest (circular: n cuts -> n fragments).
pub fn fragment_sizes(seq: &DnaSequence, cuts: &[CutSite]) -> Vec<usize> {
    let l = seq.len();
    if l == 0 {
        return Vec::new();
    }
    let mut positions: Vec<usize> = cuts.iter().map(|c| c.cut_top % l).collect();
    positions.sort_unstable();
    positions.dedup();
    if positions.is_empty() {
        return vec![l];
    }
    let mut sizes = Vec::new();
    if seq.is_circular() {
        for w in positions.windows(2) {
            sizes.push(w[1] - w[0]);
        }
        sizes.push(l - positions.last().unwrap() + positions[0]);
    } else {
        sizes.push(positions[0]);
        for w in positions.windows(2) {
            sizes.push(w[1] - w[0]);
        }
        sizes.push(l - positions.last().unwrap());
    }
    sizes.sort_unstable_by(|a, b| b.cmp(a));
    sizes
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sequence::Topology;

    #[test]
    fn ecori_sites() {
        let e = Enzyme::new("EcoRI", "GAATTC", 1, 5);
        let seq = DnaSequence::new(b"AAAGAATTCCCGAATTCGG".to_vec(), Topology::Linear);
        let sites = e.find_sites(&seq);
        assert_eq!(sites.len(), 2);
        assert_eq!(sites[0].position, 3);
        assert_eq!(sites[0].cut_top, 4);
        assert_eq!(sites[0].cut_bottom, 8);
        assert_eq!(e.overhang(), Overhang::FivePrime(4));
        assert!(e.is_palindromic());
    }

    #[test]
    fn circular_site_wraps() {
        let e = Enzyme::new("EcoRI", "GAATTC", 1, 5);
        let seq = DnaSequence::new(b"AATTCCCCG".to_vec(), Topology::Circular);
        let sites = e.find_sites(&seq);
        assert_eq!(sites.len(), 1);
        assert_eq!(sites[0].position, 8);
    }

    #[test]
    fn non_palindromic_both_strands() {
        // BsaI GGTCTC(1/5)
        let e = Enzyme::new("BsaI", "GGTCTC", 7, 11);
        let seq = DnaSequence::new(b"CCGGTCTCAAAAAGAGACCGG".to_vec(), Topology::Linear);
        let sites = e.find_sites(&seq);
        assert_eq!(sites.len(), 2);
        let top = sites.iter().find(|s| s.strand == Strand::Top).unwrap();
        let bottom = sites.iter().find(|s| s.strand == Strand::Bottom).unwrap();
        assert_eq!(top.position, 2);
        assert_eq!(top.cut_top, 9);
        assert_eq!(bottom.position, 13);
        // mirrored: cut_top = 6 - 11 = -5 -> 13 - 5 = 8
        assert_eq!(bottom.cut_top, 8);
        assert_eq!(bottom.cut_bottom, 12);
    }

    #[test]
    fn blunt_cutter() {
        let e = Enzyme::new("SmaI", "CCCGGG", 3, 3);
        assert_eq!(e.overhang(), Overhang::Blunt);
        let seq = DnaSequence::new(b"TTCCCGGGAA".to_vec(), Topology::Linear);
        let sites = e.find_sites(&seq);
        assert_eq!(sites[0].cut_top, 5);
        assert_eq!(sites[0].cut_bottom, 5);
    }

    #[test]
    fn degenerate_site() {
        let e = Enzyme::new("HincII", "GTYRAC", 3, 3);
        let seq = DnaSequence::new(b"AGTCGACT".to_vec(), Topology::Linear);
        assert_eq!(e.find_sites(&seq).len(), 1);
        let seq2 = DnaSequence::new(b"AGTTAACT".to_vec(), Topology::Linear);
        assert_eq!(e.find_sites(&seq2).len(), 1);
    }

    #[test]
    fn circular_fragments() {
        let seq = DnaSequence::new(vec![b'A'; 100], Topology::Circular);
        let cuts = vec![
            CutSite {
                enzyme: "X".into(),
                position: 10,
                strand: Strand::Top,
                cut_top: 10,
                cut_bottom: 10,
            },
            CutSite {
                enzyme: "Y".into(),
                position: 40,
                strand: Strand::Top,
                cut_top: 40,
                cut_bottom: 40,
            },
        ];
        assert_eq!(fragment_sizes(&seq, &cuts), vec![70, 30]);
    }
}
