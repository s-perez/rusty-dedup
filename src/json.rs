use crate::parser::{Element, Parser};
use serde_json::Value;
use std::fs;
use log::{info, error};


pub struct Json {
    config: Value,
    json: Value,
    target: Vec<Value>
}

impl Json {
    pub fn new(config_path: String, file_path: String) -> Json {
        let json = read_json(&file_path);
        let config = read_json(&config_path);
        let target = match config["target_path"].as_str() {
            None => fail!(""),
            Some(path) =>
                get_element_from_json(&json, &path)
        };
        Json {
            config: config,
            json: json,
            target: target
        }
    }
}

impl Parser for Json {
    fn depleted(&self) -> bool {
        true
    }
    fn next(&self) -> Element {
        Element{
            hash: "aoe",
            element: ""
        }
    }
    fn notify(&self, element: &Element) -> Result<bool, &str> {
        Ok(true)
    }
}

fn read_json(file_path: &String) -> Value {
    let text = fs::read_to_string((*file_path).clone()).unwrap();
    let json: Value = serde_json::from_str(&text).expect("BLABLUBLI");
    json
}

fn get_element_from_json(json:Value, path:&str) {

}
