#[derive(PartialEq)]
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
