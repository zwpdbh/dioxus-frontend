#![allow(non_snake_case)]
use dioxus::prelude::*;

enum FormDemoSubject {
    Option01,
    Option02,
}

struct FormDemoData {
    name: String,
    username: String,
    email: String,
    subject: FormDemoSubject,
    message: String,
    agree: bool,
}

#[component]
pub fn Form() -> Element {
    let user_input: Option<FormDemoData> = None;

    rsx!(
        match &user_input {
            Some(input) => div {
                  h1 { "No Data Available" },
            },
            None => {
                div {
                    h1 { "No Data Available" },
                }
            }
        }
    )
}

#[component]
pub fn ConditionalRender() -> Element {
    let is_logged_in: Option<i32> = None;
    rsx! {
           // We only render the welcome message if we are logged in
           // You can use if statements in the middle of a render block to conditionally render elements
           match is_logged_in {
               Some(_) => {
                   // if we are logged in, the button should say "Log Out"
                   "Log Out"
               }
               None => {
    // if we are not logged in, the button should say "Log In"
                   "Log In"

               }
           }
       }
}

#[component]
pub fn DynamicText() -> Element {
    let title = "title";
    let by = "author";
    let score = 0;
    let time = chrono::Utc::now();
    let comments = "comments";

    rsx! {"{title} by {by} ({score}) {time} {comments}"}
}
