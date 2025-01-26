# Tauri Plugin DRPC
A plugin for [Tauri](https://tauri.app/) that allows you to control Discord Rich Presence.

# Installation
You can download plugin from ``crates.io``
```bash
cd src-tauri
cargo add tauri-plugin-drpc
npm i tauri-plugin-drpc
```

# Usage example
```ts
import { setActivity } from "tauri-plugin-drpc";
import { Activity, Assets } from "tauri-plugin-drpc/activity";

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