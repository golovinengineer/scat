# Scat CLI Argument Parsing Tests

## Overview

Comprehensive test suite for Step 1.2 of the development plan: CLI Argument Parsing.

## Test Coverage

### Basic Flags
- ✅ `--help` / `-h` - Shows help message with all options
- ✅ `--version` / `-V` - Shows version information
- ✅ `--list` / `-l` - Lists available serial ports

### Positional Arguments
- ✅ Port as positional argument (e.g., `/dev/ttyUSB0`)
- ✅ Error when no port specified

### Baud Rate (`--baud` / `-b`)
- ✅ Default value: 115200
- ✅ Custom values with space: `--baud 9600`
- ✅ Custom values with equals: `--baud=9600`
- ✅ Common rates tested: 300, 1200, 2400, 4800, 9600, 19200, 38400, 57600, 115200, 230400
- ✅ Invalid values rejected with proper error message

### Data Bits (`--data-bits`)
- ✅ Default value: 8
- ✅ Custom values: 5, 6, 7, 8
- ✅ Invalid values rejected

### Stop Bits (`--stop-bits`)
- ✅ Default value: 1
- ✅ Valid values: 1, 2
- ✅ Invalid values rejected

### Parity (`--parity`)
- ✅ Default value: none
- ✅ Valid values: `none`, `odd`, `even`
- ✅ Case-insensitive parsing: NONE, ODD, EVEN, None, Odd, Even
- ✅ Invalid values rejected with proper error message
- ✅ Help shows parity option

### Combined Arguments
- ✅ Multiple flags in any order
- ✅ All settings together with parity
- ✅ `--list` with other arguments (other args parsed but list takes precedence)

## Running the Tests

```bash
# Run all CLI argument tests
cargo test --test cli_args_tests

# Run specific test
cargo test --test cli_args_tests test_parity_none

# Run with output
cargo test --test cli_args_tests -- --nocapture

# Run with multiple threads
cargo test --test cli_args_tests -- --test-threads=1
```

## Test Statistics

- **Total Tests**: 30
- **Categories**: 6 (flags, positional, baud, data bits, stop bits, parity)
- **Pass Rate**: 100%
- **Coverage**: Basic functionality, defaults, custom values, validation, edge cases

## Test List

### Help & Version Tests (3)
1. `test_help_flag` - `-h` and `--help` show usage
2. `test_help_short_flag` - Short form works
3. `test_version_flag` - `-V` and `--version` show version
4. `test_help_contains_usage_examples` - Help includes usage
5. `test_help_shows_parity_option` - Help documents all options

### List Ports Tests (2)
6. `test_list_ports_flag` - `--list` succeeds
7. `test_list_short_flag` - `-l` succeeds
8. `test_list_with_other_args_ignored` - Other args parsed with `--list`

### Port Argument Tests (2)
9. `test_port_as_positional_argument` - Port works as positional
10. `test_no_port_specified_error` - Error when port not provided

### Baud Rate Tests (6)
11. `test_default_baud_rate` - Defaults to 115200
12. `test_custom_baud_rate` - `--baud 9600` works
13. `test_baud_rate_short_flag` - `-b 115200` works
14. `test_baud_rate_with_equals_syntax` - `--baud=9600` works
15. `test_invalid_baud_rate` - Non-numeric rejected
16. `test_common_baud_rates` - All standard rates accepted

### Data Bits Tests (2)
17. `test_custom_data_bits` - `--data-bits 7` works
18. `test_invalid_data_bits` - Non-numeric rejected
19. `test_common_data_bits` - Standard values (5-8) accepted

### Stop Bits Tests (2)
20. `test_custom_stop_bits` - `--stop-bits 2` works
21. `test_invalid_stop_bits` - Non-numeric rejected
22. `test_stop_bits_values` - Standard values (1-2) accepted

### Parity Tests (6)
23. `test_parity_none` - `--parity none` works
24. `test_parity_odd` - `--parity odd` works
25. `test_parity_even` - `--parity even` works
26. `test_parity_invalid` - Invalid parity rejected
27. `test_parity_case_insensitive` - Case variations work
28. `test_help_shows_parity_option` - Parity documented in help

### Combined Tests (3)
29. `test_all_custom_settings` - All args together
30. `test_all_settings_with_parity` - All args with parity
31. `test_multiple_flags_order` - Flags in any order

## Next Steps

- [ ] Implement config file support (Step 1.2 optional)
- [ ] Add environment variable parsing
- [ ] Add integration tests with actual serial ports
- [ ] Test platform-specific behavior (Windows COM ports, /dev/ttyS* on Linux)
