use crate::extrinsics::Account;
use teloxide::utils::command::BotCommand;
use teloxide::dispatching::UpdateWithCx;
use teloxide::adaptors::AutoSend;
use teloxide::Bot;
use teloxide::types::Message;
use teloxide::requests::Request;

#[derive(BotCommand)]
#[command(rename = "lowercase", description = "These commands are supported:")]
pub enum Command {
    #[command(description = "Bot can help.")]
    Help,
    #[command(description = "Send tokens to user. Example: /drip 5Gf3M6b4hy6D7QdGwaKGv1AteiuLzpPw4XVo9FmuHZbDG6qn")]
    Drip(Account),
    #[command(description = "Check account info. Example: /info 5Gf3M6b4hy6D7QdGwaKGv1AteiuLzpPw4XVo9FmuHZbDG6qn")]
    Info(Account),
}

pub async fn answer(
    cx: UpdateWithCx<AutoSend<Bot>, Message>,
    command: Command
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    match command {
        Command::Help => cx.answer(Command::descriptions()).send().await?,
        _ => cx.answer("Not support now").send().await?,
    };

    Ok(())
}
