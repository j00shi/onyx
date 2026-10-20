//! This crate contains all shared UI for the workspace.

use dioxus::prelude::*;

use crate::views::Frame;
use crate::views::Home;
use crate::views::NewTab;
use crate::views::Settings;

// Skip formatting and clippy for components/ since these are managed by Dioxus
#[rustfmt::skip]
#[allow(clippy::all, clippy::pedantic, clippy::nursery, clippy::restriction, clippy::cargo)]
pub mod components;
pub mod elements;
pub mod views;

#[derive(Debug, Clone, Routable, PartialEq, Eq)]
pub enum Route {
	#[route("/")]
	Home {},
	#[route("/newtab")]
	NewTab {},
	#[route("/settings")]
	Settings {},
}

#[component]
pub fn App() -> Element {
	rsx! {
		document::Link { rel: "icon", href: asset!("/assets/icons/favicon.ico") }
		document::Link { rel: "stylesheet", href: asset!("./main.css") }

		Frame { Router::<Route> {} }
	}
}
