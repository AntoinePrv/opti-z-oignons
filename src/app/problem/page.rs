use dioxus::prelude::*;
use dioxus_free_icons::{Icon, icons::ld_icons as icons};

use crate::app::ui::Card;
use crate::logic::model::{RelationStrength, TableType, Tables, Tribe};

#[component]
pub fn Page() -> Element {
    let pb: crate::ProblemSignal = use_context();

    rsx! {
        Schema { tables: pb.tables, tribe: pb.tribe }
        ShowMeHowButton {
            class: "fixed bottom-4 right-4 z-50",
            tables: pb.tables,
            tribe: pb.tribe,
        }
        div { class: "p-8 flex gap-8",
            div { class: "basis-1/3",
                Card {
                    header: rsx! {
                        div { class: "w-full flex justify-between",
                            h2 { "Tables" }
                            TableInput { tables: pb.tables }
                        }
                    },
                    body: rsx! {
                        TableList { tables: pb.tables }
                    },
                }
            }
            div { class: "basis-1/3",
                Card {
                    header: rsx! {
                        div { class: "w-full flex justify-between",
                            h2 { "Persons" }
                            PersonInput { tribe: pb.tribe }
                        }
                    },
                    body: rsx! {
                        PersonList { tribe: pb.tribe }
                    },
                }
            }
            div { class: "basis-1/3",
                Card {
                    header: rsx! {
                        div { class: "w-full flex justify-between",
                            h2 { "Relations" }
                            RelationInput { tribe: pb.tribe }
                        }
                    },
                    body: rsx! {
                        RelationList { tribe: pb.tribe }
                    },
                }
            }
        }
    }
}

#[component]
fn TableAddIcon() -> Element {
    rsx! {
        Icon {
            class: "stroke-base-content stroke-2",
            width: 20,
            height: 20,
            icon: icons::LdPlus,
        }
    }
}

