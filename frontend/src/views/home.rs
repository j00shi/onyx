use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
	rsx! {
		// document::Link { rel: "stylesheet", href: asset!("./home.css") }
		div { id: "home", style: "background-color: rgba(10,10,10,0.5)",
			h1 { "Home" }
			p { "Lorem ipsum dolor sit amet consectetur adipisicing elit. Vitae velit ipsam praesentium nostrum enim ad neque, optio autem dolorum perferendis nemo eveniet obcaecati suscipit nihil ratione! Dolorum quos expedita assumenda? Lorem ipsum dolor sit amet consectetur adipisicing elit. Vitae velit ipsam praesentium nostrum enim ad neque, optio autem dolorum perferendis nemo eveniet obcaecati suscipit nihil ratione! Dolorum quos expedita assumenda? Lorem ipsum dolor sit amet consectetur adipisicing elit. Vitae velit ipsam praesentium nostrum enim ad neque, optio autem dolorum perferendis nemo eveniet obcaecati suscipit nihil ratione! Dolorum quos expedita assumenda? Lorem ipsum dolor sit amet consectetur adipisicing elit. Vitae velit ipsam praesentium nostrum enim ad neque, optio autem dolorum perferendis nemo eveniet obcaecati suscipit nihil ratione! Dolorum quos expedita assumenda?" }
		}
	}
}
