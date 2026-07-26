//! Battery state types.

/// Common estimated battery states.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct State {
    /// State of charge in the inclusive range `0.0..=1.0`.
    pub state_of_charge: f32,
    /// Optional state of health in the inclusive range `0.0..=1.0`.
    pub state_of_health: Option<f32>,
}

impl State {
    /// Creates a state and clamps fractional values to `0.0..=1.0`.
    #[must_use]
    pub fn new(state_of_charge: f32, state_of_health: Option<f32>) -> Self {
        Self {
            state_of_charge: state_of_charge.clamp(0.0, 1.0),
            state_of_health: state_of_health.map(|value| value.clamp(0.0, 1.0)),
        }
    }
}
