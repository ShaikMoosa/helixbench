//! Non-mutating sequence interpretation. Original input is retained through review.
use crate::{Result, StorageError};
use domain::{Document, Topology};
use sha2::{Digest, Sha256};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone)]
pub struct ImportSource {
    pub name: String,
    pub path: Option<PathBuf>,
    bytes: Vec<u8>,
}
impl ImportSource {
    pub fn captured_file(path: &Path, bytes: Vec<u8>) -> Self {
        Self {
            name: path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .into_owned(),
            path: Some(path.to_owned()),
            bytes,
        }
    }
    pub fn text(name: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            path: None,
            bytes: text.into().into_bytes(),
        }
    }
    pub fn file(path: &Path) -> Result<Self> {
        Ok(Self {
            name: path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .into_owned(),
            path: Some(path.to_owned()),
            bytes: std::fs::read(path)?,
        })
    }
    pub fn original_bytes(&self) -> &[u8] {
        &self.bytes
    }
    pub fn original_text(&self) -> Result<&str> {
        std::str::from_utf8(&self.bytes).map_err(|_| {
            StorageError::Format("Input is not UTF-8 text; the source file was not changed".into())
        })
    }
}

pub fn is_native_input(path: &Path, bytes: &[u8]) -> bool {
    path.extension()
        .and_then(|s| s.to_str())
        .is_some_and(|s| s.eq_ignore_ascii_case("hbx"))
        || bytes.iter().find(|byte| !byte.is_ascii_whitespace()) == Some(&b'{')
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Molecule {
    Dna,
    Rna,
    Protein,
}
impl Molecule {
    pub fn label(self) -> &'static str {
        match self {
            Self::Dna => "DNA",
            Self::Rna => "RNA",
            Self::Protein => "Protein",
        }
    }
}
pub fn declared_genbank_molecule(tokens: &[&str]) -> Option<Molecule> {
    let types: Vec<_> = tokens
        .iter()
        .skip(3)
        .map(|s| s.to_ascii_uppercase())
        .collect();
    if types.iter().any(|s| s.ends_with("RNA")) {
        Some(Molecule::Rna)
    } else if types.iter().any(|s| s == "AA" || s == "PROTEIN") {
        Some(Molecule::Protein)
    } else if types.iter().any(|s| s.ends_with("DNA")) {
        Some(Molecule::Dna)
    } else {
        None
    }
}

