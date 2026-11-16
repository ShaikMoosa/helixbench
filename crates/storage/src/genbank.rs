//! GenBank flat-file import/export with `join` / `complement` locations.

use std::time::{SystemTime, UNIX_EPOCH};

use domain::feature::{kind_color, Direction, Feature};
use domain::primer::{binding_sites, HybridizationParams, Primer};
use domain::sequence::reverse_complement;
use domain::{Document, Range, Topology};

use crate::{Result, StorageError};

pub fn parse(bytes: &[u8], fallback_name: &str) -> Result<Document> {
    let text = std::str::from_utf8(bytes)
        .map_err(|_| StorageError::Format("GenBank file is not valid UTF-8".into()))?;
    let lines: Vec<&str> = text.lines().collect();
    if lines
        .iter()
        .filter(|line| line.starts_with("LOCUS"))
        .count()
        != 1
    {
        return Err(StorageError::Format(
            "Multiple GenBank records require import review".into(),
        ));
    }

    let locus = lines
        .iter()
        .find(|l| l.starts_with("LOCUS"))
        .ok_or_else(|| StorageError::Format("missing LOCUS line".into()))?;
    let tokens: Vec<&str> = locus.split_whitespace().collect();
    let molecule = crate::imports::declared_genbank_molecule(&tokens);
    if molecule == Some(crate::imports::Molecule::Rna) {
        return Err(StorageError::Format(
            "RNA requires explicit molecule/conversion review; it was not converted to DNA".into(),
        ));
    }
    let name = tokens.get(1).copied().unwrap_or(fallback_name).to_string();
    let circular = tokens.iter().any(|t| t.eq_ignore_ascii_case("circular"));
    if molecule == Some(crate::imports::Molecule::Protein) {
        return Err(StorageError::Format(
            "Protein GenBank requires molecule-aware import review".into(),
        ));
    }

    let mut description = String::new();
    let mut raw_features: Vec<RawFeature> = Vec::new();
    let mut seq_text = String::new();

    #[derive(PartialEq)]
    enum Section {
        None,
        Definition,
        Features,
        Origin,
    }
    let mut section = Section::None;
    let mut current: Option<RawFeature> = None;

    for &line in &lines {
        if line.starts_with("//") {
            break;
        }
        let keyword = if !line.is_empty() && !line.starts_with(' ') {
            line.split_whitespace().next().unwrap_or("")
        } else {
            ""
        };
        match keyword {
            "LOCUS" | "ACCESSION" | "VERSION" | "KEYWORDS" | "SOURCE" | "REFERENCE" | "COMMENT"
            | "BASE" | "CONTIG" | "DBLINK" | "PRIMARY" => {
                if let Some(f) = current.take() {
                    raw_features.push(f);
                }
                section = Section::None;
                continue;
            }
            "DEFINITION" => {
                section = Section::Definition;
                description = line[10..].trim().to_string();
                continue;
            }
            "FEATURES" => {
                section = Section::Features;
                continue;
            }
            "ORIGIN" => {
                if let Some(f) = current.take() {
                    raw_features.push(f);
                }
                section = Section::Origin;
                continue;
            }
            _ => {}
        }
        match section {
            Section::Definition => {
                if !description.is_empty() {
                    description.push(' ');
                }
                description.push_str(line.trim());
            }
            Section::Features => {
                if line.len() > 5 && line.starts_with("     ") && !line[5..].starts_with(' ') {
                    if let Some(f) = current.take() {
                        raw_features.push(f);
                    }
                    let rest = line[5..].trim_start();
                    let (key, loc) = match rest.split_once(char::is_whitespace) {
                        Some((k, l)) => (k.to_string(), l.trim().to_string()),
                        None => (rest.to_string(), String::new()),
                    };
                    current = Some(RawFeature {
                        key,
                        location: loc,
                        qualifiers: Vec::new(),
                    });
                } else if let Some(f) = current.as_mut() {
                    let trimmed = line.trim_start();
                    if let Some(q) = trimmed.strip_prefix('/') {
                        let (k, v) = match q.split_once('=') {
                            Some((k, v)) => (k.to_string(), v.to_string()),
                            None => (q.to_string(), String::new()),
                        };
                        f.qualifiers.push((k, v));
                    } else if !trimmed.is_empty() {
                        if let Some((_, last)) = f.qualifiers.last_mut() {
                            if !last.is_empty() {
                                last.push(' ');
                            }
                            last.push_str(trimmed);
                        } else {
                            f.location.push_str(trimmed);
                        }
                    }
                }
            }
            Section::Origin => {
                seq_text.push_str(line);
            }
            Section::None => {}
        }
    }
    if let Some(f) = current.take() {
        raw_features.push(f);
    }

    let sequence_letters: String = seq_text.chars().filter(|c| !c.is_ascii_digit()).collect();
    let bases = crate::imports::strict_dna(&sequence_letters)?;
    if bases.is_empty() {
        return Err(StorageError::Format(
            "GenBank record has no ORIGIN sequence".into(),
        ));
    }
    let topology = if circular {
        Topology::Circular
    } else {
        Topology::Linear
    };
    let mut doc = Document::new(name, bases, topology);
    doc.description = description.trim_end_matches('.').to_string();
    let seq_len = doc.sequence.len();

    for raw in raw_features {
        let (segments, reversed) = parse_location(&raw.location, seq_len)?;
        if segments.is_empty() {
            continue;
        }
        let quals = Qualifiers(&raw.qualifiers);
        if raw.key == "primer_bind" {
            // spliced_region already returns the primer 5'->3' for complement.
            let primer_seq = spliced_region(&doc, &segments, reversed);
            let name = quals
                .name()
                .unwrap_or_else(|| format!("Primer {}", doc.primers.len() + 1));
            let mut p = Primer::new(0, name, &String::from_utf8_lossy(&primer_seq));
            p.description = quals.get("note").unwrap_or_default();
            doc.add_primer(p);
            continue;
        }
        let name = quals.name().unwrap_or_else(|| raw.key.clone());
        let mut f = Feature::new(0, name, raw.key.clone(), segments);
        f.direction = if reversed {
            Direction::Reverse
        } else {
            Direction::Forward
        };
        if quals
            .get("direction")
            .map(|d| d.eq_ignore_ascii_case("both"))
            .unwrap_or(false)
        {
            f.direction = Direction::Bidirectional;
        }
        f.color = kind_color(&raw.key);
        f.note = quals.get("note").unwrap_or_default();
        f.product = quals.get("product").unwrap_or_default();
        if let Some(t) = quals.get("transl_table").and_then(|v| v.parse::<u8>().ok()) {
            if domain::genetic_code::CODES.iter().any(|c| c.id == t) {
                f.genetic_code = t;
            }
        }
        if let Some(c) = quals
            .get("codon_start")
            .and_then(|v| v.parse::<usize>().ok())
        {
            f.frame_offset = c.saturating_sub(1).min(2);
        }
        if raw.key == "CDS" {
            f.translated = true;
            let spliced = spliced_region(&doc, &f.segments, reversed);
            if spliced.len() >= 3 {
                let codon = &spliced[0..3];
                f.first_codon_met = codon == b"GTG" || codon == b"TTG";
            }
        }
        doc.add_feature(f);
    }
    Ok(doc)
}

