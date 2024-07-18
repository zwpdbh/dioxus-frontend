#![allow(non_snake_case)]
use super::GeneralCard;
use dioxus::prelude::*;

#[component]
pub fn DemoProp() -> Element {
    rsx!(
        ul {
            li {
                SimpleProp { score: 42 }
            }
            li { GeneralCard { 
                h1 { "Prop Option" }
                ul {
                    li { GeneralCard { 
                        PropOtions { title: "Some Title" }
                    } }

                    li { GeneralCard { 
                        PropOtions { title: "Some Title", subtitle: "Some Subtitle" }
                    } }
                }
            } }
            li { GeneralCard { 
                h1 { "Explicitly Required Option" }
                ul {
                    li { GeneralCard { 
                        ExplicitOption { title: "Some Title", subtitle: None }
                    } }

                    li { GeneralCard { 
                        ExplicitOption { title: "Some Title", subtitle: Some("Some SubTitle".to_string()) }
                    } }
                }
            } }

            li { GeneralCard { 
                h1 { "Default Props" }

                ul {
                    li { GeneralCard { 
                        DefaultComponent { number: 5 }
                    } }

                    li { GeneralCard { DefaultComponent {} } }
                }
            } }

            li { GeneralCard { 
                h1 { "Automatic Conversion with into" }
                IntoComponent { string: "some &str can be accepted instead of just String" }
            } }

            li { GeneralCard { 
                h1 { "ComponentChildrenV1" }
                ComponentChildrenV1 {
                    href: "https://www.youtube.com/watch?v=C-M2hs3sXGo",
                    body: rsx! {
                        "How to "
                        i { "not" }
                        " be seen"
                    }
                }
            } }

            li { GeneralCard { 
                h1 { "ComponentChildrenV2 pass element like children instead of pass rsx!" }
                p { "This is must be done from the `magic` children prop" }
                ComponentChildrenV2 { href: "https://www.youtube.com/watch?v=C-M2hs3sXGo",
                    "How to "
                    i { "not" }
                    " be seen"
                }
            } }
        }
    )
}

#[derive(PartialEq, Props, Clone)]
struct LikesProps {
    score: i32,
}

#[component]
fn SimpleProp(props: LikesProps) -> Element {
    rsx!(
        GeneralCard { 
            h1 { "Simple prop" }
            div {
                "This post has "
                b { "{props.score}" }
                " likes"
            }
        }
    )
}

#[derive(PartialEq, Clone, Props)]
struct OptionalProps {
    title: String,
    subtitle: Option<String>,
}

#[component]
fn PropOtions(props: OptionalProps) -> Element {
    rsx! {
        p {
            "{props.title}: "
            {props.subtitle.unwrap_or_else(|| "No subtitle provided".to_string())}
        }
    }
}

#[derive(PartialEq, Clone, Props)]
struct ExplicitOptionProps {
    title: String,
    #[props(!optional)]
    subtitle: Option<String>,
}

fn ExplicitOption(props: ExplicitOptionProps) -> Element {
    rsx! {
        p {
            "{props.title}: "
            {props.subtitle.unwrap_or_else(|| "No subtitle provided".to_string())}
        }
    }
}

#[derive(PartialEq, Props, Clone)]
struct DefaultProps {
    // default to 42 when not provided
    #[props(default = 42)]
    number: i64,
}

fn DefaultComponent(props: DefaultProps) -> Element {
    rsx! {
        p { "{props.number}" }
    }
}

#[derive(PartialEq, Props, Clone)]
struct IntoProps {
    #[props(into)]
    string: String,
}

fn IntoComponent(props: IntoProps) -> Element {
    rsx! {
        p { "{props.string}" }
    }
}

#[derive(PartialEq, Clone, Props)]
struct ClickableProps {
    href: String,
    body: Element,
}

fn ComponentChildrenV1(props: ClickableProps) -> Element {
    rsx! {
        a { href: "{props.href}", class: "fancy-button", {props.body} }
    }
}

#[derive(PartialEq, Clone, Props)]
struct ClickablePropsV2 {
    href: String,
    // The "magic" children prop
    children: Element,
}

fn ComponentChildrenV2(props: ClickablePropsV2) -> Element {
    rsx! {
        a { href: "{props.href}", class: "fancy-button", {props.children} }
    }
}
