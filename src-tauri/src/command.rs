#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
  )]

#[tauri::command]
pub fn my_custom_command() {
  println!("I was invoked from JS!");
}

use std::env;

#[tauri::command]
pub fn os_details() -> &'static str {
  return env::consts::OS;
}

#[tauri::command]
pub fn greeting(name: &str) -> String {
  return format!("Hello, {}!", name)
}