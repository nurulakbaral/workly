#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tauri::menu::{MenuBuilder, SubmenuBuilder};
use tauri::Emitter;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let app_menu = SubmenuBuilder::new(app, "Workly App")
                .text("about", "About")
                .build()?;
            let file_menu = SubmenuBuilder::new(app, "File")
                .text("open", "Open")
                .text("close", "Close")
                .build()?;
            let edit_menu = SubmenuBuilder::new(app, "Edit")
                .text("copy", "Copy")
                .build()?;

            let menu = MenuBuilder::new(app)
                .items(&[&app_menu, &file_menu, &edit_menu])
                .build()?;

            app.set_menu(menu)?;

            app.on_menu_event(move |app_handle: &tauri::AppHandle, event| {
                match event.id().0.as_str() {
                    "about" => {
                        let _ = app_handle.emit("menu-event", "about");
                    }
                    "open" => {}
                    "close" => {}
                    _ => {
                        println!("Error: Unexpected Menu Event");
                    }
                }
            });

            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
