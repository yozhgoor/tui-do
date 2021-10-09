use crate::character::Character;
use crate::dashboard;
use crate::quest::{Quest, QuestKind};
use cursive::traits::*;
use cursive::views::ListChild;
use cursive::views::{
    Button, Checkbox, Dialog, DummyView, EditView, LinearLayout, ListView, NamedView, SelectView,
    TextArea, TextView,
};
use cursive::Cursive;
use cursive_tabs::TabPanel;
use std::collections::HashMap;

pub fn draw_view(siv: &mut Cursive, slug: String) {
    let slug = slug.clone();

    siv.pop_layer();
    let character = Character::from_slug(siv, slug.clone());

    let mut panel = TabPanel::new();
    for (label, quests) in character.quests.clone() {
        panel.add_tab(create_tab(label.clone(), quests.clone()))
    }

    let frame = if character.quests.is_empty() {
        LinearLayout::vertical()
            .child(TextView::new("Click Add to create a quest"))
            .child(DummyView)
    } else {
        LinearLayout::vertical().child(panel)
    };

    siv.add_layer(Dialog::around(
        LinearLayout::vertical()
            .child(frame)
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

fn create_tab(label: String, quests: Vec<Quest>) -> NamedView<Dialog> {
    let mut quests_select = SelectView::<Quest>::new();
    for quest in quests {
        quests_select.add_item(quest.display_for_presentation(), quest)
    }
    Dialog::around(
        LinearLayout::vertical()
            .child(TextView::new(format!("{}'s quests belong here", label)))
            .child(DummyView)
            .child(quests_select.with_name("quests_select")),
    )
    .with_name(label)
}

fn add_item(siv: &mut Cursive, slug: String) {
    let character = Character::from_slug(siv, slug.clone());

    let title = LinearLayout::vertical()
        .child(TextView::new("Title:"))
        .child(DummyView)
        .child(TextArea::new().with_name("quest_title"));

    let description = LinearLayout::vertical()
        .child(TextView::new("Description:"))
        .child(DummyView)
        .child(TextArea::new().with_name("quest_description"));

    let mut link_select = SelectView::new().item(character.name.clone(), character.name.clone());
    for faction in character.factions.iter() {
        link_select.add_item(faction.name.clone(), faction.name.clone())
    }
    let link = LinearLayout::horizontal()
        .child(TextView::new("Faction: "))
        .child(DummyView)
        .child(link_select.popup().with_name("link_select"));

    let kind = LinearLayout::horizontal()
        .child(TextView::new("Quest kind: "))
        .child(DummyView)
        .child(
            SelectView::new()
                .item("Daily Quest", QuestKind::Daily)
                .item("Special Quest", QuestKind::Special)
                .popup()
                .with_name("kind_select"),
        );

    let checklist = LinearLayout::vertical()
        .child(TextView::new("Checklist:"))
        .child(DummyView)
        .child(ListView::new().with_name("checklist").scrollable());

    let buttons = LinearLayout::horizontal()
        .child(Button::new("Add a checkbox", |siv| add_checkbox(siv)))
        .child(DummyView)
        .child(Button::new("Remove a checkbox", |siv| remove_checkbox(siv)))
        .child(DummyView)
        .child(Button::new("Save", move |siv| {
            save_quest(siv, slug.clone())
        }))
        .child(DummyView)
        .child(Button::new("Back", move |siv| {
            siv.pop_layer();
        }));

    siv.add_layer(
        Dialog::around(
            LinearLayout::vertical()
                .child(title)
                .child(DummyView)
                .child(description)
                .child(DummyView)
                .child(link)
                .child(DummyView)
                .child(kind)
                .child(DummyView)
                .child(checklist)
                .child(DummyView)
                .child(buttons),
        )
        .fixed_size((80, 40)),
    );
}

fn add_checkbox(siv: &mut Cursive) {
    siv.add_layer(
        Dialog::around(EditView::new().on_submit(|siv, label| {
            siv.call_on_name("checklist", |view: &mut ListView| {
                view.add_child(label, Checkbox::new());
            });
            siv.pop_layer();
        }))
        .button("Back", |siv| {
            siv.pop_layer();
        })
        .title("Insert a label"),
    )
}

fn remove_checkbox(siv: &mut Cursive) {
    siv.call_on_name("checklist", |view: &mut ListView| {
        view.remove_child(view.focus())
    });
}

fn save_quest(siv: &mut Cursive, slug: String) {
    let title = siv
        .call_on_name("quest_title", |view: &mut EditView| view.get_content())
        .unwrap();
    let description = siv
        .call_on_name("quest_description", |view: &mut TextArea| {
            view.get_content().to_string()
        })
        .unwrap();
    let link = siv
        .call_on_name("link_select", |view: &mut SelectView<String>| {
            view.selection().unwrap()
        })
        .unwrap();
    let kind = siv
        .call_on_name("kind_select", |view: &mut SelectView<QuestKind>| {
            let id = view.selected_id().unwrap();
            let (_, value) = view.get_item(id).unwrap();
            value.clone()
        })
        .unwrap();
    let checklist = siv
        .call_on_name("checklist", |view: &mut ListView| {
            let mut check_map = HashMap::new();
            for list in view.children() {
                match list {
                    ListChild::Row(label, boxed_view) => {
                        if let Some(view) = boxed_view.downcast_ref::<Checkbox>() {
                            check_map.insert(label.clone(), view.is_checked());
                        };
                    }
                    _ => {}
                }
            }
            check_map
        })
        .unwrap();

    if title.is_empty() {
        siv.add_layer(Dialog::info("No Title given"));
    } else {
        let quest = Quest::new(
            title.to_string(),
            description,
            link.to_string(),
            kind,
            checklist,
        );

        let character = Character::from_slug(siv, slug.clone());
        if character.quests.contains_key(&quest.link) {
            character.quests.get_mut(&quest.link).unwrap().push(quest);
        } else {
            character.quests.insert(quest.link.clone(), vec![quest]);
        }
    }
}
