//! GPUI Kit components.
//!
//! Visual contract: Paper file `Grafik UI`.

mod assets;
mod button;
mod checkbox;
mod chrome;
mod icon;
mod input;
mod label;
mod radio;
mod spinner;
mod switch;

pub use assets::{load_fonts, Assets};
pub use button::{Button, ButtonGroup, ButtonSize, ButtonVariant};
pub use checkbox::{CheckState, Checkbox};
pub use chrome::{ButtonChrome, FieldChrome, FieldState};
pub use icon::{Icon, IconName};
pub use input::{textarea, Input};
pub use label::Label;
pub use radio::Radio;
pub use spinner::{Spinner, SpinnerSize, SpinnerTone};
pub use switch::Switch;

pub use gpui_kit_motion as motion;
pub use gpui_kit_motion::{
    cubic_bezier, ease_in_cubic, ease_out_cubic, init as init_motion, use_motion_value,
    AnimatePresence, Ease, Motion, MotionStyle, MotionValue, MotionValueStore, PresenceMode,
    Stagger, StyledSlot, Transition, Variants,
};

pub use gpui_kit_theme as theme;
pub use gpui_kit_theme::{
    init as init_theme, paint, rgb, ActiveTheme, Theme, ThemeKind, FONT_FAMILY,
};

/// Bind kit keystrokes (text fields). Call once at startup.
pub fn init(cx: &mut gpui::App) {
    input::init(cx);
}