#[component]
fn ShowMeHowButton(tribe: Signal<Tribe>, tables: Signal<Tables>, class: &'static str) -> Element {
    rsx! {
        button {
            class: format!("btn btn-primary {}", class),
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

#[component]
fn PersonIcon() -> Element {
    rsx! {
        Icon {
            class: "stroke-success stroke-2",
            width: 20,
            height: 20,
            icon: icons::LdPersonStanding,
        }
    }
}

#[component]
fn ArmChairIcon() -> Element {
    rsx! {
        div { class: "w-4 h-4 rounded-r-full border-4 border-l-0 border-neutral bg-neutral-content shadow-md" }
    }
}

#[component]
fn PersonFromAbove() -> Element {
    rsx! {
        div { class: "relative w-4 h-4",
            // Head
            div { class: "absolute top-1/2  left-1/2 -translate-1/2 w-2.5 h-2.5 rounded-full bg-success" }
            // Arms
            // -translate-x-1/2 to center -translate-x-1/2 to offset
            div { class: "absolute top-1/2 left-1/2 -translate-x-full w-1 h-3 origin-top rotate-45 rounded bg-success" }
            // -translate-x-1/2 to center +translate-x-1/2 to offset
            div { class: "absolute top-1/2 left-1/2  w-1 h-3 origin-top -rotate-45 rounded bg-success" }
        }
    }
}

#[component]
fn ArmchairWithPerson() -> Element {
    rsx! {
        div { class: "relative w-4 h-4",
            div { class: "absolute", ArmChairIcon {} }
            div { class: "absolute rotate-90", PersonFromAbove {} }
        }
    }
}

#[component]
fn TableAndChairs(n_seats: u32) -> Element {
    let angle = 360.0 / (n_seats as f32);
    rsx! {
        div { class: "relative w-28 h-28",
            // Circle for the table
            div { class: "absolute top-1/2 left-1/2 w-12 h-12 -translate-x-1/2 -translate-y-1/2 bg-neutral rounded-full shadow-md" }
            // All the chairs
            for i in (0..n_seats).map(|i| i as f32) {
                div {
                    class: "absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2",
                    style: "transform: rotate({i * angle}deg) translateX(40px);",
                    ArmChairIcon {}
                }
            }
        }
    }
}

#[component]
fn Schema(tribe: Signal<Tribe>, tables: Signal<Tables>) -> Element {
    let half_persons = use_memo(move || tribe.read().persons_count() / 2);

    rsx! {
        div { class: "flex gap-8 w-full justify-center",
            div { class: "basis-1/8 flex flex-wrap justify-end items-center content-center gap-2",
                for person in tribe.read().persons().take(half_persons()) {
                    div {
                        class: "tooltip",
                        "data-tip": "{person}",
                        key: person,
                        PersonIcon {}
                    }
                }
            }
            div { class: "basis-1/2 flex flex-wrap justify-center gap-2",
                for (name , kind) in tables.read().iter() {
                    div { class: "tooltip", "data-tip": "{name}", key: name,
                        TableAndChairs { n_seats: kind.n_seats }
                    }
                }
            }
            div { class: "basis-1/8 flex flex-wrap justify-start items-center content-center gap-2",
                for person in tribe.read().persons().skip(half_persons()) {
                    div {
                        class: "tooltip",
                        "data-tip": "{person}",
                        key: "{person}",
                        PersonIcon {}
                    }
                }
            }
        }
    }
}

#[component]
fn SectionTrashButton(onclick: EventHandler<MouseEvent>) -> Element {
    rsx! {
        button { class: "btn btn-xs aspect-square p-0", onclick,
            Icon {
                class: "stroke-error",
                width: 15,
                height: 15,
                icon: icons::LdTrash2,
            }
        }
    }
}

fn safe_html_id(input: &str) -> String {
    let mut hash: u64 = 5381;
    for b in input.bytes() {
        hash = (hash.wrapping_shl(5)).wrapping_add(hash) ^ b as u64;
    }
    format!("id{:x}", hash)
}

#[component]
fn SectionAdd(title: String, children: Element) -> Element {
    let modal_id = safe_html_id(title.as_ref());

    rsx! {
        button {
            class: "btn btn-sm aspect-square p-0",
            onclick: {
                let js_code = format!("return {}.showModal() == undefined", &modal_id);
                move |_| {
                    let js_code = js_code.clone();
                    async move {
                        document::eval(&js_code).await.unwrap();
                    }
                }
            },
            TableAddIcon {}
        }
        dialog { id: modal_id.clone(), class: "modal",
            // Dialog content
            div { class: "modal-box space-y-4",
                h3 { class: "card-title", {title} }
                {children}
            }
            // Closing the dialog/popup/modal form
            form { method: "dialog", class: "modal-backdrop",
                button {
                    onclick: {
                        let js_code = format!("return {}.close() == undefined", &modal_id);
                        move |_| {
                            let js_code = js_code.clone();
                            async move {
                                document::eval(&js_code).await.unwrap();
                            }
                        }
                    },
                    "Close"
                }
            }
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
                            SectionTrashButton {
                                onclick: {
                                    let person = person.to_owned();
                                    move |_| {
                                        tribe.write().remove_person(&person);
                                    }
                                },
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
    const PERSON_NAME_ID: &str = "person_name";

    rsx! {
        SectionAdd { title: "Add a new person",
            // TODO: add "tribe" for auto conflicts
            // https://developer.mozilla.org/en-US/docs/Web/HTML/Element/datalist
            form {
                class: "mx-auto space-y-2",
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
                label { r#for: PERSON_NAME_ID, class: "floating-label",
                    input {
                        id: PERSON_NAME_ID,
                        name: PERSON_NAME_ID,
                        r#type: "text",
                        minlength: 1,
                        class: "input focus:outline-none w-full",
                        placeholder: "Person name",
                    }
                    span { "Person name" }
                }
                button {
                    class: "btn btn-primary ml-auto block w-32",
                    r#type: "submit",
                    "Add"
                }
            }
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
                            SectionTrashButton {
                                onclick: {
                                    let name = name.to_owned();
                                    move |_| {
                                        tables.write().remove(&name);
                                    }
                                },
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
    const TABLE_SEATS_ID: &str = "table_seats";
    const TABLE_NAME_ID: &str = "table_name";

    rsx! {
        SectionAdd { title: "Add multiple tables",
            form {
                class: "mx-auto space-y-2",
                onsubmit: move |event| {
                    let mut data = event.data.values();
                    let n_seats_input = data
                        .remove(TABLE_SEATS_ID)
                        .map(|val| val.as_value())
                        .and_then(|val| val.parse::<u32>().ok());
                    let name_input = data.remove(TABLE_NAME_ID).map(|val| val.as_value());
                    if let Some((n_seats, name)) = n_seats_input.zip(name_input) {
                        tables.write().insert(name, TableType { n_seats });
                    }
                },
                label { r#for: TABLE_NAME_ID, class: "floating-label",
                    input {
                        id: TABLE_NAME_ID,
                        name: TABLE_NAME_ID,
                        r#type: "text",
                        minlength: 1,
                        class: "input focus:outline-none w-full",
                        placeholder: "Table name",
                    }
                    span { "Table name" }
                }
                label { r#for: TABLE_SEATS_ID, class: "floating-label",
                    input {
                        id: TABLE_SEATS_ID,
                        name: TABLE_SEATS_ID,
                        r#type: "number",
                        min: 0,
                        step: 1,
                        class: "input focus:outline-none w-full",
                        placeholder: "Number of seats",
                    }
                    span { "Number of seats" }
                }
                button {
                    class: "btn btn-primary ml-auto block w-32",
                    r#type: "submit",
                    "Add"
                }
            }
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
                            SectionTrashButton {
                                onclick: {
                                    let p1 = p1.to_owned();
                                    let p2 = p2.to_owned();
                                    move |_| {
                                        tribe.write().remove_relation(&p1, &p2);
                                    }
                                },
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
    const RELATION_STRENGTH_ID: &str = "relation_strength";
    const RELATION_STRENGTH_DATALIST_ID: &str = "relation_strength_datalist";
    const RELATION_PERSON_1_ID: &str = "relation_person_1";
    const RELATION_PERSON_2_ID: &str = "relation_person_2";
    const RELATION_PERSON_DATALIST_ID: &str = "relation_person_datalist";

    rsx! {
        SectionAdd { title: "Add a relation between two persons",
            form {
                class: "mx-auto space-y-4",
                onsubmit: move |event| {
                    let mut data = event.data.values();
                    let person1 = data.remove(RELATION_PERSON_1_ID).map(|val| val.as_value());
                    let person2 = data.remove(RELATION_PERSON_2_ID).map(|val| val.as_value());
                    let strength = data
                        .remove(RELATION_STRENGTH_ID)
                        .map(|val| val.as_value())
                        .and_then(|val| val.parse::<usize>().ok())
                        .and_then(RelationStrength::from_repr);
                    if let Some(((person1, person2), strength)) = person1.zip(person2).zip(strength)
                    {
                        tribe.write().add_relation(person1, person2, strength);
                    }
                },

                datalist { id: RELATION_PERSON_DATALIST_ID,
                    for p in tribe.read().persons() {
                        option { value: "{p}" }
                    }
                }
                label { r#for: RELATION_PERSON_1_ID, class: "floating-label",
                    input {
                        id: RELATION_PERSON_1_ID,
                        name: RELATION_PERSON_1_ID,
                        r#type: "text",
                        list: RELATION_PERSON_DATALIST_ID,
                        minlength: 1,
                        class: "input focus:outline-none w-full",
                        placeholder: "First person name",
                    }
                    span { "First Person name" }
                }
                label { class: "hidden", r#for: RELATION_STRENGTH_ID, "Strength" }
                div {
                    input {
                        id: RELATION_STRENGTH_ID,
                        name: RELATION_STRENGTH_ID,
                        r#type: "range",
                        list: RELATION_STRENGTH_DATALIST_ID,
                        min: RelationStrength::min() as usize,
                        max: RelationStrength::max() as usize,
                        step: 1,
                        value: RelationStrength::max() as usize,
                        class: "range range-primary range-xs w-full",
                    }
                    div { class: "flex justify-between mb-2 text-xs",
                        for strength in RelationStrength::iter() {
                            span { "{strength}" }
                        }
                    }
                }
                label { r#for: RELATION_PERSON_2_ID, class: "floating-label",
                    input {
                        id: RELATION_PERSON_2_ID,
                        name: RELATION_PERSON_2_ID,
                        r#type: "text",
                        list: RELATION_PERSON_DATALIST_ID,
                        minlength: 1,
                        class: "input focus:outline-none w-full",
                        placeholder: "Second person name",
                    }
                    span { "Second Person name" }
                }
                button {
                    class: "btn btn-primary ml-auto block w-32",
                    r#type: "submit",
                    "Add"
                }
            }
        }
    }
}
