use serde::de;
#[path = "../models/read.rs"]
mod read;


fn read_csv_generic<T: de::DeserializeOwned>(path: String) -> Vec<T> {
    let mut reader = csv::Reader::from_path(path).unwrap();
    let mut generics: Vec<T> = Vec::new();
    for result in reader.deserialize() {
        let generic = result.unwrap();
        generics.push(generic);
    }
    generics
}

pub fn read_csv_file(path: String) -> Vec<read::Read> {
    println!("leyendo...");
    let read: Vec<read::Read> = read_csv_generic(path);
    read
}