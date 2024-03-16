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

function Header({getSettings}: HeaderProps) {
    return (<>
            <div id="header">
                <h1>Calgary Rust Room Booking</h1>
                <FontAwesomeIcon icon={faCog} size="lg" className="settings" onClick={
                    // eslint-disable-next-line @typescript-eslint/no-unused-vars
                    (_event) => {
                        console.log(getSettings);
                        const dialog = document.getElementById("settings-dialog") as HTMLDialogElement;
                        // eslint-disable-next-line @typescript-eslint/no-unused-vars
                        dialog.addEventListener("close", (_event) => {
                            const cog = document.querySelector(".settings") as HTMLElement;
                            const header = document.getElementById("header") as HTMLElement;
                            header.appendChild(cog);
                        });
                        if (dialog.open) {
                            dialog.close();
                        } else {
                            const cog = document.querySelector(".settings") as HTMLElement;
                            const sdheader = document.getElementById("settings-dialog-header") as HTMLElement;
                            sdheader.appendChild(cog);
                            dialog.showModal();
                        }
                    }
                } />
                <dialog id="settings-dialog">
                    <div id="settings-dialog-header"></div>
                    <SettingsDialogBody/>
                </dialog>
            </div>
        </>
    )
  }
  
  export default Header
