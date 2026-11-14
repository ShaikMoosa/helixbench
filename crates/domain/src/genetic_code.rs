//! NCBI genetic codes. Tables are 64 amino acids in TCAG codon order.

#[derive(Debug, Clone)]
pub struct GeneticCode {
    pub id: u8,
    pub name: &'static str,
    pub table: &'static str,
    pub starts: &'static [&'static str],
}

pub const CODES: &[GeneticCode] = &[
    GeneticCode {
        id: 1,
        name: "Standard",
        table: "FFLLSSSSYY**CC*WLLLLPPPPHHQQRRRRIIIMTTTTNNKKSSRRVVVVAAAADDEEGGGG",
        starts: &["ATG"],
    },
    GeneticCode {
        id: 2,
        name: "Vertebrate Mitochondrial",
        table: "FFLLSSSSYY**CCWWLLLLPPPPHHQQRRRRIIMMTTTTNNKKSS**VVVVAAAADDEEGGGG",
        starts: &["ATT", "ATC", "ATA", "ATG", "GTG"],
    },
    GeneticCode {
        id: 3,
        name: "Yeast Mitochondrial",
        table: "FFLLSSSSYY**CCWWTTTTPPPPHHQQRRRRIIMMTTTTNNKKSSRRVVVVAAAADDEEGGGG",
        starts: &["ATA", "ATG", "GTG"],
    },
    GeneticCode {
        id: 4,
        name: "Mold / Protozoan / Coelenterate Mitochondrial",
        table: "FFLLSSSSYY**CCWWLLLLPPPPHHQQRRRRIIIMTTTTNNKKSSRRVVVVAAAADDEEGGGG",
        starts: &["TTA", "TTG", "CTG", "ATT", "ATC", "ATA", "ATG", "GTG"],
    },
    GeneticCode {
        id: 5,
        name: "Invertebrate Mitochondrial",
        table: "FFLLSSSSYY**CCWWLLLLPPPPHHQQRRRRIIMMTTTTNNKKSSSSVVVVAAAADDEEGGGG",
        starts: &["TTG", "ATT", "ATC", "ATA", "ATG", "GTG"],
    },
    GeneticCode {
        id: 6,
        name: "Ciliate / Dasycladacean / Hexamita",
        table: "FFLLSSSSYYQQCC*WLLLLPPPPHHQQRRRRIIIMTTTTNNKKSSRRVVVVAAAADDEEGGGG",
        starts: &["ATG"],
    },
    GeneticCode {
        id: 11,
        name: "Bacterial / Archaeal / Plant Plastid",
        table: "FFLLSSSSYY**CC*WLLLLPPPPHHQQRRRRIIIMTTTTNNKKSSRRVVVVAAAADDEEGGGG",
        starts: &["TTG", "CTG", "ATT", "ATC", "ATA", "ATG", "GTG"],
    },
];

pub fn code_by_id(id: u8) -> &'static GeneticCode {
    CODES.iter().find(|c| c.id == id).unwrap_or(&CODES[0])
}

fn base_index(b: u8) -> Option<usize> {
    match b {
        b'T' | b'U' => Some(0),
        b'C' => Some(1),
        b'A' => Some(2),
        b'G' => Some(3),
        _ => None,
    }
}

pub fn codon_index(codon: &[u8]) -> Option<usize> {
    if codon.len() < 3 {
        return None;
    }
    Some(base_index(codon[0])? * 16 + base_index(codon[1])? * 4 + base_index(codon[2])?)
}

pub fn translate_codon(codon: &[u8], code: &GeneticCode) -> u8 {
    match codon_index(codon) {
        Some(i) => code.table.as_bytes()[i],
        None => b'X',
    }
}

pub fn is_start_codon(codon: &[u8], code: &GeneticCode, allow_alternative: bool) -> bool {
    if codon.len() < 3 {
        return false;
    }
    let s = std::str::from_utf8(&codon[..3]).unwrap_or("");
    if code.starts.contains(&s) {
        return true;
    }
    allow_alternative && (s == "GTG" || s == "TTG")
}

/// Codons in TCAG order, for table display.
pub fn codon_at(index: usize) -> [u8; 3] {
    const B: [u8; 4] = *b"TCAG";
    [B[index / 16], B[(index / 4) % 4], B[index % 4]]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standard_translation() {
        let c = code_by_id(1);
        assert_eq!(translate_codon(b"ATG", c), b'M');
        assert_eq!(translate_codon(b"TAA", c), b'*');
        assert_eq!(translate_codon(b"GGC", c), b'G');
        assert_eq!(translate_codon(b"NNN", c), b'X');
    }

    #[test]
    fn variant_codes() {
        assert_eq!(translate_codon(b"TGA", code_by_id(2)), b'W');
        assert_eq!(translate_codon(b"AGA", code_by_id(2)), b'*');
        assert_eq!(translate_codon(b"ATA", code_by_id(2)), b'M');
        assert_eq!(translate_codon(b"TAA", code_by_id(6)), b'Q');
        assert_eq!(translate_codon(b"AGA", code_by_id(5)), b'S');
        assert_eq!(translate_codon(b"CTG", code_by_id(3)), b'T');
    }

    #[test]
    fn codon_roundtrip() {
        for i in 0..64 {
            assert_eq!(codon_index(&codon_at(i)), Some(i));
        }
    }
}
