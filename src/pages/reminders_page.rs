use leptos::{html::Textarea, prelude::*};
use uuid::Uuid;
use web_sys::KeyboardEvent;

use crate::{
    app::Page,
    user_data::{Reminder, get_reminder, time_past_now, update_reminder, utc_to_local},
};

#[component]
pub fn RemindersPage(reminder_list: RwSignal<Vec<Reminder>>) -> impl IntoView {
    let focus = RwSignal::<Option<Uuid>>::new(None);
    let new_reminder = move |_| {
        new_blank_reminder(reminder_list, None, focus);
    };
    let reminder_to_widget = move |reminder: Reminder| {
        view! {
            <ReminderContainer reminder_id=reminder.id reminder_list=reminder_list next_focus=focus/>
        }
    };
    view! {
        <div class="container-reminder-list-header">
            <div class="container-reminder-list-header-inner">
                <h1 class="font-bold text-xl">"Reminders"</h1>
            </div>
        </div>
        <div class="toplevel-container scroll-container">
            <ul class="container-reminder-list">
                <For each=move || reminder_list.get() key=|reminder| reminder.id children=reminder_to_widget>
                </For>
            </ul>
        </div>
        <div class="container-reminder-input">
            <div class="container-reminder-input-inner">
                <button type="button" class="btn btn-blue font-bold btn-circle text-3xl transition-all ml-2 flex justify-center items-center" on:click=new_reminder>
                    <svg width="24" height="24" viewBox="0 0 24 24" fill="none"
                        stroke="currentColor" stroke-width="2"
                        stroke-linecap="round" stroke-linejoin="round">
                        <line x1="12" y1="5" x2="12" y2="19"></line>
                        <line x1="5" y1="12" x2="19" y2="12"></line>
                    </svg>
                </button>
            </div>
        </div>
    }
}

