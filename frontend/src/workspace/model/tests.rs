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

/// Views of a leaf's tabs, for order assertions.
fn tab_views(ws: &Workspace, leaf: NodeId) -> Vec<View> {
	match ws.node(leaf) {
		Some(Node::Leaf { tabs, .. }) => tabs.iter().map(|t| t.view).collect(),
		_ => Vec::new(),
	}
}

#[test]
fn move_within_leaf_to_end_reorders_and_follows_focus() {
	let mut ws = Workspace::single_home(); // [Home]
	let leaf = ws.root();
	ws.add_tab(leaf, View::Settings); // [Home, Settings]
	ws.add_tab(leaf, View::NewTab); // [Home, Settings, NewTab]
	ws.move_tab_to_leaf_at(leaf, 0, leaf, 3); // move Home to the end
	assert_eq!(tab_views(&ws, leaf), vec![View::Settings, View::NewTab, View::Home]);
	assert!(matches!(ws.node(leaf), Some(Node::Leaf { active: 2, .. })));
}

#[test]
fn move_within_leaf_to_front_reorders() {
	let mut ws = Workspace::single_home(); // [Home]
	let leaf = ws.root();
	ws.add_tab(leaf, View::Settings); // [Home, Settings]
	ws.move_tab_to_leaf_at(leaf, 1, leaf, 0); // move Settings to the front
	assert_eq!(tab_views(&ws, leaf), vec![View::Settings, View::Home]);
}
