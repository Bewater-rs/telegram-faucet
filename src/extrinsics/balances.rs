use crate::errors::BotError;
use subxt::{
    PairSigner, DefaultNodeRuntime, Client,
    system::AccountStoreExt, balances,
};
use sp_core::{sr25519::Pair, Pair as TraitPair};
use std::error::Error;
use sp_runtime::AccountId32;
use std::convert::From;

#[allow(dead_code)]
pub async fn balances_transfer(signer: &str, url: &str, to: &AccountId32, amount: u128) -> Result<String, BotError> {
    let client: Client<DefaultNodeRuntime> = subxt::ClientBuilder::new()
        .set_url(url)
        .skip_type_sizes_check()
        .build().await?;

    let signer = Pair::from_string(signer.as_ref(), None).map_err(|_| BotError::NoSigner)?;
    let mut signer = PairSigner::<DefaultNodeRuntime, Pair>::new(signer);

    let call = balances::TransferCall {
        to: &to.clone().into(),
        amount
    };

    let trx_id = client.submit(call, &signer).await?;

    Ok(trx_id.to_string())
}

#[allow(dead_code)]
pub async fn create_balances_transfer_call<'b, F: 'b>(
    to: &'b AccountId32,
    amount: u128,
    f: F
) -> balances::TransferCall<'b, DefaultNodeRuntime> 
    where for<'a> F: Fn(&'a AccountId32, u128) -> balances::TransferCall<'a, DefaultNodeRuntime> {
    f(to, amount)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tets_call() {
        let alice = AccountId32::default();
        let f = |to: &AccountId32, amount: u128| {
            let call = balances::TransferCall::<DefaultNodeRuntime> {
                to: &to.clone().into(),
                amount
            };
            call
        };

        let calls = create_balances_transfer_call(&alice, 1, f);
    }
}