#[component]
pub fn ReminderContainer(
    reminder_id: Uuid,
    reminder_list: RwSignal<Vec<Reminder>>,
    next_focus: RwSignal<Option<Uuid>>,
) -> impl IntoView {
    // let id = reminder.id.clone();
    let set_page = use_context::<WriteSignal<Page>>().unwrap();

    let input_ref = NodeRef::<Textarea>::new();

    // Handle focus requests
    Effect::new(move || {
        let next_focus_value = next_focus.get();
        if next_focus_value.is_some_and(|id| id == reminder_id) {
            if let Some(textarea) = input_ref.get() {
                textarea.focus().ok();
            }
            next_focus.set(None); // Reset focus request
        }
    });

    let (focused, set_focused) = signal(false);
    let mounted = StoredValue::new(false);

    // If focus is gone and the reminder is empty, delete the reminder
    Effect::new(move || {
        let focused = focused.get();
        if !mounted.get_value() {
            // Skip the first run (on mount)
            mounted.set_value(true);
            return;
        } else if !focused {
            if get_reminder(reminder_list, reminder_id)
                .map(|r| r.is_empty())
                .unwrap_or_default()
            {
                remove_reminder(reminder_list, reminder_id, next_focus);
            }
        }
    });

    view! {
        <div draggable=true class="flex flex-row space-x-2 constrain-x"
            on:focusin=move |_| set_focused.set(true)
            on:focusout=move |_| set_focused.set(false)>
            <label class="flex items-center cursor-pointer space-x-3">
                <input type="checkbox"
                    class="hidden peer"
                    prop:checked=move || {
                        get_reminder(reminder_list, reminder_id).map(|r| r.completed).unwrap_or_default()
                    }
                    on:input=move |ev| {
                        let checked = event_target_checked(&ev);
                        update_reminder(reminder_list, reminder_id, |r| r.completed = checked);
                    }
                />
                <div class="w-6 h-6 rounded-full border-2 border-neutral-400 peer-checked:bg-blue-500 peer-checked:border-blue-500 transition-colors duration-150 flex items-center justify-center">
                    <svg class="w-3 h-3 {} text-white transition-opacity"
                        class:opacity-0=move || {
                            get_reminder(reminder_list, reminder_id).map(|r| !r.completed).unwrap_or_default()
                        }
                        fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M5 13l4 4L19 7" />
                    </svg>
                </div>
            </label>
            <div class="flex flex-col grow py-2 px-2">
                <textarea node_ref=input_ref class="multiline-input reminder-input"
                prop:value=move || {
                    get_reminder(reminder_list, reminder_id).map(|r| r.title).unwrap_or_default()
                }
                on:input = move |ev| {
                    let value = event_target_value(&ev);
                    update_reminder(reminder_list, reminder_id, |r| r.title = value);
                }
                on:keydown=move |ev: KeyboardEvent| {
                    if ev.key() == "Enter" && !ev.get_modifier_state("Shift") {
                        ev.prevent_default();
                        new_blank_reminder(reminder_list, Some(reminder_id), next_focus);
                    }
                    if ev.key() == "Backspace"
                        && !ev.get_modifier_state("Shift")
                        && !ev.get_modifier_state("Control")
                    {
                        if get_reminder(reminder_list, reminder_id).map(|r| r.is_empty()).unwrap_or_default()
                        {
                            ev.prevent_default();
                            remove_reminder(reminder_list, reminder_id, next_focus);
                        }
                    }
                }
                />
                {move || {
                    let due_date = get_reminder(reminder_list, reminder_id).map(|r| r.due_date).unwrap_or_default();
                    match due_date {
                        crate::user_data::DueDate::None => None,
                        crate::user_data::DueDate::Once { due } => {
                            Some(
                                view! {
                                    <p class="grow wrap-anywhere text-neutral-500" class:text-red-500=move || {time_past_now(due)}>{utc_to_local(due)}</p>
                                }
                            )
                        },
                        crate::user_data::DueDate::Interval { orig_due, interval } => todo!(),
                        crate::user_data::DueDate::RecurAfterCompletion { orig_due, last_completion, interval } => todo!(),
                    }
                }}
            </div>
            <button class="info-button"
            class:opacity-0=move || {!focused.get()}
            on:click=move |_| {set_page.set(Page::Settings(reminder_id))}
            >
                "i"
            </button>
        </div>
    }
}

/// Remove the reminder with the given `id`, and focus the
///   previous element.
fn remove_reminder(
    reminders: RwSignal<Vec<Reminder>>,
    id: Uuid,
    next_focus: RwSignal<Option<Uuid>>,
) {
    let idx = reminders.with(|reminders| reminders.iter().position(|r| r.id == id).unwrap());
    let next_uuid = reminders.with(|reminders| reminders.get(idx.saturating_sub(1)).map(|r| r.id));
    reminders.update(|reminders| reminders.retain(|r| r.id != id));
    next_focus.set(next_uuid);
}

/// Insert a new blank reminder after the reminder with `id_after`,
///   then focus the new reminder.
fn new_blank_reminder(
    reminders: RwSignal<Vec<Reminder>>,
    id_after: Option<Uuid>,
    next_focus: RwSignal<Option<Uuid>>,
) {
    let reminders_len = reminders.get().len();
    let idx = id_after
        .and_then(|id_after| reminders.get().iter().position(|r| r.id == id_after))
        .map(|idx| idx.saturating_add(1))
        .unwrap_or(reminders_len);
    let new_uuid = Uuid::new_v4();
    reminders.update(|reminders| {
        // NOTE: The Reminder struct used to have RwSignals for its values, which caused problems with running
        //   `new_blank_reminder` within a Reminder component. This would make the origin Reminder its reactive
        //   owner and deleting the origin reminder caused a panic.
        let new_reminder = Reminder::new(new_uuid.clone(), String::new(), false);
        reminders.insert(idx, new_reminder);
    });
    next_focus.set(Some(new_uuid));
}
