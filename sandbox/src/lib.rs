use transmission;
use tauri::Emitter;

#[tauri::command]
fn log_message(message: String) {
  println!("Tauri received message: {}", message);
}

#[tauri::command]
fn send_message(app: tauri::AppHandle, message: String) {
  println!("Tauri received message: {}", message);
  match app.emit("message", message) {
      Ok(_) => (),
      Err(e) => eprintln!("Error emitting message: {}", e),
  }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .setup(|app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![log_message, send_message])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
