# tea-soft

[![crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
![Apache2/MIT licensed][license-image]
[![Build Status][build-image]][build-link]

Rust implementation of TEA cipher

Compatible with [RustCrypto: Block Cipher Traits](https://github.com/RustCrypto/traits/tree/master/block-cipher).

## Warnings
TEA is not a particularly secure cipher, let alone the implementation makes no effort on avoiding side-channel attack.

**USE AT YOUR OWN RISK.**

## Usage
See [RustCrypto Block Cipher Usage](https://github.com/RustCrypto/block-ciphers#usage)

## License

Licensed under either of:

 * [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
 * [MIT license](http://opensource.org/licenses/MIT)

at your option.

[//]: # (badges)

[crate-image]: https://img.shields.io/crates/v/tea-soft.svg
[crate-link]: https://crates.io/crates/tea-soft
[docs-image]: https://docs.rs/tea-soft/badge.svg
[docs-link]: https://docs.rs/tea-soft/
[license-image]: https://img.shields.io/badge/license-Apache2.0/MIT-blue.svg
[build-image]: https://github.com/johnmave126/tea-soft/workflows/tea/badge.svg?branch=master&event=push
[build-link]: https://github.com/johnmave126/tea-soft/actions?query=workflow:tea+branch:master
