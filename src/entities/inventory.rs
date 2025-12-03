use leptos::prelude::*;
use uuid::Uuid;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Item<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub category: &'a str,
    pub weight: f64,
    pub volume: f64,
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
    pub max_volume: f64,
    pub max_weight: f64,
    pub weight: RwSignal<f64>,
    pub volume: RwSignal<f64>,
}

impl Inventory {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: "".to_string(),
            items: RwSignal::new(Vec::from([("GRV", 100), ("BRD", 120), ("LOG", 12)])),
            max_volume: 500.0,
            max_weight: 500.0,
            weight: RwSignal::new(332.0),
            volume: RwSignal::new(206.0),
        }
    }
    pub fn empty() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: "".to_string(),
            items: RwSignal::new(Vec::from([("DBG", 50)])),
            max_volume: 500.0,
            max_weight: 500.0,
            weight: RwSignal::new(255.0),
            volume: RwSignal::new(405.0),
        }
    }
    pub fn one_item() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: "".to_string(),
            items: RwSignal::new(Vec::from([("CHR", 60)])),
            max_volume: 500.0,
            max_weight: 500.0,
            weight: RwSignal::new(54.0),
            volume: RwSignal::new(180.0),
        }
    }
    pub fn empty_weight(&self) -> f64 {
        self.max_weight - self.weight.get()
    }
    pub fn empty_volume(&self) -> f64 {
        self.max_volume - self.volume.get()
    }
    pub fn add_item(&mut self, item_id: &'static str, quantity: u64) {
        let item = ITEMS
            .iter()
            .find(|i| i.id == item_id)
            .expect("item not in ITEMS list");

        let max_qty_fits = u64::min(
            (self.empty_volume() / item.volume).floor() as u64,
            (self.empty_weight() / item.weight).floor() as u64,
        );
        let moved_qty = u64::min(max_qty_fits, quantity);
        self.items
            .update(move |items: &mut Vec<(&'static str, u64)>| {
                if let Some((_id, qty)) = items.iter_mut().find(|i| i.0 == item_id) {
                    *qty += moved_qty;
                } else {
                    items.push((&item_id, moved_qty));
                }
            });

        self.volume.update(|v| *v += item.volume * moved_qty as f64);
        self.weight.update(|w| *w += item.weight * moved_qty as f64);
    }
    pub fn remove_item(&mut self, item_id: &'static str, quantity: u64) {
        let item = ITEMS
            .iter()
            .find(|i| i.id == item_id)
            .expect("item not in ITEMS list");

        self.items.update(|items| {
            if let Some((_id, qty)) = items.iter_mut().find(|i| i.0 == item_id) {
                *qty -= quantity;
                if *qty <= 0 {
                    if let Some(pos) = items.iter().position(|i| i.0 == item_id) {
                        items.remove(pos);
                    }
                }
            } else {
                panic!("Something went wrong when removing items from inventory...");
            }
        });
        self.volume.update(|v| *v -= item.volume * quantity as f64);
        self.weight.update(|w| *w -= item.weight * quantity as f64);
    }
}

pub const ITEMS: &[Item] = &[
    Item {
        id: "LOG",
        name: "Logs",
        category: "Raw Materials",
        weight: 1.0,
        volume: 1.0,
    },
    Item {
        id: "GRV",
        name: "Gravel",
        category: "Raw Materials",
        weight: 2.0,
        volume: 1.5,
    },
    Item {
        id: "BRD",
        name: "Boards",
        category: "Processed Materials",
        weight: 1.0,
        volume: 1.2,
    },
    Item {
        id: "CHR",
        name: "Chair",
        category: "Carpentry Products",
        weight: 0.9,
        volume: 3.0,
    },
    Item {
        id: "DBG",
        name: "Debug",
        category: "Backrooms",
        weight: 5.1,
        volume: 8.1,
    },
];
