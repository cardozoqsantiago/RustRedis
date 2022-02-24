use mongodb::sync::Collection;
use mongodb::sync::Client;

#[path = "../models/record.rs"]
mod record;

pub struct Manager {
    pub coll: Collection
}

impl Manager{
    pub fn new() -> Self{
        let conn_string = std::env::var_os("MONGODB_URL").expect("missing environment variable MONGODB_URL").to_str().expect("failed to get MONGODB_URL").to_owned();
        let db_name = std::env::var_os("MONGODB_DATABASE").expect("missing environment variable MONGODB_DATABASE").to_str().expect("failed to get MONGODB_DATABASE").to_owned();
        let coll_name = std::env::var_os("MONGODB_COLLECTION").expect("missing environment variable MONGODB_COLLECTION").to_str().expect("failed to get MONGODB_COLLECTION").to_owned();
        let mongo_client = Client::with_uri_str(&*conn_string).expect("failed to create client");
        let todo_coll = mongo_client.database(db_name.as_str()).collection(coll_name.as_str());
            
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
        println!("inserted todo with id = {}", r.inserted_id);
    }
}