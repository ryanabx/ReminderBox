use leptos::prelude::*;
use uuid::Uuid;

use crate::{
    app::Page,
    user_data::{Reminder, get_reminder, update_reminder},
};

#[component]
pub fn ReminderSettings(
    reminder_id: Uuid,
    reminder_list: RwSignal<Vec<Reminder>>,
) -> impl IntoView {
    let set_page = use_context::<WriteSignal<Page>>().unwrap();

    view! {
        <div class="toplevel-container space-y-4">
            <div class="max-w-xl w-full flex flex-col space-y-2">
                <div class="flex flex-row w-full container-alt justify-end">
                    <button class="btn mx-2" on:click=move |_| {set_page.set(Page::Main)}>"Done"</button>
                </div>
                <div class="flex flex-col space-y-2 container-alt">
                    <textarea id="reminder-name" class="grow standard-input multiline-input text-xl" placeholder="Reminder"
                    prop:value=move || {
                        get_reminder(reminder_list, reminder_id).map(|r| r.title).unwrap_or_default()
                    } on:input=move |ev| {
                        let value = event_target_value(&ev);
                        update_reminder(reminder_list, reminder_id, |r| r.title = value);
                    } />
                    <textarea id="reminder-notes" class="grow standard-input multiline-input text-sm" placeholder="Notes"
                    prop:value=move || {
                        get_reminder(reminder_list, reminder_id).map(|r| r.notes).unwrap_or_default()
                    } on:input=move |ev| {
                        let value = event_target_value(&ev);
                        update_reminder(reminder_list, reminder_id, |r| r.notes = value);
                    } />
                </div>
                <div>
                    <h1>"Date & Time"</h1>
                    <div class="flex flex-col space-y-2 container-alt">
                        <div class="flex flex-row space-x-2">
                            <input type="date" class="grow" prop:value=move || {
                                get_reminder(reminder_list, reminder_id).map(|r| r.due_date).unwrap_or_default()
                            } on:input=move |ev| {
                                let value = event_target_value(&ev);
                                update_reminder(reminder_list, reminder_id, |r| r.due_date = value);
                            } />
                            <button class="remove-button" on:click=move |_| {
                                update_reminder(reminder_list, reminder_id, |r| r.due_date = String::new());
                            }>X</button>
                        </div>
                        <div class="flex flex-row space-x-2">
                            <input type="time" class="grow" prop:value=move || {
                                get_reminder(reminder_list, reminder_id).map(|r| r.due_time).unwrap_or_default()
                            } on:input=move |ev| {
                                let value = event_target_value(&ev);
                                update_reminder(reminder_list, reminder_id, |r| r.due_time = value);
                            } />
                            <button class="remove-button" on:click=move |_| {
                                update_reminder(reminder_list, reminder_id, |r| r.due_time = String::new());
                            }>X</button>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
