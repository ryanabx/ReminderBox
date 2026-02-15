use leptos::prelude::*;
use uuid::Uuid;

use crate::{
    components::{reminder_list::ReminderListWidget, reminder_settings::ReminderSettings},
    data::UserData,
};

pub mod components;
pub mod data;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App)
}

#[derive(Clone, Debug, Default)]
pub enum Page {
    #[default]
    Main,
    Settings(Uuid),
}

#[component]
fn App() -> impl IntoView {
    // The `user data` is a signal, since we need to reactively update the list
    let (user_data, set_user_data) = signal(UserData::new());
    // The current page we are on
    let (page, set_page) = signal(Page::default());

    provide_context(set_page);

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

    view! {
        // <Header />
        {
            move || match page.get() {
                Page::Main => view! { <ReminderListWidget reminder_list=move || user_data.with(|d| d.reminders_list.clone())/> }.into_any(),
                Page::Settings(reminder) => view! { <ReminderSettings reminder=user_data.with(|d| d.reminders_list.reminder(reminder).unwrap().clone()) /> }.into_any(),
            }
        }
    }
}

#[component]
fn Header() -> impl IntoView {
    let commit_hash = env!("GIT_COMMIT_HASH");
    view! {
        <div class="flex flex-col w-full headerbar">
            <div class="flex flex-row w-full grow justify-center space-x-1 items-center">
                <h1 class="main-header">"ReminderBox"</h1>
                <h2 class="main-subtitle">"- Your reminders in a box :)"</h2>
            </div>
            <h6 class="commit-info">{format!("Commit {}", commit_hash)}</h6>
        </div>
    }
}
