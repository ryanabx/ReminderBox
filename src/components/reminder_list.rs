use leptos::{html::Input, prelude::*};
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
        <div class="flex flex-col w-full max-w-xl overflow-y-auto">
            <p>"My Reminders:"</p>
            <NewReminderBox/>
            <ul class="space-y-2">
                <For each=reminders key=|reminder| reminder.id let:reminder>
                    <ReminderWidget reminder/>
                </For>
            </ul>
        </div>
    }
}

// const ESCAPE_KEY: u32 = 27;
const ENTER_KEY: u32 = 13;

#[component]
fn NewReminderBox() -> impl IntoView {
    let state_setter = use_context::<WriteSignal<UserData>>().expect("Could not find user data");

    // Callback to add a todo on pressing the `Enter` key, if the field isn't empty
    let input_ref = NodeRef::<Input>::new();
    let add_todo = move |ev: KeyboardEvent| {
        let input = input_ref.get().unwrap();
        ev.stop_propagation();
        let key_code = ev.key_code();
        if key_code == ENTER_KEY {
            let title = input.value();
            let title = title.trim();
            if !title.is_empty() {
                let new_reminder = Reminder::new(Uuid::new_v4(), title.to_string(), false);
                state_setter.update(|state| {
                    state.reminders_list.add_reminder(new_reminder);
                });
                input.set_value("");
            }
        }
    };

    view! {
        <div>
            <input type="text" placeholder="Enter a new reminder" on:keydown=add_todo node_ref=input_ref />
        </div>
    }
}
