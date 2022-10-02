#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
  )]

#[tauri::command]
pub fn my_custom_command() {
  println!("I was invoked from JS!");
}

#[tauri::command]
pub fn greet(name: &str) -> String {
  println!("Greetings, {}!", name);
  format!("Hello, {}!", name)
}

#[tauri::command]
pub fn greeting(name: &str) -> String {
  return format!("Hello, {}!", name)
}