//! Desktop platform entry point
//! This is a minimal launcher that simply runs the shared UI app.

#[cfg(feature = "desktop")]
pub mod platform;

fn main() {
	#[cfg(feature = "desktop")]
	platform::launch_desktop();

	#[cfg(not(feature = "desktop"))]
	dioxus::launch(frontend::App);
}
