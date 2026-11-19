//! Find bar: DNA / protein / name search with live results, recents and
//! Enter / Shift+Enter navigation. Rendered under the tab strip.

use gpui::{
    actions, anchored, deferred, div, point, prelude::*, px, AnyElement, App, Context, Corner, Div,
    ElementId, Focusable, KeyBinding, MouseButton, SharedString, Stateful, Window,
};

use appcore::state::{Dialog, SearchMode};

use crate::app::HelixBench;
use crate::theme;
use crate::widgets::{self, Btn};

actions!(findbar, [Next, Prev]);

pub fn bind_keys(cx: &mut App) {
    cx.bind_keys([
        KeyBinding::new("enter", Next, Some("FindBar")),
        KeyBinding::new("shift-enter", Prev, Some("FindBar")),
        KeyBinding::new("down", WorkspaceNext, Some("WorkspaceSearch")),
        KeyBinding::new("up", WorkspacePrev, Some("WorkspaceSearch")),
        KeyBinding::new("enter", WorkspaceOpen, Some("WorkspaceSearch")),
    ]);
}

fn mode_btn(id: impl Into<ElementId>, label: &str, active: bool) -> Stateful<Div> {
    let d = div()
        .id(id)
        .h(px(26.0))
        .px(px(11.0))
        .flex()
        .items_center()
        .rounded(px(5.0))
        .text_size(px(theme::T_SM))
        .cursor_pointer();
    if active {
        d.bg(theme::SURFACE_2)
            .border_1()
            .border_color(theme::ACCENT.opacity(0.7))
            .text_color(theme::ACCENT_TEXT)
    } else {
        d.text_color(theme::MUTED)
            .hover(|s| s.bg(theme::SURFACE).text_color(theme::TEXT))
    }
    .child(label.to_owned())
}

pub fn show(app: &mut HelixBench, window: &mut Window, cx: &mut Context<HelixBench>) -> AnyElement {
    // Pull the query text and re-run search live as it changes.
    let seed = app.app.search.query.clone();
    let query = app.input_entity(cx, "find:query", &seed, "Search sequence or name…", true);
    let text = query.read(cx).text();
    if text != app.app.search.query {
        app.app.search.query = text;
        app.app.run_search();
        if app.app.search.hit_count() > 0 {
            app.app.goto_current_hit();
        }
    }
    if app.app.search.focus_request {
        let fh = query.read(cx).focus_handle(cx);
        fh.focus(window);
        app.app.search.focus_request = false;
    }

    let mode = app.app.search.mode;
    let n = app.app.search.hit_count();

    let mut bar = div()
        .key_context("FindBar")
        .flex()
        .items_center()
        .gap(px(8.0))
        .px(px(12.0))
        .py(px(8.0))
        .bg(theme::PANEL)
        .border_b_1()
        .border_color(theme::BORDER)
        .on_action(cx.listener(|t, _: &Next, _, cx| {
            t.app.search_next(true);
            cx.notify();
        }))
        .on_action(cx.listener(|t, _: &Prev, _, cx| {
            t.app.search_next(false);
            cx.notify();
        }))
        .child(widgets::label("Find", theme::T_SM, theme::FAINT));

    // Mode segmented control.
    let mut seg = div()
        .flex()
        .items_center()
        .gap(px(2.0))
        .p(px(2.0))
        .rounded(px(7.0))
        .bg(theme::BG)
        .border_1()
        .border_color(theme::BORDER);
    for (m, label) in [
        (SearchMode::Dna, "DNA"),
        (SearchMode::Protein, "Protein"),
        (SearchMode::Name, "Name"),
    ] {
        seg = seg.child(
            mode_btn(
                SharedString::from(format!("find-mode-{}", m.label())),
                label,
                mode == m,
            )
            .on_click(cx.listener(move |t, _, _, cx| {
                if t.app.search.mode != m {
                    t.app.search.mode = m;
                    t.app.run_search();
                    if t.app.search.hit_count() > 0 {
                        t.app.goto_current_hit();
                    }
                }
                cx.notify();
            })),
        );
    }
    bar = bar.child(seg);
    bar = bar.child(div().w(px(280.0)).child(query.clone()));

    // Recents dropdown.
    bar = bar.child(
        widgets::button("find-recents", "Recent ▾", Btn::Subtle).on_mouse_down(
            MouseButton::Left,
            cx.listener(|t, ev: &gpui::MouseDownEvent, _, cx| {
                t.find_recents_open = !t.find_recents_open;
                t.find_recents_anchor = ev.position;
                cx.notify();
            }),
        ),
    );

    // Prev / next.
    let mut prev = widgets::icon_only("find-prev", "chevron-left", Btn::Outline)
        .opacity(if n > 0 { 1.0 } else { 0.4 });
    if n > 0 {
        prev = prev.on_click(cx.listener(|t, _, _, cx| {
            t.app.search_next(false);
            cx.notify();
        }));
    }
    bar = bar.child(prev);
    let mut next = widgets::icon_only("find-next", "chevron-right", Btn::Outline)
        .opacity(if n > 0 { 1.0 } else { 0.4 });
    if n > 0 {
        next = next.on_click(cx.listener(|t, _, _, cx| {
            t.app.search_next(true);
            cx.notify();
        }));
    }
    bar = bar.child(next);

    bar = bar.child(status_label(&app.app, n));
    bar = bar.child(div().flex_grow());
    bar = bar.child(
        widgets::button("find-tips", "?", Btn::Ghost).on_click(cx.listener(|t, _, _, cx| {
            t.app.dialog = Some(Dialog::SearchTips);
            cx.notify();
        })),
    );
    bar = bar.child(
        widgets::icon_only("find-close", "x", Btn::Ghost).on_click(cx.listener(|t, _, _, cx| {
            t.app.search.open = false;
            t.find_recents_open = false;
            cx.notify();
        })),
    );

    if let Some(popup) = recents_popup(app, cx) {
        bar = bar.child(popup);
    }
    bar.into_any_element()
}

