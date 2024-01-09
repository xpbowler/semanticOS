// $Env:LIBTORCH = "C:\Users\rnqua\AppData\Local\Programs\Python\libtorch"
// $Env:Path += ";C:\Users\rnqua\AppData\Local\Programs\Python\libtorch\lib"

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![warn(dead_code)]
use functions::files;
use std::convert::TryInto;
use serde::de::DeserializeOwned;
use tokio::{fs,task};
use kiddo::{KdTree, SquaredEuclidean};
use bincode::{serialize, deserialize};
use rust_bert::pipelines::sentence_embeddings::{SentenceEmbeddingsBuilder, SentenceEmbeddingsModelType};

static HEADING: &str = "../"; //tauri is called from src-tauri directory, so "../ represents semanticOS directory"
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

#[tauri::command]
//create the embeddings and file names under HEADING directory and save to file
async fn initialize() -> String{
    let embedding_model = task::spawn_blocking(move || { SentenceEmbeddingsBuilder::remote(SentenceEmbeddingsModelType::AllMiniLmL12V2).create_model().unwrap()})
    .await.expect("Error creating embedding model");

    //create file paths and embeddings
    let file_names = files::get_file_names(HEADING);
    let embeddings = embedding_model.encode(&file_names).expect("Error encoding embeddings");
    
    // serialize and save embeddings and file names to bincode file
    let serialized_embeddings = serialize(&embeddings).unwrap();
    let serialized_file_names = serialize(&file_names).unwrap();
    fs::write(EMBEDDINGS_PATH, serialized_embeddings).await.unwrap(); 
    fs::write(FILE_NAMES_PATH, serialized_file_names).await.unwrap();
    format!("Initialized")
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

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![search, initialize])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
