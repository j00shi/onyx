//! The content workspace data model: a recursive tree of panes addressed by stable IDs.
//!
//! Nodes live in a flat [`HashMap`] keyed by [`NodeId`]; the tree is expressed through
//! parent/child ID references rather than nested ownership, so deep operations (split, close,
//! resize, drag between panes) can address any node cheaply. All structural edits go through the
//! methods on [`Workspace`] so invariants (parent links, fraction sums) stay intact — UI code must
//! never mutate `nodes` directly.

use std::collections::HashMap;

/// Stable identifier for a tree node. Never recycled, so handles stay valid across edits.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct NodeId(pub u64);

/// Stable identifier for a tab within a leaf.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct TabId(pub u64);

/// What a single tab displays. Replaces the old `Route` enum for content; extend as new
/// view kinds are added.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum View {
	Home,
	NewTab,
	Settings,
}

impl View {
	/// Human-readable label shown on the tab.
	pub const fn title(self) -> &'static str {
		match self {
			Self::Home => "Home",
			Self::NewTab => "New Tab",
			Self::Settings => "Settings",
		}
	}
}

/// One open tab inside a leaf. The title is derived from `view` for now.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Tab {
	pub id: TabId,
	pub view: View,
}

/// Orientation of a split: `Horizontal` lays its children out left→right (a vertical divider),
/// `Vertical` lays them top→bottom (a horizontal divider).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Axis {
	Horizontal,
	Vertical,
}

/// A node in the pane tree: either a tab group (`Leaf`) or a split container.
#[derive(Clone, PartialEq, Debug)]
pub enum Node {
	/// A tab group with its own tab bar — the workspace's actual "pane".
	Leaf {
		id: NodeId,
		parent: Option<NodeId>,
		tabs: Vec<Tab>,
		/// Index into `tabs` of the active tab.
		active: usize,
	},
	/// A split container. `children` and `fractions` are index-aligned; `fractions` sums to 1.0.
	Split { id: NodeId, parent: Option<NodeId>, axis: Axis, children: Vec<NodeId>, fractions: Vec<f32> },
}

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

	/// Append a tab to `leaf` and make it active.
	pub fn add_tab(&mut self, leaf: NodeId, view: View) {
		let id = self.alloc_tab_id();
		if let Some(Node::Leaf { tabs, active, .. }) = self.nodes.get_mut(&leaf) {
			tabs.push(Tab { id, view });
			*active = tabs.len() - 1;
		}
	}

	/// Make the tab at `idx` active within `leaf`.
	pub fn set_active(&mut self, leaf: NodeId, idx: usize) {
		if let Some(Node::Leaf { tabs, active, .. }) = self.nodes.get_mut(&leaf) {
			if idx < tabs.len() {
				*active = idx;
			}
		}
	}

	/// Close the tab at `idx` in `leaf`, keeping `active` pointed at a sensible tab.
	///
	/// A leaf must always hold at least one tab until split-aware leaf removal arrives, so an
	/// emptied leaf is re-seeded with a New Tab for now.
	pub fn close_tab(&mut self, leaf: NodeId, idx: usize) {
		let empty = {
			let Some(Node::Leaf { tabs, active, .. }) = self.nodes.get_mut(&leaf) else {
				return;
			};
			if idx >= tabs.len() {
				return;
			}
			tabs.remove(idx);
			if *active > idx {
				*active -= 1;
			}
			*active = (*active).min(tabs.len().saturating_sub(1));
			tabs.is_empty()
		};
		if empty {
			let id = self.alloc_tab_id();
			if let Some(Node::Leaf { tabs, active, .. }) = self.nodes.get_mut(&leaf) {
				tabs.push(Tab { id, view: View::NewTab });
				*active = 0;
			}
		}
	}

	/// Focus an existing tab showing `view` in the focused leaf, or open a new one there.
	pub fn open_view(&mut self, view: View) {
		let leaf = self.focused_leaf;
		if let Some(Node::Leaf { tabs, active, .. }) = self.nodes.get_mut(&leaf) {
			if let Some(idx) = tabs.iter().position(|t| t.view == view) {
				*active = idx;
				return;
			}
		}
		self.add_tab(leaf, view);
	}
}
