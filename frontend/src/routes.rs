use dioxus::prelude::*;

use crate::custom::Navbar;
use crate::views::{Blog, Home, Vault};

#[derive(Debug, Clone, Routable, PartialEq, Eq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(AppNavbar)]
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
    // The vault is a full-screen app shell, so it lives outside the navbar layout.
    #[end_layout]
    #[route("/vault")]
    Vault {},
}

/// A shared navbar component that works across all platforms
#[component]
fn AppNavbar() -> Element {
	rsx! {
		Navbar {
			Link { to: Route::Home {}, "Home" }
			Link { to: Route::Blog { id: 1 }, "Blog" }
			Link { to: Route::Vault {}, "Vault" }
		}

		Outlet::<Route> {}
	}
}
