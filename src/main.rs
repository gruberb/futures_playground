use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::env;

use dotenv;

fn get_coin_price() -> Result<String, reqwest::Error>  {
    let client = reqwest::Client::new();
    let mut res = client.get("https://pro-api.coinmarketcap.com/v1/cryptocurrency/listings/latest")
        .header("X-CMC_PRO_API_KEY", env::var("CMC_PRO_API_KEY").unwrap())
        .header("Accepts", "application/json")
        .query(&[
            ("start", "1"),
            ("limit", "10"),
            ("convert", "USD"),
        ])
        .send()
        .expect("Failed to send request");
    
    Ok(res.text()?)
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

    let coins = get_coin_price().unwrap();
    write_file(coins);
}