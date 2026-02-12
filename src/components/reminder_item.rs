use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::data::UserData;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Reminder {
    pub id: Uuid,
    title: RwSignal<String>,
    completed: RwSignal<bool>,
}

impl Reminder {
    pub fn new(id: Uuid, title: String, completed: bool) -> Self {
        let title = RwSignal::new(title);
        let completed = RwSignal::new(completed);
        Reminder {
            id,
            title,
            completed,
        }
    }
}

#[component]
pub fn ReminderWidget(reminder: Reminder) -> impl IntoView {
    let state_setter = use_context::<WriteSignal<UserData>>().expect("Could not find user data");
    view! {
        <div class="flex flex-row space-x-2 py-2">
            <input type="checkbox" class="size-4" bind:checked=reminder.completed/>
            <input type="text" class="grow py-1" placeholder="Enter a reminder..." bind:value=reminder.title />
            <button type="button" on:click=move |_| {}>"..."</button>
            <button type="button" on:click=move |_| {
                state_setter.update(|state| {
                    state.reminders_list.remove_reminder(reminder.id);
                });
            }>"⌫"</button>
        </div>
    }
}
