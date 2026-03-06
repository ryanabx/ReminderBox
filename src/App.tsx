import './App.css'
import Header from './components/Header';
import { BrowserRouter, Route, Routes } from 'react-router-dom';
import ReminderPage from './pages/ReminderPage';
import AboutPage from './pages/AboutPage';
import { useFiveMinuteClock } from './effects/timer';
import { useReminders, useShowCompleted } from './effects/storage';

function App() {
  // 5 Minute timer on the minute
  // Used for updating due dates and notifications
  const fiveMinuteClock = useFiveMinuteClock();
  // Configurations
  const [reminders, setReminders] = useReminders();
  const [showCompleted, setShowCompleted] = useShowCompleted();

  return (
    <BrowserRouter basename="/ReminderBox/">
      <Header showCompleted={showCompleted} setShowCompleted={setShowCompleted} />
      <Routes>
        <Route path="/" element={<ReminderPage reminders={reminders} setReminders={setReminders} showCompleted={showCompleted} fiveMinuteClock={fiveMinuteClock} />} />
        <Route path="/about" element={<AboutPage />} />
      </Routes>
    </BrowserRouter>
  )
}

export default App
