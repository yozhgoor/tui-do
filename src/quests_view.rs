use crate::character::Character;
use crate::dashboard;
use crate::quest::Quest;
use crate::quest_view;
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
    siv.pop_layer();
    let character = Character::from_slug(siv, slug.clone());

    let character_quests_debug = TextView::new(format!("{:?}", character.quests.clone()));

    let mut panel = TabPanel::new();
    for (label, quests) in character.quests.clone() {
        panel.add_tab(create_tab(slug.clone(), label.clone(), quests.clone()))
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
            .child(character_quests_debug)
            .child(frame)
            .child(Button::new("Add", {
                let slug = slug.clone();
                move |siv| add_quest(siv, slug.clone())
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

fn create_tab(slug: String, label: String, quests: HashMap<String, Quest>) -> NamedView<Dialog> {
    let mut quests_select = SelectView::<Quest>::new();
    for (_, quest) in quests {
        quests_select.add_item(quest.display_for_presentation(), quest)
    }
    Dialog::around(
        LinearLayout::vertical()
            .child(TextView::new(format!("{}'s quests belong here", label)))
            .child(DummyView)
            .child(
                quests_select
                    .on_submit(move |siv, item| {
                        quest_view::draw_view(siv, slug.clone(), item.clone());
                    })
                    .with_name("quests_select"),
            ),
    )
    .with_name(label)
}

fn add_quest(siv: &mut Cursive, slug: String) {
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
        .child(TextView::new("Daily Quest: "))
        .child(DummyView)
        .child(Checkbox::new().with_name("quest_kind"));

    let checkboxes_buttons = LinearLayout::horizontal()
        .child(Button::new("Add checkbox", |siv| add_checkbox(siv)))
        .child(DummyView)
        .child(Button::new("Remove checkbox", |siv| remove_checkbox(siv)));

    let checkboxes = LinearLayout::vertical()
        .child(TextView::new("Todos:"))
        .child(DummyView)
        .child(ListView::new().with_name("checkboxes"))
        .child(DummyView)
        .child(checkboxes_buttons);

    let buttons = LinearLayout::horizontal()
        .child(Button::new("Save", move |siv| {
            save_quest(siv, slug.clone());
        }))
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
                .child(checkboxes)
                .child(DummyView)
                .child(buttons),
        )
        .fixed_size((80, 40)),
    );
}

pub fn add_checkbox(siv: &mut Cursive) {
    siv.add_layer(
        Dialog::around(EditView::new().on_submit(|siv, label| {
            siv.call_on_name("checkboxes", |view: &mut ListView| {
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

pub fn remove_checkbox(siv: &mut Cursive) {
    siv.call_on_name("checkboxes", |view: &mut ListView| {
        view.remove_child(view.focus())
    });
}

fn save_quest(siv: &mut Cursive, slug: String) {
    let title = siv
        .call_on_name("quest_title", |view: &mut TextArea| {
            view.get_content().to_owned()
        })
        .unwrap();

    let description = siv
        .call_on_name("quest_description", |view: &mut TextArea| {
            view.get_content().to_owned()
        })
        .unwrap();

    let link = siv
        .call_on_name("link_select", |view: &mut SelectView| {
            view.selection().unwrap()
        })
        .unwrap();

    let kind = siv
        .call_on_name("quest_kind", |view: &mut Checkbox| view.is_checked())
        .unwrap();

    let checkboxes = siv
        .call_on_name("checkboxes", |view: &mut ListView| {
            let mut checkboxes = HashMap::new();
            for listchild in view.children() {
                if let ListChild::Row(label, boxed_view) = listchild {
                    if let Some(unboxed_view) = boxed_view.downcast_ref::<Checkbox>() {
                        checkboxes.insert(label.clone(), unboxed_view.is_checked());
                    };
                }
            }
            checkboxes
        })
        .unwrap();

    let character = Character::from_slug(siv, slug.clone());

    let quest = Quest::new(title, description, link.to_string(), kind, checkboxes);

    if let Some(quests) = character.quests.get_mut(&quest.link) {
        if quests.contains_key(&quest.title) {
            Dialog::info("This quest already exist");
        } else {
            quests.insert(quest.title.clone(), quest);
        }
    } else {
        let mut map = HashMap::new();
        map.insert(quest.title.clone(), quest.clone());
        character.quests.insert(quest.link, map);
    }

    siv.pop_layer();

    draw_view(siv, slug)
}
