# rust_ESP32-C3
Embedded Rust programs for the ESP32-C3. Demonstrating practical applications and learning resources. 

## How to run the programs?
- Install / prepare the prerequisites.
- Connect the developer board to your computer.
- Move into the required project.
- Use cargo to run the program - [Run Guide](https://doc.rust-lang.org/book/ch14-01-release-profiles.html)
    - Run command to run in dev profile - cargo run
    - Run command to run in release profile - cargo run --release

## Prerequisites

### Hardware
- The ESP32-C3-DevKitM-1 - [Hardware Guide](https://docs.espressif.com/projects/arduino-esp32/en/latest/boards/ESP32-C3-DevKitM-1.html)
- Other peripherals

### Software Installations
- Install "The Rust Programming Language" - [Installation Guide](https://rust-lang.github.io/rustup/installation/index.html)

### rustup Setup
- Run command - rustup target add riscv32imc-unknown-none-elf

### cargo Installations
- Run command - cargo install espflash

### Official Guide
- For more information - [Guide](https://docs.esp-rs.org/book/installation/index.html)

### Components Explored
- [ ] GPIO
- [ ] ADCs
- [ ] Timers and Counters
- [ ] PWM
- [ ] Serial Communication
