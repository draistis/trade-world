use leptos::prelude::*;

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
            },
            Self::Warehouse => ProductionDetails {
                name: "Warehouse",
                description: "Safe storage of materials.",
                cost: 600.0,
            },
            Self::Workshop => ProductionDetails {
                name: "Workshop",
                description: "Produces wooden items from basic wood products.",
                cost: 1900.0,
            },
            Self::WaterPump => ProductionDetails {
                name: "Water Pump",
                description: "Extracts water from nearby water sources.",
                cost: 700.0,
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
}

pub struct ProductionDetails {
    pub name: &'static str,
    pub description: &'static str,
    pub cost: f64,
}
