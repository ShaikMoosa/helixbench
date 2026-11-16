//! Explicit table scope and captured whole-template restriction predictions.
use crate::state::{AppState, EnzymeFilterState};
use domain::{
    enzyme::{CutSite, Overhang},
    results::InputSnapshot,
    Enzyme, Range, SourceRef,
};
#[derive(Clone, Default)]
pub struct DigestDraft {
    pub source: Option<SourceRef>,
    pub names: Vec<String>,
    pub result: Option<Box<DigestRun>>,
    pub error: Option<String>,
}
#[derive(Clone)]
pub struct DigestRun {
    pub input: InputSnapshot,
    pub enzymes: Vec<Enzyme>,
    pub sites: Vec<CutSite>,
    pub fragments: Vec<domain::digest::Fragment>,
    pub completed_no_sites: bool,
}
pub fn validate_source(app: &AppState, source: SourceRef) -> Result<(), String> {
    if app.doc.source_ref() != source
        || app.workspace_search.open
        || app.navigation.pending.is_some()
    {
        return Err("The input changed or navigation is pending. Reopen this action on the intended document.".into());
    }
    Ok(())
}
pub fn region(app: &AppState) -> Result<Option<Range>, String> {
    let l = app.doc.sequence.len();
    let f = &app.enzyme_filter;
    match f.region {
        0 => Ok(None),
        1 => {
            let r = app
                .selection
                .ok_or("Select a sequence region or choose Anywhere")?;
            if !crate::shell::valid_range(app, r) {
                return Err("The selected range is no longer valid".into());
            }
            Ok(Some(r))
        }
        2 => {
            let s = f
                .region_start
                .trim()
                .parse::<usize>()
                .map_err(|_| "Enter a numeric region start")?;
            let e = f
                .region_end
                .trim()
                .parse::<usize>()
                .map_err(|_| "Enter a numeric region end")?;
            if s == 0 || e == 0 || s > l || e > l {
                return Err(
                    "Region positions must be within the sequence (1-based inclusive)".into(),
                );
            }
            let n = if e >= s {
                e - s + 1
            } else if app.doc.sequence.is_circular() {
                l - s + 1 + e
            } else {
                return Err("A linear region end must follow its start".into());
            };
            Ok(Some(Range::new(s - 1, n)))
        }
        _ => Err("Choose a supported region scope".into()),
    }
}
pub fn reset(app: &mut AppState) {
    app.enzyme_filter = EnzymeFilterState::default();
}
pub fn filtered(app: &AppState) -> Result<Vec<(Enzyme, usize)>, String> {
    let scope = region(app)?;
    let f = &app.enzyme_filter;
    if f.cut_count >= 6 || f.overhang >= 5 || f.site_len >= 5 {
        return Err("Reset unsupported enzyme filters".into());
    }
    let mut out = Vec::new();
    for e in domain::enzyme_db::all_enzymes() {
        if !e
            .name
            .to_ascii_lowercase()
            .contains(&f.name.trim().to_ascii_lowercase())
        {
            continue;
        }
        let sites = e.find_sites(&app.doc.sequence);
        let n = sites.len();
        if !match f.cut_count {
            0 => true,
            1 => n == 0,
            2 => n == 1,
            3 => n == 2,
            4 => n <= 2,
            _ => n >= 3,
        } {
            continue;
        }
        let outside = e.cut_top < 0 || e.cut_top > e.site_len() as i32;
        if !match f.overhang {
            0 => true,
            1 => matches!(e.overhang(), Overhang::FivePrime(_)) && !outside,
            2 => matches!(e.overhang(), Overhang::ThreePrime(_)) && !outside,
            3 => matches!(e.overhang(), Overhang::Blunt) && !outside,
            _ => outside,
        } {
            continue;
        }
        if !match f.site_len {
            0 => true,
            1 => e.site_len() == 4,
            2 => e.site_len() == 5,
            3 => e.site_len() == 6,
            _ => e.site_len() >= 7,
        } {
            continue;
        }
        // Counts describe the entire template; region adds an independent location constraint.
        if let Some(r) = scope {
            if !sites.iter().any(|s| {
                r.contains(
                    s.cut_top,
                    app.doc.sequence.len(),
                    app.doc.sequence.is_circular(),
                ) || r.contains(
                    s.position,
                    app.doc.sequence.len(),
                    app.doc.sequence.is_circular(),
                )
            }) {
                continue;
            }
        }
        out.push((e, n));
    }
    Ok(out)
}
#[derive(Clone, Copy)]
pub enum SetEdit {
    Add,
    Remove,
    Replace,
}
pub fn edit_set(
    app: &mut AppState,
    source: SourceRef,
    names: &[String],
    mode: SetEdit,
) -> Result<(), String> {
    validate_source(app, source)?;
    if !matches!(mode, SetEdit::Remove)
        && names
            .iter()
            .any(|n| domain::enzyme_db::enzyme_by_name(n).is_none())
    {
        return Err("Unknown enzyme: review the requested set before applying it".into());
    }
    let mut chosen = app.doc.chosen_enzymes.clone();
    match mode {
        SetEdit::Add => {
            for n in names {
                if !chosen.contains(n) {
                    chosen.push(n.clone());
                }
            }
        }
        SetEdit::Remove => chosen.retain(|n| !names.contains(n)),
        SetEdit::Replace => chosen = names.to_vec(),
    };
    if chosen != app.doc.chosen_enzymes {
        app.mutate("Change chosen enzyme set", vec![], |d| {
            d.chosen_enzymes = chosen
        });
    }
    Ok(())
}
pub fn draft(app: &AppState) -> DigestDraft {
    DigestDraft {
        source: Some(app.doc.source_ref()),
        names: app.doc.chosen_enzymes.clone(),
        ..Default::default()
    }
}
pub fn run(app: &AppState, draft: &mut DigestDraft) -> Result<(), String> {
    draft.result = None;
    draft.error = None;
    validate_source(app, draft.source.ok_or("Reopen digest setup")?)?;
    if app.doc.sequence.is_empty() {
        return Err("Import or create a nonempty sequence before digesting".into());
    }
    if draft.names.is_empty() {
        return Err("Choose at least one enzyme on the Enzymes page".into());
    }
    if draft.names != app.doc.chosen_enzymes {
        return Err("The chosen enzyme set changed; reopen digest setup".into());
    }
    let enzymes=draft.names.iter().map(|n|domain::enzyme_db::enzyme_by_name(n).ok_or_else(||format!("Unknown enzyme {n}; it remains in the set for review. Remove it explicitly before prediction."))).collect::<Result<Vec<_>,_>>()?;
    let sites = domain::enzyme::digest(&app.doc.sequence, &enzymes);
    if !app.doc.sequence.is_circular() {
        for site in &sites {
            let enzyme = enzymes
                .iter()
                .find(|e| e.name == site.enzyme)
                .ok_or("Enzyme identity unavailable")?;
            let (top, bottom) = if site.strand == domain::Strand::Top {
                (enzyme.cut_top, enzyme.cut_bottom)
            } else {
                (
                    enzyme.site_len() as i32 - enzyme.cut_bottom,
                    enzyme.site_len() as i32 - enzyme.cut_top,
                )
            };
            if [top, bottom].iter().any(|offset| {
                let p = site.position as i64 + *offset as i64;
                p < 0 || p > app.doc.sequence.len() as i64
            }) {
                return Err(format!("{} has a cleavage boundary outside this linear template. Review the sequence ends before predicting a complete digest.",site.enzyme));
            }
        }
    }
    let cuts = sites.iter().map(|s| s.cut_top).collect::<Vec<_>>();
    let fragments = domain::digest::fragments(
        &cuts,
        app.doc.sequence.len(),
        app.doc.sequence.is_circular(),
    );
    draft.result = Some(Box::new(DigestRun {
        input: InputSnapshot::capture(&app.doc, None)?,
        enzymes,
        completed_no_sites: sites.is_empty(),
        sites,
        fragments,
    }));
    Ok(())
}

