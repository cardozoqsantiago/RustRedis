#![allow(non_snake_case)]
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
#[params(RedisStorage, "rediss://:lWroPCjGb8RPABpYv2kV9E7XOZ7NFgjWaAzCaLntjX4=@matchredis.redis.cache.windows.net:6380")]
pub fn read_csv_file(path: String) -> Vec<record::Record> {
    println!("leyendo...");
    let read: Vec<record::Record> = read_csv_generic(path);
    read
}

pub fn add_record(type_energy: &str, date: &str , units: i32, value: i8, tol: i8){
    let m = manager::Manager::new();
    m.add_todo(type_energy, date, units, value, tol);
}

pub fn connect() -> redis::Connection {
    //format - host:port
    let redis_host_name =
    "matchredis.redis.cache.windows.net:6379";
    let redis_password = "3mMecS96MV1vta1OtZzPvdNrgACwGA125AzCaCWkEME=";
    let redis_conn_url = format!("redis://:{}@{}", redis_password, redis_host_name);
    redis::Client::open(redis_conn_url)
        .expect("Invalid connection URL")
        .get_connection()
        .expect("failed to connect to Redis")
}
