//! Insights-lite: local datasets + dashboard blocks (table / bar / line) with
//! group-by aggregation and CSV export. Operates on in-memory datasets built
//! from the local stores (registry / inventory / tasks / entries).

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Dataset {
    pub name: String,
    pub columns: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Chart {
    Table,
    Bar,
    Line,
}

impl Chart {
    pub fn label(self) -> &'static str {
        match self {
            Chart::Table => "Table",
            Chart::Bar => "Bar",
            Chart::Line => "Line",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Block {
    pub name: String,
    pub source: String,
    pub chart: Chart,
    pub group_by: Option<String>,
}

/// Count rows grouped by a column's value.
pub fn group_count(ds: &Dataset, col: &str) -> Vec<(String, usize)> {
    let Some(idx) = ds.columns.iter().position(|c| c == col) else {
        return Vec::new();
    };
    let mut map: Vec<(String, usize)> = Vec::new();
    for row in &ds.rows {
        let key = row.get(idx).cloned().unwrap_or_default();
        if let Some(e) = map.iter_mut().find(|(k, _)| *k == key) {
            e.1 += 1;
        } else {
            map.push((key, 1));
        }
    }
    map.sort_by_key(|(_, v)| std::cmp::Reverse(*v));
    map
}

/// Chart series for a block: group counts, or row count if no group_by.
pub fn series(block: &Block, ds: &Dataset) -> Vec<(String, f32)> {
    match &block.group_by {
        Some(col) => group_count(ds, col)
            .into_iter()
            .map(|(k, v)| (k, v as f32))
            .collect(),
        None => vec![(ds.name.clone(), ds.rows.len() as f32)],
    }
}

pub fn to_csv(ds: &Dataset) -> String {
    let mut out = ds.columns.join(",");
    out.push('\n');
    for row in &ds.rows {
        out.push_str(
            &row.iter()
                .map(|c| {
                    if c.contains(',') || c.contains('"') {
                        format!("\"{}\"", c.replace('"', "\"\""))
                    } else {
                        c.clone()
                    }
                })
                .collect::<Vec<_>>()
                .join(","),
        );
        out.push('\n');
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ds() -> Dataset {
        Dataset {
            name: "registry".into(),
            columns: vec!["Name".into(), "Kind".into()],
            rows: vec![
                vec!["p1".into(), "Plasmid".into()],
                vec!["p2".into(), "Plasmid".into()],
                vec!["s1".into(), "Strain".into()],
            ],
        }
    }

    #[test]
    fn group_count_and_series() {
        let d = ds();
        let g = group_count(&d, "Kind");
        assert_eq!(g[0], ("Plasmid".into(), 2));
        let b = Block {
            name: "b".into(),
            source: "registry".into(),
            chart: Chart::Bar,
            group_by: Some("Kind".into()),
        };
        let s = series(&b, &d);
        assert_eq!(s.len(), 2);
    }

    #[test]
    fn csv_export() {
        let csv = to_csv(&ds());
        assert!(csv.starts_with("Name,Kind\n"));
        assert!(csv.contains("p1,Plasmid"));
    }
}
