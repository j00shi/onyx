//! This crate contains all shared UI for the workspace.

use dioxus::prelude::*;

pub mod components;
pub mod routes;
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
