#![feature(impl_trait_in_bindings)]
extern crate clap;
extern crate log;
extern crate radix_trie;
extern crate simple_logger;

mod json;
mod parser;
mod yaml;
mod dedup;

use clap::{Arg, App};
use log::{info, error};
use json::{Json};
use parser::Parser;
use yaml::{Yaml};

fn main() {
    simple_logger::init().unwrap();
    let matches = App::new("Rusty dedup")
                      .version("1.0")
                      .author("Salvador Perez <sisekeom@protonmail.com>")
                      .about("Remove duplicates from arbitrary files")
                      .arg(Arg::with_name("type")
                           .short("t")
                           .long("type")
                           .value_name("json|yaml|csv")
                           .possible_values(&["json", "yaml", "csv"])
                           .help("Type of input file")
                           .required(true)
                           .takes_value(true))
                      .arg(Arg::with_name("config")
                           .short("c")
                           .long("config")
                           .value_name("File")
                           .help("Deduplication config")
                           .required(true)
                           .takes_value(true))
                      .arg(Arg::with_name("file")
                           .help("File to proces")
                           .required(true)
                           .index(1))
                      .get_matches();
    let config = matches.value_of("config").unwrap();
    let file = matches.value_of("file").unwrap();

    let parser : Box<Parser> = match matches.value_of("type").unwrap(){
        "json" => {
            info!("Its a json");
            let json_parser = Json::new(config.to_owned(), file.to_owned());
            Box::new(json_parser)
        }
        "yaml" => {
            info!("Its a yaml");
            let yaml_parser = Yaml::new(config.to_owned(), file.to_owned());
            Box::new(yaml_parser)
        }
        "csv" => {
            info!("Its a csv");
            let csv_parser = Json::new(config.to_owned(), file.to_owned());
            Box::new(csv_parser)
        }
        _ => {
            error!("Undefined type");
            let default_parser = Json::new(config.to_owned(), file.to_owned());
            Box::new(default_parser)
        }
    };
    dedup::deduplicate(&*parser);
    info!("All went well")
}