struct RawFeature {
    key: String,
    location: String,
    qualifiers: Vec<(String, String)>,
}

struct Qualifiers<'a>(&'a [(String, String)]);

impl Qualifiers<'_> {
    fn get(&self, key: &str) -> Option<String> {
        self.0
            .iter()
            .find(|(k, _)| k.eq_ignore_ascii_case(key))
            .map(|(_, v)| unquote(v))
    }

    fn name(&self) -> Option<String> {
        for key in ["label", "gene", "product", "locus_tag", "standard_name"] {
            if let Some(v) = self.get(key) {
                if !v.is_empty() {
                    return Some(v);
                }
            }
        }
        None
    }
}

fn unquote(v: &str) -> String {
    let v = v.trim();
    let v = v.strip_prefix('"').unwrap_or(v);
    let v = v.strip_suffix('"').unwrap_or(v);
    v.trim().to_string()
}

fn spliced_region(doc: &Document, segments: &[Range], reversed: bool) -> Vec<u8> {
    let mut out = Vec::new();
    if reversed {
        for seg in segments.iter().rev() {
            out.extend(reverse_complement(&doc.sequence.region(*seg)));
        }
    } else {
        for seg in segments {
            out.extend(doc.sequence.region(*seg));
        }
    }
    out
}

/// Parse a GenBank location into ascending genomic segments plus a strand flag.
fn parse_location(loc: &str, seq_len: usize) -> Result<(Vec<Range>, bool)> {
    let loc: String = loc.chars().filter(|c| !c.is_whitespace()).collect();
    if loc.contains(':') {
        return Err(StorageError::Format(format!(
            "remote locations are not supported: {loc}"
        )));
    }
    parse_loc_inner(&loc, seq_len)
}

