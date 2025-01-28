use anyhow::Result;
use once_cell::sync::Lazy;
use tokio::{sync::Mutex, task::JoinHandle};
use crate::rpc::rpc;

static RPC_THREAD: Lazy<Mutex<Option<JoinHandle<()>>>> = Lazy::new(|| Mutex::new(None));

#[tauri::command]
pub(crate) async fn spawn_thread(
  id: String
) -> Result<String, String> {
  let mut guard = RPC_THREAD.lock()
    .await;

  if guard.is_some() {
    return Ok(String::from("Thread already spawned!"));
  }

  *guard = Some(tokio::spawn(async {
    match rpc::initialize(id).await {
      Ok(_) => (),
      Err(e) => log::error!("Rpc error: {e}"),
    }
  }));

  Ok(String::from("Rpc thread spawned"))
}

#[tauri::command]
pub(crate) async fn destroy_thread() -> Result<String, String> {
  let mut guard = RPC_THREAD.lock().await;

  if let Some(thread) = guard.as_ref() {
    thread.abort();

    *guard = None;

    return Ok(String::from("Rpc thread destroyed"));
  }

  Err(String::from("Rpc thread not found"))
}