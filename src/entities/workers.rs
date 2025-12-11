use leptos::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WorkerType {
    Basic,
    Advanced,
    Expert,
}

impl WorkerType {
    /// Returns Vec of every worker type.
    pub fn all() -> Vec<Self> {
        vec![Self::Basic, Self::Advanced, Self::Expert]
    }

    /// Return information about specified `WorkerType`.
    pub fn details(&self) -> WorkerDetails {
        match self {
            WorkerType::Basic => WorkerDetails {
                name: "Basic Worker",
                description: "Can perform the most simple tasks.",
                cost: 100.0,
            },
            WorkerType::Advanced => WorkerDetails {
                name: "Advanced Worker",
                description: "Has good education and can perform more complex tasks.",
                cost: 175.0,
            },
            WorkerType::Expert => WorkerDetails {
                name: "Expert Worker",
                description: "Has the skills to accomplish the most complex tasks.",
                cost: 250.0,
            },
        }
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

pub struct WorkerDetails {
    pub name: &'static str,
    pub description: &'static str,
    pub cost: f64,
}
