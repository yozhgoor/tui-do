use cursive::Cursive;

use crate::character::Character;

pub fn get_dashboard(s: &mut Cursive, _character: &Character) {
    s.quit()
}
