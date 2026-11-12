//! In-leaf tab operations that don't change the tree's shape: adding, activating, and the
//! focus-or-open helper behind "open here" actions.

use super::{Node, NodeId, Tab, View, Workspace};

impl Workspace {
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
			self.focused_leaf = leaf;
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
