import { Add, Delete, InfoOutline } from "@mui/icons-material";
import { Checkbox, Container, Fab, IconButton, Stack, TextField } from "@mui/material";
import { v4 as uuidv4 } from 'uuid';
import * as React from "react";

type ReminderType = {
    id: string;
    text: string;
    completed: boolean;
};

export default function ReminderPage() {
    const [reminders, setReminders] = React.useState<ReminderType[]>(() => {
        // Load from localStorage on first render
        const saved = localStorage.getItem('reminders');
        return saved ? JSON.parse(saved) : [];
    });

    const [focusedId, setFocusedId] = React.useState<string | null>(null);

    // Save to localStorage whenever reminders change
    React.useEffect(() => {
        localStorage.setItem('reminders', JSON.stringify(reminders));
    }, [reminders]);

    const updateReminder = (id: string, fields: Partial<ReminderType>) => {
        setReminders((prev) =>
            prev.map((r) => (r.id === id ? { ...r, ...fields } : r))
        );
    };

    const addReminder = () => {
        setReminders((prev) => [
            ...prev,
            { id: uuidv4(), text: '', completed: false },
        ]);
    };

    const deleteReminder = (id: string) => {
        setReminders((prev) => prev.filter((r) => r.id !== id));
    };

    return (
        <React.Fragment>
            <Container maxWidth="sm">
                <Stack spacing={2} sx={{
                    pb: 12,
                    pt: 2
                }}>
                    {reminders.map((r) => (
                        <Stack key={r.id} direction="row" spacing={1} alignItems="center">
                            <Checkbox
                                checked={r.completed}
                                onChange={(e) =>
                                    updateReminder(r.id, { completed: e.target.checked })
                                }
                            />
                            <TextField
                                fullWidth
                                variant="outlined"
                                size="small"
                                value={r.text}
                                onFocus={() => setFocusedId(r.id)}
                                onBlur={() => {
                                    setFocusedId(null);
                                }}
                                onChange={(e) => updateReminder(r.id, { text: e.target.value })}
                                sx={{ textDecoration: r.completed ? 'line-through' : 'none', transition: 'transform 0.2s' }}
                            />
                            <Stack
                                direction="row"
                                spacing={1}
                                sx={{
                                    opacity: focusedId === r.id ? 1 : 0,
                                    position: focusedId === r.id ? "relative" : "absolute",
                                    right: focusedId === r.id ? null : 0,
                                    transition: 'opacity 0.2s, transform 0.2s',
                                    pointerEvents: focusedId === r.id ? 'auto' : 'none',
                                }}
                            >
                                <IconButton size="small" disabled={focusedId !== r.id} onMouseDown={() => { }}>
                                    <InfoOutline />
                                </IconButton>
                                <IconButton size="small" disabled={focusedId !== r.id} onMouseDown={() => deleteReminder(r.id)}>
                                    <Delete />
                                </IconButton>
                            </Stack>
                        </Stack>
                    ))}
                </Stack>
            </Container>
            <Fab color="primary" onClick={addReminder} aria-label="add" sx={{
                position: 'fixed',      // FIXED floats relative to viewport
                bottom: 24,             // distance from bottom
                right: 24,              // distance from right
                zIndex: 1000,           // make sure it’s above everything
            }}>
                <Add />
            </Fab>
        </React.Fragment>
    )
}