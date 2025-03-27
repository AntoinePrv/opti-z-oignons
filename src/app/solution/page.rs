use dioxus::prelude::*;

use crate::logic::{self, Assignment, UnsolvableError};

#[component]
pub fn Page() -> Element {
    let pb: crate::ProblemSignal = use_context();
    let mut solution: crate::SolutionSignal = use_context();

    rsx! {
        if let Err(err) = &(*solution.assignment.read()) {
            p { "{err}" }
        }
        button {
            onclick: move |_| {
                solution.assignment.set(logic::fake_solve(&pb.tables.read(), &pb.tribe.read()));
            },
            "Solve"
        }
        AssignmentList { solution: solution.assignment }
    }
}

#[component]
fn AssignmentList(solution: Signal<Result<Assignment, UnsolvableError>>) -> Element {
    if solution.read().is_err() {
        return rsx!();
    }

    rsx! {
        p { "Table assignment:" }
        ul {
            for (i , group) in solution.read().as_ref().unwrap().iter().enumerate() {
                li {
                    p { "Table {i}:" }
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
