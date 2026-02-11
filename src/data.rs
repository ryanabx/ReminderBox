use leptos::prelude::*;
use serde::{Deserialize, Serialize};

use crate::components::reminder_list::ReminderList;

pub const LOCAL_STORAGE_KEY: &str = "reminderbox";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct UserData {
    pub reminders_list: ReminderList,
}

impl Default for UserData {
    fn default() -> Self {
        get_local_storage()
    }
}

pub fn local_storage_effect(user_data: &ReadSignal<UserData>) {
    if let Ok(Some(storage)) = window().local_storage() {
        let json = serde_json::to_string(user_data).expect("couldn't serialize user data");
        if storage.set_item(LOCAL_STORAGE_KEY, &json).is_err() {
            leptos::logging::error!("error while trying to set item in localStorage");
        }
    }
}

pub fn get_local_storage() -> UserData {
    window()
        .local_storage()
        .ok()
        .flatten()
        .and_then(|storage| {
            storage
                .get_item(LOCAL_STORAGE_KEY)
                .ok()
                .flatten()
                .and_then(|value| serde_json::from_str::<UserData>(&value).ok())
        })
        .unwrap_or_default()
}
