// Copyright 2025 Sergey Golovin
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use anyhow::Result;
use serialport::SerialPort;
use std::io::{Read, Write};
use std::time::Duration;

/// List all available serial ports
pub fn list_ports() -> Result<()> {
    let ports = serialport::available_ports()?;

    if ports.is_empty() {
        println!("No serial ports found");
        return Ok(());
    }

    println!("Available serial ports:");
    for port in ports {
        match port.port_type {
            serialport::SerialPortType::UsbPort(info) => {
                println!(
                    "  {} - USB (VID: {:04X}, PID: {:04X})",
                    port.port_name, info.vid, info.pid
                );
                if let Some(manufacturer) = info.manufacturer {
                    println!("    Manufacturer: {}", manufacturer);
                }
                if let Some(product) = info.product {
                    println!("    Product: {}", product);
                }
            }
            serialport::SerialPortType::PciPort => {
                println!("  {} - PCI", port.port_name);
            }
            serialport::SerialPortType::BluetoothPort => {
                println!("  {} - Bluetooth", port.port_name);
            }
            _ => {
                println!("  {}", port.port_name);
            }
        }
    }

    Ok(())
}

/// Open a serial port with specified parameters
pub fn open_port(
    port_name: &str,
    baud_rate: u32,
    data_bits: u8,
    stop_bits: u8,
) -> Result<Box<dyn SerialPort>> {
    let mut port = serialport::new(port_name, baud_rate)
        .data_bits(parse_data_bits(data_bits)?)
        .stop_bits(parse_stop_bits(stop_bits)?)
        .parity(serialport::Parity::None)
        .flow_control(serialport::FlowControl::None)
        .timeout(Duration::from_millis(100))
        .open()?;

    // Clear any pending data
    let _ = port.clear(serialport::ClearBuffer::All);

    Ok(port)
}

fn parse_data_bits(bits: u8) -> Result<serialport::DataBits> {
    match bits {
        5 => Ok(serialport::DataBits::Five),
        6 => Ok(serialport::DataBits::Six),
        7 => Ok(serialport::DataBits::Seven),
        8 => Ok(serialport::DataBits::Eight),
        _ => Err(anyhow::anyhow!("Invalid data bits: {}. Must be 5, 6, 7, or 8", bits)),
    }
}

fn parse_stop_bits(bits: u8) -> Result<serialport::StopBits> {
    match bits {
        1 => Ok(serialport::StopBits::One),
        2 => Ok(serialport::StopBits::Two),
        _ => Err(anyhow::anyhow!("Invalid stop bits: {}. Must be 1 or 2", bits)),
    }
}

/// Read from serial port
pub fn read_from_port(port: &mut dyn SerialPort, buffer: &mut [u8]) -> Result<usize> {
    Ok(port.read(buffer)?)
}

/// Write to serial port
pub fn write_to_port(port: &mut dyn SerialPort, data: &[u8]) -> Result<()> {
    port.write_all(data)?;
    port.flush()?;
    Ok(())
}
