use crate::{random::random_between, ev::{emit, new_delay}};
use evdev::{Device, EventType, InputEvent, uinput::{VirtualDevice, VirtualDeviceBuilder}};
use std::{thread::sleep, time::{Duration, SystemTime}};

mod ev;
mod error;
mod random;

const DEFAULT_POLLING_INTERVAL_MS: u64 = 8;

/// Creates a virtual device and initializes it with keys of an existing physical device.
pub fn init_uinput(device: &Device) -> error::Result<VirtualDevice> {
    let keys = device.supported_keys().ok_or(error::Error::NoSupportedKeysError)?;

    let virtual_device = VirtualDeviceBuilder::new()?
    .name("Virtual Device")
    .with_keys(keys)?
    .build()?;

    Ok(virtual_device)
}

/// Fetches events from the kernel ring buffer and writes them to a uinput device.
/// Inserts random delays before release events where `max_delay` is the maximum delay.
pub fn emit_delay(device: &mut Device, max_delay: u64, verbose: bool) -> error::Result<()> {
    let mut virtual_device = init_uinput(device)?;
    let random_delay = random_between(0, max_delay);

    for ev in device.fetch_events()? {
        let event = new_delay(ev.event_type(), ev.code(), ev.value(), random_delay);
        // TODO: evaluate the necessarity of sleeping
        sleep(Duration::from_millis(DEFAULT_POLLING_INTERVAL_MS));

        // Ignore events other than EV_KEY
        if ev.event_type() != EventType::KEY {
            continue
        }

        match ev.value() {
            0 => emit(&mut virtual_device, &[event]),
            1 => {
                // TODO: implement rescue keys
                emit(&mut virtual_device, &[event])
            }
            _ => continue
        }

        if verbose {
            let time = ev.timestamp()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("system time is before unix epoch");

            println!("Released event at time: {}    Scheduled delay: {} ms", time.as_millis(), random_delay);
            println!("Type: {:?}    Code: {}    Value: {}", ev.event_type(), ev.code(), ev.value())
        }
    }

    Ok(())
}
