use battery_rs::{
    Algorithm, Measurement,
    algorithms::{CoulombCounter, CoulombCounterError},
};

fn main() -> Result<(), CoulombCounterError> {
    let mut estimator = CoulombCounter::new(100.0, 0.80)?;

    for second in 1..=10 {
        let measurement = Measurement::new(second as u64, -10.0, 400.0, 25.0);

        let state = estimator.update(&measurement)?;

        println!("t={second}s, SOC={:.6}", state.state_of_charge);
    }

    Ok(())
}
