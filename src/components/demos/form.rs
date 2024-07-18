#![allow(non_snake_case)]
use dioxus::prelude::*;

#[allow(unused)]
#[derive(Debug)]
enum FormDemoSubject {
    Option01,
    Option02,
}

#[derive(Debug)]
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

    match user_input {
        Some(input) => {
            rsx!(
                div {
                    h1 { "Form Data" }
                    p { "Name: {input.name}" }
                    p { "Username: {input.username}" }
                    p { "Email: {input.email}" }
                    p { "Subject: {input.subject:?}" }
                    p { "Message: {input.message}" }
                    p { "Agree: {input.agree}" }
                }
            )
        }
        None => {
            rsx!(
                div {
                    h1 { "No Data Available" }
                }
            )
        }
    }
}