fn parse_loc_inner(loc: &str, seq_len: usize) -> Result<(Vec<Range>, bool)> {
    if let Some(inner) = strip_call(loc, "complement") {
        let (segs, rev) = parse_loc_inner(inner, seq_len)?;
        let mut segs = segs;
        segs.reverse();
        return Ok((segs, !rev));
    }
    if let Some(inner) = strip_call(loc, "join").or_else(|| strip_call(loc, "order")) {
        let mut all = Vec::new();
        let mut reversed: Option<bool> = None;
        for part in split_top_level(inner) {
            let (segs, rev) = parse_loc_inner(&part, seq_len)?;
            match reversed {
                None => reversed = Some(rev),
                Some(r) if r != rev => {
                    return Err(StorageError::Format(
                        "mixed-strand joins are not supported".into(),
                    ));
                }
                _ => {}
            }
            all.extend(segs);
        }
        return Ok((all, reversed.unwrap_or(false)));
    }
    let cleaned = loc.replace(['<', '>'], "");
    let (start, end) = if let Some((a, b)) = cleaned.split_once("..") {
        (parse_pos(a)?, parse_pos(b)?)
    } else if let Some((a, _)) = cleaned.split_once('^') {
        let p = parse_pos(a)?;
        (p, p)
    } else {
        let p = parse_pos(&cleaned)?;
        (p, p)
    };
    if start == 0 || end < start {
        return Err(StorageError::Format(format!("invalid location: {loc}")));
    }
    if seq_len == 0 || start > seq_len || end > seq_len {
        return Err(StorageError::Format(format!("Location {loc} is outside the {seq_len}-base sequence; use valid joined segments for a circular origin crossing")));
    }
    let len = end - start + 1;
    Ok((vec![Range::new(start - 1, len)], false))
}

fn parse_pos(s: &str) -> Result<usize> {
    s.parse::<usize>()
        .map_err(|_| StorageError::Format(format!("invalid position {s:?} in location")))
}

fn strip_call<'a>(loc: &'a str, name: &str) -> Option<&'a str> {
    let rest = loc.strip_prefix(name)?.strip_prefix('(')?;
    rest.strip_suffix(')')
}

fn split_top_level(s: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut depth = 0usize;
    let mut start = 0;
    for (i, c) in s.char_indices() {
        match c {
            '(' => depth += 1,
            ')' => depth = depth.saturating_sub(1),
            ',' if depth == 0 => {
                parts.push(s[start..i].to_string());
                start = i + 1;
            }
            _ => {}
        }
    }
    parts.push(s[start..].to_string());
    parts.retain(|p| !p.is_empty());
    parts
}

// ---------------------------------------------------------------------------
// Export
// ---------------------------------------------------------------------------

