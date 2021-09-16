use evdev::Device;

pub fn print_input(mut device: &Device) {
    loop {
        match device.fetch_events() {
            Ok(iterator) => {
                for ev in iterator {
                    println!("Type: {:?}    Code: {}    Value: {}", ev.event_type(), ev.code(), ev.value())
                }
            }
            Err(e) => {
                eprintln!("fetching events failed: {}", e);
                process::exit(1)
            }
        }
    }
}