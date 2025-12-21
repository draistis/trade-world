use crate::{components::header::Header, entities::GameState};
use leptos::{ev::MouseEvent, prelude::*};
use leptos_router::components::A;

#[component]
pub fn TileMapPage() -> impl IntoView {
    provide_context::<SelectedTile>(SelectedTile(RwSignal::new("STR-1001")));

    view! {
        <div class="h-screen flex flex-col">
            <Header />
            <div class="flex flex-1 overflow-hidden">
                <div class="flex-1 flex items-center justify-center border-r border-primary-border">
                    <TileMap />
                </div>
                <div class="w-5/12 flex flex-col">
                    <TileOverview />
                </div>
            </div>
        </div>
    }
}

#[component]
fn TileMap() -> impl IntoView {
    view! {
        <div class="w-full h-full overflow-hidden p-2 ml-0">
            <Grid />
        </div>
    }
}

#[derive(Debug, Clone)]
struct SelectedTile(RwSignal<&'static str>);

#[component]
fn TileOverview() -> impl IntoView {
    let game_state = use_context::<GameState>().expect("couldn't get context");

    let tile_info = Memo::new(move |_| {
        let selected_tile =
            use_context::<SelectedTile>().expect("failed to get selected_tile context");

        game_state
            .tiles
            .iter()
            .find(|&tile| tile.id == selected_tile.0.get())
            .copied()
            .unwrap_or_default()
    });

    let buy_tile = move |_| {
        let tile_info = tile_info.get();
        if game_state.cash.get() >= tile_info.price {
            *game_state.cash.write() -= tile_info.price;
            tile_info.is_owned.set(true);
        }
    };

    view! {
        <div class="flex flex-col h-full">
            <div class="flex p-4 border-b border-primary-border">
                <div class="text-4xl font-semibold">{move || tile_info.get().id}</div>
            </div>

            <div class="flex-1 flex p-4 space-y-4 overflow-y-auto border-b border-primary-border">
                <div class="text-2xl">
                    <p class="">"Description: "{move || tile_info.get().description}</p>
                    <p class="">
                        "Resources: "{move || format!("{:?}", tile_info.get().resources)}
                    </p>
                </div>
            </div>

            <div class="p-6 flex justify-between items-center h-20">
                <Show
                    when=move || !tile_info.get().is_owned.get()
                    fallback=move || {
                        view! {
                            <div class="text-3xl font-semibold">"Purchased"</div>
                            <A
                                href=format!("/tile/{}", tile_info.get().id)
                                attr:class="px-6 py-2 border-2 font-bold text-xl hover:bg-hover-btn transition-colors"
                            >
                                "MANAGE TILE"
                            </A>
                        }
                    }
                >
                    <div class="text-3xl font-semibold">
                        "Price: $"{move || format!("{:.2}", tile_info.get().price)}
                    </div>
                    <button
                        on:click=buy_tile
                        class="px-6 py-2 hover:cursor-pointer border-2 font-bold text-xl bg-destructive hover:bg-destructive-hover transition-colors"
                    >
                        "PURCHASE"
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
    let tiles = game_state.tiles;

    view! {
        <svg
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
                            <Tile row=tile.row col=tile.col name=tile.id tile_size is_dragging />
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
    name: &'static str,
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

    let is_selected = move || selected_tile.0.get().eq(name);

    let on_click = move |e: MouseEvent| {
        if !is_dragging.get() && e.button() == 0 {
            e.prevent_default();
            selected_tile.0.set(name);
        }
    };

    view! {
        <polygon
            points=points
            class="stroke-primary-border stroke-2 cursor-pointer"
            class=(["fill-tertiary-bg", "hover:fill-secondary-bg"], move || !is_selected())
            class=(["fill-highlight", "hover:fill-highlight-hover"], is_selected)
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
