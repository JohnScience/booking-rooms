import { useState } from 'react';
import Calendar from './Calendar';
import Header from './Header';
import "./index.css";
import { Settings, DEFAULT_SETTINGS } from './external';

function App() {
  const [getSettings, setSettings] = useState<Settings>(DEFAULT_SETTINGS);

  return (
    <>
      <Header getSettings={getSettings} setSettings={setSettings} />
      <Calendar getSettings={getSettings} />
    </>
  )
}

export default App;
