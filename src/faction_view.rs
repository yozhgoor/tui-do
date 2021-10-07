use crate::character::Character;
use crate::dashboard;
use crate::data::Data;
use crate::faction::Faction;
use cursive::traits::*;
use cursive::views::{Button, Dialog, DummyView, EditView, LinearLayout, SelectView};
use cursive::Cursive;

pub fn draw_view(siv: &mut Cursive, slug: String) {
    siv.pop_layer();
    let character = Character::from_slug(siv, slug.clone());
    let faction_select = SelectView::<Faction>::new()
        .with_all(
            character
                .factions
                .iter()
                .map(|i| (format!("{} - Level: {}", i.name, i.lvl), i.clone())),
        )
        .with_name("faction_select")
        .fixed_size((80, 20));

    let faction_buttons = LinearLayout::vertical()
        .child(Button::new("Add new", create_item))
        .child(Button::new("Delete", delete_item))
        .child(DummyView)
        .child(Button::new("Back", move |siv| {
            let select_factions = siv
                .call_on_name("faction_select", |view: &mut SelectView<Faction>| {
                    view.iter()
                        .map(|(_, faction)| faction.clone())
                        .collect::<Vec<Faction>>()
                })
                .unwrap();
            siv.with_user_data(|data: &mut Data| {
                data.character_list.get_mut(&slug).unwrap().factions = select_factions;
            });
            dashboard::draw_view(siv, slug.clone())
        }));

    siv.add_layer(
        Dialog::around(
            LinearLayout::horizontal()
                .child(faction_select)
                .child(faction_buttons),
        )
        .title("Factions"),
    );
}

pub fn create_item(siv: &mut Cursive) {
    siv.add_layer(
        Dialog::new()
            .title("Name of the faction")
            .content(
                EditView::new()
                    .on_submit(|siv, name| {
                        let faction = Faction::new(name);

                        siv.call_on_name("faction_select", |view: &mut SelectView<Faction>| {
                            view.add_item(
                                format!("{} - Level: {}", faction.name, faction.lvl),
                                faction.clone(),
                            );
                        });

                        siv.pop_layer();
                    })
                    .with_name("new_faction"),
            )
            .button("Back", |siv| {
                siv.pop_layer();
            }),
    );
}

pub fn delete_item(siv: &mut Cursive) {
    let mut select = siv
        .find_name::<SelectView<Faction>>("faction_select")
        .unwrap();
    match select.selected_id() {
        None => siv.add_layer(Dialog::info("No faction to remove")),
        Some(focus) => {
            select.remove_item(focus);
        }
    }
}
