use dioxus::prelude::*;
use dioxus_free_icons::{Icon, icons::ld_icons as icons};

use crate::logic::model::{Tables, Tribe};

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
pub fn TableAndChairs(n_seats: u32, name: String) -> Element {
    let angle = 360.0 / (n_seats as f32);
    rsx! {
        div { class: "relative w-28 h-28",
            // All the chairs
            for i in (0..n_seats).map(|i| i as f32) {
                div {
                    class: "absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2",
                    style: "transform: rotate({i * angle}deg) translateX(40px);",
                    ArmChairIcon {}
                }
            }
            // Circle for the table
            div {
                class: concat!(
                    "w-12 h-12 absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2",
                    " bg-neutral rounded-full shadow-md tooltip tooltip-neutral tooltip-bottom",
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
