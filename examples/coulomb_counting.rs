use battery_rs::algorithms::soc::coulomb_counter::CoulombCounter;
use battery_rs::{
    Capacity, Current, Estimator, Measurement, StateOfCharge, Temperature, Timestamp, Voltage,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut estimator = CoulombCounter::new(
        Capacity::from_ampere_hours(100.0).unwrap(),
        StateOfCharge::from_fraction(0.80).unwrap(),
    )?;

    for second in 0..=3600 {
        let measurement = Measurement::new(
            Timestamp::from_micros(second * 1_000_000),
            Voltage::from_volts(400.0),
            Current::from_amperes(10.0),
            Temperature::from_celsius(25.0),
        );

        let state = estimator.update(&measurement)?;

        if second % 600 == 0 {
            println!("t={}s SOC={:.2}%", second, state.soc.unwrap().as_percent());
        }
    }

    Ok(())
}
