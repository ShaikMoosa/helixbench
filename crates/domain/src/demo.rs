//! A deterministic demo plasmid so the app opens with believable content:
//! real promoters / tags / recombination sites, a designed expression cassette,
//! a pUC19 MCS with unique cloning sites, and sequencing primers.

use crate::document::{ColorRegion, Document, StrandMask};
use crate::feature::{Direction, Feature};
use crate::primer::Primer;
use crate::sequence::{Range, Topology};

/// Small deterministic LCG so the demo is identical on every launch.
struct Rng(u64);

impl Rng {
    fn next_base(&mut self) -> u8 {
        self.0 = self
            .0
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        match (self.0 >> 33) % 4 {
            0 => b'A',
            1 => b'C',
            2 => b'G',
            _ => b'T',
        }
    }
}

fn avoided_sites() -> Vec<Vec<u8>> {
    crate::enzyme_db::COMMON_CLONING
        .iter()
        .filter_map(|n| crate::enzyme_db::enzyme_by_name(n))
        .filter(|e| (6..=8).contains(&e.site_len()) && e.is_palindromic())
        .map(|e| e.site.as_bytes().to_vec())
        .collect()
}

/// True if any 6-8 bp window ending at the last base of `buf` is an avoided site.
fn creates_site(buf: &[u8], sites: &[Vec<u8>]) -> bool {
    for n in 6..=8usize {
        if buf.len() >= n {
            let tail = &buf[buf.len() - n..];
            if sites.iter().any(|s| s.len() == n && s == tail) {
                return true;
            }
        }
    }
    false
}

fn design_cds(protein: &str, rng: &mut Rng, prefix: &[u8], sites: &[Vec<u8>]) -> Vec<u8> {
    let mut out = Vec::new();
    let mut ctx: Vec<u8> = prefix.to_vec();
    for aa in protein.bytes() {
        let codons = codons_for(aa);
        let start = (rng.0 >> 17) as usize % codons.len();
        let mut chosen = codons[start];
        for k in 0..codons.len() {
            let cand = codons[(start + k) % codons.len()];
            ctx.extend_from_slice(cand.as_bytes());
            let clash = creates_site(&ctx, sites);
            ctx.truncate(ctx.len() - 3);
            if !clash {
                chosen = cand;
                break;
            }
        }
        out.extend_from_slice(chosen.as_bytes());
        ctx.extend_from_slice(chosen.as_bytes());
        if ctx.len() > 16 {
            ctx.drain(..ctx.len() - 16);
        }
    }
    out
}

fn codons_for(aa: u8) -> &'static [&'static str] {
    const TABLE: &[(u8, &[&str])] = &[
        (b'A', &["GCT", "GCC", "GCA", "GCG"]),
        (b'R', &["CGT", "CGC", "CGA", "CGG", "AGA", "AGG"]),
        (b'N', &["AAT", "AAC"]),
        (b'D', &["GAT", "GAC"]),
        (b'C', &["TGT", "TGC"]),
        (b'Q', &["CAA", "CAG"]),
        (b'E', &["GAA", "GAG"]),
        (b'G', &["GGT", "GGC", "GGA", "GGG"]),
        (b'H', &["CAT", "CAC"]),
        (b'I', &["ATT", "ATC", "ATA"]),
        (b'L', &["TTA", "TTG", "CTT", "CTC", "CTA", "CTG"]),
        (b'K', &["AAA", "AAG"]),
        (b'M', &["ATG"]),
        (b'F', &["TTT", "TTC"]),
        (b'P', &["CCT", "CCC", "CCA", "CCG"]),
        (b'S', &["TCT", "TCC", "TCA", "TCG", "AGT", "AGC"]),
        (b'T', &["ACT", "ACC", "ACA", "ACG"]),
        (b'W', &["TGG"]),
        (b'Y', &["TAT", "TAC"]),
        (b'V', &["GTT", "GTC", "GTA", "GTG"]),
        (b'*', &["TAA"]),
    ];
    for (a, codons) in TABLE {
        if *a == aa {
            return codons;
        }
    }
    &["GCT"]
}

