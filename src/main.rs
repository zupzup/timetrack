extern crate clap;
use clap::{App, Arg};

extern crate timetrack;
use timetrack::{clear::clear};
use timetrack::config::get_config;
use timetrack::TimeTracker;

fn main() {
    // todo add cli option to clear history
    let matches = App::new("TimeTrack")
        .arg(Arg::with_name("track")
            .short("t")
            .long("track")
            .help("Set to track, otherwise show data"))
        .arg(Arg::with_name("clear")
            .short("c")
            .long("clear")
            .help("Clear all data"))
        .get_matches();

    if matches.is_present("clear") {
        // TODO don't unwrap inside the library calls, handle errors here and exit with appropriate error message and exit code
        clear();
        return; // clear should not run track/calc after
    }

    let config = get_config();
    if matches.is_present("track") {
        TimeTracker::new(&config).track();
    } else {
        TimeTracker::new(&config).calc();
    }
}
