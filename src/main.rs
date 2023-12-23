use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Serialize, Deserialize)]
struct Sm {
    /// the number of times the card has been successfully recalled in a row since the last time it was not.
    n: u32,
    /// how "easy" the card is (more precisely, it determines how quickly the inter-repetition interval grows)
    ef: f32,
    ///  is the length of time (in days) SuperMemo will wait after the previous review before asking the user to review the card again.
    l: u32,
}

impl Default for Sm {
    // for init (new word)
    fn default() -> Self {
        Self {
            n: 0,
            ef: 2.5,
            l: 1,
        }
    }
}

fn main() {
    let mut my_map = HashMap::new();
    my_map.insert("name", Sm::default());
    my_map.insert("city", Sm::default());

    // Using serde_json::to_string():
    let json_string = serde_json::to_string(&my_map).unwrap();
    println!("{}", json_string); // Output: {"name":"Alice","age":30,"city":"New York"}
}

/*
fn main() {
    let json_string = r#"{"name":"Bob","age":"25"}"#;
    let deserialized_map: HashMap<String, String> = serde_json::from_str(json_string).unwrap();
    println!("{:?}", deserialized_map); // Output: {"name": "Bob", "age": "25"}
}
*/
