use std::fs::DirEntry;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::{env, fs};

pub fn show(programs: Vec<String>) {
    println!("Programs to search for: {}", programs.join(", "));
    let os_path = match std::env::var("PATH") {
        Ok(path) => path,
        Err(_) => {
            eprintln!("Error: Could not read PATH environment");
            return;
        }
    };

    // let paths = os_path.split(':').map(|p| Path::new(p)).collect::<Vec<&Path>>();
    let paths = env::split_paths(&os_path)
        .filter(|path| path.is_dir())
        .collect::<Vec<PathBuf>>();

    for path in paths {
        if !path.exists() && !path.is_dir() {
            continue;
        }

        if let Ok(entries) = fs::read_dir(&path) {
            for entry in entries.filter_map(|entry| entry.ok()) {
                // if let Ok(entry) = entry && entry.path().is_file(){
                    // let filename = entry.file_name().to_str().unwrap().to_string();
                if !entry.path().is_file() {
                    continue;
                }

                // let filename = remove_extension(&entry.file_name().to_str().unwrap().to_string());
                if let Some(f) = entry.path().file_stem() {
                    let filename = f.to_string_lossy().into_owned();
                    if programs.contains(&filename) && is_executable(&entry) {
                        println!("{}", entry.path().display());
                    }
                }
                // }
            }
        }
        else {
            eprintln!("Error: Could not read directory: {}", path.display());
        }
    }
}

#[cfg(unix)]
fn is_executable(file: &DirEntry) -> bool {
    file.metadata().unwrap().permissions().mode() & 0o111 != 0
}

#[cfg(windows)]
fn is_executable(file: &DirEntry) -> bool {
    if let Ok(ext) = file.path().extension() {
        ext == "exe"
    } else {
        false
    }
}
