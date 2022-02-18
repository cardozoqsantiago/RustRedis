use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize)]
pub struct Read {
    pub id: String,
    pub type_energy: String,
    pub units: i32,
    pub value: i8,
    pub tol: i8,
    pub date: String,
}