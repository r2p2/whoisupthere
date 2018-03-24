pub mod humans;

use reqwest;
use serde_json;
use serde_json::{Error, Value};

/// Fetches the astronouts which are currently in space
/// from the open-notify.org api.
pub fn fetch_who_is_up_there() -> humans::Humans {
    let data = reqwest::get("http://api.open-notify.org/astros.json")
        .expect("unable to fetch data")
        .text()
        .unwrap();

    from_json(&data)
}

/// Convert json-formated data provided by the open-notify.org
/// into our own data structure.
fn from_json(data: &str) -> humans::Humans {
    let mut humans = Vec::new();

    let msg: Value = serde_json::from_str(data).expect("json error");
    for human in msg["people"].as_array().unwrap().iter() {
        humans.push(humans::Human::new(
            human["name"].as_str().unwrap(),
            human["craft"].as_str().unwrap(),
        ));
    }

    humans
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::humans::Human;

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
