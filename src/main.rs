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
use clap::Parser;

mod serial;
mod ui;

#[derive(Parser, Debug)]
#[command(name = "scat")]
#[command(about = "Serial CAT - A cross-platform CLI serial port tool for hardware developers", long_about = None)]
#[command(version)]
struct Args {
    /// Serial port device path (e.g., /dev/ttyUSB0, COM3)
    port: Option<String>,

    /// Baud rate
    #[arg(short, long, default_value = "115200")]
    baud: u32,

    /// Data bits
    #[arg(long, default_value = "8")]
    data_bits: u8,

    /// Stop bits
    #[arg(long, default_value = "1")]
    stop_bits: u8,

    /// List available serial ports
    #[arg(short, long)]
    list: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    if args.list {
        serial::list_ports()?;
        return Ok(());
    }

    let port = args.port.ok_or_else(|| {
        anyhow::anyhow!("No port specified. Use 'scat --list' to see available ports or provide port as argument.")
    })?;

    ui::run(&port, args.baud, args.data_bits, args.stop_bits)?;

    Ok(())
}