pub fn export(doc: &Document) -> String {
    let mut out = String::new();
    let locus: String = doc
        .name
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '_' || *c == '-')
        .take(16)
        .collect();
    let locus = if locus.is_empty() {
        "sequence".to_string()
    } else {
        locus
    };
    let topo = if doc.sequence.is_circular() {
        "circular"
    } else {
        "linear"
    };
    out.push_str(&format!(
        "LOCUS       {:<16} {:>7} bp    DNA     {:<8} SYN {}\n",
        locus,
        doc.sequence.len(),
        topo,
        genbank_date()
    ));
    let definition = if doc.description.is_empty() {
        format!("{}.", doc.name)
    } else if doc.description.ends_with('.') {
        doc.description.clone()
    } else {
        format!("{}.", doc.description)
    };
    push_wrapped(&mut out, "DEFINITION", &definition, 12);
    out.push_str("ACCESSION   .\nVERSION     .\nKEYWORDS    .\n");
    out.push_str("SOURCE      synthetic DNA construct\n  ORGANISM  synthetic DNA construct\n");
    out.push_str("            other sequences; artificial sequences.\n");
    out.push_str("FEATURES             Location/Qualifiers\n");

    let seq_len = doc.sequence.len();
    for f in &doc.features {
        let loc = feature_location(f, seq_len, doc.sequence.is_circular());
        push_feature_line(&mut out, &f.kind, &loc);
        push_qualifier(&mut out, "label", &f.name);
        if !f.product.is_empty() {
            push_qualifier(&mut out, "product", &f.product);
        }
        if !f.note.is_empty() {
            push_qualifier(&mut out, "note", &f.note);
        }
        if f.direction == Direction::Bidirectional {
            out.push_str("                     /direction=BOTH\n");
        }
        if f.frame_offset > 0 {
            out.push_str(&format!(
                "                     /codon_start={}\n",
                f.frame_offset + 1
            ));
        }
        if f.genetic_code != 0 {
            out.push_str(&format!(
                "                     /transl_table={}\n",
                f.genetic_code
            ));
        }
        if f.translated {
            let code = domain::genetic_code::code_by_id(if f.genetic_code == 0 {
                doc.default_genetic_code
            } else {
                f.genetic_code
            });
            let aa = f.translation(&doc.sequence, code);
            if !aa.is_empty() {
                push_qualifier(&mut out, "translation", &String::from_utf8_lossy(&aa));
            }
        }
    }
    let params = HybridizationParams::default();
    for p in &doc.primers {
        let sites = binding_sites(p, &doc.sequence, &params);
        let Some(site) = sites.first() else { continue };
        let r = Range::new(site.start, site.len);
        let loc = if site.strand == domain::Strand::Top {
            format!("{}..{}", r.start + 1, r.end())
        } else {
            format!("complement({}..{})", r.start + 1, r.end())
        };
        push_feature_line(&mut out, "primer_bind", &loc);
        push_qualifier(&mut out, "label", &p.name);
        if !p.description.is_empty() {
            push_qualifier(&mut out, "note", &p.description);
        }
    }

    out.push_str("ORIGIN\n");
    let bases = doc.sequence.bases.to_ascii_lowercase();
    for (i, row) in bases.chunks(60).enumerate() {
        out.push_str(&format!("{:>9}", i * 60 + 1));
        for group in row.chunks(10) {
            out.push(' ');
            out.push_str(std::str::from_utf8(group).unwrap_or(""));
        }
        out.push('\n');
    }
    out.push_str("//\n");
    out
}

fn feature_location(f: &Feature, seq_len: usize, circular: bool) -> String {
    let mut parts: Vec<String> = Vec::new();
    for seg in &f.segments {
        if circular && seq_len > 0 && seg.end() > seq_len {
            let tail = seg.end() - seq_len;
            parts.push(format!("{}..{}", seg.start + 1, seq_len));
            if tail > 0 {
                parts.push(format!("1..{tail}"));
            }
        } else if seg.len == 1 {
            parts.push(format!("{}", seg.start + 1));
        } else {
            parts.push(format!("{}..{}", seg.start + 1, seg.end()));
        }
    }
    if f.direction == Direction::Reverse {
        parts.reverse();
    }
    let inner = if parts.len() == 1 {
        parts.remove(0)
    } else {
        format!("join({})", parts.join(","))
    };
    if f.direction == Direction::Reverse {
        format!("complement({inner})")
    } else {
        inner
    }
}

