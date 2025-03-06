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
    rsx! {
        h1 { "Group Assignment" }
        Schema {}
    }
}

#[component]
fn Schema() -> Element {
    let mut tables = use_signal(|| vec![6u32]);
    const TABLE_SEATS_ID: &'static str = "n_seats";

    let mut persons = use_signal(Vec::<String>::new);
    const PERSON_NAME_ID: &'static str = "name";

    rsx! {
        p { "Tables:" }
        ul {
            for (i , seats) in tables.iter().enumerate() {
                // TODO Missing key
                li { "Table {i} ({seats} seats)" }
            }
        }

        p { "Persons:" }
        ul {
            for person in persons.iter() {
                // TODO Missing key
                li { "{person}" }
            }
        }

        form {
            onsubmit: move |event| {
                let n_seats_input = event
                    .data
                    .values()
                    .remove(TABLE_SEATS_ID)
                    .map(|val| val.as_value())
                    .and_then(|val| val.parse::<u32>().ok());
                if let Some(n_seats) = n_seats_input {
                    tables.push(n_seats)
                }
            },
            label { r#for: TABLE_SEATS_ID, "Number of seats" }
            input {
                id: TABLE_SEATS_ID,
                name: TABLE_SEATS_ID,
                r#type: "number",
                min: 0,
                step: 1,
                value: 6,
            }
            button { r#type: "submit", "Add a table" }
        }

        form {
            onsubmit: move |event| {
                let name_input = event
                    .data
                    .values()
                    .remove(PERSON_NAME_ID)
                    .map(|val| val.as_value());
                if let Some(name) = name_input {
                    persons.push(name)
                }
            },
            label { r#for: PERSON_NAME_ID, "Name" }
            input {
                id: PERSON_NAME_ID,
                name: PERSON_NAME_ID,
                r#type: "text",
                minlength: 1,
            }
            button { r#type: "submit", "Add a person" }
        }
    }
}
