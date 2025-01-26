use rpc::{rpc::set_activity, thread::{spawn_thread, destroy_thread}};
use tauri::{plugin::{Builder, TauriPlugin}, Runtime};

mod rpc;

pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("drpc")
    .invoke_handler(tauri::generate_handler![spawn_thread, destroy_thread, set_activity])
    .build()
}