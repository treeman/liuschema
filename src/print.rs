use std::io::stdio;
use getopts::{ Matches };

use timeedit::{ Event, Type, TypeInfo, Group, Course };

#[deriving(Show)]
pub struct Printer {
    today: bool,
    conky: bool,
}

impl Printer {
    pub fn new(matches: Matches) -> Printer {
        Printer {
            today: matches.opt_present("today"),
            conky: matches.opt_present("conky"),
        }
    }

    pub fn print_events(&self, events: Vec<Event>) {
        if events.is_empty() {
            self.print_no_events();
            return;
        }

        for event in events.iter() {
            self.print_prefix();

            if self.today {
                println!("{}", event.fmt_time_only());
            } else {
                println!("{}", event);
            }
        }
    }

    fn print_no_events(&self) {
        self.print_prefix();
        println!("I'm free!");
    }

    fn print_prefix(&self) {
        if self.conky {
            stdio::print("  ${voffset 8}");
        }
    }

    pub fn print_search(&self, types: Vec<TypeInfo>, typ: Type) {
        let t = match typ {
            Course => "course",
            Group => "group",
        };
        println!("Found {} {}{}", types.len(), t, if types.len() == 1 { "" } else { "s" });
        for t in types.iter() {
            println!("{}", t);
        }
    }
}

