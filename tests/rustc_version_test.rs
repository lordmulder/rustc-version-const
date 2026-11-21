// SPDX-License-Identifier: 0BSD
// rustc-version-const
// Copyright (C) 2026 by LoRd_MuldeR <mulder2@gmx.de>

use rustc_version_const::{rustc_version, rustc_version_full};

#[test]
fn test_rustc_version() {
    let version = rustc_version();
    assert_eq!(version, version.trim_ascii());
    assert!(!version.is_empty());
    println!("rustc version: \"{version}\"");
}

#[test]
fn test_rustc_version_full() {
    let version_full = rustc_version_full();
    assert_eq!(version_full, version_full.trim_ascii());
    assert!(!version_full.trim().is_empty());
    println!("rustc full version: \"{version_full}\"");
}
