#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_free_icons::icons::fa_brands_icons::FaGithub;
use dioxus_free_icons::Icon;

/// Example from: https://github.com/fairjm/dioxus-openai-qa-gui
#[component]
pub fn Form() -> Element {
    // let mut setting_hide = use_signal(|| true);

    rsx!(
        div { class: "container", Nav {} }
    )
}

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

// #[derive(PartialEq, Props, Clone)]
// struct SettingProps {
//     hide: bool,
// }

// #[component]
// fn Setting(props: SettingProps) -> Element {
//     rsx! {
//         button {

//             class: "button is-white is-small",
//             onclick: move |_| {
//                 if props.hide {
//                     hi
//                     setting_hide.set("is-hidden");
//                 } else {
//                     setting_hide.set("");
//                 }
//             },
//             span { class: "icon has-text-light",
//                 Icon { width: 24, height: 24, fill: "#6e7781", icon: BsGear }
//             }
//             span { "设置" }
//         }
//     }
// }
