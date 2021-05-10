use crate::errors::BotError;
use crate::types::*;
use once_cell::sync::OnceCell;
use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path
};
use serde::{Serialize, Deserialize};
use sp_runtime::AccountId32;
use sp_core::{sr25519::Pair, Pair as TraitPair, crypto::Ss58Codec};
use subxt::{
    PairSigner, DefaultNodeRuntime, Client, balances, system::AccountStoreExt
};

pub fn bot_config() -> Result<BotConfig, BotError> {
    let config = File::open(
        concat!(env!("CARGO_MANIFEST_DIR"),"/bot.toml")).map_err(|_| BotError::ErrorBotConfig
    )?;

    let mut buff = BufReader::new(config);
    let mut contents = String::new();
    buff.read_to_string(&mut contents).map_err(|_| BotError::ErrorBotConfig)?;

    let config = toml::from_str(&contents).map_err(|_| BotError::ErrorBotConfig)?;
    
    Ok(config)
}

pub fn convert_to_account_id(address: &str) -> Result<AccountId32, BotError> {
    let account_id = AccountId32::from_string(address).map_err(|_| BotError::InvalidAccountId)?;

    Ok(account_id)
}

async fn create_client() -> Result<&'static subxt::Client<subxt::DefaultNodeRuntime>, BotError> {
    let url = &bot_config()?.node.node_url;

	static INSTANCE: OnceCell<subxt::Client<subxt::DefaultNodeRuntime>> = OnceCell::new();
	let client = subxt::ClientBuilder::new()
        .set_url(url)
        .skip_type_sizes_check()
        .build()
        .await?;

	Ok(INSTANCE.get_or_init(|| {
		client
	}))
}

pub fn open_db() -> Result<&'static sled::Db, BotError> {
    let db_path = bot_config()?.telegram.db_path;
    
    static INSTANCE: OnceCell<sled::Db> = OnceCell::new();
    let db = sled::open(db_path)?;

    Ok(INSTANCE.get_or_init(|| {
		db
	}))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_to_account_id_should_work() {
        let alice = "5Gf3M6b4hy6D7QdGwaKGv1AteiuLzpPw4XVo9FmuHZbDG6qn";
        let alice = convert_to_account_id(alice);
        assert!(alice.is_ok());

        let invalid_alice = "5Gf3M6b4hy6D7QdGwaKGv1AteiuLzpPw4XVo9FmuHZbDG6q";
        let invalid_alice = convert_to_account_id(invalid_alice);
        assert!(invalid_alice.is_err());
    }

    #[test]
    fn bot_config_should_work() {
        let config = bot_config();
        assert!(config.is_ok());

        let config = config.unwrap();
        assert_eq!(config.telegram.token, "123456789:blablabla");
        assert_eq!(config.node.precision, 12);
        assert_eq!(config.node.watch, false);
    }
}
