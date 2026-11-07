use gpui::{Context, Render, Window, div, prelude::*, px, svg};
use gpui_navigator::RouterLink;

pub struct SidebarItem {
    pub label: &'static str,
    pub path: &'static str,
    pub icon: &'static [u8],
}

pub struct SidebarComponent {
    items: Vec<SidebarItem>,
    collapsed: bool,
}

impl SidebarComponent {
    pub fn new(items: Vec<SidebarItem>) -> Self {
        Self {
            items,
            collapsed: true,
        }
    }
}

impl Render for SidebarComponent {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let themes = cx.global::<crate::styles::themes::GlobalTheme>();
        let background = themes.background;
        let text = themes.text;
        let is_dark = themes.is_dark;
        let collapsed = self.collapsed;

        let mut icon_navigation = div().flex().flex_col().gap_1();
        let mut label_navigation = div().flex().flex_col().gap_1();

        for item in &self.items {
            icon_navigation = icon_navigation.child(
                RouterLink::new(item.path)
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_center()
                            .size(px(40.))
                            .child(
                                svg()
                                    .data(item.icon)
                                    .size(px(20.))
                                    .flex_shrink_0()
                                    .text_color(text),
                            ),
                    )
                    .build(cx),
            );
            label_navigation = label_navigation.child(
                RouterLink::new(item.path)
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .h(px(40.))
                            .child(item.label),
                    )
                    .build(cx),
            );
        }

        let theme_icon: &'static [u8] = if is_dark {
            include_bytes!("../../assets/icons/lucide/sun.svg")
        } else {
            include_bytes!("../../assets/icons/lucide/moon.svg")
        };
        let theme_label = if is_dark { "Tema claro" } else { "Tema escuro" };

        let theme_icon_button = div()
            .id("sidebar-theme-icon-toggle")
            .flex()
            .items_center()
            .justify_center()
            .size(px(40.))
            .cursor_pointer()
            .on_click(cx.listener(|_, _, _, cx| {
                let is_dark = cx.global::<crate::styles::themes::GlobalTheme>().is_dark;
                let theme = if is_dark {
                    crate::styles::themes::GlobalTheme::light()
                } else {
                    crate::styles::themes::GlobalTheme::dark()
                };
                cx.set_global(theme);
            }))
            .child(
                svg()
                    .data(theme_icon)
                    .size(px(20.))
                    .flex_shrink_0()
                    .text_color(text),
            );
        let theme_text_button = div()
            .id("sidebar-theme-label-toggle")
            .flex()
            .items_center()
            .h(px(40.))
            .cursor_pointer()
            .on_click(cx.listener(|_, _, _, cx| {
                let is_dark = cx.global::<crate::styles::themes::GlobalTheme>().is_dark;
                let theme = if is_dark {
                    crate::styles::themes::GlobalTheme::light()
                } else {
                    crate::styles::themes::GlobalTheme::dark()
                };
                cx.set_global(theme);
            }))
            .child(theme_label);

        let collapse_icon: &'static [u8] = if collapsed {
            include_bytes!("../../assets/icons/lucide/panel-left-open.svg")
        } else {
            include_bytes!("../../assets/icons/lucide/panel-left-close.svg")
        };
        let collapse_icon_button = div()
            .id("sidebar-collapse-icon-toggle")
            .flex()
            .items_center()
            .justify_center()
            .size(px(40.))
            .cursor_pointer()
            .on_click(cx.listener(|this, _, _, cx| {
                this.collapsed = !this.collapsed;
                cx.notify();
            }))
            .child(
                svg()
                    .data(collapse_icon)
                    .size(px(20.))
                    .flex_shrink_0()
                    .text_color(text),
            );
        let collapse_text_button = div()
            .id("sidebar-collapse-label-toggle")
            .flex()
            .items_center()
            .h(px(40.))
            .cursor_pointer()
            .on_click(cx.listener(|this, _, _, cx| {
                this.collapsed = !this.collapsed;
                cx.notify();
            }))
            .child(if collapsed { "Expandir" } else { "Recolher" });

        let icon_footer = div()
            .flex()
            .flex_col()
            .gap_1()
            .child(theme_icon_button)
            .child(collapse_icon_button);
        let label_footer = div()
            .flex()
            .flex_col()
            .gap_1()
            .child(theme_text_button)
            .child(collapse_text_button);

        let icon_column = div()
            .flex()
            .flex_col()
            .h_full()
            .w(px(64.))
            .flex_shrink_0()
            .justify_between()
            .items_center()
            .child(icon_navigation)
            .child(icon_footer);

        let mut sidebar = div()
            .flex()
            .flex_row()
            .h_full()
            .w(px(if collapsed { 64. } else { 200. }))
            .bg(background)
            .text_color(text)
            .child(icon_column);

        if !collapsed {
            sidebar = sidebar.child(
                div()
                    .flex()
                    .flex_col()
                    .h_full()
                    .flex_1()
                    .justify_between()
                    .child(label_navigation)
                    .child(label_footer),
            );
        }

        sidebar
    }
}
