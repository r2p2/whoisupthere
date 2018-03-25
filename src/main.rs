#![allow(unused_imports)]

extern crate clap;
extern crate open_notify_api;

use clap::{App, Arg};

fn main() {
    let matches = App::new("whoisupthere")
        .version("0.1.0")
        .author("Robert Peters <r2p2.gw@gmail.com>")
        .about("Show who is in space right now")
        .arg(
            Arg::with_name("count")
                .short("c")
                .long("count")
                .help("Prints only the number of people"),
        )
        .get_matches();

    match open_notify_api::astros() {
        Ok(astros) => {
            if matches.occurrences_of("count") > 0 {
                println!("{}", astros.number());
                return;
            }

            for person in astros.people().iter() {
                println!("{}, {}", person.name(), person.craft());
            }
        },
        Err(error_msg) => {
            eprintln!("Ups: {:?}", error_msg);
        }
    }
}
