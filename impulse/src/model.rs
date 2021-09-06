use std::fmt::{self, Debug, Display};

#[derive(Debug, Clone, Copy)]
pub enum SimulationStatus {
    Idle,
    Running,
    Complete,
    Cancelled,
}

impl Display for SimulationStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Debug::fmt(&self, f)
    }
}
