use crate::args::get_args;
use evdev::Device;
use kloak::emit_delay;
use std::process;

mod args;

fn main() {
    let (input, max_delay, verbose) = get_args();

    let device = match Device::open(input) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("could not open device: {}", e);
            process::exit(1)
        }
    };

    emit_delay(device, max_delay, verbose)
}