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
