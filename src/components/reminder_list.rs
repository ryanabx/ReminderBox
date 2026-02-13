use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use web_sys::KeyboardEvent;

use crate::{
    components::reminder_item::{Reminder, ReminderWidget},
    data::UserData,
};

#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
pub struct ReminderList(Vec<Reminder>);

impl ReminderList {
    /// Add a new reminder
    pub fn add_reminder(&mut self, reminder: Reminder) {
        self.0.push(reminder);
    }
    /// Remove a reminder by uuid
    pub fn remove_reminder(&mut self, id: Uuid) {
        self.0.retain(|a| a.id != id);
    }
}

#[component]
pub fn ReminderListWidget(
    reminder_list: impl Fn() -> ReminderList + Send + Sync + 'static,
) -> impl IntoView {
    let reminders = move || reminder_list().0;

    view! {
        <div class="toplevel-container overflow-y-auto overscroll-auto">
            <ul class="space-y-0">
                <For each=reminders key=|reminder| reminder.id let:reminder>
                    <ReminderWidget reminder/>
                </For>
            </ul>
        </div>
        <NewReminderBox/>
    }
}

// const ESCAPE_KEY: u32 = 27;
const ENTER_KEY: u32 = 13;

#[component]
fn NewReminderBox() -> impl IntoView {
    let state_setter = use_context::<WriteSignal<UserData>>().expect("Could not find user data");

    let text = RwSignal::new(String::new());

    let add_todo = move |ev: KeyboardEvent| {
        ev.stop_propagation();
        let key_code = ev.key_code();
        if key_code == ENTER_KEY {
            let title = text.get();
            let title = title.trim();
            if !title.is_empty() {
                let new_reminder = Reminder::new(Uuid::new_v4(), title.to_string(), false);
                state_setter.update(|state| {
                    state.reminders_list.add_reminder(new_reminder);
                });
                text.set(String::new());
            }
        }
    };

    view! {
        <div class="flex flex-row bg-transparent px-2 py-2 w-full shadow-2xl space-x-2">
            <input type="text" class="grow standard-input new-reminder-input" placeholder="Enter a new reminder" bind:value=text on:keydown=add_todo />
            <button type="button" class="btn add-button" disabled=move || text.get().trim().is_empty() on:click=move |_| {
                let title = text.get();
                let title = title.trim();
                if !title.is_empty() {
                    let new_reminder = Reminder::new(Uuid::new_v4(), title.to_string(), false);
                    state_setter.update(|state| {
                        state.reminders_list.add_reminder(new_reminder);
                    });
                    text.set(String::new());
                }
            }>+</button>
        </div>
    }
}
