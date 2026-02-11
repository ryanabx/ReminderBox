use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::data::UserData;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Reminder {
    id: Uuid,
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

    pub fn id(&self) -> Uuid {
        self.id
    }
}

#[component]
pub fn ReminderWidget(reminder: Reminder) -> impl IntoView {
    let state_setter = use_context::<WriteSignal<UserData>>().expect("Could not find user data");
    view! {
        <div>
            <input type="checkbox" bind:checked=reminder.completed/>
            <input type="text" placeholder="Enter a reminder..." bind:value=reminder.title />
            <button type="button" on:click=move |_| {}>"..."</button>
            <button type="button" on:click=move |_| {
                state_setter.update(|state| {
                    state.reminders_list.remove_reminder(reminder.id());
                });
            }>"⌫"</button>
        </div>
    }
}
