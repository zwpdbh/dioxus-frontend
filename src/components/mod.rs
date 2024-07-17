#![allow(non_snake_case)]
pub use blog::*;
pub use demos::counter::Counter;
pub use demos::form::Form;
pub use demos::*;
use dioxus::prelude::*;
pub use navbar::NavBar;

mod blog;
mod demos;
mod navbar;

#[component]
pub fn Home() -> Element {
    rsx!(
        h1 { "Welcome to the Dioxus!" }
    )
}

#[component]
pub fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
        h1 { "Page not found" }
        p { "We are terribly sorry, but the page you requested doesn't exist." }
        pre { color: "red", "log:\nattemped to navigate to: {route:?}" }
    }
}
