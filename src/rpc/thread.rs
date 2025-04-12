use std::sync::LazyLock;
use tokio::{sync::Mutex, task::JoinHandle};

use crate::rpc::rpc;

static RPC_THREAD: LazyLock<Mutex<Option<JoinHandle<()>>>> = LazyLock::new(Mutex::default);

#[tauri::command]
pub(crate) async fn spawn_thread(id: String) -> Result<(), String> {
  let mut guard = RPC_THREAD.lock().await;

  if guard.is_some() {
    return Err("Discord RPC thread already spawned!".into());
  }

  *guard = Some(tokio::spawn(async move {
    if let Err(e) = rpc::initialize(id).await {
      log::error!("Discord RPC error: {e}");
    }
  }));

  Ok(())
}

#[tauri::command]
pub(crate) async fn destroy_thread() -> Result<(), String> {
  let mut guard = RPC_THREAD.lock().await;

  if let Some(thread) = guard.as_ref() {
    thread.abort();
    *guard = None;
    return Ok(());
  }

  Err("Discord RPC thread not found".into())
}

#[tauri::command]
pub(crate) async fn is_running() -> bool {
  let guard = RPC_THREAD.lock().await;

  guard.as_ref().map(|t| !t.is_finished()).unwrap_or(false)
}