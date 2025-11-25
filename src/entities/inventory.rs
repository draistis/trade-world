use std::collections::HashMap;

use leptos::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Item<'a> {
    id: &'a str,
    name: &'a str,
    category: &'a str,
    weight: f64,
    volume: f64,
}

impl Item<'_> {
    pub fn color(&self) -> &'static str {
        match self.name {
            "Logs" => "bg-yellow-950",
            "Gravel" => "bg-zinc-800",
            "Boards" => "bg-yellow-600",
            "Chair" => "bg-orange-300",
            _ => "bg-blue-100",
        }
    }
}

#[derive(Clone, Debug)]
pub struct Inventory {
    id: u64,
    name: String,
    items: HashMap<&'static str, u64>,
    max_volume: f64,
    max_weight: f64,
    weight: f64,
    volume: f64,
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
