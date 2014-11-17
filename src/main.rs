#![feature(globs)]

extern crate time;
extern crate timeedit;

use std::time::Duration;

use timeedit::{ Config, DataId, TypeInfo, Course, Group };

fn main() {
    let conf = Config::from_file("config.json");

    let from = time::now();
    let to = time::at(from.to_timespec() + Duration::weeks(1));

    let s = "TATA";

    let (types, typ) = timeedit::search(s, &conf);
    let ts = match typ {
        Course => "courses",
        Group => "groups",
    };
    println!("Found {} {}", types.len(), ts);
    for t in types.iter() {
        println!("{}", t);
    }

    let mut types = Vec::new();
    //types.push(TypeInfo::new("TATA31", "", DataId::new("363733.219")));
    types.push(TypeInfo::new("TATA49", "", DataId::new("363741.219")));
    types.push(TypeInfo::new("FYN1", "", DataId::new("153398.205")));

    let entries = timeedit::schedule(types, from, to, &conf);

    println!("Found {} entries", entries.len());
    for entry in entries.iter() {
        println!("{}", entry);
    }
}

