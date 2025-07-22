use std::fs;
use std::fs::DirEntry;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

pub fn show(programs: Vec<String>) {
    println!("Programs to search for: {}", programs.join(", "));
    let os_path = std::env::var("PATH").unwrap();
    // println!("System path is: {}", os_path);
    let paths = os_path.split(':').map(|p| Path::new(p)).collect::<Vec<&Path>>();

    for path in paths {
        if !path.exists() && !path.is_dir() {
            continue;
        }
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries {
                if let Ok(entry) = entry && entry.path().is_file(){
                    // let filename = entry.file_name().to_str().unwrap().to_string();
                    let filename = remove_extension(&entry.file_name().to_str().unwrap().to_string());
                    if programs.contains(&filename) && is_executable(&entry) {
                        println!("{}", entry.path().display());
                    }
                }
            }
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

fn remove_extension(filename: &str) -> String {
    filename.rsplitn(2, '.').next().unwrap().to_string()
}

