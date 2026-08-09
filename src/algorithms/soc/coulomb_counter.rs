//! Minimal reference SOC estimator based on coulomb counting.

#![allow(unused_imports)]
use crate::core::state::BatteryState;
use crate::core::{
    Capacity, Current, Estimator, Measurement, StateOfCharge, Temperature, Timestamp, Voltage,
};

/// Errors returned by [`CoulombCounter`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoulombCounterError {
    /// Capactity value needs to be positive
    InvalidCapacity,
    /// Initial SOC within 0-1 bound
    InvalidInitialSoc,
    /// If timestamp is not monotonic
    NonMonotonicTimestamp,
}

impl core::fmt::Display for CoulombCounterError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidCapacity => formatter.write_str("nominal capacity must be positive"),
            Self::InvalidInitialSoc => formatter.write_str("initial SOC must be between 0 and 1"),
            Self::NonMonotonicTimestamp => {
                formatter.write_str("measurement timestamps must be monotonic")
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for CoulombCounterError {}

/// CoulombCounter
#[derive(Debug, Clone)]
pub struct CoulombCounter {
    capacity: Capacity,
    state_of_charge: StateOfCharge,
    previous_timestamp: Option<Timestamp>,
}

impl CoulombCounter {
    /// CoulombCounter Constructor
    pub fn new(
        capacity: Capacity,
        initial_soc: StateOfCharge,
    ) -> Result<Self, CoulombCounterError> {
        Ok(Self {
            capacity,
            state_of_charge: initial_soc,
            previous_timestamp: None,
        })
    }
}

impl Estimator for CoulombCounter {
    type Output = BatteryState;
    type Error = CoulombCounterError;

    fn update(&mut self, measurement: &Measurement) -> Result<Self::Output, Self::Error> {
        if let Some(previous) = self.previous_timestamp {
            let delta_us = measurement
                .timestamp
                .as_micros()
                .checked_sub(previous.as_micros())
                .ok_or(CoulombCounterError::NonMonotonicTimestamp)?;

            let delta_hours = delta_us as f32 / 3_600_000_000.0;

            let soc = self.state_of_charge.as_fraction()
                - measurement.current.as_amperes() * delta_hours / self.capacity.as_ampere_hours();

            self.state_of_charge =
                StateOfCharge::from_fraction(soc.clamp(0.0, 1.0)).expect("clamped SOC is valid");
        }

        self.previous_timestamp = Some(measurement.timestamp);

        Ok(BatteryState {
            soc: Some(self.state_of_charge),
            soh: None,
        })
    }
}

#[test]
fn integrates_discharge_current() {
    let mut estimator = CoulombCounter::new(
        Capacity::from_ampere_hours(100.0).unwrap(),
        StateOfCharge::from_fraction(1.0).unwrap(),
    )
    .unwrap();

    estimator
        .update(&Measurement::new(
            Timestamp::from_micros(0),
            Voltage::from_volts(400.0),
            Current::from_amperes(10.0),
            Temperature::from_celsius(25.0),
        ))
        .unwrap();

    let state = estimator
        .update(&Measurement::new(
            Timestamp::from_micros(3_600_000_000),
            Voltage::from_volts(390.0),
            Current::from_amperes(10.0),
            Temperature::from_celsius(25.0),
        ))
        .unwrap();

    assert!((state.soc.unwrap().as_fraction() - 0.9).abs() < 1e-6);
}
