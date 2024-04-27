// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{AppHandle, Manager, WindowBuilder, WindowEvent, WindowUrl};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn spawn_window(app_handle: AppHandle, data: String) {
    println!("Spawning window");
    let handle = std::thread::spawn(move || {
        let window = WindowBuilder::new(
            &app_handle,
            "dynamicWindow",
            WindowUrl::App("dynamicWindow.html".into()),
        )
        .build()
        .unwrap();

        let event_window = window.clone();
        window.listen("window-ready", move |event| {
            event_window.emit("greet-msg", &data);
        });
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