fn status_label(app: &appcore::state::AppState, n: usize) -> Div {
    if let Some(e) = &app.search.error {
        return div()
            .flex()
            .items_center()
            .gap(px(6.0))
            .child(crate::icons::icon("alert", 13.0, theme::DANGER))
            .child(widgets::label(e, theme::T_SM, theme::DANGER));
    }
    if app.search.query.trim().is_empty() {
        return widgets::label("Type to search both strands", theme::T_SM, theme::FAINT);
    }
    if n == 0 {
        return widgets::label("No matches", theme::T_SM, theme::WARN);
    }
    let mut row = div().flex().items_center().gap(px(8.0)).child(
        div()
            .font_family(theme::mono_family())
            .font_weight(gpui::FontWeight::SEMIBOLD)
            .text_size(px(theme::T_SM))
            .text_color(theme::ACCENT_TEXT)
            .child(format!("{} of {}", app.search.current + 1, n)),
    );
    if let Some((start, len, strand)) = current_hit_locus(app) {
        let txt = if len == 1 {
            format!("at {}", start + 1)
        } else {
            format!("at {}..{}", start + 1, start + len)
        };
        row = row.child(
            div()
                .font_family(theme::mono_family())
                .text_size(px(theme::T_XS))
                .text_color(theme::MUTED)
                .child(format!("{txt} · {strand}")),
        );
    }
    row
}

fn current_hit_locus(app: &appcore::state::AppState) -> Option<(usize, usize, &'static str)> {
    let strand = |s: domain::Strand| {
        if s == domain::Strand::Top {
            "top strand"
        } else {
            "bottom strand"
        }
    };
    match app.search.mode {
        SearchMode::Dna => app
            .search
            .dna_hits
            .get(app.search.current)
            .map(|h| (h.start, h.len, strand(h.strand))),
        SearchMode::Protein => app
            .search
            .protein_hits
            .get(app.search.current)
            .map(|h| (h.start, h.len, strand(h.strand))),
        SearchMode::Name => None,
    }
}

