use crate::components::header::Header;
use crate::components::inventory::DraggableItemOverlay;
use crate::components::InventoryContainer;
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

    window_event_listener(mouseup, move |e: MouseEvent| {
        if let Some(drag_info) = drag_state.dragging.get() {
            if let Some(destination) = drag_info.destination {
                drag_info
                    .source
                    .update(|inv| inv.remove_item(drag_info.item_id, 1));

                destination.update(|inv| inv.add_item(drag_info.item_id, 1));
            }
        }
        drag_state.dragging.set(None);
    });

    view! {
        <div class="flex flex-col h-screen overflow-hidden">
            <Header />
            <DraggableItemOverlay />
            <div class="flex flex-1">
                <div class="flex border-r border-gray-500 w-1/2 lg:w-1/3">
                    <Tabs default_value="overview">
                        <TabsList>
                            <TabsTrigger value="overview">Overview</TabsTrigger>
                            <TabsTrigger value="buildings">Buildings</TabsTrigger>
                            <TabsTrigger value="workers">Workers</TabsTrigger>
                        </TabsList>
                        <TabsContent value="overview">
                            <OverviewTab />
                        </TabsContent>
                        <TabsContent value="buildings">
                            <BuildingsTab />
                        </TabsContent>
                        <TabsContent value="workers">"workers tab content"</TabsContent>
                    </Tabs>
                </div>
                <div class="flex flex-1">
                    <div class="flex flex-col w-full h-full">
                        <div class="flex h-1/3 border-b border-gray-500 overflow-hidden">
                            {move || {
                                let inventory = tile.get().unwrap().tile_state.inventory;
                                view! { <InventoryContainer inventory=inventory /> }
                            }}
                        </div>
                        <div class="flex h-1/3 border-b border-gray-500 overflow-hidden">
                            <InventoryContainer inventory=inv2 />
                        </div>
                        <div class="flex overflow-hidden h-1/3">
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
    let tile = use_context::<TileContext>().expect("tile context").0;

    let build_cheap = move |_| {
        *tile
            .get()
            .unwrap()
            .tile_state
            .buildings
            .housing
            .cheap
            .write() += 1;
    };
    let build_standard = move |_| {
        *tile
            .get()
            .unwrap()
            .tile_state
            .buildings
            .housing
            .standard
            .write() += 1;
    };
    let build_fancy = move |_| {
        *tile
            .get()
            .unwrap()
            .tile_state
            .buildings
            .housing
            .standard
            .write() += 1;
    };

    let cheap_houses = move || {
        tile.get().unwrap().tile_state.buildings.housing.cheap.get();
    };
    let standard_houses = move || {
        tile.get()
            .unwrap()
            .tile_state
            .buildings
            .housing
            .standard
            .get();
    };
    let fancy_houses = move || {
        tile.get().unwrap().tile_state.buildings.housing.fancy.get();
    };

    view! {
        <Accordion of_type=AccordionType::Multiple collapsible=true>
            <AccordionItem value="production-buildings">
                <AccordionTrigger>"Production Buildings"</AccordionTrigger>
                <AccordionContent>
                    <div class="w-full flex justify-between pb-2">
                        <div class="flex flex-col">
                            <div class="text-md font-semibold">"Water Pump"</div>
                            <div class="text-sm">"Extracts liquid water."</div>
                        </div>
                        <button
                            on:click=move |_| {
                                *tile
                                    .get()
                                    .unwrap()
                                    .tile_state
                                    .buildings
                                    .production
                                    .water_pump
                                    .write() += 1;
                            }
                            class="border font-bold hover:cursor-pointer my-1 border-indigo-400 py-2 px-4"
                        >
                            "Build"
                        </button>
                    </div>
                    <div class="w-full flex justify-between pb-2">
                        <div class="flex flex-col">
                            <div class="text-md font-semibold">"Sawmill"</div>
                            <div class="text-sm">"Turns logs into boards and woodchips."</div>
                        </div>
                        <button
                            on:click=move |_| {
                                *tile.get().unwrap().tile_state.buildings.production.sawmill.write()
                                    += 1;
                            }
                            class="border font-bold hover:cursor-pointer my-1 border-indigo-400 py-2 px-4"
                        >
                            "Build"
                        </button>
                    </div>
                    <div class="w-full flex justify-between">
                        <div class="flex flex-col">
                            <div class="text-md font-semibold">"Warehouse"</div>
                            <div class="text-sm">"Warehouse for storing stuff."</div>
                        </div>
                        <button
                            on:click=move |_| {
                                *tile
                                    .get()
                                    .unwrap()
                                    .tile_state
                                    .buildings
                                    .production
                                    .warehouse
                                    .write() += 1;
                            }
                            class="border font-bold hover:cursor-pointer my-1 border-indigo-400 py-2 px-4"
                        >
                            "Build"
                        </button>
                    </div>
                </AccordionContent>
            </AccordionItem>
            <AccordionItem value="worker-housing">
                <AccordionTrigger>"Worker Housing"</AccordionTrigger>
                <AccordionContent>
                    <div class="w-full flex justify-between pb-2">
                        <div class="flex flex-col">
                            <div class="text-md font-semibold">"Basic housing"</div>
                            <div class="text-sm">"Can house up to 10 basic workers."</div>
                        </div>
                        <button
                            on:click=build_cheap
                            class="border font-bold hover:cursor-pointer my-1 border-indigo-400 py-2 px-4"
                        >
                            "Build"
                        </button>
                    </div>
                    <div class="w-full flex justify-between pb-2">
                        <div class="flex flex-col">
                            <div class="text-md font-semibold">"Advanced housing"</div>
                            <div class="text-sm">"Can house up to 6 advanced workers."</div>
                        </div>
                        <button
                            on:click=build_standard
                            class="border font-bold hover:cursor-pointer my-1 border-indigo-400 py-2 px-4"
                        >
                            "Build"
                        </button>
                    </div>
                    <div class="w-full flex justify-between">
                        <div class="flex flex-col">
                            <div class="text-md font-semibold">"Expert housing"</div>
                            <div class="text-sm">"Can house up to 3 expert workers."</div>
                        </div>
                        <button
                            on:click=build_fancy
                            class="border font-bold hover:cursor-pointer my-1 border-indigo-400 py-2 px-4"
                        >
                            "Build"
                        </button>
                    </div>
                </AccordionContent>
            </AccordionItem>
        </Accordion>
    }
}
