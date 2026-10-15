# battery-rs

> **The open framework for battery software engineering.**

A Rust foundation for developing, validating and deploying advanced battery algorithms—with a shared path from research experiments to industrial BMS applications.

## Why battery-rs?

Battery algorithms are often developed in isolated research environments and reimplemented for individual hardware platforms. This makes results difficult to compare and creates additional work when transferring algorithms into products.

`battery-rs` aims to connect algorithm development, reproducible validation, simulation and hardware deployment through common contracts and reusable Rust implementations.

The long-term ambition is to support advanced Battery Management Systems: from state estimation and diagnostics to integrated BMS software and, eventually, a complete reference BMS implementation.

## For researchers and industry

**Researchers** should be able to develop new algorithms, compare them against reference implementations, test them on shared datasets and simulated scenarios, and produce reproducible evidence.

**Industrial teams** should be able to evaluate those algorithms against application requirements, integrate the shared software into supported BMS hardware, and verify its behavior through target testing, hardware-in-the-loop and field operation.

The goal is to make research results easier to assess, reproduce and transfer into real battery systems.

## Architecture

The architecture separates shared battery software from the environments that develop, evaluate and deploy it:

| Part | Responsibility |
|---|---|
| **Shared foundation** | Measurement and state types, algorithm contracts, configuration, data processing and reusable algorithm implementations. |
| **Development environment** | Algorithm development, recorded-data replay, parameter exploration and benchmark experiments. |
| **Validation catalogue and harness** | Versioned datasets, ground truth, scenarios, fault profiles, reference algorithms, metrics and acceptance criteria; repeatable execution and result comparison. |
| **Simulation** | Evaluate algorithms against plant models and operating scenarios; connect external simulation tools through adapters. |
| **Adapters** | Connect datasets, models and external tools to shared contracts; later connect vendor hardware and deployment runtimes. |
| **Visualization and monitoring** | A unified dashboard for measurements, estimates, diagnostics and validation results across replay, simulation and, later, live systems. |
| **Deployment and hardware validation** | Vendor-supported target builds, hardware integration, target tests, HiL and battery bench testing. |
| **Field feedback** | Return operating data and observed failures to the validation catalogue as new datasets and regression cases. |

The preferred deployment path reuses the same algorithm implementation. Where hardware constraints require a variant, its behavior must be compared against the reference implementation.

BMS vendors and integration partners provide target-specific drivers, hardware mapping, communication, scheduling and runtime integration. These capabilities are deferred while the shared foundation and development workflow mature.

## Scope

- State estimation: SOC, SOH, SOP and uncertainty estimation
- Diagnostics, signal processing and battery data analysis
- Physics-based, statistical and data-driven algorithms
- Reproducible benchmarking, simulation and regression validation
- Unified visualization and monitoring
- Portable software for embedded, desktop, web and backend applications
- Extensible hardware, protocol and model integration
- Future integrated BMS implementations and digital-twin applications

Specialist electrochemical simulation tools can be integrated rather than recreated within the framework.

## Principles

- **Portable by design:** keep algorithm logic independent of hardware and execution environment.
- **Validation first:** evaluate behavior using explicit scenarios, reference results and acceptance criteria.
- **Reproducible evidence:** associate results with algorithm versions, configurations, datasets and targets.
- **Modular and vendor-neutral:** support different algorithms, tools and hardware through clear boundaries.
- **Rust foundations:** use Rust's type system and memory-safety guarantees, with `std` / `no_std` support for portable components.
- **Open collaboration:** enable contributions from researchers, software developers and BMS manufacturers.

## Repository structure

The Rust library is currently published as **one crate**, organized into modules:

| Module | Purpose |
|---|---|
| `core` | Common domain types and algorithm contracts. |
| `algorithms` | State estimation, diagnostics and future advanced algorithms. |
| `data` | Data representations, preprocessing and signal utilities. |
| `validation` | Planned replay, metrics, benchmarking and regression utilities. |
| `integration` | Planned adapters for datasets, models, protocols and external tools. |
| `platform` | Reserved boundary for future deployment and target support. |

A dedicated `simulation` module is proposed as the framework grows. Dashboard applications and environment-specific tooling will sit around the portable library, using its shared data contracts.

## Current status

**Early development (`0.1.0-alpha.3`).** The repository contains typed battery quantities, measurements, a battery-state representation, an `Estimator` trait and a basic coulomb-counting SOC estimator. The crate has a `std` feature and a `no_std` build configuration.

The broader architecture above describes the intended direction. Replay, the validation catalogue, simulation, the dashboard and hardware support are not yet complete implementations; several modules currently establish boundaries only.

### Example

```rust
use battery_rs::algorithms::soc::coulomb_counter::CoulombCounter;
use battery_rs::{
    Capacity, Current, Estimator, Measurement, StateOfCharge,
    Temperature, Timestamp, Voltage,
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
            println!("t={second}s SOC={:.2}%", state.soc.unwrap().as_percent());
        }
    }

    Ok(())
}
```

## Development priorities

1. Strengthen core contracts and reference algorithms.
2. Add dataset adapters, replay, metrics and a reusable validation catalogue.
3. Add simulation and a first dashboard for replay and benchmark results.
4. Extend into vendor target support, hardware validation and live monitoring.
5. Use field evidence to improve algorithms and enable integrated BMS applications.

Contributions and pilot collaborations are welcome around reference datasets, algorithms, validation scenarios, tooling and future hardware support. See [the roadmap](docs/ROADMAP.md) for development planning.

## Domains

Automotive, energy storage, maritime, aerospace, robotics, industrial machinery, defence, civil protection and critical infrastructure.

## North star

Build a trusted open foundation for advanced battery systems, connecting reproducible research with practical industrial development.

## Licence

Dual-licensed under Apache-2.0 or MIT, at your option.

