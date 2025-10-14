// SPDX-License-Identifier: 0BSD
// rustc-version-const
// Copyright (C) 2025 by LoRd_MuldeR <mulder2@gmx.de>

use rustc_version_const::{rustc_version, rustc_version_full};

const RUSTC_VERSION: &str = rustc_version();
const RUSTC_VERSION_FULL: &str = rustc_version_full();

fn main() {
    println!("rustc version: \"{}\"", RUSTC_VERSION);
    println!("rustc full version: \"{}\"", RUSTC_VERSION_FULL);
}
