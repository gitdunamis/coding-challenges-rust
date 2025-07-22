use std::fs;
use std::path::Path;

pub fn show(programs: Vec<String>) {
    println!("Programs to search for: {}", programs.join(", "));
    let os_path = std::env::var("PATH").unwrap();
    println!("System path is: {}", os_path);
    let paths = os_path.split(':').map(|p| Path::new(p)).collect::<Vec<&Path>>();

    for path in paths {
        if !path.exists() && !path.is_dir() {
            continue;
        }
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries {
                if let Ok(entry) = entry && entry.path().is_file(){
                    println!("File Name: {:?} Type: {:?}", entry.file_name(), entry.metadata().unwrap().permissions());
                }
            }
        }
    }
}
