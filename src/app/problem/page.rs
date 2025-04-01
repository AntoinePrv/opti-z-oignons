use dioxus::prelude::*;

use crate::logic::model::{RelationStrength, TableType, Tables, Tribe};

#[component]
pub fn Page() -> Element {
    let pb: crate::ProblemSignal = use_context();

    rsx! {
        h1 { "Group Assignment" }
        Schema { tables: pb.tables, tribe: pb.tribe }
        ShowMeHow { tables: pb.tables, tribe: pb.tribe }
        TableList { tables: pb.tables }
        TableInput { tables: pb.tables }
        PersonList { tribe: pb.tribe }
        PersonInput { tribe: pb.tribe }
        RelationList { tribe: pb.tribe }
        RelationInput { tribe: pb.tribe }
    }
}

#[component]
fn ShowMeHow(tribe: Signal<Tribe>, tables: Signal<Tables>) -> Element {
    rsx! {
        button {
            onclick: {
                move |_| {
                    let (ex_tribe, ex_tables) = crate::logic::examples::harry_potter();
                    tribe.set(ex_tribe);
                    tables.set(ex_tables);
                }
            },
            "Show me how!"
        }
    }
}

fn fmt_table(seats: u32) -> String {
    let right_cnt = seats / 2;
    format!(
        "{}🟡{}",
        "🪑".repeat((seats - right_cnt) as usize),
        "🪑".repeat(right_cnt as usize)
    )
}

#[component]
fn Schema(tribe: Signal<Tribe>, tables: Signal<Tables>) -> Element {
    // TODO add hover for names
    rsx! {
        ul {
            for (_name , kind) in tables.read().iter() {
                li { key: _name, "{fmt_table(kind.n_seats)}" }
            }
        }
        ul {
            for person in tribe.read().persons() {
                li { key: "{person}", "🐷" }
            }
        }
    }
}

#[component]
fn PersonList(tribe: Signal<Tribe>) -> Element {
    rsx! {
        p { "Persons:" }
        ul {
            for person in tribe.read().persons() {
                li { key: "{person}",
                    p { "{person}" }
                    button {
                        onclick: {
                            let person = person.to_owned();
                            move |_| {
                                tribe.write().remove_person(&person);
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
fn PersonInput(tribe: Signal<Tribe>) -> Element {
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
                    tribe.write().add_person(name);
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
            for (name , table) in tables.read().iter() {
                li { key: name,
                    p { "Table {name} with {table.n_seats} seats" }
                    button {
                        onclick: {
                            let name = name.to_owned();
                            move |_| {
                                tables.write().remove(&name);
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

    let mut name_generator: Signal<crate::name_generator::NameGenerator> = use_context();

    rsx! {
        form {
            onsubmit: move |event| {
                let mut data = event.data.values();
                let n_seats_input = data
                    .remove(TABLE_SEATS_ID)
                    .map(|val| val.as_value())
                    .and_then(|val| val.parse::<u32>().ok());
                let count_input = data
                    .remove(TABLE_COUNT_ID)
                    .map(|val| val.as_value())
                    .and_then(|val| val.parse::<usize>().ok());
                if let Some((n_seats, count)) = n_seats_input.zip(count_input) {
                    for _ in 0..count {
                        let name = name_generator.write().next().unwrap();
                        tables.write().insert(name, TableType { n_seats });
                    }
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
fn RelationList(tribe: Signal<Tribe>) -> Element {
    rsx! {
        p { "Relations:" }
        ul {
            for (p1 , p2 , strenght) in tribe.read().relations() {
                li {
                    // TODO missing key
                    p { "{p1} {strenght} {p2}" }
                    button {
                        onclick: {
                            {
                                let p1 = p1.to_owned();
                                let p2 = p2.to_owned();
                                move |_| {
                                    tribe.write().remove_relation(&p1, &p2);
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
fn RelationInput(mut tribe: Signal<Tribe>) -> Element {
    const RELATION_STRENGTH_ID: &'static str = "relation_strength";
    const RELATION_STRENGTH_DATALIST_ID: &'static str = "relation_strength_datalist";
    const RELATION_PERSON_1_ID: &'static str = "relation_person_1";
    const RELATION_PERSON_2_ID: &'static str = "relation_person_2";
    const RELATION_PERSON_DATALIST_ID: &'static str = "relation_person_datalist";

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
                    tribe.write().add_relation(person1, person2, strength);
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
                {tribe.read().persons().map(|p| rsx! {
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
