pub mod app;
pub mod logic;

use dioxus::prelude::*;

use app::problem::Page as ProblemPage;
use app::solution::Page as SolutionPage;
use app::NotFound;

const FAVICON: &str = concat!(
    "data:image/svg+xml,",
    "<svg xmlns=%22http://www.w3.org/2000/svg%22 viewBox=%2210 0 100 100%22>",
    "<text y=%22.90em%22 font-size=%2290%22>🚀</text>",
    "</svg>"
);
const MAIN_CSS: Asset = asset!("/assets/main.css");

#[derive(Clone)]
struct ProblemSignal {
    pub tables: Signal<logic::Tables>,
    pub tribe: Signal<logic::Tribe>,
}

impl ProblemSignal {
    pub fn new() -> Self {
        Self {
            tables: Signal::new(logic::Tables::new()),
            tribe: Signal::new(logic::Tribe::new()),
        }
    }
}

#[derive(Clone)]
struct SolutionSignal {
    pub assignment: Signal<Result<logic::Assignment, logic::UnsolvableError>>,
    pub outdated: Signal<SolutionState>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum SolutionState {
    Missing,
    Outdated,
    Valid,
}

impl SolutionSignal {
    pub fn new() -> Self {
        Self {
            assignment: Signal::new(Err("There is no solution".into())),
            outdated: Signal::new(SolutionState::Missing),
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

    // FIXME so much for encapsulation but could not manage to make it run in `new`.
    // Perhaps using a custom hook?
    use_effect(move || {
        let _r1 = &pb.tribe.read();
        let _r2 = &pb.tables.read();
        if *sol.outdated.peek() != SolutionState::Missing {
            sol.outdated.set(SolutionState::Outdated);
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
