use leptos::ev::{mousemove, mouseup, MouseEvent};
use leptos::prelude::*;
use leptos::Params;
use leptos_router::hooks::use_params;
use leptos_router::params::Params;

use crate::components::{
    Accordion, AccordionContent, AccordionItem, AccordionTrigger, AccordionType,
};
use crate::components::{DragState, DraggableItemOverlay, Header, InventoryContainer};
use crate::components::{Tabs, TabsContent, TabsList, TabsTrigger};
use crate::entities::{GameState, HousingType, Inventory, ProductionType, Tile, WorkerType};

fn use_money() -> RwSignal<f64> {
    use_context::<GameState>()
        .expect("GameState context not found.")
        .cash
}
fn use_tile() -> RwSignal<Tile> {
    use_context::<TileContext>()
        .expect("TileContext not found.")
        .0
}

#[derive(Params, PartialEq, Clone)]
pub struct TileParams {
    pub id: Option<String>,
}

#[derive(Clone)]
pub struct TileContext(RwSignal<Tile>);

#[component]
pub fn TilePage() -> impl IntoView {
    let game_state = use_context::<GameState>().expect("failed to get game state");
    let drag_state = use_context::<DragState>().expect("context");
    let params = use_params::<TileParams>();
    let tile_id = move || params.get().unwrap().id.unwrap();

    let tile = game_state
        .tiles
        .iter()
        .copied()
        .find(|&tile| tile.get().id == tile_id())
        .expect("Failed to find tile");

    provide_context(TileContext(tile));

    window_event_listener(mousemove, move |e: MouseEvent| {
        if drag_state.dragging.get().is_some() {
            drag_state.mouse_pos.set((e.client_x(), e.client_y()));
        }
    });

    window_event_listener(mouseup, move |_e: MouseEvent| {
        if let Some(drag_info) = drag_state.dragging.get() {
            if let Some(destination) = drag_info.destination {
                drag_info
                    .source
                    .update(|inv| inv.remove_item(drag_info.item_id, drag_info.to_transfer));

                destination.update(|inv| inv.add_item(drag_info.item_id, drag_info.to_transfer));
            }
        }
        drag_state.dragging.set(None);
    });

    view! {
        <div class="flex flex-col h-screen overflow-hidden">
            <Header />
            <DraggableItemOverlay />
            <div class="flex flex-1">
                <div class="flex border-r border-primary-border w-1/2 lg:w-1/3">
                    <Tabs default_value="overview">
                        <TabsList>
                            <TabsTrigger value="overview">"OVERVIEW"</TabsTrigger>
                            <TabsTrigger value="buildings">"BUILDINGS"</TabsTrigger>
                            <TabsTrigger value="workers">"WORKERS"</TabsTrigger>
                        </TabsList>
                        <TabsContent value="overview">
                            <OverviewTab />
                        </TabsContent>
                        <TabsContent value="buildings">
                            <BuildingsTab />
                        </TabsContent>
                        <TabsContent value="workers">
                            <WorkersTab />
                        </TabsContent>
                    </Tabs>
                </div>
                <div class="flex flex-1">
                    <div class="flex flex-col w-full h-full">
                        <div class="flex flex-1 border-b border-primary-border overflow-hidden">
                            {move || {
                                let inventory = tile.get().tile_state.inventory;
                                view! { <InventoryContainer inventory=inventory /> }
                            }}
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn OverviewTab() -> impl IntoView {
    let tile = use_tile();

    view! {
        <div class="flex flex-col">
            <ul>
                <li>{move || { tile.get().tile_state.buildings.housing.cheap.get() }}</li>
            </ul>
        </div>
    }
}

#[component]
pub fn BuildingsTab() -> impl IntoView {
    let tile = use_tile();
    let money = use_money();

    view! {
        <Accordion of_type=AccordionType::Multiple collapsible=true>
            <AccordionItem value="production-buildings">
                <AccordionTrigger>"Production Buildings"</AccordionTrigger>
                <AccordionContent>
                    <For
                        each=move || ProductionType::all()
                        key=|prod_type| *prod_type
                        children=move |production_type: ProductionType| {
                            let details = production_type.details();
                            view! {
                                <div class="w-full flex justify-between pb-2">
                                    <div class="flex flex-col">
                                        <div class="text-md font-semibold">
                                            {move || {
                                                format!(
                                                    "{} ({})",
                                                    details.name,
                                                    tile.get().owned_production_buildings(production_type),
                                                )
                                            }}
                                        </div>
                                        <div class="text-sm">{details.description}</div>
                                    </div>
                                    <div class="space-x-6">
                                        <span class="font-semibold text-lg text-highlight-dim">
                                            {move || format!("${:.2}", details.cost)}
                                        </span>
                                        <button
                                            on:click=move |_| {
                                                if let Err(err) = tile
                                                    .get()
                                                    .build_production(production_type, money, 1)
                                                {
                                                    leptos::logging::log!("{}", err);
                                                }
                                            }
                                            class="border font-bold hover:bg-destructive-dim/30 border-destructive-dim hover:cursor-pointer my-1 py-2 px-4"
                                        >
                                            "BUILD"
                                        </button>
                                    </div>
                                </div>
                            }
                        }
                    />
                </AccordionContent>
            </AccordionItem>
            <AccordionItem value="worker-housing">
                <AccordionTrigger>"Worker Housing"</AccordionTrigger>
                <AccordionContent>
                    <For
                        each=move || HousingType::all()
                        key=|housing_type| *housing_type
                        children=move |housing_type: HousingType| {
                            let details = housing_type.details();
                            view! {
                                <div class="w-full flex justify-between pb-2">
                                    <div class="flex flex-col">
                                        <div class="text-md font-semibold">
                                            {move || {
                                                format!(
                                                    "{} ({})",
                                                    details.name,
                                                    tile.get().owned_housing(housing_type),
                                                )
                                            }}
                                        </div>
                                        <div class="text-sm">{details.description}</div>
                                    </div>
                                    <div class="space-x-6">
                                        <span class="font-semibold text-lg text-highlight-dim">
                                            {move || format!("${:.2}", details.cost)}
                                        </span>
                                        <button
                                            on:click=move |_| {
                                                if let Err(err) = tile
                                                    .get()
                                                    .build_housing(housing_type, money, 1)
                                                {
                                                    leptos::logging::log!("{}",err);
                                                }
                                            }
                                            class="border font-bold hover:bg-destructive-dim/30 border-destructive-dim hover:cursor-pointer my-1 py-2 px-4"
                                        >
                                            "BUILD"
                                        </button>
                                    </div>
                                </div>
                            }
                                .into_any()
                        }
                    />
                </AccordionContent>
            </AccordionItem>
        </Accordion>
    }
}

#[component]
pub fn WorkersTab() -> impl IntoView {
    let money = use_money();
    let tile = use_tile();
    // let error_message = RwSignal::new(None);

    view! {
        <Accordion of_type=AccordionType::Multiple collapsible=true>
            <AccordionItem value="worker-hire">
                <AccordionTrigger>"Workers"</AccordionTrigger>
                <AccordionContent>
                    <For
                        each=move || WorkerType::all()
                        key=|worker_type| *worker_type
                        children=move |worker_type: WorkerType| {
                            let details = worker_type.details();
                            view! {
                                <div class="flex flex-1 justify-between items-center pb-2">
                                    <div class="flex flex-col">
                                        <div class="text-md font-semibold">{details.name}</div>
                                        <div class="text-sm">
                                            {move || {
                                                format!(
                                                    "Capacity {}/{}",
                                                    tile.get().hired_workers(worker_type),
                                                    tile.get().workers_can_accommodate(worker_type),
                                                )
                                            }}
                                        </div>
                                    </div>
                                    <div class="space-x-6">
                                        <span class="font-semibold text-lg text-highlight-dim">
                                            {move || format!("${:.2}", details.cost)}
                                        </span>
                                        <button
                                            on:click=move |_| {
                                                if let Err(err) = tile
                                                    .get()
                                                    .hire_workers(worker_type, money, 1)
                                                {
                                                    leptos::logging::log!("{}",err);
                                                }
                                            }
                                            class="border font-bold hover:bg-destructive-dim/30 border-destructive-dim hover:cursor-pointer my-1 py-2 px-4"
                                        >
                                            "HIRE"
                                        </button>
                                    </div>
                                </div>
                            }
                        }
                    />
                </AccordionContent>
            </AccordionItem>
        </Accordion>
    }
}
