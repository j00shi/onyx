use dioxus::{logger::tracing, prelude::*};
use dioxus_primitives::color_picker::Color;

use crate::{
	components::{color_picker::ColorPicker, *},
	elements::Hero,
};
use palette::{encoding, Hsv, IntoColor};

#[component]
pub fn Home() -> Element {
	let mut color =
		use_signal(|| -> Hsv<encoding::Srgb, f64> { Color::new(155, 128, 255).into_format::<f64>().into_color() });
	rsx! {
		Hero {}
		ColorPicker {
			label: "Pick a color",
			color: color(),
			on_color_change: move |c| {
				tracing::info!("Color changed: {:?}", c);
				color.set(c);
			},
		}
	}
}
