use crate::character::Character;
use std::collections::HashMap;

pub struct Data {
    pub character_list: HashMap<String, Character>,
}

impl Data {
    pub fn new() -> Self {
        let character_list = HashMap::new();

        Data { character_list }
    }
}
