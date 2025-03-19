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

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct TableType {
    n_seats: usize,
}

type Tables = HashMap<TableType, usize>;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // TODO use context to manage state
    // use_context_provider(|| ProblemDefinition));
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Router::<Route> {}
    }
}

#[derive(Routable, PartialEq, Clone)]
enum Route {
    #[layout(NavBar)]
    #[route("/problem")]
    #[redirect("/", || Route::ProblemPage {})]
    ProblemPage {},
    #[route("/solution")]
    SolutionPage {},
    #[end_layout]
    #[route("/:..segments")]
    NotFound { segments: Vec<String> },
}

#[component]
pub fn NavBar() -> Element {
    rsx! {
        nav {
            ul {
                li {
                    Link { to: Route::ProblemPage {}, "Problem" }
                }
                li {
                    Link { to: Route::SolutionPage {}, "Solution" }
                }
            }
        }
        Outlet::<Route> {}
    }
}

#[component]
fn NotFound(segments: Vec<String>) -> Element {
    let route = segments.join("/");
    rsx! {
        p { "Page not found /{route}" }
    }
}

#[component]
fn ProblemPage() -> Element {
    let tables = use_signal(Tables::new);
    let relationships = use_signal(Relationships::new);
    rsx! {
        h1 { "Group Assignment" }
        Schema { tables, relationships }
        TableList { tables }
        TableInput { tables }
        PersonList { relationships }
        PersonInput { relationships }
        RelationshipList { relationships }
        RelationshipInput { relationships }
    }
}

#[component]
fn SolutionPage() -> Element {
    rsx! {
        p { "Hello Solution!" }
    }
}

fn fmt_table(seats: usize) -> String {
    let right_cnt = seats / 2;
    format!(
        "{}🟡{}",
        "🪑".repeat(seats - right_cnt),
        "🪑".repeat(right_cnt)
    )
}

#[component]
fn Schema(relationships: Signal<Relationships>, tables: Signal<Tables>) -> Element {
    // TODO add hover for names
    rsx! {
        ul {
            for (kind , count) in tables.read().iter() {
                // TODO Missing key
                for _ in 0..*count {
                    li { "{fmt_table(kind.n_seats)}" }
                }
            }
        }
        ul {
            for person in relationships.read().keys() {
                li { key: "{person}", "🐷" }
            }
        }
    }
}

#[component]
fn PersonList(relationships: Signal<Relationships>) -> Element {
    rsx! {
        p { "Persons:" }
        ul {
            for person in relationships.read().keys() {
                li { key: "{person}",
                    p { "{person}" }
                    button {
                        onclick: {
                            let person = person.to_owned();
                            move |_| {
                                for neighbors in relationships.write().values_mut() {
                                    neighbors.remove(&person);
                                }
                                relationships.write().remove(&person);
                            }
                        },
                        "❌"
                    }
                }
            }
        }
    }
}

#[component]
fn PersonInput(mut relationships: Signal<Relationships>) -> Element {
    const PERSON_NAME_ID: &'static str = "person_name";

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
                placeholder: "Add a person",
                r#type: "text",
                minlength: 1,
            }
            button { r#type: "submit", "✔️" }
        }
    }
}

#[component]
fn TableList(tables: Signal<Tables>) -> Element {
    rsx! {
        p { "Tables:" }
        ul {
            for (table , count) in tables.read().iter() {
                // TODO missing key
                li {
                    p { "{count} tables with {table.n_seats} seats" }
                    button {
                        onclick: {
                            let table: TableType = table.to_owned();
                            move |_| {
                                tables.write().remove(&table);
                            }
                        },
                        "❌"
                    }
                }
            }
        }
    }
}

#[component]
fn TableInput(tables: Signal<Tables>) -> Element {
    const TABLE_SEATS_ID: &'static str = "table_seats";
    const TABLE_COUNT_ID: &'static str = "table_count";

    rsx! {
        form {
            onsubmit: move |event| {
                let mut data = event.data.values();
                let n_seats_input = data
                    .remove(TABLE_SEATS_ID)
                    .map(|val| val.as_value())
                    .and_then(|val| val.parse::<usize>().ok());
                let count_input = data
                    .remove(TABLE_COUNT_ID)
                    .map(|val| val.as_value())
                    .and_then(|val| val.parse::<usize>().ok());
                if let Some((n_seats, count)) = n_seats_input.zip(count_input) {
                    *tables.write().entry(TableType { n_seats }).or_insert(0) += count;
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
            label { r#for: TABLE_COUNT_ID, "Number of tables" }
            input {
                id: TABLE_COUNT_ID,
                name: TABLE_COUNT_ID,
                r#type: "number",
                min: 0,
                step: 1,
                value: 1,
            }
            button { r#type: "submit", "Add tables" }
        }
    }
}

#[component]
fn RelationshipList(relationships: Signal<Relationships>) -> Element {
    rsx! {
        p { "Relationships:" }
        ul {
            for (p1 , neighbors) in relationships.read().iter() {
                for (p2 , strenght) in neighbors.iter() {
                    li {
                        // TODO missing key
                        p { "{p1} {strenght} {p2}" }
                        button {
                            onclick: {
                                {
                                    let p1 = p1.to_owned();
                                    let p2 = p2.to_owned();
                                    move |_| {
                                        if let Some(neighbors) = relationships.write().get_mut(&p1) {
                                            neighbors.remove(&p2);
                                        }
                                    }
                                }
                            },
                            "❌"
                        }
                    }
                }
            }
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

    // TODO relationship invariant should be more properly handled
    rsx! {
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
