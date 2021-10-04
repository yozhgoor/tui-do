#[derive(Clone)]
pub struct Character {
    pub name: String,
    pub class: Class,
    pub lvl: u32,
    pub exp: u32,
    pub money: u32,
}

impl Character {
    pub fn new(name: String, class: Class) -> Self {
        Self {
            name,
            class,
            lvl: 0,
            exp: 0,
            money: 0,
        }
    }

    pub fn display_for_selection(&self) -> String {
        format!(
            "{} - {} | Level {}",
            self.name,
            self.class.display(),
            self.lvl
        )
    }
}

#[derive(Clone)]
pub enum Class {
    Warrior,
    Hunter,
    Rogue,
    Mage,
}

impl Class {
    pub fn display(&self) -> &str {
        match self {
            Class::Warrior => "Warrior",
            Class::Hunter => "Hunter",
            Class::Rogue => "Rogue",
            Class::Mage => "Mage",
        }
    }
}
