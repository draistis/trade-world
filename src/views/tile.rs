use crate::components::header::Header;
use crate::entities::GameState;
use crate::entities::Tile;
use leptos::prelude::*;
use leptos::Params;
use leptos_router::hooks::use_params;
use leptos_router::params::Params;

use crate::components::{
    Accordion, AccordionContent, AccordionItem, AccordionTrigger, AccordionType,
};
use crate::components::{Tabs, TabsContent, TabsList, TabsTrigger};

#[derive(Params, PartialEq)]
pub struct TileParams {
    pub id: Option<String>,
}

#[derive(Clone)]
pub struct TileContext(Memo<Option<Tile>>);

#[component]
pub fn TilePage() -> impl IntoView {
    let game_state = use_context::<GameState>().expect("failed to get game state");
    let params = use_params::<TileParams>();
    let tile_id = params
        .read_untracked()
        .as_ref()
        .ok()
        .and_then(|params| params.id.clone())
        .unwrap_or_default();
    let tile = Memo::new(move |_| {
        game_state
            .tiles
            .get()
            .iter()
            .find(|tile| tile.id == tile_id)
            .cloned()
    });

    provide_context(TileContext(tile));

    view! {
        <div class="flex flex-col h-screen overflow-hidden">
            <Header />
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
                <div class="flex flex-1">"inventory and transport/market"</div>
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
                <li>
                    {move || {
                        if let Some(tile) = tile.get() {
                            tile.tile_state.buildings.housing.cheap.get()
                        } else {
                            0
                        }
                    }}
                </li>
            </ul>
        </div>
    }
}

#[component]
pub fn BuildingsTab() -> impl IntoView {
    let tile = use_context::<TileContext>().expect("tile context").0;

    let build_cheap = move |_| {
        if let Some(tile) = tile.get() {
            *tile.tile_state.buildings.housing.cheap.write() += 1;
        }
    };
    let build_standard = move |_| {
        if let Some(tile) = tile.get() {
            *tile.tile_state.buildings.housing.standard.write() += 1;
        }
    };
    let build_fancy = move |_| {
        if let Some(tile) = tile.get() {
            *tile.tile_state.buildings.housing.standard.write() += 1;
        }
    };

    let cheap_houses = move || {
        if let Some(tile) = tile.get() {
            tile.tile_state.buildings.housing.cheap.get();
        }
    };
    let standard_houses = move || {
        if let Some(tile) = tile.get() {
            tile.tile_state.buildings.housing.standard.get();
        }
    };
    let fancy_houses = move || {
        if let Some(tile) = tile.get() {
            tile.tile_state.buildings.housing.fancy.get();
        }
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
