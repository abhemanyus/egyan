use egyan::{Client, Create, Programme, Select};
use std::{env::args, path::Path};

fn main() {
    let args: Vec<String> = args().collect();
    let index_name = args.get(2).expect("Output directory not specified!");
    let index_page = args.get(1).expect("SLM url not specified!").to_string();
    let client = Client::new();
    let programme = Programme::new("".into(), index_page, &client).unwrap();
    println!("{programme:#?}");
    programme
        .create(Path::new(index_name), &client)
        .expect("failed to save");
}
