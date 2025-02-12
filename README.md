# Tauri Plugin DRPC

A plugin for [Tauri](https://tauri.app/) that allows you to control Discord Rich Presence.

## Installation

You can download plugin from `crates.io`

1. Run the following command in the `src-tauri` folder to add the plugin to the project's dependencies in `Cargo.toml`:

```sh
cargo add tauri-plugin-cli --target 'cfg(any(target_os = "macos", windows, target_os = "linux"))'
```

2. Modify `lib.rs` to initialize the plugin:

```rust
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .setup(|app| {
      // we need this because drpc doesn't support mobile devices
      #[cfg(desktop)]
      app.handle()
        .plugin(tauri_plugin_drpc::init())
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
```

3. Install the JavaScript Guest bindings using your preferred JavaScript package manager:

```sh
npm i tauri-plugin-drpc
```

## Usage example

```ts
import { start, setActivity } from "tauri-plugin-drpc";
import { Activity, Assets } from "tauri-plugin-drpc/activity";

// Get it from https://discord.com/developers/applications
const applicationClientId = "APP_ID_HERE";

const assets = new Assets()
  .setLargeImage("IMAGE_ID")
  .setLargeText("Large Image Text");

const activity = new Activity()
  .setDetails("First line")
  .setState("Second line")
  .setAssets(assets);

(async () => {
  await setActivity(activity);
  // Spawn a discord rich presence thread
  await start(applicationClientId);
})();
```
