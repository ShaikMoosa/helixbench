//! Chemistry: a local SMILES subset parser, molecular formula / weight, simple
//! property counts, substructure search, and reaction / salt-solvate models.
//! Pure Rust, offline. The SMILES parser covers the organic subset, branches,
//! ring closures, aromatic atoms and basic bracket atoms — not full stereochemistry.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Atom {
    pub element: String,
    pub aromatic: bool,
    pub charge: i32,
    pub explicit_h: Option<usize>,
}

/// Bond order: 1 single, 2 double, 3 triple, 4 aromatic.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Bond {
    pub a: usize,
    pub b: usize,
    pub order: u8,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Molecule {
    pub atoms: Vec<Atom>,
    pub bonds: Vec<Bond>,
}

fn default_valence(el: &str) -> usize {
    match el {
        "C" => 4,
        "N" => 3,
        "O" => 2,
        "S" => 2,
        "P" => 3,
        "B" => 3,
        "F" | "Cl" | "Br" | "I" => 1,
        _ => 0,
    }
}

pub fn atomic_weight(el: &str) -> f32 {
    match el {
        "H" => 1.008,
        "B" => 10.81,
        "C" => 12.011,
        "N" => 14.007,
        "O" => 15.999,
        "F" => 18.998,
        "Na" => 22.990,
        "P" => 30.974,
        "S" => 32.06,
        "Cl" => 35.45,
        "K" => 39.098,
        "Br" => 79.904,
        "I" => 126.904,
        _ => 0.0,
    }
}

impl Molecule {
    pub fn parse(smiles: &str) -> Result<Molecule, String> {
        let chars: Vec<char> = smiles.trim().chars().collect();
        let mut mol = Molecule::default();
        let mut stack: Vec<usize> = Vec::new();
        let mut rings: std::collections::HashMap<usize, (usize, u8)> =
            std::collections::HashMap::new();
        let mut prev: Option<usize> = None;
        let mut pending_bond: u8 = 0; // 0 = default
        let mut i = 0usize;
        while i < chars.len() {
            let c = chars[i];
            match c {
                '(' => {
                    if let Some(p) = prev {
                        stack.push(p);
                    } else {
                        return Err("branch before any atom".into());
                    }
                }
                ')' => {
                    prev = stack.pop().or(prev);
                }
                '-' => pending_bond = 1,
                '=' => pending_bond = 2,
                '#' => pending_bond = 3,
                ':' => pending_bond = 4,
                '/' | '\\' => {}
                '.' => {
                    prev = None;
                    pending_bond = 0;
                }
                '0'..='9' => {
                    let d = c as usize - '0' as usize;
                    let Some(p) = prev else {
                        return Err("ring bond before any atom".into());
                    };
                    if let Some((other, order)) = rings.remove(&d) {
                        let o = if pending_bond != 0 {
                            pending_bond
                        } else if order != 0 {
                            order
                        } else if mol.atoms[other].aromatic && mol.atoms[p].aromatic {
                            4
                        } else {
                            1
                        };
                        mol.bonds.push(Bond {
                            a: other,
                            b: p,
                            order: o,
                        });
                    } else {
                        rings.insert(d, (p, pending_bond));
                    }
                    pending_bond = 0;
                }
                '[' => {
                    let mut j = i + 1;
                    let mut content = String::new();
                    while j < chars.len() && chars[j] != ']' {
                        content.push(chars[j]);
                        j += 1;
                    }
                    if j >= chars.len() {
                        return Err("unclosed bracket atom".into());
                    }
                    let atom = parse_bracket(&content)?;
                    let idx = mol.atoms.len();
                    mol.atoms.push(atom);
                    if let Some(p) = prev {
                        let o = if pending_bond != 0 { pending_bond } else { 1 };
                        mol.bonds.push(Bond {
                            a: p,
                            b: idx,
                            order: o,
                        });
                    }
                    prev = Some(idx);
                    pending_bond = 0;
                    i = j;
                }
                _ => {
                    // organic subset (two-letter first)
                    let two: String = chars[i..(i + 2).min(chars.len())].iter().collect();
                    let (el, aromatic, len) = if two == "Cl" || two == "Br" {
                        (two.clone(), false, 2)
                    } else {
                        let up = c.to_ascii_uppercase().to_string();
                        let arom = c.is_ascii_lowercase() && "cnops".contains(c);
                        if !arom && !c.is_ascii_uppercase() {
                            return Err(format!("unsupported SMILES char '{c}'"));
                        }
                        if !"BCNOPSFI".contains(up.as_str()) && !arom {
                            return Err(format!("unsupported element '{up}'"));
                        }
                        (up, arom, 1)
                    };
                    let idx = mol.atoms.len();
                    mol.atoms.push(Atom {
                        element: el,
                        aromatic,
                        charge: 0,
                        explicit_h: None,
                    });
                    if let Some(p) = prev {
                        let o = if pending_bond != 0 {
                            pending_bond
                        } else if mol.atoms[p].aromatic && aromatic {
                            4
                        } else {
                            1
                        };
                        mol.bonds.push(Bond {
                            a: p,
                            b: idx,
                            order: o,
                        });
                    }
                    prev = Some(idx);
                    pending_bond = 0;
                    i += len - 1;
                }
            }
            i += 1;
        }
        if !rings.is_empty() {
            return Err("unclosed ring bond".into());
        }
        if mol.atoms.is_empty() {
            return Err("empty SMILES".into());
        }
        Ok(mol)
    }

