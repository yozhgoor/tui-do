use cursive::traits::Boxable;
use cursive::utils::Counter;
use cursive::views::{Dialog, DummyView, LinearLayout, ProgressBar, TextView};
use cursive::Cursive;

use crate::character::Character;

pub fn get_dashboard(s: &mut Cursive, character: &Character) {
    s.pop_layer();
    s.add_layer(
        Dialog::around(
            LinearLayout::vertical()
                .child(TextView::new(character.name.clone()))
                .child(TextView::new(character.class.display()))
                .child(DummyView)
                .child(TextView::new(format!("Money: {}$", character.money)))
                .child(DummyView)
                .child(TextView::new(format!("Level: {}", character.lvl)))
                .child(
                    ProgressBar::new()
                        .min(0)
                        .max(1000)
                        .with_label(|v, (_, max)| format!("Exp: {}/{}", v, max))
                        .with_value(Counter::new(500)),
                ),
        )
        .fixed_size((80, 20)),
    );
}
