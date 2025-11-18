use std::collections::HashMap;

use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

use crate::entities::{GameState, Tile};
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
    let mut game_state = GameState::new();

    let mut tiles = Vec::new();
    tiles.push(Tile {
        name: "STR-1001".to_string(),
        description: "Test tile 1".to_string(),
        resources: HashMap::from([
            ("Water".to_string(), "60%".to_string()),
            ("Wood".to_string(), "40%".to_string()),
        ]),
        price: 490.90,
        owned: RwSignal::new(false),
        row: 0,
        col: 0,
    });
    tiles.push(Tile {
        name: "STR-1002".to_string(),
        description: "Test tile 2".to_string(),
        resources: HashMap::from([
            ("Water".to_string(), "20%".to_string()),
            ("Wood".to_string(), "20%".to_string()),
            ("Grass".to_string(), "53%".to_string()),
        ]),
        price: 121.00,
        owned: RwSignal::new(true),
        row: 0,
        col: 1,
    });
    tiles.push(Tile {
        name: "STR-1003".to_string(),
        description: "Test tile 3".to_string(),
        resources: HashMap::from([
            ("Water".to_string(), "10%".to_string()),
            ("Wood".to_string(), "78%".to_string()),
        ]),
        price: 525.00,
        owned: RwSignal::new(false),
        row: 1,
        col: 0,
    });
    tiles.push(Tile {
        name: "STR-1004".to_string(),
        description: "Test tile 4".to_string(),
        resources: HashMap::from([
            ("Water".to_string(), "80%".to_string()),
            ("Fish".to_string(), "30%".to_string()),
        ]),
        price: 710.55,
        owned: RwSignal::new(false),
        row: 1,
        col: 1,
    });

    game_state.tiles = RwSignal::new(tiles);

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
