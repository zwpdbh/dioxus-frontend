#![allow(non_snake_case)]
use super::MyCard;
use crate::Route;
use dioxus::prelude::*;
use dioxus_logger::tracing::info;

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

            Content {}
        }
    )
}

#[component]
fn Content() -> Element {
    rsx!(
        MyCard { SystemPrompt {} }
    )
}

#[component]
fn SystemPrompt() -> Element {
    rsx!(
        h3 { "system prompt list" }
        div { class: "select",
            select {
                option { "option 01" }
                option { "option 02" }
            }
        }
        div { class: "control",
            textarea {
                class: "textarea",
                r#"type"#: "text",
                readonly: true,
                placeholder: "select a prompt"
            }
        }
        div { class: "control",
            form {
                onsubmit: move |event| {
                    info!("Submitted! {event:?}");
                },
                input { name: "name" }
                button { class: "button is-primary", "Submit" }
            }
        }
    )
}