fn recents_popup(app: &mut HelixBench, cx: &mut Context<HelixBench>) -> Option<AnyElement> {
    if !app.find_recents_open {
        return None;
    }
    let mut card = div()
        .id("find-recents-card")
        .flex()
        .flex_col()
        .min_w(px(260.0))
        .max_w(px(360.0))
        .py(px(4.0))
        .bg(theme::OVERLAY)
        .border_1()
        .border_color(theme::BORDER_STRONG)
        .rounded(px(8.0))
        .shadow(theme::shadow_panel())
        .on_mouse_down(
            MouseButton::Left,
            cx.listener(|_, _, _, cx| cx.stop_propagation()),
        )
        .on_mouse_down_out(cx.listener(|t, _, _, cx| {
            t.find_recents_open = false;
            cx.notify();
        }));
    if app.app.search.recents.is_empty() {
        card = card.child(
            widgets::label("No recent searches", theme::T_SM, theme::FAINT)
                .px(px(10.0))
                .py(px(4.0)),
        );
    } else {
        let recents = app.app.search.recents.clone();
        for (i, (m, q)) in recents.into_iter().enumerate() {
            let tag = match m {
                SearchMode::Dna => "DNA",
                SearchMode::Protein => "AA",
                SearchMode::Name => "Name",
            };
            let label = format!("[{tag}]  {q}");
            card = card.child(
                widgets::menu_row(SharedString::from(format!("recent-{i}")), &label, "", true)
                    .on_click(cx.listener(move |t, _, _, cx| {
                        t.find_recents_open = false;
                        t.app.search.mode = m;
                        t.app.search.query = q.clone();
                        t.set_input(cx, "find:query", q.clone());
                        t.app.run_search();
                        if t.app.search.hit_count() > 0 {
                            t.app.goto_current_hit();
                        }
                        cx.notify();
                    })),
            );
        }
    }
    Some(
        deferred(
            anchored()
                .anchor(Corner::TopLeft)
                .position(app.find_recents_anchor)
                .offset(point(px(-4.0), px(8.0)))
                .snap_to_window()
                .child(card),
        )
        .with_priority(46)
        .into_any_element(),
    )
}

// Workspace search uses its own key context and model. Ctrl+F above remains sequence Find.
actions!(
    workspace_search_keys,
    [WorkspaceNext, WorkspacePrev, WorkspaceOpen]
);

pub fn workspace_toolbar(app: &mut HelixBench, cx: &mut Context<HelixBench>) -> AnyElement {
    let mut row = div()
        .flex()
        .flex_wrap()
        .items_center()
        .gap(px(6.0))
        .px(px(theme::PAGE_PAD))
        .py(px(8.0));
    if app.app.workspace_search.can_return && !app.app.workspace_search.open {
        row = row.child(
            widgets::button("workspace-search-back", "Back to search", Btn::Outline).on_click(
                cx.listener(|t, _, _, cx| {
                    if let Err(error) = appcore::workspace_search::back_to_search(&mut t.app) {
                        t.app.toast(error);
                    }
                    cx.notify();
                }),
            ),
        );
    }
    if (appcore::state::View::ALL.contains(&app.app.view)
        || app.app.view == appcore::state::View::Split)
        && !app.app.workspace_search.open
    {
        for view in appcore::state::View::ALL {
            row = row.child(
                widgets::view_tab(
                    SharedString::from(format!("gallery-sequence-tab-{view:?}")),
                    view.label(),
                    app.app.view == view,
                )
                .on_click(cx.listener(move |t, _, _, cx| t.set_view(view, cx))),
            );
        }
    }
    if (appcore::state::View::ALL.contains(&app.app.view)
        || app.app.view == appcore::state::View::Split)
        && !app.app.workspace_search.open
    {
        row = row.child(
            widgets::button("workspace-split-layout", "Split layout", Btn::Ghost)
                .on_click(cx.listener(|t, _, _, cx| t.set_view(appcore::state::View::Split, cx))),
        );
    }
    if !app.app.workspace_search.open
        && !matches!(
            app.app.view,
            appcore::state::View::Home
                | appcore::state::View::Sequences
                | appcore::state::View::Results
                | appcore::state::View::Activity
                | appcore::state::View::Map
                | appcore::state::View::Sequence
        )
    {
        row = row.child(
            widgets::button("gallery-open-page-tools", "Page tools", Btn::Ghost)
                .track_focus(&app.page_tools_focus)
                .on_click(cx.listener(|t, _, _, cx| crate::shell::open_page_tools(t, cx))),
        );
    }
    row.into_any_element()
}

fn activate_workspace_target(
    app: &mut HelixBench,
    target: appcore::workspace_search::SearchTarget,
    window: &mut Window,
    cx: &mut Context<HelixBench>,
) {
    if let Err(error) = appcore::workspace_search::open_target(&mut app.app, target) {
        app.app.workspace_search.error = Some(error);
    } else {
        app.focus_handle.focus(window);
    }
    cx.notify();
}
fn step_workspace(app: &mut HelixBench, forward: bool, cx: &mut Context<HelixBench>) {
    let state = &mut app.app.workspace_search;
    let count = state.results.len();
    if count == 0 {
        return;
    }
    let current = state
        .selected
        .and_then(|target| state.results.iter().position(|h| h.target == target))
        .unwrap_or(0);
    let next = if forward {
        (current + 1) % count
    } else {
        (current + count - 1) % count
    };
    state.selected = Some(state.results[next].target);
    state.visible_limit = state.visible_limit.max(next + 1);
    app.workspace_scroll.scroll_to_item(next);
    cx.notify();
}

