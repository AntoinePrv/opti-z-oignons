use dioxus::prelude::*;

#[component]
pub fn Card(header: Element, body: Element) -> Element {
    rsx! {
        div { class: "card bg-base-100 shadow-sm",
            div { class: "card-body",
                div { class: "card-title", {header} }
                div { {body} }
            }
        }
    }
}
