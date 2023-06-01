// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
use std::process::Command;

#[tauri::command]
pub fn is_dir(path: &str) -> bool {
    std::path::Path::new(path).is_dir()
}

#[tauri::command]
pub fn is_file(path: &str) -> bool {
    std::path::Path::new(path).is_file()
}

#[tauri::command]
pub fn is_link(path: &str) -> bool {
    std::path::Path::new(path).is_symlink()
}

#[tauri::command]
pub fn exists(path: &str) -> bool {
    std::path::Path::new(path).exists()
}

#[tauri::command]
pub fn create_dir(path: &str, is_recursive: bool) -> bool {
    if is_recursive {
        std::fs::create_dir_all(path).is_ok()
    } else {
        std::fs::create_dir(path).is_ok()
    }
}

#[tauri::command]
pub fn child_list(path: &str) -> Vec<String> {
    let mut result = Vec::new();
    for entry in std::fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let path = path.to_str().unwrap();
        result.push(path.to_string());
    }
    result
}

#[tauri::command]
pub fn read_buffer(path: &str) -> Vec<u8> {
    std::fs::read(path).unwrap_or_default()
}

#[tauri::command]
pub fn write_buffer(path: &str, contents: Vec<u8>) -> bool {
    std::fs::write(path, contents).is_ok()
}

#[tauri::command]
pub fn append_buffer(path: &str, contents: Vec<u8>) -> bool {
    use std::fs::OpenOptions;
    use std::io::Write;

    let file: Result<std::fs::File, std::io::Error> =
        OpenOptions::new().append(true).create(true).open(path);

    match file {
        Ok(mut f) => {
            if let Err(err) = f.write_all(&contents) {
                eprintln!("Failed to write to file: {}", err);
                return false;
            }
            true
        }
        Err(err) => {
            eprintln!("Failed to open file: {}", err);
            false
        }
    }
}

#[tauri::command]
pub fn cp_file(src: &str, dest: &str) -> bool {
    std::fs::copy(src, dest).is_ok()
}

#[tauri::command]
pub fn cp_dir(src: &str, dest: &str) -> bool {
    let options = fs_extra::dir::CopyOptions::new();
    return fs_extra::dir::copy(src, dest, &options).is_ok();
}

#[tauri::command]
pub fn remove_dir(path: &str) -> bool {
    std::fs::remove_dir_all(path).is_ok()
}

#[tauri::command]
pub fn remove_file(path: &str) -> bool {
    std::fs::remove_file(path).is_ok()
}

#[tauri::command]
pub fn rename_file(src: &str, dest: &str) -> bool {
    std::fs::rename(src, dest).is_ok()
}

#[tauri::command]
pub fn open_file(path: &str, open_in_folder: bool) {
    let mut command = match std::env::consts::OS {
        "windows" => {
            if open_in_folder {
                Command::new("explorer")
                    .arg("/select,")
                    .arg(path)
                    .spawn()
                    .expect("Failed to execute command")
            } else {
                Command::new("cmd")
                    .args(&["/C", "start", "", path])
                    .spawn()
                    .expect("Failed to execute command")
            }
        }
        "macos" => {
            if open_in_folder {
                Command::new("open")
                    .arg("-R")
                    .arg(path)
                    .spawn()
                    .expect("Failed to execute command")
            } else {
                Command::new("open")
                    .arg(path)
                    .spawn()
                    .expect("Failed to execute command")
            }
        }
        "linux" => {
            if open_in_folder {
                Command::new("xdg-open")
                    .arg("-o")
                    .arg(path)
                    .spawn()
                    .expect("Failed to execute command")
            } else {
                Command::new("xdg-open")
                    .arg(path)
                    .spawn()
                    .expect("Failed to execute command")
            }
        }
        _ => panic!("Unsupported operating system"),
    };

    command.wait().expect("Failed to wait for command");
}
