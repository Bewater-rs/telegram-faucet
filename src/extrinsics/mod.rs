pub mod balances;
pub mod utility;

pub type AnyCall = self::Encoded;
pub type Account = String;


// For fixing compilation issue about complainning about 
// subxt::Encoded doesn't derive Encoded.
#[derive(Clone, Debug, Eq, PartialEq, codec::Decode)]
pub struct Encoded(pub Vec<u8>);

impl codec::Encode for Encoded {
    fn encode(&self) -> Vec<u8> {
        self.0.to_owned()
    }
}

impl From<subxt::Encoded> for Encoded {
    fn from(coded: subxt::Encoded) -> Self {
        Self (coded.0)
    }
}
