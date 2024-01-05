// $Env:LIBTORCH = "C:\Users\rnqua\AppData\Local\Programs\Python\libtorch"
// $Env:Path += ";C:\Users\rnqua\AppData\Local\Programs\Python\libtorch\lib"

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(unused_imports)]
use serde::Serialize;
use walkdir::WalkDir;
use tokio::fs;
use tokio::task;
use std::fs::File;
use std::thread;
use std::convert::TryInto;
use std::io::{self, BufRead};
use serde::de::DeserializeOwned;
use kiddo::KdTree;
use kiddo::SquaredEuclidean;
use kiddo::NearestNeighbour;    
use bincode::{serialize, deserialize};
use rust_bert::pipelines::sentence_embeddings::{SentenceEmbeddingsBuilder, SentenceEmbeddingsModelType};

//use tauri::api::file;
// use std::io::prelude::*;
// use reqwest;
// use walkdir::DirEntry;
//use bincode::Error;
static HEADING: &str = "../";
static FILE_NAMES_PATH: &str = "./data/file_names.bincode";
static EMBEDDINGS_PATH: &str = "./data/embeddings.bincode";

#[tokio::main]
#[tauri::command]
//search for query given that the embeddings and file names are already saved to file
async fn search(query: &str) -> String {

    let embedding_model = task::spawn_blocking(move || { SentenceEmbeddingsBuilder::remote(SentenceEmbeddingsModelType::AllMiniLmL12V2).create_model().unwrap()})
    .await.expect("Error creating embedding model");

    //load embeddings and file names from bincode
    let file_names: Vec<String> = load_bincode(&FILE_NAMES_PATH.to_string()).await;
    let embeddings: Vec<Vec<f32>> = load_bincode(&EMBEDDINGS_PATH.to_string()).await;

    //encode query
    let query_embedding = embedding_model.encode(&[&query.to_string()]).expect("Error encoding query");
    let query_vec: Result<[f32;384], _> = query_embedding[0].clone().try_into();

    //initialize kdtree
    let mut kdtree: KdTree<f32, 384> = KdTree::new();
    for i in 0..embeddings.len() {
        let vec: Result<[f32;384], _> = embeddings[i].clone().try_into();
        kdtree.add(&vec.unwrap(), i as u64);
    }

    //search for nearest neighbor
    let nearest_neighbor = kdtree.nearest_one::<SquaredEuclidean>(&query_vec.unwrap());
    let res = file_names[nearest_neighbor.item as usize].clone();

    //return nearest neighbor
    format!("{}", res)
}

//create the embeddings and file names under HEADING directory and save to file
async fn initialize(){
    let embedding_model = task::spawn_blocking(move || { SentenceEmbeddingsBuilder::remote(SentenceEmbeddingsModelType::AllMiniLmL12V2).create_model().unwrap()})
    .await.expect("Error creating embedding model");

    //create file paths and embeddings
    let file_names = get_file_names(HEADING);
    let embeddings = embedding_model.encode(&file_names).expect("Error encoding embeddings");
    
    // serialize and save embeddings and file names to bincode file
    let serialized_embeddings = serialize(&embeddings).unwrap();
    let serialized_file_names = serialize(&file_names).unwrap();
    fs::write(EMBEDDINGS_PATH, serialized_embeddings).await.unwrap(); 
    fs::write(FILE_NAMES_PATH, serialized_file_names).await.unwrap();
}

//load data from bincode file. Must be a deserializable vector of type T
async fn load_bincode<T>(path: &String) -> Vec<T>
where
    T: DeserializeOwned,
{
    let data = fs::read(path).await.expect("Error reading bincode file");
    match deserialize::<Vec<T>>(&data) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error deserializing data: {}", e);
            Default::default()
        }
    }
}

//Obtain all file names in the directory
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
    // Remove the heading
    for file_name in &mut file_names {
        *file_name = file_name.replace(HEADING, "")
    }

    file_names
}

//Read lines from a file into a vector
fn read_lines(path: &str, vec: &mut Vec<String>) -> io::Result<()> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    for line in reader.lines(){
        vec.push(line?);
    }
    Ok(())
}

//Check if a file path is valid. Meets requirements of endings and not in excluded folders
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

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![search])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
