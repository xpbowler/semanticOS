use walkdir::WalkDir;
use std::fs::File;
use std::io::{self, BufRead};


//Obtain all file names in the directory
pub fn get_file_names(root_folder: &str) -> Vec<String> {
    // Read exclusions and endings
    let mut endings: Vec<String> = Vec::new();
    read_lines("./data/endings.txt", &mut endings).expect("Error reading endings");
    let mut excluded_folders: Vec<String> = Vec::new();
    read_lines("./data/excluded_folders.txt", &mut excluded_folders).expect("Error reading excluded folders");

    // Walk the directory
    let mut file_names = Vec::new();
    for entry in WalkDir::new(root_folder).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path().display().to_string();
        if entry.file_type().is_file() && is_valid_file(&path, &endings, &excluded_folders) {
            file_names.push(entry.path().display().to_string());
        }
    }
    // Remove the heading
    for file_name in &mut file_names {
        *file_name = file_name.replace(root_folder, "");
        *file_name = file_name.replace("\\", " ");

    }

    file_names
}

//Read lines from a file into a vector
pub fn read_lines(path: &str, vec: &mut Vec<String>) -> io::Result<()> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    for line in reader.lines(){
        vec.push(line?);
    }
    Ok(())
}

//Check if a file path is valid. Meets requirements of endings and not in excluded folders
pub fn is_valid_file(path: &String, endings: &Vec<String>, excluded_folders: &Vec<String>) -> bool {
    // ... logic to check if a file is valid based on endings and excluded folders
    for excluded_folder in excluded_folders.iter() {
        if path.contains(excluded_folder) {return false;}
    }
    for ending in endings.iter() {
        if path.ends_with(ending) {return true;}
    }
    return false;
}