use dioxus::prelude::*;

#[component]
pub fn Page(segments: Vec<String>) -> Element {
    let route = segments.join("/");
    rsx! {
        p { "Page not found /{route}" }
    }
}
