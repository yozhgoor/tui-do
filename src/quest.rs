use std::collections::HashMap;

#[derive(Clone)]
pub struct Quest {
    pub title: String,
    pub description: String,
    pub status: QuestStatus,
    pub link: String,
    pub kind: QuestKind,
    // due_date: Date,
    pub checklist: HashMap<String, bool>,
    // reward: Reward,
}

impl Quest {
    pub fn new(
        title: String,
        description: String,
        link: String,
        kind: QuestKind,
        checklist: HashMap<String, bool>,
    ) -> Quest {
        Quest {
            title,
            description,
            status: QuestStatus::Pending,
            link,
            kind,
            checklist,
        }
    }
    pub fn display_for_presentation(&self) -> String {
        format!(
            "{} - {} - {} | {}",
            self.title,
            self.status.display(),
            self.kind.display(),
            self.checklist.len()
        )
    }
}

#[derive(Clone)]
pub enum QuestKind {
    Daily,
    Special,
}

impl QuestKind {
    fn display(&self) -> &str {
        match self {
            QuestKind::Daily => "Daily",
            QuestKind::Special => "Special",
        }
    }
}

#[derive(Clone)]
pub enum QuestStatus {
    Pending,
    InProgress,
    Done,
}

impl QuestStatus {
    pub fn display(&self) -> &str {
        match self {
            QuestStatus::Pending => "Pending",
            QuestStatus::InProgress => "In Progress",
            QuestStatus::Done => "Done",
        }
    }
}
