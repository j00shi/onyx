//! The content workspace: a tab/split-pane tree that replaces the single router outlet.
//!
//! [`WORKSPACE`] holds the live tree (see [`model`]); [`WorkspaceView`] renders it.

use dioxus::prelude::*;

pub mod model;
mod pane;

pub use model::{Axis, Node, NodeId, Tab, TabId, View, Workspace};
pub use pane::*;

/// Global content workspace: the pane tree, its tabs, and focus. Mutate via `WORKSPACE.write()`
/// methods so the tree's invariants hold (see [`model::Workspace`]).
pub static WORKSPACE: GlobalSignal<Workspace> = Signal::global(Workspace::single_home);
