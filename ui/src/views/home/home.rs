use crate::components::{Echo, Hero};
use crate::standard::Button;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
	rsx! {
		Hero {}
		Echo {}
		Button { "Example" }
	}
}
