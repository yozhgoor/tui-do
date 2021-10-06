use cursive::traits::*;
use cursive::views::{Button, Dialog, DummyView, LinearLayout, SelectView};
use cursive::Cursive;

use crate::faction::Faction;

pub fn draw_view(siv: &mut Cursive, factions: Vec<Faction>) {
    let faction_select = SelectView::<Faction>::new()
        .with_all(
            factions
                .iter()
                .map(|i| (format!("{} - Level: {}", i.name, i.lvl), i.clone())),
        )
        .with_name("character_select")
        .fixed_size((80, 20));

    let faction_buttons = LinearLayout::vertical()
        .child(Button::new("Add new", create_item))
        .child(Button::new("Delete", delete_item))
        .child(DummyView)
        .child(Button::new("Back", |siv| {
            siv.pop_layer();
        }));

    siv.add_layer(Dialog::around(
        LinearLayout::horizontal()
            .child(faction_select)
            .child(faction_buttons),
    ))
}

pub fn create_item(siv: &mut Cursive) {
    siv.quit()
}

pub fn delete_item(siv: &mut Cursive) {
    siv.quit()
}
