use crate::{components::header::Header, entities::GameState};
use leptos::{attr::start, ev::MouseEvent, prelude::*, svg::Svg};

#[component]
pub fn TileMapPage() -> impl IntoView {
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
    let selected_tile = RwSignal::new(Option::<(i32, i32)>::None);

    let rows = 50;
    let cols = 50;

    view! {
        <div class="w-full h-full overflow-hidden p-20 ml-0 border border-green-500">
            <Grid rows cols selected_tile />
        </div>
    }
}

#[component]
fn TileOverview() -> impl IntoView {
    let game_state = use_context::<GameState>().expect("couldn't get context");

    let buy_tile = move || {
        *game_state.cash.write() -= 500.;
    };

    view! {
        <div class="flex flex-col h-full">
            <div class="p-6 border-b border-gray-700">
                <div class="text-2xl font-semibold">"Tile Name"</div>
            </div>

            <div class="flex-1 p-6 space-y-4 overflow-y-auto">
                <div>
                    <p class="text-gray-400">"Description: "</p>
                    <p class="text-gray-400">"Resources: "</p>
                </div>
            </div>

            <div class="p-6 border-t border-gray-700 flex justify-between items-center">
                <div class="text-lg">"Price: $"</div>
                <button
                    on:click=move |_| buy_tile()
                    class="px-6 py-2 border-2 border-white hover:bg-white hover:text-black transition-colors"
                >
                    "Purchase"
                </button>
            </div>
        </div>
    }
}

#[component]
fn Grid(rows: i32, cols: i32, selected_tile: RwSignal<Option<(i32, i32)>>) -> impl IntoView {
    let tile_size = 50.0;
    let tile_width = tile_size * 2.;
    let tile_height = (3.0_f32).sqrt() * tile_size;

    // let view_width = (cols as f32 * tile_width * 0.75 + tile_width * 0.25) as i32;
    // let view_height = (rows as f32 * tile_height + tile_height * 0.5) as i32;

    let pan_offset = RwSignal::new((0.0, 0.0));
    let is_dragging = RwSignal::new(false);
    let drag_start = RwSignal::new((0.0, 0.0));
    let pan_start = RwSignal::new((0.0, 0.0));

    let svg_ref = NodeRef::<Svg>::new();

    let on_mouse_down = move |e: MouseEvent| {
        e.prevent_default();
        e.stop_immediate_propagation();
        is_dragging.set(true);
        drag_start.set((e.client_x() as f64, e.client_y() as f64));
        pan_start.set(pan_offset.get());
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
        e.stop_immediate_propagation();
        is_dragging.set(false);
    };

    let on_mouse_leave = move |e: MouseEvent| {
        is_dragging.set(false);
    };

    let tiles: Vec<_> = (0..rows)
        .flat_map(|row| (0..cols).map(move |col| (row, col)))
        .collect();

    view! {
        <svg
            node_ref=svg_ref
            class="border border-red-700"
            width="100%"
            height="100%"
            class:cursor-grabbing=move || is_dragging.get()
            on:mousedown=on_mouse_down
            on:mousemove=on_mouse_move
            on:mouseup=on_mouse_up
            on:mouseleave=on_mouse_leave
        >
            <g transform=move || {
                let (x, y) = pan_offset.get();
                format!("translate({}, {})", x, y)
            }>
                {tiles
                    .into_iter()
                    .map(|(row, col)| {
                        view! { <Tile row col tile_size selected_tile is_dragging /> }
                    })
                    .collect_view()}
            </g>
        </svg>
    }
}

#[component]
fn Tile(
    row: i32,
    col: i32,
    tile_size: f32,
    selected_tile: RwSignal<Option<(i32, i32)>>,
    is_dragging: RwSignal<bool>,
) -> impl IntoView {
    let width = tile_size * 2.0;
    let height = (3.0_f32).sqrt() * tile_size;

    let x = col as f32 * width * 0.75 + tile_size;
    let offset = if col % 2 == 1 { height / 2. } else { 0. };
    let y = row as f32 * height + height / 2. + offset;

    let points = points(x, y, tile_size);

    let is_selected = move || {
        selected_tile
            .get()
            .map(|(r, c)| r == row && c == col)
            .unwrap_or(false)
    };

    let on_click = move |e: MouseEvent| {
        if !is_dragging.get() {
            e.stop_propagation();
            selected_tile.set(Some((row, col)));
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

fn points(center_x: f32, center_y: f32, size: f32) -> String {
    let points: Vec<(f32, f32)> = (0..6)
        .map(|i| {
            let angle_deg = 60.0 * i as f32;
            let angle_rad = std::f32::consts::PI / 180.0 * angle_deg;
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
