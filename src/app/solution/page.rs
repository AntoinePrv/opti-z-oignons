use dioxus::prelude::*;
use dioxus_free_icons::{Icon, icons::ld_icons as icons};

use crate::SolutionState;
use crate::app::ui::{AssignedSchema, CardSimple, UnassignedSchema};
use crate::logic::model::Assignment;

#[component]
pub fn Page() -> Element {
    let pb: crate::ProblemSignal = use_context();
    let solution: crate::SolutionSignal = use_context();

    rsx! {
        Schema { pb: pb.clone(), solution: solution.clone() }
        div { class: "px-8",
            ControlBar { class: "py-4", pb: pb.clone(), solution: solution.clone() }
            AssignmentSection { solution }
        }
    }
}

#[component]
fn ControlBar(
    pb: crate::ProblemSignal,
    solution: crate::SolutionSignal,
    #[props(default)] class: &'static str,
) -> Element {
    rsx! {
        div { class: format!("flex justify-between items-center {}", class),
            SolveText { state: solution.state }
            SolveButton { pb, solution }
        }
    }
}

#[component]
fn Schema(pb: crate::ProblemSignal, solution: crate::SolutionSignal) -> Element {
    rsx! {

        match *solution.state.read() {
            SolutionState::Missing => {
                rsx! {
                    UnassignedSchema { tribe: pb.tribe, tables: pb.tables }
                }
            }
            _ => {
                rsx! {
                    AssignedSchema { assignment: solution.assignment, tables: pb.tables }
                }
            }
        }
    }
}

#[component]
fn SolveButton(pb: crate::ProblemSignal, solution: crate::SolutionSignal) -> Element {
    rsx! {
        button {
            class: "btn btn-primary",
            onclick: move |_| {
                match crate::logic::solver::solve(&pb.tables.read(), &pb.tribe.read()) {
                    Ok(assignment) => {
                        solution.state.set(SolutionState::Valid);
                        solution.assignment.set(assignment);
                    }
                    Err(err) => {
                        solution.state.set(SolutionState::Error(err));
                    }
                }
            },
            disabled: solve_disabled(solution.state),
            "{solve_text(solution.state)}"
        }
    }
}

#[component]
fn SolveText(state: Signal<SolutionState>) -> Element {
    rsx! {
        if *state.read() == SolutionState::Missing {
            div { role: "alert", class: "alert alert-info",
                Icon { icon: icons::LdInfo }
                span { "There is no ongoing solution" }
            }
        } else if *state.read() == SolutionState::Outdated {
            div { role: "alert", class: "alert alert-warning",
                Icon { icon: icons::LdTriangleAlert }
                span { "The problem has changed since the last solution" }
            }
        } else if let SolutionState::Error(error) = &(*state.read()) {
            div { role: "alert", class: "alert alert-error",
                Icon { icon: icons::LdCircleX }
                span { "Error: {error}" }
            }
        } else {
            div {}
        }
    }
}

fn solve_text(state: Signal<SolutionState>) -> &'static str {
    match *state.read() {
        SolutionState::Missing => "Solve",
        SolutionState::Outdated => "Solve again",
        SolutionState::Error(_) => "Solve again",
        SolutionState::Valid => "Up to date",
    }
}

fn solve_disabled(state: Signal<SolutionState>) -> bool {
    *state.read() == SolutionState::Valid
}

#[component]
fn AssignmentSection(solution: crate::SolutionSignal) -> Element {
    rsx! {
        if *solution.state.read() == SolutionState::Missing {
            AssignmentSkeleton {}
        } else {
            AssignmentList { assignment: solution.assignment }
        }
    }
}

#[component]
fn AssignmentList(assignment: Signal<Assignment>) -> Element {
    rsx! {
        div { class: "flex flex-wrap gap-4",
            for table_name in assignment.read().keys().cloned() {
                div { class: "flex-1",
                    TableCard {
                        name: table_name.clone(),
                        group: assignment.map(move |a| &a[&table_name]),
                    }
                }
            }
        }
    }
}

#[component]
fn AssignmentSkeleton(#[props(default = 4)] element_count: usize) -> Element {
    rsx! {
        div { class: "flex flex-wrap gap-4",
            for _ in 0..element_count {
                div { class: "flex-1 skeleton h-96" }
            }
        }
    }
}

#[component]
fn TableCard(name: String, group: MappedSignal<Vec<String>>) -> Element {
    rsx! {
        CardSimple { title: "Table {name}",
            table { class: "table",
                tbody {
                    for person in group.read().iter() {
                        tr {
                            td { key: person, "{person}" }
                        }
                    }
                }
            }
        }
    }
}
