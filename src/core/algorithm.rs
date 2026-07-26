//! Common algorithm interface.

use super::Measurement;

/// A deterministic battery algorithm that processes measurements sequentially.
pub trait Algorithm {
    /// Algorithm output produced for each measurement.
    type Output;
    /// Algorithm-specific error.
    type Error;

    /// Processes one measurement and returns the current output.
    fn update(&mut self, measurement: &Measurement) -> Result<Self::Output, Self::Error>;
}
