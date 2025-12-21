use leptos::prelude::*;
use uuid::Uuid;

#[derive(Copy, Clone, Debug, PartialEq, Default, PartialOrd, Eq, Ord, Hash)]
pub struct ItemId(&'static str);

impl IntoRender for ItemId {
    type Output = &'static str;
    fn into_render(self) -> Self::Output {
        self.0
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct ItemStack {
    pub id: ItemId,
    pub quantity: RwSignal<u64>,
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct ItemDetails {
    pub id: ItemId,
    pub name: &'static str,
    pub category: &'static str,
    pub weight: u64,
    pub volume: u64,
}

#[derive(Clone, Debug, PartialEq, Default, Eq, PartialOrd, Ord, Hash)]
pub struct InventoryId(String);

impl IntoRender for InventoryId {
    type Output = String;
    fn into_render(self) -> Self::Output {
        self.0
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Inventory {
    pub id: InventoryId,
    pub name: String,
    pub items: RwSignal<Vec<ItemStack>>,
    pub max_volume: RwSignal<u64>,
    pub max_weight: RwSignal<u64>,
    pub weight: RwSignal<u64>,
    pub volume: RwSignal<u64>,
}

impl Inventory {
    pub fn new() -> Self {
        Self {
            id: InventoryId(Uuid::new_v4().to_string()),
            name: "".to_string(),
            items: RwSignal::new(Vec::new()),
            max_volume: RwSignal::new(500_000),
            max_weight: RwSignal::new(500_000),
            weight: RwSignal::new(0),
            volume: RwSignal::new(0),
        }
    }

    pub fn empty_weight(&self) -> u64 {
        self.max_weight.get() - self.weight.get()
    }

    pub fn empty_volume(&self) -> u64 {
        self.max_volume.get() - self.volume.get()
    }

    pub fn fits_max_items(&self, item_id: ItemId) -> u64 {
        let item_details = ItemDetails::get(item_id).unwrap();

        u64::min(
            self.empty_volume() / item_details.volume,
            self.empty_weight() / item_details.weight,
        )
    }

    pub fn add_item(&mut self, item_id: ItemId, quantity: u64) {
        let item_details = ItemDetails::get(item_id).unwrap();

        let max_qty_fits = self.fits_max_items(item_id);
        let moved_qty = u64::min(max_qty_fits, quantity);

        self.items.update(move |items| {
            if let Some(item) = items.iter().find(|i| i.id == item_id) {
                item.quantity.update(|qty| *qty += moved_qty);
            } else {
                items.push(ItemStack {
                    id: item_id,
                    quantity: RwSignal::new(moved_qty),
                });
            }
        });

        self.volume
            .update(|v| *v += item_details.volume * moved_qty);
        self.weight
            .update(|w| *w += item_details.weight * moved_qty);
    }

    pub fn remove_item(&mut self, item_id: ItemId, quantity: u64) {
        let item_details = ItemDetails::get(item_id).unwrap();

        self.items.update(|items| {
            if let Some(item) = items.iter().find(|i| i.id == item_id) {
                item.quantity
                    .update(|qty| *qty = qty.saturating_sub(quantity));
                if item.quantity.get() <= 0 {
                    if let Some(pos) = items.iter().position(|i| i.id == item_id) {
                        items.remove(pos);
                    }
                }
            } else {
                panic!("Something went wrong when removing items from inventory...");
            }
        });

        self.volume.update(|v| *v -= item_details.volume * quantity);
        self.weight.update(|w| *w -= item_details.weight * quantity);
    }
}

impl ItemDetails {
    pub fn get(id: ItemId) -> Option<&'static Self> {
        ITEMS.iter().find(|i| i.id == id)
    }

    pub fn all() -> &'static [Self] {
        ITEMS
    }

    pub fn color(&self) -> &'static str {
        match self.name {
            "Logs" => "bg-yellow-950 text-yellow-100",
            "Gravel" => "bg-zinc-800 text-zinc-200",
            "Boards" => "bg-yellow-600 text-yellow-50",
            "Chair" => "bg-orange-300 text-orange-900",
            _ => "bg-blue-100 text-blue-900",
        }
    }
}

const ITEMS: &[ItemDetails] = &[
    ItemDetails {
        id: ItemId("LOG"),
        name: "Logs",
        category: "Raw Materials",
        weight: 1_000,
        volume: 1_000,
    },
    ItemDetails {
        id: ItemId("H2O"),
        name: "Water",
        category: "Liquids",
        weight: 2_000,
        volume: 1_500,
    },
    ItemDetails {
        id: ItemId("BRD"),
        name: "Boards",
        category: "Processed Materials",
        weight: 1_000,
        volume: 1_200,
    },
    ItemDetails {
        id: ItemId("CHR"),
        name: "Chair",
        category: "Carpentry Products",
        weight: 900,
        volume: 3_000,
    },
    ItemDetails {
        id: ItemId("DBG"),
        name: "Debug",
        category: "Backrooms",
        weight: 5_100,
        volume: 8_100,
    },
];
