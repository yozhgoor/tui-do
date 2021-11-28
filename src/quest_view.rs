use crate::quest::Quest;
use crate::quests_view;
use cursive::{
    views::{Button, Dialog, DummyView, LinearLayout, TextView},
    Cursive,
};

pub fn draw_view(siv: &mut Cursive, slug: String, quest: Quest) {
    siv.pop_layer();

    let title = TextView::new(quest.title);
    let description = TextView::new(quest.description);
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

    let mut checkboxes = LinearLayout::vertical();

    for (label, state) in quest.checkboxes {
        if state {
            checkboxes.add_child(TextView::new(format!("{} : [*]", label)));
        } else {
            checkboxes.add_child(TextView::new(format!("{} : [ ]", label)));
        }
    }

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
            .child(Button::new("Edit quest", |_siv| todo!("edit quest")))
            .child(Button::new("Change status", |_siv| {
                todo!("change quest status")
            }))
            .child(Button::new("Delete quest", |_siv| todo!("delete quest")))
            .child(Button::new("Back", {
                move |siv| {
                    siv.pop_layer();
                    quests_view::draw_view(siv, slug.clone());
                }
            })),
    ))
}
