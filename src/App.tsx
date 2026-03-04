import './App.css'
import * as React from 'react';
import CssBaseline from '@mui/material/CssBaseline';
import Header from './components/Header';
import { BrowserRouter, Route, Routes } from 'react-router-dom';
import ReminderPage from './pages/ReminderPage';
import AboutPage from './pages/AboutPage';
import { createTheme, ThemeProvider } from '@mui/material';
import { enUS } from '@mui/x-date-pickers/locales';
import { LocalizationProvider } from '@mui/x-date-pickers';
import { AdapterDayjs } from '@mui/x-date-pickers/AdapterDayjs';
import { useFiveMinuteClock } from './effects/timer';
import { useReminders, useShowCompleted } from './effects/storage';

const theme = createTheme({
  colorSchemes: {
    dark: true,
  },
});

function App() {
  // 5 Minute timer on the minute
  // Used for updating due dates and notifications
  const fiveMinuteClock = useFiveMinuteClock();
  // Configurations
  const [reminders, setReminders] = useReminders();
  const [showCompleted, setShowCompleted] = useShowCompleted();

  return (
    <React.Fragment>
      <ThemeProvider theme={theme}>
        <LocalizationProvider
          localeText={enUS.components.MuiLocalizationProvider.defaultProps.localeText}
          dateAdapter={AdapterDayjs}
        >
          <CssBaseline />
          <>
            <BrowserRouter basename="/ReminderBox/">
              <Header showCompleted={showCompleted} setShowCompleted={setShowCompleted} />
              <Routes>
                <Route path="/" element={<ReminderPage reminders={reminders} setReminders={setReminders} showCompleted={showCompleted} fiveMinuteClock={fiveMinuteClock} />} />
                <Route path="/about" element={<AboutPage />} />
              </Routes>
            </BrowserRouter>
          </>
        </LocalizationProvider>
      </ThemeProvider>
    </React.Fragment>
  )
}

export default App
