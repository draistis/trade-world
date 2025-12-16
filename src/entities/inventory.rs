use leptos::prelude::*;
use uuid::Uuid;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Item<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub category: &'a str,
    pub weight: u64,
    pub volume: u64,
}

impl Item<'_> {
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

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Inventory {
    pub id: String,
    pub name: String,
    pub items: RwSignal<Vec<(&'static str, u64)>>,
    pub max_volume: RwSignal<u64>,
    pub max_weight: RwSignal<u64>,
    pub weight: RwSignal<u64>,
    pub volume: RwSignal<u64>,
}

impl Inventory {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: "".to_string(),
            items: RwSignal::new(Vec::from([("GRV", 100), ("BRD", 120), ("LOG", 12)])),
            max_volume: RwSignal::new(500_000),
            max_weight: RwSignal::new(500_000),
            weight: RwSignal::new(332_000),
            volume: RwSignal::new(206_000),
        }
    }
    pub fn empty() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: "".to_string(),
            items: RwSignal::new(Vec::new()),
            max_volume: RwSignal::new(500_000),
            max_weight: RwSignal::new(500_000),
            weight: RwSignal::new(0),
            volume: RwSignal::new(0),
        }
    }
    pub fn one_item() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: "".to_string(),
            items: RwSignal::new(Vec::from([("CHR", 60)])),
            max_volume: RwSignal::new(500_000),
            max_weight: RwSignal::new(500_000),
            weight: RwSignal::new(54_000),
            volume: RwSignal::new(180_000),
        }
    }
    pub fn empty_weight(&self) -> u64 {
        self.max_weight.get() - self.weight.get()
    }
    pub fn empty_volume(&self) -> u64 {
        self.max_volume.get() - self.volume.get()
    }
    pub fn fits_max_items(&self, item_id: &'static str) -> u64 {
        let item = ITEMS
            .iter()
            .find(|i| i.id == item_id)
            .expect("item not in ITEMS list");

        u64::min(
            self.empty_volume() / item.volume,
            self.empty_weight() / item.weight,
        )
    }
    pub fn add_item(&mut self, item_id: &'static str, quantity: u64) {
        let item = ITEMS
            .iter()
            .find(|i| i.id == item_id)
            .expect("item not in ITEMS list");

        let max_qty_fits = self.fits_max_items(item_id);
        let moved_qty = u64::min(max_qty_fits, quantity);

        self.items
            .update(move |items: &mut Vec<(&'static str, u64)>| {
                if let Some((_id, qty)) = items.iter_mut().find(|i| i.0 == item_id) {
                    *qty += moved_qty;
                } else {
                    items.push((&item_id, moved_qty));
                }
            });

        self.volume.update(|v| *v += item.volume * moved_qty);
        self.weight.update(|w| *w += item.weight * moved_qty);
    }
    pub fn remove_item(&mut self, item_id: &'static str, quantity: u64) {
        let item = ITEMS
            .iter()
            .find(|i| i.id == item_id)
            .expect("item not in ITEMS list");

        self.items.update(|items| {
            if let Some((_id, qty)) = items.iter_mut().find(|i| i.0 == item_id) {
                *qty = qty.saturating_sub(quantity);
                if *qty <= 0 {
                    if let Some(pos) = items.iter().position(|i| i.0 == item_id) {
                        items.remove(pos);
                    }
                }
            } else {
                panic!("Something went wrong when removing items from inventory...");
            }
        });

        self.volume.update(|v| *v -= item.volume * quantity);
        self.weight.update(|w| *w -= item.weight * quantity);
    }
}

pub const ITEMS: &[Item] = &[
    Item {
        id: "LOG",
        name: "Logs",
        category: "Raw Materials",
        weight: 1_000,
        volume: 1_000,
    },
    Item {
        id: "GRV",
        name: "Gravel",
        category: "Raw Materials",
        weight: 2_000,
        volume: 1_500,
    },
    Item {
        id: "BRD",
        name: "Boards",
        category: "Processed Materials",
        weight: 1_000,
        volume: 1_200,
    },
    Item {
        id: "CHR",
        name: "Chair",
        category: "Carpentry Products",
        weight: 900,
        volume: 3_000,
    },
    Item {
        id: "DBG",
        name: "Debug",
        category: "Backrooms",
        weight: 5_100,
        volume: 8_100,
    },
];
