use crate::character::Character;
use crate::character_view;
use crate::faction_view;
use crate::quest_view;
use cursive::traits::*;
use cursive::utils::Counter;
use cursive::views::{Button, Dialog, DummyView, LinearLayout, ProgressBar, TextView};
use cursive::Cursive;

pub fn draw_view(siv: &mut Cursive, slug: String) {
    let character = Character::from_slug(siv, slug.clone());
    let lvl_progress_bar = LinearLayout::vertical()
        .child(TextView::new(format!("Level : {}", character.lvl)))
        .child(
            ProgressBar::new()
                .min(0)
                .max(1000)
                .with_label(|v, (_, max)| format!("Exp: {}/{}", v, max))
                .with_value(Counter::new(character.exp as usize)),
        );

    let mut character_info = LinearLayout::vertical()
        .child(TextView::new(character.name.clone()))
        .child(TextView::new(character.class.display()))
        .child(DummyView)
        .child(TextView::new(format!("Money: {}$", character.money)))
        .child(DummyView)
        .child(lvl_progress_bar)
        .child(DummyView);

    if !character.factions.is_empty() {
        character_info.add_child(TextView::new("Factions:"));
        for faction in &character.factions {
            character_info.add_child(
                LinearLayout::vertical()
                    .child(TextView::new(format!(
                        "{} (Lvl: {})",
                        faction.name, faction.lvl
                    )))
                    .child(
                        ProgressBar::new()
                            .min(0)
                            .max(1000)
                            .with_label(|v, (_, max)| format!("Rep: {}/{}", v, max))
                            .with_value(Counter::new(faction.reputation as usize)),
                    )
                    .child(DummyView),
            )
        }
    }

    let buttons = LinearLayout::horizontal()
        .child(Button::new("Quests", {
            let slug = slug.clone();
            move |siv| quest_view::draw_view(siv, slug.clone())
        }))
        .child(DummyView)
        .child(Button::new("Factions", {
            siv.pop_layer();
            move |siv| faction_view::draw_view(siv, slug.clone())
        }))
        .child(DummyView)
        .child(Button::new("Character", |siv| {
            character_view::draw_view(siv)
        }))
        .child(DummyView)
        .child(Button::new("Quit", |siv| siv.quit()));

    character_info.add_child(buttons);

    siv.pop_layer();
    siv.add_layer(
        Dialog::around(character_info)
            .with_name("dashboard")
            .fixed_size((80, 20)),
    );
}
