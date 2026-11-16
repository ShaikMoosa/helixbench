use domain::{
    pcr,
    primer::{HybridizationParams, Primer},
    DnaSequence, Topology,
};
fn params() -> HybridizationParams {
    HybridizationParams {
        min_3prime_match: 4,
        min_tm: 0.0,
        ..Default::default()
    }
}
#[test]
fn linear_product_can_end_at_last_template_base() {
    let seq = DnaSequence::new(b"AAAAGGGCCCTTTAAACCCGGG".to_vec(), Topology::Linear);
    let f = Primer::new(1, "F".into(), "GGGCCC");
    let r = Primer::new(2, "R".into(), "CCCGGG");
    let products = pcr::products(&seq, &f, &r, &params());
    assert!(products
        .iter()
        .any(|p| p.start == 4 && p.template_len == 18));
}
#[test]
fn pcr_includes_five_prime_tails() {
    let seq = DnaSequence::new(b"AAAAGGGCCCTTTAAACCCGGGTTT".to_vec(), Topology::Linear);
    let f = Primer::new(1, "F".into(), "ATATGGGCCC");
    let r = Primer::new(2, "R".into(), "CCCCCCCGGG");
    let products = pcr::products(&seq, &f, &r, &params());
    let p = products
        .iter()
        .find(|p| p.start == 4 && p.fwd_site.len == 6 && p.rev_site.len == 6)
        .unwrap();
    assert!(p.bases.starts_with(b"ATAT"));
    assert!(p.bases.ends_with(b"GGGG"));
    assert_eq!(p.bases.len(), p.template_len + 8);
}
