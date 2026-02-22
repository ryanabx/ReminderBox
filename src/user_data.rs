use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct UserData {
    pub reminders: RwSignal<Vec<Reminder>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Reminder {
    pub id: Uuid,
    pub title: RwSignal<String>,
    pub notes: RwSignal<String>,
    pub completed: RwSignal<bool>,
    pub due_date: RwSignal<String>,
    pub due_time: RwSignal<String>,
}

impl Reminder {
    pub fn new(id: Uuid, title: String, completed: bool) -> Self {
        let title = RwSignal::new(title);
        let notes = RwSignal::new(String::new());
        let completed = RwSignal::new(completed);
        let due_date = RwSignal::new(String::new());
        let due_time = RwSignal::new(String::new());
        Reminder {
            id,
            title,
            notes,
            completed,
            due_date,
            due_time,
        }
    }

    /// `true` if the `Reminder` is empty, `false` otherwise.
    pub fn is_empty(&self) -> bool {
        self.title.get().is_empty()
            && self.notes.get().is_empty()
            && self.due_date.get().is_empty()
            && self.due_time.get().is_empty()
    }
}
