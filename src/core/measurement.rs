//! Battery measurement types.

/// A timestamped set of electrical and thermal battery measurements.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Measurement {
    /// Monotonic timestamp in microseconds.
    pub timestamp_us: u64,
    /// Battery voltage in volts.
    pub voltage_v: f32,
    /// Battery current in amperes. Positive current denotes discharge.
    pub current_a: f32,
    /// Battery temperature in degrees Celsius.
    pub temperature_c: f32,
}

impl Measurement {
    /// Creates a new measurement.
    #[must_use]
    pub const fn new(
        timestamp_us: u64,
        voltage_v: f32,
        current_a: f32,
        temperature_c: f32,
    ) -> Self {
        Self {
            timestamp_us,
            voltage_v,
            current_a,
            temperature_c,
        }
    }
}
