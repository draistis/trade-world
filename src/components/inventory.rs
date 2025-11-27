use leptos::html::Div;
use leptos::prelude::*;
use leptos_use::core::Position;
use leptos_use::{use_draggable_with_options, UseDraggableOptions, UseDraggableReturn};

use crate::entities::Inventory;
use crate::entities::ITEMS;

#[derive(Copy, Clone)]
pub struct DragState {
    pub dragging: RwSignal<Option<DragInfo>>,
    pub mouse_pos: RwSignal<(i32, i32)>,
}

#[derive(Clone)]
pub struct DragInfo {
    pub item_id: &'static str,
    pub quantity: u64,
    pub source_inventory: String,
}

#[component]
pub fn InventoryContainer(inventory: RwSignal<Inventory>) -> impl IntoView {
    let drag_state = use_context::<DragState>().expect("DragState ctx");

    view! {
        <div class="flex p-2">
            <For
                each=move || inventory.get().items.get()
                key=|item| item.0
                children=move |item| {
                    let item_id = item.0;
                    let quantity = item.1;
                    view! { <DraggableItem item_id quantity inventory /> }
                }
            />
        </div>
    }
}

// Items teleport to 0,0 or last drop-off location on click(drag start)
#[component]
pub fn DraggableItem(
    item_id: &'static str,
    quantity: u64,
    inventory: RwSignal<Inventory>,
) -> impl IntoView {
    let drag_state = use_context::<DragState>().expect("DragState ctx");
    let element = NodeRef::<Div>::new();
    let item = ITEMS
        .iter()
        .find(|i| i.id == item_id)
        .expect("failed to find item id in ITEMS list");

    let UseDraggableReturn {
        position,
        is_dragging,
        ..
    } = use_draggable_with_options(
        element,
        UseDraggableOptions::default()
            .on_start(move |_| {
                drag_state.dragging.set(Some(DragInfo {
                    item_id,
                    quantity,
                    source_inventory: inventory.get().id,
                }));
                true
            })
            .on_end(move |_| {
                drag_state.dragging.set(None);
            }),
    );

    view! {
        <div
            node_ref=element
            class=format!(
                "relative p-2 m-1 flex items-center justify-center font-bold text-lg select-none h-16 w-16 text-shadow-lg {}",
                item.color(),
            )
            style=move || {
                if is_dragging.get() {
                    format!(
                        "position: fixed; left: {}px; top: {}px; cursor: grabbing;",
                        position.get().x,
                        position.get().y,
                    )
                } else {
                    String::new()
                }
            }
        >
            <span>{item.id}</span>
            <div class="absolute bottom-0 right-0 flex items-center justify-center px-1 bg-black/80 text-white text-xs font-bold rounded-tl-lg">
                {quantity}
            </div>
        </div>
    }
}
