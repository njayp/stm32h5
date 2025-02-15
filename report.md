# Mid-Project Report: Successfully Flashing STM32H5 with Rust and rs-probe

## Overview
This report details the progress made in flashing an STM32H5 microcontroller using Rust and `rs-probe`. The specific chip used in this project is the STM32H563ZITx, and the development environment is set up on a Mac M3. The goal was to establish a reliable workflow for building and flashing firmware to the microcontroller using Rust tools.

## Environment Setup
The development environment was configured with the following components:
- **Hardware**: STM32H563ZITx development board
- **Host Machine**: Mac M3
- **Toolchain**: Rust with `thumbv8m.main-none-eabihf` target
- **Flashing Utility**: `rs-probe`

### Configuration
The `.cargo/config.toml` file was set up with the following parameters:
```toml
[build]
target = "thumbv8m.main-none-eabihf"
```

The `Embed.toml` file was set up with the following parameters:
```toml
[default.general]
chip = "STM32H563ZITx"

[default.flashing]
enabled = true
```

## Flashing Process
1. **Building the Firmware**
   - The firmware was compiled using the appropriate Rust target for ARM Cortex-M.
   - A minimal Rust application was written to test the build process.
   - Embassy was used for convenience and memory safety to build a LED blinker (https://github.com/njayp/stm32h5)
   
2. **Connecting to the Board**
   - The board was connected via USB.
   - `rs-probe` successfully detected the STM32H5 chip.

3. **Flashing the Firmware**
   - `rs-probe` was used to flash the compiled binary onto the chip.
   - The process completed without errors, and the firmware was verified to be running.
   - The command `cargo embed --release` was used.

## Challenges and Solutions
### 1. **Initial Connection Issues**
- Issue: The board was not initially recognized by `rs-probe`.
- Solution: Ensured correct USB drivers and permissions were set up on macOS.

### 2. **Target Configuration**
- Issue: The correct Rust target needed to be set explicitly.
- Solution: Specified `thumbv8m.main-none-eabihf` in `.cargo/config.toml`.

### 3. **Flashing Stability**
- Issue: Inconsistent flashing due to power fluctuations.
- Solution: Used a powered USB hub to ensure stable power delivery.

## Next Steps
- Expand the firmware to include peripheral interactions (ADC) and accelerators (FFT).
- Optimize debugging workflow with `probe-rs` for better logging.

## Conclusion
The mid-project milestone of successfully flashing an STM32H5 microcontroller using Rust and `rs-probe` has been achieved. The setup is now stable, and further development can proceed with confidence.

