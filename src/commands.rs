use crate::extrinsics::Account;
use crate::extrinsics::utility::*;
use crate::extrinsics::balances::*;
use crate::errors::BotError;
use crate::utils::convert_to_account_id;
use once_cell::sync::OnceCell;
use sp_runtime::AccountId32;
use sp_core::{sr25519::Pair, Pair as TraitPair, crypto::Ss58Codec};
use subxt::{
    PairSigner, DefaultNodeRuntime, Client, balances, system::AccountStoreExt
};
use std::rc::Rc;
use teloxide::{
    Bot, utils::command::BotCommand, dispatching::UpdateWithCx,
    adaptors::AutoSend, types::Message, requests::Request
};


#[derive(BotCommand)]
#[command(rename = "lowercase", description = "These commands are supported:")]
pub enum Command {
    #[command(description = "The Usage of Bot.")]
    Help,
    #[command(description = "Send tokens to user. \nExample: /drip 5Gf3M6b4hy6D7QdGwaKGv1AteiuLzpPw4XVo9FmuHZbDG6qn")]
    Drip(Account),
    #[command(description = "Check account info. \nExample: /info 5Gf3M6b4hy6D7QdGwaKGv1AteiuLzpPw4XVo9FmuHZbDG6qn")]
    Info(Account),
}

pub async fn answer(
    cx: UpdateWithCx<AutoSend<Bot>, Message>,
    command: Command
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    match command {
        Command::Help => {
            cx.answer(Command::descriptions()).send().await?;
        }
        Command::Drip(ref account) => {
            let url = "ws://120.79.25.151:9944";

            let signer = Pair::from_string("//Alice", None).map_err(|_| BotError::NoSigner)?;
            let signer = PairSigner::<DefaultNodeRuntime, Pair>::new(signer);

            let client: Client<DefaultNodeRuntime> = subxt::ClientBuilder::new()
                .set_url(url)
                .skip_type_sizes_check()
                .build()
                .await?;

            let sender = cx.update.from().map(|f| f.username.as_ref()).flatten().unwrap_or(&"ghost".to_owned()).to_owned();
            if let Ok(ref to) = convert_to_account_id(account) {
                let amount = 30_000_000_000_000u128;
                match balances_transfer("//Alice", url, &to, amount).await {
                    Ok(trx_id) => {
                        let resonse_text = format!(
                            "@{}, you received 30MA.",
                            sender
                        );
                        cx.answer(resonse_text).send().await?;
                    }
                    Err(e) => {
                        let resonse_text = format!(
                            "@{}, bot's busy, try it later.",
                            sender
                        );
                        cx.answer(resonse_text).send().await?;
                    }
                }
            } else {
                let resonse_text = format!(
                    "@{}, the address you input is invalid, please provide an valid one.",
                    sender
                );
                cx.answer(resonse_text).send().await?;
            }
        }
        Command::Info(ref account) => {
            let sender = cx.update.from().map(|f| f.username.as_ref()).flatten().unwrap_or(&"ghost".to_owned()).to_owned();
            if let Ok(ref who) = convert_to_account_id(account) {
                let url = "ws://120.79.25.151:9944";

                let client: Client<DefaultNodeRuntime> = subxt::ClientBuilder::new()
                    .set_url(url)
                    .skip_type_sizes_check()
                    .build()
                    .await?;

                let balance = client.account(who, None).await.expect("failed to get balance").data.free / 10u128.pow(12u32);
                let resonse_text = format!(
                    "@{}, your balances: {:?}.",
                    sender,
                    balance
                );
                cx.answer(resonse_text).send().await?;
            } else {
                let resonse_text = format!(
                    "@{}, the address you input is invalid, please provide an valid one.",
                    sender
                );
                cx.answer(resonse_text).send().await?;
            }
        }
        _ => {
            cx.answer("Not support now").send().await?;
        }
    }

    Ok(())
}
