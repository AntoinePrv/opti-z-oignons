use dioxus::prelude::*;
use dioxus_free_icons::{Icon, icons::ld_icons as icons};

use crate::app::ui::{Card, UnassignedSchema};
use crate::logic::model::{RelationStrength, TableType, Tables, Tribe};

#[component]
pub fn Page() -> Element {
    let pb: crate::ProblemSignal = use_context();

    rsx! {
        UnassignedSchema { tables: pb.tables, tribe: pb.tribe }
        ShowMeHowButton {
            class: "fixed bottom-4 right-4 z-50",
            tables: pb.tables,
            tribe: pb.tribe,
        }
        div { class: "space-y-2 p-2 pb-8 lg:px-8  lg:gap-4 lg:flex",
            div { class: "lg:basis-1/3",
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
            div { class: "lg:basis-1/3",
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
            div { class: "lg:basis-1/3",
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
    const SPLIT_CHARS: [char; 3] = [',', ';', '\n'];

    let mut persons = use_signal(Vec::<String>::new);
    let mut current = use_signal(String::new);
    let mut input_key = use_signal(|| 0);

    let parse_input = move |event: Event<FormData>| {
        let mut remaining = event.value();
        let mut count: usize = 0;
        while let Some((h, r)) = remaining.split_once(SPLIT_CHARS) {
            let h = h.trim();
            persons.push(h.trim().into());
            remaining = r.into();
            count += 1;
        }

        // Detect paste-like event (even when disabled) when there are more than one separator
        if count >= 2 {
            let remaining = remaining.trim();
            if !remaining.is_empty() {
                persons.push(remaining.into());
            }
            current.set("".into());
        }
        // User is still typing
        else {
            current.set(remaining);
        }
        // We must trigger a rerender to really clear the value from the pasted data.
        *input_key.write() += 1;
    };

    rsx! {
        SectionAdd { title: "Add a new person",
            // TODO: add "group" for auto conflicts
            // https://developer.mozilla.org/en-US/docs/Web/HTML/Element/datalist
            form { class: "mx-auto space-y-2",
                fieldset { class: "fieldset",
                    label { class: "floating-label input focus-within:outline-none w-full flex-wrap h-auto py-2",
                        // Phantom button so that the first delete button does not trigger when the
                        // whole element is hovered
                        button { class: "hidden" }
                        for (i , pers) in persons.read().iter().enumerate() {
                            if !pers.is_empty() {
                                div {
                                    key: "{i}",
                                    class: "badge badge-soft badge-accent overflow-hidden pr-0",
                                    span { "{pers}" }
                                    button {
                                        class: "btn-ghost cursor-pointer hover:bg-accent-content h-full pl-1 pr-2",
                                        onclick: move |_| {
                                            if let Some(name) = persons.write().get_mut(i) {
                                                name.clear();
                                            }
                                        },
                                        Icon {
                                            class: "size-[1em]",
                                            icon: icons::LdX,
                                        }
                                    }
                                }
                            }
                        }
                        textarea {
                            class: "resize-none w-auto min-w-60 overflow-hidden border-none outline-none focus:outline-none",
                            rows: 1,
                            placeholder: "First Person, Second Person, ...",
                            value: current,
                            key: input_key,
                            oninput: parse_input,
                        }
                        span { "Persons" }
                    }
                    p { class: "label",
                        Icon { class: "size-[1em]", icon: icons::LdInfo }
                        "You can data from a spreadsheet"
                    }
                }

                div { class: "divider", "OR" }

                button {
                    class: "btn btn-primary ml-auto block w-32",
                    r#type: "submit",
                    onclick: {
                        let mut persons = persons;
                        let mut current = current;
                        move |_| {
                            std::mem::take(&mut *persons.write())
                                .into_iter()
                                .for_each(|p| tribe.write().add_person(p));
                            let maybe_person = std::mem::take(&mut *current.write());
                            if !maybe_person.is_empty() {
                                tribe.write().add_person(maybe_person);
                            }
                        }
                    },
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
        SectionAdd { title: "Add tables",
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
