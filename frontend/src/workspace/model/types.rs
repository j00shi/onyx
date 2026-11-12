//! Value types for the pane tree: node/tab IDs, the [`View`] a tab shows, and the [`Node`] enum.

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
	pub(super) const fn axis(self) -> Axis {
		match self {
			Self::Left | Self::Right => Axis::Horizontal,
			Self::Top | Self::Bottom => Axis::Vertical,
		}
	}

	/// Whether the new pane is placed before the target along the axis.
	pub(super) const fn before(self) -> bool {
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
