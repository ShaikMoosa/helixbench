use domain::digest::fragments;
#[test]
fn single_circular_cut_linearizes_the_full_molecule() {
    let f = fragments(&[5], 23, true);
    assert_eq!(f.len(), 1);
    assert_eq!(f[0].size, 23);
    assert_eq!(f[0].start, 5);
    assert_eq!(f[0].end, 5);
}
#[test]
fn coincident_and_terminal_cuts_do_not_create_empty_fragments() {
    let f = fragments(&[0, 5, 5, 23], 23, false);
    assert_eq!(f.iter().map(|f| f.size).collect::<Vec<_>>(), vec![5, 18]);
}
