pub mod humans;

use reqwest;
use serde_json;
use serde_json::{Error, Value};

/// Fetches the astronouts which are currently in space
/// from the open-notify.org api.
pub fn fetch_who_is_up_there() -> Result<humans::Humans, humans::HumanError> {
    let data = reqwest::get("http://api.open-notify.org/astros.json")
        .expect("unable to fetch data")
        .text()
        .unwrap();

    from_json(&data)
}

/// Convert json-formated data provided by the open-notify.org
/// into our own data structure.
fn from_json(data: &str) -> Result<humans::Humans, humans::HumanError> {
    let mut humans = Vec::new();

    let msg: Value = serde_json::from_str(data)?;

    let people = msg["people"]
        .as_array()
        .ok_or(String::from("'people' field is missing"))?;

    for person in people.iter() {
        let name = person["name"]
            .as_str()
            .ok_or(String::from("'name' field is missing"))?;

        let craft = person["craft"]
            .as_str()
            .ok_or(String::from("'craft' field is missing"))?;

        humans.push(humans::Human::new(name, craft));
    }

    Ok(humans)
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::humans::Human;
    #[test]
    fn parse_successful_data() {
        let input_data = r#"{
            "message": "success",
            "number": 6,
            "people": [
            {"name": "Anton Shkaplerov", "craft": "ISS"},
            {"name": "Scott Tingle", "craft": "ISS"},
            {"name": "Norishige Kanai", "craft": "ISS"},
            {"name": "Oleg Artemyev", "craft": "Soyuz MS-08"},
            {"name": "Andrew Feustel", "craft": "Soyuz MS-08"},
            {"name": "Richard Arnold", "craft": "Soyuz MS-08"}]
            }"#;

        let expected_humans = vec![
            Human::new("Anton Shkaplerov", "ISS"),
            Human::new("Scott Tingle", "ISS"),
            Human::new("Norishige Kanai", "ISS"),
            Human::new("Oleg Artemyev", "Soyuz MS-08"),
            Human::new("Andrew Feustel", "Soyuz MS-08"),
            Human::new("Richard Arnold", "Soyuz MS-08"),
        ];

        if let Ok(humans) = from_json(input_data) {
            assert_eq!(humans.len(), 6);
            for human in expected_humans.iter() {
                assert_eq!(humans.contains(&human), true);
            }
        } else {
            assert!(false);
        }
    }

    #[test]
    fn parse_faulty_data() {
        let input_data = r#"{
            "message": "success",
            "number": 6,
            "people": [
            {"name": "Anton Shkaplerov", "craft": "ISS"},
            {"name": "Scott Tingle", "craft": "ISS"},
            {"name": "Norishige Kanai", "craft": "ISS"},
            {"name": "Oleg Artemyev" },
            {"name": "Andrew Feustel", "craft": "Soyuz MS-08"},
            {"name": "Richard Arnold", "craft": "Soyuz MS-08"}]
            }"#;

        if let Err(_) = from_json(input_data) {
            assert!(true);
        } else {
            assert!(false);
        }
    }
}
