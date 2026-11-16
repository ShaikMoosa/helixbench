//! Validated selection in top-strand coordinates; strand is an independent view attribute.
use crate::state::AppState;
use domain::{Range, SourceRef, Strand};
pub fn from_inclusive(app: &AppState, start: usize, end: usize) -> Result<Range, String> {
    let len = app.doc.sequence.len();
    if start == 0 || end == 0 || start > len || end > len {
        return Err(format!("Coordinates must be between 1 and {len}"));
    }
    if end < start && !app.doc.sequence.is_circular() {
        return Err("Origin-spanning ranges require circular DNA".into());
    }
    Ok(Range::new(
        start - 1,
        if end >= start {
            end - start + 1
        } else {
            len - start + 1 + end
        },
    ))
}
pub fn segments(app: &AppState, range: Range) -> Result<Vec<Range>, String> {
    if !crate::shell::valid_range(app, range) {
        return Err("Select a nonempty range within this sequence".into());
    }
    let first = range.len.min(app.doc.sequence.len() - range.start);
    let mut out = vec![Range::new(range.start, first)];
    if first < range.len {
        out.push(Range::new(0, range.len - first));
    }
    Ok(out)
}
pub fn apply(
    app: &mut AppState,
    source: SourceRef,
    range: Option<Range>,
    strand: Strand,
) -> Result<(), String> {
    if source != app.doc.source_ref() {
        return Err("The sequence changed. Select the range again".into());
    }
    if app.dialog.is_some() || app.navigation.pending.is_some() || app.workspace_search.open {
        return Err("Return to the sequence and finish the open form first".into());
    }
    if let Some(r) = range {
        segments(app, r)?;
    }
    app.selection = range;
    app.selection_strand = if range.is_some() { strand } else { Strand::Top };
    app.selected_feature = None;
    app.selected_primer = None;
    app.history_highlight = None;
    Ok(())
}
pub fn summary(app: &AppState) -> String {
    let Some(r) = app.selection.filter(|r| crate::shell::valid_range(app, *r)) else {
        return "No range selected · coordinates are 1-based inclusive".into();
    };
    let len = app.doc.sequence.len();
    let end = (r.start + r.len - 1) % len + 1;
    format!(
        "Bases {}–{} · {} nt · {} strand{}",
        r.start + 1,
        end,
        r.len,
        if app.selection_strand == Strand::Top {
            "forward"
        } else {
            "reverse"
        },
        if r.start + r.len > len {
            " · crosses origin"
        } else {
            ""
        }
    )
}
#[derive(Clone, Copy)]
pub enum DisplayToggle {
    Features,
    Primers,
    Enzymes,
    Orfs,
    Translation,
    Complement,
    Colors,
}
impl DisplayToggle {
    pub const ALL: [Self; 7] = [
        Self::Features,
        Self::Primers,
        Self::Enzymes,
        Self::Orfs,
        Self::Translation,
        Self::Complement,
        Self::Colors,
    ];
    pub fn label(self) -> &'static str {
        match self {
            Self::Features => "Features",
            Self::Primers => "Primers",
            Self::Enzymes => "Enzymes",
            Self::Orfs => "ORFs",
            Self::Translation => "Translation",
            Self::Complement => "Complement",
            Self::Colors => "Custom colors",
        }
    }
    pub fn enabled(self, app: &AppState) -> bool {
        let p = &app.prefs;
        match self {
            Self::Features => p.show_features,
            Self::Primers => p.show_primers,
            Self::Enzymes => p.show_enzymes,
            Self::Orfs => p.show_orfs,
            Self::Translation => p.show_translations,
            Self::Complement => p.show_complement,
            Self::Colors => p.show_colors,
        }
    }
}
pub fn toggle_display(app: &mut AppState, kind: DisplayToggle) {
    let p = &mut app.prefs;
    let slot = match kind {
        DisplayToggle::Features => &mut p.show_features,
        DisplayToggle::Primers => &mut p.show_primers,
        DisplayToggle::Enzymes => &mut p.show_enzymes,
        DisplayToggle::Orfs => &mut p.show_orfs,
        DisplayToggle::Translation => &mut p.show_translations,
        DisplayToggle::Complement => &mut p.show_complement,
        DisplayToggle::Colors => &mut p.show_colors,
    };
    *slot = !*slot;
}
pub fn fit(app: &mut AppState) {
    app.zoom_start = 0;
    app.zoom_width = app.doc.sequence.len().max(1);
}
pub fn zoom(app: &mut AppState, inward: bool) {
    let len = app.doc.sequence.len();
    if len == 0 {
        fit(app);
        return;
    }
    let min = 20.min(len);
    app.zoom_width = if inward {
        (app.zoom_width / 2).max(min)
    } else {
        app.zoom_width.saturating_mul(2).min(len)
    }
    .clamp(min, len);
    app.center_zoom_on_selection();
}

pub fn primer_site(
    app: &mut AppState,
    source: SourceRef,
    id: u64,
    range: Range,
    strand: Strand,
) -> Result<(), String> {
    if app.doc.primer(id).is_none()
        || !app.analysis.primer_sites.get(&id).is_some_and(|sites| {
            sites.iter().any(|site| {
                site.start == range.start && site.len == range.len && site.strand == strand
            })
        })
    {
        return Err("This primer binding site is no longer available".into());
    }
    apply(app, source, Some(range), strand)?;
    app.selected_primer = Some(id);
    app.scroll_to = Some(range.start);
    Ok(())
}

/// A click outside the base rows does not change an existing selection.
pub fn click_base(
    app: &mut AppState,
    source: SourceRef,
    hit: Option<(usize, Strand)>,
) -> Result<(), String> {
    let Some((position, strand)) = hit else {
        return Ok(());
    };
    let range = Range::new(position, 1);
    let clear = app.selection == Some(range) && app.selection_strand == strand;
    apply(app, source, if clear { None } else { Some(range) }, strand)
}
/// The list names its first binding explicitly; a no-site primer has no DNA range.
pub fn primer_from_list(app: &mut AppState, source: SourceRef, id: u64) -> Result<(), String> {
    if app.doc.primer(id).is_none() {
        return Err("The primer is no longer available".into());
    }
    if let Some(site) = app
        .analysis
        .primer_sites
        .get(&id)
        .and_then(|sites| sites.first())
        .cloned()
    {
        primer_site(
            app,
            source,
            id,
            Range::new(site.start, site.len),
            site.strand,
        )
    } else {
        apply(app, source, None, Strand::Top)?;
        app.selected_primer = Some(id);
        Ok(())
    }
}
