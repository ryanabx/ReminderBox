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
    /// Get a mutable reference to a reminder
    pub fn reminder_mut(&mut self, id: Uuid) -> Option<&mut Reminder> {
        self.0.iter_mut().find(|a| a.id == id)
    }
    /// Get a mutable reference to a reminder
    pub fn reminder(&self, id: Uuid) -> Option<&Reminder> {
        self.0.iter().find(|a| a.id == id)
    }
}

#[component]
pub fn ReminderListWidget(
    reminder_list: impl Fn() -> ReminderList + Send + Sync + 'static,
) -> impl IntoView {
    let reminders = move || reminder_list().0;

    view! {
        <div class="toplevel-container scroll-container">
            <ul class="space-y-0 w-full flex flex-col items-center">
                <hr />
                <For each=reminders key=|reminder| reminder.id let:reminder>
                    <ReminderWidget reminder/>
                    <hr />
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
        <div class="flex flex-row rounded-full bg-neutral-300/50 dark:bg-neutral-800/50 px-2 py-2 constrain-x">
            <input type="text" class="grow reminder-input rounded-full" placeholder="Enter a new reminder" bind:value=text on:keydown=add_todo />
            <button type="button" class="btn btn-blue btn-circle transition-all ml-2" class:functionally-hidden-x=move || text.get().trim().is_empty() on:click=move |_| {
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
