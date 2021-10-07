#[derive(Clone)]
pub struct Faction {
    pub name: String,
    pub reputation: u32,
    pub lvl: u32,
}

impl Faction {
    pub fn new(name: &str) -> Faction {
        Faction {
            name: name.to_string(),
            reputation: 0,
            lvl: 0,
        }
    }
}
