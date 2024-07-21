#![allow(non_snake_case)]
use super::MyCard;
use dioxus::prelude::*;
use futures_util::StreamExt;
use tokio::time::Duration;

#[component]
pub fn DemoCoroutines() -> Element {
    rsx!(
        h1 { "Coroutines" }
        p {
            "It allows you to write asynchronous code that can yield values over time, suitable for tasks like WebSocket polling, background timers, and other periodic actions."
        }
        ul {
            li { SimpleCoroutine {} }
        }
    )
}

struct Profile {
    name: String,
    age: i32,
}

enum ProfileUpdate {
    SetUsername(String),
    SetAge(i32),
}

async fn connect_to_server() -> Server {
    tokio::time::sleep(Duration::from_secs(1)).await;
    Server {}
}

struct Server {}

impl Server {
    pub async fn update_username(&self, _name: String) {
        tokio::time::sleep(Duration::from_secs(1)).await;
    }

    pub async fn update_age(&self, _age: i32) {
        tokio::time::sleep(Duration::from_secs(1)).await;
    }

    pub async fn get_profile(&self) -> Option<Profile> {
        tokio::time::sleep(Duration::from_secs(1)).await;
        Some(Profile {
            name: "test".to_string(),
            age: 1,
        })
    }
}

#[component]
fn SimpleCoroutine() -> Element {
    let profile = use_coroutine(|mut rx: UnboundedReceiver<ProfileUpdate>| async move {
        let server = connect_to_server().await;

        while let Some(msg) = rx.next().await {
            match msg {
                ProfileUpdate::SetUsername(name) => server.update_username(name).await,
                ProfileUpdate::SetAge(age) => server.update_age(age).await,
            }
        }
    });

    let _: Coroutine<()> = use_coroutine(|rx| async move { todo!("xx") });

    rsx!(
        MyCard {
            h2 { "use_coroutine to send value" }

            button {
                class: "button",
                onclick: move |_| profile.send(ProfileUpdate::SetUsername("Bob".to_string())),
                "Update username"
            }

            button {
                class: "button",
                onclick: move |_| profile.send(ProfileUpdate::SetAge(12)),
                "Update age"
            }
        }
    )
}
