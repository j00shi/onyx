//! The content workspace data model: a recursive tree of panes addressed by stable IDs.
//!
//! Nodes live in a flat [`HashMap`] keyed by [`NodeId`]; the tree is expressed through
//! parent/child ID references rather than nested ownership, so deep operations (split, close,
//! resize, drag between panes) can address any node cheaply. All structural edits go through the
//! methods on [`Workspace`] so invariants (parent links, fraction sums) stay intact — UI code must
//! never mutate `nodes` directly.
//!
//! The methods are split across modules by concern: [`tabs`] holds in-leaf tab operations and
//! [`tree`] holds structural edits (split, move, close, collapse); the shared primitives
//! (construction, node lookup, ID allocation) live here.

use std::collections::HashMap;

mod tabs;
#[cfg(test)]
mod tests;
mod tree;
mod types;

pub use types::{Axis, DropZone, Edge, Node, NodeId, Tab, TabId, View};

/// The content workspace: the pane tree, its root, and the focused leaf.
#[derive(Clone, PartialEq, Debug)]
pub struct Workspace {
	nodes: HashMap<NodeId, Node>,
	root: NodeId,
	/// Leaf that receives "open here" actions (e.g. the settings gear).
	focused_leaf: NodeId,
	/// Monotonic ID counter shared by nodes and tabs; never decreases, so IDs are never reused.
	next_id: u64,
}

impl Workspace {
	/// A fresh workspace: a single root leaf holding one Home tab.
	pub fn single_home() -> Self {
		let mut ws = Self { nodes: HashMap::new(), root: NodeId(0), focused_leaf: NodeId(0), next_id: 0 };
		let root = ws.alloc_node_id();
		let tab = ws.alloc_tab_id();
		ws.root = root;
		ws.focused_leaf = root;
		ws.nodes.insert(
			root,
			Node::Leaf { id: root, parent: None, tabs: vec![Tab { id: tab, view: View::Home }], active: 0 },
		);
		ws
	}

	/// The root node's ID.
	pub const fn root(&self) -> NodeId {
		self.root
	}

	/// Borrow a node by ID.
	pub fn node(&self, id: NodeId) -> Option<&Node> {
		self.nodes.get(&id)
	}

	const fn alloc_node_id(&mut self) -> NodeId {
		let id = NodeId(self.next_id);
		self.next_id += 1;
		id
	}

	const fn alloc_tab_id(&mut self) -> TabId {
		let id = TabId(self.next_id);
		self.next_id += 1;
		id
	}

	fn parent_of(&self, id: NodeId) -> Option<NodeId> {
		match self.nodes.get(&id)? {
			Node::Leaf { parent, .. } | Node::Split { parent, .. } => *parent,
		}
	}

	/// Remove and return the tab at `idx` in `leaf`, fixing `active` to stay on a valid tab. Does
	/// not touch the leaf itself when it empties — callers prune it afterwards.
	fn take_tab(&mut self, leaf: NodeId, idx: usize) -> Option<Tab> {
		let Some(Node::Leaf { tabs, active, .. }) = self.nodes.get_mut(&leaf) else {
			return None;
		};
		if idx >= tabs.len() {
			return None;
		}
		let tab = tabs.remove(idx);
		if *active > idx {
			*active -= 1;
		}
		*active = (*active).min(tabs.len().saturating_sub(1));
		Some(tab)
	}
}
