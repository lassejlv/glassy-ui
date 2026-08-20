//! Glassy UI components.
//!
//! Visual contract: design spec file `Glassy UI`.

pub mod motion;
pub mod theme;

mod alert_dialog;
mod assets;
mod badge;
mod button;
mod checkbox;
mod chrome;
mod compat;
mod context_menu;
mod dialog;
mod dropdown_menu;
mod icon;
mod input;
mod kbd;
mod label;
mod popover;
mod progress;
mod radio;
mod select;
mod separator;
mod skeleton;
mod spinner;
mod switch;
mod tooltip;

pub use alert_dialog::AlertDialog;
pub use assets::{load_fonts, Assets};
pub use badge::{Badge, BadgeVariant};
pub use button::{Button, ButtonGroup, ButtonSize, ButtonVariant};
pub use checkbox::{CheckState, Checkbox};
pub use chrome::{ButtonChrome, FieldChrome, FieldState};
pub use context_menu::ContextMenu;
pub use dialog::{
    Dialog, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogTitle,
};
pub use dropdown_menu::{DropdownMenu, DropdownMenuEntry, DropdownMenuItem};
pub use icon::{Icon, IconName};
pub use input::{textarea, Input};
pub use kbd::Kbd;
pub use label::Label;
pub use popover::{Popover, PopoverContent, PopoverDescription, PopoverPlacement, PopoverTitle};
pub use progress::{CircularProgress, Progress};
pub use radio::Radio;
pub use select::{Select, SelectItem};
pub use separator::{Separator, SeparatorOrientation};
pub use skeleton::{Skeleton, SkeletonShape};
pub use spinner::{Spinner, SpinnerSize, SpinnerTone};
pub use switch::Switch;
pub use tooltip::{Tooltip, TooltipPlacement};

pub use motion::{
    cubic_bezier, ease_in_cubic, ease_out_cubic, init as init_motion, use_motion_value,
    AnimatePresence, Ease, Motion, MotionStyle, MotionValue, MotionValueStore, PresenceMode,
    Stagger, StyledSlot, Transition, Variants,
};

pub use theme::{init as init_theme, paint, rgb, ActiveTheme, Theme, ThemeKind, FONT_FAMILY};

/// Bind kit keystrokes (text fields). Call once at startup.
pub fn init(cx: &mut gpui::App) {
    input::init(cx);
}
