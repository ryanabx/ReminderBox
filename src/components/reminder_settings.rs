use leptos::prelude::*;

use crate::{Page, UserData, types::Reminder};

#[component]
pub fn ReminderSettings(reminder: Reminder) -> impl IntoView {
    let set_page = use_context::<WriteSignal<Page>>().unwrap();
    let set_user_data = use_context::<WriteSignal<UserData>>().unwrap();

    // let on_reminder_name_change = move |ev| {
    //     let value = event_target_value(&ev);
    //     set_user_data.update(|user_data| {
    //         if let Some(reminder) = user_data.reminders_list.reminder_mut(reminder.id) {
    //             reminder.title = value;
    //         }
    //     });
    // };

    // let on_reminder_notes_change = move |ev| {
    //     let value = event_target_value(&ev);
    //     set_user_data.update(|user_data| {
    //         if let Some(reminder) = user_data.reminders_list.reminder_mut(reminder.id) {
    //             reminder.notes = value;
    //         }
    //     });
    // };

    // let on_reminder_date_change = move |ev| {
    //     let value = event_target_value(&ev);
    //     set_user_data.update(|user_data| {
    //         if let Some(reminder) = user_data.reminders_list.reminder_mut(reminder.id) {
    //             reminder.due_date = value;
    //         }
    //     });
    // };

    // let on_clear_reminder_date = move |_| {
    //     set_user_data.update(|user_data| {
    //         if let Some(reminder) = user_data.reminders_list.reminder_mut(reminder.id) {
    //             reminder.due_date = String::new();
    //         }
    //     });
    // };

    // let on_reminder_time_change = move |ev| {
    //     let value = event_target_value(&ev);
    //     set_user_data.update(|user_data| {
    //         if let Some(reminder) = user_data.reminders_list.reminder_mut(reminder.id) {
    //             reminder.due_time = value;
    //         }
    //     });
    // };

    // let on_clear_reminder_time = move |_| {
    //     set_user_data.update(|user_data| {
    //         if let Some(reminder) = user_data.reminders_list.reminder_mut(reminder.id) {
    //             reminder.due_time = String::new();
    //         }
    //     });
    // };

    view! {
        // <div class="toplevel-container space-y-4">
        //     <div class="flex flex-row w-full container-alt justify-between">
        //         <button class="btn btn-circle mx-2" on:click=move |_| {set_page.set(Page::Main)}>"X"</button>
        //         <button class="btn mx-2">"Save"</button>
        //     </div>
        //     <div class="flex flex-col space-y-2 container-alt">
        //         <input type="text" id="reminder-name" class="grow standard-input text-xl" placeholder="Reminder" value={reminder.title} on:change=on_reminder_name_change />
        //         <input type="text" id="reminder-notes" class="grow standard-input text-sm" placeholder="Notes" value={reminder.notes} on:change=on_reminder_notes_change />
        //     </div>
        //     <div>
        //         <h1>"Date & Time"</h1>
        //         <div class="flex flex-col space-y-2 container-alt">
        //             <div class="flex flex-row space-x-2">
        //                 <input type="date" class="grow" value={reminder.due_date} on:change=on_reminder_date_change />
        //                 <button class="remove-button" on:click=on_clear_reminder_date>X</button>
        //             </div>
        //             <div class="flex flex-row space-x-2">
        //                 <input type="time" class="grow" value={reminder.due_time} on:change=on_reminder_time_change />
        //                 <button class="remove-button" on:click=on_clear_reminder_time>X</button>
        //             </div>
        //         </div>
        //     </div>
        // </div>
    }
}
