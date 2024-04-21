// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{AppHandle, WindowBuilder, WindowUrl};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn spawn_window(app_handle: AppHandle) {
    println!("Spawning window");
    let handle = std::thread::spawn(move || {
        let handle = WindowBuilder::new(&app_handle, "welcome", WindowUrl::default())
            .build()
            .unwrap();
    });
    let _ = handle.join();
}

fn main() {
    tauri::Builder::default()
        .on_page_load(|window, _payload| {
            println!("Page loaded on");
        })
        .on_window_event(|event| {
            println!(
                "Window event, {} {:?}",
                event.window().label(),
                event.event()
            );
        })
        .invoke_handler(tauri::generate_handler![spawn_window])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
