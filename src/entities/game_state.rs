use leptos::prelude::*;

use crate::entities::Tile;

#[derive(Clone, Debug)]
pub struct GameState {
    pub cash: RwSignal<f64>,
    pub tiles: Vec<Tile<'static>>,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            cash: RwSignal::new(10000.),
            tiles: Vec::new(),
        }
    }
}
