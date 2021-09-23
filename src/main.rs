use crate::args::get_args;
use evdev::Device;
use kloak::emit_delay;
use std::process;

mod args;

fn main() {
    let (input, max_delay, verbose) = get_args();

    let mut device = match Device::open(input) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("could not open device: {}", e);
            process::exit(1)
        }
    };
    
    // Grab the device to receive all events
    // This has not landed in stable yet, so we cannot use it
    /*
    if let Err(e) = device.grab() {
        eprintln!("grabbing device failed: {}", e);
        process::exit(1)
    } 
    */

    loop {
        if let Err(e) = emit_delay(&device, max_delay, verbose) {
            eprintln!("fetching events failed: {}", e);
            process::exit(1)
        }
    }
}