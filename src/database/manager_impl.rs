#![allow(non_snake_case)]
use mongodb::sync::Collection;
use mongodb::sync::Client;

#[path = "../models/record.rs"]
mod record;

pub struct Manager {
    pub coll: Collection
}

impl Manager{
    pub fn new() -> Self{
        let conn_string = "mongodb://localhost:C2y6yDjf5%2FR%2Bob0N8A7Cgv30VRDJIWEHLM%2B4QDU5DE2nQ9nDuVTqobD4b8mGGyPMbIZnqyMsEcaGQy67XIw%2FJw%3D%3D@localhost:10255/admin?ssl=true";
        let db_name = "SampleDB";
        let coll_name = "Persons";
        let mongo_client = Client::with_uri_str(&*conn_string).expect("failed to create client");
        let todo_coll = mongo_client.database(db_name).collection(coll_name);
            
        Manager{coll: todo_coll}
    }

    pub fn add_todo(self, type_energy: &str, date: &str , units: i32, value: i8, tol: i8 ) {
        let new_record = record::Record {
            id: "001".to_string(),
            type_energy: type_energy.to_string(),
            units: units,
            value: value,
            tol: tol,
            date: date.to_string(),
        };
    
        let todo_doc = mongodb::bson::to_bson(&new_record).expect("struct to BSON conversion failed").as_document().expect("BSON to Document conversion failed").to_owned();
        
        let r = self.coll.insert_one(todo_doc, None).expect("failed to add todo");    
        println!("inserted record with id = {}", r.inserted_id);
    }
}