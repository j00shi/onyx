use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
	rsx! {
		// document::Link { rel: "stylesheet", href: asset!("./home.css") }
		div { id: "home",
			h1 { "Home" }
		}
	}
}