pub fn show_workspace(
    app: &mut HelixBench,
    window: &mut Window,
    cx: &mut Context<HelixBench>,
) -> AnyElement {
    use appcore::{
        commands,
        workspace_search::{self as search, SearchScope, SearchTarget},
    };
    let seed = app.app.workspace_search.query.text.clone();
    let input = app.input_entity(
        cx,
        "workspace:query",
        &seed,
        "Search names, IDs, notes or tools…",
        true,
    );
    let text = input.read(cx).text();
    if text != app.app.workspace_search.query.text {
        let mut query = app.app.workspace_search.query.clone();
        query.text = text;
        search::run(&mut app.app, query);
        app.workspace_scroll.set_offset(point(px(0.0), px(0.0)));
    }
    if app.app.workspace_search.focus_request {
        input.read(cx).focus_handle(cx).focus(window);
        app.app.workspace_search.focus_request = false;
    }
    let mut page=div().key_context("WorkspaceSearch").flex().flex_col().size_full().p(px(20.0)).gap(px(12.0))
        .on_action(cx.listener(|t,_:&WorkspaceNext,_,cx|step_workspace(t,true,cx)))
        .on_action(cx.listener(|t,_:&WorkspacePrev,_,cx|step_workspace(t,false,cx)))
        .on_action(cx.listener(|t,_:&WorkspaceOpen,window,cx|{if let Some(target)=t.app.workspace_search.selected{activate_workspace_target(t,target,window,cx);}}))
        .child(div().flex().items_center().justify_between()
            .child(widgets::label("Search workspace",theme::T_XL,theme::TEXT))
            .child(widgets::button("close-workspace-search","Close search · Esc",Btn::Ghost).on_click(cx.listener(|t,_,window,cx|{
                if let Err(error)=search::close(&mut t.app){t.app.workspace_search.error=Some(error);}else{t.focus_handle.focus(window);}cx.notify();
            }))))
        .child(widgets::label("Search loaded workspace records and tools. Sequence Find uses Ctrl+F. Use ↑ / ↓ and Enter to open a result.",theme::T_SM,theme::MUTED))
        .child(input);
    let mut scopes = div().flex().flex_wrap().gap(px(6.0));
    for scope in SearchScope::ALL {
        let active = app.app.workspace_search.query.scope == scope;
        scopes = scopes.child(
            widgets::button(
                SharedString::from(format!("scope-{scope:?}")),
                scope.label(),
                if active { Btn::Primary } else { Btn::Subtle },
            )
            .on_click(cx.listener(move |t, _, _, cx| {
                let mut q = t.app.workspace_search.query.clone();
                q.scope = scope;
                search::run(&mut t.app, q);
                t.workspace_scroll.set_offset(point(px(0.0), px(0.0)));
                cx.notify();
            })),
        );
    }
    let archived = app.app.workspace_search.query.include_archived;
    scopes = scopes.child(
        widgets::button(
            "workspace-archived",
            if archived {
                "Archived included"
            } else {
                "Include archived"
            },
            if archived { Btn::Primary } else { Btn::Outline },
        )
        .on_click(cx.listener(|t, _, _, cx| {
            let mut q = t.app.workspace_search.query.clone();
            q.include_archived = !q.include_archived;
            search::run(&mut t.app, q);
            cx.notify();
        })),
    );
    page = page.child(scopes);
    if let Some(error) = &app.app.workspace_search.error {
        page = page.child(widgets::label(error, theme::T_BASE, theme::DANGER));
    }
    let total = app.app.workspace_search.results.len();
    page = page.child(
        div()
            .flex()
            .justify_between()
            .child(widgets::label(
                &format!("{total} results"),
                theme::T_SM,
                theme::MUTED,
            ))
            .child(
                widgets::button("workspace-refresh", "Refresh results", Btn::Ghost).on_click(
                    cx.listener(|t, _, _, cx| {
                        let q = t.app.workspace_search.query.clone();
                        search::run(&mut t.app, q);
                        cx.notify();
                    }),
                ),
            ),
    );
    let mut list = div()
        .id("workspace-search-results")
        .flex()
        .flex_col()
        .flex_1()
        .min_h(px(0.0))
        .overflow_y_scroll()
        .track_scroll(&app.workspace_scroll);
    if total == 0 {
        list=list.child(widgets::empty_state("search","No matching items","Change the query or scope, or include archived records. Only loaded workspace data is searched."));
    }
    for hit in app
        .app
        .workspace_search
        .results
        .iter()
        .take(app.app.workspace_search.visible_limit)
    {
        let target = hit.target;
        let selected = app.app.workspace_search.selected == Some(target);
        let availability = if let SearchTarget::Command(id) = target {
            Some(commands::availability(&app.app, id))
        } else {
            None
        };
        let summary = availability
            .as_ref()
            .filter(|a| !a.enabled())
            .map(|a| a.reason().to_string())
            .unwrap_or_else(|| {
                hit.detail
                    .lines()
                    .next()
                    .unwrap_or("")
                    .chars()
                    .take(150)
                    .collect()
            });
        list = list.child(
            div()
                .id(SharedString::from(format!("search-hit-{target:?}")))
                .flex()
                .flex_col()
                .gap(px(4.0))
                .px(px(12.0))
                .py(px(12.0))
                .rounded(px(6.0))
                .border_1()
                .border_color(if selected {
                    theme::ACCENT
                } else {
                    theme::BORDER
                })
                .bg(if selected {
                    theme::SURFACE_2
                } else {
                    theme::SURFACE
                })
                .cursor_pointer()
                .hover(|s| s.bg(theme::FILL_HOVER))
                .child(
                    div()
                        .flex()
                        .justify_between()
                        .child(widgets::label(&hit.title, theme::T_MD, theme::TEXT))
                        .child(widgets::label(hit.kind, theme::T_SM, theme::MUTED)),
                )
                .child(widgets::label(&summary, theme::T_SM, theme::MUTED))
                .when(hit.archived, |d| {
                    d.child(widgets::label("Archived", theme::T_SM, theme::MUTED))
                })
                .on_click(cx.listener(move |t, _, window, cx| {
                    activate_workspace_target(t, target, window, cx)
                })),
        );
    }
    if total > app.app.workspace_search.visible_limit {
        list = list.child(
            widgets::button("workspace-show-more", "Show more results", Btn::Outline).on_click(
                cx.listener(|t, _, _, cx| {
                    t.app.workspace_search.visible_limit =
                        t.app.workspace_search.visible_limit.saturating_add(50);
                    cx.notify();
                }),
            ),
        );
    }
    page.child(list).into_any_element()
}

