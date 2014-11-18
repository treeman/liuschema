#![feature(slicing_syntax)]

extern crate time;
extern crate timeedit;
extern crate getopts;
extern crate serialize;

use std::os;
use std::io::stdio;
use std::time::Duration;
use getopts::{ optopt, optflag, getopts, usage, Matches };

use config::Config;
use print::Printer;

mod config;
mod print;

fn main() {
    let args = os::args();

    let opts = [
        optopt("c", "config", "Specify config file, default: config.json", "CFILE"),
        optflag("h", "help", "Display this help and exit"),
        optopt("", "search", "Search for a course/group", "TEXT"),
        optflag("", "today", "Limit schedule to today only"),
        optflag("", "conky", "Pretty print for conky"),
    ];

    let matches = match getopts(args.tail(), opts) {
        Ok(m) => m,
        Err(e) => panic!("{}", e),
    };

    let progname = args[0].clone();
    let usage = usage("A simple cli for timeedit scheduling.", opts);

    let mode = if matches.opt_present("help") {
        Mode::Help
    } else if matches.opt_present("search") {
        Mode::Search
    } else {
        Mode::Schedule
    };

    let conf_file = match matches.opt_str("c") {
        Some(c) => c,
        None => "config.json".to_string(),
    };
    let conf = Config::from_file(conf_file[]);

    match mode {
        Mode::Help => help(progname[], usage[]),
        Mode::Search => {
            let string = matches.opt_str("search").unwrap();
            search(string[], matches, conf);
        },
        Mode::Schedule => schedule(matches, conf),
    }
}

enum Mode {
    Help,
    Search,
    Schedule
}

fn search(string: &str, matches: Matches, conf: Config) {
    println!("Searching for \"{}\"", string);

    let types = timeedit::multi_search(string, conf.base[]);
    let printer = Printer::new(matches);
    printer.print_search(types);
}

fn schedule(matches: Matches, conf: Config) {
    let from = time::now();
    let to = time::at(from.to_timespec() + Duration::weeks(1));

    let mut events = timeedit::schedule_from_ids(conf.data_ids, from, to, conf.base[]);

    // Filter out day by transforming to YYYY-MM-DD and discarding non-matches
    if matches.opt_present("today") {
        let date_format = "%F";
        let today = time::strftime(date_format, &from).unwrap();
        events = events.into_iter().filter(|x| {
            today == time::strftime(date_format, &x.start).unwrap()
        }).collect();
    }

    let printer = Printer::new(matches);
    printer.print_events(events);
}

fn help(progname: &str, usage: &str) {
    println!("Usage: {:s} [OPTION]", progname);
    stdio::println(usage);
}

