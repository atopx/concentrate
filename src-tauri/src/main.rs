#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]


use chrono::NaiveDateTime;


#[tauri::command]
fn timestamp_format(value: &str) -> String {
    println!("{}", value);
    let timestamp = value.parse::<i64>();
    match timestamp {
        Ok(ts) => {
            let time: NaiveDateTime = NaiveDateTime::from_timestamp(ts, 0);
            time.format("%Y-%m-%d %H:%M:%S").to_string()
        },
        Err(err) => {
            println!("invalid timestamp: {}", err);
            String::from("Invalid timestamp")
        }
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![timestamp_format])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
