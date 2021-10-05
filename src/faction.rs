use cursive::Cursive;

#[derive(Clone)]
pub struct Faction {
    pub name: String,
    pub reputation: u32,
    pub lvl: u32,
}

pub fn get_faction_view(s: &mut Cursive, factions: Vec<Faction>) {
    s.quit()
}
