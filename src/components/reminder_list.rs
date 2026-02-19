use leptos::prelude::*;
use uuid::Uuid;

use crate::{
    UserData,
    components::reminder_item::ReminderWidget,
    types::{Reminder, ReminderList},
};

#[component]
pub fn ReminderListWidget(
    reminder_list: impl Fn() -> ReminderList + Send + Sync + 'static,
) -> impl IntoView {
    view! {
        <div class="toplevel-container scroll-container">
            <ul class="container-reminder-list">
                <For each=move || reminder_list().0 key=|reminder| reminder.id let:reminder>
                    <ReminderWidget reminder=reminder/>
                </For>
            </ul>
        </div>
        <NewReminderBox/>
    }
}

#[component]
fn NewReminderBox() -> impl IntoView {
    let set_user_data = use_context::<WriteSignal<UserData>>().expect("Could not find user data");

    let new_reminder = move |_| {
        let new_reminder = Reminder::new(Uuid::new_v4(), String::new(), false);
        set_user_data.update(|state| {
            state.reminders_list.add_reminder(new_reminder);
        });
    };

    view! {
        <div class="container-reminder-input">
            <div class="container-reminder-input-inner">
                <button type="button" class="btn btn-blue font-bold btn-circle text-3xl transition-all ml-2" on:click=new_reminder>+</button>
            </div>
        </div>
    }
}
