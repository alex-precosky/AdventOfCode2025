# AdventOfCode2025

GitHub Action Status: ![rust](https://github.com/alex-precosky/AdventOfCode2025/actions/workflows/rust.yml/badge.svg)


My solutions to advent of code 2025, my second year doing it in Rust.

Development was done in a mix of Windows Subsystem for Linux 2 on Windows 11 and macOS.

# Requirements

A recentish rust toolchain. ~1.90 was used.

https://www.rust-lang.org/tools/install for hints on installation.

Any modern Linux, macOS, or Windows OS ought work.

# Run

Each solution is in a cargo project named after what day's problem that solution
is for. To run the day 1 solution, simply run:

```
cargo run day01
```

# Testing

From the project directory, run the unit tests for a day with:

```
cargo test day01
```
