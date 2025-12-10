use std::collections::HashMap;

use leptos::prelude::*;

use crate::entities::Inventory;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Tile {
    pub id: &'static str,
    pub description: &'static str,
    pub resources: Vec<&'static str>,
    pub price: f64,
    pub row: u32,
    pub col: u32,
    pub is_owned: RwSignal<bool>,
    pub tile_state: TileState,
}

impl Tile {
    pub fn new() -> Self {
        Self {
            id: "",
            description: "",
            resources: Vec::new(),
            price: 0.,
            is_owned: RwSignal::new(false),
            row: 0,
            col: 0,
            tile_state: TileState::new(),
        }
    }
    pub fn hired_workers(&self, worker_type: WorkerType) -> u64 {
        self.tile_state.workers.get_workers(worker_type)
    }
    pub fn available_workers(&self, worker_type: WorkerType) -> u64 {
        self.tile_state.buildings.get_capacity(worker_type)
    }
    pub fn hire_worker(
        &self,
        worker_type: WorkerType,
        money: RwSignal<f64>,
    ) -> Result<(), &'static str> {
        if self.available_workers(worker_type) <= 0 {
            return Err("Insufficient housing space.");
        }
        match worker_type {
            WorkerType::Basic => {
                if money.get() < 100.0 {
                    return Err("Insufficient funds. Need $100.");
                }
                self.tile_state.workers.basic.update(|a| *a += 1);
                Ok(())
            }
            WorkerType::Advanced => {
                if money.get() < 175.0 {
                    return Err("Insufficient funds. Need $175.");
                }
                self.tile_state.workers.advanced.update(|a| *a += 1);
                Ok(())
            }
            WorkerType::Expert => {
                if money.get() < 250.0 {
                    return Err("Insufficient funds. Need $250.");
                }
                self.tile_state.workers.expert.update(|a| *a += 1);
                Ok(())
            }
        }
    }
    pub fn owned_housing(&self, housing_type: HousingType) -> u64 {
        let housing = self.tile_state.buildings.housing;
        match housing_type {
            HousingType::Cheap => housing.cheap.get(),
            HousingType::Standard => housing.standard.get(),
            HousingType::Fancy => housing.fancy.get(),
        }
    }
    pub fn build_housing(&self, housing_type: HousingType, money: RwSignal<f64>) {
        match housing_type {
            HousingType::Cheap => {
                if money.get() < 500.0 {
                    return;
                }
                money.update(|m| *m -= 500.0);
                self.tile_state.buildings.housing.cheap.update(|a| *a += 1);
            }
            HousingType::Standard => {
                if money.get() < 750.0 {
                    return;
                }
                money.update(|m| *m -= 750.0);
                self.tile_state
                    .buildings
                    .housing
                    .standard
                    .update(|a| *a += 1);
            }
            HousingType::Fancy => {
                if money.get() < 1250.0 {
                    return;
                }
                money.update(|m| *m -= 1250.0);
                self.tile_state.buildings.housing.fancy.update(|a| *a += 1);
            }
        }
    }
    pub fn owned_production_buildings(&self, production_type: ProductionBuildingType) -> u64 {
        let production = self.tile_state.buildings.production;
        match production_type {
            ProductionBuildingType::Warehouse => production.warehouse.get(),
            ProductionBuildingType::Sawmill => production.sawmill.get(),
            ProductionBuildingType::WaterPump => production.water_pump.get(),
            ProductionBuildingType::Workshop => production.workshop.get(),
        }
    }
    pub fn build_production(&self, production_type: ProductionBuildingType, money: RwSignal<f64>) {
        match production_type {
            ProductionBuildingType::Sawmill => {
                if money.get() < 1000.0 {
                    return;
                }
                money.update(|m| *m -= 1000.0);
                self.tile_state
                    .buildings
                    .production
                    .sawmill
                    .update(|a| *a += 1);
            }
            ProductionBuildingType::Warehouse => {
                if money.get() < 600.0 {
                    return;
                }
                money.update(|m| *m -= 600.0);
                self.tile_state
                    .buildings
                    .production
                    .warehouse
                    .update(|a| *a += 1);
            }
            ProductionBuildingType::WaterPump => {
                if money.get() < 700.0 {
                    return;
                }
                money.update(|m| *m -= 700.0);
                self.tile_state
                    .buildings
                    .production
                    .water_pump
                    .update(|a| *a += 1);
            }
            ProductionBuildingType::Workshop => {
                if money.get() < 1900.0 {
                    return;
                }
                money.update(|m| *m -= 1900.0);
                self.tile_state
                    .buildings
                    .production
                    .workshop
                    .update(|a| *a += 1);
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct TileState {
    pub inventory: RwSignal<Inventory>,
    pub total_land: RwSignal<u32>,
    pub empty_land: RwSignal<u32>,
    pub buildings: Buildings,
    pub workers: Workers,
    pub housing: Housing,
}

impl TileState {
    pub fn new() -> Self {
        Self {
            inventory: RwSignal::new(Inventory::new()),
            total_land: RwSignal::new(500),
            empty_land: RwSignal::new(500),
            buildings: Buildings::new(),
            workers: Workers::new(),
            housing: Housing::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WorkerType {
    Basic,
    Advanced,
    Expert,
}

impl WorkerType {
    pub fn all() -> Vec<Self> {
        vec![Self::Basic, Self::Advanced, Self::Expert]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Workers {
    pub basic: RwSignal<u64>,
    pub advanced: RwSignal<u64>,
    pub expert: RwSignal<u64>,
}

impl Workers {
    pub fn new() -> Self {
        Self {
            basic: RwSignal::new(0),
            advanced: RwSignal::new(0),
            expert: RwSignal::new(0),
        }
    }
    pub fn get_workers(&self, worker_type: WorkerType) -> u64 {
        match worker_type {
            WorkerType::Basic => self.basic.get(),
            WorkerType::Advanced => self.advanced.get(),
            WorkerType::Expert => self.expert.get(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Buildings {
    pub production: Production,
    pub housing: Housing,
    pub roads: RwSignal<HashMap<String, u64>>,
}

impl Buildings {
    pub fn new() -> Self {
        Self {
            production: Production::new(),
            housing: Housing::new(),
            roads: RwSignal::new(HashMap::new()),
        }
    }
    pub fn get_capacity(&self, worker_type: WorkerType) -> u64 {
        match worker_type {
            WorkerType::Basic => self.housing.cheap.get() * 10,
            WorkerType::Advanced => self.housing.standard.get() * 5,
            WorkerType::Expert => self.housing.fancy.get() * 3,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum ProductionBuildingType {
    Warehouse,
    Sawmill,
    Workshop,
    WaterPump,
}
impl ProductionBuildingType {
    pub fn all() -> Vec<Self> {
        vec![
            Self::Warehouse,
            Self::Sawmill,
            Self::Workshop,
            Self::WaterPump,
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Production {
    pub warehouse: RwSignal<u64>,
    pub sawmill: RwSignal<u64>,
    pub workshop: RwSignal<u64>,
    pub water_pump: RwSignal<u64>,
}

impl Production {
    pub fn new() -> Self {
        Self {
            warehouse: RwSignal::new(0),
            sawmill: RwSignal::new(0),
            workshop: RwSignal::new(0),
            water_pump: RwSignal::new(0),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum HousingType {
    Cheap,
    Standard,
    Fancy,
}

impl HousingType {
    pub fn all() -> Vec<Self> {
        vec![Self::Cheap, Self::Standard, Self::Fancy]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Housing {
    pub cheap: RwSignal<u64>,
    pub standard: RwSignal<u64>,
    pub fancy: RwSignal<u64>,
}

impl Housing {
    pub fn new() -> Self {
        Self {
            cheap: RwSignal::new(0),
            standard: RwSignal::new(0),
            fancy: RwSignal::new(0),
        }
    }
}
