## ESP32-C3 Lighting Controller Example

This is an example of using my [Lighting Controller](https://github.com/kiyoshigawa/lighting_controller) code with an ESP32-C3 microcontroller. The code is set up such that there is a strip of 16 WS2812 LEDs attached to GPIO pin 9 of the ESP32-C3.

The ESP32-C3 uses the [espflash](https://github.com/esp-rs/espflash) utility to upload the code to the board over USB. This can be installed easily using the `cargo install espflash` command.

You will also need to install the `riscv32imc-unknown-none-elf` target for using the `rustup target add riscv32imc-unknown-none-elf` command in order to compile for the board.

From there, `cargo run --release` should result in the code being compiled and uploaded to the serial port that your board is connected to, and remaining connected in monitor mode. If multiple serial ports are found, espflash should ask you to select a port at upload time.
