import { faCog } from '@fortawesome/free-solid-svg-icons';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import "./Header.css";
import { Settings } from "./App.tsx";
import { Dispatch, SetStateAction } from 'react';
import SettingsDialogBody from './SettingsDialogBody.tsx';

type HeaderProps = {
    getSettings: Settings;
    setSettings: Dispatch<SetStateAction<Settings>>;
};

function Header({getSettings, setSettings}: HeaderProps) {
    return (<>
            <div id="header">
                <h1>Calgary Rust Room Booking</h1>
                <FontAwesomeIcon icon={faCog} size="lg" className="settings" onClick={ (_event) => {
                    console.log(getSettings);
                    let dialog = document.getElementById("settings-dialog") as HTMLDialogElement;
                    dialog.addEventListener("close", (_event) => {
                        let cog = document.querySelector(".settings") as HTMLElement;
                        let header = document.getElementById("header") as HTMLElement;
                        header.appendChild(cog);
                    });
                    if (dialog.open) {
                        dialog.close();
                    } else {
                        let cog = document.querySelector(".settings") as HTMLElement;
                        let sdheader = document.getElementById("settings-dialog-header") as HTMLElement;
                        sdheader.appendChild(cog);
                        dialog.showModal();
                    };
                } } />
                <dialog id="settings-dialog">
                    <div id="settings-dialog-header"></div>
                    <SettingsDialogBody/>
                </dialog>
            </div>
        </>
    )
  }
  
  export default Header
