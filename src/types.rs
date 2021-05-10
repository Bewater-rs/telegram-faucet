use crate::utils;
use crate::errors::BotError;
use serde::{Serialize, Deserialize};
use sp_runtime::AccountId32;
use sp_core::{sr25519::Pair, Pair as TraitPair, crypto::Ss58Codec};
use subxt::{
    PairSigner, DefaultNodeRuntime, Client, balances, system::AccountStoreExt
};
use teloxide::Bot;
use teloxide::prelude::RequesterExt;
use teloxide::adaptors::AutoSend;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BotConfig {
    pub telegram: TelegramConfig,
    pub node: NodeConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelegramConfig {
    pub token: String,
    pub bot_name: String,
    pub db_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeConfig {
    pub seed: String,
    pub node_url: String,
    pub amount: u64,
    pub precision: u64,
    pub watch: bool,
}

pub struct TelegramNodeBot {
    pub telegram_bot: AutoSend<Bot>,
    pub client: Client<DefaultNodeRuntime>,
    pub signer: PairSigner<DefaultNodeRuntime, Pair>,
    pub amount: u64,
    pub precision: u64,
    pub watch: bool,
    pub db: sled::Db,
}

impl TelegramNodeBot {
    pub async fn new() -> Result<Self, BotError> {
        let bot_config = utils::bot_config()?;

        let bot_token = &bot_config.telegram.token;
        let bot = Bot::new(bot_token).auto_send();

        let url = &bot_config.node.node_url;
        let client: Client<DefaultNodeRuntime> = subxt::ClientBuilder::new()
            .set_url(url)
            .skip_type_sizes_check()
            .build()
            .await?;

        let seed = &bot_config.node.seed;
        let pair = Pair::from_string(seed, None).map_err(|_| BotError::NoSigner)?;
        let signer = PairSigner::<DefaultNodeRuntime, Pair>::new(pair);

        let db = sled::open(&bot_config.telegram.db_path)?;

        let precision = bot_config.node.precision;
        let amount = bot_config.node.amount;

        Ok(Self {
            telegram_bot: bot,
            client,
            signer,
            db,
            precision,
            amount,
            watch: bot_config.node.watch
        })
    }
}

#[derive(Default, Debug)]
pub struct NodeBotBuilder {
    token: Option<String>,
    url: Option<String>,
    db_path: Option<String>,
    signer: Option<String>,
}

impl NodeBotBuilder {
    pub fn set_bot_token(mut self, token: &str) -> Self {
        self.token = Some(token.to_owned());

        self
    }

    pub fn set_url(mut self, url: &str) -> Self {
        self.url = Some(url.to_owned());

        self
    }

    pub fn with_db(mut self, db_path: &str) -> Self {
        self.db_path = Some(db_path.to_owned());

        self
    }

    pub fn with_seed_or_private_key(mut self, seed_or_private_key: &str) -> Self {
        self.signer = Some(seed_or_private_key.to_owned());

        self
    }

    pub async fn build(mut self) -> Result<TelegramNodeBot, BotError> {
        let bot_config = utils::bot_config()?;
        
        let bot_token = self.url.as_ref().unwrap_or(&bot_config.telegram.token);
        let url = self.url.as_ref().unwrap_or(&bot_config.node.node_url);
        let seed = self.url.as_ref().unwrap_or(&bot_config.node.node_url);
        let db_path = self.url.as_ref().unwrap_or(&bot_config.node.node_url);

        let bot = Bot::new(bot_token).auto_send();

        let client: Client<DefaultNodeRuntime> = subxt::ClientBuilder::new()
            .set_url(url)
            .skip_type_sizes_check()
            .build()
            .await?;

        let pair = Pair::from_string(seed, None).map_err(|_| BotError::NoSigner)?;
        let signer = PairSigner::<DefaultNodeRuntime, Pair>::new(pair);

        let db = sled::open(&bot_config.telegram.db_path)?;
        
        let precision = bot_config.node.precision;
        let amount = bot_config.node.amount;

        Ok(TelegramNodeBot {
            telegram_bot: bot,
            client,
            signer,
            db,
            precision,
            amount,
            watch: bot_config.node.watch
        })
    }
}
