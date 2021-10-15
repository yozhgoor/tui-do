use crate::data::Data;
use crate::faction::Faction;
use crate::quest::Quest;
use cursive::Cursive;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Character {
    pub name: String,
    pub class: Class,
    pub lvl: u32,
    pub exp: u32,
    pub money: u32,
    pub factions: Vec<Faction>,
    pub quests: HashMap<String, HashMap<String, Quest>>,
}

impl Character {
    pub fn new(name: String, class: Class) -> Self {
        Self {
            name,
            class,
            lvl: 0,
            exp: 0,
            money: 0,
            factions: Vec::new(),
            quests: HashMap::new(),
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

    pub fn from_slug(siv: &mut Cursive, slug: String) -> &mut Self {
        siv.user_data::<Data>()
            .unwrap()
            .character_list
            .get_mut(&slug)
            .unwrap()
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

pub fn mock() -> Vec<Character> {
    vec![
        Character {
            name: String::from("Frodo"),
            class: Class::Rogue,
            lvl: 300,
            exp: 270,
            money: 100_000_000,
            factions: vec![
                Faction {
                    name: "Hobbit".to_string(),
                    reputation: 500,
                    lvl: 30,
                },
                Faction {
                    name: "Fellowship of the Ring".to_string(),
                    reputation: 0,
                    lvl: 0,
                },
            ],
            quests: HashMap::new(),
        },
        Character {
            name: String::from("Gandalf"),
            class: Class::Mage,
            lvl: 700,
            exp: 999,
            money: 0,
            factions: vec![
                Faction {
                    name: "Istari".to_string(),
                    reputation: 600,
                    lvl: 80,
                },
                Faction {
                    name: "Fellowship of the Ring".to_string(),
                    reputation: 500,
                    lvl: 50,
                },
            ],
            quests: HashMap::new(),
        },
        Character {
            name: String::from("Legolas"),
            class: Class::Hunter,
            lvl: 549,
            exp: 15,
            money: 160_000,
            factions: vec![
                Faction {
                    name: "Elf".to_string(),
                    reputation: 300,
                    lvl: 50,
                },
                Faction {
                    name: "FellowShip of the Ring".to_string(),
                    reputation: 500,
                    lvl: 40,
                },
            ],
            quests: HashMap::new(),
        },
        Character {
            name: String::from("Gimli"),
            class: Class::Warrior,
            lvl: 549,
            exp: 12,
            money: 150_000,
            factions: vec![
                Faction {
                    name: "Dwarf".to_string(),
                    reputation: 600,
                    lvl: 50,
                },
                Faction {
                    name: "Fellowship of the Ring".to_string(),
                    reputation: 500,
                    lvl: 40,
                },
            ],
            quests: HashMap::new(),
        },
    ]
}
