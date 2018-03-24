#![allow(unused_imports)]

extern crate serde_json;
extern crate reqwest;

use serde_json::{Value, Error};

#[derive(PartialEq)]
struct Human {
    name: String,
    ship: String,
}

fn from_json(data: &str) -> Vec<Human> {
    let mut humans = Vec::new();
    
    let msg: Value = serde_json::from_str(data).expect("json error");
    for human in msg["people"].as_array().unwrap().iter() {
        humans.push(
            Human {
                name: String::from(human["name"].as_str().unwrap()),
                ship: String::from(human["craft"].as_str().unwrap()),
            });
    }

    humans
}

fn fetch_who_is_up_there() -> Vec<Human> {
    let data = reqwest::get("http://api.open-notify.org/astros.json")
        .expect("unable to fetch data")
        .text()
        .unwrap();

    from_json(&data)
}

fn main() {
    for human in fetch_who_is_up_there().iter() {
        println!("{}, {}", human.name, human.ship);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_data() -> &'static str {
        r#"{"message": "success", "number": 6, "people": [{"name": "Anton Shkaplerov", "craft": "ISS"}, {"name": "Scott Tingle", "craft": "ISS"}, {"name": "Norishige Kanai", "craft": "ISS"}, {"name": "Oleg Artemyev", "craft": "Soyuz MS-08"}, {"name": "Andrew Feustel", "craft": "Soyuz MS-08"}, {"name": "Richard Arnold", "craft": "Soyuz MS-08"}]}"#
    }
    
    #[test]
    fn parse_successful_data() {
        let expected_humans = vec! [
            Human { name: String::from("Anton Shkaplerov"), ship: String::from("ISS") },
            Human { name: String::from("Scott Tingle")    , ship: String::from("ISS") },
            Human { name: String::from("Norishige Kanai") , ship: String::from("ISS") },
            Human { name: String::from("Oleg Artemyev")   , ship: String::from("Soyuz MS-08") },
            Human { name: String::from("Andrew Feustel")  , ship: String::from("Soyuz MS-08") },
            Human { name: String::from("Richard Arnold")  , ship: String::from("Soyuz MS-08") },
            ];

        let humans = from_json(example_data());
        assert_eq!(6, humans.len());
        for human in expected_humans.iter() {
            assert_eq!(true, humans.contains(&human))
        }
    }
}
