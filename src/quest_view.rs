use cursive::Cursive;

use crate::quest::Quest;

pub fn draw_view(siv: &mut Cursive, _quests: Vec<Quest>) {
    siv.quit()
}
