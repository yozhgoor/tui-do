use crate::data::Data;
use crate::faction::Faction;
use crate::quest::{Checkboxes, Quest};
use cursive::Cursive;
use std::collections::HashMap;

type CharacterQuests = HashMap<String, Quests>;
type Quests = HashMap<String, Quest>;

#[derive(Debug, Clone)]
pub struct Character {
    pub name: String,
    pub class: Class,
    pub lvl: u32,
    pub exp: u32,
    pub money: u32,
    pub factions: Vec<Faction>,
    pub character_quests: CharacterQuests,
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
            character_quests: CharacterQuests::new(),
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

#[derive(Debug, Clone)]
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
    let mut frodo_character_quests = CharacterQuests::new();
    let mut gandalf_character_quests = CharacterQuests::new();
    let mut legolas_character_quests = CharacterQuests::new();
    let mut gimli_character_quests = CharacterQuests::new();
    let mut frodo_quests = Quests::new();
    let mut gandalf_quests = Quests::new();
    let mut legolas_quests = Quests::new();
    let mut gimli_quests = Quests::new();

    let mut fellowship_of_the_ring_quests = Quests::new();
    let fellowship_of_the_ring_quest = Quest::new(
        "Throw the ring into big fire in a mountain".to_string(),
        "It's a very big fire".to_string(),
        "Fellowship of the Ring".to_string(),
        false,
        Checkboxes::new(),
    );
    fellowship_of_the_ring_quests.insert(
        fellowship_of_the_ring_quest.title.clone(),
        fellowship_of_the_ring_quest,
    );

    let frodo_quest = Quest::new(
        "Bring the ring into Mordor".to_string(),
        "It's a very powerful ring".to_string(),
        "Frodo".to_string(),
        false,
        Checkboxes::new(),
    );
    frodo_quests.insert(frodo_quest.title.clone(), frodo_quest);

    let istari_quest = Quest::new(
        "Counsel and assist all those who opposed the Dark Lord Sauron".to_string(),
        "Mission of the five istari sent in Middle-Earth".to_string(),
        "Istari".to_string(),
        false,
        Checkboxes::new(),
    );
    gandalf_quests.insert(istari_quest.title.clone(), istari_quest);

    let legolas_quest = Quest::new(
        "Sharpen arrowhead".to_string(),
        "Be always ready".to_string(),
        "Legolas".to_string(),
        true,
        Checkboxes::new(),
    );
    legolas_quests.insert(legolas_quest.title.clone(), legolas_quest);

    let gimli_quest = Quest::new(
        "Sharpen axe".to_string(),
        "Too loud for the elf, so it's funny".to_string(),
        "Gimli".to_string(),
        true,
        Checkboxes::new(),
    );
    gimli_quests.insert(gimli_quest.title.clone(), gimli_quest);

    frodo_character_quests.insert("Frodo".to_string(), frodo_quests);
    gandalf_character_quests.insert("Istari".to_string(), gandalf_quests);
    legolas_character_quests.insert("Legolas".to_string(), legolas_quests);
    gimli_character_quests.insert("Gimli".to_string(), gimli_quests);

    frodo_character_quests.insert(
        "Fellowship of the Ring".to_string(),
        fellowship_of_the_ring_quests.clone(),
    );
    gandalf_character_quests.insert(
        "Fellowship of the Ring".to_string(),
        fellowship_of_the_ring_quests.clone(),
    );
    legolas_character_quests.insert(
        "Fellowship of the Ring".to_string(),
        fellowship_of_the_ring_quests.clone(),
    );
    gimli_character_quests.insert(
        "Fellowship of the Ring".to_string(),
        fellowship_of_the_ring_quests.clone(),
    );

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
            character_quests: frodo_character_quests,
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
            character_quests: gandalf_character_quests,
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
            character_quests: legolas_character_quests,
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
            character_quests: gimli_character_quests,
        },
    ]
}
