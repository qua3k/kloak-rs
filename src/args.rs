use clap::{App, Arg};
use std::process;

/// List of command-line arguments the program takes
pub fn get_args() -> (&str, &str, bool) {
    let matches = App::new("kloak-rs")
    .arg(
        Arg::with_name("input")
        .required(true)
        .short("r")
        .long("input")
        .value_name("FILE")
        .help("Device file to read events from"),
    )
    .arg(
        Arg::with_name("delay")
        .short("d")
        .long("delay")
        .value_name("INTEGER")
        .help("Maximum delay (milliseconds) of released events. Default 100."),
    )
    .arg(
        Arg::with_name("verbose")
        .short("v")
        .long("verbosity")
        .help("Control the verbosity of the output"),
    )
    .get_matches();

    let input = match matches.value_of("input") {
        Some(i) => i,
        None => "none"
    };

    let max_delay = match matches.value_of("delay") {
        Some(d) => match d.parse::<u64>() {
            Ok(d) => d,
            Err(e) => {
                eprintln!("{} is not a u64!", e);
                process::exit(1)
            }
        },
        // Defaults to a max delay of 100ms
        // Will never error
        None => "100".parse::<u64>().unwrap()
    };

    let verbose = matches.is_present("verbose");

    (input, max_delay, verbose)
}