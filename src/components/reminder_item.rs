use leptos::{ev, html::Button, prelude::*};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use web_sys::{Event, Node, wasm_bindgen::JsCast};

use crate::{Page, data::UserData};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Reminder {
    pub id: Uuid,
    pub title: String,
    pub notes: String,
    pub completed: bool,
    pub due_date: String,
    pub due_time: String,
}

impl Reminder {
    pub fn new(id: Uuid, title: String, completed: bool) -> Self {
        Reminder {
            id,
            title,
            notes: String::new(),
            completed,
            due_date: String::new(),
            due_time: String::new(),
        }
    }
}

#[component]
pub fn ReminderWidget(reminder: Reminder) -> impl IntoView {
    // let id = reminder.id.clone();
    let set_user_data = use_context::<WriteSignal<UserData>>().unwrap();

    let on_reminder_change = move |ev| {
        set_user_data.update(|user_data| {
            let checked = event_target_checked(&ev);
            if let Some(reminder) = user_data.reminders_list.reminder_mut(reminder.id) {
                reminder.completed = checked;
            }
        })
    };

    let on_reminder_name_change = move |ev| {
        set_user_data.update(|user_data| {
            let title = event_target_value(&ev);
            if let Some(reminder) = user_data.reminders_list.reminder_mut(reminder.id) {
                reminder.title = title;
            }
        })
    };

    let due_date_fn = move || {
        let due_date = reminder.due_date.clone();
        let due_time = reminder.due_time.clone();
        (!due_date.clone().is_empty() || !due_time.is_empty()).then(|| {
            view! {
                <p class="grow wrap-anywhere text-neutral-500">{due_date}{if due_time.is_empty() {String::new()} else {format!(", {}", due_time)}}</p>
            }
        })
    };

    view! {
        <div draggable=true class="flex flex-row space-x-2 constrain-x">
            <ReminderCheckbox completed={reminder.completed} on_change=on_reminder_change />
            <div class="flex flex-col grow py-2 px-2">
                <input type="text" class="grow wrap-anywhere reminder-input" value={reminder.title} on:change=on_reminder_name_change />
                {due_date_fn}
            </div>
            // <InfoButton reminder_id=id/>
            // <RemoveButton reminder_id=id/>
        </div>
    }
}

#[component]
pub fn ReminderCheckbox(completed: bool, on_change: impl FnMut(Event) + 'static) -> impl IntoView {
    view! {
        <label class="flex items-center cursor-pointer space-x-3">
        <input type="checkbox" class="hidden peer" checked=completed on:change=on_change />
        <div class="w-6 h-6 rounded-full border-2 border-neutral-400 peer-checked:bg-blue-500 peer-checked:border-blue-500 transition-colors duration-150 flex items-center justify-center">
            <svg class="w-3 h-3 {} text-white transition-opacity" class:opacity-0=move || {!completed} fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" d="M5 13l4 4L19 7" />
            </svg>
        </div>
        </label>
    }
}

#[component]
pub fn InfoButton(reminder_id: Uuid) -> impl IntoView {
    let set_page = use_context::<WriteSignal<Page>>().unwrap();
    view! {
        <button class="btn btn-circle" on:click=move |_| {set_page.set(Page::Settings(reminder_id))}>"i"</button>
    }
}

#[component]
pub fn RemoveButton(reminder_id: Uuid) -> impl IntoView {
    let state_setter = use_context::<WriteSignal<UserData>>().expect("Could not find user data");
    // Signal to toggle the confirmation dialog
    let show_dialog = RwSignal::new(false);

    let btn_ref = NodeRef::<Button>::new();
    let btn_ref_2 = NodeRef::<Button>::new();

    let handle = window_event_listener(ev::pointerdown, move |ev| {
        if show_dialog.get()
            && ev.target().is_none_or(|x| {
                x.dyn_into::<Node>().is_ok_and(|x| {
                    btn_ref.get().is_some_and(|button| {
                        !button.contains(Some(&x))
                            && btn_ref_2
                                .get()
                                .is_none_or(|button2| !button2.contains(Some(&x)))
                    })
                })
            })
        {
            show_dialog.set(false);
        }
    });
    on_cleanup(move || handle.remove());

    view! {
        <>
            <Show
                when=move || {show_dialog.get()}
                fallback=move || view!{
                    <button
                        type="button"
                        class="remove-button w-6"
                        on:click=move |_| {show_dialog.set(true);}
                        node_ref=btn_ref_2
                    >"X"</button>
                }>
            // The delete button
            <button
                type="button"
                class="btn btn-red"
                on:click=move |_| {
                    state_setter.update(|state| {
                        state.reminders_list.remove_reminder(reminder_id);
                    });
                }
                node_ref=btn_ref
            >"Delete"</button>
            // <div class="fixed inset-0 flex items-center full-screen justify-center bg-black/50 z-50">
            //     <div class="absolute inset-0" on:click=move |evt| {show_dialog.set(false); evt.stop_propagation();}></div>
            //     <div class="bg-white dark:bg-neutral-800 p-6 rounded space-y-4 relative z-10">
            //         <p>"Are you sure you want to delete this reminder?"</p>
            //         <div class="flex justify-end space-x-2">
            //             <button
            //                 class="btn"
            //                 on:click=move |_| show_dialog.set(false)
            //             >
            //                 "Cancel"
            //             </button>
            //             <button
            //                 class="btn btn-red"
            //                 on:click=move |_| {
            //                     state_setter.update(|state| {
            //                         state.reminders_list.remove_reminder(reminder_id);
            //                     });
            //                 }
            //                 >
            //                 "Delete"
            //             </button>
            //         </div>
            //     </div>
            // </div>
            </Show>
        </>
    }
}
