use serde_json;
use serde_json::Error;

#[derive(Debug, PartialEq)]
pub struct Human {
    name: String,
    ship: String,
}

pub type Humans = Vec<Human>;

impl Human {
    pub fn new(name: &str, ship: &str) -> Human {
        Human {
            name: String::from(name),
            ship: String::from(ship),
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn ship(&self) -> &str {
        self.ship.as_str()
    }
}

#[derive(Debug)]
pub struct HumanError(String);

impl From<serde_json::Error> for HumanError {
    fn from(e: serde_json::Error) -> HumanError {
        use std::error::Error;
        HumanError(e.description().to_string())
    }
}

impl From<String> for HumanError {
    fn from(s: String) -> HumanError {
        HumanError(s)
    }
}
