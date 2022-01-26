# TinyUF2 flash unlocker for STM32F411 (easily adaptable to other STM32F4xx)

This small program is meant to unlock the first 4 (=64kb) flash sectors that are occupied by the TinyUF2 bootloader, if they have been locked by it. After the unlock procedure it jumps to the built-in DFU bootloader of the STM32F4 MCUs. This is useful for e.g. self-updating the TinyUF2 bootloader or replacing it with something else.

## Usage

1. Put your TinyUF2 bootloader into DFU mode, this is usually done by double tapping reset.
2. **Warning this will overwrite your currently flashed program** copy `tinyuf2-unlocker-stm32f411.uf2` image into mass storage device that pops up.
3. The mass storage device will disappear and a new STM32 DFU device appear.
4. Flash a new version of TinyUF2 or another image via `dfu-util` or similar.

## Pre-built

Download the `tinyuf2-unlocker-stm32f411.uf2` UF2 image from the releases section.

## Building from Source

1. Install Rust via https://rustup.rs/
2. Add ARM-Thumbv7 toolchain `rustup target add thumbv7em-none-eabi`
3. Install Cargo binutils https://github.com/rust-embedded/cargo-binutils
4. Compile the binary to a Hex file `cargo objcopy --release -- -O ihex tinyuf2-unlocker-stm32f411.hex`
5. Get the `uf2conv` converter from https://github.com/microsoft/uf2/tree/master/utils
6. Convert the Hex file to a valid UF2 image `uf2conv.py tinyuf2-unlocker-stm32f411.hex --family STM32F4 --output tinyuf2-unlocker-stm32f411.uf2`
