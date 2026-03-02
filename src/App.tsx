import './App.css'
import * as React from 'react';
import CssBaseline from '@mui/material/CssBaseline';
import Header from './components/Header';
import { BrowserRouter, Route, Routes } from 'react-router-dom';
import ReminderPage from './pages/ReminderPage';
import AboutPage from './pages/AboutPage';

function App() {
  return (
    <React.Fragment>
      <CssBaseline />
      <>
        <BrowserRouter basename="/ReminderBox/">
          <Header />
          <Routes>
            <Route path="/" element={<ReminderPage />} />
            <Route path="/about" element={<AboutPage />} />
          </Routes>
        </BrowserRouter>
      </>
    </React.Fragment>
  )
}

export default App