    fn bond_order_sum(&self, idx: usize) -> usize {
        let mut sum = 0usize;
        let mut has_arom = false;
        for b in &self.bonds {
            if b.a == idx || b.b == idx {
                sum += if b.order == 4 { 1 } else { b.order as usize };
                if b.order == 4 {
                    has_arom = true;
                }
            }
        }
        if has_arom && self.atoms[idx].aromatic {
            sum += 1;
        }
        sum
    }

    pub fn implicit_h(&self, idx: usize) -> usize {
        let a = &self.atoms[idx];
        if let Some(h) = a.explicit_h {
            return h;
        }
        let v = default_valence(&a.element);
        v.saturating_sub(self.bond_order_sum(idx))
    }

    /// Hill-order molecular formula including implicit hydrogens.
    pub fn formula(&self) -> String {
        let mut counts: std::collections::BTreeMap<String, usize> =
            std::collections::BTreeMap::new();
        let mut h = 0usize;
        for (i, a) in self.atoms.iter().enumerate() {
            *counts.entry(a.element.clone()).or_insert(0) += 1;
            h += self.implicit_h(i);
        }
        if h > 0 {
            counts.insert("H".into(), h);
        }
        let has_c = counts.contains_key("C");
        let mut order: Vec<String> = Vec::new();
        if has_c {
            order.push("C".into());
            if counts.contains_key("H") {
                order.push("H".into());
            }
            for k in counts.keys() {
                if k != "C" && k != "H" {
                    order.push(k.clone());
                }
            }
        } else {
            order = counts.keys().cloned().collect();
        }
        let mut out = String::new();
        for k in order {
            let n = counts[&k];
            out.push_str(&k);
            if n > 1 {
                out.push_str(&n.to_string());
            }
        }
        out
    }

    pub fn molecular_weight(&self) -> f32 {
        let mut mw = 0.0;
        for (i, a) in self.atoms.iter().enumerate() {
            mw += atomic_weight(&a.element);
            mw += atomic_weight("H") * self.implicit_h(i) as f32;
        }
        mw
    }

    pub fn h_bond_donors(&self) -> usize {
        self.atoms
            .iter()
            .enumerate()
            .filter(|(i, a)| (a.element == "N" || a.element == "O") && self.implicit_h(*i) > 0)
            .count()
    }

    pub fn h_bond_acceptors(&self) -> usize {
        self.atoms
            .iter()
            .filter(|a| a.element == "N" || a.element == "O")
            .count()
    }

    fn neighbors(&self, idx: usize) -> Vec<(usize, u8)> {
        self.bonds
            .iter()
            .filter_map(|b| {
                if b.a == idx {
                    Some((b.b, b.order))
                } else if b.b == idx {
                    Some((b.a, b.order))
                } else {
                    None
                }
            })
            .collect()
    }

    /// Substructure search: does `self` contain `query` as a subgraph?
    pub fn contains(&self, query: &Molecule) -> bool {
        if query.atoms.is_empty() {
            return true;
        }
        if query.atoms.len() > self.atoms.len() {
            return false;
        }
        let mut map: Vec<Option<usize>> = vec![None; query.atoms.len()];
        let mut used: Vec<bool> = vec![false; self.atoms.len()];
        self.match_from(query, 0, &mut map, &mut used)
    }

    fn match_from(
        &self,
        q: &Molecule,
        qi: usize,
        map: &mut Vec<Option<usize>>,
        used: &mut Vec<bool>,
    ) -> bool {
        if qi == q.atoms.len() {
            return true;
        }
        for ti in 0..self.atoms.len() {
            if used[ti] {
                continue;
            }
            let qa = &q.atoms[qi];
            let ta = &self.atoms[ti];
            if qa.element != ta.element || qa.aromatic != ta.aromatic {
                continue;
            }
            // check bonds to already-mapped query atoms
            let ok = q.bonds.iter().filter(|b| b.a == qi || b.b == qi).all(|b| {
                let (other, order) = if b.a == qi {
                    (b.b, b.order)
                } else {
                    (b.a, b.order)
                };
                match map[other] {
                    None => true,
                    Some(mt) => self
                        .neighbors(ti)
                        .iter()
                        .any(|(n, o)| *n == mt && *o == order),
                }
            });
            if !ok {
                continue;
            }
            map[qi] = Some(ti);
            used[ti] = true;
            if self.match_from(q, qi + 1, map, used) {
                return true;
            }
            map[qi] = None;
            used[ti] = false;
        }
        false
    }
}

