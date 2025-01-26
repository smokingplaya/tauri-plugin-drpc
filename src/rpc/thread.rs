use once_cell::sync::Lazy;
use tokio::{sync::Mutex, task::JoinHandle};
use crate::rpc::rpc;

static RPC_THREAD: Lazy<Mutex<Option<JoinHandle<()>>>> = Lazy::new(|| Mutex::new(None));

#[tauri::command]
pub(crate) async fn spawn_thread(
  id: String
) -> String {
  let mut guard = RPC_THREAD.lock()
    .await;

  *guard = Some(tokio::spawn(async {
    match rpc::initialize(id).await {
      Ok(_) => (),
      Err(e) => log::error!("Rpc error: {e}"),
    }
  }));

  String::from("Rpc thread spawned")
}

#[tauri::command]
pub(crate) async fn destroy_thread() -> String {
  let thread = RPC_THREAD.lock().await;

  if let Some(thread) = thread.as_ref() {
    thread.abort();

    return String::from("Rpc thread destroyed")
  }

  String::from("Rpc thread not found")
}