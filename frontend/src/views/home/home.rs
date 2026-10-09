use crate::components::radio_group::{RadioGroup, RadioItem};
use crate::custom::{Echo, Hero};
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
	rsx! {
		Hero {}
		Echo {}
		RadioGroup {
			RadioItem { value: "option1".to_string(), index: 0usize, "Blue" }
			RadioItem { value: "option2".to_string(), index: 1usize, "Red" }
			RadioItem { value: "option3".to_string(), index: 2usize, disabled: true, "Green" }
		}
	}
}
