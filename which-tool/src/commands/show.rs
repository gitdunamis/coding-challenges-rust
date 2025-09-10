use std::fs::DirEntry;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::{env, fs};
use std::collections::HashSet;
use crate::cli::Which;

pub fn show(args: Which) {
    let programs: Vec<String> = args.programs;
    let show_all = args.all;

    println!("Programs to search for: {}", programs.join(", "));
    let os_path = match env::var("PATH") {
        Ok(path) => path,
        Err(_) => {
            eprintln!("Error: Could not read PATH environment");
            return;
        }
    };

    let paths = env::split_paths(&os_path)
        .filter(|path| path.is_dir())
        .collect::<Vec<PathBuf>>();

    let mut found_programs: Vec<String> = vec![];

    for path in paths {
        if !path.exists() && !path.is_dir() {
            continue;
        }

        if let Ok(entries) = fs::read_dir(&path) {
            let entries: Vec<_> = entries.flatten().collect();
            for entry in &entries {
                if !entry.path().is_file() {
                    continue;
                }

                if let Some(f) = entry.path().file_stem() {
                    let filename = f.to_string_lossy().into_owned();
                    if programs.contains(&filename) && is_executable(entry) {
                        found_programs.push(entry.path().display().to_string());
                    }
                }
            }
        }
        else {
            eprintln!("Error: Could not read directory: {}", path.display());
        }
    }

    if show_all {
        for program in found_programs {
            println!("{}", program);
        }
    } else {
        //remove duplicate from found_programs
        let unique_programs: HashSet<String> = found_programs.into_iter().collect();
        for program in unique_programs {
            println!("{}", program);
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