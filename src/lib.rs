use crate::random::random_between;
use evdev::{Device, EventType, InputEvent, uinput::{VirtualDevice, VirtualDeviceBuilder}};
use std::{io, process, thread::sleep, time::{Duration, SystemTime}};

mod random;

const DEFAULT_POLLING_INTERVAL_MS: u64 = 8;

// Emit events to the virtual device
fn emit(virtual_device: &VirtualDevice, ev: &[InputEvent]) {
    if let Err(e) = virtual_device.emit(ev) {
        eprintln!("could not write to device: {}", e);
        process::exit(1)
    }
}

/// Creates a virtual device and initializes it with keys of an existing physical device.
pub fn init_uinput(device: &Device) -> io::Result<VirtualDevice> {
    let keys = match device.supported_keys() {
        Some(k) => k,
        None => {
            eprintln!("No supported keys!");
            process::exit(1)
        }
    };

    let virtual_device = VirtualDeviceBuilder::new()?
    .name("Virtual Device")
    .with_keys(keys)?
    .build()?;

    Ok(virtual_device)
}

/// Fetches events from the kernel ring buffer and writes them to a uinput device.
/// Inserts random delays before release events where max_delay is the maximum delay.
pub fn emit_delay(mut device: Device, max_delay: u64, verbose: bool) {
    let mut virtual_device = match init_uinput(&device) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("could not build virtual device: {}", e);
            process::exit(1)
        }
    };

    let random_delay = random_between(0, max_delay);
    loop {
        for ev in device.fetch_events().unwrap() {
            sleep(Duration::from_millis(DEFAULT_POLLING_INTERVAL_MS));

            if ev.event_type() != EventType::KEY {
                continue
            }

            match ev.value() {
                0 => emit(&virtual_device, &[ev]),
                1 => {
                    sleep(Duration::from_millis(random_delay));
                    // TODO: implement rescue keys
                    emit(&virtual_device, &[ev])
                },
                _ => continue
            }

            if verbose {
                let time = match ev.timestamp().duration_since(SystemTime::UNIX_EPOCH) {
                    Ok(t) => t.as_millis(),
                    Err(e) => panic!("System time is before Unix Epoch!")
                };
                println!("Released event at time: {}", time)
            }

        }
    }
}
