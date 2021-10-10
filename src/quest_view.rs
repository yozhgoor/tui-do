use crate::character::Character;
use crate::dashboard;
use crate::quest::{Checkboxes, Kind, Lists, Quest};
use cursive::traits::*;
use cursive::views::ListChild;
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
        .child(TextArea::new().content("").with_name("quest_title"));

    let description = LinearLayout::vertical()
        .child(TextView::new("Description:"))
        .child(DummyView)
        .child(TextArea::new().content("").with_name("quest_description"));

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
                .item("Daily Quest", Kind::Daily)
                .item("Special Quest", Kind::Special)
                .popup()
                .with_name("kind_select"),
        );

    let checkboxes_buttons = LinearLayout::horizontal()
        .child(Button::new("Add a checkbox", |siv| add_checkbox(siv)))
        .child(DummyView)
        .child(Button::new("Remove checkbox", |siv| remove_checkbox(siv)));

    let checkboxes = LinearLayout::vertical()
        .child(TextView::new("Todos:"))
        .child(DummyView)
        .child(ListView::new().with_name("checkboxes"))
        .child(DummyView)
        .child(checkboxes_buttons);

    let lists_buttons = LinearLayout::horizontal()
        .child(Button::new("Add list", |siv| add_list(siv)))
        .child(DummyView)
        .child(Button::new("Remove list", |siv| remove_list(siv)));

    let lists = LinearLayout::vertical()
        .child(TextView::new("lists:"))
        .child(DummyView)
        .child(ListView::new().with_name("lists").scrollable())
        .child(DummyView)
        .child(lists_buttons);

    let buttons = LinearLayout::horizontal()
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
                .child(checkboxes)
                .child(DummyView)
                .child(lists)
                .child(DummyView)
                .child(buttons),
        )
        .fixed_size((80, 40)),
    );
}

fn add_list(siv: &mut Cursive) {
    siv.add_layer(
        Dialog::around(EditView::new().on_submit(|siv, label| {
            let list_name = TextView::new(label).with_name("list_name");
            let list_overview = LinearLayout::vertical()
                .child(list_name)
                .child(DummyView)
                .child(ListView::new().with_name("list_checkboxes"));

            let list_buttons = LinearLayout::horizontal()
                .child(Button::new("Add Checkbox", |siv| {
                    siv.add_layer(
                        Dialog::around(EditView::new().on_submit(|siv, label| {
                            siv.call_on_name("list_checkboxes", |view: &mut ListView| {
                                view.add_child(label, Checkbox::new());
                            });
                            siv.pop_layer();
                        }))
                        .button("Back", |siv| {
                            siv.pop_layer();
                        }),
                    )
                }))
                .child(DummyView)
                .child(Button::new("Remove checkbox", |siv| {
                    siv.call_on_name("list_checkboxes", |view: &mut ListView| {
                        view.remove_child(view.focus())
                    });
                }))
                .child(DummyView)
                .child(Button::new("Save List", |siv| {
                    let list_name = siv
                        .call_on_name("list_name", |view: &mut TextView| {
                            view.get_content().source().to_string()
                        })
                        .unwrap();

                    let checkboxes = siv
                        .call_on_name("list_checkboxes", |view: &mut ListView| {
                            let mut checkboxes = Checkboxes::new();
                            for child in view.children() {
                                match child {
                                    ListChild::Row(label, boxed_view) => {
                                        if let Some(view) = boxed_view.downcast_ref::<Checkbox>() {
                                            checkboxes.insert(label.clone(), view.is_checked());
                                        };
                                    }
                                    _ => {}
                                }
                            }
                            checkboxes
                        })
                        .unwrap();

                    let mut checkboxes_view = ListView::new();
                    for (label, checkbox_value) in checkboxes {
                        if checkbox_value {
                            checkboxes_view.add_child(&label, Checkbox::new().checked());
                        } else {
                            checkboxes_view.add_child(&label, Checkbox::new());
                        }
                    }

                    siv.call_on_name("lists", |view: &mut ListView| {
                        view.add_child(
                            &list_name,
                            checkboxes_view.with_name(format!("{}", list_name)),
                        )
                    });
                    siv.pop_layer();
                    siv.pop_layer();
                }))
                .child(Button::new("Back", |siv| {
                    siv.pop_layer();
                }));

            siv.add_layer(Dialog::around(
                LinearLayout::vertical()
                    .child(list_overview)
                    .child(DummyView)
                    .child(list_buttons),
            ))
        }))
        .title("Insert a label")
        .button("Back", |siv| {
            siv.pop_layer();
        }),
    );
}

fn remove_list(siv: &mut Cursive) {
    siv.call_on_name("lists", |view: &mut ListView| {
        view.remove_child(view.focus())
    });
}

fn add_checkbox(siv: &mut Cursive) {
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

fn remove_checkbox(siv: &mut Cursive) {
    siv.call_on_name("checkboxes", |view: &mut ListView| {
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
        .call_on_name("kind_select", |view: &mut SelectView<Kind>| {
            let id = view.selected_id().unwrap();
            let (_, value) = view.get_item(id).unwrap();
            value.clone()
        })
        .unwrap();

    let checkboxes = siv
        .call_on_name("checkboxes", |view: &mut ListView| {
            let mut checkboxes = Checkboxes::new();
            for listchild in view.children() {
                match listchild {
                    ListChild::Row(label, boxed_view) => {
                        if let Some(unboxed_view) = boxed_view.downcast_ref::<Checkbox>() {
                            checkboxes.insert(label.clone(), unboxed_view.is_checked());
                        };
                    }
                    _ => {}
                }
            }
            checkboxes
        })
        .unwrap();

    let list_names = siv
        .call_on_name("lists", |view: &mut ListView| {
            let mut list_names = Vec::new();
            for listchild in view.children() {
                match listchild {
                    ListChild::Row(label, boxed_view) => {
                        if let Some(_) = boxed_view.downcast_ref::<TextView>() {
                            list_names.push(label.clone())
                        }
                    }
                    _ => {}
                }
            }
            list_names
        })
        .unwrap();

    let mut lists = Lists::new();
    for list_name in list_names {
        siv.call_on_name(&list_name, |view: &mut ListView| {
            let mut list_checkboxes_map = Checkboxes::new();
            for listchild in view.children() {
                match listchild {
                    ListChild::Row(label, boxed_view) => {
                        if let Some(unboxed_view) = boxed_view.downcast_ref::<Checkbox>() {
                            list_checkboxes_map.insert(label.clone(), unboxed_view.is_checked());
                        };
                    }
                    _ => {}
                }
            }
            lists.insert(list_name.clone(), list_checkboxes_map)
        });
    }

    if title.is_empty() {
        siv.add_layer(Dialog::info("No Title given"));
    } else {
        let quest = Quest::new(
            title.to_string(),
            description,
            link.to_string(),
            kind,
            checkboxes,
            lists,
        );

        let character = Character::from_slug(siv, slug.clone());
        if character.quests.contains_key(&quest.link) {
            character.quests.get_mut(&quest.link).unwrap().push(quest);
        } else {
            character.quests.insert(quest.link.clone(), vec![quest]);
        }
    }
}
