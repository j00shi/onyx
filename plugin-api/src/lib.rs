//! The stable plugin contract for Onyx.
//!
//! This crate holds the [`Plugin`] trait, the [`App`] handle a plugin uses to contribute, and the
//! command registry. Core plugin crates depend only on this crate — never on `frontend` — so the
//! contract stays clean. Later a WASM/JS bridge implements the same [`Plugin`] trait, so nothing
//! here may assume native Rust.

mod api;
mod command;
mod plugin;

pub use api::App;
pub use command::{invoke_command, Command, CommandId, CommandRegistry, COMMANDS};
pub use plugin::{run_setup, Plugin, PLUGINS};
