use teloxide::prelude::*;
use crate::commands::answer;

mod commands;
mod db;
mod errors;
mod extrinsics;
mod types;
mod utils;

#[tokio::main]
async fn main() {
    teloxide::enable_logging!();
    log::info!("Starting substrate-faucet-bot...");
    log::info!("Openning sled db...");

    let faucet_bot = Bot::new("1761630653:AAE4iLSKl9tyjkPbjkr88Kp9xd6A-2pa-ew").auto_send();

    let bot_name: String = String::from("faucet-bot");
    teloxide::commands_repl(faucet_bot, bot_name, answer).await;
}
