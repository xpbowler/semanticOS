// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use functions::{files, model};
use std::{collections::HashMap, convert::TryInto, sync::Arc};
use kiddo::{KdTree, SquaredEuclidean};
use bincode::serialize;
use ort::{Environment,SessionBuilder};
use rust_tokenizers::tokenizer::BertTokenizer;
use tokio::fs;

const MODEL_PATH: &str = "./data/embedding-model/all-MiniLM-L6-v2.onnx";
const VOCAB_PATH: &str = "./data/embedding-model/vocab.txt";
const SPECIAL_TOKEN_PATH: &str = "./data/embedding-model/special_tokens_map.json";
static HEADING: &str = "../"; //tauri is called from src-tauri directory, so "../ represents semanticOS directory"
static FILE_NAMES_PATH: &str = "./data/vectors-metadata/file_names.bin";
static EMBEDDINGS_PATH: &str = "./data/vectors-metadata/embeddings.bin";
static CONTENT_EMBEDDINGS_PATH: &str = "./data/vectors-metadata/content_embeddings.bin";
static META_DATA_PATH: &str = "./data/vectors-metadata/meta_data.bin";

#[tokio::main]
#[tauri::command]
//search for query given that the embeddings and file names are already saved to file
async fn search(query: &str) -> String {
    let tokenizer = BertTokenizer::from_file_with_special_token_mapping(VOCAB_PATH,true,false,SPECIAL_TOKEN_PATH,).unwrap();
    let env = Arc::new(Environment::builder().build().unwrap());
    let session = SessionBuilder::new(&env).unwrap().with_model_from_file(MODEL_PATH).unwrap();

    //load embeddings and file names from bincode
    let file_names: Vec<String> = model::load_bincode(&FILE_NAMES_PATH.to_string()).await;
    let embeddings: Vec<Vec<f32>> = model::load_bincode(&EMBEDDINGS_PATH.to_string()).await;
    let content_embeddings: Vec<Vec<f32>> = model::load_bincode(&CONTENT_EMBEDDINGS_PATH.to_string()).await;
    let content_lengths: HashMap<String, i32> = model::read_from_file(&META_DATA_PATH).expect("Error reading meta data from file");

    //initialize kdtree
    let mut kdtree: KdTree<f32, 384> = KdTree::new();
    for i in 0..embeddings.len() {
        let vec: Result<[f32;384], _> = embeddings[i].clone().try_into();
        kdtree.add(&vec.unwrap(), i as u64);
    }
    for i in 0..content_embeddings.len() {
        let vec: Result<[f32;384], _> = content_embeddings[i].clone().try_into();
        kdtree.add(&vec.unwrap(), (i+embeddings.len()) as u64);
    }

    //encode query
    let query_embedding = model::get_embedding(&tokenizer, &session, query);
    let query_vec: Result<[f32;384], _> = query_embedding.clone().try_into();

    //search for nearest neighbor
    let position = kdtree.nearest_one::<SquaredEuclidean>(&query_vec.unwrap()).item as usize;
    let mut res = String::new();
    if position < embeddings.len() {
        res = file_names[position].clone();
    } else {
        //determine element from content_embeddings and meta data
        let mut temp = embeddings.len();
        for file_name in file_names.iter(){
            temp  += *content_lengths.get(file_name).unwrap() as usize;
            if position <= temp {
                res = file_name.clone();
                break;
            }
        }
    }

    res = res.replace(HEADING, "");
    res = res.replace("\\", " ");

    //return nearest neighbor
    format!("{res}")
}

#[tauri::command]
//create the embeddings and file names under HEADING directory and save to file
async fn initialize() -> String{
    let tokenizer = BertTokenizer::from_file_with_special_token_mapping(VOCAB_PATH,true,false,SPECIAL_TOKEN_PATH,).unwrap();
    let env = Arc::new(Environment::builder().build().unwrap());
    let session = SessionBuilder::new(&env).unwrap().with_model_from_file(MODEL_PATH).unwrap();

    //create file_names, file_contents, and content_lengths
    let file_names = files::get_file_names(HEADING);
    let (content_lengths, file_contents) = files::get_file_contents(&file_names);

    //generate embeddings
    let mut embeddings = Vec::new();
    for file_name in file_names.iter(){
        let embedding = model::get_embedding(&tokenizer, &session, file_name.as_str());
        embeddings.push(embedding);
    }
    let mut content_embeddings = Vec::new();
    for file_content in file_contents.iter(){
        let embedding = model::get_embedding(&tokenizer, &session, file_content.as_str());
        content_embeddings.push(embedding);
    }
    
    // serialize and save embeddings and file names to bincode file
    let serialized_embeddings = serialize(&embeddings).unwrap();
    let serialized_file_names = serialize(&file_names).unwrap();
    let serialized_content_embeddings = serialize(&content_embeddings).unwrap();

    fs::write(EMBEDDINGS_PATH, serialized_embeddings).await.unwrap(); 
    fs::write(FILE_NAMES_PATH, serialized_file_names).await.unwrap();
    fs::write(CONTENT_EMBEDDINGS_PATH, serialized_content_embeddings).await.unwrap();
    model::write_to_file(&content_lengths, META_DATA_PATH).expect("Error writing meta data to file");

    format!("Initialized")
}


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![search, initialize])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}