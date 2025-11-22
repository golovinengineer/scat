# Scat - Serial CAT

A cross-platform CLI serial port tool for hardware developers. Similar to [tio](https://github.com/tio/tio), designed for connecting to microcontroller-based devices via USB-UART adapters.

## Features (Planned)

- **Cross-platform**: Works on Linux, macOS, and Windows
- **Simple CLI**: Easy-to-use command-line interface
- **Hardware-friendly**: Designed for embedded developers
- **Real-time monitoring**: Live data streaming from serial devices
- **Multiple view modes**: Plain text, hex dump, and more
- **Flexible configuration**: Baud rate, data bits, stop bits, parity
- **Port discovery**: List and identify connected USB devices
- **Session logging**: Record serial communication sessions
- **Keyboard shortcuts**: Quick commands and macros

## Quick Start

### Build

```bash
cargo build --release
```

### Usage

List available serial ports:
```bash
./target/release/scat --list
```

Connect to a device:
```bash
./target/release/scat /dev/ttyUSB0
# or Windows:
./target/release/scat COM3
```

Specify baud rate and other parameters:
```bash
scat /dev/ttyUSB0 --baud 9600 --data-bits 8 --stop-bits 1
```

### Help

```bash
scat --help
```

## Development

See [DEVELOPMENT_PLAN.md](DEVELOPMENT_PLAN.md) for the detailed roadmap and implementation phases.

### Project Structure

- `src/main.rs` - CLI argument parsing and entry point
- `src/serial.rs` - Serial port communication logic
- `src/ui.rs` - Terminal UI and user interaction

## Dependencies

- **serialport** - Cross-platform serial port access
- **clap** - Command-line argument parsing
- **anyhow** - Error handling
- **crossterm** - Terminal manipulation
- **tokio** - Async runtime for concurrent operations

## Status

Currently in MVP development phase (Phase 1).

## License

Apache License 2.0 - See [LICENSE](LICENSE) file for details.

## Author

Sergey Golovin <golovinengineer@icloud.com>
