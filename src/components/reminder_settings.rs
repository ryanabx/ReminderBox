use leptos::prelude::*;

use crate::components::reminder_item::Reminder;

#[component]
pub fn ReminderSettings(reminder: Reminder) -> impl IntoView {
    view! {
        <div class="toplevel-container">
            <div class="flex flex-row space-x-2">
                <label for="reminder-name">"Reminder:"</label>
                <input type="text" id="reminder-name" class="grow standard-input" placeholder="Enter reminder..." bind:value=reminder.title />
            </div>
        </div>
    }
}