fn has_omitted_genbank_metadata(original: &str) -> bool {
    original.lines().any(|line| {
        let trimmed = line.trim_start();
        if trimmed == "//" {
            return false;
        }
        let keyword = line.split_whitespace().next().unwrap_or("");
        let omitted_header = line.as_bytes().first().is_some_and(u8::is_ascii_alphabetic)
            && !["LOCUS", "DEFINITION", "FEATURES", "ORIGIN"].contains(&keyword);
        omitted_header
            || keyword == "ORGANISM"
            || trimmed.strip_prefix('/').is_some_and(|qualifier| {
                let key = qualifier.split('=').next().unwrap_or("");
                ![
                    "label",
                    "gene",
                    "name",
                    "note",
                    "product",
                    "direction",
                    "transl_table",
                    "codon_start",
                ]
                .contains(&key)
            })
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImportFormat {
    Fasta,
    GenBank,
    Raw,
}
impl ImportFormat {
    pub fn label(self) -> &'static str {
        match self {
            Self::Fasta => "FASTA",
            Self::GenBank => "GenBank",
            Self::Raw => "Raw sequence",
        }
    }
}
#[derive(Debug, Clone, Default)]
pub struct ImportOptions {
    pub molecule: Option<Molecule>,
    pub convert_rna_to_dna: bool,
    pub remove_gaps: bool,
    pub topology: Option<Topology>,
}
#[derive(Debug, Clone)]
pub struct ImportRecord {
    /// One-based source record index; invalid records are never removed.
    pub index: usize,
    pub name: String,
    pub original_record: String,
    pub original_sequence: String,
    pub detected: Molecule,
    pub interpreted: Molecule,
    pub transformations: Vec<String>,
    pub requires_acknowledgment: bool,
    pub errors: Vec<String>,
    pub document: Option<Document>,
}
#[derive(Debug, Clone)]
pub struct ImportPreview {
    pub source: ImportSource,
    pub format: ImportFormat,
    pub source_sha256: String,
    pub records: Vec<ImportRecord>,
}
struct RawRecord {
    original: String,
    sequence: String,
    name: String,
    description: String,
    topology: Topology,
    declared: Option<Molecule>,
    errors: Vec<String>,
}

pub fn preview_import(
    source: ImportSource,
    options: &HashMap<usize, ImportOptions>,
) -> Result<ImportPreview> {
    let original = source.original_text()?;
    let text = original.trim_start_matches('\u{feff}');
    let ext = source
        .path
        .as_deref()
        .unwrap_or_else(|| Path::new(&source.name))
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();
    let format = if text.lines().any(|line| line.trim_start().starts_with('>')) {
        ImportFormat::Fasta
    } else if text.trim_start().starts_with("LOCUS")
        || matches!(ext.as_str(), "gb" | "gbk" | "genbank" | "gp")
    {
        ImportFormat::GenBank
    } else {
        ImportFormat::Raw
    };
    if text.trim_start().starts_with('{') || ext == "hbx" {
        return Err(StorageError::Format(
            "Open native HelixBench files through the native document loader".into(),
        ));
    }
    let declared = match ext.as_str() {
        "faa" | "gp" => Some(Molecule::Protein),
        "fna" => Some(Molecule::Dna),
        _ => None,
    };
    let fallback = Path::new(&source.name)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("Imported sequence");
    let raw = match format {
        ImportFormat::Fasta => split_fasta(text, fallback, declared)?,
        ImportFormat::GenBank => split_genbank(text, fallback)?,
        ImportFormat::Raw => vec![RawRecord {
            original: text.into(),
            sequence: text.into(),
            name: fallback.into(),
            description: String::new(),
            topology: Topology::Linear,
            declared,
            errors: vec![],
        }],
    };
    let hash: String = Sha256::digest(source.original_bytes())
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect();
    let records = raw
        .into_iter()
        .enumerate()
        .map(|(i, record)| {
            interpret(
                record,
                i + 1,
                format,
                options.get(&(i + 1)).cloned().unwrap_or_default(),
                &source.name,
                &hash,
            )
        })
        .collect();
    Ok(ImportPreview {
        source,
        format,
        source_sha256: hash,
        records,
    })
}

fn split_fasta(text: &str, fallback: &str, declared: Option<Molecule>) -> Result<Vec<RawRecord>> {
    let mut records = vec![];
    let mut current: Option<RawRecord> = None;
    for line in text.split_inclusive('\n') {
        if let Some(header) = line.trim_start().strip_prefix('>') {
            if let Some(record) = current.take() {
                records.push(record);
            }
            let header = header.trim();
            let mut parts = header.splitn(2, char::is_whitespace);
            let name = parts
                .next()
                .filter(|s| !s.is_empty())
                .unwrap_or(fallback)
                .to_string();
            let mut description = parts.next().unwrap_or("").trim().to_string();
            let circular = description
                .split_whitespace()
                .any(|s| s.eq_ignore_ascii_case("circular"));
            if let Some(first) = description.split_whitespace().next() {
                if first.eq_ignore_ascii_case("circular") || first.eq_ignore_ascii_case("linear") {
                    description = description[first.len()..].trim().to_string();
                }
            }
            current = Some(RawRecord {
                original: line.into(),
                sequence: String::new(),
                name,
                description,
                topology: if circular {
                    Topology::Circular
                } else {
                    Topology::Linear
                },
                declared,
                errors: vec![],
            });
        } else if let Some(record) = &mut current {
            record.original.push_str(line);
            if !line.trim_start().starts_with(';') {
                record.sequence.push_str(line);
            }
        } else if !line.trim().is_empty() && !line.trim_start().starts_with(';') {
            return Err(StorageError::Format(
                "FASTA sequence occurs before its first > header".into(),
            ));
        }
    }
    if let Some(record) = current {
        records.push(record);
    }
    if records.is_empty() {
        return Err(StorageError::Format("No FASTA record headers found".into()));
    }
    Ok(records)
}

fn split_genbank(text: &str, fallback: &str) -> Result<Vec<RawRecord>> {
    let mut blocks = vec![];
    let mut block = String::new();
    for line in text.split_inclusive('\n') {
        if line.starts_with("LOCUS") && !block.trim().is_empty() {
            blocks.push(std::mem::take(&mut block));
        }
        block.push_str(line);
    }
    if !block.trim().is_empty() {
        blocks.push(block);
    }
    let mut records = vec![];
    for original in blocks {
        let locus = original
            .lines()
            .find(|line| line.starts_with("LOCUS"))
            .ok_or_else(|| StorageError::Format("Missing GenBank LOCUS line".into()))?;
        let tokens: Vec<_> = locus.split_whitespace().collect();
        let declared = declared_genbank_molecule(&tokens);
        let mut sequence = String::new();
        let mut origin = false;
        let mut ended = false;
        let mut trailing_content = false;
        for line in original.lines() {
            if ended {
                trailing_content |= !line.trim().is_empty();
                continue;
            }
            if line.starts_with("ORIGIN") {
                origin = true;
                continue;
            }
            if line.trim() == "//" {
                ended = true;
                continue;
            }
            if origin {
                sequence.extend(line.chars().filter(|c| !c.is_ascii_digit()));
                sequence.push('\n');
            }
        }
        let mut errors = vec![];
        if trailing_content {
            errors.push("Unparsed content after the GenBank terminator; correct the source before importing".into());
        }
        if !origin {
            errors.push("Missing ORIGIN sequence".into());
        }
        if !ended {
            errors.push("Truncated GenBank record: missing // terminator".into());
        }
        let count = sequence.chars().filter(|c| !c.is_whitespace()).count();
        if let Some(expected) = tokens.get(2).and_then(|value| value.parse::<usize>().ok()) {
            if expected != count {
                errors.push(format!("LOCUS declares {expected} residues but ORIGIN contains {count}; correct the source before importing"));
            }
        }
        records.push(RawRecord {
            name: tokens.get(1).copied().unwrap_or(fallback).into(),
            description: String::new(),
            topology: if tokens.iter().any(|s| s.eq_ignore_ascii_case("circular")) {
                Topology::Circular
            } else {
                Topology::Linear
            },
            declared,
            original,
            sequence,
            errors,
        });
    }
    Ok(records)
}

fn interpret(
    raw: RawRecord,
    index: usize,
    format: ImportFormat,
    options: ImportOptions,
    source_name: &str,
    hash: &str,
) -> ImportRecord {
    let compact: String = raw
        .sequence
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| c.to_ascii_uppercase())
        .collect();
    let detected = raw.declared.unwrap_or_else(|| {
        if compact
            .chars()
            .any(|c| c.is_ascii_alphabetic() && c != 'U' && domain::iupac::dna_mask(c as u8) == 0)
        {
            Molecule::Protein
        } else if compact.contains('U') {
            Molecule::Rna
        } else {
            Molecule::Dna
        }
    });
    let interpreted = options.molecule.unwrap_or(detected);
    let mut errors = raw.errors;
    let mut changes = vec![];
    let mut needs_ack = false;
    if raw.sequence.chars().any(char::is_whitespace) {
        changes.push("Sequence whitespace removed".into());
    }
    if raw.sequence.chars().any(|c| c.is_ascii_lowercase()) {
        changes.push("Letters normalized to uppercase".into());
    }
    if format == ImportFormat::GenBank && has_omitted_genbank_metadata(&raw.original) {
        changes.push("Some GenBank metadata is retained in the original record but is not represented as editable native fields".into());
        needs_ack = true;
    }
    if format == ImportFormat::Fasta
        && raw
            .original
            .lines()
            .any(|line| line.trim_start().starts_with(';'))
    {
        changes.push("FASTA comments are retained in the original record, outside the native sequence fields".into());
        needs_ack = true;
    }
    if interpreted != detected {
        changes.push(format!(
            "Interpret {} source as {}",
            detected.label(),
            interpreted.label()
        ));
        needs_ack = true;
    }
    if interpreted != Molecule::Dna {
        errors.push(format!("{} input cannot be imported as a DNA document. Original residues are retained; use {} or explicitly choose a reviewed DNA conversion.", interpreted.label(), if interpreted == Molecule::Protein { "Biologics" } else { "RNA / Oligos" }));
    }
    if compact.is_empty() {
        errors.push("Record has no sequence".into());
    }
    let mut normalized = String::new();
    let mut uracils = 0;
    let mut gaps = 0;
    let mut omitted_errors = 0;
    let mut report_error = |errors: &mut Vec<String>, message: String| {
        if errors.len() < 16 {
            errors.push(message);
        } else {
            omitted_errors += 1;
        }
    };
    for (position, c) in raw.sequence.chars().enumerate() {
        if interpreted != Molecule::Dna {
            continue;
        }
        if c.is_whitespace() {
            continue;
        }
        let c = c.to_ascii_uppercase();
        if c == 'U' {
            uracils += 1;
            if options.convert_rna_to_dna && interpreted == Molecule::Dna {
                normalized.push('T');
            } else {
                report_error(
                    &mut errors,
                    format!(
                        "U at input character {} requires an explicit RNA→DNA conversion",
                        position + 1
                    ),
                );
            }
        } else if c == '-' || c == '.' {
            gaps += 1;
            if !options.remove_gaps {
                report_error(
                    &mut errors,
                    format!(
                        "Gap at input character {} requires reviewed gap removal",
                        position + 1
                    ),
                );
            }
            if format == ImportFormat::GenBank {
                report_error(&mut errors, "Cannot remove gaps from GenBank without remapping annotations; correct the source".into());
            }
        } else if c.is_ascii() && domain::iupac::dna_mask(c as u8) != 0 {
            normalized.push(c);
        } else if interpreted == Molecule::Dna {
            report_error(
                &mut errors,
                format!(
                    "Unsupported symbol {c:?} at input character {}; nothing has been discarded",
                    position + 1
                ),
            );
        }
    }
    if omitted_errors > 0 {
        errors.push(format!(
            "{omitted_errors} additional input issues; the complete original input is retained"
        ));
    }
    if uracils > 0 && options.convert_rna_to_dna {
        changes.push(format!("Convert {uracils} U residues to T"));
        needs_ack = true;
    }
    if gaps > 0 && options.remove_gaps {
        changes.push(format!("Remove {gaps} gap characters"));
        needs_ack = true;
    }
    if normalized.is_empty() && interpreted == Molecule::Dna {
        errors.push("No DNA residues remain".into());
    }
    let topology = options.topology.unwrap_or(raw.topology);
    if options
        .topology
        .is_some_and(|topology| topology != raw.topology)
    {
        changes.push("Topology overridden by review choice".into());
        needs_ack = true;
    }
    let mut document = None;
    if interpreted == Molecule::Dna && errors.is_empty() {
        let candidate = if format == ImportFormat::GenBank {
            let adjusted = raw
                .original
                .lines()
                .map(|line| {
                    if line.starts_with("LOCUS") && raw.declared == Some(Molecule::Rna) {
                        line.split_whitespace()
                            .enumerate()
                            .map(|(index, s)| {
                                if index >= 3 && s.to_ascii_uppercase().ends_with("RNA") {
                                    "DNA"
                                } else {
                                    s
                                }
                            })
                            .collect::<Vec<_>>()
                            .join(" ")
                    } else {
                        line.to_string()
                    }
                })
                .collect::<Vec<_>>()
                .join("\n");
            let mut origin = false;
            let adjusted = adjusted
                .lines()
                .map(|line| {
                    if line.starts_with("ORIGIN") {
                        origin = true;
                    }
                    if origin && options.convert_rna_to_dna {
                        line.replace('u', "t").replace('U', "T")
                    } else {
                        line.to_string()
                    }
                })
                .collect::<Vec<_>>()
                .join("\n");
            crate::genbank::parse(adjusted.as_bytes(), &raw.name).map_err(|e| e.to_string())
        } else {
            Ok(Document::new(
                raw.name.clone(),
                normalized.as_bytes().to_vec(),
                topology,
            ))
        };
        match candidate {
            Ok(mut doc) => {
                doc.sequence.topology = topology;
                if format != ImportFormat::GenBank {
                    doc.description = raw.description;
                }
                if !doc.sequence.is_circular()
                    && doc.features.iter().flat_map(|f| &f.segments).any(|r| {
                        r.start
                            .checked_add(r.len)
                            .is_none_or(|end| end > doc.sequence.len())
                    })
                {
                    errors.push("Linear topology conflicts with annotation coordinates".into());
                } else {
                    doc.import_source = Some(domain::document::ImportProvenance {
                        source_name: source_name.into(),
                        source_sha256: hash.into(),
                        format: format.label().into(),
                        record_index: index,
                        original_record: raw.original.clone(),
                        transformations: changes.clone(),
                        interpretation: "DNA".into(),
                        target: None,
                    });
                    document = Some(doc);
                }
            }
            Err(error) => errors.push(error),
        }
    }
    errors.dedup();
    ImportRecord {
        index,
        name: raw.name,
        original_record: raw.original,
        original_sequence: raw.sequence,
        detected,
        interpreted,
        transformations: changes,
        requires_acknowledgment: needs_ack,
        errors,
        document,
    }
}

/// Safe adapter for callers that cannot present a review surface.
pub fn strict_dna(text: &str) -> Result<Vec<u8>> {
    let mut out = Vec::new();
    for (position, character) in text.chars().enumerate() {
        if character.is_whitespace() {
            continue;
        }
        let c = character.to_ascii_uppercase();
        if !c.is_ascii() || c == 'U' || domain::iupac::dna_mask(c as u8) == 0 {
            return Err(StorageError::Format(format!("Symbol {character:?} at character {} requires import review; input was not cleaned", position+1)));
        }
        out.push(c as u8);
    }
    if out.is_empty() {
        return Err(StorageError::Format("No DNA sequence found".into()));
    }
    Ok(out)
}
