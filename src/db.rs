use chrono::{NaiveDate, NaiveDateTime, Utc};
use crate::errors::BotError;
use sled::{
    Db,
};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use sp_runtime::AccountId32;

pub type TelegramAccount = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserFaucetInfo {
    pub account: AccountId32,
    pub timestamp: NaiveDateTime,
    pub amount: u32,
}

impl UserFaucetInfo {
    fn new(account_id: AccountId32, amount: u32) -> Self {
        Self {
            account: account_id,
            timestamp: Utc::now().naive_utc(),
            amount
        }
    }
}

pub fn open_db(db_path: &str) -> Result<Db, BotError> {
    let db = sled::open(db_path)?;

    Ok(db)
}

// pub fn record_user_faucet_info(
//     db: &Db,
//     telegram_account: TelegramAccount,
//     address: &str,
//     amount: u32
// ) -> Result<(), BotError> {
//     let now = Utc::now().date().to_string();

//     // ensure this is now a brand new day
//     if db.contains_key(&now) {
//         // ensure this user doesn't claim any token within one day
//         let m = db.update_and_fetch(&now, |val| {

//         });
//     } else {
//         // A new day starts

//     }


//     let user_info = 

//     let encoded = bincode::serialize(&info).expect("failed to serialzie data.");
//     let address = "5Gf3M6b4hy6D7QdGwaKGv1AteiuLzpPw4XVo9FmuHZbDG6qn";

//     let db = open_db("/home/jamie/my-repo/telegram-faucet/sled/bot").expect("failed to open sled db");
//     db.insert(address, encoded);

// }

#[cfg(test)]
mod tests {
    use crate::utils::convert_to_account_id;
    use chrono::{NaiveDate, NaiveDateTime, Utc};
    use std::collections::HashMap;
    use super::*;
    use sp_runtime::AccountId32;

    #[test]
    fn test_sled() {
        // insert_new_record();
        let now = Utc::now();
        println!("now: {:?}", now.date());
        let db = open_db("/home/jamie/my-repo/telegram-faucet/sled/bot").expect("failed to open sled db");
        
        
        assert!(db.insert(now.date().to_string(), "now").is_ok());
        println!("now from sled: {:?}", db.get(now.date().to_string()));

        let address = "5Gf3M6b4hy6D7QdGwaKGv1AteiuLzpPw4XVo9FmuHZbDG6qn";
        let account_id = convert_to_account_id(address).expect("failed to convert address to account id.");
        let mut map = HashMap::new();
        let telegram_acco = "alice";

        let info = UserFaucetInfo::new(account_id, 20);

        map.insert(telegram_acco, info);

        let encoded = bincode::serialize(&map).expect("failed to serialzie data.");
        db.insert(now.date().to_string(), encoded);

        let m = db.get(now.date().to_string());
        println!("now from sled: {:?}", m);
        match m {
            Ok(Some(ref map)) => {
                let mp: HashMap::<String, UserFaucetInfo> = bincode::deserialize(map).unwrap();
                println!("alice applied faucet: {:?}", mp.contains_key(telegram_acco));
                dbg!(mp);
            }
            _ => {},
        }

        // db.update_and_fetch()
    }
}
