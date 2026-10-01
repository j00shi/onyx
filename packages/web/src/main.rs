//! Web platform entry point
//! This is a minimal launcher that simply runs the shared UI app.

fn main() {
	dioxus::launch(ui::App);
}
