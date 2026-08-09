use super::types::{Current, Temperature, Timestamp, Voltage};

/// Initial measurement struct
/// Future work will take cycles in to consideration
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Measurement {
    /// timestamp
    pub timestamp: Timestamp,
    pub voltage: Voltage,
    pub current: Current,
    pub temperature: Temperature,
}

impl Measurement {
    pub const fn new(
        timestamp: Timestamp,
        voltage: Voltage,
        current: Current,
        temperature: Temperature,
    ) -> Self {
        Self {
            timestamp,
            voltage,
            current,
            temperature,
        }
    }
}
