use leptos::prelude::*;
use uuid::Uuid;
use web_sys::KeyboardEvent;

use crate::{
    UserData,
    components::reminder_item::ReminderWidget,
    types::{Reminder, ReminderList},
};

#[component]
pub fn ReminderListWidget(
    reminder_list: impl Fn() -> ReminderList + Send + Sync + 'static,
) -> impl IntoView {
    let reminders = move || reminder_list().0;

    view! {
        <div class="toplevel-container scroll-container">
            <ul class="container-reminder-list">
                <For each=reminders key=|reminder| reminder.id let:reminder>
                    <ReminderWidget reminder/>
                </For>
            </ul>
        </div>
        <NewReminderBox/>
    }
}

// const ESCAPE_KEY: u32 = 27;
const ENTER_KEY: u32 = 13;

#[component]
fn NewReminderBox() -> impl IntoView {
    let state_setter = use_context::<WriteSignal<UserData>>().expect("Could not find user data");

    let text = RwSignal::new(String::new());

    let add_todo = move |ev: KeyboardEvent| {
        ev.stop_propagation();
        let key_code = ev.key_code();
        if key_code == ENTER_KEY {
            let title = text.get();
            let title = title.trim();
            if !title.is_empty() {
                let new_reminder = Reminder::new(Uuid::new_v4(), title.to_string(), false);
                state_setter.update(|state| {
                    state.reminders_list.add_reminder(new_reminder);
                });
                text.set(String::new());
            }
        }
    };

    view! {
        <div class="container-reminder-input">
            <div class="container-reminder-input-inner">
                <input type="text" class="grow new-reminder-input rounded-full" placeholder="Enter a new reminder" bind:value=text on:keydown=add_todo />
                <button type="button" class="btn btn-blue btn-circle transition-all ml-2" class:functionally-hidden-x=move || text.get().trim().is_empty() on:click=move |_| {
                    let title = text.get();
                    let title = title.trim();
                    if !title.is_empty() {
                        let new_reminder = Reminder::new(Uuid::new_v4(), title.to_string(), false);
                        state_setter.update(|state| {
                            state.reminders_list.add_reminder(new_reminder);
                        });
                        text.set(String::new());
                    }
                }>+</button>
            </div>
        </div>
    }
}
