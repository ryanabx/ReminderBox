use leptos::prelude::*;
use uuid::Uuid;

use crate::{components::{reminder_item::Reminder, reminder_list::ReminderListWidget, reminder_settings::ReminderSettings}, data::UserData};

pub mod components;
pub mod data;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App)
}

#[component]
fn App() -> impl IntoView {
    // The `user data` is a signal, since we need to reactively update the list
    let (user_data, set_user_data) = signal(UserData::new());

    // We provide a context that each <Todo/> component can use to update the list
    // Here, I'm just passing the `WriteSignal`; a <Todo/> doesn't need to read the whole list
    // (and shouldn't try to, as that would cause each individual <Todo/> to re-render when
    // a new todo is added! This kind of hygiene is why `signal` defaults to read-write
    // segregation.)
    provide_context(set_user_data);

    // Serialization
    //
    // the effect reads the `todos` signal, and each `Todo`'s title and completed
    // status,  so it will automatically re-run on any change to the list of tasks
    //
    // this is the main point of effects: to synchronize reactive state
    // with something outside the reactive system (like localStorage)

    Effect::new(move |_| {
        data::local_storage_effect(&user_data);
    });

    let reminders = move || user_data.with(|d| d.reminders_list.clone());

    let reminder = Reminder::new(Uuid::new_v4(), "Reminder Example".to_string(), false);

    view! {
        <Header />
        <ReminderListWidget reminder_list=reminders/>
        // <ReminderSettings reminder=reminder />
    }
}

#[component]
fn Header() -> impl IntoView {
    view! {
        <div class="flex flex-col w-full headerbar">
            <h1 class="font-bold text-center text-xl">"ReminderBox"</h1>
            <h2 class="italic text-center text-sm">"Your reminders in a box :)"</h2>
        </div>
    }
}
