use cursive::{Cursive, CursiveExt};

mod character;
mod character_view;
mod dashboard;
mod data;
mod faction;
mod faction_view;
mod quest;
mod quest_view;

use crate::data::Data;

fn main() {
    let mut siv = Cursive::new();

    siv.add_global_callback('q', |siv| siv.quit());
    siv.set_user_data(Data::new());

    character_view::draw_view(&mut siv);

    siv.run();
}
