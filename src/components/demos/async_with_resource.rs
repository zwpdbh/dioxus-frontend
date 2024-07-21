#![allow(non_snake_case)]
use super::MyCard;
use dioxus::prelude::*;
use reqwest;
use serde::Deserialize;

#[component]
pub fn DemoAsyncResource() -> Element {
    rsx!(
        h1 { "Resource" }
        p { "use_resource lets you run an async closure, and provides you with its result." }
        ul {
            li { MyCard { DemoApiRequest {} } }
        }
    )
}

#[component]
pub fn DemoApiRequest() -> Element {
    let mut future = use_resource(|| async move {
        reqwest::get("https://dog.ceo/api/breeds/image/random")
            .await
            .unwrap()
            .json::<ApiResponse>()
            .await
    });

    match &*future.read_unchecked() {
        Some(Ok(response)) => rsx! {
            button { class: "button", onclick: move |_| future.restart(), "Click to fetch another doggo" }
            div {
                img {
                    max_width: "500px",
                    max_height: "500px",
                    src: "{response.image_url}"
                }
            }
        },
        Some(Err(e)) => rsx! {
            div {
                p { "Loading dogs failed" }
                p { "Error: {e}" }
            }
        },
        None => rsx! {
            div { "Loading dogs..." }
        },
    }
}

#[derive(Deserialize)]
struct ApiResponse {
    #[serde(rename = "message")]
    image_url: String,
}
