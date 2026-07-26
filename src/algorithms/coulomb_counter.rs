//! Minimal reference SOC estimator based on coulomb counting.

use crate::core::{Algorithm, Measurement, State};

/// Errors returned by [`CoulombCounter`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoulombCounterError {
    /// The configured nominal capacity is zero or negative.
    InvalidCapacity,
    /// Measurement timestamps moved backwards.
    NonMonotonicTimestamp,
}

impl core::fmt::Display for CoulombCounterError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidCapacity => formatter.write_str("nominal capacity must be positive"),
            Self::NonMonotonicTimestamp => {
                formatter.write_str("measurement timestamps must be monotonic")
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for CoulombCounterError {}

/// A minimal coulomb-counting state-of-charge estimator.
#[derive(Debug, Clone)]
pub struct CoulombCounter {
    capacity_ah: f32,
    state_of_charge: f32,
    previous_timestamp_us: Option<u64>,
}

impl CoulombCounter {
    /// Creates an estimator from nominal capacity and initial SOC.
    pub fn new(capacity_ah: f32, initial_soc: f32) -> Result<Self, CoulombCounterError> {
        if capacity_ah <= 0.0 {
            return Err(CoulombCounterError::InvalidCapacity);
        }

        Ok(Self {
            capacity_ah,
            state_of_charge: initial_soc.clamp(0.0, 1.0),
            previous_timestamp_us: None,
        })
    }
}

impl Algorithm for CoulombCounter {
    type Output = State;
    type Error = CoulombCounterError;

    fn update(&mut self, measurement: &Measurement) -> Result<Self::Output, Self::Error> {
        if let Some(previous) = self.previous_timestamp_us {
            let delta_us = measurement
                .timestamp_us
                .checked_sub(previous)
                .ok_or(CoulombCounterError::NonMonotonicTimestamp)?;
            let delta_hours = delta_us as f32 / 3_600_000_000.0;
            self.state_of_charge -= measurement.current_a * delta_hours / self.capacity_ah;
            self.state_of_charge = self.state_of_charge.clamp(0.0, 1.0);
        }

        self.previous_timestamp_us = Some(measurement.timestamp_us);
        Ok(State::new(self.state_of_charge, None))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integrates_discharge_current() {
        let mut estimator = CoulombCounter::new(100.0, 1.0).expect("valid configuration");
        estimator
            .update(&Measurement::new(0, 400.0, 10.0, 25.0))
            .expect("first update");
        let state = estimator
            .update(&Measurement::new(3_600_000_000, 390.0, 10.0, 25.0))
            .expect("second update");

        assert!((state.state_of_charge - 0.9).abs() < 1e-6);
    }
}
