#![allow(non_snake_case)]
use super::MyCard;
use dioxus::prelude::*;
use dioxus_logger::tracing::info;

#[derive(Clone, Copy)]
struct IsLoggedIn(bool);

#[component]
pub fn DemoDynamicRendering() -> Element {
    use_context_provider(|| Signal::new(IsLoggedIn(false)));
    rsx!(
        div { class: "container",
            h1 { "Dynamic Rendering" }
            ul {
                li {
                    ConditionalRendering {
                        login: move |is_logged_in| {
                            info!("Do something for login, is_logged_in: {is_logged_in} ");
                        },
                        logout: move |is_logged_in| {
                            info!("Do something for logout, is_logged_in: {is_logged_in}");
                        }
                    }
                }
            }
        }
    )
}

#[derive(PartialEq, Clone, Props)]
struct ConditionalRenderingProps {
    login: EventHandler<bool>,
    logout: EventHandler<bool>,
}

fn ConditionalRendering(prop: ConditionalRenderingProps) -> Element {
    // let log_in = is_logged_in.set(true);
    // let log_out = is_logged_in.set(false);
    let mut is_logged_in = use_context::<Signal<IsLoggedIn>>();
    rsx!(
        MyCard {
            h2 { "Conditional Rendering" }
            // We only render the welcome message if we are logged in
            // You can use if statements in the middle of a render block to conditionally render elements
            if is_logged_in().0 {
                // Notice the body of this if statement is rsx code, not an expression
                "Welcome!"
            }
            button {
                class: "button",
                // depending on the value of `is_logged_in`, we will call a different event handler
                onclick: move |_| {
                    if is_logged_in().0 {
                        prop.login.call(is_logged_in().0);
                        is_logged_in.write().0 = false;
                    } else {
                        prop.login.call(is_logged_in().0);
                        is_logged_in.write().0 = true;
                    }
                },
                if is_logged_in().0 {
                    // if we are logged in, the button should say "Log Out"
                    "Log Out"
                } else {
                    // if we are not logged in, the button should say "Log In"
                    "Log In"
                }
            }
        }
    )
}
