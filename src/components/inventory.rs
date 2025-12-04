use leptos::ev::MouseEvent;
use leptos::prelude::*;

use crate::entities::Inventory;
use crate::entities::ITEMS;

#[derive(Copy, Clone)]
pub struct DragState {
    pub dragging: RwSignal<Option<DragInfo>>,
    pub mouse_pos: RwSignal<(i32, i32)>,
}

#[derive(Clone, Debug)]
pub struct DragInfo {
    pub item_id: &'static str,
    pub quantity: u64,
    pub source: RwSignal<Inventory>,
    pub destination: Option<RwSignal<Inventory>>,
    pub to_transfer: u64,
    pub offset: (i32, i32),
}

#[component]
pub fn InventoryContainer(inventory: RwSignal<Inventory>) -> impl IntoView {
    let drag_state = use_context::<DragState>().expect("DragState ctx");
    let is_hovered = RwSignal::new(false);
    let inv_id = inventory.get_untracked().id.clone();

    let stored_items = Memo::new(move |_| {
        let mut items: Vec<_> = inventory.get().items.get().into_iter().collect();
        items.sort_by(|a, b| a.0.cmp(&b.0));
        items
    });

    let show_overlay = move || {
        is_hovered.get()
            && drag_state
                .dragging
                .get()
                .map(|info| info.source.get().id != inv_id)
                .unwrap_or(false)
    };

    view! {
        <div class="flex p-2 w-full h-full flex-col">
            <InventoryCapacityProgress inventory />
            <div
                on:mouseenter=move |_| is_hovered.set(true)
                on:mouseleave=move |_| is_hovered.set(false)
                class="relative flex w-full h-full"
            >
                <InventoryTransferOverlay show_overlay destination_inventory=inventory />
                <For
                    each=move || stored_items.get()
                    key=|item| item.0
                    children=move |item| {
                        let item_id = item.0;
                        let quantity = item.1;
                        view! { <DraggableItem item_id quantity inventory /> }
                    }
                />
            </div>
        </div>
    }
}

#[component]
pub fn DraggableItem(
    item_id: &'static str,
    quantity: u64,
    inventory: RwSignal<Inventory>,
) -> impl IntoView {
    let drag_state = use_context::<DragState>().expect("DragState ctx");
    let item = ITEMS
        .iter()
        .find(|i| i.id == item_id)
        .expect("failed to find item id in ITEMS list");

    let item_qty = move || {
        inventory
            .get()
            .items
            .get()
            .iter()
            .find(|i| i.0 == item_id)
            .unwrap()
            .1
    };
    let on_mouse_down = move |e: MouseEvent| {
        e.prevent_default();
        drag_state.dragging.set(Some(DragInfo {
            item_id,
            quantity: item_qty(),
            source: inventory,
            destination: None,
            to_transfer: 0,
            offset: (e.offset_x(), e.offset_y()),
        }));
        drag_state.mouse_pos.set((e.client_x(), e.client_y()));
    };

    view! {
        <div
            on:mousedown=on_mouse_down
            class=format!(
                "relative p-2 m-1 flex items-center justify-center font-bold text-lg select-none h-16 w-16 text-shadow-lg {}",
                item.color(),
            )
        >
            <span class="pointer-events-none">{item.id}</span>
            <div class="absolute bottom-0 right-0 flex items-center justify-center px-1 bg-black/80 text-white text-xs font-bold pointer-events-none rounded-tl-lg">
                {item_qty}
            </div>
        </div>
    }
}

#[component]
pub fn DraggableItemOverlay() -> impl IntoView {
    let drag_state = use_context::<DragState>().expect("drag state ctx");

    view! {
        <Show when=move || {
            drag_state.dragging.get().is_some()
        }>
            {move || {
                let info = drag_state.dragging.get().unwrap();
                let item = ITEMS
                    .iter()
                    .find(|i| i.id == info.item_id)
                    .expect("item missing from ITEMS");
                let (x, y) = drag_state.mouse_pos.get();
                let (off_x, off_y) = info.offset;
                let item_qty = move || {
                    info.source
                        .get()
                        .items
                        .get()
                        .iter()
                        .find(|i| i.0 == info.item_id)
                        .unwrap_or(&("NAI", 0))
                        .1
                };

                view! {
                    <div
                        class=format!(
                            "fixed flex items-center justify-center font-bold text-lg h-16 w-16 pointer-events-none z-20 {}",
                            item.color(),
                        )
                        style=format!("left: {}px; top: {}px;", x - off_x, y - off_y)
                    >
                        <span>{item.id}</span>
                        <div class="absolute bottom-0 right-0 flex items-center justify-center px-1 bg-black/80 text-white text-xs font-bold rounded-tl-lg">
                            {item_qty}
                        </div>
                    </div>
                }
            }}
        </Show>
    }
}

