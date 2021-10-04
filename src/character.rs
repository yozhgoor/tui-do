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

    pub fn mock(n: u8) -> Vec<Character> {
        let first = Character {
            name: String::from("Frodo"),
            class: Class::Rogue,
            lvl: 300,
            exp: 270,
            money: 100_000_000,
        };

        let second = Character {
            name: String::from("Gandalf"),
            class: Class::Mage,
            lvl: 700,
            exp: 999,
            money: 0,
        };

        let third = Character {
            name: String::from("Legolas"),
            class: Class::Hunter,
            lvl: 549,
            exp: 15,
            money: 160_000,
        };

        let fourth = Character {
            name: String::from("Gimli"),
            class: Class::Warrior,
            lvl: 549,
            exp: 12,
            money: 150_000,
        };

        match n {
            1 => vec![first],
            2 => vec![first, second],
            3 => vec![first, second, third],
            4 => vec![first, second, third, fourth],
            _ => vec![],
        }
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
