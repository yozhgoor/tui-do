use cursive::traits::*;
use cursive::views::{Button, Dialog, DummyView, EditView, LinearLayout, SelectView, TextView};
use cursive::Cursive;

use crate::character::{Character, Class};
use crate::dashboard::get_dashboard;
use crate::data::Data;

pub fn character_selection(siv: &mut Cursive) {
    let character_list = &siv.user_data::<Data>().unwrap().character_list;

    let select = SelectView::<Character>::new()
        .on_submit(get_dashboard)
        .with_all(
            character_list
                .into_iter()
                .map(|(_, v)| (v.display_for_selection(), v.clone())),
        )
        .with_name("character_select")
        .fixed_size((80, 20));

    let buttons = LinearLayout::vertical()
        .child(Button::new("Add new", create_character))
        .child(Button::new("Delete", delete_character))
        .child(DummyView)
        .child(Button::new("Quit", Cursive::quit));

    siv.add_layer(
        Dialog::around(
            LinearLayout::horizontal()
                .child(select)
                .child(DummyView)
                .child(buttons),
        )
        .title("Select a character"),
    );
}

pub fn create_character(siv: &mut Cursive) {
    siv.add_layer(
        Dialog::new()
            .title("Enter a name")
            .content(
                EditView::new()
                    .on_submit(select_class)
                    .with_name("new_name"),
            )
            .button("Ok", |s| {
                let name = s
                    .call_on_name("new_character_name", |v: &mut EditView| v.get_content())
                    .unwrap();

                select_class(s, &name);
            }),
    );
}

pub fn select_class(s: &mut Cursive, name: &str) {
    let name = name.to_string();
    if name.is_empty() {
        s.add_layer(Dialog::info("Please enter a name"));
    } else {
        s.pop_layer();
        let text_view =
            TextView::new("Warrior: Gain more exp on Special Quest").with_name("presentation");
        let select_view = SelectView::new()
            .item("Warrior", Class::Warrior)
            .item("Hunter", Class::Hunter)
            .item("Rogue", Class::Rogue)
            .item("Mage", Class::Mage)
            .on_select(|s, item| {
                let content = match *item {
                    Class::Warrior => "Warrior: Gain more exp on Special Quest",
                    Class::Hunter => "Hunter: Gain more exp on Daily Quest",
                    Class::Rogue => "Rogue: Gain more money but less exp on each Quest",
                    Class::Mage => "Mage: Gain more exp but less money on each Quest",
                };

                // Update the Text view with the presentation for each class
                s.call_on_name("presentation", |v: &mut TextView| {
                    v.set_content(content);
                })
                .unwrap();
            })
            .on_submit(move |s, item| {
                let character = Character::new(name.to_string(), item.clone());

                s.with_user_data(|data: &mut Data| {
                    data.character_list
                        .insert(character.name.clone(), character.clone())
                });
                s.call_on_name("character_select", |v: &mut SelectView<Character>| {
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
                s.pop_layer();
            });
        s.add_layer(
            Dialog::around(LinearLayout::vertical().child(select_view).child(text_view))
                .title("Pick a class"),
        )
    }
}

pub fn delete_character(s: &mut Cursive) {
    let mut select = s
        .find_name::<SelectView<Character>>("character_select")
        .unwrap();
    match select.selected_id() {
        None => s.add_layer(Dialog::info("No character to remove")),
        Some(focus) => {
            select.remove_item(focus);
        }
    }
}
