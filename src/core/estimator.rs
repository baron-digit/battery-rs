use super::measurement::Measurement;

pub trait Estimator {
    //! Estimator
    /// Output
    type Output;
    /// Error
    type Error;
    /// Update method
    fn update(&mut self, measurement: &Measurement) -> Result<Self::Output, Self::Error>;
}
