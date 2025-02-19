import { invoke } from '@tauri-apps/api/core';
import { Activity } from './activity';

/**
 * Spawns a thread that manages the Discord Rich Presence state
 *
 * @param id Discord application (client) Id ()
 * @throws If the Drpc thread is already running
 */
export async function spawn(id: string) {
  await invoke('plugin:drpc|spawn_thread', { id });
}

/**
 * Destroys a thread that manages the Discord Rich Presence state
 *
 * @throws If the Drpc thread is not running
 */
export async function destroy() {
  await invoke('plugin:drpc|destroy_thread');
}

/**
 * @returns Is drpc thread running?
 */
export async function isRunning(): Promise<boolean> {
  return await invoke('plugin:drpc|is_running');
}

/**
 * Safe function that starts Discord RPC thread
 * @param id Discord Application ID
 */
export async function start(id: string) {
  await stop();

  await spawn(id);
}

/**
 * Safe function that stops Discord RPC thread
 * @param id Discord Application ID
 */
export async function stop() {
  if (await isRunning())
    await destroy();
}

export async function setActivity(activity: Activity) {
  await invoke("plugin:drpc|set_activity", {
    activityJson: activity.toString()
  });
}

export async function clearActivity() {
  if (!await isRunning())
    return;

  await invoke("plugin:drpc|clear_activity");
}