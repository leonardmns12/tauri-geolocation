// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use geolocation::Location;
use tokio::sync::Mutex;
use crate::geolocation::Geolocation;

mod geolocation;

#[tauri::command]
async fn get_location(geolocation: tauri::State<'_, Mutex<Geolocation>>) -> Result<Option<Location>, ()> {
    let mut base = geolocation.inner().lock().await;
    let response = base.get_location().await;
    Ok(response)
}

fn main() {
    let geolocation = Mutex::new(Geolocation::new()); 

    tauri::Builder::default()
        .manage(geolocation)
        .invoke_handler(tauri::generate_handler![get_location])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

