use core::fmt::Debug;
use crate::errors::BotError;
use codec::{Decode, Encode};
use core::marker::PhantomData;
use super::*;
use sp_core::{sr25519::Pair};
use sp_runtime::DispatchError;
use std::rc::Rc;
use subxt::{
	PairSigner, DefaultNodeRuntime, Call, Client, system::System, Event
};

/// 
#[derive(Clone)]
pub struct UtilityPallet {
    pub client: Rc<Client<DefaultNodeRuntime>>,
    pub signer: Option<PairSigner<DefaultNodeRuntime, Pair>>,
    pub watch: bool,
}

impl UtilityPallet {
	pub async fn batch(&self, calls: Vec<AnyCall>) -> Result<String, BotError> {
		let signer = self.signer.as_ref().ok_or(BotError::NoSigner)?;

		let batch_calls = BatchCall::<DefaultNodeRuntime> { calls };
		
		let hash = if self.watch {
			let extrinsic = self.client.create_signed(batch_calls, signer).await?;
			let extrinsic_success = self.client.submit_and_watch_extrinsic(extrinsic).await?;
			let event = extrinsic_success
				.find_event::<BatchCompletedEvent::<DefaultNodeRuntime>>()?
				.ok_or(BotError::NoEventFound)?;

			let block_hash = extrinsic_success.block;
			log::info!(
				"Batch all extrinsics successfully with event {:?}, and block hash: {}",
				event,
				block_hash
			);

			block_hash
		} else {
			let transaction_hash = self.client.submit(batch_calls, signer).await?;

			log::info!(
                "Submit batch all call to polakdot node , and transaction hash: {}",
                transaction_hash
            );

			transaction_hash
		};

		Ok(hash.to_string())
	}

	pub async fn batch_all(&self, calls: Vec<AnyCall>) -> Result<String, BotError> {
		let signer = self.signer.as_ref().ok_or(BotError::NoSigner)?;

		let batch_all_calls = BatchAllCall::<DefaultNodeRuntime> { calls };
		
		let hash = if self.watch {
			let extrinsic = self.client.create_signed(batch_all_calls, signer).await?;
			let extrinsic_success = self.client.submit_and_watch_extrinsic(extrinsic).await?;
			let event = extrinsic_success
				.find_event::<BatchCompletedEvent::<DefaultNodeRuntime>>()?
				.ok_or(BotError::NoEventFound)?;

			let block_hash = extrinsic_success.block;
			log::info!(
				"Batch all extrinsics successfully with event {:?}, and block hash: {}",
				event,
				block_hash
			);

			block_hash
		} else {
			let transaction_hash = self.client.submit(batch_all_calls, signer).await?;

			log::info!(
                "Submit batch all call to polakdot node , and transaction hash: {}",
                transaction_hash
            );

			transaction_hash
		};

		Ok(hash.to_string())
	}
}

#[subxt::module]
pub trait Utility: System {
	type Call:  Debug + Encode + Decode + Sync + Send;
}

impl Utility for DefaultNodeRuntime {
	type Call = AnyCall;
}

/// batch
/// Send a batch of dispatch calls.
#[derive(Clone, Debug, PartialEq, Call, Encode)]
pub struct BatchCall<T: Utility> {
	pub calls: Vec<T::Call>,
}

/// batch_all
/// Send a batch of dispatch calls and atomically execute them.
#[derive(Clone, Debug, PartialEq, Call, Encode)]
pub struct BatchAllCall<T: Utility> {
	pub calls: Vec<T::Call>,
}

/// Batch of dispatches completed fully with no error.
#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct BatchCompletedEvent<T: Utility> {
	pub _runtime: PhantomData<T>,
}

/// Batch of dispatches did not complete fully. Index of first failing dispatch given, as
/// well as the error. [index, error]
#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct BatchInterruptedEvent<T: Utility> {
	pub index: u32,
	pub error: DispatchError,
	pub _runtime: PhantomData<T>,
}
