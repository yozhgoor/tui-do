use cursive::traits::*;
use cursive::utils::Counter;
use cursive::views::{Button, Dialog, DummyView, LinearLayout, ProgressBar, TextView};
use cursive::Cursive;

use crate::character::Character;
use crate::faction::get_faction_view;
use crate::quest::get_quest_view;

pub fn get_dashboard(s: &mut Cursive, character: Character) {
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
        for faction in character.factions.clone() {
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
                    ),
            )
        }
    }

    let buttons = LinearLayout::horizontal()
        .child(Button::new("Quests", move |s| {
            get_quest_view(s, character.quests.clone())
        }))
        .child(Button::new("Factions", move |s| {
            get_faction_view(s, character.factions.clone())
        }));

    character_info.add_child(buttons);

    s.pop_layer();
    s.add_layer(Dialog::around(character_info).fixed_size((80, 20)));
}
