use std::collections::HashMap;

use leptos::prelude::*;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Tile {
    pub name: String,
    pub description: String,
    pub resources: HashMap<String, String>,
    pub price: f64,
    pub owned: RwSignal<bool>,
    pub row: u32,
    pub col: u32,
}

impl Tile {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            description: String::new(),
            resources: HashMap::new(),
            price: 0.,
            owned: RwSignal::new(false),
            row: 0,
            col: 0,
        }
    }
}
