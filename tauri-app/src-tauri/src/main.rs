// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use calgary_central_library::{AsyncClient, Availability, Client, Room};

#[tauri::command]
#[specta::specta]
async fn available_rooms(
    days_from_today: u8,
    group_size: u8,
) -> Result<Vec<(Room, Availability)>, String> {
    let days_from_today: i64 = days_from_today.try_into().unwrap();
    let now = chrono::Local::now();
    let today = now.date_naive();
    let day = today
        + chrono::Duration::try_days(days_from_today).ok_or(format!(
            "Failed to create chorono::Duration from the value {days_from_today} of {var_name}",
            var_name = stringify!(days_from_today)
        ))?;

    let chromedriver_addr = ("localhost", 4444);
    let client = Client::new(chromedriver_addr)
        .await
        .map_err(|e| e.to_string())?;
    let rooms = client
        .available_rooms(day, group_size)
        .await
        .map_err(|e| e.to_string())?;
    Ok(rooms)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![available_rooms])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use crate::available_rooms;

    #[test]
    fn generate_bidings() {
        tauri_specta::ts::export(
            specta::collect_types![available_rooms],
            "../bindings/bindings.ts",
        )
        .unwrap();
    }
}
