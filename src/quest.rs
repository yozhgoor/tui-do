use std::collections::HashMap;

pub type Checkboxes = HashMap<String, bool>;
pub type Lists = HashMap<String, Checkboxes>;

#[derive(Clone)]
pub struct Quest {
    pub title: String,
    pub description: String,
    pub status: QuestStatus,
    pub link: String,
    pub kind: Kind,
    // due_date: Date,
    pub checkboxes: Checkboxes,
    pub lists: Lists,
    // reward: Reward,
}

impl Quest {
    pub fn new(
        title: String,
        description: String,
        link: String,
        kind: Kind,
        checkboxes: Checkboxes,
        lists: Lists,
    ) -> Quest {
        Quest {
            title,
            description,
            status: QuestStatus::Pending,
            link,
            kind,
            checkboxes,
            lists,
        }
    }
    pub fn display_for_presentation(&self) -> String {
        format!(
            "{} - {} - {} | {} todos, {} lists",
            self.title,
            self.status.display(),
            self.kind.display(),
            self.checkboxes.len(),
            self.lists.len(),
        )
    }
}

#[derive(Clone)]
pub enum Kind {
    Daily,
    Special,
}

impl Kind {
    pub fn display(&self) -> &str {
        match self {
            Kind::Daily => "Daily",
            Kind::Special => "Special",
        }
    }
}

#[derive(Clone)]
pub enum QuestStatus {
    Pending,
    // InProgress,
    // Done,
}

impl QuestStatus {
    pub fn display(&self) -> &str {
        match self {
            QuestStatus::Pending => "Pending",
            // QuestStatus::InProgress => "In Progress",
            // QuestStatus::Done => "Done",
        }
    }
}
