use leptos::{html::Textarea, prelude::*};
use web_sys::KeyboardEvent;

use crate::{Page, UserData, types::Reminder};

#[component]
pub fn ReminderWidget(reminder: Reminder) -> impl IntoView {
    // let id = reminder.id.clone();
    let set_page = use_context::<WriteSignal<Page>>().unwrap();
    let set_user_data = use_context::<WriteSignal<UserData>>().unwrap();

    let input_ref = NodeRef::<Textarea>::new();

    let due_date_fn = move || {
        let due_date = reminder.due_date.get();
        let due_time = reminder.due_time.get();
        (!due_date.clone().is_empty() || !due_time.is_empty()).then(|| {
            view! {
                <p class="grow wrap-anywhere text-neutral-500">{due_date}{if due_time.is_empty() {String::new()} else {format!(", {}", due_time)}}</p>
            }
        })
    };

    let make_new_reminder = move || {
        if input_ref
            .get()
            .is_some_and(|text_input| !text_input.value().is_empty())
        {
            set_user_data.update(|user_data| {
                user_data.reminders_list.new_blank_after(reminder.id);
            });
        } else {
            set_user_data.update(|user_data| {
                user_data.reminders_list.remove_reminder(reminder.id);
            });
        }
    };

    let (focused, set_focused) = signal(false);
    let mounted = StoredValue::new(false);

    let reminder_clone = reminder.clone();

    Effect::new(move || {
        let focused = focused.get();
        if !mounted.get_value() {
            // Skip the first run (on mount)
            mounted.set_value(true);
            return;
        } else if !focused {
            if reminder_clone.is_empty() {
                set_user_data.update(|user_data| {
                    user_data.reminders_list.remove_reminder(reminder.id);
                });
            }
        }
    });

    Effect::new(move || {
        if let Some(textarea) = input_ref.get() {
            textarea.focus().ok();
        }
    });

    view! {
        <div draggable=true class="flex flex-row space-x-2 constrain-x"
            on:focusin=move |_| set_focused.set(true)
            on:focusout=move |_| set_focused.set(false)>
            <ReminderCheckbox completed={reminder.completed} />
            <div class="flex flex-col grow py-2 px-2">
                <textarea node_ref=input_ref class="multiline-input reminder-input"
                bind:value=reminder.title
                on:keydown=move |ev: KeyboardEvent| {
                    if ev.key() == "Enter" && !ev.get_modifier_state("Shift") {
                        ev.prevent_default();
                        make_new_reminder();
                    }
                }
                />
                {due_date_fn}
            </div>
            <button class="info-button"
            class:opacity-0=move || {!focused.get()}
            on:click=move |_| {set_page.set(Page::Settings(reminder.id))}
            >
                "i"
            </button>
        </div>
    }
}

#[component]
pub fn ReminderCheckbox(completed: RwSignal<bool>) -> impl IntoView {
    view! {
        <label class="flex items-center cursor-pointer space-x-3">
            <input type="checkbox" class="hidden peer" bind:checked=completed />
            <div class="w-6 h-6 rounded-full border-2 border-neutral-400 peer-checked:bg-blue-500 peer-checked:border-blue-500 transition-colors duration-150 flex items-center justify-center">
                <svg class="w-3 h-3 {} text-white transition-opacity" class:opacity-0=move || {!completed.get()} fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" d="M5 13l4 4L19 7" />
                </svg>
            </div>
        </label>
    }
}
