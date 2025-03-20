use std::collections::HashMap;

use dioxus::prelude::*;
use strum::IntoEnumIterator;

#[derive(Clone, Copy, strum::Display, strum::EnumIter, strum::FromRepr)]
#[strum(serialize_all = "lowercase")]
pub enum RelationStrength {
    Hates,
    Dislikes,
    Likes,
    Loves,
}

impl RelationStrength {
    pub fn min() -> Self {
        return Self::iter().next().unwrap();
    }

    pub fn max() -> Self {
        return Self::iter().next_back().unwrap();
    }
}

type PersonName = String;
type PersonNameRef = str;
type PersonId = PersonName;

pub struct Tribe {
    directed_relations: HashMap<PersonId, HashMap<PersonId, RelationStrength>>,
}

impl Tribe {
    pub fn new() -> Self {
        Self {
            directed_relations: HashMap::new(),
        }
    }

    pub fn add_person(&mut self, name: PersonName) {
        self.directed_relations.insert(name, HashMap::new());
    }

    pub fn remove_person(&mut self, name: &PersonNameRef) {
        for neighbors in self.directed_relations.values_mut() {
            neighbors.remove(name);
        }
        self.directed_relations.remove(name);
    }

    pub fn persons(&self) -> impl Iterator<Item = &PersonName> {
        self.directed_relations.keys()
    }

    pub fn add_relation(
        &mut self,
        name1: impl Into<PersonName>,
        name2: impl Into<PersonName>,
        strength: RelationStrength,
    ) {
        let name2 = name2.into();
        self.directed_relations
            .entry(name2.clone())
            .or_insert_with(HashMap::new);
        self.directed_relations
            .entry(name1.into())
            .or_insert_with(HashMap::new)
            .entry(name2)
            .or_insert(strength);
    }

    pub fn remove_relation(&mut self, name1: &PersonNameRef, name2: &PersonNameRef) {
        if let Some(neighbors) = self.directed_relations.get_mut(name1) {
            neighbors.remove(name2);
        }
    }

    pub fn relations(&self) -> impl Iterator<Item = (&PersonName, &PersonName, RelationStrength)> {
        self.directed_relations
            .iter()
            .map(|(p1, neighbors)| {
                neighbors
                    .iter()
                    .map(move |(p2, strength)| (p1, p2, *strength))
            })
            .flatten()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct TableType {
    n_seats: usize,
}

type Tables = HashMap<TableType, usize>;

#[component]
pub fn Page() -> Element {
    let tables = use_signal(Tables::new);
    let relations = use_signal(Tribe::new);
    rsx! {
        h1 { "Group Assignment" }
        Schema { tables, relations }
        TableList { tables }
        TableInput { tables }
        PersonList { relations }
        PersonInput { relations }
        RelationList { relations }
        RelationInput { relations }
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
fn Schema(relations: Signal<Tribe>, tables: Signal<Tables>) -> Element {
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
            for person in relations.read().persons() {
                li { key: "{person}", "🐷" }
            }
        }
    }
}

#[component]
fn PersonList(relations: Signal<Tribe>) -> Element {
    rsx! {
        p { "Persons:" }
        ul {
            for person in relations.read().persons() {
                li { key: "{person}",
                    p { "{person}" }
                    button {
                        onclick: {
                            let person = person.to_owned();
                            move |_| {
                                relations.write().remove_person(&person);
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
fn PersonInput(mut relations: Signal<Tribe>) -> Element {
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
                    relations.write().add_person(name);
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
fn RelationList(relations: Signal<Tribe>) -> Element {
    rsx! {
        p { "Relations:" }
        ul {
            for (p1 , p2 , strenght) in relations.read().relations() {
                li {
                    // TODO missing key
                    p { "{p1} {strenght} {p2}" }
                    button {
                        onclick: {
                            {
                                let p1 = p1.to_owned();
                                let p2 = p2.to_owned();
                                move |_| {
                                    relations.write().remove_relation(&p1, &p2);
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

#[component]
fn RelationInput(relations: Signal<Tribe>) -> Element {
    const RELATION_STRENGTH_ID: &'static str = "relation_strength";
    const RELATION_STRENGTH_DATALIST_ID: &'static str = "relation_strength_datalist";
    const RELATION_PERSON_1_ID: &'static str = "relation_person_1";
    const RELATION_PERSON_2_ID: &'static str = "relation_person_2";
    const RELATION_PERSON_DATALIST_ID: &'static str = "relation_person_1_datalist";

    // TODO relation invariant should be more properly handled
    rsx! {
        form {
            onsubmit: move |event| {
                let mut data = event.data.values();
                let person1 = data.remove(RELATION_PERSON_1_ID).map(|val| val.as_value());
                let person2 = data.remove(RELATION_PERSON_2_ID).map(|val| val.as_value());
                let strength = data
                    .remove(RELATION_STRENGTH_ID)
                    .map(|val| val.as_value())
                    .and_then(|val| val.parse::<usize>().ok())
                    .and_then(|val| RelationStrength::from_repr(val));
                if let Some(((person1, person2), strength)) = person1.zip(person2).zip(strength)
                {
                    relations.write().add_relation(person1, person2, strength);
                }
            },

            label { r#for: RELATION_PERSON_1_ID, "First Person" }
            input {
                id: RELATION_PERSON_1_ID,
                name: RELATION_PERSON_1_ID,
                r#type: "text",
                list: RELATION_PERSON_DATALIST_ID,
                minlength: 1,
            }
            datalist { id: RELATION_PERSON_DATALIST_ID,
                {relations.read().persons().map(|p| rsx! {
                    option { value: "{p}" }
                })}
            }

            label { r#for: RELATION_STRENGTH_ID, "Relation" }
            input {
                id: RELATION_STRENGTH_ID,
                name: RELATION_STRENGTH_ID,
                r#type: "range",
                list: RELATION_STRENGTH_DATALIST_ID,
                min: RelationStrength::min() as usize,
                max: RelationStrength::max() as usize,
                step: 1,
                value: RelationStrength::max() as usize,
            }
            // TODO: labels can be shown with CSS
            datalist { id: RELATION_STRENGTH_DATALIST_ID,
                for strength in RelationStrength::iter() {
                    option { value: strength as usize, label: "{strength}" }
                }
            }

            label { r#for: RELATION_PERSON_2_ID, "Person 2" }
            input {
                id: RELATION_PERSON_2_ID,
                name: RELATION_PERSON_2_ID,
                r#type: "text",
                list: RELATION_PERSON_DATALIST_ID,
                minlength: 1,
            }

            button { r#type: "submit", "Add a relation" }
        }
    }
}
