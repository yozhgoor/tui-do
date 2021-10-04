use std::collections::HashMap;

use crate::character::Character;

pub struct Data {
    pub character_list: HashMap<String, Character>,
}

impl Data {
    pub fn new() -> Self {
        let character_list = HashMap::new();

        Data { character_list }
    }
}
