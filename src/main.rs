use cursive::{Cursive, CursiveExt};

mod character;
mod character_selection;
mod dashboard;
mod data;

use crate::character_selection::character_selection;
use crate::data::Data;

fn main() {
    let mut siv = Cursive::new();

    siv.add_global_callback('q', |s| s.quit());
    siv.set_user_data(Data::new());

    character_selection(&mut siv);

    siv.run();
}
