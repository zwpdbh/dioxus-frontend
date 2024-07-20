#![allow(non_snake_case)]
use super::MyCard;
use dioxus::prelude::*;

#[component]
pub fn DemoHooks() -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
        h1 { "Demo hooks" }
        p { "Hooks allow us to create state in our components. " }
        p {
            "Hooks are Rust functions you call in a constant order in a component that add additional functionality to the component."
        }

        ul {
            li { MyCard {
                h2 { "use_signal hook" }
                p { "It is one of the simplest hooks." }

                p { "High-Five counter: {count}" }
                button { class: "button", onclick: move |_| count += 1, "Up " }
                button { class: "button", onclick: move |_| count -= 1, "Down " }
            } }
        }
    }
}
