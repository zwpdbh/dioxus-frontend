#![allow(non_snake_case)]
use super::MyCard;
use dioxus::prelude::*;
use dioxus_free_icons::icons::fa_brands_icons::FaGithub;
use dioxus_free_icons::Icon;

/// Example from: https://github.com/fairjm/dioxus-openai-qa-gui
#[component]
pub fn UserInput() -> Element {
    // let mut setting_hide = use_signal(|| true);

    rsx!(
        div { class: "container",
            ul {
                li { ControlledInput {} }
            }
        }
    )
}

#[component]
fn ControlledInput() -> Element {
    let mut name = use_signal(|| "bob".to_string());

    rsx!(
        MyCard {
            h1 { "Controlled Inputs" }
            p { "With controlled inputs, you are directly in charge of the state of the input. " }
            p { "This gives you a lot of flexibility, and makes it easy to keep things in sync. " }

            input {
                // we tell the component what to render
                value: "{name}",
                // and what to do when the value changes
                oninput: move |event| name.set(event.value())
            }

            p { "Your input is: {name}" }
        }
    )
}

#[allow(unused)]
#[component]
fn Nav() -> Element {
    rsx! {
        nav { class: "level mt-2 mb-2",
            div { class: "level-left",
                div { class: "level-item",
                    p { class: "title is-size-4 has-text-centered", "LLA test" }
                }
                div { class: "level-item",
                    a {
                        class: "button is-small",
                        target: "_blank",
                        href: "https://github.com/fairjm/dioxus-openai-qa-gui",
                        span { class: "icon is-small",
                            Icon { width: 24, height: 24, fill: "#6e7781", icon: FaGithub }
                        }
                        span { "GitHub" }
                    }
                }
            }
        }
    }
}