/// Backbone free of the common cloning sites so the MCS stays unique.
/// `prefix` supplies upstream context so junctions stay clean too.
fn clean_backbone(len: usize, rng: &mut Rng, prefix: &[u8], sites: &[Vec<u8>]) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::with_capacity(len);
    let mut ctx: Vec<u8> = prefix.to_vec();
    while out.len() < len {
        let b = rng.next_base();
        ctx.push(b);
        if creates_site(&ctx, sites) {
            let alt = match b {
                b'A' => b'C',
                b'C' => b'G',
                b'G' => b'T',
                _ => b'A',
            };
            let last = ctx.len() - 1;
            ctx[last] = alt;
            out.push(alt);
        } else {
            out.push(b);
        }
        if ctx.len() > 16 {
            ctx.drain(..ctx.len() - 16);
        }
    }
    out
}

pub fn build_demo() -> Document {
    let mut rng = Rng(0x5EED_1234_ABCD_0001);
    let mut doc = Document::new("pHelixBench-Demo".into(), Vec::new(), Topology::Circular);
    doc.description = "Demo expression vector built for HelixBench. The backbone and marker CDS are synthetic; promoters, tags, recombination sites and the MCS are real sequences.".into();

    let mut seq: Vec<u8> = Vec::new();
    let sites = avoided_sites();
    let put = |seq: &mut Vec<u8>, bases: &[u8]| -> Range {
        let start = seq.len();
        seq.extend_from_slice(bases);
        Range::new(start, bases.len())
    };
    let bb = |seq: &mut Vec<u8>, n: usize, rng: &mut Rng, sites: &[Vec<u8>]| {
        let prefix: Vec<u8> = seq.iter().rev().take(16).rev().copied().collect();
        seq.extend_from_slice(&clean_backbone(n, rng, &prefix, sites));
    };

    // --- Expression cassette ---
    bb(&mut seq, 96, &mut rng, &sites);
    let t7 = put(&mut seq, b"TAATACGACTCACTATAGGG");
    bb(&mut seq, 8, &mut rng, &sites);
    let laco = put(&mut seq, b"GGAATTGTGAGCGGATAACAATT");
    bb(&mut seq, 10, &mut rng, &sites);
    let rbs = put(&mut seq, b"AGGAGG");
    bb(&mut seq, 7, &mut rng, &sites);

    let protein =
        "MGSSHHHHHHSSGLVPRGSHMASMTGGQQMGGSGGSGGSAEAAAKEAAAKEAAAKAGGSGGSGGSENLYFQGGSGGSGGSDYKDDDDK*";
    let prefix: Vec<u8> = seq.iter().rev().take(16).rev().copied().collect();
    let cds_bases = design_cds(protein, &mut rng, &prefix, &sites);
    let cds_start = seq.len();
    seq.extend_from_slice(&cds_bases);
    let cds = Range::new(cds_start, cds_bases.len());
    bb(&mut seq, 24, &mut rng, &sites);
    let term = put(&mut seq, b"CCAGGCATCAAATAAAACGAAAGGCTCAGTCGAAAGACTGGGCCTTTCGTTTTATCTGTTGTTTGTCGGTGAACGCTCTCCTGAGTAGGACAAAT");
    bb(&mut seq, 60, &mut rng, &sites);

    // --- Cloning region ---
    let m13f = put(&mut seq, b"GTAAAACGACGGCCAGT");
    bb(&mut seq, 14, &mut rng, &sites);
    let mcs = put(
        &mut seq,
        b"GAATTCGAGCTCGGTACCCGGGGATCCTCTAGAGTCGACCTGCAGGCATGCAAGCTTCTCGAGGCGGCCGC",
    );
    bb(&mut seq, 14, &mut rng, &sites);
    let m13r = put(&mut seq, b"CAGGAAACAGCTATGAC");
    bb(&mut seq, 120, &mut rng, &sites);

    // --- Replication & selection (synthetic demo elements) ---
    let ori = put(&mut seq, &clean_backbone(588, &mut rng, &[], &sites));
    bb(&mut seq, 40, &mut rng, &sites);
    let marker_protein = "MKKTAIAIAVALAGFATVAQAAPKDNTWYTGAKLGWSQYHDTGFKIDAKGFTSTQDNGKLVGASIDVQSKDAAHYGAQVDTLNAKAAALQAGADVLKVGSTYDDAKAAAEKGLKVTLTGDNTKFTDGKTVTLPGDTITADGKYTLTGDNTKFTDGKTVTLPGDTITADGKYTLTGDNTKFTDGKTVTLPGDTITADGKY*";
    let prefix: Vec<u8> = seq.iter().rev().take(16).rev().copied().collect();
    let marker_bases = design_cds(marker_protein, &mut rng, &prefix, &sites);
    // Place the marker on the bottom strand: insert its reverse complement.
    let marker_rc = crate::sequence::reverse_complement(&marker_bases);
    let marker_start = seq.len();
    seq.extend_from_slice(&marker_rc);
    let marker = Range::new(marker_start, marker_rc.len());
    bb(&mut seq, 80, &mut rng, &sites);

    // --- Recombination sites & tags ---
    let loxp = put(&mut seq, b"ATAACTTCGTATAATGTATGCTATACGAAGTTAT");
    bb(&mut seq, 150, &mut rng, &sites);
    let frt = put(&mut seq, b"GAAGTTCCTATTCTCTAGAAAGTATAGGAACTTC");
    bb(&mut seq, 200, &mut rng, &sites);
    let kozak = put(&mut seq, b"GCCACC");
    bb(&mut seq, 420, &mut rng, &sites);

    doc.sequence.bases = seq;
    let l = doc.sequence.len();

    let add = |doc: &mut Document,
               name: &str,
               kind: &str,
               range: Range,
               dir: Direction,
               color: [u8; 3],
               note: &str| {
        let mut f = Feature::new(0, name.into(), kind.into(), vec![range]);
        f.direction = dir;
        f.color = color;
        f.note = note.into();
        doc.add_feature(f);
    };

    add(
        &mut doc,
        "T7 promoter",
        "promoter",
        t7,
        Direction::Forward,
        [0x6f, 0xd0, 0x8c],
        "T7 RNA polymerase promoter",
    );
    add(
        &mut doc,
        "lac operator",
        "operator",
        laco,
        Direction::None,
        [0x9e, 0xd0, 0x72],
        "lac repressor binding site",
    );
    add(
        &mut doc,
        "RBS",
        "RBS",
        rbs,
        Direction::None,
        [0x9e, 0xd0, 0x72],
        "Shine-Dalgarno ribosome binding site",
    );
    let mut cds_f = Feature::new(0, "demo_cds".into(), "CDS".into(), vec![cds]);
    cds_f.direction = Direction::Forward;
    cds_f.color = [0x8a, 0xb4, 0xf8];
    cds_f.product = "synthetic demo protein (His6 - T7 tag - helical linker - FLAG)".into();
    cds_f.note = "Designed open reading frame; translates without internal stops.".into();
    cds_f.first_codon_met = true;
    doc.add_feature(cds_f);
    add(
        &mut doc,
        "rrnB T1 terminator (demo)",
        "terminator",
        term,
        Direction::None,
        [0xf2, 0x9e, 0x6b],
        "Transcription terminator region",
    );
    add(
        &mut doc,
        "M13 fwd site",
        "primer_bind",
        m13f,
        Direction::Forward,
        [0xf2, 0x8b, 0x82],
        "M13 forward (-20) priming site",
    );
    add(
        &mut doc,
        "pUC19 MCS + XhoI/NotI",
        "MCS",
        mcs,
        Direction::None,
        [0x8a, 0xb4, 0xf8],
        "Multiple cloning site: EcoRI SacI KpnI SmaI BamHI XbaI SalI PstI SphI HindIII XhoI NotI",
    );
    add(
        &mut doc,
        "M13 rev site",
        "primer_bind",
        m13r,
        Direction::Reverse,
        [0xf2, 0x8b, 0x82],
        "M13 reverse priming site",
    );
    add(
        &mut doc,
        "ori (demo)",
        "rep_origin",
        ori,
        Direction::None,
        [0xb0, 0xb8, 0xc4],
        "Synthetic demo origin of replication",
    );
    let mut marker_f = Feature::new(0, "demo_marker".into(), "CDS".into(), vec![marker]);
    marker_f.direction = Direction::Reverse;
    marker_f.color = [0xf2, 0xc9, 0x4c];
    marker_f.product = "synthetic selection marker (demo)".into();
    marker_f.note = "Designed reverse-strand open reading frame.".into();
    marker_f.first_codon_met = true;
    doc.add_feature(marker_f);
    add(
        &mut doc,
        "loxP",
        "misc_binding",
        loxp,
        Direction::None,
        [0xf2, 0x9e, 0x6b],
        "Cre recombinase recognition site",
    );
    add(
        &mut doc,
        "FRT",
        "misc_binding",
        frt,
        Direction::None,
        [0xf2, 0x9e, 0x6b],
        "FLP recombinase recognition site",
    );
    add(
        &mut doc,
        "Kozak",
        "regulatory",
        kozak,
        Direction::None,
        [0x9e, 0xd0, 0x72],
        "Translation initiation consensus",
    );

    // --- Primers ---
    let fwd_start = cds.start.saturating_sub(20);
    let fwd_seq = doc.sequence.subseq(fwd_start, 24);
    let mut p1 = Primer::new(0, "Demo-Fwd".into(), std::str::from_utf8(&fwd_seq).unwrap());
    p1.description = "Sequencing primer upstream of demo_cds".into();
    p1.color = [0x6f, 0xd0, 0x8c];
    doc.add_primer(p1);

    let rev_region = doc.sequence.subseq(cds.end() - 4, 24);
    let rev_seq = crate::sequence::reverse_complement(&rev_region);
    let mut p2 = Primer::new(0, "Demo-Rev".into(), std::str::from_utf8(&rev_seq).unwrap());
    p2.description = "Reverse sequencing primer downstream of demo_cds".into();
    p2.color = [0xf2, 0x8b, 0x82];
    doc.add_primer(p2);

    let mut p3 = Primer::new(0, "M13-F (-20)".into(), "GTAAAACGACGGCCAGT");
    p3.description = "Universal M13 forward sequencing primer".into();
    doc.add_primer(p3);
    let mut p4 = Primer::new(0, "M13-R".into(), "CAGGAAACAGCTATGAC");
    p4.description = "Universal M13 reverse sequencing primer".into();
    doc.add_primer(p4);

    // --- Custom DNA colors: highlight the affinity/epitope tags in the CDS ---
    let tag = |motif: &str, color: [u8; 3]| -> Option<ColorRegion> {
        protein.find(motif).map(|aa| ColorRegion {
            range: Range::new(cds.start + aa * 3, motif.len() * 3),
            strands: StrandMask(StrandMask::TOP),
            color,
        })
    };
    for region in [
        tag("HHHHHH", [0x2d, 0xd4, 0xbf]),
        tag("MASMTGGQQMG", [0x4d, 0xa3, 0xff]),
        tag("DYKDDDDK", [0xfb, 0xbf, 0x24]),
    ]
    .into_iter()
    .flatten()
    {
        doc.color_regions.push(region);
    }

    // Default enzyme set: unique cutters of six or more bases.
    let sets = crate::enzyme_set::predefined_sets(&doc.sequence);
    if let Some(s) = sets.iter().find(|s| s.name == "Unique 6+ Cutters") {
        doc.chosen_enzymes = s.enzymes.clone();
    }
    let _ = l;
    doc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn demo_is_valid() {
        let d = build_demo();
        assert!(d.sequence.len() > 2500);
        assert!(d.sequence.is_circular());
        assert!(d.features.len() >= 12);
        assert_eq!(d.primers.len(), 4);
        assert_eq!(d.color_regions.len(), 3);

        let code = crate::genetic_code::code_by_id(1);
        for f in d.features.iter().filter(|f| f.translated) {
            let aa = f.translation(&d.sequence, code);
            assert!(!aa.is_empty(), "{} has no translation", f.name);
            assert_eq!(*aa.last().unwrap(), b'*', "{} lacks stop", f.name);
            assert!(
                !f.has_internal_stop(&d.sequence, code),
                "{} has internal stop",
                f.name
            );
        }
    }

    #[test]
    fn demo_mcs_unique_cutters() {
        let d = build_demo();
        for name in [
            "EcoRI", "BamHI", "HindIII", "XhoI", "NotI", "SalI", "PstI", "SphI", "SacI", "KpnI",
            "SmaI",
        ] {
            let e = crate::enzyme_db::enzyme_by_name(name).unwrap();
            let sites = e.find_sites(&d.sequence);
            assert_eq!(
                sites.len(),
                1,
                "{name} should cut once, got {}",
                sites.len()
            );
        }
        // XbaI cuts twice: once in the MCS and once inside the real FRT site.
        let xba = crate::enzyme_db::enzyme_by_name("XbaI").unwrap();
        assert_eq!(xba.find_sites(&d.sequence).len(), 2);
    }

    #[test]
    fn demo_primers_bind() {
        let d = build_demo();
        let params = crate::primer::HybridizationParams::default();
        for p in &d.primers {
            let sites = crate::primer::binding_sites(p, &d.sequence, &params);
            assert!(!sites.is_empty(), "{} has no binding site", p.name);
        }
    }

    #[test]
    fn demo_deterministic() {
        assert_eq!(build_demo().sequence.bases, build_demo().sequence.bases);
    }
}