fn push_feature_line(out: &mut String, key: &str, location: &str) {
    let prefix = format!("     {:<16}", key);
    let mut line = prefix.clone();
    let mut first = true;
    for part in location.split_inclusive(',') {
        if !first && line.len() + part.len() > 79 {
            out.push_str(line.trim_end());
            out.push('\n');
            line = "                     ".to_string();
        }
        line.push_str(part);
        first = false;
    }
    out.push_str(line.trim_end());
    out.push('\n');
}

fn push_qualifier(out: &mut String, key: &str, value: &str) {
    let value = value.replace('"', "'");
    let text = format!("/{key}=\"{value}\"");
    let indent = "                     ";
    let width = 79 - indent.len();
    let mut rest = text.as_str();
    while !rest.is_empty() {
        if rest.len() <= width {
            out.push_str(indent);
            out.push_str(rest);
            out.push('\n');
            break;
        }
        let cut = rest[..width].rfind(' ').map(|i| i + 1).unwrap_or(width);
        out.push_str(indent);
        out.push_str(&rest[..cut]);
        out.push('\n');
        rest = &rest[cut..];
    }
}

fn push_wrapped(out: &mut String, keyword: &str, text: &str, indent: usize) {
    let prefix = format!("{:<width$}", keyword, width = indent);
    let mut line = prefix.clone();
    for word in text.split_whitespace() {
        if line.len() + word.len() + 1 > 79 && line.len() > indent {
            out.push_str(&line);
            out.push('\n');
            line = " ".repeat(indent);
        }
        if line.len() > indent {
            line.push(' ');
        }
        line.push_str(word);
    }
    out.push_str(&line);
    out.push('\n');
}

fn genbank_date() -> String {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let (y, m, d) = civil_from_days((secs / 86_400) as i64);
    const MONTHS: [&str; 12] = [
        "JAN", "FEB", "MAR", "APR", "MAY", "JUN", "JUL", "AUG", "SEP", "OCT", "NOV", "DEC",
    ];
    format!(
        "{d:02}-{}-{y:04}",
        MONTHS[(m as usize).saturating_sub(1).min(11)]
    )
}

/// Howard Hinnant's civil-from-days algorithm.
fn civil_from_days(z: i64) -> (i64, u32, u32) {
    let z = z + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    (if m <= 2 { y + 1 } else { y }, m, d)
}

#[cfg(test)]
mod tests {
    use super::*;
    use domain::Strand;

    const SAMPLE: &str =
        "LOCUS       pTest                 60 bp    DNA     circular SYN 01-JAN-2020
DEFINITION  test construct with joined features.
FEATURES             Location/Qualifiers
     CDS             join(1..12,25..36)
                     /label=\"fwd CDS\"
                     /product=\"forward protein\"
                     /codon_start=1
                     /transl_table=11
     CDS             complement(join(49..60,37..48))
                     /label=\"rev CDS\"
                     /note=\"reverse joined\"
     promoter        13..24
                     /gene=\"pX\"
     primer_bind     complement(52..60)
                     /label=\"rev primer\"
ORIGIN
        1 atgaaaccct aagggcccta aaaaaaaaaa atgaaaccct aagggcccta aaaaaaaaaa
//
";

