use crate::character::Character;
use crate::dashboard;
use cursive::traits::*;
use cursive::views::{
    Button, Checkbox, Dialog, DummyView, EditView, LinearLayout, ListView, NamedView, SelectView,
    TextArea, TextView,
};
use cursive::Cursive;
use cursive_tabs::TabPanel;

pub fn draw_view(siv: &mut Cursive, slug: String) {
    let slug = slug.clone();

    siv.pop_layer();
    let character = Character::from_slug(siv, slug.clone());
    let mut panel = TabPanel::new();

    for label in character.quests.keys() {
        panel.add_tab(create_tab(label.clone()))
    }

    siv.add_layer(Dialog::around(
        LinearLayout::vertical()
            .child(Button::new("Add", {
                let slug = slug.clone();
                move |siv| add_item(siv, slug.clone())
            }))
            .child(DummyView)
            .child(Button::new("Back", {
                move |siv| {
                    siv.pop_layer();
                    dashboard::draw_view(siv, slug.clone());
                }
            })),
    ));
}

fn create_tab(title: String) -> NamedView<Dialog> {
    Dialog::around(
        LinearLayout::vertical().child(TextView::new(format!("{}'s quests belong here", title))),
    )
    .with_name(title)
}

fn add_item(siv: &mut Cursive, slug: String) {
    let character = Character::from_slug(siv, slug);

    let title = LinearLayout::horizontal()
        .child(TextView::new("Title:"))
        .child(DummyView)
        .child(EditView::new().with_name("quest_title"));

    let description = LinearLayout::horizontal()
        .child(TextView::new("Description:"))
        .child(DummyView)
        .child(TextArea::new().with_name("quest_description"));

    let mut link_select = SelectView::new().item(character.name.clone(), character.name.clone());
    for faction in character.factions.iter() {
        link_select.add_item(faction.name.clone(), faction.name.clone())
    }

    let checklist = LinearLayout::vertical()
        .child(TextView::new("Checklist:"))
        .child(DummyView).with_name("checklist");

    let buttons = LinearLayout::horizontal().child(Button::new("Add checkbox", |siv| {
        todo!()
    }))

    siv.add_layer(Dialog::around(
        LinearLayout::vertical()
            .child(title)
            .child(DummyView)
            .child(description)
            .child(DummyView)
            .child(link_select.with_name("link_select"))
            .child(DummyView)
            .child(checklist)
            .child(DummyView)
            .child(buttons),
    ));
}
