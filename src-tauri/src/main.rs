// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use functions::files;
use std::convert::TryInto;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read, Write};
use serde::de::DeserializeOwned;
use serde::{Serialize, Deserialize};
use tokio::fs;
use kiddo::{KdTree, SquaredEuclidean};
use bincode::{serialize, deserialize};

use ndarray::{Array, Axis, CowArray};
use ort::{Environment, Session, SessionBuilder};
use ort::tensor::OrtOwnedTensor;
use ort::Value as OrtValue;
use std::sync::Arc;

use rust_tokenizers::tokenizer::{BertTokenizer, Tokenizer, TruncationStrategy};

pub const COLLECTION_NAME: &str = "site";
pub const PREFIX_COLLECTION_NAME: &str = "prefix-cache";
pub const MODEL_PATH: &str = "all-MiniLM-L6-v2.onnx";
const VOCAB_PATH: &str = "vocab.txt";
const SPECIAL_TOKEN_PATH: &str = "special_tokens_map.json";

static HEADING: &str = "../"; //tauri is called from src-tauri directory, so "../ represents semanticOS directory"
static FILE_NAMES_PATH: &str = "./data/file_names.bincode";
static EMBEDDINGS_PATH: &str = "./data/embeddings.bincode";
static CONTENT_EMBEDDINGS_PATH: &str = "./data/content_embeddings.bincode";
static META_DATA_PATH: &str = "./data/meta_data.bincode";

#[tokio::main]
#[tauri::command]
//search for query given that the embeddings and file names are already saved to file
async fn search(query: &str) -> String {
    //initialize embedding model
    let tokenizer = BertTokenizer::from_file_with_special_token_mapping(
        VOCAB_PATH,
        true,
        false,
        SPECIAL_TOKEN_PATH,
    )
        .unwrap();
    let env = Arc::new(Environment::builder().build().unwrap());
    let session = SessionBuilder::new(&env)
    .unwrap()
    .with_model_from_file(MODEL_PATH)
    .unwrap();

    //load embeddings and file names from bincode
    let file_names: Vec<String> = load_bincode(&FILE_NAMES_PATH.to_string()).await;
    let embeddings: Vec<Vec<f32>> = load_bincode(&EMBEDDINGS_PATH.to_string()).await;
    let content_embeddings: Vec<Vec<f32>> = load_bincode(&CONTENT_EMBEDDINGS_PATH.to_string()).await;
    let content_lengths: HashMap<String, i32> = read_from_file(&META_DATA_PATH).expect("Error reading meta data from file");

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
    let query_embedding = get_embedding(&tokenizer, &session, query);
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
    let tokenizer = BertTokenizer::from_file_with_special_token_mapping(
        VOCAB_PATH,
        true,
        false,
        SPECIAL_TOKEN_PATH,
    )
        .unwrap();
    let env = Arc::new(Environment::builder().build().unwrap());
    let session = SessionBuilder::new(&env)
    .unwrap()
    .with_model_from_file(MODEL_PATH)
    .unwrap();

    //create file_names, file_contents, and content_lengths
    let file_names = files::get_file_names(HEADING);
    let (content_lengths, file_contents) = files::get_file_contents(&file_names);

    //generate embeddings
    let mut embeddings = Vec::new();
    for file_name in file_names.iter(){
        let embedding = get_embedding(&tokenizer, &session, file_name.as_str());
        embeddings.push(embedding);
    }
    let mut content_embeddings = Vec::new();
    for file_content in file_contents.iter(){
        let embedding = get_embedding(&tokenizer, &session, file_content.as_str());
        content_embeddings.push(embedding);
    }
    
    // serialize and save embeddings and file names to bincode file
    let serialized_embeddings = serialize(&embeddings).unwrap();
    let serialized_file_names = serialize(&file_names).unwrap();
    let serialized_content_embeddings = serialize(&content_embeddings).unwrap();

    fs::write(EMBEDDINGS_PATH, serialized_embeddings).await.unwrap(); 
    fs::write(FILE_NAMES_PATH, serialized_file_names).await.unwrap();
    fs::write(CONTENT_EMBEDDINGS_PATH, serialized_content_embeddings).await.unwrap();

    write_to_file(&content_lengths, META_DATA_PATH).expect("Error writing meta data to file");

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

fn write_to_file<T>(data: &T, file_path: &str) -> io::Result<()>
where
    T: Serialize,
{
    let serialized = serde_json::to_string(data)?;

    let mut file = File::create(file_path)?;
    file.write_all(serialized.as_bytes())?;

    Ok(())
}

fn read_from_file<T>(file_path: &str) -> io::Result<T>
where
    T: for<'de> Deserialize<'de>,
{
    let mut file = File::open(file_path)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    let deserialized: T = serde_json::from_str(&buffer)?;

    Ok(deserialized)
}

pub fn get_embedding(tokenizer: &BertTokenizer, session: &Session, query: &str) -> Vec<f32> {
    // tokenize
    let mut encoding = tokenizer.encode(query, None, 512, &TruncationStrategy::LongestFirst, 1);
    let alloc = session.allocator();
    let token_ids = std::mem::take(&mut encoding.token_ids);
    let shape = (1, token_ids.len());
    let token_ids = Array::from_shape_vec(shape, token_ids).unwrap();
    let attentions = Array::from_elem(shape, 1_i64);
    let type_ids = Array::from_elem(shape, 0_i64);
    // embed
    let output: OrtOwnedTensor<f32, _> = session
        .run(vec![
            OrtValue::from_array(alloc, &CowArray::from(token_ids.into_dyn())).unwrap(),
            OrtValue::from_array(alloc, &CowArray::from(attentions.into_dyn())).unwrap(),
            OrtValue::from_array(alloc, &CowArray::from(type_ids.into_dyn())).unwrap(),
        ])
        .unwrap()[0]
        .try_extract()
        .unwrap();
    let pooled = output.view().mean_axis(Axis(1)).unwrap();
    pooled.as_slice().unwrap().to_vec()
}


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![search, initialize])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

