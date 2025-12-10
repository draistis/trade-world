use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    path, StaticSegment,
};

use crate::views::*;
use crate::{
    components::inventory::DragState,
    entities::{tile::TileState, GameState, Tile},
};

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
            <body class="antialiased bg-primary-bg text-primary-text">
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
        id: "STR-1001",
        description: "Test tile 1",
        resources: ["Water: 60%", "Wood: 24%"].into(),
        price: 490.90,
        is_owned: RwSignal::new(false),
        row: 0,
        col: 0,
        tile_state: TileState::new(),
        ..Default::default()
    });
    tiles.push(Tile {
        id: "STR-1002",
        description: "Test tile 2",
        resources: ["Water: 60%", "Wood: 24%"].into(),
        price: 121.00,
        is_owned: RwSignal::new(true),
        row: 0,
        col: 1,
        tile_state: TileState::new(),
        ..Default::default()
    });
    tiles.push(Tile {
        id: "STR-1003",
        description: "Test tile 3",
        resources: ["Water: 60%", "Wood: 24%"].into(),
        price: 525.00,
        is_owned: RwSignal::new(false),
        row: 1,
        col: 0,
        tile_state: TileState::new(),
        ..Default::default()
    });
    tiles.push(Tile {
        id: "STR-1004",
        description: "Test tile 4",
        resources: ["Water: 60%", "Wood: 24%"].into(),
        price: 710.55,
        is_owned: RwSignal::new(false),
        row: 1,
        col: 1,
        tile_state: TileState::new(),
        ..Default::default()
    });

    game_state.tiles = RwSignal::new(tiles);

    provide_context(game_state);
    provide_context(DragState {
        dragging: RwSignal::new(None),
        mouse_pos: RwSignal::new((0, 0)),
    });
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
                    <Route path=path!("/tile/:id") view=TilePage />
                </Routes>
            </main>
        </Router>
    }
}
