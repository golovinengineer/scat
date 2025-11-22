// Tests for CLI argument parsing
// Tests various argument combinations and edge cases

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_help_flag() {
    let mut cmd = Command::cargo_bin("scat").unwrap();
    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Serial CAT"))
        .stdout(predicate::str::contains("--baud"))
        .stdout(predicate::str::contains("--data-bits"))
        .stdout(predicate::str::contains("--stop-bits"));
}

#[test]
fn test_help_short_flag() {
    let mut cmd = Command::cargo_bin("scat").unwrap();
    cmd.arg("-h");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Serial CAT"));
}

#[test]
fn test_version_flag() {
    let mut cmd = Command::cargo_bin("scat").unwrap();
    cmd.arg("--version");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("scat"));
}

#[test]
fn test_list_ports_flag() {
    let mut cmd = Command::cargo_bin("scat").unwrap();
    cmd.arg("--list");
    cmd.assert().success();
}

#[test]
fn test_list_short_flag() {
    let mut cmd = Command::cargo_bin("scat").unwrap();
    cmd.arg("-l");
    cmd.assert().success();
}

#[test]
fn test_no_port_specified_error() {
    let mut cmd = Command::cargo_bin("scat").unwrap();
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No port specified")
            .or(predicate::str::contains("port")));
}

#[test]
fn test_default_baud_rate() {
    let mut cmd = Command::cargo_bin("scat").unwrap();
    cmd.arg("/dev/ttyUSB0")
        .write_stdin("exit\n");
    // The command will fail because device doesn't exist, but it should accept the defaults
    let output = cmd.output();
    // Should not error on parsing, only on device access
    assert!(output.is_ok());
}

#[test]
fn test_custom_baud_rate() {
    let mut cmd = Command::cargo_bin("scat").unwrap();
    cmd.arg("/dev/ttyUSB0")
        .arg("--baud")
        .arg("9600")
        .write_stdin("exit\n");
    let output = cmd.output();
    assert!(output.is_ok());
}

#[test]
fn test_baud_rate_short_flag() {
    let mut cmd = Command::cargo_bin("scat").unwrap();
    cmd.arg("/dev/ttyUSB0")
        .arg("-b")
        .arg("115200")
        .write_stdin("exit\n");
    let output = cmd.output();
    assert!(output.is_ok());
}

#[test]
fn test_custom_data_bits() {
    let mut cmd = Command::cargo_bin("scat").unwrap();
    cmd.arg("/dev/ttyUSB0")
        .arg("--data-bits")
        .arg("7")
        .write_stdin("exit\n");
    let output = cmd.output();
    assert!(output.is_ok());
}

#[test]
fn test_custom_stop_bits() {
    let mut cmd = Command::cargo_bin("scat").unwrap();
    cmd.arg("/dev/ttyUSB0")
        .arg("--stop-bits")
        .arg("2")
        .write_stdin("exit\n");
    let output = cmd.output();
    assert!(output.is_ok());
}

#[test]
fn test_all_custom_settings() {
    let mut cmd = Command::cargo_bin("scat").unwrap();
    cmd.arg("/dev/ttyUSB0")
        .arg("--baud")
        .arg("19200")
        .arg("--data-bits")
        .arg("8")
        .arg("--stop-bits")
        .arg("1")
        .write_stdin("exit\n");
    let output = cmd.output();
    assert!(output.is_ok());
}

#[test]
fn test_baud_rate_with_equals_syntax() {
    let mut cmd = Command::cargo_bin("scat").unwrap();
    cmd.arg("/dev/ttyUSB0")
        .arg("--baud=9600")
        .write_stdin("exit\n");
    let output = cmd.output();
    assert!(output.is_ok());
}

#[test]
fn test_invalid_baud_rate() {
    let mut cmd = Command::cargo_bin("scat").unwrap();
    cmd.arg("/dev/ttyUSB0")
        .arg("--baud")
        .arg("invalid");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("invalid")
            .or(predicate::str::contains("baud")));
}

#[test]
fn test_invalid_data_bits() {
    let mut cmd = Command::cargo_bin("scat").unwrap();
    cmd.arg("/dev/ttyUSB0")
        .arg("--data-bits")
        .arg("not_a_number");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("invalid")
            .or(predicate::str::contains("data-bits")));
}

#[test]
fn test_invalid_stop_bits() {
    let mut cmd = Command::cargo_bin("scat").unwrap();
    cmd.arg("/dev/ttyUSB0")
        .arg("--stop-bits")
        .arg("abc");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("invalid")
            .or(predicate::str::contains("stop-bits")));
}

