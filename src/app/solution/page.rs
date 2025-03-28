use dioxus::prelude::*;

use crate::{
    logic::{self, Assignment, UnsolvableError},
    SolutionState,
};

#[component]
pub fn Page() -> Element {
    let pb: crate::ProblemSignal = use_context();
    let mut solution: crate::SolutionSignal = use_context();

    rsx! {
        SolveText { outdated: *solution.outdated.read() }
        button {
            onclick: move |_| {
                solution.assignment.set(logic::fake_solve(&pb.tables.read(), &pb.tribe.read()));
                solution.outdated.set(SolutionState::Valid);
            },
            disabled: solve_disabled(*solution.outdated.read()),
            "{solve_text(*solution.outdated.read())}"
        }
        AssignmentList { assignment: solution.assignment }
    }
}

#[component]
fn SolveText(outdated: SolutionState) -> Element {
    rsx!(
        if outdated == SolutionState::Missing {
            p { "There is no ongoing solution" }
        }
        if outdated == SolutionState::Outdated {
            // TODO Warning style
            p { "The problem has changed since the last solution" }
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
fn AssignmentList(assignment: Signal<Result<Assignment, UnsolvableError>>) -> Element {
    if let Err(err) = &(*assignment.read()) {
        return rsx!(
            p { "{err}" }
        );
    }

    rsx! {
        p { "Table assignment:" }
        ul {
            for (table_name , group) in assignment.read().as_ref().unwrap().iter() {
                li {
                    p { "Table {table_name}:" }
                    ul {
                        for person in group {
                            li { key: person, "{person}" }
                        }
                    }
                }
            }
        }
    }
}
