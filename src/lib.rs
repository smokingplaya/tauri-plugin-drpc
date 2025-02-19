use rpc::{rpc::{set_activity, clear_activity}, thread::{spawn_thread, destroy_thread, is_running}};
use tauri::{plugin::{Builder, TauriPlugin}, Runtime};

mod rpc;

pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("drpc")
    .invoke_handler(tauri::generate_handler![spawn_thread, destroy_thread, is_running, set_activity, clear_activity])
    .build()
}