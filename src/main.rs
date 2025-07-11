pub mod app;
pub mod logic;
pub mod name_generator;

use dioxus::prelude::*;

use app::NotFound;
use app::problem::Page as ProblemPage;
use app::solution::Page as SolutionPage;
use logic::{
    model::{Assignment, Tables, Tribe},
    solver::SolverError,
};

const FAVICON: &str = concat!(
    "data:image/svg+xml,",
    "<svg xmlns=%22http://www.w3.org/2000/svg%22 viewBox=%2210 0 100 100%22>",
    "<text y=%22.90em%22 font-size=%2290%22>🧅</text>",
    "</svg>"
);
const MAIN_CSS: Asset = asset!("/assets/main.css");

#[derive(Clone, PartialEq, Eq)]
struct ProblemSignal {
    pub tables: Signal<Tables>,
    pub tribe: Signal<Tribe>,
}

impl ProblemSignal {
    pub fn new() -> Self {
        Self {
            tables: Signal::new(Tables::new()),
            tribe: Signal::new(Tribe::new()),
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
struct SolutionSignal {
    pub assignment: Signal<Assignment>,
    pub state: Signal<SolutionState>,
}

#[derive(Clone, PartialEq, Eq)]
enum SolutionState {
    Missing,
    Outdated,
    Valid,
    Error(SolverError),
}

impl SolutionSignal {
    pub fn new() -> Self {
        Self {
            assignment: Signal::new(Assignment::new()),
            state: Signal::new(SolutionState::Missing),
        }
    }
}

#[derive(Routable, PartialEq, Clone)]
enum Route {
    #[layout(app::Layout)]
    #[route("/problem")]
    #[redirect("/", || Route::ProblemPage {})]
    ProblemPage {},
    #[route("/solution")]
    SolutionPage {},
    #[end_layout]
    #[route("/:..segments")]
    NotFound { segments: Vec<String> },
}

#[component]
fn App() -> Element {
    let pb = use_context_provider(ProblemSignal::new);
    let mut sol = use_context_provider(SolutionSignal::new);
    use_context_provider(|| Signal::new(name_generator::NameGenerator::new()));

    // Fill some data in Debug mode
    #[cfg(debug_assertions)]
    {
        let (ex_tribe, ex_tables) = crate::logic::examples::harry_potter();
        pb.tribe.clone().set(ex_tribe);
        pb.tables.clone().set(ex_tables);
    }

    // FIXME so much for encapsulation but could not manage to make it run in `new`.
    // Perhaps using a custom hook?
    use_effect(move || {
        let _r1 = &pb.tribe.read();
        let _r2 = &pb.tables.read();
        if *sol.state.peek() != SolutionState::Missing {
            sol.state.set(SolutionState::Outdated);
        }
    });

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Router::<Route> {}
    }
}

fn main() {
    dioxus::launch(App);
}
