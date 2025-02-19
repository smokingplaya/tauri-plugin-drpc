# Tauri Plugin DRPC

A plugin for [Tauri](https://tauri.app/) that allows you to control Discord Rich Presence.

> [!NOTE]
> This plugin is designed to be used on Desktop only.

# Table of contents
* [Concept (how it works)](#concept)
* [Installation](#installation)
* [Usage](#usage)
  * [Thread managment](#thread-managment)
  * [Activity managment](#activity-managment)
  * [Activity components](#activity-components)
    * [Assets](#assets)
    * [Buttons](#buttons)
    * [Party](#party)
    * [Timestamps](#timestamps)

# Concept
The plugin implements an interface *([as a JavaScript plugin](./guest-js/README.md))* between JavaScript and crate for Rust [discord-rich-presence](https://github.com/vionya/discord-rich-presence) (we have our own fork of it - [rpcdiscord](https://github.com/smokingplaya/rpcdiscord)).

Right now the plugin works as follows:
A developer has to call a function (`spawn`) in the code that sparks a thread, and that thread keeps Discord Rich Presence running, constantly updating the activity in it. See [Usage](#usage)

> [!NOTE]
> We are now looking at an option without using the creation of separate threads, since Tauri allows you to do asynchronous commands.

# Installation
[![version](https://img.shields.io/crates/v/tauri-plugin-drpc)](https://crates.io/crates/tauri-plugin-drpc)

Open a terminal, and in the root folder of your project, type these commands:
```bash
cd src-tauri
cargo add tauri-plugin-drpc
```
or just type this line into ``src-tauri/Cargo.toml``.
```toml
tauri-plugin-drpc = "*"
```

Then install the package for JavaScript (in root folder of your project):
```bash
npm i tauri-plugin-drpc
```

# Usage
## Thread managment
As I said earlier [(here)](#concepts), to use Discord Rich Presence you need to spawn a thread, which is what will run Rich Presence.

This can be done through the ``start`` function:
```js
import { start } from "tauri-plugin-drpc";

// start(APPLICATION_ID)
await start("700000000000000000");
```

The ``start`` function checks that the drpc thread is not running, if it is running it stops it and then starts a new one.

This means that the ``start`` function cannot throw an error, and it is safe to use.

If you need to start a thread without checks, you can use the ``spawn`` function.

```js
import { spawn } from "tauri-plugin-drpc";

try {
  // spawn(APPLICATION_ID)
  await spawn("700000000000000000");
} catch (err) {
  console.error(err);
}
```

If you need to, you can stop the thread yourself via the ``stop`` function.

The ``stop`` function is as safe as the ``spawn`` function, it stops the thread only if it is running, so it does not throw an error.

```js
import { stop } from "tauri-plugin-drpc";

await stop();
```

And there is a function ``destroy`` which stops the thread without checks and may throw an error, so it should be wrapped in ``try {} catch (...)``.

```js
import { destroy } from "tauri-plugin-drpc";

try {
  await destroy();
} catch (err) {
  console.error(err);
}
```

# Activity managment
> [!NOTE]
> You can get the Application ID in the [settings of your Discord application](https://discord.com/developers/applications), in the `General Information` tab.

You can now customize your Discord Rich Presence activity. This can be done through the ``setActivity`` function. But before that, you need to create an activity object. I have written about this [below](#activity-managment).

You can also clear the activity via ``clearActivity``.

Example:
```js
import { setActivity, clearActivity, destroy } from "tauri-plugin-drpc";
import { Activity } from "tauri-plugin-drpc";

const activity = new Activity()
  .setState("example string")
  .setState("hello")
  .setTimestamps(new Timestamps(Date.now())

await setActivity(activity);

setTimeout(async () => {
  console.log("Clearing activity");

  // clear drpc's activity
  await clearActivity();
  // destroy drpc's thread
  await destroy();
}, 3000)
```

# Activity components
## Assets
Assets is responsible for the large and small icons on the left side of the activity.

```js
import { Assets, Activity } from "tauri-plugin-drpc/activity";

const assets = new Assets()
  .setLargeImage("IMAGE_ID")
  .setLargeText("Large image hovered!")
  .setSmallImage("IMAGE_ID")
  .setSmallText("Small image hovered!")

const activity = new Activity()
  .setAssets(assets);
```

## Buttons
``Button`` add buttons to your activity, when clicked, the user will have the link given to each button open in the browser.

```js
import { Button } from "tauri-plugin-drpc/activity";

const activity = new Activity()
  .setButtons([
    new Button("Button text #1", "https://example.com"),
    new Button("Button text #2", "https://example.com"),
  ]);
```

## Party
There is no documentation for the ``Party`` component, so you, dear friend, will have to check the [discord-rich-presence crate documentation](https://docs.rs/discord-rich-presence/) to understand how this component works.

## Timestamps
```js
import { Timestamps, Activity } from "tauri-plugin-drpc/activity";

const timestamp = new Timestamps(Date.now() - 1000000); // 1000 seconds ago

const activity = new Activity()
  .setTimestamps(timestamp);
```