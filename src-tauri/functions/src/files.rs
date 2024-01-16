use walkdir::WalkDir;
use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;
use lopdf::Document;

//Obtain all file names in the directory
pub fn get_file_names(root_folder: &str) -> Vec<String> {
    // Read exclusions and endings
    let mut endings: Vec<String> = Vec::new();
    read_lines("./data/filters/endings.txt", &mut endings).expect("Error reading endings");
    let mut excluded_folders: Vec<String> = Vec::new();
    read_lines("./data/filters/excluded_folders.txt", &mut excluded_folders).expect("Error reading excluded folders");

    // Walk the directory
    let mut file_names = Vec::new();
    for entry in WalkDir::new(root_folder).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path().display().to_string();
        if entry.file_type().is_file() && is_valid_file(&path, &endings, &excluded_folders) {
            file_names.push(entry.path().display().to_string());
        }
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

pub fn get_file_contents(file_names: &Vec<String>) -> (HashMap<&String, i32>, Vec<String>) {
    let mut content_lengths = HashMap::new();
    let mut file_contents = Vec::new();
    for file_name in file_names{
        //Parse file if PDF
        let mut count = 0;
        let mut content: Vec<String> = Vec::new();
        if file_name.ends_with(".pdf") {(content,count) = parse_pdf(file_name)};
        // if file_name.ends_with(".txt") {content; count = parse_txt(file_name)};
        // if file_name.ends_with(".docx") {content; count = parse_docx(file_name)};
        file_contents.append(&mut content);
        content_lengths.insert(file_name, count);
    }
    (content_lengths, file_contents)
}

// fn parse_txt(file_name: &String) -> (Vec<String>, i32){
//     let mut count = 0;
//     let mut content = Vec::new();
//     let file = File::open(file_name);
//     let reader = BufReader::new(file);
//     for line in reader.lines(){
//         match line{
//             Ok(line) => {
//                 content.push(line);
//                 count+=1;
//             }
//             Err(err) => {
//                 eprintln!("{err}");
//             }
//         }
//     }
// }

// fn parse_docx(file_name: &String) -> (Vec<String>, i32){

// }

fn parse_pdf(file_name: &String) -> (Vec<String>, i32){
    let mut count = 0;
    let mut content = Vec::new();
    let doc = Document::load(file_name);
    match doc{
        Ok(document) => {
            let pages = document.get_pages();
            for (i, _) in pages.iter().enumerate() {
                let page_number = (i + 1) as u32;
                let mut text = document.extract_text(&[page_number]).unwrap_or_default();
                text = condense_text(text);
                content.push(text);
                count+=1;
            }

        } Err(err) => {
            eprintln!("{err}");
        }
    }
    (content,count)
}

fn condense_text(text: String) -> String{
    // Split the text into sentences based on common sentence-ending punctuation
    let sentences: Vec<&str> = text.split(|c| c == '.' || c == '!' || c == '?').collect();

    // Return the first sentence if it exists, otherwise return an empty string
    sentences.get(0).map_or(String::new(), |first_sentence| first_sentence.trim().to_string())
}

