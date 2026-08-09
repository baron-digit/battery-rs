//! Reusable battery algorithms.

pub mod soc;

pub use soc::coulomb_counter::{CoulombCounter, CoulombCounterError};
