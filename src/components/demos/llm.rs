#![allow(non_snake_case)]
use super::MyCard;
use crate::Route;
use dioxus::prelude::*;
use dioxus_logger::tracing::info;
use futures_util::io::Sink;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::json;
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
    name: String,
    content: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Props, Clone)]
struct MessageBody {
    role: String,
    content: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Props, Clone)]
struct ChatResponse {
    content: String,
    prompt_tokens: u64,
    completion_tokens: u64,
}

#[component]
fn Content() -> Element {
    let configuration = use_context_provider(|| Signal::new(Configuration::new()));
    let system_prompt: Signal<Vec<SystemPrompt>> = use_context_provider(|| Signal::new(vec![]));
    let setting_hide = use_context_provider(|| Signal::new("is-hidden"));

    let system_prompt = use_context_provider(|| Signal::new(""));
    let system_prompt_name = use_context_provider(|| Signal::new(""));
    let prompt = use_context_provider(|| Signal::new(""));
    let loading = use_context_provider(|| Signal::new(""));
    let error_msg = use_context_provider(|| Signal::new(""));
    let response = use_context_provider(|| {
        Signal::new(ChatResponse {
            content: String::from(""),
            prompt_tokens: 0,
            completion_tokens: 0,
        })
    });
    let system_prompt_dropdown = use_context_provider(|| Signal::new(""));

    rsx! {

        head { meta { name: "viewport", content: "width=device-width, initial-scale=1" } }
        div { class: "container is-max-desktop px-2",
            nav { class: "level mt-2 mb-2",
                div { class: "level-left",
                    div { class: "level-item", p { class: "title is-size-4 has-text-centered", "OpenAI测试" } }
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

            button {

                class: "button is-white is-small",
                onclick: move |_| {
                    if setting_hide.is_empty() {
                        setting_hide.set("is-hidden");
                    } else {
                        setting_hide.set("");
                    }
                },
                span { class: "icon has-text-light",
                    Icon { width: 24, height: 24, fill: "#6e7781", icon: BsGear }
                }
                span { "设置" }
            }

            div { class: "columns {setting_hide}",
                div { class: "column is-6",
                    input {
                        class: "input",
                        r#type: "text",
                        value: "{configuration.url_prefix}",
                        oninput: move |evt| {
                            let conf = Configuration {
                                url_prefix: evt.value.clone(),
                                secret: configuration.current().secret.clone(),
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
                        value: "{configuration.secret}",
                        oninput: move |evt| {
                            let conf = Configuration {
                                url_prefix: configuration.current().url_prefix.clone(),
                                secret: evt.value.clone(),
                            };
                            save_configuration(&conf);
                            configuration.set(conf);
                        }
                    }
                }
            }

            div { class: "columns",
                div { class: "column pb-1",
                    nav { class: "level mb-1",
                        div { class: "level-left",
                            div { class: "level-item", p { class: "has-text-grey-light", "系统prompt" } }
                            div { class: "level-item",
                                div { class: "dropdown {system_prompt_dropdown}",
                                    div { class: "dropdown-trigger",
                                        button {
                                            class: "button is-small",
                                            "aria-haspopup": true,
                                            "aria-controls": "dropdown-menu",
                                            onclick: move |_| {
                                                if system_prompt_dropdown.current().is_empty() {
                                                    system_prompt_dropdown.set("is-active");
                                                } else {
                                                    system_prompt_dropdown.set("");
                                                }
                                            },
                                            span { "prompt列表" }
                                            span { class: "icon is-small",
                                                if system_prompt_dropdown.is_empty() {
                                    rsx!(
                                        Icon { width: 24, height: 24, fill: "#6e7781", icon: BsArrowDownShort }
                                    )
                                } else {
                                    rsx!(
                                        Icon { width: 24, height: 24, fill: "#6e7781", icon: BsArrowUpShort }
                                    )
                                }
                                            }
                                        }
                                    }

                                    div {

                                        class: "dropdown-menu",

                                        id: "dropdown-menu",

                                        role: "menu",
                                        div { class: "dropdown-content",
                                            a {
                                                class: "dropdown-item py-0",
                                                onclick: move |_| {
                                                    system_prompt_dropdown.set("");
                                                },
                                                "关闭"
                                            }
                                            hr { class: "dropdown-divider" }
                                            if system_prompts.is_empty() {
                                rsx! {
                                    div { class: "dropdown-item",
                                        p {
                                            "没有system prompts"
                                        }
                                    }
                                }
                            }
                                            div { class: "dropdown-item",
                                                div { class: "columns is-multiline",
                                                    system_prompts.iter().map(|e| {
                                    rsx!(
                                        div {class: "column",
                                            span { class: "tag is-primary is-light",
                                                onclick: move |_| {
                                                    system_prompt_name.set(e.name.clone());
                                                    system_prompt.set(e.content.clone());
                                                    system_prompt_dropdown.set("");
                                                },
                                                "{e.name}"

                                                button { class: "delete is-small",
                                                    onclick: move |_| {
                                                        system_prompts.with_mut(|v| {
                                                            if let Some(p) = v.iter().position(|value| value.name.eq(&e.name)) {
                                                                v.remove(p);
                                                            }
                                                        });
                                                        save_system_prompts(&*system_prompts.current().clone());
                                                    }
                                                }
                                            }
                                        })
                                    })
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                div { class: "column pb-1", p { class: "has-text-grey-light", "用户prompt" } }
            }

            div { class: "columns",
                div { class: "column pt-1",
                    p { class: "control",
                        textarea {
                            class: "textarea",
                            value: "{system_prompt}",
                            oninput: move |evt| {
                                system_prompt.set(evt.value.clone());
                            }
                        }
                    }
                    div { class: "level {save_button_attr(system_prompt)} mt-1",
                        div { class: "level-left",
                            div { class: "level-item",
                                input {
                                    class: "input",
                                    placeholder: "prompt名(重名将会覆盖已有的内容)",
                                    r#type: "text",
                                    value: "{system_prompt_name}",
                                    oninput: move |evt| { system_prompt_name.set(evt.value.clone()) }
                                }
                            }
                            div { class: "level-item",
                                button {
                                    class: "button is-primary",
                                    disabled: "{system_prompt_name.is_empty()}",
                                    onclick: move |_| {
                                        system_prompts
                                            .with_mut(|e| {
                                                if let Some(v)
                                                    = e.iter_mut().find(|p| p.name.eq(&*system_prompt_name.current()))
                                                {
                                                    v.content = system_prompt.current().clone().to_string();
                                                } else {
                                                    e.push(SystemPrompt {
                                                        name: system_prompt_name.current().clone().to_string(),
                                                        content: system_prompt.current().clone().to_string(),
                                                    });
                                                }
                                            });
                                        save_system_prompts(&*system_prompts.current().clone());
                                    },
                                    "保存prompt"
                                }
                            }
                        }
                    }
                }
                div { class: "column pt-1",
                    p { class: "control {loading}",
                        textarea {
                            class: "textarea",
                            value: "{prompt}",
                            oninput: move |evt| {
                                prompt.set(evt.value.clone());
                            }
                        }
                    }
                }
            }

            button {

                class: "button is-primary my-1 {loading}",
                disabled: "{request_button_disable(configuration, system_prompt, prompt)}",
                onclick: move |_| {
                    cx.spawn({
                        let loading = loading.clone();
                        loading.set("is-loading".to_string());
                        let configuration = configuration.clone();
                        let system_prompt = system_prompt.clone();
                        let prompt = prompt.clone();
                        let response = response.clone();
                        let error_msg = error_msg.clone();
                        async move {
                            let result = request(
                                    configuration.current().url_prefix.clone(),
                                    configuration.current().secret.clone(),
                                    system_prompt.current().to_string(),
                                    prompt.current().to_string(),
                                )
                                .await;
                            match result {
                                Ok(res) => {
                                    error_msg.set("".to_string());
                                    response.set(res);
                                }
                                Err(e) => error_msg.set(e.to_string()),
                            }
                            loading.set("".to_string());
                        }
                    })
                },
                "提交"
            }

            if request_button_disable(configuration, system_prompt, prompt) {
                rsx! {
                    div { class: "notification is-warning",
                        "请检查url前缀, openAI密钥是否为空, system prompt和用户prompt必须有一个不为空"
                    }
                }
            }

            if !error_msg.is_empty() {
                rsx! {
                    div { class: "notification is-warning",
                        button { class: "delete",
                        onclick: move |_| {
                            error_msg.set("".to_string());
                        }},
                        "{error_msg}"
                    }
                }
            }
            if !response.content.is_empty() {
                rsx! {
                    article { class: "message mt-2",
                        div { class: "message-body",
                            dangerous_inner_html: "{response.content}",
                        }
                    }
                }
            }
        }
    }
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
