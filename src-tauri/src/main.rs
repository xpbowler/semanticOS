// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use walkdir::WalkDir;
// use walkdir::DirEntry;
// use serde::{Serialize, Deserialize};
use std::fs::File;
// use std::io::prelude::*;
// use reqwest;
use std::io::{self, BufRead};

use serde::Serialize;
//create static variable that is a string
static HEADING: &str = "../";

w#[derive(Debug, Serialize)]
struct MyError {
    message: String,
}

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for MyError {}

#[tauri::command]
async fn search(query: &str) -> Result<String, MyError> {
    //obtain search result here
    let result = get_file_names(HEADING).await;
    Ok(format!("{} {}",query, result[0]))
}

async fn get_file_names(root_folder: &str) -> Vec<String> {
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
    for file_name in &mut file_names {
        *file_name = file_name.replace(HEADING, "");
    }
    
    // ... call your server to get embeddingss
    // ... save embeddings and file names using serde_pickle

    // Return paths to saved files
    file_names
}

fn read_lines(path: &str, vec: &mut Vec<String>) -> io::Result<()> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    for line in reader.lines(){
        vec.push(line?);
    }
    Ok(())
}

fn is_valid_file(path: &String, endings: &Vec<String>, excluded_folders: &Vec<String>) -> bool {
    // ... logic to check if a file is valid based on endings and excluded folders
    for excluded_folder in excluded_folders.iter() {
        if path.contains(excluded_folder) {return false;}
    }
    for ending in endings.iter() {
        if path.ends_with(ending) {return true;}
    }
    return false;
}

// async fn search_for_query(query: &str) {
//     // ... logic to search for query
//     // You'll need to call your ML model server and perform similarity search
// }

// ... other helper functions as needed

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![search])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
