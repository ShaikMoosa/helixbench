//! IUPAC ambiguity expansion for DNA and protein matching.

pub const A: u8 = 1;
pub const C: u8 = 2;
pub const G: u8 = 4;
pub const T: u8 = 8;

/// Bitmask of concrete DNA bases represented by an IUPAC code.
pub fn dna_mask(b: u8) -> u8 {
    match b.to_ascii_uppercase() {
        b'A' => A,
        b'C' => C,
        b'G' => G,
        b'T' | b'U' => T,
        b'R' => A | G,
        b'Y' => C | T,
        b'S' => G | C,
        b'W' => A | T,
        b'K' => G | T,
        b'M' => A | C,
        b'B' => C | G | T,
        b'D' => A | G | T,
        b'H' => A | C | T,
        b'V' => A | C | G,
        b'N' | b'X' => A | C | G | T,
        _ => 0,
    }
}

/// A query base matches a target base when every concrete base the target
/// could be is allowed by the query.
pub fn dna_query_matches(query: u8, target: u8) -> bool {
    let q = dna_mask(query);
    let t = dna_mask(target);
    q != 0 && t != 0 && (t & !q) == 0
}

/// Bitmask over the 26-letter alphabet for protein ambiguity codes.
pub fn protein_mask(b: u8) -> u32 {
    let c = b.to_ascii_uppercase();
    match c {
        b'B' => (1 << (b'D' - b'A')) | (1 << (b'N' - b'A')),
        b'Z' => (1 << (b'E' - b'A')) | (1 << (b'Q' - b'A')),
        b'X' => (1u32 << 26) - 1,
        b'A'..=b'Z' => 1 << (c - b'A'),
        _ => 0,
    }
}

pub fn protein_query_matches(query: u8, target: u8) -> bool {
    let q = protein_mask(query);
    let t = protein_mask(target);
    q != 0 && t != 0 && (t & !q) == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn degenerate_matching() {
        assert!(dna_query_matches(b'R', b'A'));
        assert!(dna_query_matches(b'R', b'G'));
        assert!(!dna_query_matches(b'R', b'C'));
        assert!(dna_query_matches(b'N', b'T'));
        assert!(dna_query_matches(b'A', b'A'));
        assert!(!dna_query_matches(b'A', b'R'));
        assert!(dna_query_matches(b'R', b'R'));
    }

    #[test]
    fn protein_ambiguity() {
        assert!(protein_query_matches(b'X', b'W'));
        assert!(protein_query_matches(b'B', b'D'));
        assert!(protein_query_matches(b'B', b'N'));
        assert!(!protein_query_matches(b'B', b'E'));
        assert!(protein_query_matches(b'Z', b'Q'));
    }
}
