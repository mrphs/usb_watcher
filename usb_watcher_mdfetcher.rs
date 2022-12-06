use std::fs::OpenOptions;
use std::io::prelude::*;
use std::time::SystemTime;
use usb::{Device, Result};

// This function logs the insertion of a USB device
fn log_device_insertion(log_file: &str, device: &Device) -> Result<()> {
    // Open the log file in append mode
    let mut log = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_file)?;

    // Get the current date and time as a human-readable string
    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);

    // Get the hardware and firmware information for the USB device
    let device_info = format!("{:?}", device);

    // Write a log message with the metadata about the USB device
    writeln!(log, "{}: {}", timestamp, device_info)?;

    Ok(())
}

fn main() -> Result<()> {
    // Set the log file location
    let log_file = "/var/log/usb_device_metadata.log";

    // Log the insertion of any USB device
    for device in Device::list()? {
        log_device_insertion(log_file, &device)?;
    }

    Ok(())
}
