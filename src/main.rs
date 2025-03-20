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
    use_context_provider(ProblemSignal::new);
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Router::<Route> {}
    }
}

fn main() {
    dioxus::launch(App);
}
