use crate::quest::Quest;
use crate::quests_view;
use cursive::view::Nameable;
use cursive::{
    views::{Button, Dialog, DummyView, LinearLayout, ListView, TextArea, TextView},
    Cursive,
};

pub fn draw_view(siv: &mut Cursive, slug: String, quest: Quest) {
    siv.pop_layer();

    let title = LinearLayout::vertical()
        .child(TextView::new("Title:"))
        .child(DummyView)
        .child(
            TextArea::new()
                .content(quest.title)
                .with_name("quest_title"),
        );
    let description = LinearLayout::vertical()
        .child(TextView::new("Description:"))
        .child(DummyView)
        .child(
            TextArea::new()
                .content(quest.description)
                .with_name("quest_description"),
        );
    let status = LinearLayout::horizontal()
        .child(TextView::new("Status: "))
        .child(TextView::new(quest.status.display()));

    let link = LinearLayout::horizontal()
        .child(TextView::new("Faction: "))
        .child(TextView::new(quest.link));

    let kind = if quest.daily_quest {
        TextView::new("Daily quest")
    } else {
        TextView::new("Main quest")
    };

    let checkboxes_buttons = LinearLayout::horizontal()
        .child(Button::new("Add checkbox", |siv| {
            quests_view::add_checkbox(siv)
        }))
        .child(DummyView)
        .child(Button::new("Remove checkbox", |siv| {
            quests_view::remove_checkbox(siv)
        }));

    let checkboxes = LinearLayout::vertical()
        .child(TextView::new("Todos:"))
        .child(DummyView)
        .child(ListView::new().with_name("checkboxes"))
        .child(DummyView)
        .child(checkboxes_buttons);

    let quest_card = LinearLayout::vertical()
        .child(title)
        .child(DummyView)
        .child(description)
        .child(DummyView)
        .child(status)
        .child(DummyView)
        .child(link)
        .child(DummyView)
        .child(kind)
        .child(checkboxes);

    siv.add_layer(Dialog::around(
        LinearLayout::vertical()
            .child(quest_card)
            .child(DummyView)
            .child(Button::new("Back", {
                move |siv| {
                    siv.pop_layer();
                    quests_view::draw_view(siv, slug.clone());
                }
            })),
    ))
}
