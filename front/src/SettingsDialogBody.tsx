import { Settings, DEFAULT_SETTINGS } from "./App.tsx";
import { Dispatch, SetStateAction } from 'react';

type SettingsDialogBodyProps = {
    getSettings: Settings;
    setSettings: Dispatch<SetStateAction<Settings>>;
};

function SettingsDialogBody(/*{getSettings, setSettings}: SettingsDialogBodyProps*/): JSX.Element {
    function updateToggleStateForWebdriverInputs() {
        const dataSourceSelect = document.getElementById("library-data-source") as HTMLSelectElement;
        const dataSource = dataSourceSelect.value;
        if (dataSource === "crawling_server") {
            document.getElementById("webdriver-host")!.removeAttribute("disabled");
            document.getElementById("webdriver-port")!.removeAttribute("disabled")
        } else {
            document.getElementById("webdriver-host")!.setAttribute("disabled", "true");
            document.getElementById("webdriver-port")!.setAttribute("disabled", "true");
        }
    }

    return (<>
            <div>
                <label htmlFor="attendance">Attendance: </label>
                <input type="number" id="attendance" list="attendance-options" min="5" max="100" defaultValue={DEFAULT_SETTINGS.attendance.toString()} />
                <datalist id="attendance-options">
                    <option value="10" />
                    <option value="16" />
                    <option value="25" />
                </datalist>
            </div>
            <div>
                <label htmlFor="library-data-source">Library data source: </label>
                <select id="library-data-source" onChange={updateToggleStateForWebdriverInputs} defaultValue={DEFAULT_SETTINGS.libraryDataSource.kind}>
                    <option value="tauri_webview">Tauri IPC (command)</option>
                    <option value="crawling_server">Crawling WebDriver server</option>
                </select>
            </div>
            <div>
                <label htmlFor="webdriver-host">WebDriver host: </label>
                <input id="webdriver-host" disabled={true} list="webdriver-host-options" defaultValue="localhost" />
                <datalist id="webdriver-host-options">
                    <option value="localhost" />
                </datalist>
            </div>
            <div>
                <label htmlFor="webdriver-port">WebDriver port: </label>
                <input type="number" id="webdriver-port" disabled={true} list="webdriver-port-options" min="0" max="65535" defaultValue="4444" />
                <datalist id="webdriver-port-options">
                    <option value="4444" />
                </datalist>
            </div>
        </>
    )
}
  
export default SettingsDialogBody