#[component]
fn InventoryCapacityProgress(inventory: RwSignal<Inventory>) -> impl IntoView {
    view! {
        <div class="flex w-full h-fit items-center gap-2">
            <span class="text-sm text-gray-300">{move || inventory.get().id}</span>
            {move || {
                let max_weight = inventory.get().max_weight;
                let max_volume = inventory.get().max_volume;
                let weight = inventory.get().weight.get();
                let volume = inventory.get().volume.get();

                view! {
                    <div class="flex items-center gap-2">
                        <progress
                            class="flex-1 h-3 w-12 bg-[#151515] border border-gray-300 [&::-webkit-progress-value]:bg-amber-300 [&::-moz-progress-bar]:bg-amber-300"
                            max=max_weight
                            value=weight
                        />
                        <span>{format!("{} / {}t", weight, max_weight)}</span>
                        <progress
                            class="flex-1 h-3 w-12 bg-[#151515] border border-gray-300 [&::-webkit-progress-value]:bg-amber-300 [&::-moz-progress-bar]:bg-amber-300"
                            max=max_volume
                            value=volume
                        />
                        <span>{format!("{:.1} / {}m", volume, max_volume)}<sup>3</sup></span>
                    </div>
                }
            }}
        </div>
    }
}

#[component]
fn InventoryTransferOverlay<T>(
    show_overlay: T,
    destination_inventory: RwSignal<Inventory>,
) -> impl IntoView
where
    T: Fn() -> bool + std::marker::Sync + std::marker::Send + 'static,
{
    let drag_state = use_context::<DragState>().expect("DragState context missing");

    let qty_can_transfer = Memo::new(move |_| {
        let drag_info = drag_state
            .dragging
            .get()
            .expect("DragState should be Some when dragging over an inventory overlay");
        let available_items = drag_info.quantity;
        let max_items = destination_inventory.with(|dest| dest.fits_max_items(drag_info.item_id));

        available_items.min(max_items)
    });

    let reset_destination = move |_| {
        drag_state.dragging.update(|drag_info_opt| {
            if let Some(drag_info) = drag_info_opt {
                *drag_info = DragInfo {
                    destination: None,
                    to_transfer: 0,
                    ..drag_info.clone()
                }
            }
        })
    };
    let set_destination_with_qty = move |qty: u64| {
        drag_state.dragging.update(|drag_info_opt| {
            if let Some(drag_info) = drag_info_opt {
                *drag_info = DragInfo {
                    destination: Some(destination_inventory),
                    to_transfer: qty,
                    ..drag_info.clone()
                }
            }
        })
    };

    let transfer_options = [(1, "1"), (10, "10"), (100, "100"), (1000, "1k")];

    view! {
        <Show when=show_overlay>
            <div
                on:mouseleave=reset_destination
                class="absolute inset-0 z-20 flex bg-white/30 text-white text-5xl"
            >
                <For
                    each=move || {
                        transfer_options
                            .iter()
                            .copied()
                            .filter(|&(qty, _)| qty_can_transfer.get() >= qty)
                            .collect::<Vec<_>>()
                    }
                    key=|&(qty, _)| qty
                    children=move |(qty, label)| {
                        view! {
                            <div
                                on:mouseenter=move |_| { set_destination_with_qty(qty) }
                                class="flex flex-1 items-center justify-center border-r border-gray-300 transition-all hover:text-amber-400"
                            >
                                {label}
                            </div>
                        }
                    }
                />
                {
                    let show_max = move || {
                        qty_can_transfer.get() > 0
                            && drag_state
                                .dragging
                                .get()
                                .is_some_and(|info| info.quantity > qty_can_transfer.get())
                    };
                    let show_full = move || { qty_can_transfer.get() <= 0 };
                    view! {
                        <Show when=move || { !show_max() && !show_full() }>
                            <div
                                on:mouseenter=move |_| {
                                    if let Some(info) = drag_state.dragging.get() {
                                        set_destination_with_qty(info.quantity);
                                    }
                                }
                                class="flex flex-1 h-full items-center justify-center hover:text-amber-300"
                            >
                                <span>"ALL"</span>
                            </div>
                        </Show>
                        <Show when=show_max>
                            <div
                                on:mouseenter=move |_| set_destination_with_qty(
                                    qty_can_transfer.get(),
                                )
                                class="flex flex-1 h-full items-center justify-center hover:text-amber-300"
                            >
                                <span>"MAX"</span>
                            </div>
                        </Show>
                        <Show when=show_full>
                            <div
                                on:mouseenter=reset_destination
                                class="flex flex-1 h-full items-center justify-center hover:text-rose-300"
                            >
                                <span>"INVENTORY CAPACITY EXCEEDED"</span>
                            </div>
                        </Show>
                    }
                }

            </div>
        </Show>
    }
}
