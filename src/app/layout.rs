use dioxus::prelude::*;

use crate::Route;

#[component]
pub fn Layout() -> Element {
    rsx! {
        nav { class: "w-screen sticky top-0 flex items-center justify-center bg-slate-100",
            div { class: "basis-1/4", "Group Assignment" }
            ul { class: "basis-1/2 flex justify-center items-center gap-4",
                li {
                    Link { to: Route::ProblemPage {}, "Problem" }
                }
                li {
                    Link { to: Route::SolutionPage {}, "Solution" }
                }
            }
            div { class: "basis-1/4", "" }
        }
        main { Outlet::<Route> {} }
    }
}