pub fn inspected_record(app: &mut HelixBench, cx: &mut Context<HelixBench>) -> Option<AnyElement> {
    use appcore::workspace_search::{self as search, SearchTarget};
    if app.app.workspace_search.open {
        return None;
    }
    let target = app.app.workspace_search.inspected?;
    if !matches!(target, SearchTarget::Record(_) | SearchTarget::Project(_)) {
        return None;
    }
    let Some(hit) = search::lookup(&app.app, target) else {
        return Some(widgets::operation_notice("This search result is no longer available. Return to search and refresh the results.").into_any_element());
    };
    if app.app.view != hit.view {
        return None;
    }
    let mut notice = widgets::operation_notice(&format!(
        "{} · {}{}",
        hit.title,
        hit.kind,
        if hit.archived { " · Archived" } else { "" }
    ))
    .max_h(px(200.0))
    .id("search-record-detail")
    .overflow_y_scroll()
    .child(widgets::label(&hit.detail, theme::T_BASE, theme::TEXT));
    if matches!(
        target,
        SearchTarget::Record(domain::RecordRef {
            store: domain::StoreKind::Notebook | domain::StoreKind::Registry,
            ..
        })
    ) {
        if let Some(reason) = search::edit_unavailable(&app.app, target) {
            notice = notice.child(widgets::label(reason, theme::T_SM, theme::MUTED));
        } else {
            notice = notice.child(
                widgets::button("edit-inspected-record", "Edit this record", Btn::Outline)
                    .on_click(cx.listener(|t, _, _, cx| {
                        if let Err(error) = search::edit_inspected(&mut t.app) {
                            t.app.toast(error);
                        }
                        cx.notify();
                    })),
            );
        }
    }
    Some(notice.into_any_element())
}
