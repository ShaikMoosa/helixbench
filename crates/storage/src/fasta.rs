//! FASTA import/export. Headers may carry `circular` / `linear` topology hints.

use domain::{Document, Topology};

use crate::{Result, StorageError};

/// Parse every record in a FASTA file.
pub fn parse_all(bytes: &[u8], fallback_name: &str) -> Result<Vec<Document>> {
    let text = std::str::from_utf8(bytes)
        .map_err(|_| StorageError::Format("FASTA file is not valid UTF-8".into()))?;
    let mut docs = Vec::new();
    let mut name: Option<String> = None;
    let mut description = String::new();
    let mut seq = String::new();
    let mut circular = false;
    let mut saw_header = false;

    let flush = |name: &Option<String>,
                 description: &str,
                 seq: &str,
                 circular: bool,
                 docs: &mut Vec<Document>| {
        if let Some(n) = name {
            let topo = if circular {
                Topology::Circular
            } else {
                Topology::Linear
            };
            let mut d = Document::from_sequence_text(n.clone(), seq, topo);
            d.description = description.to_string();
            if !d.sequence.is_empty() {
                docs.push(d);
            }
        }
    };

    for line in text.lines() {
        let line = line.trim();
        if let Some(header) = line.strip_prefix('>') {
            saw_header = true;
            flush(&name, &description, &seq, circular, &mut docs);
            let header = header.trim();
            let (n, d) = match header.split_once(char::is_whitespace) {
                Some((n, d)) => (n.to_string(), d.trim().to_string()),
                None => (header.to_string(), String::new()),
            };
            // A leading "circular"/"linear" token is a topology hint, not prose.
            let mut desc = d;
            circular = false;
            let first = desc.split_whitespace().next().unwrap_or("");
            if first.eq_ignore_ascii_case("circular") || first.eq_ignore_ascii_case("linear") {
                circular = first.eq_ignore_ascii_case("circular");
                desc = desc[first.len()..].trim().to_string();
            }
            if !circular {
                circular = header.to_ascii_lowercase().contains("circular");
            }
            name = Some(if n.is_empty() {
                fallback_name.to_string()
            } else {
                n
            });
            description = desc;
            seq.clear();
        } else if !line.is_empty() && !line.starts_with(';') {
            seq.push_str(line);
        }
    }
    flush(&name, &description, &seq, circular, &mut docs);

    if docs.is_empty() {
        if saw_header {
            return Err(StorageError::Format(
                "FASTA records contain no sequence".into(),
            ));
        }
        // No headers: treat the whole file as a raw sequence.
        let d = Document::from_sequence_text(fallback_name.to_string(), text, Topology::Linear);
        if d.sequence.is_empty() {
            return Err(StorageError::Format(
                "no sequence found in FASTA file".into(),
            ));
        }
        docs.push(d);
    }
    Ok(docs)
}

/// Parse a FASTA file, returning the first record.
pub fn parse_first(bytes: &[u8], fallback_name: &str) -> Result<Document> {
    let mut docs = parse_all(bytes, fallback_name)?;
    Ok(docs.remove(0))
}

/// Export a document as FASTA (70 columns).
pub fn export(doc: &Document) -> String {
    let mut out = String::new();
    out.push('>');
    out.push_str(&sanitize_header(&doc.name).replace(char::is_whitespace, "_"));
    if doc.sequence.is_circular() {
        out.push_str(" circular");
    }
    if !doc.description.is_empty() {
        out.push(' ');
        out.push_str(&sanitize_header(&doc.description));
    }
    out.push('\n');
    let bases = String::from_utf8_lossy(&doc.sequence.bases);
    for chunk in bases.as_bytes().chunks(70) {
        out.push_str(std::str::from_utf8(chunk).unwrap_or(""));
        out.push('\n');
    }
    out
}

fn sanitize_header(s: &str) -> String {
    s.chars()
        .map(|c| if c == '\n' || c == '\r' { ' ' } else { c })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multi_record() {
        let text = b">alpha first record\nATGC\nATGC\n>beta circular plasmid\nGGGGCCCC\n";
        let docs = parse_all(text, "fb").unwrap();
        assert_eq!(docs.len(), 2);
        assert_eq!(docs[0].name, "alpha");
        assert_eq!(docs[0].description, "first record");
        assert_eq!(docs[0].sequence.bases, b"ATGCATGC".to_vec());
        assert_eq!(docs[0].sequence.topology, Topology::Linear);
        assert_eq!(docs[1].name, "beta");
        assert_eq!(docs[1].sequence.topology, Topology::Circular);
    }

    #[test]
    fn headerless_raw() {
        let docs = parse_all(b"atgccc\n", "raw").unwrap();
        assert_eq!(docs[0].name, "raw");
        assert_eq!(docs[0].sequence.bases, b"ATGCCC".to_vec());
    }

    #[test]
    fn roundtrip() {
        let mut d = Document::new("my plasmid".into(), b"ATGCCC".to_vec(), Topology::Circular);
        d.description = "a test".into();
        let text = export(&d);
        assert!(text.starts_with(">my_plasmid circular a test\n"));
        let back = parse_first(text.as_bytes(), "x").unwrap();
        assert_eq!(back.name, "my_plasmid");
        assert_eq!(back.sequence, d.sequence);
        assert_eq!(back.description, "a test");
    }

    #[test]
    fn empty_fails() {
        assert!(parse_all(b">x\n", "fb").is_err());
    }
}
