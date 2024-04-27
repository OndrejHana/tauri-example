// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{AppHandle, WindowBuilder, WindowUrl};

#[tauri::command]
fn spawn_window(app: AppHandle, msg: String) {
    let handle = std::thread::spawn(move || {
        let window = WindowBuilder::new(
            &app,
            "dynamicWindow",
            WindowUrl::App("dynamicWindow.html".into()),
        )
        .initialization_script(&format!(r#"localStorage.setItem("greetMsg", "{}");"#, &msg))
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
