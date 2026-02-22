use leptos::{html::Textarea, prelude::*};
use uuid::Uuid;
use web_sys::KeyboardEvent;

use crate::{app::Page, user_data::Reminder};

#[component]
pub fn RemindersPage(reminder_list: RwSignal<Vec<Reminder>>) -> impl IntoView {
    let focus = RwSignal::<Option<Uuid>>::new(None);
    let new_reminder = move |_| {
        new_blank_reminder(reminder_list, None, focus);
    };
    let reminder_to_widget = move |reminder| {
        view! {
            <ReminderContainer reminder=reminder reminder_list=reminder_list next_focus=focus/>
        }
    };
    view! {
        <div class="toplevel-container scroll-container">
            <ul class="container-reminder-list">
                <For each=move || reminder_list.get() key=|reminder| reminder.id children=reminder_to_widget>
                </For>
            </ul>
        </div>
        <div class="container-reminder-input">
            <div class="container-reminder-input-inner">
                <button type="button" class="btn btn-blue font-bold btn-circle text-3xl transition-all ml-2" on:click=new_reminder>+</button>
            </div>
        </div>
    }
}

#[component]
pub fn ReminderContainer(
    reminder: Reminder,
    reminder_list: RwSignal<Vec<Reminder>>,
    next_focus: RwSignal<Option<Uuid>>,
) -> impl IntoView {
    // let id = reminder.id.clone();
    let set_page = use_context::<WriteSignal<Page>>().unwrap();

    let input_ref = NodeRef::<Textarea>::new();

    // Reminder ID does not change
    let reminder_id = reminder.id;

    let on_reminder_keydown = move |ev: KeyboardEvent| {
        if ev.key() == "Enter" && !ev.get_modifier_state("Shift") {
            ev.prevent_default();
            new_blank_reminder(reminder_list, Some(reminder_id), next_focus);
        }
        if ev.key() == "Backspace"
            && !ev.get_modifier_state("Shift")
            && !ev.get_modifier_state("Control")
        {
            if reminder.title.get().is_empty()
                && reminder.notes.get().is_empty()
                && reminder.due_date.get().is_empty()
                && reminder.due_time.get().is_empty()
            {
                ev.prevent_default();
                remove_reminder(reminder_list, reminder_id, next_focus);
            }
        }
    };

    let due_date_fn = move || {
        let due_date = reminder.due_date.get();
        let due_time = reminder.due_time.get();
        (!due_date.clone().is_empty() || !due_time.is_empty()).then(|| {
            view! {
                <p class="grow wrap-anywhere text-neutral-500">{due_date}{if due_time.is_empty() {String::new()} else {format!(", {}", due_time)}}</p>
            }
        })
    };

    // let make_new_reminder = move || {
    //     if reminder
    //         .input_ref
    //         .get()
    //         .is_some_and(|text_input| !text_input.value().is_empty())
    //     {
    //         new_blank_reminder(reminder_list, reminder_id, next_focus);
    //     } else {
    //         remove_reminder(reminder_list, reminder_id, next_focus);
    //     }
    // };

    let (focused, set_focused) = signal(false);
    let mounted = StoredValue::new(false);

    // If focus is gone and the reminder is empty, delete the reminder
    // Effect::new(move || {
    //     let focused = focused.get();
    //     if !mounted.get_value() {
    //         // Skip the first run (on mount)
    //         mounted.set_value(true);
    //         return;
    //     } else if !focused {
    //         if reminder.title.get().is_empty()
    //             && reminder.notes.get().is_empty()
    //             && reminder.due_date.get().is_empty()
    //             && reminder.due_time.get().is_empty()
    //         {
    //             remove_reminder(reminder_list, reminder_id, next_focus);
    //         }
    //     }
    // });

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

    view! {
        <div draggable=true class="flex flex-row space-x-2 constrain-x"
            on:focusin=move |_| set_focused.set(true)
            on:focusout=move |_| set_focused.set(false)>
            <label class="flex items-center cursor-pointer space-x-3">
                <input type="checkbox" class="hidden peer" bind:checked=reminder.completed />
                <div class="w-6 h-6 rounded-full border-2 border-neutral-400 peer-checked:bg-blue-500 peer-checked:border-blue-500 transition-colors duration-150 flex items-center justify-center">
                    <svg class="w-3 h-3 {} text-white transition-opacity" class:opacity-0=move || {!reminder.completed.get()} fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M5 13l4 4L19 7" />
                    </svg>
                </div>
            </label>
            <div class="flex flex-col grow py-2 px-2">
                <textarea node_ref=input_ref class="multiline-input reminder-input"
                bind:value=reminder.title
                on:keydown=on_reminder_keydown
                />
                {due_date_fn}
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
        .unwrap_or(reminders_len.saturating_sub(1));
    let new_uuid = Uuid::new_v4();
    let new_reminder = Reminder::new(new_uuid.clone(), String::new(), false);
    reminders.update(|reminders| reminders.insert(idx, new_reminder));
    next_focus.set(Some(new_uuid));
}
