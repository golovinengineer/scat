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
