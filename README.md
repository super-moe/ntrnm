# Ntrnm

![Rust Version][rustc-image]
[![crates.io][crate-image]][crate-link]
[![Documentation][docs-image]][docs-link]
[![Dependency Status][deps-image]][deps-link]

An IM protocol project. Built in rust, serving `OneBot 11` and `Kritor`.

<!-- markdown-toc start - Don't edit this section. Run M-x markdown-toc-refresh-toc -->

**Table of Contents**

- [`ntrnm`](#ntrnm)
    - [Supported features or functions](#supported-features-or-functions)
    - [Risk Behaviour](#risk-behaviour)
- [Installation](#installation)
    - [Compile from Source](#compile-from-source)
- [Usage](#usage)
    - [Command Line Interface](#command-line-interface)
- [License](#license)
    - [Contribution](#contribution)

<!-- markdown-toc end -->

## Supported features or functions

<details>

<summary>Supported Functions</summary>

| Login          | State              | Group          | State |
|----------------|--------------------|----------------|-------|
| Password Login |                    | Get Group List |       |
| QRCode Login   |                    |                |       |
| Ticket Login   | :heavy_check_mark: |                |       |

</details>

> It is recommended to use the `Shamrock` framework to export login tickets, login via tickets has bypassed the wind control as well as signature errors.

## Risk behaviour

<details>

<summary>Impossible Functions</summary>

- **Money-related functions**

</details>

# Installation

`{{crate_name}}` is a single binary that must be placed somewhere in your
`$PATH`. One can either download 64-bit Linux binaries from [the Release Page](https://github.com/inmes-dev/ntrnm/releases)
or one can also compile from source.

## Compile from Source

Ensure you have a [Rust toolchain installed](https://rustup.rs). Some of the
dependencies also require `gcc` to be installed.

```
$ git clone https://github.com/inmes-dev/ntrnm
$ cd ntrnm
$ cargo build --release
$ sudo cp target/release/ntrnm /usr/local/bin/
```

# Usage

## Command Line Interface

```
$ ntrnm --help
```

# License

This crate is licensed under either of

 * [GPLv3 license](https://opensource.org/license/gpl-3-0)

at your option.

## Contribution

[![][contrib-image]][contrib-link]

[//]: # (badges)

[rustc-image]: https://img.shields.io/badge/rustc-1.73+-blue.svg
[crate-image]: https://img.shields.io/crates/v/ntrnm.svg
[crate-link]: https://crates.io/crates/ntrnm
[docs-image]: https://docs.rs/ntrnm/badge.svg
[docs-link]: https://docs.rs/ntrnm
[deps-image]: https://deps.rs/repo/github/inmes-dev/ntrnm/status.svg
[deps-link]: https://deps.rs/repo/github/inmes-dev/ntrnm
[contrib-image]: https://contrib.rocks/image?repo=inmes-dev/ntrnm
[contrib-link]: https://github.com/inmes-dev/ntrnm/graphs/contributors
