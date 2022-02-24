extern crate persistentcache;
extern crate persistentcache_procmacro;
use persistentcache::*;
use persistentcache::storage::{RedisStorage};
use persistentcache_procmacro::persistent_cache;

use serde::de;
#[path = "../models/record.rs"]
mod record;

#[path = "../database/manager_impl.rs"]
mod manager;


fn read_csv_generic<T: de::DeserializeOwned>(path: String) -> Vec<T> {
    let mut reader = csv::Reader::from_path(path).unwrap();
    let mut generics: Vec<T> = Vec::new();
    for result in reader.deserialize() {
        let generic = result.unwrap();
        generics.push(generic);
    }
    generics
}

#[persistent_cache]
#[params(RedisStorage, "redis://127.0.0.1")]
pub fn read_csv_file(path: String) -> Vec<record::Record> {
    println!("leyendo...");
    let read: Vec<record::Record> = read_csv_generic(path);
    read
}

pub fn add_record(type_energy: &str, date: &str , units: i32, value: i8, tol: i8){
    let m = manager::Manager::new();
    m.add_todo(type_energy, date, units, value, tol);
}