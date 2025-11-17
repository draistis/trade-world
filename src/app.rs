use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

use crate::entities::GameState;
use crate::views::*;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
            </head>
            <body class="antialiased bg-[#0a0a0a] text-[#fafafa]">
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn ProvideGameState() -> impl IntoView {
    let game_state = GameState::new();
    provide_context(game_state);
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/trade-world.css" />

        // sets the document title
        <Title text="Trade World" />

        // content for this welcome page
        <Router>
            <main>
                <ProvideGameState />
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage />
                    <Route path=StaticSegment("/forestry") view=ForestryPage />
                    <Route path=StaticSegment("/tile-map") view=TileMapPage />
                </Routes>
            </main>
        </Router>
    }
}
