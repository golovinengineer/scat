# Scat - Serial CAT Development Plan

A cross-platform CLI serial port tool for hardware developers. Similar to tio, focused on connecting to microcontroller-based devices via USB-UART adapters.

## Project Overview

**Name**: Scat (Serial CAT)
**Language**: Rust
**Platforms**: Linux, macOS, Windows
**Target Users**: Hardware developers working with microcontrollers

---

## Phase 1: MVP - Core Serial Communication

### Step 1.1: Project Setup & Dependencies

- [x] Initialize Cargo project
- [x] Add dependencies: `serialport`, `clap`, `anyhow`
- [x] Create basic project structure:
  - `src/main.rs` - CLI entry point
  - `src/serial.rs` - Serial port communication
  - `src/ui.rs` - Terminal UI components

**Expected Output**: Project compiles with all dependencies

---

### Step 1.2: CLI Argument Parsing

- [ ] Implement CLI argument handling using `clap` for:
  - Port selection (e.g., `/dev/ttyUSB0`, `COM3`)
  - Baud rate (default 115200, configurable)
  - Data bits (default 8)
  - Stop bits (default 1)
  - Parity (default none)
- [ ] Add `--help` and `--version` support
- [ ] Add config file support (optional in MVP)

**Expected Output**: `scat --help` shows all options;
`scat /dev/ttyUSB0 -b 115200` parses correctly

---

### Step 1.3: Basic Serial Port Opening & Closure

- [ ] Implement `SerialPort` struct to wrap serialport crate
- [ ] Handle port opening with proper error messages
- [ ] Graceful shutdown (Ctrl+C handling)
- [ ] Test on actual hardware or virtual serial port

**Expected Output**: Program opens/closes ports cleanly;
informative error if port not found

---

### Step 1.4: Read from Serial Port

- [ ] Spawn background thread to read from serial port
- [ ] Display incoming data to stdout
- [ ] Handle disconnections gracefully
- [ ] Show received data in hex/ASCII format (configurable)

**Expected Output**: Connect to device, see real-time data stream

---

### Step 1.5: Write to Serial Port

- [ ] Read user input from stdin
- [ ] Send input to serial port
- [ ] Handle line endings (LF, CR, CRLF - configurable)
- [ ] Optionally echo sent data to screen

**Expected Output**: Type commands and receive responses from device

---

## Phase 2: Enhanced UX & Features

### Step 2.1: Status Bar / Info Display

- [ ] Show port name, baud rate, connection status
- [ ] Display connected/disconnected state with visual indicator
- [ ] Show data statistics (bytes sent/received)

**Expected Output**: Status line at bottom of terminal showing connection info

---

### Step 2.2: Local Echo & Line Control

- [ ] Toggle local echo on/off
- [ ] Configurable line endings on send
- [ ] Show/hide sent characters
- [ ] Handle backspace and delete properly

**Expected Output**: Terminal behaves like minicom with proper input handling

---

### Step 2.3: Hexdump / Raw View Modes

- [ ] Toggle between plain text and hexdump view
- [ ] Show both ASCII and hex simultaneously
- [ ] Color-code received vs sent data
- [ ] Timestamp each line (optional)

**Expected Output**: Key command to switch between views in real-time

---

### Step 2.4: Reconnection Logic

- [ ] Auto-detect port disconnection
- [ ] Retry connection with exponential backoff
- [ ] Notify user of reconnection attempts
- [ ] Optional auto-reconnect toggle

**Expected Output**: Pull USB cable and device reconnects cleanly when plugged back

---

## Phase 3: Developer Quality of Life

### Step 3.1: Logging / Session Recording

- [ ] Save session to file (optional)
- [ ] Log only output, or both input & output
- [ ] Rotating logs or single-session logs
- [ ] Timestamp each logged line

**Expected Output**: `--log session.log` creates session transcript

---

### Step 3.2: Macro / Shortcut Commands

- [ ] Define quick commands in config file
- [ ] Hotkeys to send predefined strings
- [ ] Environment variable expansion
- [ ] Newline handling in macros

**Expected Output**: Press Ctrl+S to send "reset\n" or similar

---

### Step 3.3: Search & Filtering

- [ ] Search/grep received data in real-time
- [ ] Highlight matching lines
- [ ] Filter to show only matching lines
- [ ] Case-sensitive/insensitive toggle

**Expected Output**: Ctrl+F to search, highlight matches

---

### Step 3.4: Port Discovery / Auto-Connect

- [ ] List available serial ports with info
- [ ] Show USB VID:PID for identification
- [ ] Auto-connect to known device
- [ ] Handle multiple identical devices gracefully

**Expected Output**: `scat --list` shows ports; `scat --find "CH340"` connects automatically

---

## Good-to-Have Features

### Performance & Stability

- [ ] Handle high baud rates (>1MHz) without data loss
- [ ] Buffered read/write for performance
- [ ] Configurable buffer sizes
- [ ] Memory usage monitoring

### Terminal Features

- [ ] Custom color schemes / themes
- [ ] Configurable key bindings
- [ ] Vi-mode and Emacs-mode support
- [ ] Mouse support for selection/scrolling

### Advanced Serial Features

- [ ] RTS/CTS flow control toggle
- [ ] DTR/DSR line control
- [ ] XON/XOFF software flow control
- [ ] Send/receive break signals
- [ ] Serial line state monitoring (CD, RI, etc.)

### Cross-Platform Enhancements

- [ ] Native Windows console color support
- [ ] Linux TTY/PTY handling
- [ ] macOS specific optimizations
- [ ] USB CDC device detection

### Developer Convenience

- [ ] YAML config file support
- [ ] Multiple saved profiles (per device)
- [ ] Integration with common dev boards (Arduino, ESP32 presets)
- [ ] Plugin/script support for custom processing
- [ ] Integration with tmux/screen

### Documentation & UX

- [ ] Built-in man page
- [ ] Interactive tutorial on first run
- [ ] Command cheat-sheet
- [ ] Dark/light mode themes
- [ ] Accessibility features (high contrast mode)

### Testing & Validation

- [ ] Unit tests for serial communication
- [ ] Integration tests with virtual serial ports
- [ ] Stress tests for data integrity
- [ ] Performance benchmarks

---

## Technical Decisions

### Libraries

- **Serial Communication**: `serialport` crate (cross-platform, well-maintained)
- **CLI Args**: `clap` (ergonomic, generates help automatically)
- **TUI**: Consider `crossterm` or `termion` for terminal control if adding advanced UI
- **Error Handling**: `anyhow` for error propagation

### Architecture

- Single-threaded main with read/write on separate OS-level threads
- Event-based model for commands (Ctrl+C, etc.)
- Modular design for easy feature addition

### Testing Strategy

- Test with real USB-UART adapters (CH340, FT232)
- Test with virtual serial ports (socat on Linux)
- Verify on Linux (Fedora/Ubuntu), macOS, Windows

---

## Success Criteria

**MVP Complete**: Tool can open port, send/receive data, display properly
**Phase 2 Complete**: Professional UX with status, echo control, view modes
**Phase 3 Complete**: Power-user features for serious hardware developers

---

## Known Challenges

1. **Cross-platform serial port access** - Handled by `serialport` crate
2. **Terminal control** - May need platform-specific code for optimal UX
3. **Unicode/line encoding** - Serial data is bytes, need flexible handling
4. **Port discovery** - USB enumeration differs per OS
5. **Real-time performance** - Ensure no dropped bytes at high baud rates
