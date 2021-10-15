use crate::quest::Quest;
use cursive::Cursive;

pub fn draw_view(siv: &mut Cursive, _quest: Quest) {
    siv.quit();
}
