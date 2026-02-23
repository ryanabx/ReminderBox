use leptos::prelude::*;
use uuid::Uuid;

use crate::{
    pages::{reminders_page::RemindersPage, settings_page::ReminderSettings},
    user_data::UserData,
};

#[derive(Clone, Debug, Default)]
pub enum Page {
    #[default]
    Main,
    Settings(Uuid),
}

#[component]
pub fn App() -> impl IntoView {
    // The `user data` is a signal, since we need to reactively update the list
    let (user_data, _set_user_data) = signal(get_local_storage());
    // The current page we are on
    let (page, set_page) = signal(Page::default());

    provide_context(set_page);

    Effect::new(move |_| {
        local_storage_effect(&user_data);
    });

    view! {
        {
            move || match page.get() {
                Page::Main => view! { <RemindersPage reminder_list=user_data.get().reminders/> }.into_any(),
                Page::Settings(id) => view! { <ReminderSettings reminder_id=id reminder_list=user_data.get().reminders/> }.into_any(),
            }
        }
    }
}

pub const LOCAL_STORAGE_KEY: &str = "reminderbox";

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
