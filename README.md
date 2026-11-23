# FACES
# CPU

A custom CPU project combining FPGA development and a Rust-based CPU emulator.

## Structure

- `ffpga/` — FPGA development files
- `firmware/` — Firmware
- `shrike_gen/` — Shrike project tooling
- `sim/` — Simulation and testbenches
- `src/` — Rust CPU emulator

## Build

```bash
cargo build
cargo test
```

## Hardware

Target: Shrike Lite FPGA
