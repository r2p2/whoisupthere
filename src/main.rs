#![allow(unused_imports)]

extern crate clap;
extern crate reqwest;
extern crate serde_json;

pub mod open_notify;

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

    match open_notify::fetch_who_is_up_there() {
        Ok(humans_up_there) => {
            if matches.occurrences_of("count") > 0 {
                println!("{}", humans_up_there.len());
                return;
            }

            for human in humans_up_there.iter() {
                println!("{}, {}", human.name(), human.ship());
            }
        },
        Err(error_msg) => {
            eprintln!("Ups: {:?}", error_msg);
        }
    }
}
