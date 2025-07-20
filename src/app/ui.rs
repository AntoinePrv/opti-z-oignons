use dioxus::prelude::*;
use dioxus_free_icons::{Icon, icons::ld_icons as icons};

use crate::logic::model::{Assignment, PersonName, Tables, Tribe};

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

#[component]
pub fn CardSimple(title: String, children: Element) -> Element {
    rsx! {
        Card {
            header: rsx! {
                div { {title} }
            },
            body: children,
        }
    }
}

#[component]
pub fn Person(name: String) -> Element {
    rsx! {
        div { class: "tooltip tooltip-bottom tooltip-success", "data-tip": name,
            Icon {
                class: "stroke-success stroke-2",
                width: 20,
                height: 20,
                icon: icons::LdPersonStanding,
            }
        }
    }
}

#[component]
pub fn ArmChairIcon() -> Element {
    rsx! {
        div { class: "w-4 h-4 rounded-r-full border-4 border-l-0 border-neutral bg-neutral-content shadow-md" }
    }
}

#[component]
pub fn PersonFromAbove() -> Element {
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
pub fn ArmchairWithPerson() -> Element {
    rsx! {
        div { class: "relative w-4 h-4",
            div { class: "absolute", ArmChairIcon {} }
            div { class: "absolute rotate-90", PersonFromAbove {} }
        }
    }
}

#[component]
pub fn ArmchairWithMaybePerson(empty: bool) -> Element {
    rsx! {
        if empty {
            ArmChairIcon {}
        } else {
            ArmchairWithPerson {}
        }
    }
}

#[component]
fn Rotated(angle: f32, children: Element) -> Element {
    return rsx! {
        div { style: "transform:  rotate({angle}deg);", {children} }
    };
}

fn seat_angle_deg(i: u32, n: u32) -> f32 {
    const FULL_CIRCLE: f32 = 360.0;
    (i as f32) * FULL_CIRCLE / (n as f32)
}

fn seat_translate_unit(i: u32, n: u32) -> (f32, f32) {
    const FULL_CIRCLE: f64 = std::f64::consts::PI * 2.0;
    let angle = (i as f64) * FULL_CIRCLE / (n as f64);
    ((angle.cos() as f32), (angle.sin() as f32))
}

#[component]
pub fn TableAndChairs(
    n_seats: u32,
    name: String,
    #[props(default)] persons: Option<Vec<PersonName>>,
) -> Element {
    let n_persons = persons.as_ref().map(Vec::len).unwrap_or(0);
    let remaining = n_seats as usize - n_persons;
    const SEAT_TRANSLATE_PX: f32 = 40.0;

    let positions = (0..n_seats).map(|i| {
        let (tx, ty) = seat_translate_unit(i, n_seats);
        (
            seat_angle_deg(i, n_seats),
            tx * SEAT_TRANSLATE_PX,
            ty * SEAT_TRANSLATE_PX,
        )
    });

    let seats_is_empty =
        std::iter::repeat_n(false, n_persons).chain(std::iter::repeat_n(true, remaining));

    let persons = persons
        .into_iter()
        .flatten()
        .map(Some)
        .chain(std::iter::repeat_n(None, remaining));

    rsx! {
        div { class: "relative w-28 h-28",
            // Circle for the table
            div {
                class: concat!(
                    "absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2",
                    " w-12 h-12 bg-neutral rounded-full shadow-md ",
                ),
            }
            // All the chairs
            for ((angle , tx , ty) , empty) in positions.clone().zip(seats_is_empty) {
                div {
                    class: concat!("absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2"),
                    style: "transform: translateX({tx}px) translateY({ty}px);",
                    Rotated { angle,
                        ArmchairWithMaybePerson { empty }
                    }
                }
            }
            // Tooltip are hidden under other chairs or tables.
            // z-index does not help, perhaps because of stacking index?
            // This solution is still imperfect as the tooltip can hide under other
            // tables.
            for ((_ , tx , ty) , person) in positions.zip(persons) {
                div {
                    class: concat!(
                        "absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2",
                        " w-4 h-4 tooltip tooltip-success tooltip-bottom",
                    ),
                    style: "transform: translateX({tx}px) translateY({ty}px);",
                    "data-tip": person,
                }
            }
            div {
                class: concat!(
                    "absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2",
                    " w-12 h-12 tooltip tooltip-neutral tooltip-bottom",
                ),
                "data-tip": name,
            }
        }
    }
}

#[component]
pub fn UnassignedSchema(tribe: Signal<Tribe>, tables: Signal<Tables>) -> Element {
    let half_persons = use_memo(move || tribe.read().persons_count() / 2);

    rsx! {
        div { class: "flex gap-8 w-full justify-center",
            div { class: "basis-1/8 flex flex-wrap justify-end items-center content-center gap-2",
                for person in tribe.read().persons().take(half_persons()) {
                    Person { name: person }
                }
            }
            div { class: "basis-1/2 flex flex-wrap justify-center gap-2",
                for (name , kind) in tables.read().iter() {
                    TableAndChairs { n_seats: kind.n_seats, name }
                }
            }
            div { class: "basis-1/8 flex flex-wrap justify-start items-center content-center gap-2",
                for person in tribe.read().persons().skip(half_persons()) {
                    Person { name: person }
                }
            }
        }
    }
}

#[component]
pub fn AssignedSchema(tables: Signal<Tables>, assignment: Signal<Assignment>) -> Element {
    rsx! {
        div { class: "flex gap-8 w-full justify-center",
            div { class: "basis-1/8" }
            div { class: "basis-1/2 flex flex-wrap justify-center gap-2",
                for (name , kind) in tables.read().iter() {
                    TableAndChairs {
                        n_seats: kind.n_seats,
                        name,
                        persons: assignment.read().get(name).cloned(),
                    }
                }
            }
            div { class: "basis-1/8" }
        }
    }
}
