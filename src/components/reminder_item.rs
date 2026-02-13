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
        <div draggable=true class="flex flex-row space-x-2">
            <ReminderCheckbox completed={reminder.completed} />
            <p class="grow py-2 px-2">{reminder.title}</p>
            <button type="button" class="remove-button w-6" on:click=move |_| {
                state_setter.update(|state| {
                    state.reminders_list.remove_reminder(reminder.id);
                });
            }>X</button>
        </div>
    }
}

#[component]
pub fn ReminderCheckbox(completed: RwSignal<bool>) -> impl IntoView {
    view! {
        <label class="flex items-center cursor-pointer space-x-3">
        <input type="checkbox" class="hidden peer" bind:checked=completed />
        <div class="w-6 h-6 rounded-full border-2 border-gray-400 peer-checked:bg-blue-500 peer-checked:border-blue-500 transition-colors duration-150 flex items-center justify-center">
            <svg class="w-3 h-3 {} text-white transition-opacity" class:opacity-0=move || {!completed.get()} fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" d="M5 13l4 4L19 7" />
            </svg>
        </div>
        </label>
    }
}
