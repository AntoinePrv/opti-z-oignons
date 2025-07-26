use dioxus::prelude::*;

use crate::Route;

#[component]
pub fn Layout() -> Element {
    rsx! {
        body { class: "min-h-screen max-w-full overflow-x-hidden flex flex-col",
            NavBar {}
            main { class: "flex-1 bg-base-200", Outlet::<Route> {} }
        }
    }
}

#[component]
fn NavBar() -> Element {
    let path: Route = use_route();

    rsx! {
        nav { class: "bg-base-200 w-full flex items-center justify-between py-2 px-8",
            div { class: "basis-1/4 justify-self-start",
                span { class: "text-2xl mr-2", "🧅" }
                span { class: "text-xl hidden md:inline", "Opti•z•oignons" }
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
    }
}
