#![allow(non_snake_case)]

use dioxus::prelude::*;
use tracing::Level;

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    // All routes under the NavBar layout will be rendered inside of the NavBar Outlet
    #[layout(NavBar)]
    #[route("/")]
    Home {},
    #[nest("/blog")]
    #[layout(Blog)]
    #[route("/")]
    BlogList {},
    #[route("/post/:name")]
    BlogPost { name: String },
    #[end_layout]
    #[end_nest]
    #[route("/counter")]
    Counter {},
    #[end_layout]
    #[nest("/myblog")]
    #[redirect("/", || Route::BlogList {})]
    #[redirect("/:name", |name: String| Route::BlogPost { name })]
    #[end_nest]
    // #[route("/acstor")]
    // Acstor {},
    // PageNotFound is a catch all route that will match any route and placing the matched segments in the route field
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}

#[component]
fn Home() -> Element {
    rsx!(
        h1 { "Welcome to the Dioxus!" }
    )
}

#[component]
fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
        h1 { "Page not found" }
        p { "We are terribly sorry, but the page you requested doesn't exist." }
        pre { color: "red", "log:\nattemped to navigate to: {route:?}" }
    }
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Blog() -> Element {
    rsx! {
        h1 { "Blog Section" }
        Outlet::<Route> {}
    }
}

#[component]
fn BlogList() -> Element {
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
fn BlogPost(name: String) -> Element {
    rsx! {
        h2 { "Blog Post: {name}" }
    }
}

#[component]
fn Counter() -> Element {
    let mut count = use_signal(|| 0);
    rsx! {
        div {
            h1 { "High-Five counter: {count}" }
            button { onclick: move |_| count += 1, "Up high!" }
            button { onclick: move |_| count -= 1, "Down low!" }
        }
    }
}

#[component]
fn Menu() -> Element {
    rsx!(
        aside { class: "menu",
            p { class: "menu-label", "General" }
            ul { class: "menu-list",
                li {
                    a { "Dashboard" }
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
                    a { class: "is-active", "Manage Your Team" }
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

#[component]
fn NavBar() -> Element {
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
                    a { class: "navbar-item", "Demos" }
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
