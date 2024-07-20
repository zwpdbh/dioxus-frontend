#![allow(non_snake_case)]
use super::MyCard;
use dioxus::prelude::*;
use dioxus_logger::tracing::info;

#[component]
pub fn DemoEventHandler() -> Element {
    rsx!(
        div { class: "container",
            h1 { "DemoEventHandler" }
            ul {
                li { MyCard {
                    div {
                        p { "Simple event handler" }
                        button { onclick: move |event| info!("Clicked! Event: {event:?}"),
                            "click me!"
                        }
                    }
                } }
            }
        }
    )
}
