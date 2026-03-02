import { Button, Dialog, DialogActions, DialogContent, DialogTitle, FormGroup, TextField } from '@mui/material';
import * as React from 'react';
import type { ReminderType } from '../pages/ReminderPage';

type ReminderSettingsDialogProps = {
    open: string | null;
    setOpen: React.Dispatch<React.SetStateAction<string | null>>;
    reminders: ReminderType[];
    setReminders: React.Dispatch<React.SetStateAction<ReminderType[]>>;
};

export default function ReminderSettingsDialog({ open, setOpen, reminders, setReminders }: ReminderSettingsDialogProps) {

    // const handleOpen = () => {
    //     setOpen(true);
    // };

    const handleClose = () => {
        setOpen(null);
    };

    const handleSave = () => {
        if (draft) {
            updateReminder(draft.id, draft);
        }
        setOpen(null);
    }

    // Update a reminder
    const updateReminder = (id: string, fields: Partial<ReminderType>) => {
        setReminders((prev) =>
            prev.map((r) => (r.id === id ? { ...r, ...fields } : r))
        );
    };


    const [draft, setDraft] = React.useState<ReminderType | null>(null);
    // Update draft
    const updateDraft = (fields: Partial<ReminderType>) => {
        setDraft((prev) => {
            return prev ? { ...prev, ...fields } : prev;
        })
    }

    React.useEffect(() => {
        if (open) {
            const reminder = reminders.find(r => r.id === open);
            if (reminder) {
                setDraft({ ...reminder }); // shallow clone
            }
        } else {
            setDraft(null);
        }
    }, [open, reminders]);

    return (
        <React.Fragment>
            <Dialog open={draft != null} onClose={handleClose} fullWidth>
                <DialogTitle>Reminder Settings</DialogTitle>
                <DialogContent>
                    {
                        draft && (
                            <FormGroup>
                                <TextField
                                    variant="filled" label="Reminder Name" multiline
                                    value={draft.text}
                                    onChange={(e) => updateDraft({ text: e.target.value })}
                                />
                                <TextField variant="filled" label="Reminder Notes" multiline
                                    value={draft.notes}
                                    onChange={(e) => updateDraft({ notes: e.target.value })}
                                />
                            </FormGroup>
                        )
                    }
                </DialogContent>
                <DialogActions>
                    <Button onClick={handleClose}>Cancel</Button>
                    <Button onClick={handleSave}>
                        Save
                    </Button>
                </DialogActions>
            </Dialog>
        </React.Fragment>
    )
}