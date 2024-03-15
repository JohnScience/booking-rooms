import { useState } from 'react';
import Calendar from './Calendar';
import Header from './Header';
import "./index.css";

type CrawlingServer =  {
  kind: "crawling_server",
  host: string,
  port: number,
}

type TauriWebview = {
  kind: "tauri_webview",
}

type LibraryDataSource = CrawlingServer | TauriWebview;

type AvailabilityForDay = null;

export function getAvailabilityForDay(source: LibraryDataSource, date: Date): AvailabilityForDay {
  switch (source.kind) {
    case "tauri_webview":
      console.log("Calling getAvailabilityForDay() with a tauri_webview");
      break;
    case "crawling_server":
      console.log("Calling getAvailabilityForDay() with a crawling_server");
      break;
  };
  // the difference in days from today to the date
  let delta = (date.getTime() - new Date().getTime()) / (1000 * 3600 * 24);
  console.log("delta =", delta);

  return null;
}

type Settings = {
  attendance: number,
  libraryDataSource: LibraryDataSource,
};

function App() {
  const [getSettings, setSettings] = useState<Settings>(DEFAULT_SETTINGS);

  return (
    <>
      <Header getSettings={getSettings} setSettings={setSettings} />
      <Calendar getSettings={getSettings} />
    </>
  )
}

export const DEFAULT_SETTINGS: Settings = {
  attendance: 10,
  libraryDataSource: { kind: "tauri_webview" }
};
export type { Settings };
export default App;
