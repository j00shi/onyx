//! This crate contains all shared UI for the workspace.

use dioxus::prelude::*;

pub mod components;
pub mod routes;
// Vendored Dioxus components, overwritten by updates. Excluded from our lints.
#[rustfmt::skip]
#[allow(clippy::all, clippy::pedantic, clippy::nursery, clippy::restriction)]
#[allow(unused_qualifications, unused_import_braces, unreachable_pub)]
pub mod standard;
pub mod views;

pub use routes::Route;

const FAVICON: Asset = asset!("/assets/icons/favicon.ico");

#[component]
pub fn App() -> Element {
	rsx! {
		// Global app resources
		document::Link { rel: "icon", href: FAVICON }
		document::Link { rel: "stylesheet", href: asset!("./main.css") }

		Router::<Route> {}
	}
}
