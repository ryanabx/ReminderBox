use chrono::{DateTime, Duration, Local, NaiveDateTime, TimeZone, Utc};
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct UserData {
    pub reminders: RwSignal<Vec<Reminder>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Reminder {
    pub id: Uuid,
    pub title: String,
    pub notes: String,
    pub completed: bool,
    pub due_date: DueDate,
}

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub enum DueDate {
    #[default]
    None,
    Once {
        due: DateTime<Utc>,
    },
    Interval {
        orig_due: DateTime<Utc>,
        interval: Duration,
    },
    RecurAfterCompletion {
        orig_due: DateTime<Utc>,
        last_completion: DateTime<Utc>,
        interval: Duration,
    },
}

impl DueDate {
    pub fn new_from_string(s: &str) -> Self {
        match s {
            "none" => Self::None,
            "once" => Self::Once { due: Utc::now() },
            "interval" => Self::Interval {
                orig_due: Utc::now(),
                interval: Duration::weeks(1),
            },
            "recuraftercompletion" => Self::RecurAfterCompletion {
                orig_due: Utc::now(),
                last_completion: Utc::now(),
                interval: Duration::weeks(1),
            },
            _ => Self::None, // Default to None
        }
    }

    pub fn to_category(&self) -> String {
        match self {
            DueDate::None => "none",
            DueDate::Once { .. } => "once",
            DueDate::Interval { .. } => "interval",
            DueDate::RecurAfterCompletion { .. } => "recuraftercompletion",
        }
        .to_string()
    }
}

pub fn datetime_local_to_utc(input: &str) -> Option<DateTime<Utc>> {
    // 1️ Parse local string (no timezone)
    let naive = NaiveDateTime::parse_from_str(input, "%Y-%m-%dT%H:%M").ok()?;

    // 2️ Attach user's local timezone safely

    let local_dt = Local.from_local_datetime(&naive).single()?;

    // 3️ Convert to UTC
    Some(local_dt.with_timezone(&Utc))
}

pub fn utc_to_input_string(dt: DateTime<Utc>) -> String {
    dt.with_timezone(&Local)
        .format("%Y-%m-%dT%H:%M")
        .to_string()
}

pub fn utc_to_local(dt: DateTime<Utc>) -> String {
    dt.with_timezone(&Local)
        .format("%-m/%-d/%y, %l:%M %p")
        .to_string()
}

pub fn time_past_now(dt: DateTime<Utc>) -> bool {
    Utc::now() > dt
}

impl Reminder {
    pub fn new(id: Uuid, title: String, completed: bool) -> Self {
        Reminder {
            id,
            title,
            notes: String::new(),
            completed,
            due_date: DueDate::None,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.title.is_empty() && self.notes.is_empty() && matches!(self.due_date, DueDate::None)
    }
}

pub fn update_reminder(
    reminders: RwSignal<Vec<Reminder>>,
    id: Uuid,
    f: impl FnOnce(&mut Reminder),
) {
    reminders.update(|list| {
        if let Some(r) = list.iter_mut().find(|r| r.id == id) {
            f(r);
        }
    });
}

pub fn get_reminder(reminders: RwSignal<Vec<Reminder>>, id: Uuid) -> Option<Reminder> {
    reminders.with(|list| list.iter().find(|r| r.id == id).cloned())
}
