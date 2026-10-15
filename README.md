# R Calc

A modern, fast, and native calculator designed for the [COSMIC desktop](https://system76.com/cosmic) environment, built with Rust and [`libcosmic`](https://github.com/pop-os/libcosmic).

[![License: MPL-2.0](https://img.shields.io/badge/License-MPL_2.0-blue.svg)](https://opensource.org/licenses/MPL-2.0)
[![Rust Edition: 2024](https://img.shields.io/badge/Rust-2024_Edition-orange.svg)](https://www.rust-lang.org/)
[![Desktop: COSMIC](https://img.shields.io/badge/Desktop-COSMIC-teal.svg)](https://system76.com/cosmic)

---

## Features

- **Native COSMIC Design**: Seamlessly integrates with the COSMIC desktop theme and styling guidelines (HIG), featuring distinct action button highlights (`Destructive` for clear/backspace, `Suggested` for calculation).
- **Math Expression Engine**: Fast and reliable mathematical evaluation powered by `meval`, supporting operations with 4-decimal place rounding to avoid floating-point inaccuracies.
- **Calculation History**: Live history preview displayed above the primary visor for tracking multi-step calculations.
- **Chained Calculations**: Retain previous results when typing operators (`+`, `-`, `*`, `/`, `%`) directly after an evaluation.
- **Comprehensive Keyboard & Numpad Support**:
  - Dedicated hardware numpad mapping for fast entry on full keyboards.
  - Full primary keyboard navigation.
  - Clipboard support: Press `Ctrl + C` to copy the current display value directly to your system clipboard.
- **Fixed Layout Geometry**: Designed with a locked compact aspect ratio (340 × 600 px) ideal for tiling window managers and quick desktop access.
- **Internationalization (i18n)**: Localization ready with Project Fluent.

---

## Keyboard Shortcuts

| Key / Shortcut | Action |
| --- | --- |
| `0` - `9` | Enter digit |
| `+`, `-`, `*`, `/` | Arithmetic operators |
| `%` | Modulo / percentage operator |
| `.` or `,` | Decimal point |
| `Enter` or `=` | Calculate expression |
| `Backspace` | Delete last character (or reset error) |
| `Escape`, `Delete`, `C`, or `c` | Clear calculator and history |
| `Ctrl + C` | Copy current display value to clipboard |
| **Numpad Keys** (`0`-`9`, `+`, `-`, `*`, `/`, `.`, `Enter`) | Full numeric keypad support |

---

## Installation & Running

A [`justfile`](./justfile) is provided for common development and packaging workflows using the [`just`](https://github.com/casey/just) command runner.

### Quick Run

```sh
# Run with cargo
cargo run --release

# Or with just
just run
```

### Build

```sh
# Debug build
just build-debug

# Release build
just build-release
```

### Running Tests and Linter

```sh
# Run unit tests
cargo test

# Run pedantic Clippy checks
just check
```

### System Installation

To install `r-calc` along with its desktop file, metadata, and scalable icon:

```sh
sudo just install
```

To install with a custom prefix or root directory:

```sh
just rootdir=debian/r-calc prefix=/usr install
```

To uninstall:

```sh
sudo just uninstall
```

---

## Packaging

For Linux distribution packaging, vendor the project dependencies locally using the `vendor` recipe and compile offline with `build-vendored`:

```sh
just vendor
just build-vendored
just rootdir=debian/r-calc prefix=/usr install
```

---

## Localization (Translators)

Localization is powered by [Project Fluent](https://projectfluent.org/) through `i18n-embed`.

Translation files are located in the [`i18n/`](./i18n) directory:
- Default translations reside in [`i18n/en/r_calc.ftl`](./i18n/en/r_calc.ftl).
- To add a new language, create a new directory inside `i18n/` named with the corresponding [ISO 639-1 code](https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes) (e.g., `i18n/pt-BR/` or `i18n/de/`) and copy the `.ftl` file with translated strings.

---

## Development

- **Compiler**: Rust 2024 edition (`rustup default stable`)
- **Language Server**: Recommended setup with [rust-analyzer](https://rust-analyzer.github.io/)
- **Linker Acceleration**: Consider installing [mold](https://github.com/rui314/mold) and [sccache](https://github.com/mozilla/sccache) for faster rebuild cycles.

---

## License

This project is licensed under the [Mozilla Public License 2.0 (MPL-2.0)](./Cargo.toml).
