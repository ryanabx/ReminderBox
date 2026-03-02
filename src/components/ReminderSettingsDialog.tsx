import { Button, Dialog, DialogActions, DialogContent, DialogTitle, FormGroup, InputLabel, MenuItem, Select, Stack, TextField, Typography } from '@mui/material';
import * as React from 'react';
import type { ReminderType } from '../pages/ReminderPage';
import { DateTimePicker } from '@mui/x-date-pickers';
import dayjs from 'dayjs';
import utc from "dayjs/plugin/utc";

dayjs.extend(utc);

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

    const dateSettingsUi = () => {
        if (draft) {
            switch (draft.repeat_type) {
                case "one_time":
                    return (
                        <React.Fragment>
                            <DateTimePicker
                                value={dayjs(draft.due_date)}
                                onChange={(newValue) => updateDraft({ due_date: newValue?.utc().toDate() })}
                            />
                        </React.Fragment>
                    )
                case "repeat":
                    return (
                        <React.Fragment>
                            <DateTimePicker
                                value={dayjs(draft.due_date)}
                                onChange={(newValue) => updateDraft({ due_date: newValue?.utc().toDate() })}
                            />
                            <Stack direction="row" spacing={1}>
                                <Typography variant='body1' component="div" justifySelf="center" justifyContent="center" align="center" alignContent="center">Every:</Typography>
                                <Select
                                    fullWidth
                                    id="repeat-short"
                                    // value={draft.repeat_type}
                                    label="Due Date"
                                // onChange={(e) => updateDraft({ repeat_type: String(e.target.value) })}
                                >
                                    <MenuItem value={1}>1</MenuItem>
                                    <MenuItem value={2}>2</MenuItem>
                                    <MenuItem value={3}>3</MenuItem>
                                    <MenuItem value={4}>4</MenuItem>
                                    <MenuItem value={5}>5</MenuItem>
                                    <MenuItem value={6}>6</MenuItem>
                                    <MenuItem value={7}>7</MenuItem>
                                    <MenuItem value={8}>8</MenuItem>
                                    <MenuItem value={9}>9</MenuItem>
                                    <MenuItem value={10}>10</MenuItem>
                                    <MenuItem value={11}>11</MenuItem>
                                    <MenuItem value={12}>12</MenuItem>
                                </Select>
                                <Select
                                    id="repeat-long"
                                    fullWidth
                                // value={draft.repeat_type}
                                // onChange={(e) => updateDraft({ repeat_type: String(e.target.value) })}
                                >
                                    <MenuItem value="days">Days</MenuItem>
                                    <MenuItem value="weeks">Weeks</MenuItem>
                                    <MenuItem value="months">Months</MenuItem>
                                    <MenuItem value="years">Years</MenuItem>
                                </Select>
                            </Stack>
                        </React.Fragment>
                    )
                default:
                    return (
                        <></>
                    )
            }
        }
        else {
            return (
                <></>
            )
        }
    }

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
                                <InputLabel id="due-date-label">Due Date</InputLabel>
                                <Select
                                    labelId="due-date-label"
                                    id="due-date-option"
                                    value={draft.repeat_type}
                                    label="Due Date"
                                    onChange={(e) => updateDraft({ repeat_type: String(e.target.value) })}
                                >
                                    <MenuItem value="none">None</MenuItem>
                                    <MenuItem value="one_time">One-Time</MenuItem>
                                    <MenuItem value="repeat">Repeat</MenuItem>
                                    <MenuItem value="repeat_after_completion">Repeat After Completion</MenuItem>
                                </Select>
                                {
                                    dateSettingsUi()
                                }
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