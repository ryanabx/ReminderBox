// use leptos::prelude::*;

// use crate::{Page, UserData, user_data::Reminder};

// #[component]
// pub fn ReminderSettings(reminder: Reminder) -> impl IntoView {
//     let set_page = use_context::<WriteSignal<Page>>().unwrap();
//     let set_user_data = use_context::<WriteSignal<UserData>>().unwrap();

//     let on_clear_reminder_date = move |_| {
//         set_user_data.update(|user_data| {
//             if let Some(reminder) = user_data.reminders_list.reminder_mut(reminder.id) {
//                 reminder.due_date.set(String::new());
//             }
//         });
//     };

//     let on_clear_reminder_time = move |_| {
//         set_user_data.update(|user_data| {
//             if let Some(reminder) = user_data.reminders_list.reminder_mut(reminder.id) {
//                 reminder.due_time.set(String::new());
//             }
//         });
//     };

//     view! {
//         <div class="toplevel-container space-y-4">
//             <div class="max-w-xl w-full flex flex-col space-y-2">
//                 <div class="flex flex-row w-full container-alt justify-end">
//                     <button class="btn mx-2" on:click=move |_| {set_page.set(Page::Main)}>"Done"</button>
//                 </div>
//                 <div class="flex flex-col space-y-2 container-alt">
//                     <textarea id="reminder-name" class="grow standard-input multiline-input text-xl" placeholder="Reminder" bind:value={reminder.title} />
//                     <textarea id="reminder-notes" class="grow standard-input multiline-input text-sm" placeholder="Notes" bind:value={reminder.notes} />
//                 </div>
//                 <div>
//                     <h1>"Date & Time"</h1>
//                     <div class="flex flex-col space-y-2 container-alt">
//                         <div class="flex flex-row space-x-2">
//                             <input type="date" class="grow" bind:value={reminder.due_date} />
//                             <button class="remove-button" on:click=on_clear_reminder_date>X</button>
//                         </div>
//                         <div class="flex flex-row space-x-2">
//                             <input type="time" class="grow" bind:value={reminder.due_time} />
//                             <button class="remove-button" on:click=on_clear_reminder_time>X</button>
//                         </div>
//                     </div>
//                 </div>
//             </div>
//         </div>
//     }
// }
