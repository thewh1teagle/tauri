#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::path::PathBuf;
use std::time::Duration;

use tauri;
use tauri::{LogicalPosition, LogicalSize, WebviewUrl};


fn setup(app: &tauri::App) -> Result<(), Box<dyn std::error::Error>> {
  let width = 800.;
  let height = 600.;

  let window_builder = tauri::WindowBuilder::new(app, "main")
        .inner_size(width, height)
        .min_inner_size(width, height)
        .shadow(true)
        .visible(true)
        .focused(true)
        .resizable(true)
        .title("MultiWebview");

  let mut titlebar_builder: tauri::WebviewBuilder<tauri::Wry> = tauri::WebviewBuilder::new(
    "menu",
      WebviewUrl::External("https://github.com/".parse().unwrap())
  );

  let mut content_builder: tauri::WebviewBuilder<tauri::Wry> = tauri::WebviewBuilder::new(
    "content",
      WebviewUrl::External("https://google.com/".parse().unwrap())
  ).data_directory(PathBuf::from("content"));
  let window = window_builder.build().unwrap();
  window.add_child(
    titlebar_builder, 
    LogicalPosition::new(0., 0.),
     LogicalSize::new(window.outer_size().unwrap().width.into(),100.0)
  ).unwrap();
  std::thread::sleep(Duration::from_secs(1)); // for checking
  window.add_child(
    content_builder, 
    LogicalPosition::new(0., 100.), LogicalSize::new(window.inner_size().unwrap().width as u32,window.outer_size().unwrap().height as u32 - 100)
  ).unwrap();
  
  Ok(())
}

fn main() {
    tauri::Builder::default()
        .setup(|app| setup(app))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}