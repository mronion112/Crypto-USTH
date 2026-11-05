#![deny(missing_docs)]

//! Toy key-recovery benchmark for RSA trial division and ECDLP brute force.
//!
//! The library exposes deterministic field, curve, and search primitives used by
//! the `toy-bench` and `gen-curve` binaries. It targets toy parameters only; the
//! measured costs do not describe standard RSA or elliptic-curve parameters.

pub mod ec;
pub mod ec_bench;
pub mod field;
pub mod rho;
pub mod rsa;

pub use ec::{AffinePoint, Curve, JacobianPoint};
pub use ec_bench::{EcMode, EcOutcome, recover_scalar};
pub use rho::{RhoOutcome, factor_rho, recover_scalar_rho};
pub use rsa::{RsaMode, RsaOutcome, factor_toy};
