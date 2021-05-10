use crate::errors::BotError;
use subxt::{
    PairSigner, DefaultNodeRuntime, Client, balances,
};
use sp_core::{sr25519::Pair, Pair as TraitPair};
use sp_runtime::AccountId32;

#[allow(dead_code)]
pub async fn balances_transfer(signer: &str, url: &str, to: &AccountId32, amount: u128) -> Result<String, BotError> {
    let client: Client<DefaultNodeRuntime> = subxt::ClientBuilder::new()
        .set_url(url)
        .skip_type_sizes_check()
        .build()
        .await?;

    let signer = Pair::from_string(signer.as_ref(), None).map_err(|_| BotError::NoSigner)?;
    let signer = PairSigner::<DefaultNodeRuntime, Pair>::new(signer);

    let call = balances::TransferCall {
        to: &to.clone().into(),
        amount
    };

    let trx_id = client.submit(call, &signer).await?;

    Ok(trx_id.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use sp_core::crypto::Ss58Codec;

    #[tokio::test]
    async fn tets_balances_call() {
        let signer = "//Alice";
        let bob = AccountId32::from_string("5Gf3M6b4hy6D7QdGwaKGv1AteiuLzpPw4XVo9FmuHZbDG6qn").expect("invalid adress");
        let url = "ws://120.79.25.151:9944";
        let result = balances_transfer(signer, url, &bob, 1000_000_000_000u128).await;
        assert!(result.is_ok());
    }
}
