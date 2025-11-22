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
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::execute;
use std::io::{self, Read, Write};
use std::thread;
use std::sync::{Arc, mpsc};
use std::time::Duration;

use crate::serial;

pub fn run(port_name: &str, baud_rate: u32, data_bits: u8, stop_bits: u8) -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    // Open serial port
    let mut serial_port = serial::open_port(port_name, baud_rate, data_bits, stop_bits)?;
    
    println!("Connected to {} at {} baud", port_name, baud_rate);
    println!("Press Ctrl+C to exit");
    println!("");

    // Placeholder: In a full implementation, this would:
    // - Spawn a thread to read from serial port
    // - Read from stdin for user input
    // - Display incoming data
    // - Handle terminal input (Ctrl+C, etc.)
    
    thread::sleep(Duration::from_secs(2));

    // Cleanup
    disable_raw_mode()?;
    execute!(stdout, LeaveAlternateScreen)?;

    Ok(())
}
