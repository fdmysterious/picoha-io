# PicoHA (Pico Host Adapter)

Simple host adapter based on RP2040 (RaspberryPi Pico) for the various application:

- IO control

Each application need its specific firmware.

## Dependencies

```bash
# Install rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# install target specifics
rustup target install thumbv6m-none-eabi
cargo install elf2uf2-rs
```

On Ubuntu

```bash
# You will need udev libc
sudo apt-get install libudev-dev

# If you need to load udev rules
sudo cp tools/99-aardvark-pico-clone.rules  /etc/udev/rules.d/
```

## Load a firmware

```bash
# !!! before plugin the pico, keep the bootsel press to enable the bootloader !!!

# Build and load the firmware
cargo run --release
```

## Pinout

The pinout of the board

![](img/raspberry-pi-pico-pinout.png)

