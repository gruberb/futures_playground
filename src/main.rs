mod play_sync;
mod play_async;

use tokio;
use dotenv;

fn main() {
    dotenv::dotenv().expect("Failed to read .env file");

    let coins = play_sync::get_coin_price().unwrap();
    play_sync::write_file(coins);

    tokio::run(play_async::fetch());
}