    #[test]
    fn parse_sample() {
        let d = parse(SAMPLE.as_bytes(), "fb").unwrap();
        assert_eq!(d.name, "pTest");
        assert_eq!(d.sequence.topology, Topology::Circular);
        assert_eq!(d.sequence.len(), 60);
        assert_eq!(d.description, "test construct with joined features");
        assert_eq!(d.features.len(), 3);
        let fwd = &d.features[0];
        assert_eq!(fwd.name, "fwd CDS");
        assert_eq!(fwd.segments, vec![Range::new(0, 12), Range::new(24, 12)]);
        assert_eq!(fwd.direction, Direction::Forward);
        assert_eq!(fwd.genetic_code, 11);
        assert!(fwd.translated);
        let rev = &d.features[1];
        assert_eq!(rev.name, "rev CDS");
        assert_eq!(rev.segments, vec![Range::new(36, 12), Range::new(48, 12)]);
        assert_eq!(rev.direction, Direction::Reverse);
        assert_eq!(rev.note, "reverse joined");
        assert_eq!(d.features[2].name, "pX");
        assert_eq!(d.primers.len(), 1);
        assert_eq!(d.primers[0].name, "rev primer");
        let expected = reverse_complement(&d.sequence.region(Range::new(51, 9)));
        assert_eq!(d.primers[0].sequence, expected);
    }

    #[test]
    fn export_import_roundtrip() {
        let mut d = Document::new(
            "pRound".into(),
            b"ATGAAACCCTAAGGGCCCTAAAAAAAAAAAATGAAACCCTAAGGGCCCTAAAAAAAAAAA".to_vec(),
            Topology::Circular,
        );
        d.description = "round trip".into();
        let mut f = Feature::new(
            0,
            "joined".into(),
            "CDS".into(),
            vec![Range::new(0, 12), Range::new(24, 12)],
        );
        f.direction = Direction::Forward;
        f.genetic_code = 11;
        f.frame_offset = 0;
        f.product = "fwd".into();
        d.add_feature(f);
        let mut r = Feature::new(
            0,
            "rev".into(),
            "CDS".into(),
            vec![Range::new(36, 12), Range::new(48, 12)],
        );
        r.direction = Direction::Reverse;
        d.add_feature(r);
        d.add_primer(Primer::new(0, "binds".into(), "ATGAAACCCTAAGGGCCCTA"));

        let text = export(&d);
        let back = parse(text.as_bytes(), "x").unwrap();
        assert_eq!(back.sequence, d.sequence);
        assert_eq!(back.name, "pRound");
        assert_eq!(back.features.len(), 2);
        assert_eq!(back.features[0].segments, d.features[0].segments);
        assert_eq!(back.features[0].direction, Direction::Forward);
        assert_eq!(back.features[0].genetic_code, 11);
        assert_eq!(back.features[1].segments, d.features[1].segments);
        assert_eq!(back.features[1].direction, Direction::Reverse);
        assert_eq!(back.primers.len(), 1);
        assert_eq!(back.primers[0].name, "binds");
        assert_eq!(back.primers[0].sequence, d.primers[0].sequence);
    }

    #[test]
    fn location_edge_cases() {
        let (s, rev) = parse_location("complement(<1..>12)", 100).unwrap();
        assert_eq!(s, vec![Range::new(0, 12)]);
        assert!(rev);
        let (s, _) = parse_location("7", 100).unwrap();
        assert_eq!(s, vec![Range::new(6, 1)]);
        let (s, _) = parse_location("5^6", 100).unwrap();
        assert_eq!(s, vec![Range::new(4, 1)]);
        assert!(parse_location("J00194.1:100..200", 100).is_err());
        assert!(parse_location("join(complement(1..5),8..12)", 100).is_err());
    }

    #[test]
    fn reverse_primer_binding_strand() {
        // Exported reverse primer must re-import with the same 5'->3' sequence.
        let seq = b"GGGCCCATGAAACCCTAAGGGCCC".to_vec();
        let mut d = Document::new("p".into(), seq, Topology::Linear);
        let primer_seq = reverse_complement(&d.sequence.region(Range::new(4, 18)));
        d.add_primer(Primer::new(
            0,
            "r".into(),
            &String::from_utf8_lossy(&primer_seq),
        ));
        let sites = binding_sites(&d.primers[0], &d.sequence, &HybridizationParams::default());
        assert!(sites.iter().any(|s| s.strand == Strand::Bottom));
        let back = parse(export(&d).as_bytes(), "x").unwrap();
        assert_eq!(back.primers[0].sequence, primer_seq);
    }
}
