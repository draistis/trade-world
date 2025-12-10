use crate::components::header::Header;
use crate::components::inventory::DraggableItemOverlay;
use crate::components::InventoryContainer;
use crate::entities::tile::HousingType;
use crate::entities::tile::ProductionBuildingType;
use crate::entities::tile::WorkerType;
use crate::entities::GameState;
use crate::entities::Inventory;
use crate::entities::Tile;
use leptos::ev::mousemove;
use leptos::ev::mouseup;
use leptos::ev::MouseEvent;
use leptos::prelude::*;
use leptos::Params;
use leptos_router::hooks::use_params;
use leptos_router::params::Params;

use crate::components::inventory::DragState;
use crate::components::{
    Accordion, AccordionContent, AccordionItem, AccordionTrigger, AccordionType,
};
use crate::components::{Tabs, TabsContent, TabsList, TabsTrigger};

#[derive(Params, PartialEq, Clone)]
pub struct TileParams {
    pub id: Option<String>,
}

#[derive(Clone)]
pub struct TileContext(Memo<Option<Tile>>);

#[component]
pub fn TilePage() -> impl IntoView {
    let game_state = use_context::<GameState>().expect("failed to get game state");
    let drag_state = use_context::<DragState>().expect("context");
    let params = use_params::<TileParams>();
    let tile_id = move || params.get().unwrap().id.unwrap();

    let inv2 = RwSignal::new(Inventory::empty());
    let inv3 = RwSignal::new(Inventory::one_item());

    let tile = Memo::new(move |_| {
        game_state
            .tiles
            .get()
            .iter()
            .find(|tile| tile.id == tile_id())
            .cloned()
    });

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
                                let inventory = tile.get().unwrap().tile_state.inventory;
                                view! { <InventoryContainer inventory=inventory /> }
                            }}
                        </div>
                        <div class="flex flex-1 border-b border-primary-border overflow-hidden">
                            <InventoryContainer inventory=inv2 />
                        </div>
                        <div class="flex flex-1 overflow-hidden h-1/3">
                            <InventoryContainer inventory=inv3 />
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn OverviewTab() -> impl IntoView {
    let tile = use_context::<TileContext>().expect("context").0;

    view! {
        <div class="flex flex-col">
            <ul>
                <li>{move || { tile.get().unwrap().tile_state.buildings.housing.cheap.get() }}</li>
            </ul>
        </div>
    }
}

#[component]
pub fn BuildingsTab() -> impl IntoView {
    let game_state = use_context::<GameState>().expect("game state context");
    let tile = use_context::<TileContext>().expect("tile context").0;
    let money = game_state.cash;

    view! {
        <Accordion of_type=AccordionType::Multiple collapsible=true>
            <AccordionItem value="production-buildings">
                <AccordionTrigger>"Production Buildings"</AccordionTrigger>
                <AccordionContent>
                    <For
                        each=move || ProductionBuildingType::all()
                        key=|prod_type| *prod_type
                        children=move |production_type: ProductionBuildingType| {
                            view! {
                                <div class="w-full flex justify-between pb-2">
                                    <div class="flex flex-col">
                                        <div class="text-md font-semibold">
                                            {move || {
                                                format!(
                                                    "Water pump ({})",
                                                    tile
                                                        .get()
                                                        .unwrap()
                                                        .owned_production_buildings(production_type),
                                                )
                                            }}
                                        </div>
                                        <div class="text-sm">"Extracts liquid water."</div>
                                    </div>
                                    <button
                                        on:click=move |_| {
                                            tile.get().unwrap().build_production(production_type, money)
                                        }
                                        class="border font-bold hover:bg-destructive-dim/30 border-destructive-dim hover:cursor-pointer my-1 py-2 px-4"
                                    >
                                        "BUILD"
                                    </button>
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
                            view! {
                                <div class="w-full flex justify-between pb-2">
                                    <div class="flex flex-col">
                                        <div class="text-md font-semibold">
                                            {move || {
                                                format!(
                                                    "Cheap housing ({})",
                                                    tile.get().unwrap().owned_housing(housing_type),
                                                )
                                            }}
                                        </div>
                                        <div class="text-sm">
                                            "Can house up to 10 basic workers."
                                        </div>
                                    </div>
                                    <button
                                        on:click=move |_| {
                                            tile.get().unwrap().build_housing(housing_type, money)
                                        }
                                        class="border font-bold hover:cursor-pointer my-1 border-destructive-dim hover:bg-destructive-dim/30 py-2 px-4"
                                    >
                                        "BUILD"
                                    </button>
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
    let game_state = use_context::<GameState>().expect("Game state context");
    let tile = use_context::<TileContext>()
        .expect("context of tile type")
        .0;
    let money = game_state.cash;
    let error_message = RwSignal::new(None);

    view! {
        <Accordion of_type=AccordionType::Multiple collapsible=true>
            <AccordionItem value="worker-hire">
                <AccordionTrigger>"Workers"</AccordionTrigger>
                <AccordionContent>
                    <For
                        each=move || WorkerType::all()
                        key=|worker_type| *worker_type
                        children=move |worker_type: WorkerType| {
                            view! {
                                <div class="flex flex-1 justify-between items-center pb-2">
                                    <div class="flex flex-col">
                                        <div class="text-md font-semibold">"Basic workers"</div>
                                        <div class="text-sm">
                                            {move || {
                                                let tile = tile.get().unwrap();
                                                format!(
                                                    "Capacity {}/{}",
                                                    tile.hired_workers(worker_type),
                                                    tile.available_workers(worker_type),
                                                )
                                            }}
                                        </div>
                                    </div>
                                    <button
                                        class="border font-bold hover:cursor-pointer my-1 border-destructive-dim hover:bg-destructive-dim/30 py-2 px-4"
                                        on:click=move |_| {
                                            error_message
                                                .set(
                                                    tile.get().unwrap().hire_worker(worker_type, money).err(),
                                                );
                                        }
                                    >
                                        "HIRE"
                                    </button>
                                </div>
                            }
                        }
                    />
                </AccordionContent>
            </AccordionItem>
        </Accordion>
    }
}
