import { GitHub } from "@mui/icons-material";
import { Container, IconButton, Stack, Typography } from "@mui/material";

export default function AboutPage() {
    return (
        <Container maxWidth="sm">
            <Stack alignItems="center">
                <Typography variant="h5" component="div" sx={{ flexGrow: 1 }}>
                    ReminderBox
                </Typography>
                <Typography variant="body1" component="div" sx={{ flexGrow: 1 }}>
                    Commit {__APP_COMMIT__}
                </Typography>
                <IconButton target="_blank" href="https://github.com/ryanabx/ReminderBox" onClick={() => { }}>
                    <GitHub />
                </IconButton>
            </Stack>
        </Container>
    )
}