use std::{error::Error, time::Duration};
use anyhow::Result;
use rpcdiscord::{DiscordIpc, DiscordIpcClient};
use once_cell::sync::Lazy;
use tokio::{sync::Mutex, time::sleep};

const SLEEP_FOR: Duration = Duration::from_secs(3);

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

    sleep(SLEEP_FOR)
      .await;
  }

  loop {
    if let Some(json) = get_activity().await {
      let payload = serde_json::from_str(&json)
        .unwrap(); // yeah i can do that shit here

      if client.set_activity(payload).is_err() && client.reconnect().is_ok() {
        std::thread::sleep(std::time::Duration::from_secs(5));

        continue;
    }

      std::thread::sleep(std::time::Duration::from_secs(1));
    }
  }
}