use dioxus::prelude::*;

use crate::Route;

#[component]
pub fn Layout() -> Element {
    let path: Route = use_route();

    rsx! {
        nav { class: "bg-base-200 w-screen flex items-center justify-between py-2 px-8",
            div { class: "basis-1/4 justify-self-start",
                span { class: "text-2xl mr-2", "🧅" }
                span { class: "text-xl", "Optizoignons" }
            }
            ul { class: "flex justify-center items-center gap-4 menu menu-horizontal rounded-box",
                li {
                    Link {
                        class: if let Route::ProblemPage { .. } = path { "menu-active" },
                        to: Route::ProblemPage {},
                        "Problem"
                    }
                }
                li {
                    Link {
                        class: if let Route::SolutionPage { .. } = path { "menu-active" },
                        to: Route::SolutionPage {},
                        "Solution"
                    }
                }
            }
            div { class: "basis-1/4", "" }
        }
        main { class: "bg-base-200", Outlet::<Route> {} }
    }
}
