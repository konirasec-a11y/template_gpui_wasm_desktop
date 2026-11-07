use gpui_navigator::{GlobalRouter, Route, Transition};

pub fn register(router: &mut GlobalRouter) {
    router.add_route(
        Route::component("/", super::views::login::LoginPage::default)
            .transition(Transition::fade(300)),
    );
}