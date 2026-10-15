# Roadmap

Build a shared Rust foundation that connects battery algorithm research with advanced industrial BMS development.

The immediate focus is the portable library and the environment for developing, comparing and validating algorithms. Vendor-specific hardware support follows once that workflow is established.

This roadmap describes development priorities, not completed capabilities or delivery dates. Release groupings are provisional; the longer-term phases are not assigned versions yet.

## 0.1 — Shared foundation

- Refine measurement and state types, units, timestamps and current sign conventions.
- Define algorithm initialization, update, configuration, reset and error behavior.
- Strengthen the coulomb-counting reference implementation and its tests.
- Keep portable components independent of files, networking, drivers and scheduling.
- Verify `std` and `no_std` builds in CI and document supported assumptions.
- Document input validity, security assumptions and integration boundaries.

**Milestone:** a documented reference estimator runs reproducibly against known measurement sequences, with tested edge cases and portable build checks.

## 0.2 — Algorithm development and validation

- Add dataset adapters for import, unit normalization and timestamp handling.
- Implement recorded-data replay through the common algorithm contracts.
- Add baseline comparisons and metrics for accuracy, robustness and execution cost.
- Establish an extensible validation catalogue: datasets, ground truth, scenarios, sensor faults, reference algorithms and acceptance criteria.
- Record algorithm version, configuration, dataset version and execution environment with results.
- Add regression execution and machine-readable result export.
- Extend reference algorithms and signal-processing utilities as concrete experiments require them.

**Milestone:** compare two algorithm implementations on the same dataset and reproduce their benchmark results from a saved experiment configuration.

## 0.3 — Simulation and visualization

- Add a simulation boundary and simple battery plant models, separate from the algorithms under test.
- Provide adapters for external models and tools, including Python workflows where useful.
- Execute operating scenarios and fault profiles in software-in-the-loop (SiL).
- Reuse catalogue scenarios, acceptance criteria and result formats across replay and simulation.
- Build the first unified dashboard for measurements, estimates, diagnostics, reference comparisons and validation results.
- Support algorithm composition and parameter exploration through explicit configurations.

Dashboard applications and environment-specific tooling sit around the portable library and use its shared data contracts.

**Milestone:** run a shared algorithm against recorded data and a simulated plant, then inspect and compare the results in the same dashboard.

## Later — Vendor-supported deployment and hardware validation

- Define target adapter contracts with BMS manufacturers and integration partners.
- Add vendor-owned hardware and protocol mapping, drivers, scheduling, runtime integration and target builds.
- Establish a reference hardware target before extending the supported-target catalogue.
- Reuse algorithm implementations; compare any hardware-constrained variants against their reference implementations.
- Measure numerical behavior, execution time and memory on real target hardware, including processor-in-the-loop (PiL) where appropriate.
- Extend validation into HiL with real BMS hardware and a simulated battery, followed by real battery bench testing.
- Reuse applicable catalogue scenarios and criteria, adding hardware-specific tests where needed.

**Milestone:** demonstrate one algorithm from development through SiL, target execution and HiL, with linked validation evidence.

## Later — Field monitoring and advanced BMS applications

- Add telemetry adapters and live dashboard views using the shared measurement and diagnostic contracts.
- Link field results to deployed software, calibration and hardware versions.
- Turn field failures and operating conditions into replay datasets and regression scenarios.
- Extend SOC, SOH, SOP, diagnostics, uncertainty estimation and advanced data-driven algorithms.
- Explore digital twins associated with actual battery configurations and operating histories.
- Progress toward integrated BMS software and, eventually, a complete reference BMS implementation.

**Milestone:** demonstrate a closed feedback loop from field measurements to reproducible validation and an evaluated algorithm improvement.

## Cybersecurity throughout development

Cybersecurity work starts with the foundation and grows with each integration boundary:

- **Foundation:** documented trust boundaries, robust input handling, security reporting guidance and dependency review.
- **Development tooling:** SBOM generation, malformed-input tests, fuzzing and security regression scenarios.
- **Simulation and dashboard:** threat analysis, access-control requirements and protection of imported data, models and configurations.
- **Deployment:** agreed vendor responsibilities for secure boot, keys, authenticated communication, calibration integrity and software updates.
- **Operation:** vulnerability handling, security-event monitoring and maintained verification evidence.

Assess application-specific engineering and regulatory obligations, including ISO/SAE 21434, UN R155 and the EU Cyber Resilience Act where relevant. Track requirements and evidence without treating the framework itself as certified or compliant. See [the README](../README.md#cybersecurity-and-cyber-resilience) for scope and applicability.

## Collaboration

Early collaborations should focus on reference datasets, algorithms, validation scenarios, adapters and development tooling. Hardware pilots follow the shared development workflow.

A useful first pilot is a reference estimator evaluated on partner data, compared against an agreed baseline and presented in the dashboard. This provides a concrete basis for a later hardware integration pilot.

