use gpui::{
    px, svg, App, Hsla, IntoElement, Pixels, RenderOnce, SharedString, StyleRefinement, Styled,
    Window,
};
use grafik_motion::StyledSlot;

/// Icons used on the Paper Buttons page.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum IconName {
    Plus,
    Download,
    ChevronRight,
    ChevronDown,
    Search,
    Spinner,
    Check,
}

impl IconName {
    pub fn path(self) -> SharedString {
        SharedString::from(match self {
            Self::Plus => "icons/plus.svg",
            Self::Download => "icons/download.svg",
            Self::ChevronRight => "icons/chevron-right.svg",
            Self::ChevronDown => "icons/chevron-down.svg",
            Self::Search => "icons/search.svg",
            Self::Spinner => "icons/spinner.svg",
            Self::Check => "icons/check.svg",
        })
    }
}

/// SVG icon painted with `text_color`.
#[derive(IntoElement)]
pub struct Icon {
    name: IconName,
    style: StyleRefinement,
}

impl Icon {
    pub fn new(name: IconName) -> Self {
        Self {
            name,
            style: StyleRefinement::default().size(px(16.)).flex_shrink_0(),
        }
    }

    pub fn px(self, size: impl Into<Pixels>) -> Self {
        self.size(size.into())
    }

    pub fn color(self, color: Hsla) -> Self {
        self.text_color(color)
    }

    pub fn name(&self) -> IconName {
        self.name
    }
}

impl Styled for Icon {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Icon {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        svg().path(self.name.path()).refine_style(&self.style)
    }
}
