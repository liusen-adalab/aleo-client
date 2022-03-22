#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    // windows_subsystem = "windows"
)]
use aleo_prover;

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![start_mine, create_key])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn start_mine(addr: String, pool: String) -> Result<String, String> {
    aleo_prover::start_mine(addr, pool);
    Ok("Ok".to_string())
}

#[tauri::command]
fn create_key() -> Result<String, String> {
    Ok("Ok".to_string())
}
