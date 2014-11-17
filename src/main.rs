#![feature(slicing_syntax)]

extern crate time;
extern crate timeedit;
extern crate getopts;
extern crate serialize;

use std::os;
use std::io;
use std::time::Duration;
use getopts::{ optopt, optflag, getopts, usage, Matches };

use timeedit::{ Course, Group };

use config::Config;

mod config;

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
        Help
    } else if matches.opt_present("search") {
        Search
    } else {
        Run
    };

    let conf_file = match matches.opt_str("c") {
        Some(c) => c,
        None => "config.json".to_string(),
    };
    let conf = Config::from_file(conf_file[]);

    match mode {
        Help => help(progname[], usage[]),
        Search => {
            let string = matches.opt_str("search").unwrap();
            search(string[], conf);
        },
        Run => run(matches, conf),
    }
}

enum Mode {
    Help,
    Search,
    Run
}

fn search(string: &str, conf: Config) {
    println!("Searching for \"{}\"", string);

    let (types, typ) = timeedit::search(string, conf.base[]);
    let t = match typ {
        Course => "course",
        Group => "group",
    };
    println!("Found {} {}{}", types.len(), t, if types.len() == 1 { "" } else { "s" });
    for t in types.iter() {
        println!("{}", t);
    }
}

fn run(matches: Matches, conf: Config) {
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

    for event in events.iter() {
        println!("{}", event);
    }
}

fn help(progname: &str, usage: &str) {
    println!("Usage: {:s} [OPTION]", progname);
    io::stdio::println(usage);
}

