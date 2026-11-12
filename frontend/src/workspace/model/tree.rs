//! Structural edits to the pane tree: splitting, moving tabs between panes, and closing tabs
//! (which may prune empty leaves and collapse one-child splits). These keep the parent links and
//! per-split fraction sums intact.

use super::{Edge, Node, NodeId, Tab, View, Workspace};

impl Workspace {
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

	/// Close the tab at `idx` in `leaf`. When that empties the leaf, remove the leaf and collapse
	/// its parent split if it drops to a single child. The root leaf can't be removed, so emptying
	/// it re-seeds a New Tab instead (the workspace always keeps at least one pane).
	pub fn close_tab(&mut self, leaf: NodeId, idx: usize) {
		if self.take_tab(leaf, idx).is_some() {
			self.prune_leaf(leaf);
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
		let Some(Node::Leaf { tabs, .. }) = self.nodes.get(&dest) else {
			return;
		};
		self.move_tab_to_leaf_at(source, idx, dest, tabs.len());
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
