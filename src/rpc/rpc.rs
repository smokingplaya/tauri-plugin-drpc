use std::{error::Error, sync::LazyLock, time::Duration};
use rpcdiscord::{DiscordIpc, DiscordIpcClient};
use tokio::{sync::Mutex, time::sleep};

const SLEEP_FOR: Duration = Duration::from_secs(3);

static ACTIVITY: LazyLock<Mutex<Option<String>>> = LazyLock::new(Mutex::default);

#[tauri::command]
pub async fn set_activity(activity_json: String) -> Result<(), String> {
  *ACTIVITY.lock().await = Some(activity_json);
  Ok(())
}

#[tauri::command]
pub async fn clear_activity() -> Result<(), String> {
  *ACTIVITY.lock().await = Some("{}".to_string()); // Очистка активити
  Ok(())
}

async fn get_activity() -> Option<String> {
  ACTIVITY.lock().await.as_ref().cloned()
}

pub(crate) async fn initialize(id: String) -> Result<(), Box<dyn Error>> {
  let mut client = DiscordIpcClient::new(&id)?;

  // Подключение к Discord IPC
  loop {
    if client.connect().is_ok() {
      break;
    }
    sleep(SLEEP_FOR).await;
  }

  // Основной цикл обновления активности
  loop {
    if let Some(json) = get_activity().await {
      let payload_result = serde_json::from_str(&json);

      if let Ok(payload) = payload_result {
        if client.set_activity(payload).is_err() && client.reconnect().is_ok() {
          std::thread::sleep(Duration::from_secs(5));
          continue;
        }
        std::thread::sleep(Duration::from_secs(1));
      } else {
        log::error!("Activity has invalid JSON");
        std::thread::sleep(Duration::from_secs(3));
      }
    }
  }
}
