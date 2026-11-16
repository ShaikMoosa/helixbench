use serde::{Deserialize, Serialize};

use crate::feature::Feature;
use crate::primer::Primer;
use crate::sequence::{clean_sequence, DnaSequence, Range, Topology};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct StrandMask(pub u8);

impl StrandMask {
    pub const TOP: u8 = 1;
    pub const BOTTOM: u8 = 2;
    pub const BOTH: u8 = 3;

    pub fn has_top(self) -> bool {
        self.0 & Self::TOP != 0
    }
    pub fn has_bottom(self) -> bool {
        self.0 & Self::BOTTOM != 0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ColorRegion {
    pub range: Range,
    pub strands: StrandMask,
    pub color: [u8; 3],
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Version {
    pub label: String,
    /// Unix seconds when the snapshot was taken.
    pub created: u64,
    pub doc: Document,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PasteTarget {
    pub source: crate::SourceRef,
    pub selection: Option<Range>,
    pub replace_selection: bool,
    pub insertion_position: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImportProvenance {
    pub source_name: String,
    pub source_sha256: String,
    pub format: String,
    pub record_index: usize,
    pub original_record: String,
    pub transformations: Vec<String>,
    pub interpretation: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<PasteTarget>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Document {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pcr_source: Option<crate::primer_provenance::PcrProvenance>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub primer_pairs: Vec<crate::primer_provenance::PrimerPairProvenance>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub import_source: Option<ImportProvenance>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub import_history: Vec<ImportProvenance>,
    /// Missing IDs are nil only while reading a legacy document; storage migrates them.
    #[serde(default)]
    pub id: crate::identity::DocumentId,
    #[serde(default)]
    pub revision_id: crate::identity::RevisionId,
    pub name: String,
    pub description: String,
    pub sequence: DnaSequence,
    pub features: Vec<Feature>,
    pub primers: Vec<Primer>,
    pub chosen_enzymes: Vec<String>,
    pub color_regions: Vec<ColorRegion>,
    /// NCBI genetic code id used for ORFs and features without an override.
    pub default_genetic_code: u8,
    /// Saved snapshots (newest last); each snapshot's own `versions` is empty.
    #[serde(default)]
    pub versions: Vec<Version>,
    /// Free-form organization tags.
    #[serde(default)]
    pub tags: Vec<String>,
    next_id: u64,
}

impl Document {
    pub fn new(name: String, bases: Vec<u8>, topology: Topology) -> Self {
        Self {
            pcr_source: None,
            primer_pairs: Vec::new(),
            import_source: None,
            import_history: Vec::new(),
            id: crate::identity::DocumentId::new(),
            revision_id: crate::identity::RevisionId::new(),
            name,
            description: String::new(),
            sequence: DnaSequence::new(bases, topology),
            features: Vec::new(),
            primers: Vec::new(),
            chosen_enzymes: Vec::new(),
            color_regions: Vec::new(),
            default_genetic_code: 1,
            versions: Vec::new(),
            tags: Vec::new(),
            next_id: 1,
        }
    }

    pub fn from_sequence_text(name: String, text: &str, topology: Topology) -> Self {
        Self::new(name, clean_sequence(text), topology)
    }

    pub fn alloc_id(&mut self) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    pub fn source_ref(&self) -> crate::identity::SourceRef {
        crate::identity::SourceRef {
            document: self.id,
            revision: self.revision_id,
        }
    }

    /// Application mutation boundary; undo restores the prior revision verbatim.
    pub fn advance_revision(&mut self) {
        self.revision_id = crate::identity::RevisionId::new();
    }

    /// Create another scientific record, retaining annotations and history content.
    /// A normal clone is deliberately not a copy: it is used by undo and persistence.
    pub fn independent_copy(&self, name: String) -> Self {
        let mut copy = self.clone();
        copy.name = name;
        copy.id = crate::identity::DocumentId::new();
        copy.advance_revision();
        for version in &mut copy.versions {
            version.doc.id = copy.id;
            version.doc.advance_revision();
        }
        copy
    }

    /// Snapshot the current state as a named version (newest last; retained until an explicit deletion policy exists).
    pub fn save_version(&mut self, label: String, created: u64) {
        let mut doc = self.clone();
        doc.versions = Vec::new();
        self.versions.push(Version {
            label,
            created,
            doc,
        });
    }

    /// Restore a snapshot by index, preserving the version list and id counter.
    pub fn restore_version(&mut self, idx: usize) -> bool {
        let Some(v) = self.versions.get(idx) else {
            return false;
        };
        let snap = v.doc.clone();
        self.restore_snapshot(&snap);
        true
    }

    /// Restore content while preserving this record's identity, checkpoints and ID allocator.
    pub fn restore_snapshot(&mut self, snapshot: &Document) {
        let versions = std::mem::take(&mut self.versions);
        let next = self.next_id;
        let identity = self.id;
        *self = snapshot.clone();
        self.id = identity;
        self.advance_revision();
        self.versions = versions;
        self.next_id = next.max(self.next_id);
    }

    pub fn add_feature(&mut self, mut feature: Feature) -> u64 {
        let id = self.alloc_id();
        feature.id = id;
        self.features.push(feature);
        id
    }

    pub fn feature(&self, id: u64) -> Option<&Feature> {
        self.features.iter().find(|f| f.id == id)
    }

    pub fn feature_mut(&mut self, id: u64) -> Option<&mut Feature> {
        self.features.iter_mut().find(|f| f.id == id)
    }

    pub fn delete_feature(&mut self, id: u64) -> Option<Feature> {
        if let Some(pos) = self.features.iter().position(|f| f.id == id) {
            Some(self.features.remove(pos))
        } else {
            None
        }
    }

    pub fn add_primer(&mut self, mut primer: Primer) -> u64 {
        let id = self.alloc_id();
        primer.id = id;
        self.primers.push(primer);
        id
    }

    pub fn primer(&self, id: u64) -> Option<&Primer> {
        self.primers.iter().find(|p| p.id == id)
    }

    pub fn primer_mut(&mut self, id: u64) -> Option<&mut Primer> {
        self.primers.iter_mut().find(|p| p.id == id)
    }

    pub fn delete_primer(&mut self, id: u64) -> Option<Primer> {
        if let Some(pos) = self.primers.iter().position(|p| p.id == id) {
            Some(self.primers.remove(pos))
        } else {
            None
        }
    }

    /// Rotate a circular document so position `offset` becomes position 0.
    pub fn rotate(&mut self, offset: usize) {
        let l = self.sequence.len();
        if l == 0 || !self.sequence.is_circular() {
            return;
        }
        let off = offset % l;
        if off == 0 {
            return;
        }
        self.sequence.rotate(off);
        for f in &mut self.features {
            for seg in &mut f.segments {
                seg.start = (seg.start + l - off) % l;
            }
            f.cleavage_arrows = f
                .cleavage_arrows
                .iter()
                .map(|&p| (p + l - off) % l)
                .collect();
        }
        for c in &mut self.color_regions {
            c.range.start = (c.range.start + l - off) % l;
        }
    }

    /// Delete a region, remapping annotations. Wrap-around selections on
    /// circular documents are rotated to the origin first.
    pub fn delete_range(&mut self, range: Range) {
        let l = self.sequence.len();
        if l == 0 || range.len == 0 {
            return;
        }
        let range = range.normalized(l);
        if self.sequence.is_circular() && range.end() > l {
            self.rotate(range.start);
            self.delete_range_linear(Range::new(0, range.len));
            return;
        }
        self.delete_range_linear(range);
    }

    fn delete_range_linear(&mut self, range: Range) {
        let l = self.sequence.len();
        let removed = range.len.min(l - range.start);
        if removed == 0 {
            return;
        }
        self.sequence
            .bases
            .drain(range.start..range.start + removed);
        self.remap_annotations(range.start, removed, 0);
    }

    /// Insert bases at a position, remapping annotations.
    pub fn insert_bases(&mut self, pos: usize, bases: &[u8]) {
        let l = self.sequence.len();
        if bases.is_empty() {
            return;
        }
        let pos = if self.sequence.is_circular() && l > 0 {
            pos % l
        } else {
            pos.min(l)
        };
        self.sequence.bases.splice(pos..pos, bases.iter().copied());
        self.remap_annotations(pos, 0, bases.len());
    }

    /// Replace a region with new bases (delete + insert in one remap).
    pub fn replace_range(&mut self, range: Range, bases: &[u8]) {
        let l = self.sequence.len();
        if l == 0 {
            self.sequence.bases.extend_from_slice(bases);
            return;
        }
        let range = range.normalized(l);
        if self.sequence.is_circular() && range.end() > l {
            self.rotate(range.start);
            let removed = range.len.min(self.sequence.len());
            self.sequence
                .bases
                .splice(0..removed, bases.iter().copied());
            self.remap_annotations(0, removed, bases.len());
            return;
        }
        let removed = range.len.min(l.saturating_sub(range.start));
        self.sequence
            .bases
            .splice(range.start..range.start + removed, bases.iter().copied());
        self.remap_annotations(range.start, removed, bases.len());
    }

    /// Remap annotation coordinates through an edit at `edit_start` that
    /// removes `removed` bases and inserts `added` bases. Positions are mapped
    /// individually so circular wrap-around segments survive edits correctly.
    fn remap_annotations(&mut self, edit_start: usize, removed: usize, added: usize) {
        let old_l = self.sequence.len() + removed - added;
        let new_l = self.sequence.len();
        let circular = self.sequence.is_circular();
        let edit_end = edit_start + removed;
        let map_pos = |p: usize| -> usize {
            let p = if circular && old_l > 0 { p % old_l } else { p };
            if p < edit_start {
                p
            } else if p >= edit_end {
                p - removed + added
            } else {
                // Inside the removed region (or at its start): collapse to the
                // edit point so replacements stay inside spanning features.
                edit_start
            }
        };
        let map_range = |r: Range| -> Option<Range> {
            if r.len == 0 {
                return None;
            }
            if circular && r.len >= old_l && new_l > 0 {
                return Some(Range::new(0, new_l));
            }
            let ns = map_pos(r.start);
            let ne = map_pos(r.end());
            let len = if circular && new_l > 0 {
                let raw = (ne + new_l - ns) % new_l;
                // A segment that lost bases to the edit shrinks; one that did
                // not keeps its length (raw == 0 also means fully deleted).
                if raw == 0 {
                    0
                } else {
                    raw.min(r.len).min(new_l)
                }
            } else {
                ne.saturating_sub(ns)
            };
            if len == 0 {
                None
            } else {
                Some(Range::new(ns, len))
            }
        };
        for f in &mut self.features {
            f.segments = f.segments.iter().filter_map(|&r| map_range(r)).collect();
            f.cleavage_arrows = f
                .cleavage_arrows
                .iter()
                .filter(|&&p| {
                    let p = if circular && old_l > 0 { p % old_l } else { p };
                    p < edit_start || p >= edit_end
                })
                .map(|&p| map_pos(p))
                .collect();
        }
        self.features.retain(|f| !f.segments.is_empty());
        self.color_regions = self
            .color_regions
            .iter()
            .filter_map(|c| {
                map_range(c.range).map(|r| ColorRegion {
                    range: r,
                    strands: c.strands,
                    color: c.color,
                })
            })
            .collect();
    }

    pub fn set_topology(&mut self, topology: Topology) {
        self.sequence.topology = topology;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::feature::Direction;

    fn doc_with_feature() -> Document {
        let mut d = Document::new("t".into(), b"AAAACCCCGGGGTTTT".to_vec(), Topology::Linear);
        let mut f = Feature::new(
            0,
            "mid".into(),
            "misc_feature".into(),
            vec![Range::new(4, 8)],
        );
        f.direction = Direction::Forward;
        d.add_feature(f);
        d
    }

    #[test]
    fn delete_shifts_downstream_feature() {
        let mut d = doc_with_feature();
        d.delete_range(Range::new(0, 2));
        assert_eq!(d.sequence.bases, b"AACCCCGGGGTTTT".to_vec());
        assert_eq!(d.features[0].segments[0], Range::new(2, 8));
    }

    #[test]
    fn delete_inside_feature_shrinks() {
        let mut d = doc_with_feature();
        d.delete_range(Range::new(6, 2));
        assert_eq!(d.features[0].segments[0], Range::new(4, 6));
    }

    #[test]
    fn delete_whole_feature_removes_it() {
        let mut d = doc_with_feature();
        d.delete_range(Range::new(3, 10));
        assert!(d.features.is_empty());
    }

    #[test]
    fn insert_shifts() {
        let mut d = doc_with_feature();
        d.insert_bases(2, b"TT");
        assert_eq!(d.features[0].segments[0], Range::new(6, 8));
        assert_eq!(d.sequence.len(), 18);
    }

    #[test]
    fn circular_delete_wrapping() {
        let mut d = Document::new("c".into(), b"AAAACCCCGGGGTTTT".to_vec(), Topology::Circular);
        let mut f = Feature::new(
            0,
            "origin".into(),
            "misc_feature".into(),
            vec![Range::new(14, 4)],
        );
        f.direction = Direction::Forward;
        d.add_feature(f);
        // delete 15..19 (wraps): removes TTT + A
        d.delete_range(Range::new(15, 4));
        assert_eq!(d.sequence.len(), 12);
        assert_eq!(d.sequence.bases, b"ACCCCGGGGTTT".to_vec());
        assert_eq!(d.features[0].segments[0], Range::new(11, 1));
    }

    #[test]
    fn replace_range() {
        let mut d = doc_with_feature();
        d.replace_range(Range::new(4, 4), b"TT");
        assert_eq!(d.sequence.bases, b"AAAATTGGGGTTTT".to_vec());
        assert_eq!(d.features[0].segments[0], Range::new(4, 6));
    }

    #[test]
    fn version_snapshot_and_restore() {
        let mut d = doc_with_feature();
        let original = d.sequence.bases.clone();
        d.save_version("before edit".into(), 1);
        d.replace_range(Range::new(0, 4), b"CCCC");
        assert_ne!(d.sequence.bases, original);
        assert_eq!(d.versions.len(), 1);
        assert!(d.versions[0].doc.versions.is_empty());
        assert!(d.restore_version(0));
        assert_eq!(d.sequence.bases, original);
        assert_eq!(d.versions.len(), 1); // list preserved
        assert!(!d.restore_version(9));
    }
}
