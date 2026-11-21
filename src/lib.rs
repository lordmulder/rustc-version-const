// SPDX-License-Identifier: 0BSD
// rustc-version-const
// Copyright (C) 2026 by LoRd_MuldeR <mulder2@gmx.de>

#![no_std]
#![allow(clippy::needless_doctest_main)]

//! # rustc-version-const
//!
//! Makes the current **rustc** version available via a `const fn` at runtime.
//!
//! The version detection happens automatically at build-time.
//!
//! ### Usage
//!
//! Here is a simple example that demonstrates how to use `rustc_version()` in your code:
//!
//! ```rust
//! use rustc_version_const::{rustc_version, rustc_version_full};
//!
//! const RUSTC_VERSION: &str = rustc_version();
//!
//! fn main() {
//!    println!("rustc version: \"{}\"", RUSTC_VERSION);
//! }
//! ```
//!
//! ### Features
//!
//! Optional feature `wrapper` (enabled by default) enables [`RUSTC_WRAPPER`](https://doc.rust-lang.org/cargo/reference/environment-variables.html) support.

/// Returns the `rustc` version that was used to build this application
///
/// This function returns just the plain version number *without* revision, e.g., `1.90.0`.
pub const fn rustc_version() -> &'static str {
    const RUSTC_VERSION: &str = env!("_RUSTC_VERSION_CONST");
    RUSTC_VERSION
}

/// Returns the `rustc` version that was used to build this application
///
/// This function returns the full version string, e.g., `rustc 1.90.0 (1159e78c4 2025-09-14)`.
pub const fn rustc_version_full() -> &'static str {
    const RUSTC_VERSION_FULL: &str = env!("_RUSTC_VERSION_CONST_FULL");
    RUSTC_VERSION_FULL
}
