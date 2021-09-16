use evdev::Device;

pub fn print_input(mut device: &Device) {
    loop {
        // TODO: Get rid of unwrap
        for ev in device.fetch_events().unwrap() {
            println!("Type: {:?}    Code: {}    Value: {}", ev.event_type(), ev.code(), ev.value())
        }
    }
}