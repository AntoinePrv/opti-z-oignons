use dioxus::prelude::*;

const FAVICON: &str = concat!(
    "data:image/svg+xml,",
    "<svg xmlns=%22http://www.w3.org/2000/svg%22 viewBox=%2210 0 100 100%22>",
    "<text y=%22.90em%22 font-size=%2290%22>🚀</text>",
    "</svg>"
);
const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Home {}
    }
}

#[component]
fn Home() -> Element {
    rsx! { "Group Assignment" }
}
