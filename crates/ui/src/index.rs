use crate::layouts::main;
use crate::routes::routes;
use crate::styles::themes::GlobalTheme;
use gpui::prelude::*;
use gpui::*;

pub fn init(cx: &mut App) {
    cx.text_system()
        .add_fonts(vec![std::borrow::Cow::Borrowed(include_bytes!(
            "assets/fonts/ibm-plex-sans/IBMPlexSans-Regular.ttf"
        ))])
        .expect("falha ao carregar a fonte IBM Plex Sans");
    cx.set_global(GlobalTheme::light());
    cx.observe_global::<GlobalTheme>(|cx| cx.refresh_windows())
        .detach();
    routes(cx);
    cx.open_window(
        WindowOptions {
            window_background: WindowBackgroundAppearance::Blurred,
            ..Default::default()
        },
        |_, cx| cx.new(|cx| main::MainLayout::new(cx)),
    )
    .expect("failed to open GPUI window");
}

#[cfg(not(target_arch = "wasm32"))]
pub fn desktop_init() {
    gpui_platform::application().run(init);
}

#[cfg(target_arch = "wasm32")]
thread_local! {
    static APPLICATION: std::cell::RefCell<Option<gpui::ApplicationHandle>> =
        const { std::cell::RefCell::new(None) };
}

#[cfg(target_arch = "wasm32")]
pub fn wasm_init() {
    gpui_platform::web_init();
    let application = gpui_platform::single_threaded_web().run_embedded(|cx| {
        cx.text_system()
            .add_fonts(vec![std::borrow::Cow::Borrowed(include_bytes!(
                "assets/fonts/ibm-plex-sans/IBMPlexSans-Regular.ttf"
            ))])
            .expect("failed to load GPUI web font");
        init(cx);
        cx.activate(true);
    });
    APPLICATION.with(|cell| *cell.borrow_mut() = Some(application));
}
