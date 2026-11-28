//! The stable plugin contract for Onyx.
//!
//! This crate holds the [`Plugin`] trait, the [`App`] handle a plugin uses to contribute, and the
//! command registry. Core plugin crates depend only on this crate — never on `frontend` — so the
//! contract stays clean. Later a WASM/JS bridge implements the same [`Plugin`] trait, so nothing
//! here may assume native Rust.

mod api;
mod command;
mod hotkey;
mod plugin;
mod ribbon;
mod view;

pub use api::App;
pub use command::{invoke_command, Command, CommandId, CommandRegistry, COMMANDS};
pub use hotkey::{KeyChord, HOTKEYS};
pub use plugin::{run_setup, Plugin, PLUGINS};
pub use ribbon::{RibbonItemSpec, RibbonSection, RIBBON_ITEMS};
pub use view::{ViewId, ViewSpec, OPEN_VIEW, VIEWS};
