use std::fmt::{self, Debug, Display};

#[derive(Debug, Clone, Copy)]
pub enum SimulationEvent {
    StatusUpdate(SimulationStatus),
}

#[derive(Debug, Clone, Copy)]
pub enum InterfaceEvent {
    StartSimulation
}

#[derive(Debug, Clone, Copy)]
pub enum SimulationStatus {
    Idle,
    Pending,
    Running,
    Complete,
    Cancelled,
}

impl Display for SimulationStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Debug::fmt(&self, f)
    }
}
