use crate::{components::header::Header, entities::GameState};
use leptos::{ev::MouseEvent, prelude::*};

#[component]
pub fn TileMapPage() -> impl IntoView {
    provide_context::<SelectedTile>(SelectedTile(RwSignal::new(String::from("STR-1001"))));

    view! {
        <div class="h-screen flex flex-col">
            <Header />
            <div class="flex flex-1 overflow-hidden">
                <div class="flex-1 flex items-center justify-center bg-gray-900 border-r border-gray-700">
                    <TileMap />
                </div>
                <div class="w-1/3 flex flex-col bg-gray-900 border-r">
                    <TileOverview />
                </div>
            </div>
        </div>
    }
}

#[component]
fn TileMap() -> impl IntoView {
    view! {
        <div class="w-full h-full overflow-hidden p-20 ml-0 border border-green-500">
            <Grid />
        </div>
    }
}

#[derive(Debug, Clone)]
struct SelectedTile(RwSignal<String>);

#[component]
fn TileOverview() -> impl IntoView {
    let game_state = use_context::<GameState>().expect("couldn't get context");

    let tile_info = Memo::new(move |_| {
        let selected_tile =
            use_context::<SelectedTile>().expect("failed to get selected_tile context");

        game_state
            .tiles
            .get()
            .iter()
            .find(|tile| tile.name == selected_tile.0.get())
            .cloned()
            .unwrap_or_default()
    });

    let buy_tile = move |_| {
        if game_state.cash.get() >= tile_info.get().price {
            *game_state.cash.write() -= tile_info.get().price;
            tile_info.get().owned.set(true);
        }
    };

    view! {
        <div class="flex flex-col h-full">
            <div class="p-6 border-b border-gray-700">
                <div class="text-2xl font-semibold">{move || tile_info.get().name}</div>
            </div>

            <div class="flex-1 p-6 space-y-4 overflow-y-auto">
                <div>
                    <p class="text-gray-400">
                        "Description: "{move || tile_info.get().description}
                    </p>
                    <p class="text-gray-400">
                        "Resources: "{move || format!("{:?}", tile_info.get().resources)}
                    </p>
                </div>
            </div>

            <div class="p-6 border-t border-gray-700 flex justify-between items-center h-18">
                <Show
                    when=move || !tile_info.get().owned.get()
                    fallback=|| {
                        view! { <div class="text-lg">"You already own this tile."</div> }
                    }
                >
                    <div class="text-lg">
                        "Price: $"{move || format!("{:.2}", tile_info.get().price)}
                    </div>
                    <button
                        on:click=buy_tile
                        class="px-6 py-2 border-2 border-white hover:bg-white hover:text-black transition-colors"
                    >
                        "Purchase"
                    </button>
                </Show>
            </div>
        </div>
    }
}

#[component]
fn Grid() -> impl IntoView {
    let tile_size = 50.0;

    let pan_offset = RwSignal::new((0.0, 0.0));
    let is_dragging = RwSignal::new(false);
    let drag_start = RwSignal::new((0.0, 0.0));
    let pan_start = RwSignal::new((0.0, 0.0));

    let on_mouse_down = move |e: MouseEvent| {
        e.prevent_default();
        if e.button() == 2 {
            is_dragging.set(true);
            drag_start.set((e.client_x() as f64, e.client_y() as f64));
            pan_start.set(pan_offset.get());
        }
    };

    let on_mouse_move = move |e: MouseEvent| {
        if is_dragging.get() {
            e.prevent_default();
            let (start_x, start_y) = drag_start.get();
            let (pan_x, pan_y) = pan_start.get();

            let dx = e.client_x() as f64 - start_x;
            let dy = e.client_y() as f64 - start_y;

            pan_offset.set((pan_x + dx, pan_y + dy));
        }
    };

    let on_mouse_up = move |e: MouseEvent| {
        e.prevent_default();
        is_dragging.set(false);
    };

    let on_mouse_leave = move |e: MouseEvent| {
        e.prevent_default();
        is_dragging.set(false);
    };

    let game_state = use_context::<GameState>().expect("cannot get GameState from context");
    let tiles = game_state.tiles.get_untracked();

    view! {
        <svg
            class="border border-red-700"
            width="100%"
            height="100%"
            class:cursor-grabbing=move || is_dragging.get()
            on:mousedown=on_mouse_down
            on:mousemove=on_mouse_move
            on:mouseup=on_mouse_up
            on:mouseleave=on_mouse_leave
            on:contextmenu=move |e: MouseEvent| {
                e.prevent_default();
            }
        >
            <g transform=move || {
                let (x, y) = pan_offset.get();
                format!("translate({}, {})", x, y)
            }>
                {tiles
                    .into_iter()
                    .map(|tile| {
                        view! {
                            <Tile row=tile.row col=tile.col name=tile.name tile_size is_dragging />
                        }
                    })
                    .collect_view()}
            </g>
        </svg>
    }
}

#[component]
fn Tile(
    row: u32,
    col: u32,
    name: String,
    tile_size: f64,
    is_dragging: RwSignal<bool>,
) -> impl IntoView {
    let width = tile_size * 2.0;
    let height = (3.0_f64).sqrt() * tile_size;

    let x = col as f64 * width * 0.75 + tile_size;
    let offset = if col % 2 == 1 { height / 2. } else { 0. };
    let y = row as f64 * height + height / 2. + offset;

    let points = points(x + 1., y + 1., tile_size);

    let selected_tile =
        use_context::<SelectedTile>().expect("failed to get SelectedTile from context");

    let name_clone = name.clone();
    let is_selected = move || selected_tile.0.get().eq(&name_clone);

    let on_click = move |e: MouseEvent| {
        if !is_dragging.get() && e.button() == 0 {
            e.prevent_default();
            selected_tile.0.set(name.clone());
        }
    };

    view! {
        <polygon
            points=points
            class=move || {
                if is_selected() {
                    "fill-green-700 stroke-gray-500 stroke-2 cursor-pointer hover:fill-green-600"
                } else {
                    "fill-gray-700 stroke-gray-500 stroke-2 cursor-pointer hover:fill-gray-600"
                }
            }
            on:click=on_click
        />
    }
}

// TODO: precalculate points and just offset
fn points(center_x: f64, center_y: f64, size: f64) -> String {
    let points: Vec<(f64, f64)> = (0..6)
        .map(|i| {
            let angle_deg = 60.0 * i as f64;
            let angle_rad = std::f64::consts::PI / 180.0 * angle_deg;
            (
                center_x + size * angle_rad.cos(),
                center_y + size * angle_rad.sin(),
            )
        })
        .collect();

    points
        .iter()
        .map(|(x, y)| format!("{:.2},{:.2}", x, y))
        .collect::<Vec<_>>()
        .join(" ")
}
