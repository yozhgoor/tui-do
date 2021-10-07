#[derive(Clone, PartialEq)]
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

    pub fn display_for_presentation(&self) -> String {
        format!("{} - Level {}", self.name, self.lvl)
    }
}
