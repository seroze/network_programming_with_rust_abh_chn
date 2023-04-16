//extern crate serde;
//extern crate serde_json;

use serde::{Deserialize, Serialize};
use serde_json;


#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Work{
    pub id: i32,
    pub work_code: String,
    pub add_up_to: i32,
    pub done: bool,

}

impl std::fmt::Display for Work {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\n---\nWORK:\n id: {}\n work_code: {}\n add_up_to: {}\n done: {}\n---",
               self.id, self.work_code, self.add_up_to, self.done,)
    }
}

fn main() {
    
    // give rust a struct 
    let work = Work{
        id: 1,
        work_code: "foo".to_string(),
        add_up_to: 100,
        done: false,
    };


    // Serialize the struct into a JSON String 
    let json_str = serde_json::to_string(&work).unwrap();
    println!("Serialize: {}", json_str);

    // Deserialize the JSON string into another Rust struct 
    let work_from_json: Work = serde_json::from_str(json_str.as_str()).unwrap();
    println!("Deserialize: {}", work_from_json);

    // assert both Rust structs are equal 
    assert_eq!(work, work_from_json);
    println!("Hello, world!");

}
