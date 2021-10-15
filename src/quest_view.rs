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
    let character = Character::from_slug(siv, slug);

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

    let buttons = LinearLayout::horizontal()
        .child(Button::new("View", |siv| {
            view_quest(siv);
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

fn view_quest(siv: &mut Cursive) {
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
        .call_on_name("kind_select", |view: &mut SelectView<Kind>| {
            view.selection().unwrap()
        })
        .unwrap();

    let checkboxes = siv
        .call_on_name("checkboxes", |view: &mut ListView| {
            let mut checkboxes = Checkboxes::new();
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

    siv.add_layer(Dialog::info(format!(
        "{}\n{}\n{}\n{}\n{:?}",
        title,
        description,
        link,
        kind.display(),
        checkboxes,
    )));
}
