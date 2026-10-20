use dioxus::prelude::*;

#[component]
pub fn NewTab() -> Element {
	rsx! {
		// document::Link { rel: "stylesheet", href: asset!("./newtab.css") }
		div { id: "NewTab" }
	}
}
