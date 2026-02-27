use leptos::{logging, prelude::*};
use uuid::Uuid;

use crate::{
    app::Page,
    user_data::{
        DueDate, Reminder, datetime_local_to_utc, get_reminder, update_reminder,
        utc_to_input_string,
    },
};

#[component]
pub fn ReminderSettings(
    reminder_id: Uuid,
    reminder_list: RwSignal<Vec<Reminder>>,
) -> impl IntoView {
    let set_page = use_context::<WriteSignal<Page>>().unwrap();

    view! {
        <div class="container-reminder-list-header">
            <div class="container-reminder-list-header-inner">
                <button class="btn opacity-0" disabled=true>"Done"</button> // Hidden button to make header centered
                <h1 class="font-bold text-xl">"Details"</h1>
                <button class="btn" on:click=move |_| {set_page.set(Page::Main)}>"Done"</button>
            </div>
        </div>
        <div class="toplevel-container space-y-4 accomodate-header">
            <div class="max-w-xl w-full flex flex-col space-y-2">
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
                <div class="flex flex-col space-y-1">
                    <h1>"Date & Time"</h1>
                    <div class="flex flex-col space-y-2 container-alt">
                        <select name="repeat-style" id="repeat-style"
                            prop:value=move || {
                                get_reminder(reminder_list, reminder_id).map(|r| r.due_date.to_category()).unwrap_or(String::from("none"))
                            }
                            on:input=move |evt| {
                                let value = event_target_value(&evt);
                                logging::log!("Setting value to {}", &value);
                                update_reminder(reminder_list, reminder_id, |r| r.due_date = DueDate::new_from_string(&value));
                            }
                        >
                            <option value="none">"No due date"</option>
                            <option value="once">"One-time due date"</option>
                            <option value="interval">"Recurring Reminder"</option>
                            <option value="recuraftercompletion">"Recurring Reminder (After Completion)"</option>
                        </select>
                        <div class="flex flex-row space-x-2">
                            <ReminderDateTimeSection reminder_id reminder_list/>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn ReminderDateTimeSection(
    reminder_id: Uuid,
    reminder_list: RwSignal<Vec<Reminder>>,
) -> impl IntoView {
    view! {
        <div>
        {
            move || {
                let due_date = get_reminder(reminder_list, reminder_id).map(|r| r.due_date);
                logging::log!("Due date changed!");
                let view = due_date.map(|due_date| {
                    match due_date {
                        DueDate::None => view! {
                            <></>
                        }.into_any(),
                        DueDate::Once { due } => view! {
                                <>
                                <input type="datetime-local" class="grow" prop:value=move || {
                                    let input_string = utc_to_input_string(due);
                                    logging::log!("{}", input_string);
                                    input_string
                                } on:input=move |ev| {
                                    let value = event_target_value(&ev);
                                    update_reminder(reminder_list, reminder_id, |r| r.due_date = DueDate::Once { due: datetime_local_to_utc(&value).unwrap_or_default() });
                                } />
                                </>
                            }.into_any(),
                        DueDate::Interval { orig_due, interval } => view! {
                            <></>
                        }.into_any(),
                        DueDate::RecurAfterCompletion { orig_due, last_completion, interval } => view! {
                            <></>
                        }.into_any(),
                    }
                }).unwrap_or(view! {
                    <></>
                }.into_any());
                view.into_view()
            }
        }
        </div>
    }
}
