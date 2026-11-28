//! Ribbon contributions: plugins register buttons here instead of the sidebar hand-wiring them.

use dioxus::prelude::*;

use crate::command::CommandId;

/// Which ribbon group a contributed button sits in.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum RibbonSection {
	Top,
	Bottom,
}

/// A plugin-contributed ribbon button. `icon` is a builder because an `Element` can't be made at
/// setup time; `command` is invoked on click so ribbon buttons compose with the palette/hotkeys.
#[derive(Clone)]
pub struct RibbonItemSpec {
	pub icon: fn() -> Element,
	pub tooltip: String,
	pub section: RibbonSection,
	pub command: CommandId,
}

/// Contributed ribbon buttons, in registration order. Written by [`crate::App::add_ribbon_item`].
pub static RIBBON_ITEMS: GlobalSignal<Vec<RibbonItemSpec>> = Signal::global(Vec::new);
