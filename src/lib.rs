#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

//! `battery-rs` is an open framework for battery software engineering.
//!
//! It provides common, platform-independent building blocks for battery
//! algorithms, integration, validation, data handling and deployment.

/// Battery algorithms such as state estimation and diagnostics.
pub mod algorithms;
/// Core domain types and framework traits.
pub mod core;
/// Battery data representations and processing utilities.
pub mod data;
/// Interfaces to hardware, protocols and external tools.
pub mod integration;
/// Deployment abstractions for embedded, desktop and backend targets.
pub mod platform;
/// Replay, metrics and validation utilities.
pub mod validation;

pub use crate::core::{Algorithm, Measurement, State};
