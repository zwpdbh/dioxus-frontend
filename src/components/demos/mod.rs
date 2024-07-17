#![allow(non_snake_case)]
use crate::routes::Route;
use dioxus::prelude::*;

pub mod counter;

/// Place holder for Demo section
#[component]
pub fn Demo() -> Element {
    rsx!(
        div { class: "columns",
            div { class: "column is-one-fifth", DemoMenu {} }
            div { class: "column", Outlet::<Route> {} }
        }
    )
}

#[component]
pub fn DemoMenuDefault() -> Element {
    rsx!(  )
}

#[component]
fn DemoMenu() -> Element {
    rsx!(
        aside { class: "menu",
            p { class: "menu-label", "General" }
            ul { class: "menu-list",
                li {
                    Link { to: Route::Counter {}, "Counter" }
                }
                li {
                    a { "Customers" }
                }
            }
            p { class: "menu-label", "ACStor CRUD" }
            ul { class: "menu-list",
                li {
                    a { "Team Settings" }
                }
                li {
                    a { "Manage Your Team" }
                    ul {
                        li {
                            a { "Members" }
                        }
                        li {
                            a { "Plugins" }
                        }
                        li {
                            a { "Add a member" }
                        }
                    }
                }
                li {
                    a { "Invitations" }
                }
                li {
                    a { "Cloud Storage Environment Settings" }
                }
                li {
                    a { "Authentication" }
                }
            }
        }
    )
}
