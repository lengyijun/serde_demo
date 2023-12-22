use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

/*
fn main() {
    let mut my_map = HashMap::new();
    my_map.insert("name", "Alice");
    my_map.insert("city", "New York");

    // Using serde_json::to_string():
    let json_string = serde_json::to_string(&my_map).unwrap();
    println!("{}", json_string); // Output: {"name":"Alice","age":30,"city":"New York"}
}
*/

fn main() {
    let json_string = r#"{"name":"Bob","age":"25"}"#;
    let deserialized_map: HashMap<String, String> = serde_json::from_str(json_string).unwrap();
    println!("{:?}", deserialized_map); // Output: {"name": "Bob", "age": "25"}
}
