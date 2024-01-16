use std::{fs::File, io::{self, Read, Write}};
use serde::{Serialize, Deserialize};
use ndarray::{Array, Axis, CowArray};
use ort::{Session, tensor::OrtOwnedTensor, Value as OrtValue};
use rust_tokenizers::tokenizer::{BertTokenizer, Tokenizer, TruncationStrategy};
use serde::de::DeserializeOwned;
use bincode::deserialize;
use tokio::fs;



//load data from bincode file. Must be a deserializable vector of type T
pub async fn load_bincode<T>(path: &String) -> Vec<T>
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

pub fn write_to_file<T>(data: &T, file_path: &str) -> io::Result<()>
where
    T: Serialize,
{
    let serialized = serde_json::to_string(data)?;

    let mut file = File::create(file_path)?;
    file.write_all(serialized.as_bytes())?;

    Ok(())
}

pub fn read_from_file<T>(file_path: &str) -> io::Result<T>
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