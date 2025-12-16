use leptos::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WorkerType {
    Basic,
    Advanced,
    Expert,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Workers {
    pub basic: WorkerCategory,
    pub advanced: WorkerCategory,
    pub expert: WorkerCategory,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct WorkerDetails {
    pub name: &'static str,
    pub description: &'static str,
    pub cost: f64,
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
pub struct WorkerCategory {
    total: RwSignal<u64>,
    assigned: RwSignal<u64>,
}

impl WorkerCategory {
    pub fn new() -> Self {
        Self {
            total: RwSignal::new(0),
            assigned: RwSignal::new(0),
        }
    }

    pub fn available(&self) -> u64 {
        self.total.get() - self.assigned.get()
    }

    pub fn hire(&self, amount: u64) {
        self.total.update(|w| *w += amount);
    }

    pub fn fire(&self, amount: u64) -> Result<(), String> {
        if self.total.get() < amount {
            return Err(format!(
                "Cannot fire {} workers, only have {}.",
                amount,
                self.total.get()
            ));
        }
        if self.available() <= amount {
            return Err("Cannot fire assigned workers.".into());
        }
        self.total.update(|w| *w -= amount);
        Ok(())
    }

    pub fn assign(&self, amount: u64) -> Result<(), String> {
        if self.available() < amount {
            return Err(format!(
                "Cannot assign {} workers, only {} available.",
                amount,
                self.available()
            ));
        }
        self.assigned.update(|w| *w += amount);
        Ok(())
    }

    pub fn unassign(&self, amount: u64) -> Result<(), String> {
        if self.assigned.get() < amount {
            return Err(format!(
                "Cannot unassign {} workers, only {} assigned.",
                amount,
                self.assigned.get()
            ));
        }
        self.assigned.update(|w| *w -= amount);
        Ok(())
    }
}

impl Workers {
    pub fn new() -> Self {
        Self {
            basic: WorkerCategory::new(),
            advanced: WorkerCategory::new(),
            expert: WorkerCategory::new(),
        }
    }

    pub fn get_total(&self, worker_type: WorkerType) -> u64 {
        match worker_type {
            WorkerType::Basic => self.basic.total.get(),
            WorkerType::Advanced => self.advanced.total.get(),
            WorkerType::Expert => self.expert.total.get(),
        }
    }

    pub fn get_available(&self, worker_type: WorkerType) -> u64 {
        match worker_type {
            WorkerType::Basic => self.basic.available(),
            WorkerType::Advanced => self.advanced.available(),
            WorkerType::Expert => self.expert.available(),
        }
    }

    pub fn assign(&self, worker_type: WorkerType, amount: u64) -> Result<(), String> {
        match worker_type {
            WorkerType::Basic => self.basic.assign(amount),
            WorkerType::Advanced => self.advanced.assign(amount),
            WorkerType::Expert => self.expert.assign(amount),
        }
    }

    pub fn check_assign(&self, workers: &[(WorkerType, u64)]) -> Result<(), String> {
        for &(worker_type, amount) in workers {
            if self.get_available(worker_type) < amount {
                return Err(format!(
                    "Not enough {} workers. Need {}, have {}.",
                    worker_type.details().name,
                    amount,
                    self.get_available(worker_type)
                ));
            }
        }
        for &(worker_type, amount) in workers {
            self.assign(worker_type, amount)?;
        }
        Ok(())
    }

    pub fn hire(&self, worker_type: WorkerType, amount: u64) {
        match worker_type {
            WorkerType::Basic => self.basic.hire(amount),
            WorkerType::Advanced => self.advanced.hire(amount),
            WorkerType::Expert => self.expert.hire(amount),
        };
    }
}
