use dioxus::prelude::*;

use crate::Route;

#[component]
pub fn Layout() -> Element {
    rsx! {
        nav {
            class: concat!(
                " navbar bg-base-100 shadow-sm",
                " sticky top-0 z-100",
                " w-screen flex items-center justify-center ",
            ),
            div { class: "basis-1/4", "Optionions" }
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
        main { class: "bg-base-200", Outlet::<Route> {} }
    }
}
