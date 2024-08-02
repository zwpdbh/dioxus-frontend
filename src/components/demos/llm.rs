#![allow(non_snake_case)]
#[allow(unused)]
use super::MyCard;
use crate::Route;
use dioxus::prelude::*;
use dioxus_logger::tracing::info;
// use futures_util::io::Sink;

use dioxus_free_icons::icons::fa_brands_icons::FaGithub;
use dioxus_free_icons::Icon;
#[allow(unused)]
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
#[allow(unused)]
use serde_json::json;
#[allow(unused)]
use serde_json::Value;

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

#[derive(Serialize, Deserialize, Debug, PartialEq, Props, Clone)]
struct Configuration {
    url_prefix: String,
    secret: String,
}
#[allow(unused)]
impl Configuration {
    fn new() -> Self {
        Configuration {
            url_prefix: "".to_string(),
            secret: "".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Props, Clone)]
struct SystemPrompt {
    pub name: String,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Props, Clone)]
struct SystemPrompts {
    pub prompt_list: Vec<SystemPrompt>,
}

#[allow(unused)]
impl SystemPrompts {
    fn new() -> Self {
        SystemPrompts {
            prompt_list: vec![],
        }
    }
    fn is_empty(self) -> bool {
        self.prompt_list.is_empty()
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Props, Clone)]
struct MessageBody {
    pub role: String,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Props, Clone)]
struct ChatResponse {
    pub content: String,
    pub prompt_tokens: u64,
    pub completion_tokens: u64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Props, Clone)]
struct SystemPromptDropdown {
    pub dropdown_list: Vec<String>,
}

#[allow(unused)]
impl SystemPromptDropdown {
    fn new() -> Self {
        SystemPromptDropdown {
            dropdown_list: vec![],
        }
    }
}

#[component]
fn Content() -> Element {
    // let configuration = use_context_provider(|| Signal::new(Configuration::new()));
    // let system_prompt: Signal<Vec<SystemPrompt>> = use_context_provider(|| Signal::new(vec![]));
    // let setting_hide = use_context_provider(|| Signal::new("is-hidden"));

    // let system_prompt_name = use_context_provider(|| Signal::new(""));
    // let prompt = use_context_provider(|| Signal::new(""));
    // let loading = use_context_provider(|| Signal::new(""));
    // let error_msg = use_context_provider(|| Signal::new(""));
    // let response = use_context_provider(|| {
    //     Signal::new(ChatResponse {
    //         content: String::from(""),
    //         prompt_tokens: 0,
    //         completion_tokens: 0,
    //     })
    // });
    // let system_prompt_dropdown = use_context_provider(|| Signal::new(SystemPromptDropdown::new()));

    rsx! {
        // head {
        //     meta { name: "viewport", content: "width=device-width, initial-scale=1" } }
        div { class: "container is-max-desktop px-2",
            Nav {}
            Setting {}
        }
    }
}

#[component]
fn Setting() -> Element {
    use dioxus_free_icons::icons::bs_icons::BsGear;

    let mut setting_hide = use_context::<Signal<&str>>();
    let mut configuration = use_context::<Signal<Configuration>>();

    rsx!(
        button {
            class: "button is-white is-small",
            onclick: move |_| {
                if setting_hide().is_empty() {
                    setting_hide.set("is-hidden");
                } else {
                    setting_hide.set("");
                }
            },
            span { class: "icon has-text-light",
                Icon { width: 24, height: 24, fill: "#6e7781", icon: BsGear }
            }
            span { "setting" }
        }

        div { class: "columns {setting_hide}",
            div { class: "column is-6",
                input {
                    class: "input",
                    r#type: "text",
                    value: "{configuration().url_prefix}",
                    oninput: move |evt| {
                        let conf = Configuration {
                            url_prefix: evt.value().clone(),
                            secret: configuration().secret.clone(),
                        };
                        save_configuration(&conf);
                        configuration.set(conf);
                    }
                }
            }
            div { class: "column is-6",
                input {
                    class: "input",
                    placeholder: "OpenAi Secret",
                    r#type: "password",
                    value: "{configuration().secret}",
                    oninput: move |evt| {
                        let conf = Configuration {
                            url_prefix: configuration().url_prefix.clone(),
                            secret: evt.value().clone(),
                        };
                        save_configuration(&conf);
                        configuration.set(conf);
                    }
                }
            }
        }
    )
}

#[allow(unused)]
fn save_configuration(config: &Configuration) {
    todo!("implement save_configuration")
}

#[component]
fn Nav() -> Element {
    rsx!(
        nav { class: "level mt-2 mb-2",
            div { class: "level-left",
                div { class: "level-item",
                    p { class: "title is-size-4 has-text-centered", "OpenAI测试" }
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

#[component]
fn DropdownMenu() -> Element {
    let mut system_prompt_dropdown = use_context::<Signal<SystemPrompts>>();
    let system_prompts = use_context::<Signal<SystemPrompts>>();
    rsx!(
        div { class: "dropdown-menu", id: "dropdown-menu", role: "menu",
            div { class: "dropdown-content",
                a {
                    class: "dropdown-item py-0",
                    onclick: move |_| {
                        system_prompt_dropdown.set(SystemPrompts::new());
                    },
                    "关闭"
                }
                hr { class: "dropdown-divider" }
                if system_prompts().is_empty() {
                    div { class: "dropdown-item",
                        p { "没有system prompts" }
                    }
                }
            }
        }
    )
}

#[component]
fn DropdownItem() -> Element {
    // Need optimize this part
    let system_prompts = use_context::<Signal<Vec<SystemPrompt>>>();
    let mut system_prompt = use_context::<Signal<String>>();
    let mut system_prompt_name = use_context::<Signal<String>>();
    let mut system_prompt_dropdown = use_context::<Signal<String>>();

    let prompt_rendered = system_prompts().into_iter().map(|each_prompt| {
        let each_prompt_clone = each_prompt.clone();
        let system_prompts_read = system_prompts();

        rsx! {
            div { class: "column",
                span {
                    class: "tag is-primary is-light",
                    onclick: move |_| {
                        system_prompt_name.set(each_prompt_clone.name.clone());
                        system_prompt.set(each_prompt_clone.content.clone());
                        system_prompt_dropdown.set("".to_string());
                    },
                    "{each_prompt.name}"

                    button {
                        class: "delete is-small",
                        onclick: move |_| {
                            let system_prompts_read_filtered = system_prompts_read
                                .iter()
                                .filter(|each| each.name != each_prompt.name)
                                .collect::<Vec<&SystemPrompt>>();
                            save_system_prompts(system_prompts_read_filtered)
                        }
                    }
                }
            }
        }
    });

    rsx!(
        div { class: "dropdown-item",
            div { class: "columns is-multiline", {prompt_rendered} }
        }
    )
}

#[allow(unused)]
fn save_system_prompts(prompts: Vec<&SystemPrompt>) {
    todo!()
    // write_data(SYSTEM_PROMPTS_FILE_NAME, prompts);
}
