//! The content workspace's rendering layer: the pane tree as Dioxus components, plus the shared
//! tab-drag/drop state every pane reads. [`WorkspaceView`] is the entry point; [`split`] renders
//! split containers and their dividers, [`leaf`] renders tab groups.

use dioxus::prelude::*;

mod leaf;
mod split;

use super::model::{DropZone, Edge, Node, NodeId};
use super::WORKSPACE;
use leaf::LeafPane;
use split::SplitPane;

/// Where a dragged tab currently points: a pane body (edge → split, center → move to end) or a
/// specific insert slot in a pane's tab bar.
#[derive(Clone, Copy, PartialEq)]
enum DropTarget {
	Pane(NodeId, DropZone),
	Bar(NodeId, usize),
}

/// An in-progress drag of a tab out of its bar, tracked globally so any pane can be a drop target.
#[derive(Clone, PartialEq)]
struct TabDrag {
	/// Leaf the tab is being dragged from.
	source: NodeId,
	/// Index of the dragged tab within `source`.
	index: usize,
	/// Where the pointer currently points, if over a drop target.
	over: Option<DropTarget>,
}

/// The active tab drag, or `None`. Pressing a tab records a pending press; moving past
/// `DRAG_THRESHOLD` promotes it to a drag (whether it stays in the bar or leaves it).
static DRAG: GlobalSignal<Option<TabDrag>> = Signal::global(|| None);

/// Pointer travel (px) past which a press on a tab becomes a drag rather than a click.
const DRAG_THRESHOLD: f64 = 4.0;

/// Update the current drop target (no-op if unchanged or no drag is active).
fn set_over(target: DropTarget) {
	let Some(mut d) = DRAG() else {
		return;
	};
	if d.over != Some(target) {
		d.over = Some(target);
		*DRAG.write() = Some(d);
	}
}

/// Apply the current drop target to the workspace and end the drag.
fn commit_drag() {
	if let Some(TabDrag { source, index, over: Some(target) }) = DRAG() {
		match target {
			DropTarget::Pane(t, DropZone::Edge(edge)) => {
				WORKSPACE.write().move_tab_to_edge(source, index, t, edge);
			},
			DropTarget::Pane(t, DropZone::Center) => WORKSPACE.write().move_tab_to_leaf(source, index, t),
			DropTarget::Bar(dest, at) => WORKSPACE.write().move_tab_to_leaf_at(source, index, dest, at),
		}
	}
	end_drag();
}

/// Clear the drag and its visual hint.
fn end_drag() {
	*DRAG.write() = None;
	*DROP_HINT.write() = None;
}

/// Classify a pointer at `(x, y)` within a `w`×`h` pane into a drop zone: an edge band near each
/// side, or the center otherwise.
fn drop_zone(x: f64, y: f64, w: f64, h: f64) -> DropZone {
	if w <= 0.0 || h <= 0.0 {
		return DropZone::Center;
	}
	let (left, right, top, bottom) = (x, w - x, y, h - y);
	let nearest = left.min(right).min(top).min(bottom);
	if nearest > w.min(h) * 0.25 {
		DropZone::Center
	} else if left <= right && left <= top && left <= bottom {
		DropZone::Edge(Edge::Left)
	} else if right <= top && right <= bottom {
		DropZone::Edge(Edge::Right)
	} else if top <= bottom {
		DropZone::Edge(Edge::Top)
	} else {
		DropZone::Edge(Edge::Bottom)
	}
}

/// Viewport-pixel rectangle the drop indicator should occupy for the zone a pointer at `(x, y)`
/// (element-relative) is in, given the pane's viewport origin `(ox, oy)` and size `w`×`h`.
fn zone_rect(x: f64, y: f64, ox: f64, oy: f64, w: f64, h: f64) -> DropHint {
	let (rx, ry, rw, rh) = match drop_zone(x, y, w, h) {
		DropZone::Center => (ox, oy, w, h),
		DropZone::Edge(Edge::Left) => (ox, oy, w / 2.0, h),
		DropZone::Edge(Edge::Right) => (ox + w / 2.0, oy, w / 2.0, h),
		DropZone::Edge(Edge::Top) => (ox, oy, w, h / 2.0),
		DropZone::Edge(Edge::Bottom) => (ox, oy + h / 2.0, w, h / 2.0),
	};
	DropHint { x: rx, y: ry, w: rw, h: rh, line: false }
}

/// The drop indicator's target rectangle in viewport pixels. A single element tweens between
/// these, so the highlight glides between zones and narrows into a line over a tab bar.
#[derive(Clone, Copy, PartialEq)]
struct DropHint {
	x: f64,
	y: f64,
	w: f64,
	h: f64,
	/// Draw as a slim bar-insertion line rather than a filled zone.
	line: bool,
}

/// Where the drop indicator should be drawn, or `None` when nothing is hovered.
static DROP_HINT: GlobalSignal<Option<DropHint>> = Signal::global(|| None);

/// Update the drop hint, skipping redundant writes so the indicator only re-renders on change.
fn set_hint(hint: DropHint) {
	if *DROP_HINT.read() != Some(hint) {
		*DROP_HINT.write() = Some(hint);
	}
}

/// A single viewport-fixed element that animates (via CSS transitions) between drop zones and
/// bar-insertion lines as [`DROP_HINT`] changes.
#[component]
fn DropIndicator() -> Element {
	let Some(h) = DROP_HINT() else {
		return rsx! {};
	};
	rsx! {
		div {
			class: "drop-indicator",
			"data-line": h.line,
			style: "left: {h.x}px; top: {h.y}px; width: {h.w}px; height: {h.h}px;",
		}
	}
}

/// Root of the content workspace: renders the pane tree from the root node down.
#[component]
pub fn WorkspaceView() -> Element {
	let root = WORKSPACE.read().root();
	rsx! {
		document::Link { rel: "stylesheet", href: asset!("./pane.css") }
		div {
			class: "workspace",
			// Releasing a tab drag anywhere that isn't a pane drop target cancels it (pane overlays
			// stop propagation so their own release commits instead).
			onpointerup: move |_| {
				if DRAG().is_some() {
					end_drag();
				}
			},
			PaneNode { id: root }
			DropIndicator {}
		}
	}
}

/// Renders a single node: a [`LeafPane`] for a leaf, or a [`SplitPane`] for a split. Only one of
/// the two arms is reachable per node.
#[component]
fn PaneNode(id: NodeId) -> Element {
	let node = WORKSPACE.read().node(id).cloned();
	match node {
		Some(Node::Leaf { .. }) => rsx! {
			LeafPane { id }
		},
		Some(Node::Split { .. }) => rsx! {
			SplitPane { id }
		},
		None => rsx! {},
	}
}
