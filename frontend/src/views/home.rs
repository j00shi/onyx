use dioxus::prelude::*;

use crate::elements::Hero;

#[component]
pub fn Home() -> Element {
	rsx! {
		// document::Link { rel: "stylesheet", href: asset!("./home.css") }
		div { id: "home", Hero {} }
	}
}
