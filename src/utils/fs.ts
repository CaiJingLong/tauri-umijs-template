import { invoke } from '@tauri-apps/api';
import iconv from 'iconv-lite';

export function isDir(path: string): Promise<boolean> {
  return invoke<boolean>('is_dir', { path });
}

export function isFile(path: string): Promise<boolean> {
  return invoke<boolean>('is_file', { path });
}

export function isLink(path: string): Promise<boolean> {
  return invoke<boolean>('is_link', { path });
}

export function exists(path: string): Promise<boolean> {
  return invoke<boolean>('exists', { path });
}

export function mkdir(path: string, isRecursive = true): Promise<boolean> {
  return invoke<boolean>('create_dir', { path, isRecursive });
}

export function copyFile(src: string, dest: string): Promise<boolean> {
  return invoke<boolean>('cp_file', { src, dest });
}

export function copyDir(src: string, dest: string): Promise<boolean> {
  return invoke<boolean>('cp_dir', { src, dest });
}

export function removeFile(path: string): Promise<boolean> {
  return invoke<boolean>('remove_file', { path });
}

export function removeDir(path: string): Promise<boolean> {
  return invoke<boolean>('remove_dir', { path });
}

export function readBuffer(path: string): Promise<Array<number>> {
  return invoke<Array<number>>('read_buffer', { path });
}

export function writeBuffer(
  path: string,
  contents: number[],
): Promise<boolean> {
  return invoke<boolean>('write_buffer', { path, contents });
}

export function appendBuffer(
  path: string,
  contents: Uint8Array,
): Promise<boolean> {
  return invoke<boolean>('append_buffer', { path, contents });
}

export async function readText(path: string, encoding: string = 'utf-8') {
  const bytes = await readBuffer(path);
  const buffer = Buffer.from(bytes);
  return iconv.decode(buffer, encoding);
}

export function writeText(
  path: string,
  text: string,
  encoding: string = 'utf-8',
) {
  const buffer = iconv.encode(text, encoding);
  const bytes = Array.from(buffer);
  return writeBuffer(path, bytes);
}

export function appendText(
  path: string,
  text: string,
  encoding: string = 'utf-8',
) {
  const buffer = iconv.encode(text, encoding);
  return appendBuffer(path, buffer);
}

interface File {
  name: string;
  isDir: boolean;
  isFile: boolean;
  isLink: boolean;
  fullPath: string;
}

type FileStr = File & {
  toString: () => string;
};

export async function childList(path: string): Promise<FileStr[]> {
  const fullPathList = await invoke<string[]>('child_list', { path });

  const result: FileStr[] = [];

  for (const fullPath of fullPathList) {
    const name = fullPath.replaceAll(path, '').slice(1);
    const pathIsDir = await isDir(fullPath);
    const pathIsFile = await isFile(fullPath);
    const pathIsLink = await isLink(fullPath);

    const file = {
      name,
      isDir: pathIsDir,
      isFile: pathIsFile,
      isLink: pathIsLink,
      fullPath,
    };

    result.push({ ...file, toString: () => JSON.stringify(file) });
  }

  return result;
}

export function renameFile(src: string, dest: string): Promise<boolean> {
  return invoke<boolean>('rename_file', { src, dest });
}