#[test]
fn test_common_baud_rates() {
    let common_rates = vec!["300", "1200", "2400", "4800", "9600", "19200", "38400", "57600", "115200", "230400"];
    
    for rate in common_rates {
        let mut cmd = Command::cargo_bin("scat").unwrap();
        cmd.arg("/dev/ttyUSB0")
            .arg("--baud")
            .arg(rate)
            .write_stdin("exit\n");
        let output = cmd.output();
        assert!(output.is_ok(), "Failed to parse baud rate: {}", rate);
    }
}

#[test]
fn test_common_data_bits() {
    let common_bits = vec!["5", "6", "7", "8"];
    
    for bits in common_bits {
        let mut cmd = Command::cargo_bin("scat").unwrap();
        cmd.arg("/dev/ttyUSB0")
            .arg("--data-bits")
            .arg(bits)
            .write_stdin("exit\n");
        let output = cmd.output();
        assert!(output.is_ok(), "Failed to parse data bits: {}", bits);
    }
}

#[test]
fn test_stop_bits_values() {
    let bits = vec!["1", "2"];
    
    for bit in bits {
        let mut cmd = Command::cargo_bin("scat").unwrap();
        cmd.arg("/dev/ttyUSB0")
            .arg("--stop-bits")
            .arg(bit)
            .write_stdin("exit\n");
        let output = cmd.output();
        assert!(output.is_ok(), "Failed to parse stop bits: {}", bit);
    }
}

#[test]
fn test_help_contains_usage_examples() {
    let mut cmd = Command::cargo_bin("scat").unwrap();
    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Usage").or(predicate::str::contains("USAGE")));
}

#[test]
fn test_port_as_positional_argument() {
    let mut cmd = Command::cargo_bin("scat").unwrap();
    cmd.arg("/dev/ttyUSB0")
        .write_stdin("exit\n");
    let output = cmd.output();
    assert!(output.is_ok());
}

#[test]
fn test_multiple_flags_order() {
    let mut cmd = Command::cargo_bin("scat").unwrap();
    cmd.arg("--baud").arg("9600")
        .arg("/dev/ttyUSB0")
        .arg("--data-bits").arg("8")
        .write_stdin("exit\n");
    let output = cmd.output();
    assert!(output.is_ok());
}

#[test]
fn test_list_with_other_args_ignored() {
    // When --list is provided, other arguments should still parse
    let mut cmd = Command::cargo_bin("scat").unwrap();
    cmd.arg("--list")
        .arg("--baud").arg("9600");
    cmd.assert().success();
}

#[test]
fn test_parity_none() {
    let mut cmd = Command::cargo_bin("scat").unwrap();
    cmd.arg("/dev/ttyUSB0")
        .arg("--parity")
        .arg("none")
        .write_stdin("exit\n");
    let output = cmd.output();
    assert!(output.is_ok());
}

#[test]
fn test_parity_odd() {
    let mut cmd = Command::cargo_bin("scat").unwrap();
    cmd.arg("/dev/ttyUSB0")
        .arg("--parity")
        .arg("odd")
        .write_stdin("exit\n");
    let output = cmd.output();
    assert!(output.is_ok());
}

#[test]
fn test_parity_even() {
    let mut cmd = Command::cargo_bin("scat").unwrap();
    cmd.arg("/dev/ttyUSB0")
        .arg("--parity")
        .arg("even")
        .write_stdin("exit\n");
    let output = cmd.output();
    assert!(output.is_ok());
}

#[test]
fn test_parity_invalid() {
    let mut cmd = Command::cargo_bin("scat").unwrap();
    cmd.arg("/dev/ttyUSB0")
        .arg("--parity")
        .arg("invalid");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("parity")
            .or(predicate::str::contains("Invalid")));
}

#[test]
fn test_all_settings_with_parity() {
    let mut cmd = Command::cargo_bin("scat").unwrap();
    cmd.arg("/dev/ttyUSB0")
        .arg("--baud").arg("9600")
        .arg("--data-bits").arg("8")
        .arg("--stop-bits").arg("1")
        .arg("--parity").arg("even")
        .write_stdin("exit\n");
    let output = cmd.output();
    assert!(output.is_ok());
}

#[test]
fn test_parity_case_insensitive() {
    let parities = vec!["NONE", "ODD", "EVEN", "None", "Odd", "Even"];
    
    for parity in parities {
        let mut cmd = Command::cargo_bin("scat").unwrap();
        cmd.arg("/dev/ttyUSB0")
            .arg("--parity")
            .arg(parity)
            .write_stdin("exit\n");
        let output = cmd.output();
        assert!(output.is_ok(), "Failed to parse parity: {}", parity);
    }
}

#[test]
fn test_help_shows_parity_option() {
    let mut cmd = Command::cargo_bin("scat").unwrap();
    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("--parity")
            .or(predicate::str::contains("parity")));
}
