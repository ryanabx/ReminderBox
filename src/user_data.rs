use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct UserData {
    pub reminders_list: ReminderList,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
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

#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
pub struct ReminderList(pub Vec<Reminder>);

impl ReminderList {
    /// Add a new reminder
    pub fn add_reminder(&mut self, reminder: Reminder) {
        self.0.push(reminder);
    }
    /// Remove a reminder by uuid
    pub fn remove_reminder(&mut self, id: Uuid) {
        self.0.retain(|a| a.id != id);
    }
    /// Get a mutable reference to a reminder
    pub fn reminder_mut(&mut self, id: Uuid) -> Option<&mut Reminder> {
        self.0.iter_mut().find(|a| a.id == id)
    }
    /// Get a mutable reference to a reminder
    pub fn reminder(&self, id: Uuid) -> Option<&Reminder> {
        self.0.iter().find(|a| a.id == id)
    }
    /// Insert a new blank reminder after this reminder
    pub fn new_blank_after(&mut self, id: Uuid) {
        let idx = self
            .0
            .iter()
            .position(|r| r.id == id)
            .unwrap_or(self.0.len().saturating_sub(1));
        self.0
            .insert(idx + 1, Reminder::new(Uuid::new_v4(), String::new(), false));
    }
}
