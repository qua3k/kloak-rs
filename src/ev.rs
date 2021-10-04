/// Functions originating from the evdev crate
use evdev::{EventType, InputEvent, uinput::VirtualDevice};
use std::process;

/// Originates from the evdev crate
/// Create a libc::timeval from the system time + a delay in milliseconds
fn systime_delay_to_timeval(time: &SystemTime, delay_ms: u64) -> libc::timeval {
    let (sign, dur) = match time.duration_since(SystemTime::UNIX_EPOCH) {
        Ok(dur) => (1, dur + Duration::from_millis(delay_ms)),
        Err(e) => (-1, e.duration()),
    };

    libc::timeval {
        tv_sec: dur.as_secs() as libc::time_t * sign,
        tv_usec: dur.subsec_micros() as libc::suseconds_t,
    }
}

/// Originates from the evdev crate
/// Create a new InputEvent with the time field set to "now + a random delay in ms" on the system clock.
///
/// Note that this isn't usually necessary simply for emitting events on a virtual device, as
/// even though [`InputEvent::new`] creates an `input_event` with the time field as zero,
/// the kernel will update `input_event.time` when it emits the events to any programs reading
/// the event "file".
pub fn new_delay(type_: EventType, code: u16, value: i32, delay_ms: u64) -> InputEvent {
    InputEvent(libc::input_event {
        time: systime_delay_to_timeval(&SystemTime::now(), delay_ms),
        type_: type_.0,
        code,
        value,
    })
}


// Handle VirtualDevice.emit() errors
pub fn emit(virtual_device: &mut VirtualDevice, ev: &[InputEvent]) {
    if let Err(e) = virtual_device.emit(ev) {
        eprintln!("could not write to device: {}", e);
        process::exit(1)
    }
}