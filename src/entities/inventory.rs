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
            items: RwSignal::new(Vec::from([("GRV", 55), ("BRD", 120), ("LOG", 12)])),
            max_volume: 500.0,
            max_weight: 500.0,
            weight: RwSignal::new(0.0),
            volume: RwSignal::new(0.0),
        }
    }
    pub fn empty() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: "".to_string(),
            items: RwSignal::new(Vec::new()),
            max_volume: 500.0,
            max_weight: 500.0,
            weight: RwSignal::new(0.0),
            volume: RwSignal::new(0.0),
        }
    }
    pub fn one_item() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: "".to_string(),
            items: RwSignal::new(Vec::from([("GRV", 102)])),
            max_volume: 500.0,
            max_weight: 500.0,
            weight: RwSignal::new(0.0),
            volume: RwSignal::new(0.0),
        }
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
];
