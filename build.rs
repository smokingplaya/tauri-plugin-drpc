const COMMANDS: &[&str] = &["spawn_thread", "destroy_thread", "is_running", "set_activity", "clear_activity"];

fn main() {
  tauri_plugin::Builder::new(COMMANDS)
    .android_path("android")
    .ios_path("ios")
    .build();
}
