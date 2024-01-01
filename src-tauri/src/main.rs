// $Env:LIBTORCH = "C:\Users\rnqua\AppData\Local\Programs\Python\libtorch"
// $Env:Path += ";C:\Users\rnqua\AppData\Local\Programs\Python\libtorch\lib"

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use serde::Serialize;
//use tauri::api::file;
use walkdir::WalkDir;
use tokio::fs;
use tokio::task;
use std::fs::File;
use std::thread;
use std::io::{self, BufRead};
static HEADING: &str = "../";

// use std::io::prelude::*;
// use reqwest;
// use walkdir::DirEntry;
use bincode::{serialize, deserialize};
//use bincode::Error;

use rust_bert::pipelines::sentence_embeddings::{SentenceEmbeddingsBuilder, SentenceEmbeddingsModelType};

#[tokio::main]
#[tauri::command]
async fn search(query: &str) -> String {
    //obtain search result here
    //let file_names = get_file_names(HEADING);
    // let embedding_model = task::spawn_blocking(move || { SentenceEmbeddingsBuilder::remote(SentenceEmbeddingsModelType::AllMiniLmL12V2).create_model().unwrap()})
    // .await.expect("Error creating embedding model");
    // let embeddings = embedding_model.encode(&file_names).expect("Error encoding embeddings");
    // // save embeddings and file names to file
    // let serialized_embeddings = serialize(&embeddings).unwrap();
    // let serialized_file_names = serialize(&file_names).unwrap();

    let embeddings_file_path = "./data/embeddings.bincode";
    let file_names_file_path = "./data/file_names.bincode";

    // fs::write(embeddings_file_path, serialized_embeddings).await.unwrap(); 
    // fs::write(file_names_file_path, serialized_file_names).await.unwrap();
    let file_names = load_file_names(&file_names_file_path.to_string()).await;
    format!("{} {}", query, file_names[10])
    
}

async fn load_file_names(file_names_file_path: &String) -> Vec<String> {

    let file_names_bincode = fs::read(file_names_file_path).await.expect("Error reading file names");
    match deserialize::<Vec<String>>(&file_names_bincode) {
        Ok(file_names) => file_names,
        Err(e) => {
            eprintln!("Error deserializing data: {}", e);
            Vec::new()
        }
    }
}

fn get_file_names(root_folder: &str) -> Vec<String> {
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
    // let embeddings = embedding_model.encode(&file_names)?;
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
//     //similarity search
// }

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![search])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
