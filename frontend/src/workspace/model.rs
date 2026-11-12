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

/// A side of a pane to split toward. Maps to an [`Axis`] plus whether the new pane goes before
/// or after the target in that axis.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Edge {
	Left,
	Right,
	Top,
	Bottom,
}

impl Edge {
	const fn axis(self) -> Axis {
		match self {
			Self::Left | Self::Right => Axis::Horizontal,
			Self::Top | Self::Bottom => Axis::Vertical,
		}
	}

	/// Whether the new pane is placed before the target along the axis.
	const fn before(self) -> bool {
		matches!(self, Self::Left | Self::Top)
	}
}

/// Where a dragged tab would land when dropped over a pane: against an [`Edge`] (creating a split)
/// or into the pane's tab list (`Center`).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DropZone {
	Edge(Edge),
	Center,
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
		let is_leaf = if let Some(Node::Leaf { tabs, active, .. }) = self.nodes.get_mut(&leaf) {
			if idx < tabs.len() {
				*active = idx;
			}
			true
		} else {
			false
		};
		if is_leaf {
			self.focused_leaf = leaf;
		}
	}

	/// Split `target`'s pane toward `edge`, putting a new tab of `view` in the new leaf.
	///
	/// Splitting along the parent split's own axis *flattens* (the new leaf joins its siblings and
	/// all fractions reset to equal, so half→thirds→quarters); any other case *nests* a fresh
	/// split. Returns the new leaf's ID.
	pub fn split(&mut self, target: NodeId, edge: Edge, view: View) -> Option<NodeId> {
		let tab = Tab { id: self.alloc_tab_id(), view };
		self.split_with_tab(target, edge, tab)
	}

	/// Split `target`'s pane toward `edge`, placing the given `tab` in the new leaf. See [`split`]
	/// for the flatten-vs-nest rule.
	///
	/// [`split`]: Self::split
	fn split_with_tab(&mut self, target: NodeId, edge: Edge, tab: Tab) -> Option<NodeId> {
		let parent = match self.nodes.get(&target)? {
			Node::Leaf { parent, .. } | Node::Split { parent, .. } => *parent,
		};
		let axis = edge.axis();
		let before = edge.before();

		let new_leaf = self.alloc_node_id();

		// Flatten only when the target already sits in a split of the same axis.
		let flatten = matches!(
			parent.and_then(|p| self.nodes.get(&p)),
			Some(Node::Split { axis: a, .. }) if *a == axis
		);

		if flatten {
			let parent = parent?;
			self.nodes.insert(new_leaf, Node::Leaf { id: new_leaf, parent: Some(parent), tabs: vec![tab], active: 0 });
			if let Some(Node::Split { children, fractions, .. }) = self.nodes.get_mut(&parent) {
				if let Some(pos) = children.iter().position(|c| *c == target) {
					children.insert(if before { pos } else { pos + 1 }, new_leaf);
					*fractions = equal_fractions(children.len());
				}
			}
		} else {
			let split = self.alloc_node_id();
			self.nodes.insert(new_leaf, Node::Leaf { id: new_leaf, parent: Some(split), tabs: vec![tab], active: 0 });
			let children = if before { vec![new_leaf, target] } else { vec![target, new_leaf] };
			self.nodes.insert(split, Node::Split { id: split, parent, axis, children, fractions: vec![0.5, 0.5] });
			self.set_parent(target, Some(split));
			match parent {
				// Replace the target's slot in its old parent with the new split.
				Some(gp) => {
					if let Some(Node::Split { children, .. }) = self.nodes.get_mut(&gp) {
						if let Some(pos) = children.iter().position(|c| *c == target) {
							children[pos] = split;
						}
					}
				},
				// The target was the root, so the new split becomes the root.
				None => self.root = split,
			}
		}
		self.focused_leaf = new_leaf;
		Some(new_leaf)
	}

	fn set_parent(&mut self, id: NodeId, parent: Option<NodeId>) {
		if let Some(Node::Leaf { parent: p, .. } | Node::Split { parent: p, .. }) = self.nodes.get_mut(&id) {
			*p = parent;
		}
	}

	/// Replace a split's child `fractions` (e.g. after a divider drag). Ignored unless the lengths
	/// match the current child count.
	pub fn set_fractions(&mut self, split: NodeId, fractions: Vec<f32>) {
		if let Some(Node::Split { children, fractions: f, .. }) = self.nodes.get_mut(&split) {
			if f.len() == fractions.len() && children.len() == fractions.len() {
				*f = fractions;
			}
		}
	}

	/// Move the tab at `from` to index `to` within `leaf`, keeping the same tab active.
	pub fn reorder_tab(&mut self, leaf: NodeId, from: usize, to: usize) {
		if let Some(Node::Leaf { tabs, active, .. }) = self.nodes.get_mut(&leaf) {
			if from >= tabs.len() || to >= tabs.len() || from == to {
				return;
			}
			let Some(active_id) = tabs.get(*active).map(|t| t.id) else {
				return;
			};
			let tab = tabs.remove(from);
			tabs.insert(to, tab);
			if let Some(idx) = tabs.iter().position(|t| t.id == active_id) {
				*active = idx;
			}
		}
	}

	/// Close the tab at `idx` in `leaf`. When that empties the leaf, remove the leaf and collapse
	/// its parent split if it drops to a single child. The root leaf can't be removed, so emptying
	/// it re-seeds a New Tab instead (the workspace always keeps at least one pane).
	pub fn close_tab(&mut self, leaf: NodeId, idx: usize) {
		if self.take_tab(leaf, idx).is_some() {
			self.prune_leaf(leaf);
		}
	}

	/// Remove and return the tab at `idx` in `leaf`, fixing `active` to stay on a valid tab. Does
	/// not touch the leaf itself when it empties — call [`prune_leaf`] for that.
	///
	/// [`prune_leaf`]: Self::prune_leaf
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

	/// After a tab removal, fix up an emptied leaf: remove + collapse a non-root leaf, or re-seed a
	/// New Tab in the root leaf (which can never be removed). No-op if the leaf still has tabs.
	fn prune_leaf(&mut self, leaf: NodeId) {
		let empty = matches!(self.nodes.get(&leaf), Some(Node::Leaf { tabs, .. }) if tabs.is_empty());
		if !empty {
			return;
		}
		if self.parent_of(leaf).is_some() {
			let was_focused = self.focused_leaf == leaf;
			self.remove_leaf(leaf);
			if was_focused {
				self.focused_leaf = self.first_leaf(self.root);
			}
		} else {
			let id = self.alloc_tab_id();
			if let Some(Node::Leaf { tabs, active, .. }) = self.nodes.get_mut(&leaf) {
				tabs.push(Tab { id, view: View::NewTab });
				*active = 0;
			}
		}
	}

	/// Move the tab at `(source, idx)` into a new leaf created by splitting `target` toward `edge`.
	/// Dropping a tab on its own pane when it's the only tab is a no-op.
	pub fn move_tab_to_edge(&mut self, source: NodeId, idx: usize, target: NodeId, edge: Edge) {
		if source == target && matches!(self.nodes.get(&source), Some(Node::Leaf { tabs, .. }) if tabs.len() <= 1) {
			return;
		}
		let Some(tab) = self.take_tab(source, idx) else {
			return;
		};
		self.split_with_tab(target, edge, tab);
		self.prune_leaf(source);
	}

	/// Move the tab at `(source, idx)` into `dest`'s tab list and focus it. No-op within one leaf.
	pub fn move_tab_to_leaf(&mut self, source: NodeId, idx: usize, dest: NodeId) {
		if source == dest {
			return;
		}
		let Some(tab) = self.take_tab(source, idx) else {
			return;
		};
		let placed = if let Some(Node::Leaf { tabs, active, .. }) = self.nodes.get_mut(&dest) {
			tabs.push(tab);
			*active = tabs.len() - 1;
			true
		} else {
			false
		};
		if placed {
			self.focused_leaf = dest;
		}
		self.prune_leaf(source);
	}

	/// Move the tab at `(source, idx)` into `dest` at index `dest_idx`, focusing it. Works within a
	/// single leaf too (a reorder), accounting for the shift left by removing the tab first.
	pub fn move_tab_to_leaf_at(&mut self, source: NodeId, idx: usize, dest: NodeId, dest_idx: usize) {
		let Some(tab) = self.take_tab(source, idx) else {
			return;
		};
		let at = if source == dest && idx < dest_idx { dest_idx - 1 } else { dest_idx };
		let placed = if let Some(Node::Leaf { tabs, active, .. }) = self.nodes.get_mut(&dest) {
			let at = at.min(tabs.len());
			tabs.insert(at, tab);
			*active = at;
			true
		} else {
			false
		};
		if placed {
			self.focused_leaf = dest;
		}
		self.prune_leaf(source);
	}

	/// Remove an (assumed empty, non-root) leaf: drop it from its parent split, redistribute the
	/// freed space across the remaining siblings, and collapse the split if only one child is left.
	fn remove_leaf(&mut self, leaf: NodeId) {
		let Some(parent) = self.parent_of(leaf) else {
			return;
		};
		self.nodes.remove(&leaf);

		let mut survivor = None;
		if let Some(Node::Split { children, fractions, .. }) = self.nodes.get_mut(&parent) {
			if let Some(pos) = children.iter().position(|c| *c == leaf) {
				children.remove(pos);
				if pos < fractions.len() {
					fractions.remove(pos);
				}
				normalize(fractions);
			}
			if children.len() == 1 {
				survivor = children.first().copied();
			}
		}
		if let Some(survivor) = survivor {
			self.collapse_split(parent, survivor);
		}
	}

	/// Replace a one-child `split` with its `survivor`, which inherits the split's slot (and its
	/// fraction) in the grandparent, or becomes the root.
	fn collapse_split(&mut self, split: NodeId, survivor: NodeId) {
		let grandparent = self.parent_of(split);
		self.set_parent(survivor, grandparent);
		match grandparent {
			Some(gp) => {
				if let Some(Node::Split { children, .. }) = self.nodes.get_mut(&gp) {
					if let Some(pos) = children.iter().position(|c| *c == split) {
						children[pos] = survivor;
					}
				}
			},
			None => self.root = survivor,
		}
		self.nodes.remove(&split);
	}

	fn parent_of(&self, id: NodeId) -> Option<NodeId> {
		match self.nodes.get(&id)? {
			Node::Leaf { parent, .. } | Node::Split { parent, .. } => *parent,
		}
	}

	/// Descend from `from` to its first leaf (used to re-home focus after a removal).
	fn first_leaf(&self, from: NodeId) -> NodeId {
		let mut cur = from;
		while let Some(Node::Split { children, .. }) = self.nodes.get(&cur) {
			match children.first() {
				Some(&first) => cur = first,
				None => break,
			}
		}
		cur
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

/// Even fractions summing to 1.0 for `n` siblings.
#[allow(clippy::cast_precision_loss)] // `n` is a small pane count, so the usize→f32 cast is exact.
fn equal_fractions(n: usize) -> Vec<f32> {
	if n == 0 {
		return Vec::new();
	}
	vec![1.0 / n as f32; n]
}

/// Rescale `fractions` to sum to 1.0, preserving their relative sizes (falling back to equal
/// shares if they sum to zero).
#[allow(clippy::cast_precision_loss)] // small pane count; the usize→f32 cast is exact.
fn normalize(fractions: &mut [f32]) {
	let sum: f32 = fractions.iter().sum();
	if sum > 0.0 {
		for f in &mut *fractions {
			*f /= sum;
		}
	} else if !fractions.is_empty() {
		let eq = 1.0 / fractions.len() as f32;
		for f in &mut *fractions {
			*f = eq;
		}
	}
}

#[cfg(test)]
mod tests {
	use super::{Axis, Edge, Node, NodeId, View, Workspace};

	/// A split's `(axis, children, fractions)`, or `None` if `id` isn't a split.
	fn split_of(ws: &Workspace, id: NodeId) -> Option<(Axis, Vec<NodeId>, Vec<f32>)> {
		match ws.node(id) {
			Some(Node::Split { axis, children, fractions, .. }) => Some((*axis, children.clone(), fractions.clone())),
			_ => None,
		}
	}

	fn children_of(ws: &Workspace, id: NodeId) -> Vec<NodeId> {
		split_of(ws, id).map(|s| s.1).unwrap_or_default()
	}

	/// True if every fraction is within epsilon of `expected`.
	fn all_near(fracs: &[f32], expected: f32) -> bool {
		fracs.iter().all(|f| (f - expected).abs() < 1e-6)
	}

	#[test]
	fn single_home_is_one_root_leaf() {
		let ws = Workspace::single_home();
		assert_eq!(ws.nodes.len(), 1);
		let root = ws.node(ws.root());
		assert!(matches!(root, Some(Node::Leaf { .. })));
		if let Some(Node::Leaf { tabs, active, parent, .. }) = root {
			assert_eq!(tabs.len(), 1);
			assert_eq!(*active, 0);
			assert!(parent.is_none());
		}
	}

	#[test]
	fn split_on_root_leaf_nests_a_split() {
		let mut ws = Workspace::single_home();
		let leaf = ws.root();
		ws.split(leaf, Edge::Right, View::NewTab);

		let split = split_of(&ws, ws.root());
		assert_eq!(split.as_ref().map(|s| s.0), Some(Axis::Horizontal));
		assert_eq!(split.as_ref().map(|s| s.1.len()), Some(2));
		assert!(split.is_some_and(|s| all_near(&s.2, 0.5)));
	}

	#[test]
	fn same_axis_split_flattens_to_equal_thirds() {
		let mut ws = Workspace::single_home();
		let leaf = ws.root();
		ws.split(leaf, Edge::Right, View::NewTab); // root → horizontal split of 2
		if let Some(&child) = children_of(&ws, ws.root()).first() {
			ws.split(child, Edge::Right, View::NewTab); // same axis → flatten to 3
		}

		let split = split_of(&ws, ws.root());
		assert_eq!(split.as_ref().map(|s| s.1.len()), Some(3));
		assert!(split.is_some_and(|s| all_near(&s.2, 1.0 / 3.0)));
	}

	#[test]
	fn perpendicular_split_nests_inside_the_child() {
		let mut ws = Workspace::single_home();
		let leaf = ws.root();
		ws.split(leaf, Edge::Right, View::NewTab); // root → horizontal split of 2
		let first = children_of(&ws, ws.root()).first().copied();
		if let Some(child) = first {
			ws.split(child, Edge::Bottom, View::NewTab); // perpendicular → nests a vertical split
		}

		// Root is still a horizontal split of 2; its first child became a vertical split of 2.
		assert_eq!(split_of(&ws, ws.root()).map(|s| s.0), Some(Axis::Horizontal));
		if let Some(child) = children_of(&ws, ws.root()).first().copied() {
			assert_eq!(split_of(&ws, child).map(|s| s.0), Some(Axis::Vertical));
			assert_eq!(split_of(&ws, child).map(|s| s.1.len()), Some(2));
		}
	}

	/// Count of leaves reachable from the root.
	fn leaf_count(ws: &Workspace) -> usize {
		ws.nodes.values().filter(|n| matches!(n, Node::Leaf { .. })).count()
	}

	#[test]
	fn closing_a_split_leaf_collapses_back_to_the_other() {
		let mut ws = Workspace::single_home();
		let root_leaf = ws.root();
		let new = ws.split(root_leaf, Edge::Right, View::NewTab);
		assert!(matches!(ws.node(ws.root()), Some(Node::Split { .. })));

		// Close the lone tab in the new pane; its leaf is removed and the split collapses.
		if let Some(new) = new {
			ws.close_tab(new, 0);
		}
		assert_eq!(ws.nodes.len(), 1);
		assert_eq!(leaf_count(&ws), 1);
		assert!(matches!(ws.node(ws.root()), Some(Node::Leaf { .. })));
		// The surviving leaf is the root with no parent.
		assert_eq!(ws.parent_of(ws.root()), None);
	}

	#[test]
	fn closing_one_of_three_keeps_a_normalized_two_child_split() {
		let mut ws = Workspace::single_home();
		let root_leaf = ws.root();
		ws.split(root_leaf, Edge::Right, View::NewTab); // 2 children
		let victim = children_of(&ws, ws.root()).get(1).copied();
		if let Some(v) = victim {
			ws.split(v, Edge::Right, View::NewTab); // same axis → 3 children
		}
		assert_eq!(split_of(&ws, ws.root()).map(|s| s.1.len()), Some(3));

		// Close a leaf: the split drops to 2 children with fractions still summing to 1.
		if let Some(&first) = children_of(&ws, ws.root()).first() {
			ws.close_tab(first, 0);
		}
		let split = split_of(&ws, ws.root());
		assert_eq!(split.as_ref().map(|s| s.1.len()), Some(2));
		if let Some((_, _, fracs)) = split {
			let sum: f32 = fracs.iter().sum();
			assert!((sum - 1.0).abs() < 1e-6);
		}
	}

	#[test]
	fn closing_the_last_tab_of_the_root_leaf_reseeds_a_new_tab() {
		let mut ws = Workspace::single_home();
		let root = ws.root();
		ws.close_tab(root, 0);
		// Root stays a single leaf holding one New Tab.
		assert_eq!(leaf_count(&ws), 1);
		match ws.node(root) {
			Some(Node::Leaf { tabs, .. }) => {
				assert_eq!(tabs.len(), 1);
				assert_eq!(tabs.first().map(|t| t.view), Some(View::NewTab));
			},
			_ => assert!(ws.node(root).is_some()),
		}
	}

	#[test]
	fn move_tab_to_edge_on_own_pane_splits_it() {
		let mut ws = Workspace::single_home();
		let leaf = ws.root();
		ws.add_tab(leaf, View::Settings); // two tabs so the pane can give one away
		ws.move_tab_to_edge(leaf, 0, leaf, Edge::Right);
		assert_eq!(leaf_count(&ws), 2);
		assert!(matches!(ws.node(ws.root()), Some(Node::Split { .. })));
	}

	#[test]
	fn move_tab_to_edge_is_noop_for_a_single_tab_own_pane() {
		let mut ws = Workspace::single_home();
		let leaf = ws.root();
		ws.move_tab_to_edge(leaf, 0, leaf, Edge::Right);
		assert_eq!(leaf_count(&ws), 1);
		assert!(matches!(ws.node(ws.root()), Some(Node::Leaf { .. })));
	}

	#[test]
	fn moving_a_leafs_only_tab_into_another_collapses_the_split() {
		let mut ws = Workspace::single_home();
		let root_leaf = ws.root();
		let right = ws.split(root_leaf, Edge::Right, View::Home);
		assert_eq!(leaf_count(&ws), 2);

		// Move the left pane's only tab into the right pane: left empties and the split collapses.
		let left = children_of(&ws, ws.root()).first().copied();
		if let (Some(left), Some(right)) = (left, right) {
			ws.move_tab_to_leaf(left, 0, right);
		}
		assert_eq!(leaf_count(&ws), 1);
		match ws.node(ws.root()) {
			Some(Node::Leaf { tabs, .. }) => assert_eq!(tabs.len(), 2),
			_ => assert!(matches!(ws.node(ws.root()), Some(Node::Leaf { .. }))),
		}
	}
}
