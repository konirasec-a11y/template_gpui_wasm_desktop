use gpui::{Context, Entity, Render, Window, div, prelude::*};
use gpui_navigator::router_view;

use crate::components::sidebar::{SidebarComponent, SidebarItem};

pub struct MainLayout {
    sidebar: Entity<SidebarComponent>,
}

impl MainLayout {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let sidebar = cx.new(|_| {
            SidebarComponent::new(vec![SidebarItem {
                label: "Home",
                path: "/home",
                icon: include_bytes!("../assets/icons/lucide/home.svg"),
            }])
        });

        Self { sidebar }
    }
}

impl Render for MainLayout {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let themes = cx.global::<crate::styles::themes::GlobalTheme>();

        div()
            .flex()
            .gap_1()
            .size_full()
            .bg(themes.background)
            .text_color(themes.text)
            .child(self.sidebar.clone())
            .child(router_view(window, cx))
    }
}
