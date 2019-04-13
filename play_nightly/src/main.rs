#![feature(futures_api, async_await, await_macro)]

use futures::future::Future;
use futures::compat::Future01CompatExt;
use futures::executor::{self, ThreadPool};
use reqwest::r#async::Client;
use std::env;
use tokio::prelude::*;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

async fn get_coin_price() {
    let res = await!(Client::new()
        .get("https://pro-api.coinmarketcap.com/v1/cryptocurrency/listings/latest")
        .header("X-CMC_PRO_API_KEY", env::var("CMC_PRO_API_KEY").unwrap())
        .header("Accepts", "application/json")
        .query(&[
            ("start", "1"),
            ("limit", "10"),
            ("convert", "USD"),
        ])
        .send()
        .compat());
    
    println!("{}", res.unwrap().status());
    // write_file(String::from_utf8_lossy(&res).to_string());
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
    // executor::block_on(get_coin_price());
    tokio::run_async(get_coin_price());
    println!("After");
}