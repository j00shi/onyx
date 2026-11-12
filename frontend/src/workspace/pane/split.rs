//! Split containers: lay children along an axis with a draggable divider between each pair.

use dioxus::prelude::*;

use super::super::model::{Axis, Node, NodeId};
use super::super::WORKSPACE;
use super::PaneNode;

/// Minimum pixels a pane can be squeezed to while dragging a divider.
const MIN_PANE_PX: f64 = 100.0;

/// Live state while a divider is dragged. Fractions are kept local during the drag and only
/// committed to `WORKSPACE` on release, so a pointer-move doesn't re-render the whole tree.
#[derive(Clone, PartialEq)]
struct DividerDrag {
	/// Divider sits between child `index` and `index + 1`.
	index: usize,
	/// Pointer position (px) along the axis at drag start.
	start: f64,
	/// Fractions at drag start (the base the drag is applied to).
	base: Vec<f32>,
	/// Fractions to render during the drag.
	live: Vec<f32>,
}

/// Shift `delta_px` of the container between adjacent panes `a` and `b`, clamping each to
/// `MIN_PANE_PX`. Returns the new `(a, b)` fractions (their sum is preserved).
#[allow(clippy::cast_possible_truncation)] // fractions are f32; sub-f32 precision is irrelevant here
fn resize_pair(a: f32, b: f32, delta_px: f64, container_px: f64) -> (f32, f32) {
	let total = a + b;
	let delta = (delta_px / container_px) as f32;
	let min = (MIN_PANE_PX / container_px) as f32;
	let hi = total - min;
	let new_a = if min <= hi { (a + delta).clamp(min, hi) } else { total / 2.0 };
	(new_a, total - new_a)
}

/// A split container: lays its children along one axis with a draggable divider between each pair.
#[component]
pub(super) fn SplitPane(id: NodeId) -> Element {
	let node = WORKSPACE.read().node(id).cloned();
	let Some(Node::Split { axis, children, fractions, .. }) = node else {
		return rsx! {};
	};

	// Container size along the axis (px), kept current via the ResizeObserver; drives px→fraction.
	let mut container_px = use_signal(|| 0.0_f64);
	let mut drag = use_signal(|| None::<DividerDrag>);

	let horizontal = axis == Axis::Horizontal;
	let flow = if horizontal { "row" } else { "column" };
	let axis_attr = if horizontal { "horizontal" } else { "vertical" };

	// Render the live fractions mid-drag, otherwise the model's.
	let fracs = drag.read().as_ref().map_or_else(|| fractions.clone(), |d| d.live.clone());

	rsx! {
		div {
			class: "pane-split",
			style: "flex-direction: {flow};",
			onresize: move |e| {
				if let Ok(size) = e.get_border_box_size() {
					container_px.set(if horizontal { size.width } else { size.height });
				}
			},
			for (i , child) in children.iter().enumerate() {
				div {
					key: "{child.0}",
					class: "pane-split-child",
					style: "flex: {fracs.get(i).copied().unwrap_or(0.0)} 1 0;",
					PaneNode { id: *child }
				}
				if i + 1 < children.len() {
					div {
						class: "pane-divider",
						"data-axis": axis_attr,
						onpointerdown: move |e| {
							let p = if horizontal { e.client_coordinates().x } else { e.client_coordinates().y };
							if let Some(Node::Split { fractions, .. }) = WORKSPACE.read().node(id).cloned() {
								drag.set(Some(DividerDrag { index: i, start: p, base: fractions.clone(), live: fractions }));
							}
						},
					}
				}
			}
		}
		if drag.read().is_some() {
			div {
				class: "resize-overlay",
				"data-axis": axis_attr,
				onpointermove: move |e| {
					let s = container_px();
					let Some(mut d) = drag() else {
						return;
					};
					if s <= 0.0 {
						return;
					}
					let p = if horizontal { e.client_coordinates().x } else { e.client_coordinates().y };
					if let (Some(&a), Some(&b)) = (d.base.get(d.index), d.base.get(d.index + 1)) {
						let (na, nb) = resize_pair(a, b, p - d.start, s);
						if let Some(x) = d.live.get_mut(d.index) {
							*x = na;
						}
						if let Some(x) = d.live.get_mut(d.index + 1) {
							*x = nb;
						}
						drag.set(Some(d));
					}
				},
				onpointerup: move |_| {
					if let Some(d) = drag() {
						WORKSPACE.write().set_fractions(id, d.live);
					}
					drag.set(None);
				},
				onpointercancel: move |_| drag.set(None),
			}
		}
	}
}
