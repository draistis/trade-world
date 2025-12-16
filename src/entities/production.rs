use leptos::prelude::*;

use crate::entities::WorkerType;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum ProductionType {
    Warehouse,
    Sawmill,
    Workshop,
    WaterPump,
}

impl ProductionType {
    pub fn all() -> Vec<Self> {
        vec![
            Self::Warehouse,
            Self::Sawmill,
            Self::Workshop,
            Self::WaterPump,
        ]
    }

    pub fn details(&self) -> ProductionDetails {
        match self {
            Self::Sawmill => ProductionDetails {
                name: "Sawmill",
                description: "Processes logs into basic wood products.",
                cost: 1000.0,
                workers: vec![(WorkerType::Basic, 5), (WorkerType::Advanced, 2)],
                land: 45,
            },
            Self::Warehouse => ProductionDetails {
                name: "Warehouse",
                description: "Safe storage of materials.",
                cost: 600.0,
                workers: vec![],
                land: 50,
            },
            Self::Workshop => ProductionDetails {
                name: "Workshop",
                description: "Produces wooden items from basic wood products.",
                cost: 1900.0,
                workers: vec![(WorkerType::Advanced, 5), (WorkerType::Expert, 3)],
                land: 40,
            },
            Self::WaterPump => ProductionDetails {
                name: "Water Pump",
                description: "Extracts water from nearby water sources.",
                cost: 700.0,
                workers: vec![(WorkerType::Basic, 6)],
                land: 25,
            },
        }
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

    pub fn build(&self, production_type: ProductionType, amount: u64) {
        match production_type {
            ProductionType::WaterPump => self.water_pump.update(|a| *a += amount),
            ProductionType::Warehouse => self.warehouse.update(|a| *a += amount),
            ProductionType::Workshop => self.workshop.update(|a| *a += amount),
            ProductionType::Sawmill => self.sawmill.update(|a| *a += amount),
        }
    }
}

pub struct ProductionDetails {
    pub name: &'static str,
    pub description: &'static str,
    pub cost: f64,
    pub workers: Vec<(WorkerType, u64)>,
    pub land: u64,
}
