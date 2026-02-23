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
    pub title: String,
    pub notes: String,
    pub completed: bool,
    pub due_date: String,
    pub due_time: String,
}

impl Reminder {
    pub fn new(id: Uuid, title: String, completed: bool) -> Self {
        let notes = String::new();
        let due_date = String::new();
        let due_time = String::new();
        Reminder {
            id,
            title,
            notes,
            completed,
            due_date,
            due_time,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.title.is_empty()
            && self.notes.is_empty()
            && self.due_date.is_empty()
            && self.due_time.is_empty()
    }
}

pub fn update_reminder(
    reminders: RwSignal<Vec<Reminder>>,
    id: Uuid,
    f: impl FnOnce(&mut Reminder),
) {
    reminders.update(|list| {
        if let Some(r) = list.iter_mut().find(|r| r.id == id) {
            f(r);
        }
    });
}

pub fn get_reminder(reminders: RwSignal<Vec<Reminder>>, id: Uuid) -> Option<Reminder> {
    reminders.with(|list| list.iter().find(|r| r.id == id).cloned())
}
