// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_location])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn get_location() -> Option<String> {
    if let Some(ip) = public_ip::addr().await {
        println!("public ip address: {:?}", ip);
        Some(ip.to_string())
    } else {
        println!("couldn't get an IP address");
        None
    }
}
