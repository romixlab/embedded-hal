[![crates.io](https://img.shields.io/crates/d/embedded-hal-async.svg)](https://crates.io/crates/embedded-hal-async)
[![crates.io](https://img.shields.io/crates/v/embedded-hal-async.svg)](https://crates.io/crates/embedded-hal-async)
[![Documentation](https://docs.rs/embedded-hal-async/badge.svg)](https://docs.rs/embedded-hal-async)

# `embedded-hal-async`

An asynchronous Hardware Abstraction Layer (HAL) for embedded systems.

This crate contains asynchronous versions of the [`embedded-hal`](https://crates.io/crates/embedded-hal) traits and shares its scope and [design goals](https://docs.rs/embedded-hal/latest/embedded_hal/#design-goals).

This project is developed and maintained by the [HAL team](https://github.com/rust-embedded/wg#the-hal-team).

## Serial/UART traits

There is no serial traits in `embedded-hal-async`. Instead, use [`embedded-io-async`](https://crates.io/crates/embedded-io-async).
A serial port is essentially a byte-oriented stream, and that's what `embedded-io-async` models. Sharing the traits
with all byte streams has some advantages. For example, it allows generic code providing a command-line interface
or a console to operate either on hardware serial ports or on virtual ones like Telnet or USB CDC-ACM.

## Optional Cargo features

- **`defmt-03`**: Derive `defmt::Format` from `defmt` 0.3 for enums and structs.

## Minimum Supported Rust Version (MSRV)

This crate requires Rust nightly newer than `nightly-2022-11-22`, due to requiring support for
`async fn` in traits (AFIT), which is not stable yet.

Keep in mind Rust nightlies can make backwards-incompatible changes to unstable features
at any time. If this happens, we might do changes that increase the minimum required nightly
version in any patch release.

When AFIT becomes stable, MSRV will be bumped to the Rust version that stabilizes it, after which
point the [standard MSRV bump policy](../docs/msrv.md) will apply.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
