use dioxus::prelude::*;

use crate::logic::{self, Assignment, UnsolvableError};

#[component]
pub fn Page() -> Element {
    let pb: crate::ProblemSignal = use_context();

    let mut assignment: Signal<Option<Assignment>> = use_signal(|| None);
    let mut error: Signal<Option<UnsolvableError>> = use_signal(|| None);

    rsx! {
        if assignment.read().is_none() {
            p { "There is no solution!" }
        }
        if let Some(err) = error.read().as_ref() {
            p { "Error: {err}!" }
        }
        button {
            onclick: move |_| {
                match logic::fake_solve(&pb.tables.read(), &pb.tribe.read()) {
                    Ok(a) => {
                        assignment.set(Some(a));
                        error.set(None);
                    }
                    Err(err) => error.set(Some(err)),
                }
            },
            "Solve"
        }
        AssignmentList { assignment }
    }
}

#[component]
fn AssignmentList(assignment: Signal<Option<Assignment>>) -> Element {
    if assignment.read().is_none() {
        return rsx!();
    }

    rsx! {
        p { "Table assignment:" }
        ul {
            for (i , group) in assignment.read().as_ref().unwrap().iter().enumerate() {
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
