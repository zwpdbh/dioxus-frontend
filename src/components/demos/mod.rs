#![allow(non_snake_case)]
use crate::routes::Route;
use dioxus::prelude::*;

pub mod counter;
pub mod form;
pub mod prop;
pub mod rsx_basic;

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

/// This is the sidebar menu to show different demos for demo section
#[component]
fn DemoMenu() -> Element {
    rsx!(
        aside { class: "menu",
            p { class: "menu-label", "General" }
            ul { class: "menu-list",
                li {
                    Link { to: Route::RsxBasic {}, "RsxBasic" }
                }
                li {
                    Link { to: Route::Counter {}, "Counter" }
                }
                li {
                    Link { to: Route::DemoProp {}, "Prop" }
                }
                li {
                    Link { to: Route::Form {}, "Form" }
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

#[component]
fn GeneralCard(children: Element) -> Element {
    rsx!(
        div { class: "card",
            div { class: "card-content",
                div { class: "content", { children } }
            }
        }
    )
}
