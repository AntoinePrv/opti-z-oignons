use dioxus::prelude::*;
use dioxus_free_icons::{Icon, icons::ld_icons as icons};

use crate::SolutionState;
use crate::app::ui::CardSimple;
use crate::logic::{model::Assignment, solver::SolverError};

#[component]
pub fn Page() -> Element {
    let pb: crate::ProblemSignal = use_context();
    let solution: crate::SolutionSignal = use_context();

    rsx! {
        div { class: "p-8",
            ControlBar { class: "py-4", pb, solution: solution.clone() }
            AssignmentList { assignment: solution.assignment }
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
            SolveText { outdated: *solution.outdated.read() }
            SolveButton { pb, solution }
        }
    }
}

#[component]
fn SolveButton(pb: crate::ProblemSignal, solution: crate::SolutionSignal) -> Element {
    rsx! {
        button {
            class: "btn btn-primary",
            onclick: move |_| {
                solution
                    .assignment
                    .set(crate::logic::solver::solve(&pb.tables.read(), &pb.tribe.read()));
                solution.outdated.set(SolutionState::Valid);
            },
            disabled: solve_disabled(*solution.outdated.read()),
            "{solve_text(*solution.outdated.read())}"
        }
    }
}

#[component]
fn SolveText(outdated: SolutionState) -> Element {
    rsx!(
        if outdated == SolutionState::Missing {
            div { role: "alert", class: "alert alert-info",
                Icon { icon: icons::LdInfo }
                span { "There is no ongoing solution" }
            }
        } else if outdated == SolutionState::Outdated {
            div { role: "alert", class: "alert alert-warning",
                Icon { icon: icons::LdTriangleAlert }
                span { "The problem has changed since the last solution" }
            }
        } else {
            div {}
        }
    )
}

fn solve_text(state: SolutionState) -> &'static str {
    match state {
        SolutionState::Missing => "Solve",
        SolutionState::Outdated => "Solve again",
        SolutionState::Valid => "Up to date",
    }
}

fn solve_disabled(state: SolutionState) -> bool {
    state == SolutionState::Valid
}

#[component]
fn AssignmentList(assignment: Signal<Result<Assignment, SolverError>>) -> Element {
    if let Err(err) = &(*assignment.read()) {
        return rsx!(
            p { "{err}" }
        );
    }

    let assignment = assignment.map(|a| a.as_ref().unwrap());

    rsx! {
        div { class: "flex flex-wrap gap-4",
            for table_name in assignment.read().keys().cloned() {
                div { class: "flex-1",
                    TableCard {
                        name: table_name.clone(),
                        group: assignment.clone().map(move |a| &a[&table_name]),
                    }
                }
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
