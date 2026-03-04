import { useEffect, useState } from "react";
import { v4 as uuidv4 } from 'uuid';

export type ReminderType = {
    id: string;
    text: string;
    notes: string;
    completed: boolean;
    due_date: Date | null;
    repeat_type: string;
    repeat_frequency_type: string;
    repeat_frequency_amount: number;
};

export function defaultReminder() {
    const id = uuidv4();
    return {
        id,
        text: "",
        notes: "",
        completed: false,
        due_date: null,
        repeat_type: "none",
        repeat_frequency_type: "weeks",
        repeat_frequency_amount: 1,
    };
}

export function useReminders(): [
    ReminderType[],
    React.Dispatch<React.SetStateAction<ReminderType[]>>
] {
    // Reminder list signal
    const [reminders, setReminders] = useState<ReminderType[]>(() => {
        // Load from localStorage on first render
        const saved = localStorage.getItem('reminders');
        return saved ? JSON.parse(saved).map((r: any) => {
            if (r.due_date) {
                r.due_date = new Date(r.due_date)
            }
            return r;
        }) : [];
    });
    // Save to localStorage whenever reminders change
    useEffect(() => {
        localStorage.setItem('reminders', JSON.stringify(reminders));
    }, [reminders]);
    return [reminders, setReminders];
}

export function useShowCompleted(): [
    boolean,
    React.Dispatch<React.SetStateAction<boolean>>
] {
    // Show completed setting
    const [showCompleted, setShowCompleted] = useState<boolean>(() => {
        // Load from localStorage on first render
        const saved = localStorage.getItem('showCompleted');
        return saved ? JSON.parse(saved) : true; // default: show completed
    });

    // Save whenever it changes
    useEffect(() => {
        localStorage.setItem('showCompleted', JSON.stringify(showCompleted));
    }, [showCompleted]);

    return [showCompleted, setShowCompleted];
}