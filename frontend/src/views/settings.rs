use dioxus::prelude::*;

#[component]
pub fn Settings() -> Element {
	rsx! {
		// document::Link { rel: "stylesheet", href: asset!("./settings.css") }
		div { id: "Settings" }
	}
}
