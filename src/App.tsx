import './App.css'
import * as React from 'react';
import CssBaseline from '@mui/material/CssBaseline';
import Header from './components/Header';
import { BrowserRouter, Route, Routes } from 'react-router-dom';
import ReminderPage from './pages/ReminderPage';
import AboutPage from './pages/AboutPage';
import { createTheme, ThemeProvider } from '@mui/material';

const theme = createTheme({
  colorSchemes: {
    dark: true,
  },
});

function App() {

  const [showCompleted, setShowCompleted] = React.useState<boolean>(() => {
    // Load from localStorage on first render
    const saved = localStorage.getItem('showCompleted');
    return saved ? JSON.parse(saved) : true; // default: show completed
  });

  // Save whenever it changes
  React.useEffect(() => {
    localStorage.setItem('showCompleted', JSON.stringify(showCompleted));
  }, [showCompleted]);

  return (
    <React.Fragment>
      <ThemeProvider theme={theme}>
        <CssBaseline />
        <>
          <BrowserRouter basename="/ReminderBox/">
            <Header showCompleted={showCompleted} setShowCompleted={setShowCompleted} />
            <Routes>
              <Route path="/" element={<ReminderPage showCompleted={showCompleted} />} />
              <Route path="/about" element={<AboutPage />} />
            </Routes>
          </BrowserRouter>
        </>
      </ThemeProvider>
    </React.Fragment>
  )
}

export default App
