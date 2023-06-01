import { invoke } from '@tauri-apps/api';

export function openFile(path: string, openInFolder = false) {
  return invoke<void>('open_file', { path, openInFolder });
}
