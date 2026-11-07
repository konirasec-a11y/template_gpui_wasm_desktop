use gpui::{ParentElement, Render, Styled, div};
use gpui::prelude::*;

#[derive(Default)]
pub struct CounterPage;

impl Render for CounterPage {
    fn render(&mut self, window: &mut gpui::Window, cx: &mut gpui::prelude::Context<Self>) -> impl gpui::prelude::IntoElement {
        div()
            .size_full()
            .child("Counter Page")
            .bg(cx.global::<crate::styles::themes::GlobalTheme>().background)
            .text_color(cx.global::<crate::styles::themes::GlobalTheme>().text)
    }
}