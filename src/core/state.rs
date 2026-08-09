use super::types::{StateOfCharge, StateOfHealth};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BatteryState {
    pub soc: Option<StateOfCharge>,
    pub soh: Option<StateOfHealth>,
}

impl BatteryState {
    pub const fn new() -> Self {
        Self {
            soc: None,
            soh: None,
        }
    }
}

impl Default for BatteryState {
    fn default() -> Self {
        Self::new()
    }
}
