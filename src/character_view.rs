use crate::character;
use crate::character::{Character, Class};
use crate::dashboard;
use crate::data::Data;
use cursive::traits::*;
use cursive::views::{Button, Dialog, DummyView, EditView, LinearLayout, SelectView, TextView};
use cursive::Cursive;

pub fn draw_view(siv: &mut Cursive) {
    let character_list = &mut siv.user_data::<Data>().unwrap().character_list;
    for character in character::mock() {
        character_list.insert(character.name.clone(), character);
    }

    let character_select = SelectView::<Character>::new()
        .on_submit(|siv, character| dashboard::draw_view(siv, character.name.clone()))
        .with_all(character_list.iter().map(|(_, used_character)| {
            (
                used_character.display_for_selection(),
                used_character.clone(),
            )
        }))
        .with_name("character_select")
        .fixed_size((80, 20));

    let character_buttons = LinearLayout::vertical()
        .child(Button::new("Add new", create_item))
        .child(Button::new("Delete", delete_item))
        .child(DummyView)
        .child(Button::new("Quit", Cursive::quit));

    siv.add_layer(
        Dialog::around(
            LinearLayout::horizontal()
                .child(character_select)
                .child(DummyView)
                .child(character_buttons),
        )
        .title("Select a character"),
    );
}

pub fn create_item(siv: &mut Cursive) {
    siv.add_layer(
        Dialog::new()
            .title("Enter a name")
            .content(
                EditView::new()
                    .on_submit(select_class)
                    .with_name("new_name"),
            )
            .button("Back", |s| {
                s.pop_layer();
            }),
    );
}

pub fn delete_item(siv: &mut Cursive) {
    let mut select = siv
        .find_name::<SelectView<Character>>("character_select")
        .unwrap();
    match select.selected_id() {
        None => siv.add_layer(Dialog::info("No character to remove")),
        Some(focus) => {
            select.remove_item(focus);
        }
    }
}

pub fn select_class(siv: &mut Cursive, name: &str) {
    let name = name.to_string();
    if name.is_empty() {
        siv.add_layer(Dialog::info("Please enter a name"));
    } else {
        siv.pop_layer();
        let text_view =
            TextView::new("Warrior: Gain more exp on Special Quest").with_name("presentation");
        let select_view = SelectView::new()
            .item("Warrior", Class::Warrior)
            .item("Hunter", Class::Hunter)
            .item("Rogue", Class::Rogue)
            .item("Mage", Class::Mage)
            .on_select(|siv, class| {
                let content = match *class {
                    Class::Warrior => "Warrior: Gain more exp on Special Quest",
                    Class::Hunter => "Hunter: Gain more exp on Daily Quest",
                    Class::Rogue => "Rogue: Gain more money but less exp on each Quest",
                    Class::Mage => "Mage: Gain more exp but less money on each Quest",
                };

                // Update the TextView with the presentation for each class
                siv.call_on_name("presentation", |view: &mut TextView| {
                    view.set_content(content);
                })
                .unwrap();
            })
            .on_submit(move |siv, class| {
                let character = Character::new(name.to_string(), class.clone());

                siv.with_user_data(|data: &mut Data| {
                    data.character_list
                        .insert(character.name.clone(), character.clone())
                });
                siv.call_on_name("character_select", |v: &mut SelectView<Character>| {
                    v.add_item(
                        format!(
                            "{} - {} | Level: {}",
                            character.name.clone(),
                            character.class.display(),
                            character.lvl.clone()
                        ),
                        character.clone(),
                    );
                });
                siv.pop_layer();
            });
        siv.add_layer(
            Dialog::around(LinearLayout::vertical().child(select_view).child(text_view))
                .title("Pick a class"),
        )
    }
}
