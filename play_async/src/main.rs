use futures::{Future, Stream};
use reqwest::r#async::{Client, Decoder};
use std::env;
use std::mem;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn get_coin_price() -> impl Future<Item=(), Error=()> {
    Client::new()
        .get("https://pro-api.coinmarketcap.com/v1/cryptocurrency/listings/latest")
        .header("X-CMC_PRO_API_KEY", env::var("CMC_PRO_API_KEY").unwrap())
        .header("Accepts", "application/json")
        .query(&[
            ("start", "1"),
            ("limit", "10"),
            ("convert", "USD"),
        ])
        .send()
        .and_then(|mut res| {
            println!("{}", res.status());

            let body = mem::replace(res.body_mut(), Decoder::empty());
            body.concat2()
        })
        .map_err(|err| println!("request error: {}", err))
        .map(|body| {
            let v = body.to_vec();
            write_file(String::from_utf8_lossy(&v).to_string());
        })
}

fn write_file(content: String) {
    let path = Path::new("coin_charts.json");
    let mut file = match File::create(&path) {
        Err(e) => panic!("{}", e),
        Ok(file) => file,
    };

    match file.write_all(content.as_bytes()) {
        Err(e) => panic!("{}", e),
        Ok(_) => println!("succesfully wrote file"),
    }
}

fn main() {
    dotenv::dotenv().expect("Failed to read .env file");
    
    println!("Before");
    tokio::run(get_coin_price());
    println!("After");
}