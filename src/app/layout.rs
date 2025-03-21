use dioxus::prelude::*;

use crate::Route;

#[component]
pub fn Layout() -> Element {
    rsx! {
        nav {
            ul {
                li {
                    Link { to: Route::ProblemPage {}, "Problem" }
                }
                li {
                    Link { to: Route::SolutionPage {}, "Solution" }
                }
            }
        }
        main { Outlet::<Route> {} }
    }
}
