import { Room, Availability, availableRooms } from "../../tauri-app/bindings/bindings"

type CrawlingServer =  {
  kind: "crawling_server",
  host: string,
  port: number,
}

type TauriWebview = {
  kind: "tauri_webview",
}

type LibraryDataSource = CrawlingServer | TauriWebview;

type Settings = {
  attendance: number,
  libraryDataSource: LibraryDataSource,
};

export function getAvailabilityForDay(source: LibraryDataSource, date: Date, groupSize: number): Promise<([Room, Availability])[]> | null {
  const delta = Math.round((date.getTime() - new Date().getTime()) / (1000 * 3600 * 24));
  if (delta < 0) {
    console.log("Calling getAvailabilityForDay() with a date in the past");
    return null;
  }
  switch (source.kind) {
    case "tauri_webview":
      return availableRooms(delta, groupSize);
    case "crawling_server":
      console.log("Calling getAvailabilityForDay() with a crawling_server");
      break;
  }
  // the difference in days from today to the date
  
  console.log("delta =", delta);

  return null;
}

export const DEFAULT_SETTINGS: Settings = {
    attendance: 10,
    libraryDataSource: { kind: "tauri_webview" }
};


export type { Settings };
