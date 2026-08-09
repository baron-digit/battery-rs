//! Reusable battery algorithms for SOC

pub mod coulomb_counter;
pub use coulomb_counter::{CoulombCounter, CoulombCounterError};
