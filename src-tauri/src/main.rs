// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{thread::{sleep, spawn}, time::Duration};

use tauri::{AppHandle, Manager};


fn expensive_operation() -> String {
    sleep(Duration::from_millis(5000));
    "got it".to_string()
}

fn close_splashscreen(app: AppHandle) {
    app.get_window("main").unwrap().show().unwrap();
    app.get_window("loading-screen").unwrap().hide().unwrap();
}

fn main() {
    tauri::Builder::default()
        .setup(|app|{ 

            let handle = app.handle();
            let _ = spawn(move || {
                expensive_operation();
                close_splashscreen(handle);
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
