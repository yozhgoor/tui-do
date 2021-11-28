use std::collections::HashMap;

pub type Checkboxes = HashMap<String, bool>;

#[derive(Clone, Debug)]
pub struct Quest {
    pub title: String,
    pub description: String,
    pub status: QuestStatus,
    pub link: String,
    pub daily_quest: bool,
    pub checkboxes: Checkboxes,
    // due_date: Date,
    // lists: Lists,
    // reward: Reward,
}

impl Quest {
    pub fn new(
        title: String,
        description: String,
        link: String,
        daily_quest: bool,
        checkboxes: HashMap<String, bool>,
    ) -> Quest {
        Quest {
            title,
            description,
            status: QuestStatus::Pending,
            link,
            daily_quest,
            checkboxes,
        }
    }

    pub fn display_quest_kind(&self) -> &str {
        if self.daily_quest {
            "Daily quest"
        } else {
            "Main quest"
        }
    }

    pub fn display_for_presentation(&self) -> String {
        format!(
            "{} - {} - {} | {} todos",
            self.title,
            self.status.display(),
            self.display_quest_kind(),
            self.checkboxes.len(),
        )
    }
}

#[derive(Clone, Debug)]
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
