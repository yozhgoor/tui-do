use cursive::Cursive;

#[derive(Clone)]
pub struct Quest {
    kind: QuestKind,
    name: String,
}

#[derive(Clone)]
pub enum QuestKind {
    Daily,
    Special,
}

pub fn get_quest_view(s: &mut Cursive, _quests: Vec<Quest>) {
    s.quit()
}
