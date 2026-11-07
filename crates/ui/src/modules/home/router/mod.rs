use gpui_navigator::{GlobalRouter, Route, Transition};

pub fn register(router: &mut GlobalRouter) {
    router.add_route(
        Route::component("/home", super::views::home::HomePage::default)
            .transition(Transition::fade(300)),
    );
}