use evdev::Device;
use eventcap::print_input;
use nix::unistd::Uid;
use std::process;

fn usage() {
    eprintln!("Usage:
    eventcap [DEVICE]

DEVICE is the device file that eventcap reads from.

Example:
    $ eventcap /dev/input/event4
    Reading from /dev/input/event4 (AT Translated Set 2 keyboard)
    Type: MISC    Code: 4    Value: 15
    Type: KEY     Code: 15   Value: 0");
    process::exit(1)
}

fn main() {
    let args: Vec<String> = env::args().collect();
   
    if args.len() == 1 {
        usage()
    }

    let path = &args[1];
    
    if !Uid::current().is_root() {
        println!("You are not root! This may not work...")
    }

    let device = match Device::open(path) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("could not open device: {}", e);
            process::exit(1)
        }
    };
    
    let name = match Device::name(&device) {
        Some(n) => n,
        None => "(no name)"
    };

    println!("Reading from {} {}", path, name);
    print_input(device)
}
