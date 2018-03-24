#![allow(unused_imports)]

extern crate clap;
extern crate reqwest;
extern crate serde_json;

mod humans;

use serde_json::{Error, Value};
use clap::{App, Arg};
use humans::{Human, Humans};

fn from_json(data: &str) -> Humans {
    let mut humans = Vec::new();

    let msg: Value = serde_json::from_str(data).expect("json error");
    for human in msg["people"].as_array().unwrap().iter() {
        humans.push(Human::new(
            human["name"].as_str().unwrap(),
            human["craft"].as_str().unwrap(),
        ));
    }

    humans
}

fn fetch_who_is_up_there() -> Humans {
    let data = reqwest::get("http://api.open-notify.org/astros.json")
        .expect("unable to fetch data")
        .text()
        .unwrap();

    from_json(&data)
}

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

    if matches.occurrences_of("count") > 0 {
        println!("{}", fetch_who_is_up_there().len());
        return;
    }

    for human in fetch_who_is_up_there().iter() {
        println!("{}, {}", human.name(), human.ship());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_data() -> &'static str {
        r#"{
"message": "success",
"number": 6,
"people": [
  {"name": "Anton Shkaplerov", "craft": "ISS"},
  {"name": "Scott Tingle", "craft": "ISS"},
  {"name": "Norishige Kanai", "craft": "ISS"},
  {"name": "Oleg Artemyev", "craft": "Soyuz MS-08"},
  {"name": "Andrew Feustel", "craft": "Soyuz MS-08"},
  {"name": "Richard Arnold", "craft": "Soyuz MS-08"}]
}"#
    }

    #[test]
    fn parse_successful_data() {
        let expected_humans = vec![
            Human::new("Anton Shkaplerov", "ISS"),
            Human::new("Scott Tingle", "ISS"),
            Human::new("Norishige Kanai", "ISS"),
            Human::new("Oleg Artemyev", "Soyuz MS-08"),
            Human::new("Andrew Feustel", "Soyuz MS-08"),
            Human::new("Richard Arnold", "Soyuz MS-08"),
        ];

        let humans = from_json(example_data());
        assert_eq!(6, humans.len());
        for human in expected_humans.iter() {
            assert_eq!(true, humans.contains(&human))
        }
    }
}
