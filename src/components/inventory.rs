use std::collections::HashMap;

use leptos::ev::MouseEvent;
use leptos::logging::log;
use leptos::prelude::*;

use crate::entities::ITEMS;
use crate::entities::{Inventory, Item};

#[derive(Copy, Clone)]
struct DragState {
    dragging: RwSignal<Option<DragInfo>>,
    mouse_pos: RwSignal<(i32, i32)>,
}

#[derive(Clone)]
struct DragInfo {
    item_id: &'static str,
    quantity: u64,
    source_inventory: String,
}

#[component]
pub fn InventoryContainer(inventory: RwSignal<Inventory>) -> impl IntoView {
    let drag_state = DragState {
        dragging: RwSignal::new(None),
        mouse_pos: RwSignal::new((0, 0)),
    };
    let start_drag = move |_| {
        drag_state.dragging.set(Some(DragInfo {
            item_id: "123",
            quantity: 12,
            source_inventory: inventory.get().id,
        }))
    };
    let on_mouse_move = move |mouse_event: MouseEvent| {
        if drag_state.dragging.get().is_some() {
            drag_state.mouse_pos.update(|pos| {
                pos.0 = mouse_event.screen_x();
                pos.1 = mouse_event.screen_y()
            })
        }
    };
    let end_drag = move |_| {
        drag_state.dragging.set(None);
    };

    view! {
        <div class="flex p-2">
            <For
                each=move || inventory.get().items.get()
                key=|item| item.0
                children=move |item| {
                    let item = ITEMS
                        .iter()
                        .find(|i| i.id == item.0)
                        .expect("couldn't find item in ITEMS list");
                    view! {
                        <div
                            class=format!(
                                "p-2 m-1 flex items-center justify-center font-bold text-lg h-14 w-14 text-shadow-lg {}",
                                item.color(),
                            )
                            on:mousedown=start_drag
                            on:mousemove=on_mouse_move
                            on:mouseup=end_drag
                        >
                            <span>{item.id}</span>
                        </div>
                    }
                }
            />
        </div>
    }
}
