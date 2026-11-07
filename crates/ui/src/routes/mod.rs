use gpui::App;
use gpui_navigator::init_router;

pub fn routes(cx: &mut App) {
    init_router(cx, |router| {
        crate::modules::counter::router::register(router);
        crate::modules::home::router::register(router);
    });
}