use std::collections::HashMap;

use dioxus::prelude::*;
use strum::IntoEnumIterator;

const FAVICON: &str = concat!(
    "data:image/svg+xml,",
    "<svg xmlns=%22http://www.w3.org/2000/svg%22 viewBox=%2210 0 100 100%22>",
    "<text y=%22.90em%22 font-size=%2290%22>🚀</text>",
    "</svg>"
);
const MAIN_CSS: Asset = asset!("/assets/main.css");

#[derive(Clone, Copy, strum::Display, strum::EnumIter, strum::FromRepr)]
#[strum(serialize_all = "lowercase")]
enum RelationshipStrength {
    Hates,
    Dislikes,
    Likes,
    Loves,
}

impl RelationshipStrength {
    pub fn min() -> RelationshipStrength {
        return RelationshipStrength::iter().next().unwrap();
    }

    pub fn max() -> RelationshipStrength {
        return RelationshipStrength::iter().next_back().unwrap();
    }
}

type PersonId = String;
type Relationships = HashMap<PersonId, HashMap<PersonId, RelationshipStrength>>;

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
    let tables = use_signal(|| vec![6u32]);
    let relationships = use_signal(Relationships::new);
    rsx! {
        h1 { "Group Assignment" }
        Schema { tables, relationships }
        TableInput { tables }
        PersonInput { relationships }
        RelationshipInput { relationships }
    }
}

#[component]
fn Schema(relationships: Signal<Relationships>, tables: Signal<Vec<u32>>) -> Element {
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
            for person in relationships.read().keys() {
                li { key: person, "{person}" }
            }
        }
    }
}

#[component]
fn PersonInput(mut relationships: Signal<Relationships>) -> Element {
    const PERSON_NAME_ID: &'static str = "name";

    rsx! {
        // TODO: add "tribe" for auto conflicts
        // https://developer.mozilla.org/en-US/docs/Web/HTML/Element/datalist
        form {
            onsubmit: move |event| {
                let name_input = event
                    .data
                    .values()
                    .remove(PERSON_NAME_ID)
                    .map(|val| val.as_value());
                if let Some(name) = name_input {
                    relationships.write().insert(name, HashMap::new());
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

#[component]
fn TableInput(tables: Signal<Vec<u32>>) -> Element {
    const TABLE_SEATS_ID: &'static str = "n_seats";

    rsx! {
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
    }
}

#[component]
fn RelationshipInput(relationships: Signal<Relationships>) -> Element {
    const RELATIONSHIP_STRENGTH_ID: &'static str = "relationship_strength";
    const RELATIONSHIP_STRENGTH_DATALIST_ID: &'static str = "relationship_strength_datalist";
    const RELATIONSHIP_PERSON_1_ID: &'static str = "relationship_person_1";
    const RELATIONSHIP_PERSON_2_ID: &'static str = "relationship_person_2";
    const RELATIONSHIP_PERSON_DATALIST_ID: &'static str = "relationship_person_1_datalist";

    rsx! {
        for (p1 , neighbors) in relationships.read().iter() {
            for (p2 , strenght) in neighbors.iter() {
                p { "{p1} {strenght} {p2}" }
            }
        }
        form {
            onsubmit: move |event| {
                let mut data = event.data.values();
                let person1 = data.remove(RELATIONSHIP_PERSON_1_ID).map(|val| val.as_value());
                let person2 = data.remove(RELATIONSHIP_PERSON_2_ID).map(|val| val.as_value());
                let strength = data
                    .remove(RELATIONSHIP_STRENGTH_ID)
                    .map(|val| val.as_value())
                    .and_then(|val| val.parse::<usize>().ok())
                    .and_then(|val| RelationshipStrength::from_repr(val));
                if let Some(((person1, person2), strength)) = person1.zip(person2).zip(strength)
                {
                    relationships
                        .write()
                        .entry(person1.clone())
                        .or_insert_with(HashMap::new)
                        .entry(person2.clone())
                        .or_insert(strength);
                    relationships.write().entry(person2).or_insert_with(HashMap::new);
                }
            },

            label { r#for: RELATIONSHIP_PERSON_1_ID, "First Person" }
            input {
                id: RELATIONSHIP_PERSON_1_ID,
                name: RELATIONSHIP_PERSON_1_ID,
                r#type: "text",
                list: RELATIONSHIP_PERSON_DATALIST_ID,
                minlength: 1,
            }
            datalist { id: RELATIONSHIP_PERSON_DATALIST_ID,
                {relationships.read().keys().map(|p| rsx! {
                    option { value: "{p}" }
                })}
            }

            label { r#for: RELATIONSHIP_STRENGTH_ID, "Relationship" }
            input {
                id: RELATIONSHIP_STRENGTH_ID,
                name: RELATIONSHIP_STRENGTH_ID,
                r#type: "range",
                list: RELATIONSHIP_STRENGTH_DATALIST_ID,
                min: RelationshipStrength::min() as usize,
                max: RelationshipStrength::max() as usize,
                step: 1,
                value: RelationshipStrength::max() as usize,
            }
            // TODO: labels can be shown with CSS
            datalist { id: RELATIONSHIP_STRENGTH_DATALIST_ID,
                for strength in RelationshipStrength::iter() {
                    option { value: strength as usize, label: "{strength}" }
                }
            }

            label { r#for: RELATIONSHIP_PERSON_2_ID, "Person 2" }
            input {
                id: RELATIONSHIP_PERSON_2_ID,
                name: RELATIONSHIP_PERSON_2_ID,
                r#type: "text",
                list: RELATIONSHIP_PERSON_DATALIST_ID,
                minlength: 1,
            }

            button { r#type: "submit", "Add a relationship" }
        }
    }
}