fn parse_bracket(content: &str) -> Result<Atom, String> {
    let mut chars = content.chars().peekable();
    let mut element = String::new();
    if let Some(c) = chars.next() {
        element.push(c);
        if let Some(&n) = chars.peek() {
            if n.is_ascii_lowercase() {
                element.push(n);
                chars.next();
            }
        }
    } else {
        return Err("empty bracket atom".into());
    }
    let mut explicit_h = None;
    let mut charge = 0i32;
    while let Some(c) = chars.next() {
        match c {
            'H' => {
                let mut n = 0usize;
                let mut any = false;
                while let Some(&d) = chars.peek() {
                    if d.is_ascii_digit() {
                        n = n * 10 + (d as usize - '0' as usize);
                        any = true;
                        chars.next();
                    } else {
                        break;
                    }
                }
                explicit_h = Some(if any { n } else { 1 });
            }
            '+' => {
                let mut n = 0i32;
                let mut any = false;
                while let Some(&d) = chars.peek() {
                    if d.is_ascii_digit() {
                        n = n * 10 + (d as i32 - '0' as i32);
                        any = true;
                        chars.next();
                    } else {
                        break;
                    }
                }
                charge = if any { n } else { 1 };
            }
            '-' => {
                let mut n = 0i32;
                let mut any = false;
                while let Some(&d) = chars.peek() {
                    if d.is_ascii_digit() {
                        n = n * 10 + (d as i32 - '0' as i32);
                        any = true;
                        chars.next();
                    } else {
                        break;
                    }
                }
                charge = -(if any { n } else { 1 });
            }
            _ => {}
        }
    }
    let aromatic = element
        .chars()
        .next()
        .is_some_and(|c| c.is_ascii_lowercase());
    Ok(Atom {
        element: element.to_ascii_uppercase(),
        aromatic,
        charge,
        explicit_h,
    })
}

/// A reusable salt / solvate form.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Form {
    pub id: u64,
    pub name: String,
    pub formula: String,
    pub mw: f32,
}

/// Built-in salt & solvate library.
pub fn salt_solvate_library() -> Vec<Form> {
    let lib = [
        ("Hydrochloride (HCl)", "HCl", 36.46),
        ("Sodium (Na)", "Na", 22.99),
        ("Potassium (K)", "K", 39.10),
        ("Trifluoroacetate (TFA)", "C2HF3O2", 114.02),
        ("Sulfate (H2SO4)", "H2SO4", 98.08),
        ("Water (hydrate)", "H2O", 18.02),
        ("Ethanol (solvate)", "C2H6O", 46.07),
        ("Methanol (solvate)", "CH4O", 32.04),
    ];
    lib.iter()
        .enumerate()
        .map(|(i, (n, f, mw))| Form {
            id: i as u64 + 1,
            name: (*n).into(),
            formula: (*f).into(),
            mw: *mw,
        })
        .collect()
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Compound {
    pub id: u64,
    pub name: String,
    pub smiles: String,
    pub formula: String,
    pub mw: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReactionComponent {
    pub compound: u64,
    pub equivalents: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Reaction {
    pub id: u64,
    pub name: String,
    pub reactants: Vec<ReactionComponent>,
    pub products: Vec<ReactionComponent>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ethanol_formula_and_mw() {
        let m = Molecule::parse("CCO").unwrap();
        assert_eq!(m.formula(), "C2H6O");
        assert!((m.molecular_weight() - 46.07).abs() < 0.05);
        assert_eq!(m.h_bond_donors(), 1);
    }

    #[test]
    fn benzene_aromatic() {
        let m = Molecule::parse("c1ccccc1").unwrap();
        assert_eq!(m.atoms.len(), 6);
        assert_eq!(m.formula(), "C6H6");
        assert!(m.bonds.iter().all(|b| b.order == 4));
    }

    #[test]
    fn aspirin_substructure() {
        let aspirin = Molecule::parse("CC(=O)Oc1ccccc1C(=O)O").unwrap();
        let benzene = Molecule::parse("c1ccccc1").unwrap();
        let carboxyl = Molecule::parse("C(=O)O").unwrap();
        assert!(aspirin.contains(&benzene));
        assert!(aspirin.contains(&carboxyl));
        assert!(!benzene.contains(&aspirin));
        assert_eq!(aspirin.formula(), "C9H8O4");
    }

    #[test]
    fn branches_and_charges() {
        let m = Molecule::parse("CC(C)C").unwrap();
        assert_eq!(m.formula(), "C4H10");
        let na = Molecule::parse("[Na+]").unwrap();
        assert_eq!(na.atoms[0].charge, 1);
        assert!(Molecule::parse("C1CC").is_err()); // unclosed ring
    }

    #[test]
    fn salt_library_present() {
        let lib = salt_solvate_library();
        assert!(lib.iter().any(|f| f.name.contains("HCl")));
    }
}
