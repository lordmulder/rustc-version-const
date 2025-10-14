# rustc-version-const

[![no_std](https://img.shields.io/badge/rust-no__std-orchid?logo=rust)](https://docs.rust-embedded.org/book/intro/no-std.html)
[![Crates.io](https://img.shields.io/crates/v/rustc-version-const)](https://crates.io/crates/rustc-version-const)
[![Downloads](https://img.shields.io/crates/d/rustc-version-const)](https://crates.io/crates/rustc-version-const)
[![Release Date](https://img.shields.io/github/release-date/lordmulder/rustc-version-const)](https://crates.io/crates/rustc-version-const/versions)
[![Docs.rs](https://img.shields.io/docsrs/rustc-version-const)](https://docs.rs/rustc-version-const/latest/)
[![License](https://img.shields.io/crates/l/rustc-version-const)](https://opensource.org/license/0BSD)

Makes the current **rustc** version available via a `const fn` at runtime.

The version detection happens automatically at build-time.

This crate is `#![no_std]` and does *not* require any additional dependencies.

### Installation

In order to use this crate, you have to add it under `[dependencies]` to your **`Cargo.toml`**:

```
[dependencies]
rustc-version-const = "1.0.0"
```

### Usage

Here is a simple example that demonstrates how to use `rustc_version()` in your code:

```rust
use rustc_version_const::rustc_version;

fn main() {
    println!("rustc version: \"{}\"", rustc_version());
}
```

## License

This software is released under the BSD Zero Clause (“0BSD”) License.

Copyright (C) 2025 by LoRd_MuldeR &lt;mulder2@gmx.de&gt;.
