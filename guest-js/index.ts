import { invoke } from '@tauri-apps/api/core';
import { Activity } from './activity';

export async function start(id: string) {
  return await invoke('plugin:drpc|spawn_thread', { id });
}

export async function destory() {
  return await invoke('plugin:drpc|destroy_thread');
}

export async function setActivity(activity: Activity) {
  return await invoke("plugin:drpc|set_activity", {
    activityJson: activity.toString()
  });
}