#[derive(Clone)]
pub struct CutSiteDraft {
    pub source: SourceRef,
    pub enzyme: String,
    pub sites: Vec<CutSite>,
    pub error: Option<String>,
}
pub fn cut_sites(app: &AppState, source: SourceRef, enzyme: &str) -> Result<CutSiteDraft, String> {
    validate_source(app, source)?;
    let e = domain::enzyme_db::enzyme_by_name(enzyme).ok_or("Enzyme no longer available")?;
    let sites = e.find_sites(&app.doc.sequence);
    if sites.is_empty() {
        return Err("No recognition sites for this enzyme in the current template".into());
    }
    Ok(CutSiteDraft {
        source,
        enzyme: enzyme.into(),
        sites,
        error: None,
    })
}
pub fn show_cut(app: &mut AppState, draft: &CutSiteDraft, site: &CutSite) -> Result<(), String> {
    validate_source(app, draft.source)?;
    if !draft.sites.contains(site)
        || !cut_sites(app, draft.source, &draft.enzyme)?
            .sites
            .contains(site)
    {
        return Err("This cleavage site changed; select it again".into());
    }
    let len = app.doc.sequence.len();
    if site.cut_top > len {
        return Err("The cleavage boundary lies outside this linear template".into());
    }
    let previous = app.dialog.take();
    let route = crate::navigation::Route {
        view: crate::state::View::Map,
        document: app.doc.id,
        record: None,
    };
    if let Err(error) = crate::navigation::request_navigation(
        app,
        crate::navigation::NavigationIntent::Navigate(route),
    ) {
        app.dialog = previous;
        return Err(error);
    }
    let position = if site.cut_top == 0 && app.doc.sequence.is_circular() {
        len - 1
    } else {
        site.cut_top.saturating_sub(1)
    };
    crate::selection::apply(
        app,
        draft.source,
        Some(Range::new(position, 1)),
        site.strand,
    )?;
    app.selected_feature = None;
    app.selected_primer = None;
    app.scroll_to = Some(position);
    app.center_zoom_on_selection();
    app.toast(format!(
        "{} cleavage after base {}",
        draft.enzyme, site.cut_top
    ));
    Ok(())
}
