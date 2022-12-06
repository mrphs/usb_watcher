use std::fs::OpenOptions;
use std::io::prelude::*;
use std::sync::Arc;
use std::time::SystemTime;
use std::thread;
use usb::{Device, Result};
use notify_rust::Notification;

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

    // Set up a watcher for USB device events
    let mut watcher = Notification::new()?;
    watcher.watch("/dev")?;

    // Create a shared Arc to the watcher, so that it can be accessed by multiple threads
    let watcher = Arc::new(watcher);

    // Spawn a thread to handle USB device events
    let watcher_clone = watcher.clone();
    thread::spawn(move || {
        // Loop indefinitely, waiting for USB device events
        loop {
            // Get the next USB device event
            let event = watcher_clone.recv().unwrap();

            // If the event is a device insertion, log the metadata for the device
            if event.event_mask.contains(notify_rust::EventKind::CREATE) {
                for device in Device::list().unwrap() {
                    log_device_insertion(log_file, &device).unwrap();
                }
            }
        }
    });

    // Wait for the watcher thread to finish
    watcher.join().unwrap();

    Ok(())
}
