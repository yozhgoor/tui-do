use cursive::Cursive;

#[derive(Clone)]
pub struct Quest {
    name: String,
    // kind: QuestKind,
}

/*
#[derive(Clone)]
pub enum QuestKind {
    Daily,
    Special,
}
*/

pub fn get_quest_view(s: &mut Cursive, _quests: Vec<Quest>) {
    s.quit()
}
