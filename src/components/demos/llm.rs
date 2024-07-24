#![allow(non_snake_case)]
use super::MyCard;
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn DemoLLM() -> Element {
    rsx!(
        h1 { "Demo LLM" }
        h3 { "Refereces:" }
        ul {
            li {
                Link {
                    to: NavigationTarget::<
                        Route,
                    >::External(
                        String::from("https://github.com/ealmloff/dioxus-streaming-llm/tree/main"),
                    ),
                    "ealmloff/dioxus-streaming-llm"
                }
            }
            li {
                Link {
                    to: NavigationTarget::<
                        Route,
                    >::External(String::from("https://github.com/fairjm/dioxus-openai-qa-gui")),
                    "fairjm/dioxus-openai-qa-gui"
                }
            }
        }
    )
}

#[component]
fn Content() -> Element {
    rsx!(MyCard {})
}
