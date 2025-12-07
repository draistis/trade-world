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
                        <TabsContent value="workers">"workers tab content"</TabsContent>
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
    let tile = use_context::<TileContext>().expect("tile context").0;

    let build_waterpump = move |_| {
        tile.get().map(|tile| {
            tile.tile_state
                .buildings
                .production
                .water_pump
                .update(|h| *h += 1)
        });
    };
    let build_sawmill = move |_| {
        tile.get().map(|tile| {
            tile.tile_state
                .buildings
                .production
                .sawmill
                .update(|h| *h += 1)
        });
    };
    let build_warehouse = move |_| {
        tile.get().map(|tile| {
            tile.tile_state
                .buildings
                .production
                .warehouse
                .update(|h| *h += 1)
        });
    };
    let water_pumps = move || {
        tile.get()
            .map(|tile| tile.tile_state.buildings.production.water_pump.get())
            .unwrap_or(0)
    };
    let sawmills = move || {
        tile.get()
            .map(|tile| tile.tile_state.buildings.production.sawmill.get())
            .unwrap_or(0)
    };
    let warehouses = move || {
        tile.get()
            .map(|tile| tile.tile_state.buildings.production.warehouse.get())
            .unwrap_or(0)
    };

    let build_cheap = move |_| {
        tile.get()
            .map(|tile| tile.tile_state.buildings.housing.cheap.update(|h| *h += 1));
    };
    let build_standard = move |_| {
        tile.get().map(|tile| {
            tile.tile_state
                .buildings
                .housing
                .standard
                .update(|h| *h += 1)
        });
    };
    let build_fancy = move |_| {
        tile.get()
            .map(|tile| tile.tile_state.buildings.housing.fancy.update(|h| *h += 1));
    };

    let cheap_houses = move || {
        tile.get()
            .map(|tile| tile.tile_state.buildings.housing.cheap.get())
            .unwrap_or(0)
    };
    let standard_houses = move || {
        tile.get()
            .map(|tile| tile.tile_state.buildings.housing.standard.get())
            .unwrap_or(0)
    };
    let fancy_houses = move || {
        tile.get()
            .map(|tile| tile.tile_state.buildings.housing.fancy.get())
            .unwrap_or(0)
    };

    view! {
        <Accordion of_type=AccordionType::Multiple collapsible=true>
            <AccordionItem value="production-buildings">
                <AccordionTrigger>"Production Buildings"</AccordionTrigger>
                <AccordionContent>
                    <div class="w-full flex justify-between pb-2">
                        <div class="flex flex-col">
                            <div class="text-md font-semibold">
                                {move || format!("Water pump ({})", water_pumps())}
                            </div>
                            <div class="text-sm">"Extracts liquid water."</div>
                        </div>
                        <button
                            on:click=build_waterpump
                            class="border font-bold hover:bg-destructive-dim/30 border-destructive-dim hover:cursor-pointer my-1 py-2 px-4"
                        >
                            "BUILD"
                        </button>
                    </div>
                    <div class="w-full flex justify-between pb-2">
                        <div class="flex flex-col">
                            <div class="text-md font-semibold">
                                {move || format!("Sawmill ({})", sawmills())}
                            </div>
                            <div class="text-sm">"Turns logs into boards and woodchips."</div>
                        </div>
                        <button
                            on:click=build_sawmill
                            class="border font-bold hover:cursor-pointer hover:bg-destructive-dim/30 my-1 border-destructive-dim py-2 px-4"
                        >
                            "BUILD"
                        </button>
                    </div>
                    <div class="w-full flex justify-between">
                        <div class="flex flex-col">
                            <div class="text-md font-semibold">
                                {move || format!("Warehouse ({})", warehouses())}
                            </div>
                            <div class="text-sm">"Warehouse for storing stuff."</div>
                        </div>
                        <button
                            on:click=build_warehouse
                            class="border font-bold hover:cursor-pointer my-1 border-destructive-dim hover:bg-destructive-dim/30 py-2 px-4"
                        >
                            "BUILD"
                        </button>
                    </div>
                </AccordionContent>
            </AccordionItem>
            <AccordionItem value="worker-housing">
                <AccordionTrigger>"Worker Housing"</AccordionTrigger>
                <AccordionContent>
                    <div class="w-full flex justify-between pb-2">
                        <div class="flex flex-col">
                            <div class="text-md font-semibold">
                                {move || format!("Cheap housing ({})", cheap_houses())}
                            </div>
                            <div class="text-sm">"Can house up to 10 basic workers."</div>
                        </div>
                        <button
                            on:click=build_cheap
                            class="border font-bold hover:cursor-pointer my-1 border-destructive-dim hover:bg-destructive-dim/30 py-2 px-4"
                        >
                            "BUILD"
                        </button>
                    </div>
                    <div class="w-full flex justify-between pb-2">
                        <div class="flex flex-col">
                            <div class="text-md font-semibold">
                                {move || format!("Standard housing ({})", standard_houses())}
                            </div>
                            <div class="text-sm">"Can house up to 6 advanced workers."</div>
                        </div>
                        <button
                            on:click=build_standard
                            class="border font-bold hover:cursor-pointer my-1 border-destructive-dim hover:bg-destructive-dim/30 py-2 px-4"
                        >
                            "BUILD"
                        </button>
                    </div>
                    <div class="w-full flex justify-between">
                        <div class="flex flex-col">
                            <div class="text-md font-semibold">
                                {move || format!("Fancy housing ({})", fancy_houses())}
                            </div>
                            <div class="text-sm">"Can house up to 3 expert workers."</div>
                        </div>
                        <button
                            on:click=build_fancy
                            class="border font-bold hover:cursor-pointer my-1 border-destructive-dim hover:bg-destructive-dim/30 py-2 px-4"
                        >
                            "BUILD"
                        </button>
                    </div>
                </AccordionContent>
            </AccordionItem>
        </Accordion>
    }
}
