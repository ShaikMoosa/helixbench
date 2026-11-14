#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod dialogs;
mod draw;
mod editor;
mod find;
mod icons;
mod text_input;
mod theme;
mod util;
mod views;
mod widgets;

use gpui::{
    px, size, App, AppContext, Application, Bounds, TitlebarOptions, WindowBounds, WindowOptions,
};

use appcore::actions as act;
use appcore::state::{AppState, PendingAction};
use draw::PxVal;

fn main() {
    Application::new()
        .with_assets(icons::Assets)
        .run(|cx: &mut App| {
            app::bind_keys(cx);
            text_input::bind_keys(cx);
            find::bind_keys(cx);

            let config = storage::config::load();
            let doc = util::initial_document(&config);
            let [w, h] = config.window_size.unwrap_or([1360.0, 860.0]);
            let restore = Bounds::centered(None, size(px(w), px(h)), cx);
            let bounds = if config.window_maximized {
                WindowBounds::Maximized(restore)
            } else {
                WindowBounds::Windowed(restore)
            };
            let state = AppState::new(doc, config);

            cx.open_window(
                WindowOptions {
                    window_bounds: Some(bounds),
                    titlebar: Some(TitlebarOptions {
                        title: Some("HelixBench".into()),
                        appears_transparent: false,
                        ..Default::default()
                    }),
                    window_min_size: Some(size(px(980.0), px(640.0))),
                    app_id: Some("HelixBench".into()),
                    ..Default::default()
                },
                |window, cx| {
                    let entity = cx.new(|cx| app::HelixBench::new(state, cx));
                    let weak = entity.downgrade();
                    // Intercept the OS close while there are unsaved edits; the
                    // Unsaved dialog then drives save / discard / cancel.
                    window.on_window_should_close(cx, move |window, cx| {
                        let Some(e) = weak.upgrade() else {
                            return true;
                        };
                        let mut allow = true;
                        e.update(cx, |this, cx| {
                            let s = window.viewport_size();
                            this.app.config.window_size = Some([s.width.v(), s.height.v()]);
                            this.app.config.window_maximized = window.is_maximized();
                            if this.app.dirty {
                                act::guard(&mut this.app, PendingAction::CloseApp);
                                allow = false;
                                cx.notify();
                            } else {
                                act::persist_config(&mut this.app);
                            }
                        });
                        allow
                    });
                    entity
                },
            )
            .unwrap();

            cx.on_window_closed(|cx| cx.quit()).detach();
            cx.activate(true);
        });
}
