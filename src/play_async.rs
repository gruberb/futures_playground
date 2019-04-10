use futures;
use reqwest;

use std::mem;
use std::io::{self, Cursor};
use futures::{Future, Stream};
use reqwest::r#async::{Client, Decoder};
use std::env;

pub fn fetch() -> impl Future<Item=(), Error=()> {
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
            let mut body = Cursor::new(body);
            let _ = io::copy(&mut body, &mut io::stdout())
                .map_err(|err| {
                    println!("stdout error: {}", err);
                });
        })
}