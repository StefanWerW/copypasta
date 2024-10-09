// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;
use std::thread;

use arboard::{Clipboard, ImageData};
use tauri::{window, Manager, SystemTray, SystemTrayEvent};
use tauri::{CustomMenuItem, SystemTrayMenu, SystemTrayMenuItem};

 

#[derive(Clone)]
enum ClipboardHistoryItem <'a>{
  Text(String),
  Image(ImageData<'a>),
}

impl PartialEq for ClipboardHistoryItem<'_> {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (ClipboardHistoryItem::Text(a), ClipboardHistoryItem::Text(b)) => a == b,
      (ClipboardHistoryItem::Image(a), ClipboardHistoryItem::Image(b)) => a.bytes == b.bytes,
      _ => false,
    }
  }
}

impl Eq for ClipboardHistoryItem<'_> {}

struct ClipboardHistory <'a>{
  current: Option<ClipboardHistoryItem<'a>>,
  items: Vec<ClipboardHistoryItem<'a>>,
}

impl<'a> ClipboardHistory <'a>{
  fn new() -> Self {
    Self { items: Vec::new(), current: None }
  }

  fn add_item(&mut self, item: ClipboardHistoryItem<'a>) {
    self.current = Some(item.clone());
    self.items.push(item.clone());
  }

  fn get_item(&self, index: usize) -> Option<&ClipboardHistoryItem> {
    self.items.get(index)
  }
}

fn main() {

  let clipboard_history = Mutex::new(ClipboardHistory::new());

  // thread::spawn(move || {
  //   let mut clipboard = Clipboard::new().unwrap();
  //   loop {
  //     let clipboard_text = clipboard.get_text();
  //     let clipboard_image = clipboard.get_image();

  //     match clipboard_text {
  //       Ok(text) => {
  //         let mut clipboard_history = clipboard_history.lock().unwrap();
  //         clipboard_history.add_item(ClipboardHistoryItem::Text(text));
  //       }
  //       Err(_) => {}
  //     }
  //   }
  // });

  let quit = CustomMenuItem::new("quit".to_string(), "Quit");
  let open = CustomMenuItem::new("open".to_string(), "Open");
  let tray_menu = SystemTrayMenu::new().add_item(open).add_native_item(SystemTrayMenuItem::Separator).add_item(quit);
  let system_tray = SystemTray::new()
    .with_menu(tray_menu);

  tauri::Builder::default()
  .manage(clipboard_history)
  .system_tray(system_tray)
  .on_system_tray_event(|app, event| match event {
    SystemTrayEvent::MenuItemClick { tray_id, id, .. } => {
      let item_handle = app.tray_handle().get_item(&id);
      match id.as_str() {
        "quit" => {
          app.exit(0);
        }
        "open" => {
          
        }
        _ => {}
      }
    }
    _ => {}
  })
    .build(tauri::generate_context!())
    .expect("error while running tauri application")
    .run(|_app_handle, event| match event {
      tauri::RunEvent::ExitRequested { api, .. } => {
        api.prevent_exit();
      }
      _ => {}
    });
}
