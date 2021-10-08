use crate::character::Character;
use crate::dashboard;
use cursive::traits::*;
use cursive::views::{Dialog, LinearLayout, NamedView, TextView};
use cursive::Cursive;
use cursive_tabs::TabPanel;

pub fn draw_view(siv: &mut Cursive, slug: String) {
    siv.pop_layer();
    let character = Character::from_slug(siv, slug.clone());
    let factions = character.factions.clone();
    let mut panel = TabPanel::new();
    for faction in factions {
        panel.add_tab(create_tab(faction.name))
    }
    panel.add_tab(create_tab(character.name.clone()));

    siv.add_layer(Dialog::around(panel).button("Back", {
        move |siv| {
            siv.pop_layer();
            dashboard::draw_view(siv, slug.clone());
        }
    }));
}

fn create_tab(title: String) -> NamedView<Dialog> {
    Dialog::around(
        LinearLayout::vertical().child(TextView::new(format!("{}'s quests belong here", title))),
    )
    .with_name(title)
}
