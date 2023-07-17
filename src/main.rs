#![allow(non_snake_case)]
// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::prelude::*;

fn main() {
    // launch the web app
    dioxus_web::launch(App);
}

// create a component that renders a div with the text "Hello, world!"
fn App(cx: Scope) -> Element {
    let value = use_state(cx, || String::from("test"));
    cx.render(rsx! {
        div {
            style { include_str!("../src/style.css") },
            Header{}
            input {
                value: "{value}",
                oninput: move |evt| value.set(evt.value.clone()),
            }
            div {
                "{value.get()}"
            }
        }
    })
}

pub fn Header(cx: Scope) -> Element {
    cx.render(rsx!(div {
        h2 {"Header"}
    }))
}
