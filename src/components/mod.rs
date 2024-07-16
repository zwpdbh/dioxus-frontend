#![allow(non_snake_case)]
use crate::routes::Route;
use dioxus::prelude::*;

#[component]
pub fn NavBar() -> Element {
    rsx!(
        nav {
            "aria-label": "main navigation",
            role: "navigation",
            class: "navbar",

            div { class: "navbar-menu", id: "navbarBasicExample",
                div { class: "navbar-start",
                    Link { class: "navbar-item", to: Route::Home {}, "Home" }
                    Link { class: "navbar-item", to: Route::BlogList {},
                        {},
                        "Blog List"
                    }
                    Link { class: "navbar-item", to: Route::DemoMenuDefault {},
                        {},
                        "Demos"
                    }
                    div { class: "navbar-item has-dropdown is-hoverable",
                        a { class: "navbar-link", "More" }
                        div { class: "navbar-dropdown",
                            a { class: "navbar-item", "About" }
                            a { class: "navbar-item is-selected", "Jobs" }
                            a { class: "navbar-item", "Contact" }
                            hr { class: "navbar-divider" }
                            a { class: "navbar-item", "Report an issue" }
                        }
                    }
                }
                div { class: "navbar-end",
                    div { class: "navbar-item",
                        div { class: "buttons",
                            a { class: "button is-primary",
                                strong { "Sign up" }
                            }
                            a { class: "button is-light", "Log in" }
                        }
                    }
                }
            }
        }

        // The Outlet component will render child routes (In this case just the Home component) inside the Outlet component
        Outlet::<Route> {}
    )
}

#[component]
pub fn Home() -> Element {
    rsx!(
        h1 { "Welcome to the Dioxus!" }
    )
}

#[component]
pub fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
        h1 { "Page not found" }
        p { "We are terribly sorry, but the page you requested doesn't exist." }
        pre { color: "red", "log:\nattemped to navigate to: {route:?}" }
    }
}

/// Place holder for Blog section
#[component]
pub fn Blog() -> Element {
    rsx! {
        h1 { "Blog Detail" }
        Outlet::<Route> {}
    }
}

#[component]
pub fn BlogList() -> Element {
    rsx! {
        h2 { "Choose a post" }
        ul {
            li {
                Link {
                    to: Route::BlogPost {
                        name: "Blog post 1".into(),
                    },
                    "Read the first blog post"
                }
            }
            li {
                Link {
                    to: Route::BlogPost {
                        name: "Blog post 2".into(),
                    },
                    "Read the second blog post"
                }
            }
        }
    }
}

// The name prop comes from the /:name route segment
#[component]
pub fn BlogPost(name: String) -> Element {
    rsx! {
        h2 { "Blog Post: {name}" }
    }
}

#[component]
pub fn Counter() -> Element {
    let mut count = use_signal(|| 0);
    rsx! {
        div {
            h1 { "High-Five counter: {count}" }
            button { onclick: move |_| count += 1, "Up high!" }
            button { onclick: move |_| count -= 1, "Down low!" }
        }
    }
}

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
    rsx!()
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
            p { class: "menu-label", "Administration" }
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
            p { class: "menu-label", "Transactions" }
            ul { class: "menu-list",
                li {
                    a { "Payments" }
                }
                li {
                    a { "Transfers" }
                }
                li {
                    a { "Balance" }
                }
            }
        }
    )
}
