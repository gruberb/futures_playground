mod sync;

use dotenv;

fn main() {
    dotenv::dotenv().expect("Failed to read .env file");

    let coins = sync::get_coin_price().unwrap();
    sync::write_file(coins);
}