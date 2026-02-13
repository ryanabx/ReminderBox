use leptos::prelude::*;

use crate::components::reminder_item::Reminder;

#[component]
pub fn ReminderSettings(reminder: Reminder) -> impl IntoView {
    view! {
        <div class="toplevel-container space-y-4">
            <div class="flex flex-row w-full container-alt justify-between">
                <button class="btn btn-circle mx-2">X</button>
                <button class="btn mx-2">Save</button>
            </div>
            <div class="flex flex-col space-y-2 container-alt">
                <input type="text" id="reminder-name" class="grow standard-input text-xl" placeholder="Reminder" bind:value=reminder.title />
                <input type="text" id="reminder-notes" class="grow standard-input text-sm" placeholder="Notes" bind:value=reminder.notes />
            </div>
            <div>
                <h1>"Date & Time"</h1>
                <div class="flex flex-col space-y-2 container-alt">
                    <input type="date" />
                    <input type="time" />
                    <input type="datetime-local" />
                </div>
            </div>
        </div>
    }
}
