use dioxus::prelude::*;
use dioxus_free_icons::{Icon, icons::ld_icons as icons};

use crate::logic::model::{RelationStrength, TableType, Tables, Tribe};

#[component]
pub fn Page() -> Element {
    let pb: crate::ProblemSignal = use_context();

    // Fill some data in Debug mode
    #[cfg(debug_assertions)]
    {
        let (ex_tribe, ex_tables) = crate::logic::examples::harry_potter();
        pb.tribe.clone().set(ex_tribe);
        pb.tables.clone().set(ex_tables);
    }

    rsx! {
        Schema { tables: pb.tables, tribe: pb.tribe }
        ShowMeHow { tables: pb.tables, tribe: pb.tribe }
        div { class: "p-8 flex gap-8",
            div { class: "basis-1/3",
                SectionCard {
                    title: rsx! {
                        h2 { "Tables" }
                    },
                    body: rsx! {
                        TableList { tables: pb.tables }
                    },
                    input: rsx! {
                        TableInput { tables: pb.tables }
                    },
                }
            }
            div { class: "basis-1/3",
                SectionCard {
                    title: rsx! {
                        h2 { "Persons" }
                    },
                    body: rsx! {
                        PersonList { tribe: pb.tribe }
                    },
                    input: rsx! {
                        PersonInput { tribe: pb.tribe }
                    },
                }
            }
            div { class: "basis-1/3",
                SectionCard {
                    title: rsx! {
                        h2 { "Relations" }
                    },
                    body: rsx! {
                        RelationList { tribe: pb.tribe }
                    },
                    input: rsx! {
                        RelationInput { tribe: pb.tribe }
                    },
                }
            }
        }
    }
}

#[component]
fn ShowMeHow(tribe: Signal<Tribe>, tables: Signal<Tables>) -> Element {
    rsx! {
        button {
            class: "btn",
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
        div { class: "flex gap-8",
            ul { class: "basis-1/2 flex flex-wrap justify-center gap-2",
                for (_name , kind) in tables.read().iter() {
                    li { key: _name, "{fmt_table(kind.n_seats)}" }
                }
            }
            ul { class: "basis-1/2 flex flex-wrap justify-center gap-2",
                for person in tribe.read().persons() {
                    li { key: "{person}", "🐷" }
                }
            }
        }
    }
}

#[component]
fn SectionCard(title: Element, body: Element, input: Element) -> Element {
    rsx! {
        div { class: "card bg-base-100 p-4 shadow-sm",
            div { class: "card-title", {title} }
            div { class: "card-body", {body} }
            div { class: "card-action", {input} }
        }
    }
}

#[component]
fn TableTrashIcon() -> Element {
    rsx! {
        Icon {
            class: "stroke-error",
            width: 15,
            height: 15,
            icon: icons::LdTrash2,
        }
    }
}

#[component]
fn PersonList(tribe: Signal<Tribe>) -> Element {
    rsx! {
        table { class: "table",
            thead {
                th { "Name" }
                th { class: "w-4" }
            }
            tbody {
                for person in tribe.read().persons() {
                    tr {
                        td { "{person}" }
                        td {
                            button {
                                class: "btn btn-xs p-1",
                                onclick: {
                                    let person = person.to_owned();
                                    move |_| {
                                        tribe.write().remove_person(&person);
                                    }
                                },
                                TableTrashIcon {}
                            }
                        }
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
            button { class: "btn", r#type: "submit", "✔️" }
        }
    }
}

#[component]
fn TableList(tables: Signal<Tables>) -> Element {
    rsx! {
        table { class: "table",
            thead {
                th { "Name" }
                th { "Seats" }
                th { class: "w-4" }
            }
            tbody {
                for (name , table) in tables.read().iter() {
                    tr {
                        td { "{name}" }
                        td { "{table.n_seats}" }
                        td {
                            button {
                                class: "btn btn-xs",
                                onclick: {
                                    let name = name.to_owned();
                                    move |_| {
                                        tables.write().remove(&name);
                                    }
                                },
                                TableTrashIcon {}
                            }
                        }
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
            button { class: "btn", r#type: "submit", "Add tables" }
        }
    }
}

#[component]
fn RelationList(tribe: Signal<Tribe>) -> Element {
    rsx! {
        table { class: "table",
            thead {
                th { "Person" }
                th { "Strength" }
                th { "Person" }
                th { class: "w-4" }
            }
            tbody {
                for (p1 , p2 , strength) in tribe.read().relations() {
                    tr {
                        td { "{p1}" }
                        td { "{strength}" }
                        td { "{p2}" }
                        td {
                            button {
                                class: "btn btn-xs",
                                onclick: {
                                    let p1 = p1.to_owned();
                                    let p2 = p2.to_owned();
                                    move |_| {
                                        tribe.write().remove_relation(&p1, &p2);
                                    }
                                },
                                TableTrashIcon {}
                            }
                        }
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
                for p in tribe.read().persons() {
                    option { value: "{p}" }
                }
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

            button { class: "btn", r#type: "submit", "Add a relation" }
        }
    }
}
