use gpui::{Global, Hsla, rgb, rgba};

pub struct GlobalTheme {
    pub is_dark: bool,
    pub background: Hsla,
    pub text: Hsla,
    pub primary: Hsla,
    pub surface: Hsla,
    pub element_hover: Hsla,
    pub element_selected: Hsla,
    pub border: Hsla,
    pub panel: Hsla,
    pub status_bar_bg: Hsla,
    pub status_bar_fg: Hsla,
    pub error: Hsla,
    pub warning: Hsla,
    pub info: Hsla,
    pub modified: Hsla,
}

impl Global for GlobalTheme {}

impl GlobalTheme {
    pub fn light() -> Self {
        #[cfg(target_os = "windows")]
        let background = rgba(0xffffff40).into();
        #[cfg(not(target_os = "windows"))]
        let background = rgba(0xffffffd7).into();

        Self {
            is_dark: false,
            background,
            text: rgb(0x4c4f69).into(),
            primary: rgb(0x1e66f5).into(),
            surface: rgba(0xe6e9efd0).into(),
            element_hover: rgba(0x4c4f691a).into(),
            element_selected: rgba(0x4c4f6926).into(),
            border: rgba(0x4c4f6926).into(),
            panel: rgba(0xe6e9efd7).into(),
            status_bar_bg: rgba(0xe6e9efd7).into(),
            status_bar_fg: rgb(0x4c4f69).into(),
            error: rgb(0xd20f39).into(),
            warning: rgb(0xdf8e1d).into(),
            info: rgb(0x1e66f5).into(),
            modified: rgb(0xdf8e1d).into(),
        }
    }

    pub fn dark() -> Self {
        #[cfg(target_os = "windows")]
        let background = rgba(0x00000040).into();
        #[cfg(not(target_os = "windows"))]
        let background = rgba(0x000000d7).into();

        Self {
            is_dark: true,
            background,
            text: rgb(0xffffff).into(),
            primary: rgb(0x1e66f5).into(),
            surface: rgba(0x000000d0).into(),
            element_hover: rgba(0x0000004d).into(),
            element_selected: rgba(0x0000004d).into(),
            border: rgba(0x00000000).into(),
            panel: rgba(0x00000000).into(),
            status_bar_bg: rgba(0x000000d7).into(),
            status_bar_fg: rgb(0xffffff).into(),
            error: rgb(0xd20f39).into(),
            warning: rgb(0xe5c07b).into(),
            info: rgb(0x0c194d).into(),
            modified: rgb(0xe5c07b).into(),
        }
    }
}
