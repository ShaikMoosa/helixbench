//! Selection identity and coordinate feedback shared by the Map and Sequence views.
use crate::{
    app::HelixBench,
    theme,
    widgets::{self, Btn},
};
use appcore::{selection, state::View};
use gpui::{div, prelude::*, px, AnyElement, Context};
pub fn bar(app: &mut HelixBench, cx: &mut Context<HelixBench>) -> AnyElement {
    let source = app.app.doc.source_ref();
    let mut actions = div()
        .flex()
        .flex_wrap()
        .gap(px(6.0))
        .child(
            widgets::button("sequence-find-visible", "Find in sequence", Btn::Ghost).on_click(
                cx.listener(|t, _, _, cx| {
                    t.app.search.open = true;
                    t.app.search.focus_request = true;
                    cx.notify();
                }),
            ),
        )
        .child(
            widgets::button(
                "sequence-annotations-visible",
                "Annotation list",
                Btn::Ghost,
            )
            .on_click(cx.listener(|t, _, _, cx| t.set_view(View::Features, cx))),
        )
        .child(div().flex_1())
        .when(appcore::viewer::active(&app.app).is_some(), |d| {
            d.child(crate::viewer::options_button(app, cx))
        });
    if app.app.selection.is_some() {
        actions = actions.child(
            widgets::button("sequence-clear-visible", "Clear selection", Btn::Outline).on_click(
                cx.listener(move |t, _, _, cx| {
                    if let Err(e) = selection::apply(&mut t.app, source, None, domain::Strand::Top)
                    {
                        t.app.toast(e);
                    }
                    cx.notify();
                }),
            ),
        );
    }
    div()
        .flex()
        .flex_col()
        .flex_shrink_0()
        .gap(px(7.0))
        .px(px(32.0))
        .py(px(8.0))
        .child(actions)
        .when(app.app.selection.is_some(), |d| {
            d.child(widgets::label(
                &selection::summary(&app.app),
                theme::T_BASE,
                theme::MUTED,
            ))
        })
        .into_any_element()
}
