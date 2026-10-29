use dioxus::prelude::*;

use crate::components::sidebar::{
	Sidebar, SidebarCollapsible, SidebarInset, SidebarProvider, SidebarSide, SidebarVariant,
};
use crate::views::{MOBILE_REACH_LAYOUT, SIDEBAR_VARIANT};

/// Minimal and maximal width for the sidebar
const MIN_WIDTH: f64 = 200.0;
const MAX_WIDTH: f64 = 500.0;
/// Dragging the expanded sidebar narrower than `COLLAPSE_BELOW` collapses it; dragging the
/// collapsed sidebar wider than `EXPAND_ABOVE` re-expands it (gap needed to prevent flickering)
const COLLAPSE_BELOW: f64 = 120.0;
const EXPAND_ABOVE: f64 = 160.0;
/// Visible width of the collapsed edge. Floating adds 1rem of container
/// padding (`3rem + 1rem - 2px`); the docked sidebar is just the icon width (`3rem`).
const ICON_WIDTH_FLOATING: f64 = 62.0;
const ICON_WIDTH_SIDEBAR: f64 = 48.0;
/// Pointer travel (px) below which a press on the handle counts as a click, not a drag
const CLICK_THRESHOLD: f64 = 4.0;

/// Shared sidebar scaffolding: the resizable, collapsible panel with its drag handle and
/// transitions.
#[component]
pub fn BaseSidebar(
	/// Used to build the per-sidebar classes, e.g. `"primary"` -> `primary-sidebar-wrapper` and
	/// `primary-sidebar`
	name: String,
	/// Resolved physical side the sidebar docks on (drives drag direction and the inner edge)
	side: SidebarSide,
	/// Starting width in px
	#[props(default = 256.0)]
	default_width: f64,
	/// The sidebar's own content; its root must carry the `sidebar-body` class
	body: Element,
	/// Content shown in the inset (nested sidebars / outlet)
	children: Element,
) -> Element {
	let mut width = use_signal(|| default_width); // Sidebar width (px)
	let mut open = use_signal(|| true); // True when sidebar is open
	let mut dragging = use_signal(|| false); // True when handle is being dragged
	let mut moved = use_signal(|| false); // True once a press has travelled past `CLICK_THRESHOLD`
	let mut start_x = use_signal(|| 0.0_f64); // Pointer x at drag start (px)
	let mut start_width = use_signal(|| 0.0_f64); // Sidebar width at drag start (px)

	rsx! {
		document::Link { rel: "stylesheet", href: asset!("./base_sidebar.css") }

		div {
			class: format!("base-sidebar-wrapper {name}-sidebar-wrapper"),
			"data-reach": MOBILE_REACH_LAYOUT(),
			SidebarProvider {
				class: if dragging() { "resizing" } else { "" }, // See reason for this in base_sidebar.scss
				style: format!(
					"--dx-sidebar-width: {}px; --dx-sidebar-width-mobile: 18rem; --dx-sidebar-width-icon: 3rem",
					width(),
				),
				open: Some(open()),
				on_open_change: move |v| open.set(v),
				Sidebar {
					class: format!("base-sidebar {name}-sidebar"),
					side,
					variant: SIDEBAR_VARIANT(),
					collapsible: SidebarCollapsible::Icon,
					{body}
					div {
						class: "resize-handle",
						onpointerdown: move |e| {
							dragging.set(true);
							moved.set(false);
							start_x.set(e.client_coordinates().x);
							// Anchor to the visible collapsed width (differs per variant)
							let icon_width = if SIDEBAR_VARIANT() == SidebarVariant::Floating {
								ICON_WIDTH_FLOATING
							} else {
								ICON_WIDTH_SIDEBAR
							};
							start_width.set(if open() { width() } else { icon_width });
						},
					}
				}
				SidebarInset { {children} }
			}
			if dragging() {
				div {
					class: "resize-overlay",
					onpointermove: move |e| {
						// A right-docked sidebar grows as the pointer moves left, so invert the travel
						let direction = if side == SidebarSide::Right { -1.0_f64 } else { 1.0 };
						let travel = e.client_coordinates().x - start_x();
						if travel.abs() > CLICK_THRESHOLD {
							moved.set(true);
						}
						let desired = direction.mul_add(travel, start_width()); // Drag past the minimum and keep going -> collapse.
						if open() {
							if desired < COLLAPSE_BELOW {
								open.set(false);
							} else {
								width.set(desired.clamp(MIN_WIDTH, MAX_WIDTH));
							}
						} else if desired > EXPAND_ABOVE {
							open.set(true);
							width.set(desired.clamp(MIN_WIDTH, MAX_WIDTH));
						}
					},
					onpointerup: move |_| {
						// A press that never travelled past the threshold is a click -< toggle
						if !moved() {
							open.set(!open());
						}
						dragging.set(false);
					},
					onpointercancel: move |_| dragging.set(false),
				}
			}
		}
	}
}
