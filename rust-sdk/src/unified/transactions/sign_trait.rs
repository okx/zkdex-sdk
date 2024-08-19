use crate::crypto::packed_public_key::PrivateKeyType;
use crate::crypto::sign::TxSignature;
use crate::felt::LeBytesConvert;
use crate::unified::transactions::hash_trait::HashTrait;

pub trait SignTrait: HashTrait {
    fn sign(&self, private_key: &PrivateKeyType) -> TxSignature {
        let hash = self.hash();
        let (sig, _pk) = TxSignature::sign_msg(&private_key, hash.as_le_bytes());
        sig
    }
}
