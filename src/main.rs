use teloxide::prelude::*;
use crate::commands::answer;

mod commands;
mod db;
mod errors;
mod extrinsics;

#[tokio::main]
async fn main() {
    teloxide::enable_logging!();
    log::info!("Starting substrate-faucet-bot...");

    let faucet_bot = Bot::new("").auto_send();

    let bot_name: String = String::from("faucet-bot");
    teloxide::commands_repl(faucet_bot, bot_name, answer).await;
}

