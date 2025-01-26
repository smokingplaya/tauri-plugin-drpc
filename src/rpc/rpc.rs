use std::error::Error;
use anyhow::Result;
use rpcdiscord::{DiscordIpc, DiscordIpcClient};
use once_cell::sync::Lazy;
use tokio::sync::Mutex;

static ACTIVITY: Lazy<Mutex<Option<String>>> = Lazy::new(|| Mutex::new(None));

#[tauri::command]
pub async fn set_activity(
  activity_json: String
) -> Result<String, String> {
  *ACTIVITY.lock()
    .await = Some(activity_json);

  Ok(String::from("Rpc activity updated"))
}

async fn get_activity() -> Option<String> {
  ACTIVITY.lock()
    .await
    .as_ref()
    .cloned()
}

pub(crate) async fn initialize(
  id: String
) -> Result<(), Box<dyn Error>> {
  let mut client = DiscordIpcClient::new(&id)?;

  loop {
    if client.connect().is_ok() {
        break;
    }
  }

  loop {
    if let Some(json) = get_activity().await {
      let payload = serde_json::from_str(&json)
        .unwrap(); // yeah i can do that shit here

      if client.set_activity(payload).is_err() && client.reconnect().is_ok() {
        continue;
    }

      std::thread::sleep(std::time::Duration::from_secs(2));
    }
  }
}