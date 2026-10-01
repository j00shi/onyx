use dioxus::prelude::*;

use crate::components::Navbar;
use crate::views::{Blog, Home};

#[derive(Debug, Clone, Routable, PartialEq, Eq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(AppNavbar)]
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}

/// A shared navbar component that works across all platforms
#[component]
fn AppNavbar() -> Element {
	rsx! {
		Navbar {
			Link { to: Route::Home {}, "Home" }
			Link { to: Route::Blog { id: 1 }, "Blog" }
		}

		Outlet::<Route> {}
	}
}
