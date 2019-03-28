use crate::parser::{Element,Parser};

pub struct Yaml {
    config_file: String
}

impl Yaml {
    pub fn new(config_path: String, _file_path: String) -> Yaml {
        Yaml {config_file: config_path}
    }
}

impl Parser for Yaml {
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
